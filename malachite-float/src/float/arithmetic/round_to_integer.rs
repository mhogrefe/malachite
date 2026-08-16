// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the GNU MPFR Library.
//
//      Copyright © 1999-2025 Free Software Foundation, Inc.
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Float;
use crate::InnerFloat::{Finite, Infinity, Zero};
use core::cmp::Ordering::{self, *};
use malachite_base::num::arithmetic::traits::{IsPowerOf2, NegModPowerOf2, PowerOf2};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::conversion::traits::{ExactFrom, IsInteger};
use malachite_base::num::logic::traits::{LowMask, SignificantBits};
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_nz::natural::Natural;
use malachite_nz::natural::arithmetic::float::round::{
    limbs_float_round_to_integer, with_float_significand_limbs,
};
use malachite_nz::platform::Limb;

// One with the given sign, at the given precision.
fn signed_one(sign: bool, prec: u64) -> Float {
    Float(Finite {
        sign,
        exponent: 1,
        precision: prec,
        significand: Natural::power_of_2(prec.neg_mod_power_of_2(Limb::LOG_WIDTH) + prec - 1),
    })
}

impl Float {
    // This is mpfr_rint from rint.c, MPFR 4.2.2, with the result's precision passed explicitly and
    // with MPFR_RNDNA (round to nearest, ties away from zero) selected by the `ties_away` flag
    // alongside `Nearest` rather than by a distinct rounding mode. Returns the rounded value, an
    // `Ordering` comparing it to the exact input, and whether the input was an integer; the pair is
    // a bijection with MPFR's refined ternary. The rounding is a single rounding to an integer
    // representable at `prec`: no double rounding is performed.

    // The maximum finite value with the given sign at the given precision, for overflow in the
    // directed modes that cannot produce an infinity.
    fn max_finite(sign: bool, prec: u64) -> Self {
        Self(Finite {
            sign,
            exponent: Self::MAX_EXPONENT,
            precision: prec,
            significand: Natural::low_mask(prec) << prec.neg_mod_power_of_2(Limb::LOG_WIDTH),
        })
    }

    fn round_to_integer_then_helper(
        &self,
        irm: RoundingMode,
        ties_away: bool,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        if !matches!(self, Self(Finite { .. })) || self.is_integer() {
            return Self::from_float_prec_round_ref(self, prec, rm);
        }
        // Rounding to an integer at self's own precision is exact, unless the carry at the maximum
        // exponent overflows.
        let t = self
            .round_to_integer_helper(self.significant_bits(), irm, ties_away)
            .0;
        if let Self(Infinity { sign }) = t {
            // The integer exceeds the exponent range; apply the final rounding mode's overflow
            // behavior, as mpfr_overflow does.
            let away = match rm {
                Floor => !sign,
                Ceiling => sign,
                Down => false,
                Up | Nearest => true,
                Exact => panic!("overflow in round_to_integer_then with the Exact mode"),
            };
            return if away {
                (t, if sign { Greater } else { Less })
            } else {
                (
                    Self::max_finite(sign, prec),
                    if sign { Less } else { Greater },
                )
            };
        }
        Self::from_float_prec_round(t, prec, rm)
    }

