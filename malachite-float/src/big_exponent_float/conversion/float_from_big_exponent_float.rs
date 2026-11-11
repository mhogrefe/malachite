// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.
use crate::big_exponent_float::BigExponentFloat;
use crate::big_exponent_float::InnerBigExponentFloat::{Finite, Infinity, NaN, Zero};
use crate::{Float, InnerFloat};
use core::cmp::Ordering::{self, *};
use malachite_base::num::arithmetic::traits::IsPowerOf2;
use malachite_base::num::basic::traits::{
    Infinity as _, NaN as _, NegativeInfinity, NegativeZero, Zero as _,
};
use malachite_base::num::conversion::traits::ConvertibleFrom;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_q::rational::conversion::primitive_float_from_rational::FloatConversionError;

// The `Float` with the same value as the finite nonzero `x`, at `x`'s precision, if `x`'s exponent
// is in `Float`'s range; otherwise, whether `x` overflows (`Ok(true)`) or underflows (`Ok(false)`).
fn exact_float(x: &BigExponentFloat) -> Result<Float, bool> {
    let BigExponentFloat(Finite {
        sign,
        exponent,
        significand,
    }) = x
    else {
        unreachable!()
    };
    match i32::try_from(exponent) {
        Ok(e) if (Float::MIN_EXPONENT..=Float::MAX_EXPONENT).contains(&e) => {
            Ok(Float(InnerFloat::Finite {
                sign: *sign,
                exponent: e,
                precision: x.get_prec().unwrap(),
                significand: significand.clone(),
            }))
        }
        _ => Err(*exponent > 0u32),
    }
}

impl TryFrom<&BigExponentFloat> for Float {
    type Error = FloatConversionError;

    /// Converts a [`BigExponentFloat`] to a [`Float`], taking the [`BigExponentFloat`] by
    /// reference.
    ///
    /// The result has the [`BigExponentFloat`]'s value and its precision. Every precision is
    /// available, so the conversion fails only when the exponent is outside [`Float`]'s range:
    /// [`FloatConversionError::Overflow`] is returned for a value whose absolute value is at least
    /// $2^{2^{30}-1}$, and [`FloatConversionError::Underflow`] for a nonzero value whose absolute
    /// value is less than $2^{-2^{30}}$. NaN, $\pm\infty$, and zero become the corresponding
    /// [`Float`]s; zero becomes positive zero.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `x.complexity()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_float::{BigExponentFloat, Float};
    ///
    /// assert_eq!(Float::try_from(&BigExponentFloat::ONE).unwrap(), 1);
    /// ```
    fn try_from(x: &BigExponentFloat) -> Result<Self, Self::Error> {
        match x {
            BigExponentFloat(NaN) => Ok(Self::NAN),
            BigExponentFloat(Infinity { sign: true }) => Ok(Self::INFINITY),
            BigExponentFloat(Infinity { sign: false }) => Ok(Self::NEGATIVE_INFINITY),
            BigExponentFloat(Zero) => Ok(Self::ZERO),
            _ => exact_float(x).map_err(|overflow| {
                if overflow {
                    FloatConversionError::Overflow
                } else {
                    FloatConversionError::Underflow
                }
            }),
        }
    }
}

impl TryFrom<BigExponentFloat> for Float {
    type Error = FloatConversionError;

    /// Converts a [`BigExponentFloat`] to a [`Float`], taking the [`BigExponentFloat`] by value.
    ///
    /// The result has the [`BigExponentFloat`]'s value and its precision. Every precision is
    /// available, so the conversion fails only when the exponent is outside [`Float`]'s range:
    /// [`FloatConversionError::Overflow`] is returned for a value whose absolute value is at least
    /// $2^{2^{30}-1}$, and [`FloatConversionError::Underflow`] for a nonzero value whose absolute
    /// value is less than $2^{-2^{30}}$. NaN, $\pm\infty$, and zero become the corresponding
    /// [`Float`]s; zero becomes positive zero.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `x.complexity()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_float::{BigExponentFloat, Float};
    ///
    /// assert_eq!(Float::try_from(BigExponentFloat::ONE).unwrap(), 1);
    /// ```
    #[inline]
    fn try_from(x: BigExponentFloat) -> Result<Self, Self::Error> {
        Self::try_from(&x)
    }
}

