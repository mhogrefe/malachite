// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.
use crate::big_exponent_float::BigExponentFloat;
use core::cmp::Ordering::{self, *};
use malachite_base::num::arithmetic::traits::{Abs, CheckedLogBase2, IsPowerOf2};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::conversion::traits::{ConvertibleFrom, RoundingFrom};
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_q::Rational;

/// The error that results when converting a non-dyadic [`Rational`] (one whose denominator is not a
/// power of 2) to a [`BigExponentFloat`] exactly.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BigExponentFloatFromRationalError;

impl BigExponentFloat {
    /// Converts a [`Rational`] to a [`BigExponentFloat`], rounding to a given precision with the
    /// specified rounding mode. The [`Rational`] is taken by value. An [`Ordering`] is also
    /// returned, indicating whether the returned value is less than, equal to, or greater than the
    /// original value.
    ///
    /// The exponent is unbounded, so the result never overflows or underflows.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(x.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the [`Rational`] cannot be represented
    /// exactly with the given precision.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (x, o) =
    ///     BigExponentFloat::from_rational_prec_round(Rational::from_signeds(1, 3), 10, Floor);
    /// assert_eq!(format!("{x:?}"), "0x0.554");
    /// assert_eq!(o, Less);
    /// ```
    pub fn from_rational_prec_round(x: Rational, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        assert_ne!(prec, 0);
        if x == 0u32 {
            return (Self::ZERO, Equal);
        }
        let sign = x > 0u32;
        let e = x.floor_log_base_2_abs();
        Self::from_abs_rational_prec_round(x.abs(), sign, e, prec, rm)
    }

    /// Converts a [`Rational`] to a [`BigExponentFloat`], rounding to a given precision with the
    /// specified rounding mode. The [`Rational`] is taken by reference. An [`Ordering`] is also
    /// returned, indicating whether the returned value is less than, equal to, or greater than the
    /// original value.
    ///
    /// The exponent is unbounded, so the result never overflows or underflows.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(x.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the [`Rational`] cannot be represented
    /// exactly with the given precision.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (x, o) = BigExponentFloat::from_rational_prec_round_ref(
    ///     &Rational::from_signeds(-1, 3),
    ///     10,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{x:?}"), "-0x0.556");
    /// assert_eq!(o, Less);
    /// ```
    pub fn from_rational_prec_round_ref(
        x: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        assert_ne!(prec, 0);
        if *x == 0u32 {
            return (Self::ZERO, Equal);
        }
        let sign = *x > 0u32;
        let e = x.floor_log_base_2_abs();
        Self::from_abs_rational_prec_round(x.abs(), sign, e, prec, rm)
    }

    // The rounding of the nonzero rational with absolute value `abs` and sign `sign` to precision
    // `prec`, where e is the floor of the base-2 logarithm of `abs`.
    fn from_abs_rational_prec_round(
        abs: Rational,
        sign: bool,
        e: i64,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        let abs_rm = if sign { rm } else { -rm };
        // `abs` is in [2^e, 2^(e + 1)), so `abs` * 2^(prec - 1 - e) is in [2^(prec - 1), 2^prec).
        let shift = i64::try_from(i128::from(prec) - 1 - i128::from(e))
            .expect("exponent shift out of range");
        let (m, o) = Natural::rounding_from(abs << shift, abs_rm);
        let bits = m.significant_bits();
        // m * 2^(e + 1 - prec), with m having `bits` bits
        let exponent = Integer::from(e) + Integer::from(bits) + Integer::ONE - Integer::from(prec);
        let o = if sign { o } else { o.reverse() };
        (Self::from_parts(sign, exponent, m), o)
    }