    /// Rounds a [`Float`] to an integer, representable at the specified precision, in the direction
    /// given by the specified rounding mode. An [`Ordering`] comparing the result to the exact
    /// input is also returned, along with a `bool` indicating whether the input was an integer. The
    /// [`Float`] is taken by value.
    ///
    /// The result is produced by a single rounding to an integer representable at the target
    /// precision: if the input's integer part needs more bits than the precision provides, no
    /// intermediate integer is formed. For example, $10.5$ rounded to the nearest integer at a
    /// precision of 2 bits is $12$: not first $10$, and then $10$ rounded again. The rounding mode
    /// gives the integer-rounding direction: `Floor` and `Ceiling` are the floor and ceiling
    /// functions, `Down` is truncation, `Up` rounds away from zero, and `Nearest` rounds to the
    /// nearest integer with ties to even. `Exact` is not allowed.
    ///
    /// The pair of the [`Ordering`] and the `bool` carries the same information as `mpfr_rint`'s
    /// ternary value: `(Equal, true)` means the input was an integer representable at the target
    /// precision, returned unchanged; `(Less, true)` and `(Greater, true)` mean the input was an
    /// integer that required rounding to fit the precision; `(Less, false)` and `(Greater, false)`
    /// mean the input was not an integer. `(Equal, false)` cannot occur.
    ///
    /// `NaN`s, infinities, and zeros are returned unchanged with `Equal`; of these, only zeros are
    /// considered integers.
    ///
    /// If rounding away from zero at the maximum exponent produces an integer too large to
    /// represent, the result is $\pm\infty$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec,
    /// self.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero or if `rm` is `Exact`.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    ///
    /// let x = Float::from(2.5f64);
    /// assert_eq!(
    ///     x.round_to_integer_prec_round_ref(4, Floor),
    ///     (Float::from(2u32), Less, false)
    /// );
    /// assert_eq!(
    ///     x.round_to_integer_prec_round_ref(4, Ceiling),
    ///     (Float::from(3u32), Greater, false)
    /// );
    /// assert_eq!(
    ///     x.round_to_integer_prec_round_ref(4, Nearest),
    ///     (Float::from(2u32), Less, false)
    /// );
    ///
    /// // A single rounding: the nearest integer to 10.5 representable at 2 bits is 12.
    /// let x = Float::from(10.5f64);
    /// assert_eq!(
    ///     x.round_to_integer_prec_round_ref(2, Nearest),
    ///     (Float::from(12u32), Greater, false)
    /// );
    ///
    /// // 7 is an integer, but needs rounding to fit 2 bits.
    /// let x = Float::from(7u32);
    /// assert_eq!(
    ///     x.round_to_integer_prec_round_ref(2, Nearest),
    ///     (Float::from(8u32), Greater, true)
    /// );
    /// ```
    #[inline]
    pub fn round_to_integer_prec_round(
        self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering, bool) {
        self.round_to_integer_helper(prec, rm, false)
    }

    /// Rounds a [`Float`] to an integer, representable at the specified precision, in the direction
    /// given by the specified rounding mode. An [`Ordering`] comparing the result to the exact
    /// input is also returned, along with a `bool` indicating whether the input was an integer. The
    /// [`Float`] is taken by reference.
    ///
    /// The result is produced by a single rounding to an integer representable at the target
    /// precision: if the input's integer part needs more bits than the precision provides, no
    /// intermediate integer is formed. For example, $10.5$ rounded to the nearest integer at a
    /// precision of 2 bits is $12$: not first $10$, and then $10$ rounded again. The rounding mode
    /// gives the integer-rounding direction: `Floor` and `Ceiling` are the floor and ceiling
    /// functions, `Down` is truncation, `Up` rounds away from zero, and `Nearest` rounds to the
    /// nearest integer with ties to even. `Exact` is not allowed.
    ///
    /// The pair of the [`Ordering`] and the `bool` carries the same information as `mpfr_rint`'s
    /// ternary value: `(Equal, true)` means the input was an integer representable at the target
    /// precision, returned unchanged; `(Less, true)` and `(Greater, true)` mean the input was an
    /// integer that required rounding to fit the precision; `(Less, false)` and `(Greater, false)`
    /// mean the input was not an integer. `(Equal, false)` cannot occur.
    ///
    /// `NaN`s, infinities, and zeros are returned unchanged with `Equal`; of these, only zeros are
    /// considered integers.
    ///
    /// If rounding away from zero at the maximum exponent produces an integer too large to
    /// represent, the result is $\pm\infty$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec,
    /// self.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero or if `rm` is `Exact`.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    ///
    /// let x = Float::from(2.5f64);
    /// assert_eq!(
    ///     x.round_to_integer_prec_round_ref(4, Floor),
    ///     (Float::from(2u32), Less, false)
    /// );
    /// assert_eq!(
    ///     x.round_to_integer_prec_round_ref(4, Ceiling),
    ///     (Float::from(3u32), Greater, false)
    /// );
    /// assert_eq!(
    ///     x.round_to_integer_prec_round_ref(4, Nearest),
    ///     (Float::from(2u32), Less, false)
    /// );
    ///
    /// // A single rounding: the nearest integer to 10.5 representable at 2 bits is 12.
    /// let x = Float::from(10.5f64);
    /// assert_eq!(
    ///     x.round_to_integer_prec_round_ref(2, Nearest),
    ///     (Float::from(12u32), Greater, false)
    /// );
    ///
    /// // 7 is an integer, but needs rounding to fit 2 bits.
    /// let x = Float::from(7u32);
    /// assert_eq!(
    ///     x.round_to_integer_prec_round_ref(2, Nearest),
    ///     (Float::from(8u32), Greater, true)
    /// );
    /// ```
    #[inline]
    pub fn round_to_integer_prec_round_ref(
        &self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering, bool) {
        self.round_to_integer_helper(prec, rm, false)
    }

