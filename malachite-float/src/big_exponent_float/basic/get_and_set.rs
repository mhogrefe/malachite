// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::big_exponent_float::BigExponentFloat;
use crate::big_exponent_float::InnerBigExponentFloat::Finite;
use core::cmp::Ordering::{self, *};
use malachite_base::num::arithmetic::traits::RoundToMultipleOfPowerOf2Assign;
use malachite_base::num::basic::traits::One;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;

impl BigExponentFloat {
    /// Gets the significand of a [`BigExponentFloat`], taking the [`BigExponentFloat`] by
    /// reference.
    ///
    /// The significand is the smallest positive integer which is some power of 2 times the
    /// [`BigExponentFloat`], and whose number of significant bits is a multiple of the limb width.
    /// If the [`BigExponentFloat`] is NaN, infinite, or zero, then `None` is returned.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{Infinity, NaN, One, Zero};
    /// use malachite_float::BigExponentFloat;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(BigExponentFloat::NAN.to_significand(), None);
    /// assert_eq!(BigExponentFloat::INFINITY.to_significand(), None);
    /// assert_eq!(BigExponentFloat::ZERO.to_significand(), None);
    ///
    /// #[cfg(not(feature = "32_bit_limbs"))]
    /// assert_eq!(
    ///     BigExponentFloat::ONE.to_significand(),
    ///     Some(Natural::from(1u64 << 63))
    /// );
    /// ```
    #[inline]
    pub fn to_significand(&self) -> Option<Natural> {
        match self {
            Self(Finite { significand, .. }) => Some(significand.clone()),
            _ => None,
        }
    }

    /// Gets the significand of a [`BigExponentFloat`], taking the [`BigExponentFloat`] by value.
    ///
    /// The significand is the smallest positive integer which is some power of 2 times the
    /// [`BigExponentFloat`], and whose number of significant bits is a multiple of the limb width.
    /// If the [`BigExponentFloat`] is NaN, infinite, or zero, then `None` is returned.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{Infinity, NaN, One, Zero};
    /// use malachite_float::BigExponentFloat;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(BigExponentFloat::NAN.into_significand(), None);
    /// assert_eq!(BigExponentFloat::INFINITY.into_significand(), None);
    /// assert_eq!(BigExponentFloat::ZERO.into_significand(), None);
    ///
    /// #[cfg(not(feature = "32_bit_limbs"))]
    /// assert_eq!(
    ///     BigExponentFloat::ONE.into_significand(),
    ///     Some(Natural::from(1u64 << 63))
    /// );
    /// ```
    #[allow(clippy::missing_const_for_fn)] // destructor doesn't work with const
    #[inline]
    pub fn into_significand(self) -> Option<Natural> {
        match self {
            Self(Finite { significand, .. }) => Some(significand),
            _ => None,
        }
    }

    /// Returns a reference to the significand of a [`BigExponentFloat`].
    ///
    /// The significand is the smallest positive integer which is some power of 2 times the
    /// [`BigExponentFloat`], and whose number of significant bits is a multiple of the limb width.
    /// If the [`BigExponentFloat`] is NaN, infinite, or zero, then `None` is returned.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{Infinity, NaN, One, Zero};
    /// use malachite_float::BigExponentFloat;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(BigExponentFloat::NAN.significand_ref(), None);
    /// assert_eq!(BigExponentFloat::INFINITY.significand_ref(), None);
    /// assert_eq!(BigExponentFloat::ZERO.significand_ref(), None);
    ///
    /// #[cfg(not(feature = "32_bit_limbs"))]
    /// assert_eq!(
    ///     *BigExponentFloat::ONE.significand_ref().unwrap(),
    ///     1u64 << 63
    /// );
    /// ```
    #[inline]
    pub const fn significand_ref(&self) -> Option<&Natural> {
        match self {
            Self(Finite { significand, .. }) => Some(significand),
            _ => None,
        }
    }