impl ConvertibleFrom<&BigExponentFloat> for Float {
    /// Determines whether a [`BigExponentFloat`] can be converted to a [`Float`]: whether it is not
    /// finite, is zero, or has an exponent in [`Float`]'s range.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_base::num::conversion::traits::ConvertibleFrom;
    /// use malachite_float::{BigExponentFloat, Float};
    ///
    /// assert!(Float::convertible_from(&BigExponentFloat::ONE));
    /// ```
    fn convertible_from(x: &BigExponentFloat) -> bool {
        match x {
            BigExponentFloat(Finite { exponent, .. }) => i32::try_from(exponent)
                .is_ok_and(|e| (Self::MIN_EXPONENT..=Self::MAX_EXPONENT).contains(&e)),
            _ => true,
        }
    }
}

impl Float {
    /// Converts a [`BigExponentFloat`] to a [`Float`], rounding to a given precision with the
    /// specified rounding mode. The [`BigExponentFloat`] is taken by value. An [`Ordering`] is also
    /// returned, indicating whether the returned value is less than, equal to, or greater than the
    /// original value.
    ///
    /// Overflow and underflow are handled as in [`Float::from_rational_prec_round`]:
    /// - If the [`BigExponentFloat`] rounds to a value greater than or equal to $2^{2^{30}-1}$,
    ///   this function overflows to $\infty$ if `rm` is `Ceiling`, `Up`, or `Nearest`, and rounds
    ///   down to $(1-(1/2)^p)2^{2^{30}-1}$ otherwise, where $p$ is `prec`.
    /// - If the [`BigExponentFloat`] rounds to a value less than or equal to $-2^{2^{30}-1}$, this
    ///   function overflows to $-\infty$ if `rm` is `Floor`, `Up`, or `Nearest`, and rounds up to
    ///   $-(1-(1/2)^p)2^{2^{30}-1}$ otherwise.
    /// - If the [`BigExponentFloat`] rounds to a positive value less than $2^{-2^{30}}$, this
    ///   function underflows to positive zero if `rm` is `Floor` or `Down`, rounds up to
    ///   $2^{-2^{30}}$ if `rm` is `Ceiling` or `Up`, underflows to positive zero if `rm` is
    ///   `Nearest` and the [`BigExponentFloat`] rounds to a value less than or equal to
    ///   $2^{-2^{30}-1}$, and rounds up to $2^{-2^{30}}$ if `rm` is `Nearest` and it rounds to a
    ///   greater value.
    /// - If the [`BigExponentFloat`] rounds to a negative value greater than $-2^{-2^{30}}$, this
    ///   function behaves symmetrically, underflowing to negative zero.
    ///
    /// NaN, $\pm\infty$, and zero become the corresponding [`Float`]s, with `Equal`; zero becomes
    /// positive zero.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(x.complexity(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the [`BigExponentFloat`] cannot be
    /// represented exactly with the given precision.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::{BigExponentFloat, Float};
    /// use std::cmp::Ordering::*;
    ///
    /// let x = BigExponentFloat::from(Float::from(1.75));
    /// let (y, o) = Float::from_big_exponent_float_prec_round(x, 2, Floor);
    /// assert_eq!(y, 1.5);
    /// assert_eq!(o, Less);
    /// ```
    pub fn from_big_exponent_float_prec_round(
        x: BigExponentFloat,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        assert_ne!(prec, 0);
        let sign = match &x {
            BigExponentFloat(Finite { sign, .. }) => *sign,
            _ => return (Self::try_from(x).unwrap(), Equal),
        };
        let (y, o) = x.round_to_prec_round(prec, rm);
        Self::from_rounded_big_exponent_float(&y, o, sign, prec, rm)
    }

    /// Converts a [`BigExponentFloat`] to a [`Float`], rounding to a given precision with the
    /// specified rounding mode. The [`BigExponentFloat`] is taken by reference. An [`Ordering`] is
    /// also returned, indicating whether the returned value is less than, equal to, or greater than
    /// the original value.
    ///
    /// See [`Float::from_big_exponent_float_prec_round`] for details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(x.complexity(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the [`BigExponentFloat`] cannot be
    /// represented exactly with the given precision.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::{BigExponentFloat, Float};
    /// use std::cmp::Ordering::*;
    ///
    /// let x = BigExponentFloat::from(Float::from(1.75));
    /// let (y, o) = Float::from_big_exponent_float_prec_round_ref(&x, 2, Ceiling);
    /// assert_eq!(y, 2);
    /// assert_eq!(o, Greater);
    /// ```
    pub fn from_big_exponent_float_prec_round_ref(
        x: &BigExponentFloat,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        assert_ne!(prec, 0);
        let sign = match x {
            BigExponentFloat(Finite { sign, .. }) => *sign,
            _ => return (Self::try_from(x).unwrap(), Equal),
        };
        let (y, o) = x.round_to_prec_round_ref(prec, rm);
        Self::from_rounded_big_exponent_float(&y, o, sign, prec, rm)
    }