    /// Rounds a [`Float`] to the nearest integer representable at the specified precision, with
    /// ties going to even. An [`Ordering`] comparing the result to the exact input is also
    /// returned, along with a `bool` indicating whether the input was an integer. The [`Float`] is
    /// taken by value.
    ///
    /// The result is produced by a single rounding to an integer representable at the target
    /// precision: if the input's integer part needs more bits than the precision provides, no
    /// intermediate integer is formed. For example, $10.5$ rounded to the nearest integer at a
    /// precision of 2 bits is $12$: not first $10$, and then $10$ rounded again.
    ///
    /// The pair of the [`Ordering`] and the `bool` carries the same information as `mpfr_rint`'s
    /// ternary value: `(Equal, true)` means the input was an integer representable at the target
    /// precision, returned unchanged; `(Less, true)` and `(Greater, true)` mean the input was an
    /// integer that required rounding to fit the precision; `(Less, false)` and `(Greater, false)`
    /// mean the input was not an integer. `(Equal, false)` cannot occur.
    ///
    /// `NaN`s, infinities, and zeros are returned unchanged with `Equal`; of these, only zeros are
    /// considered integers.
    ///
    /// If rounding away from zero at the maximum exponent produces an integer too large to
    /// represent, the result is $\pm\infty$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec,
    /// self.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    ///
    /// let x = Float::from(2.5f64);
    /// assert_eq!(
    ///     x.round_to_integer_prec_ref(4),
    ///     (Float::from(2u32), Less, false)
    /// );
    /// assert_eq!(
    ///     Float::from(4u32).round_to_integer_prec(2),
    ///     (Float::from(4u32), Equal, true)
    /// );
    /// ```
    #[inline]
    pub fn round_to_integer_prec(self, prec: u64) -> (Self, Ordering, bool) {
        self.round_to_integer_helper(prec, Nearest, false)
    }

    /// Rounds a [`Float`] to the nearest integer representable at the specified precision, with
    /// ties going to even. An [`Ordering`] comparing the result to the exact input is also
    /// returned, along with a `bool` indicating whether the input was an integer. The [`Float`] is
    /// taken by reference.
    ///
    /// The result is produced by a single rounding to an integer representable at the target
    /// precision: if the input's integer part needs more bits than the precision provides, no
    /// intermediate integer is formed. For example, $10.5$ rounded to the nearest integer at a
    /// precision of 2 bits is $12$: not first $10$, and then $10$ rounded again.
    ///
    /// The pair of the [`Ordering`] and the `bool` carries the same information as `mpfr_rint`'s
    /// ternary value: `(Equal, true)` means the input was an integer representable at the target
    /// precision, returned unchanged; `(Less, true)` and `(Greater, true)` mean the input was an
    /// integer that required rounding to fit the precision; `(Less, false)` and `(Greater, false)`
    /// mean the input was not an integer. `(Equal, false)` cannot occur.
    ///
    /// `NaN`s, infinities, and zeros are returned unchanged with `Equal`; of these, only zeros are
    /// considered integers.
    ///
    /// If rounding away from zero at the maximum exponent produces an integer too large to
    /// represent, the result is $\pm\infty$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec,
    /// self.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    ///
    /// let x = Float::from(2.5f64);
    /// assert_eq!(
    ///     x.round_to_integer_prec_ref(4),
    ///     (Float::from(2u32), Less, false)
    /// );
    /// assert_eq!(
    ///     Float::from(4u32).round_to_integer_prec(2),
    ///     (Float::from(4u32), Equal, true)
    /// );
    /// ```
    #[inline]
    pub fn round_to_integer_prec_ref(&self, prec: u64) -> (Self, Ordering, bool) {
        self.round_to_integer_helper(prec, Nearest, false)
    }