    /// Returns a reference to the exponent of a [`BigExponentFloat`].
    ///
    /// The exponent is the smallest integer $e$ such that the absolute value of the
    /// [`BigExponentFloat`] is less than $2^e$; equivalently, it is one more than
    /// $\lfloor\log_2|x|\rfloor$. If the [`BigExponentFloat`] is NaN, infinite, or zero, then
    /// `None` is returned.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{Infinity, NaN, One, OneHalf, Two, Zero};
    /// use malachite_float::BigExponentFloat;
    ///
    /// assert_eq!(BigExponentFloat::NAN.get_exponent(), None);
    /// assert_eq!(BigExponentFloat::INFINITY.get_exponent(), None);
    /// assert_eq!(BigExponentFloat::ZERO.get_exponent(), None);
    /// assert_eq!(*BigExponentFloat::ONE.get_exponent().unwrap(), 1);
    /// assert_eq!(*BigExponentFloat::TWO.get_exponent().unwrap(), 2);
    /// assert_eq!(*BigExponentFloat::ONE_HALF.get_exponent().unwrap(), 0);
    /// ```
    #[inline]
    pub const fn get_exponent(&self) -> Option<&Integer> {
        match self {
            Self(Finite { exponent, .. }) => Some(exponent),
            _ => None,
        }
    }

    /// Returns the precision of a [`BigExponentFloat`]: the number of bits from its most
    /// significant 1 to its least significant 1, inclusive.
    ///
    /// A [`BigExponentFloat`] stores no precision of its own, so this is the smallest precision at
    /// which it is exactly representable, the counterpart of
    /// [`Float::get_min_prec`](crate::Float::get_min_prec). If the [`BigExponentFloat`] is NaN,
    /// infinite, or zero, then `None` is returned.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{Infinity, NaN, One, Zero};
    /// use malachite_float::BigExponentFloat;
    ///
    /// assert_eq!(BigExponentFloat::NAN.get_prec(), None);
    /// assert_eq!(BigExponentFloat::INFINITY.get_prec(), None);
    /// assert_eq!(BigExponentFloat::ZERO.get_prec(), None);
    /// assert_eq!(BigExponentFloat::ONE.get_prec(), Some(1));
    /// ```
    pub fn get_prec(&self) -> Option<u64> {
        match self {
            Self(Finite { significand, .. }) => {
                Some(significand.significant_bits() - significand.trailing_zeros().unwrap())
            }
            _ => None,
        }
    }

    /// Rounds a [`BigExponentFloat`] to a given precision, in place, using the specified rounding
    /// mode. An [`Ordering`] is returned, indicating whether the rounded value is less than, equal
    /// to, or greater than the original value.
    ///
    /// A [`BigExponentFloat`] whose precision is at most `prec` is left unchanged. The exponent is
    /// unbounded, so rounding never overflows; rounding up to the next power of 2 just increments
    /// the exponent. NaN, $\pm\infty$, and zero are left unchanged, and `Equal` is returned.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the [`BigExponentFloat`] cannot be
    /// represented exactly with precision `prec`.
    ///
    /// This is FLINT's `arf_set_round` with the input and output identified.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = BigExponentFloat::ONE;
    /// assert_eq!(x.round_to_prec_round_assign(10, Nearest), Equal);
    /// assert_eq!(format!("{x:?}"), "0x1.0");
    /// ```
    pub fn round_to_prec_round_assign(&mut self, prec: u64, rm: RoundingMode) -> Ordering {
        assert_ne!(prec, 0);
        match self {
            Self(Finite {
                sign,
                exponent,
                significand,
            }) => {
                let bits = significand.significant_bits();
                if prec >= bits - significand.trailing_zeros().unwrap() {
                    return Equal;
                }
                let limb_count = significand.limb_count();
                let abs_rm = if *sign { rm } else { -rm };
                let o = significand.round_to_multiple_of_power_of_2_assign(bits - prec, abs_rm);
                // Rounding up to the next power of 2 adds a limb.
                if significand.limb_count() > limb_count {
                    *significand >>= 1u32;
                    *exponent += Integer::ONE;
                }
                Self::trim_significand(significand);
                if *sign { o } else { o.reverse() }
            }
            _ => Equal,
        }
    }