    /// Converts a [`Rational`] to a [`BigExponentFloat`], rounding to the nearest value of a given
    /// precision. The [`Rational`] is taken by value. An [`Ordering`] is also returned, indicating
    /// whether the returned value is less than, equal to, or greater than the original value.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(x.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (x, o) = BigExponentFloat::from_rational_prec(Rational::from_signeds(1, 3), 10);
    /// assert_eq!(format!("{x:?}"), "0x0.556");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn from_rational_prec(x: Rational, prec: u64) -> (Self, Ordering) {
        Self::from_rational_prec_round(x, prec, Nearest)
    }

    /// Converts a [`Rational`] to a [`BigExponentFloat`], rounding to the nearest value of a given
    /// precision. The [`Rational`] is taken by reference. An [`Ordering`] is also returned,
    /// indicating whether the returned value is less than, equal to, or greater than the original
    /// value.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(x.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (x, o) = BigExponentFloat::from_rational_prec_ref(&Rational::from_signeds(1, 3), 10);
    /// assert_eq!(format!("{x:?}"), "0x0.556");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn from_rational_prec_ref(x: &Rational, prec: u64) -> (Self, Ordering) {
        Self::from_rational_prec_round_ref(x, prec, Nearest)
    }
}

impl TryFrom<&Rational> for BigExponentFloat {
    type Error = BigExponentFloatFromRationalError;

    /// Converts a [`Rational`] to a [`BigExponentFloat`] exactly, taking the [`Rational`] by
    /// reference.
    ///
    /// The exponent is unbounded and the precision is not fixed, so every [`Rational`] whose
    /// denominator is a power of 2 is converted exactly; any other [`Rational`] gives an error.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `x.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::BigExponentFloat;
    /// use malachite_float::big_exponent_float::conversion::from_rational::*;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(
    ///     format!(
    ///         "{:?}",
    ///         BigExponentFloat::try_from(&Rational::from_signeds(-3, 8)).unwrap()
    ///     ),
    ///     "-0x0.6"
    /// );
    /// assert_eq!(
    ///     BigExponentFloat::try_from(&Rational::from_signeds(1, 3)).unwrap_err(),
    ///     BigExponentFloatFromRationalError
    /// );
    /// ```
    fn try_from(x: &Rational) -> Result<Self, Self::Error> {
        if *x == 0u32 {
            return Ok(Self::ZERO);
        }
        let pow = x
            .denominator_ref()
            .checked_log_base_2()
            .ok_or(BigExponentFloatFromRationalError)?;
        let n = x.numerator_ref();
        let bits = n.significant_bits();
        Ok(Self::from_parts(
            *x > 0u32,
            Integer::from(bits) - Integer::from(pow),
            n.clone(),
        ))
    }
}

impl TryFrom<Rational> for BigExponentFloat {
    type Error = BigExponentFloatFromRationalError;

    /// Converts a [`Rational`] to a [`BigExponentFloat`] exactly, taking the [`Rational`] by value.
    ///
    /// The exponent is unbounded and the precision is not fixed, so every [`Rational`] whose
    /// denominator is a power of 2 is converted exactly; any other [`Rational`] gives an error.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `x.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(
    ///     format!(
    ///         "{:?}",
    ///         BigExponentFloat::try_from(Rational::from_signeds(3, 8)).unwrap()
    ///     ),
    ///     "0x0.6"
    /// );
    /// assert!(BigExponentFloat::try_from(Rational::from_signeds(1, 3)).is_err());
    /// ```
    #[inline]
    fn try_from(x: Rational) -> Result<Self, Self::Error> {
        Self::try_from(&x)
    }
}

impl ConvertibleFrom<&Rational> for BigExponentFloat {
    /// Determines whether a [`Rational`] can be converted to a [`BigExponentFloat`] exactly: that
    /// is, whether its denominator is a power of 2.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `x.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::conversion::traits::ConvertibleFrom;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    ///
    /// assert!(BigExponentFloat::convertible_from(&Rational::from_signeds(
    ///     3, 8
    /// )));
    /// assert!(!BigExponentFloat::convertible_from(
    ///     &Rational::from_signeds(1, 3)
    /// ));
    /// ```
    #[inline]
    fn convertible_from(x: &Rational) -> bool {
        x.denominator_ref().is_power_of_2()
    }
}