    /// Rounds a [`Float`] to an integer, representable at the input's own precision, in the
    /// direction given by the specified rounding mode. An [`Ordering`] comparing the result to the
    /// exact input is also returned, along with a `bool` indicating whether the input was an
    /// integer. The [`Float`] is taken by value.
    ///
    /// The rounding mode gives the integer-rounding direction: `Floor` and `Ceiling` are the floor
    /// and ceiling functions, `Down` is truncation, `Up` rounds away from zero, and `Nearest`
    /// rounds to the nearest integer with ties to even. `Exact` is not allowed.
    ///
    /// The pair of the [`Ordering`] and the `bool` carries the same information as `mpfr_rint`'s
    /// ternary value: `(Equal, true)` means the input was an integer representable at the target
    /// precision, returned unchanged; `(Less, true)` and `(Greater, true)` mean the input was an
    /// integer that required rounding to fit the precision; `(Less, false)` and `(Greater, false)`
    /// mean the input was not an integer. `(Equal, false)` cannot occur.
    ///
    /// `NaN`s, infinities, and zeros are returned unchanged with `Equal`; of these, only zeros are
    /// considered integers.
    ///
    /// If rounding away from zero at the maximum exponent produces an integer too large to
    /// represent, the result is $\pm\infty$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact`.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    ///
    /// let x = Float::from(2.5f64);
    /// assert_eq!(
    ///     x.round_to_integer_round_ref(Ceiling),
    ///     (Float::from(3u32), Greater, false)
    /// );
    /// assert_eq!(
    ///     x.round_to_integer_round_ref(Floor),
    ///     (Float::from(2u32), Less, false)
    /// );
    /// ```
    #[inline]
    pub fn round_to_integer_round(self, rm: RoundingMode) -> (Self, Ordering, bool) {
        self.round_to_integer_helper(self.significant_bits(), rm, false)
    }

    /// Rounds a [`Float`] to an integer, representable at the input's own precision, in the
    /// direction given by the specified rounding mode. An [`Ordering`] comparing the result to the
    /// exact input is also returned, along with a `bool` indicating whether the input was an
    /// integer. The [`Float`] is taken by reference.
    ///
    /// The rounding mode gives the integer-rounding direction: `Floor` and `Ceiling` are the floor
    /// and ceiling functions, `Down` is truncation, `Up` rounds away from zero, and `Nearest`
    /// rounds to the nearest integer with ties to even. `Exact` is not allowed.
    ///
    /// The pair of the [`Ordering`] and the `bool` carries the same information as `mpfr_rint`'s
    /// ternary value: `(Equal, true)` means the input was an integer representable at the target
    /// precision, returned unchanged; `(Less, true)` and `(Greater, true)` mean the input was an
    /// integer that required rounding to fit the precision; `(Less, false)` and `(Greater, false)`
    /// mean the input was not an integer. `(Equal, false)` cannot occur.
    ///
    /// `NaN`s, infinities, and zeros are returned unchanged with `Equal`; of these, only zeros are
    /// considered integers.
    ///
    /// If rounding away from zero at the maximum exponent produces an integer too large to
    /// represent, the result is $\pm\infty$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact`.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    ///
    /// let x = Float::from(2.5f64);
    /// assert_eq!(
    ///     x.round_to_integer_round_ref(Ceiling),
    ///     (Float::from(3u32), Greater, false)
    /// );
    /// assert_eq!(
    ///     x.round_to_integer_round_ref(Floor),
    ///     (Float::from(2u32), Less, false)
    /// );
    /// ```
    #[inline]
    pub fn round_to_integer_round_ref(&self, rm: RoundingMode) -> (Self, Ordering, bool) {
        self.round_to_integer_helper(self.significant_bits(), rm, false)
    }

    /// Rounds a [`Float`] to the nearest integer representable at the input's own precision, with
    /// ties going to even. An [`Ordering`] comparing the result to the exact input is also
    /// returned, along with a `bool` indicating whether the input was an integer. The [`Float`] is
    /// taken by value.
    ///
    /// The pair of the [`Ordering`] and the `bool` carries the same information as `mpfr_rint`'s
    /// ternary value: `(Equal, true)` means the input was an integer representable at the target
    /// precision, returned unchanged; `(Less, true)` and `(Greater, true)` mean the input was an
    /// integer that required rounding to fit the precision; `(Less, false)` and `(Greater, false)`
    /// mean the input was not an integer. `(Equal, false)` cannot occur.
    ///
    /// `NaN`s, infinities, and zeros are returned unchanged with `Equal`; of these, only zeros are
    /// considered integers.
    ///
    /// If rounding away from zero at the maximum exponent produces an integer too large to
    /// represent, the result is $\pm\infty$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Never panics.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    ///
    /// // ties to even
    /// let x = Float::from(2.5f64);
    /// assert_eq!(x.round_to_integer_ref(), (Float::from(2u32), Less, false));
    /// ```
    #[inline]
    pub fn round_to_integer(self) -> (Self, Ordering, bool) {
        self.round_to_integer_helper(self.significant_bits(), Nearest, false)
    }