    /// Rounds a [`BigExponentFloat`] to a given precision, using the specified rounding mode. The
    /// [`BigExponentFloat`] is taken by value. An [`Ordering`] is also returned, indicating whether
    /// the rounded value is less than, equal to, or greater than the original value.
    ///
    /// See [`BigExponentFloat::round_to_prec_round_assign`] for details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the [`BigExponentFloat`] cannot be
    /// represented exactly with precision `prec`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::Two;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (x, o) = BigExponentFloat::TWO.round_to_prec_round(1, Floor);
    /// assert_eq!(format!("{x:?}"), "0x2.0");
    /// assert_eq!(o, Equal);
    /// ```
    #[inline]
    pub fn round_to_prec_round(mut self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        let o = self.round_to_prec_round_assign(prec, rm);
        (self, o)
    }

    /// Rounds a [`BigExponentFloat`] to a given precision, using the specified rounding mode. The
    /// [`BigExponentFloat`] is taken by reference. An [`Ordering`] is also returned, indicating
    /// whether the rounded value is less than, equal to, or greater than the original value.
    ///
    /// See [`BigExponentFloat::round_to_prec_round_assign`] for details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the [`BigExponentFloat`] cannot be
    /// represented exactly with precision `prec`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::Two;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (x, o) = BigExponentFloat::TWO.round_to_prec_round_ref(1, Ceiling);
    /// assert_eq!(format!("{x:?}"), "0x2.0");
    /// assert_eq!(o, Equal);
    /// ```
    #[inline]
    pub fn round_to_prec_round_ref(&self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        self.clone().round_to_prec_round(prec, rm)
    }

    /// Rounds a [`BigExponentFloat`] to a given precision, in place, rounding to the nearest value
    /// and breaking ties to even. An [`Ordering`] is returned, indicating whether the rounded value
    /// is less than, equal to, or greater than the original value.
    ///
    /// See [`BigExponentFloat::round_to_prec_round_assign`] for details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = BigExponentFloat::ONE;
    /// assert_eq!(x.round_to_prec_assign(1), Equal);
    /// assert_eq!(format!("{x:?}"), "0x1.0");
    /// ```
    #[inline]
    pub fn round_to_prec_assign(&mut self, prec: u64) -> Ordering {
        self.round_to_prec_round_assign(prec, Nearest)
    }

    /// Rounds a [`BigExponentFloat`] to a given precision, rounding to the nearest value and
    /// breaking ties to even. The [`BigExponentFloat`] is taken by value. An [`Ordering`] is also
    /// returned, indicating whether the rounded value is less than, equal to, or greater than the
    /// original value.
    ///
    /// See [`BigExponentFloat::round_to_prec_round_assign`] for details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (x, o) = BigExponentFloat::ONE.round_to_prec(1);
    /// assert_eq!(format!("{x:?}"), "0x1.0");
    /// assert_eq!(o, Equal);
    /// ```
    #[inline]
    pub fn round_to_prec(self, prec: u64) -> (Self, Ordering) {
        self.round_to_prec_round(prec, Nearest)
    }

    /// Rounds a [`BigExponentFloat`] to a given precision, rounding to the nearest value and
    /// breaking ties to even. The [`BigExponentFloat`] is taken by reference. An [`Ordering`] is
    /// also returned, indicating whether the rounded value is less than, equal to, or greater than
    /// the original value.
    ///
    /// See [`BigExponentFloat::round_to_prec_round_assign`] for details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (x, o) = BigExponentFloat::ONE.round_to_prec_ref(1);
    /// assert_eq!(format!("{x:?}"), "0x1.0");
    /// assert_eq!(o, Equal);
    /// ```
    #[inline]
    pub fn round_to_prec_ref(&self, prec: u64) -> (Self, Ordering) {
        self.round_to_prec_round_ref(prec, Nearest)
    }
}