    // The `Float` for `y`, the result of rounding a finite nonzero `BigExponentFloat` with sign
    // `sign` to precision `prec` with `rm`, with `o` the direction of that rounding: `y` itself if
    // its exponent is in range, or else the overflow or underflow result.
    fn from_rounded_big_exponent_float(
        y: &BigExponentFloat,
        o: Ordering,
        sign: bool,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        let away_from_0 = if sign { Greater } else { Less };
        match exact_float(y) {
            Ok(mut f) => {
                // `y` has at most `prec` bits, so this only widens the significand.
                f.set_prec(prec);
                (f, o)
            }
            Err(true) => match (sign, rm) {
                (true, Up | Ceiling | Nearest) => (Self::INFINITY, Greater),
                (true, Floor | Down) => (Self::max_finite_value_with_prec(prec), Less),
                (false, Up | Floor | Nearest) => (Self::NEGATIVE_INFINITY, Less),
                (false, Ceiling | Down) => (-Self::max_finite_value_with_prec(prec), Greater),
                (_, Exact) => panic!("Inexact conversion from BigExponentFloat to Float"),
            },
            Err(false) => {
                assert!(
                    rm != Exact,
                    "Inexact conversion from BigExponentFloat to Float"
                );
                let BigExponentFloat(Finite {
                    exponent,
                    significand,
                    ..
                }) = y
                else {
                    unreachable!()
                };
                // `y` lies in [2^(MIN_EXPONENT - 2), 2^(MIN_EXPONENT - 1)), and is more than half
                // of the smallest positive `Float` unless it is exactly half of it, before rounding
                // down to it.
                let more_than_half = *exponent == Self::MIN_EXPONENT_MINUS_1
                    && (o == away_from_0.reverse() || !significand.is_power_of_2());
                if rm == Nearest && more_than_half {
                    if sign {
                        (Self::min_positive_value_prec(prec), Greater)
                    } else {
                        (-Self::min_positive_value_prec(prec), Less)
                    }
                } else {
                    match (sign, rm) {
                        (true, Up | Ceiling) => (Self::min_positive_value_prec(prec), Greater),
                        (true, Floor | Down | Nearest) => (Self::ZERO, Less),
                        (false, Up | Floor) => (-Self::min_positive_value_prec(prec), Less),
                        (false, Ceiling | Down | Nearest) => (Self::NEGATIVE_ZERO, Greater),
                        (_, Exact) => unreachable!(),
                    }
                }
            }
        }
    }

    /// Converts a [`BigExponentFloat`] to a [`Float`], rounding to the nearest value of a given
    /// precision. The [`BigExponentFloat`] is taken by value. An [`Ordering`] is also returned,
    /// indicating whether the returned value is less than, equal to, or greater than the original
    /// value.
    ///
    /// See [`Float::from_big_exponent_float_prec_round`] for details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(x.complexity(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::{BigExponentFloat, Float};
    /// use std::cmp::Ordering::*;
    ///
    /// let x = BigExponentFloat::from(Float::from(1.75));
    /// let (y, o) = Float::from_big_exponent_float_prec(x, 2);
    /// assert_eq!(y, 2);
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn from_big_exponent_float_prec(x: BigExponentFloat, prec: u64) -> (Self, Ordering) {
        Self::from_big_exponent_float_prec_round(x, prec, Nearest)
    }

    /// Converts a [`BigExponentFloat`] to a [`Float`], rounding to the nearest value of a given
    /// precision. The [`BigExponentFloat`] is taken by reference. An [`Ordering`] is also returned,
    /// indicating whether the returned value is less than, equal to, or greater than the original
    /// value.
    ///
    /// See [`Float::from_big_exponent_float_prec_round`] for details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(x.complexity(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::{BigExponentFloat, Float};
    /// use std::cmp::Ordering::*;
    ///
    /// let x = BigExponentFloat::from(Float::from(1.75));
    /// let (y, o) = Float::from_big_exponent_float_prec_ref(&x, 2);
    /// assert_eq!(y, 2);
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn from_big_exponent_float_prec_ref(x: &BigExponentFloat, prec: u64) -> (Self, Ordering) {
        Self::from_big_exponent_float_prec_round_ref(x, prec, Nearest)
    }
}