    /// Rounds a [`Float`] to the nearest integer representable at the input's own precision, with
    /// ties going to even. An [`Ordering`] comparing the result to the exact input is also
    /// returned, along with a `bool` indicating whether the input was an integer. The [`Float`] is
    /// taken by reference.
    ///
    /// The pair of the [`Ordering`] and the `bool` carries the same information as `mpfr_rint`'s
    /// ternary value: `(Equal, true)` means the input was an integer representable at the target
    /// precision, returned unchanged; `(Less, true)` and `(Greater, true)` mean the input was an
    /// integer that required rounding to fit the precision; `(Less, false)` and `(Greater, false)`
    /// mean the input was not an integer. `(Equal, false)` cannot occur.
    ///
    /// `NaN`s, infinities, and zeros are returned unchanged with `Equal`; of these, only zeros are
    /// considered integers.
    ///
    /// If rounding away from zero at the maximum exponent produces an integer too large to
    /// represent, the result is $\pm\infty$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Never panics.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    ///
    /// // ties to even
    /// let x = Float::from(2.5f64);
    /// assert_eq!(x.round_to_integer_ref(), (Float::from(2u32), Less, false));
    /// ```
    #[inline]
    pub fn round_to_integer_ref(&self) -> (Self, Ordering, bool) {
        self.round_to_integer_helper(self.significant_bits(), Nearest, false)
    }

    /// Rounds a [`Float`] to the nearest integer representable at the specified precision, with
    /// ties going away from zero. An [`Ordering`] comparing the result to the exact input is also
    /// returned, along with a `bool` indicating whether the input was an integer. The [`Float`] is
    /// taken by value.
    ///
    /// The result is produced by a single rounding to an integer representable at the target
    /// precision: if the input's integer part needs more bits than the precision provides, no
    /// intermediate integer is formed. For example, $10.5$ rounded to the nearest integer at a
    /// precision of 2 bits is $12$: not first $10$, and then $10$ rounded again. Ties round away
    /// from zero, as in IEEE 754's roundTiesToAway and MPFR's `mpfr_round`; the other
    /// integer-rounding directions are available through [`Float::round_to_integer_prec_round`].
    ///
    /// The pair of the [`Ordering`] and the `bool` carries the same information as `mpfr_rint`'s
    /// ternary value: `(Equal, true)` means the input was an integer representable at the target
    /// precision, returned unchanged; `(Less, true)` and `(Greater, true)` mean the input was an
    /// integer that required rounding to fit the precision; `(Less, false)` and `(Greater, false)`
    /// mean the input was not an integer. `(Equal, false)` cannot occur.
    ///
    /// `NaN`s, infinities, and zeros are returned unchanged with `Equal`; of these, only zeros are
    /// considered integers.
    ///
    /// If rounding away from zero at the maximum exponent produces an integer too large to
    /// represent, the result is $\pm\infty$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec,
    /// self.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    ///
    /// // ties away from zero
    /// let x = Float::from(2.5f64);
    /// assert_eq!(
    ///     x.round_to_integer_ties_away_prec_ref(4),
    ///     (Float::from(3u32), Greater, false)
    /// );
    /// ```
    #[inline]
    pub fn round_to_integer_ties_away_prec(self, prec: u64) -> (Self, Ordering, bool) {
        self.round_to_integer_helper(prec, Nearest, true)
    }

    /// Rounds a [`Float`] to the nearest integer representable at the specified precision, with
    /// ties going away from zero. An [`Ordering`] comparing the result to the exact input is also
    /// returned, along with a `bool` indicating whether the input was an integer. The [`Float`] is
    /// taken by reference.
    ///
    /// The result is produced by a single rounding to an integer representable at the target
    /// precision: if the input's integer part needs more bits than the precision provides, no
    /// intermediate integer is formed. For example, $10.5$ rounded to the nearest integer at a
    /// precision of 2 bits is $12$: not first $10$, and then $10$ rounded again. Ties round away
    /// from zero, as in IEEE 754's roundTiesToAway and MPFR's `mpfr_round`; the other
    /// integer-rounding directions are available through [`Float::round_to_integer_prec_round`].
    ///
    /// The pair of the [`Ordering`] and the `bool` carries the same information as `mpfr_rint`'s
    /// ternary value: `(Equal, true)` means the input was an integer representable at the target
    /// precision, returned unchanged; `(Less, true)` and `(Greater, true)` mean the input was an
    /// integer that required rounding to fit the precision; `(Less, false)` and `(Greater, false)`
    /// mean the input was not an integer. `(Equal, false)` cannot occur.
    ///
    /// `NaN`s, infinities, and zeros are returned unchanged with `Equal`; of these, only zeros are
    /// considered integers.
    ///
    /// If rounding away from zero at the maximum exponent produces an integer too large to
    /// represent, the result is $\pm\infty$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec,
    /// self.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    ///
    /// // ties away from zero
    /// let x = Float::from(2.5f64);
    /// assert_eq!(
    ///     x.round_to_integer_ties_away_prec_ref(4),
    ///     (Float::from(3u32), Greater, false)
    /// );
    /// ```
    #[inline]
    pub fn round_to_integer_ties_away_prec_ref(&self, prec: u64) -> (Self, Ordering, bool) {
        self.round_to_integer_helper(prec, Nearest, true)
    }

    /// Rounds a [`Float`] to the nearest integer representable at the input's own precision, with
    /// ties going away from zero. An [`Ordering`] comparing the result to the exact input is also
    /// returned, along with a `bool` indicating whether the input was an integer. The [`Float`] is
    /// taken by value.
    ///
    /// Ties round away from zero, as in IEEE 754's roundTiesToAway and MPFR's `mpfr_round`; the
    /// other integer-rounding directions are available through
    /// [`Float::round_to_integer_prec_round`].
    ///
    /// The pair of the [`Ordering`] and the `bool` carries the same information as `mpfr_rint`'s
    /// ternary value: `(Equal, true)` means the input was an integer representable at the target
    /// precision, returned unchanged; `(Less, true)` and `(Greater, true)` mean the input was an
    /// integer that required rounding to fit the precision; `(Less, false)` and `(Greater, false)`
    /// mean the input was not an integer. `(Equal, false)` cannot occur.
    ///
    /// `NaN`s, infinities, and zeros are returned unchanged with `Equal`; of these, only zeros are
    /// considered integers.
    ///
    /// If rounding away from zero at the maximum exponent produces an integer too large to
    /// represent, the result is $\pm\infty$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Never panics.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    ///
    /// // ties away from zero
    /// let x = Float::from(2.5f64);
    /// assert_eq!(
    ///     x.round_to_integer_ties_away_ref(),
    ///     (Float::from(3u32), Greater, false)
    /// );
    /// ```
    #[inline]
    pub fn round_to_integer_ties_away(self) -> (Self, Ordering, bool) {
        self.round_to_integer_helper(self.significant_bits(), Nearest, true)
    }

    /// Rounds a [`Float`] to the nearest integer representable at the input's own precision, with
    /// ties going away from zero. An [`Ordering`] comparing the result to the exact input is also
    /// returned, along with a `bool` indicating whether the input was an integer. The [`Float`] is
    /// taken by reference.
    ///
    /// Ties round away from zero, as in IEEE 754's roundTiesToAway and MPFR's `mpfr_round`; the
    /// other integer-rounding directions are available through
    /// [`Float::round_to_integer_prec_round`].
    ///
    /// The pair of the [`Ordering`] and the `bool` carries the same information as `mpfr_rint`'s
    /// ternary value: `(Equal, true)` means the input was an integer representable at the target
    /// precision, returned unchanged; `(Less, true)` and `(Greater, true)` mean the input was an
    /// integer that required rounding to fit the precision; `(Less, false)` and `(Greater, false)`
    /// mean the input was not an integer. `(Equal, false)` cannot occur.
    ///
    /// `NaN`s, infinities, and zeros are returned unchanged with `Equal`; of these, only zeros are
    /// considered integers.
    ///
    /// If rounding away from zero at the maximum exponent produces an integer too large to
    /// represent, the result is $\pm\infty$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Never panics.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    ///
    /// // ties away from zero
    /// let x = Float::from(2.5f64);
    /// assert_eq!(
    ///     x.round_to_integer_ties_away_ref(),
    ///     (Float::from(3u32), Greater, false)
    /// );
    /// ```
    #[inline]
    pub fn round_to_integer_ties_away_ref(&self) -> (Self, Ordering, bool) {
        self.round_to_integer_helper(self.significant_bits(), Nearest, true)
    }

    /// Rounds a [`Float`] to an integer in the direction `irm`, and then correctly rounds that
    /// exact integer to the specified precision with `rm`. An [`Ordering`] comparing the result to
    /// the exact integer is also returned. The [`Float`] is taken by value.
    ///
    /// Unlike [`Float::round_to_integer_prec_round`], which rounds once, this function is the
    /// composition of two roundings, matching MPFR's `mpfr_rint_`-prefixed functions: the exact
    /// integer is formed first, then rounded to the target precision. The two can differ: under
    /// this function with both modes `Nearest`, $10.5$ becomes $10$, which then rounds to $8$ at a
    /// precision of 2 bits, while the single-rounding form gives $12$.
    ///
    /// `NaN`s, infinities, and non-integer-producing specials pass through the final rounding only.
    /// If the integer overflows the exponent range, the result follows `rm`: $\pm\infty$ for the
    /// modes rounding away from zero, and the maximum finite value at the target precision for the
    /// modes rounding toward zero.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec,
    /// self.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, if `irm` is `Exact`, or if `rm` is `Exact` and the integer is not
    /// exactly representable at the target precision.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    ///
    /// let x = Float::from(10.5f64);
    /// assert_eq!(
    ///     x.round_to_integer_then_prec_round_ref(Nearest, 2, Nearest),
    ///     (Float::from(8u32), Less)
    /// );
    /// assert_eq!(
    ///     Float::from(2.5f64).round_to_integer_then_prec_round_ref(Ceiling, 10, Nearest),
    ///     (Float::from(3u32), Equal)
    /// );
    /// ```
    #[inline]
    pub fn round_to_integer_then_prec_round(
        self,
        irm: RoundingMode,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        assert_ne!(irm, Exact);
        self.round_to_integer_then_helper(irm, false, prec, rm)
    }

    /// Rounds a [`Float`] to an integer in the direction `irm`, and then correctly rounds that
    /// exact integer to the specified precision with `rm`. An [`Ordering`] comparing the result to
    /// the exact integer is also returned. The [`Float`] is taken by reference.
    ///
    /// Unlike [`Float::round_to_integer_prec_round`], which rounds once, this function is the
    /// composition of two roundings, matching MPFR's `mpfr_rint_`-prefixed functions: the exact
    /// integer is formed first, then rounded to the target precision. The two can differ: under
    /// this function with both modes `Nearest`, $10.5$ becomes $10$, which then rounds to $8$ at a
    /// precision of 2 bits, while the single-rounding form gives $12$.
    ///
    /// `NaN`s, infinities, and non-integer-producing specials pass through the final rounding only.
    /// If the integer overflows the exponent range, the result follows `rm`: $\pm\infty$ for the
    /// modes rounding away from zero, and the maximum finite value at the target precision for the
    /// modes rounding toward zero.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec,
    /// self.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, if `irm` is `Exact`, or if `rm` is `Exact` and the integer is not
    /// exactly representable at the target precision.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    ///
    /// let x = Float::from(10.5f64);
    /// assert_eq!(
    ///     x.round_to_integer_then_prec_round_ref(Nearest, 2, Nearest),
    ///     (Float::from(8u32), Less)
    /// );
    /// assert_eq!(
    ///     Float::from(2.5f64).round_to_integer_then_prec_round_ref(Ceiling, 10, Nearest),
    ///     (Float::from(3u32), Equal)
    /// );
    /// ```
    #[inline]
    pub fn round_to_integer_then_prec_round_ref(
        &self,
        irm: RoundingMode,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        assert_ne!(irm, Exact);
        self.round_to_integer_then_helper(irm, false, prec, rm)
    }

    /// Rounds a [`Float`] to the nearest integer with ties going away from zero, and then correctly
    /// rounds that exact integer to the specified precision with `rm`. An [`Ordering`] comparing
    /// the result to the exact integer is also returned. The [`Float`] is taken by value.
    ///
    /// Unlike [`Float::round_to_integer_prec_round`], which rounds once, this function is the
    /// composition of two roundings, matching MPFR's `mpfr_rint_`-prefixed functions: the exact
    /// integer is formed first, then rounded to the target precision. The two can differ: under
    /// this function with both modes `Nearest`, $10.5$ becomes $10$, which then rounds to $8$ at a
    /// precision of 2 bits, while the single-rounding form gives $12$.
    ///
    /// `NaN`s, infinities, and non-integer-producing specials pass through the final rounding only.
    /// If the integer overflows the exponent range, the result follows `rm`: $\pm\infty$ for the
    /// modes rounding away from zero, and the maximum finite value at the target precision for the
    /// modes rounding toward zero.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec,
    /// self.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the integer is not exactly representable
    /// at the target precision.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    ///
    /// let x = Float::from(10.5f64);
    /// assert_eq!(
    ///     x.round_to_integer_ties_away_then_prec_round_ref(2, Nearest),
    ///     (Float::from(12u32), Greater)
    /// );
    /// ```
    #[inline]
    pub fn round_to_integer_ties_away_then_prec_round(
        self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        self.round_to_integer_then_helper(Nearest, true, prec, rm)
    }

    /// Rounds a [`Float`] to the nearest integer with ties going away from zero, and then correctly
    /// rounds that exact integer to the specified precision with `rm`. An [`Ordering`] comparing
    /// the result to the exact integer is also returned. The [`Float`] is taken by reference.
    ///
    /// Unlike [`Float::round_to_integer_prec_round`], which rounds once, this function is the
    /// composition of two roundings, matching MPFR's `mpfr_rint_`-prefixed functions: the exact
    /// integer is formed first, then rounded to the target precision. The two can differ: under
    /// this function with both modes `Nearest`, $10.5$ becomes $10$, which then rounds to $8$ at a
    /// precision of 2 bits, while the single-rounding form gives $12$.
    ///
    /// `NaN`s, infinities, and non-integer-producing specials pass through the final rounding only.
    /// If the integer overflows the exponent range, the result follows `rm`: $\pm\infty$ for the
    /// modes rounding away from zero, and the maximum finite value at the target precision for the
    /// modes rounding toward zero.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec,
    /// self.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the integer is not exactly representable
    /// at the target precision.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    ///
    /// let x = Float::from(10.5f64);
    /// assert_eq!(
    ///     x.round_to_integer_ties_away_then_prec_round_ref(2, Nearest),
    ///     (Float::from(12u32), Greater)
    /// );
    /// ```
    #[inline]
    pub fn round_to_integer_ties_away_then_prec_round_ref(
        &self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        self.round_to_integer_then_helper(Nearest, true, prec, rm)
    }

    pub(crate) fn round_to_integer_helper(
        &self,
        prec: u64,
        rm: RoundingMode,
        ties_away: bool,
    ) -> (Self, Ordering, bool) {
        assert_ne!(prec, 0);
        let Self(Finite {
            sign,
            exponent,
            significand,
            ..
        }) = self
        else {
            // NaN, infinities, and zeros are returned unchanged and are exact; among them, only
            // zeros are integers.
            return (self.clone(), Equal, matches!(self, Self(Zero { .. })));
        };
        let sign = *sign;
        let neg = !sign;
        let exp = *exponent;
        // The rounding direction in terms of magnitude: away from zero, toward zero, or (for the
        // nearest modes) not yet decided.
        let rnd_away = match rm {
            Floor => Some(neg),
            Ceiling => Some(sign),
            Down => Some(false),
            Up => Some(true),
            Nearest => None,
            Exact => panic!("round_to_integer with the Exact rounding mode"),
        };
        if exp <= 0 {
            // 0 < |u| < 1, so the result is 0 or +/-1, and the input is never an integer. In the
            // Nearest mode, 1/2 rounds to 0 by the even rule, but to +/-1 when ties go away from
            // zero.
            let away = match rnd_away {
                Some(away) => away,
                None => exp == 0 && (ties_away || !significand.is_power_of_2()),
            };
            return if away {
                (
                    signed_one(sign, prec),
                    if sign { Greater } else { Less },
                    false,
                )
            } else {
                (
                    Self(Zero { sign }),
                    if sign { Less } else { Greater },
                    false,
                )
            };
        }
        // Now exp > 0, so |u| >= 1.
        let (rp, exp_increment, uflags, rnd_away) =
            with_float_significand_limbs(significand, |up| {
                limbs_float_round_to_integer(up, u64::exact_from(exp), prec, rnd_away, ties_away)
            });
        if uflags == 0 {
            return (
                Self(Finite {
                    sign,
                    exponent: exp,
                    precision: prec,
                    significand: Natural::from_owned_limbs_asc(rp),
                }),
                Equal,
                true,
            );
        }
        if exp_increment && exp == Self::MAX_EXPONENT {
            // The rounded integer would exceed the maximum exponent; since the rounding was away
            // from zero, the result overflows to infinity.
            return (
                Self(Infinity { sign }),
                if sign { Greater } else { Less },
                uflags == 1,
            );
        }
        let o = if rnd_away == sign { Greater } else { Less };
        (
            Self(Finite {
                sign,
                exponent: if exp_increment { exp + 1 } else { exp },
                precision: prec,
                significand: Natural::from_owned_limbs_asc(rp),
            }),
            o,
            uflags == 1,
        )
    }
}
