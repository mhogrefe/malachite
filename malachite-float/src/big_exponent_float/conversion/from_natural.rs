// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::big_exponent_float::BigExponentFloat;
use core::cmp::Ordering;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;

impl From<Natural> for BigExponentFloat {
    /// Converts a [`Natural`] to a [`BigExponentFloat`], taking the [`Natural`] by value.
    ///
    /// The conversion is exact: the exponent is unbounded, so every [`Natural`] is representable.
    /// The result's precision is the number of bits from the most significant 1 of the
    /// [`Natural`]'s absolute value to its least significant 1.
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
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     format!("{:?}", BigExponentFloat::from(Natural::ZERO)),
    ///     "0x0.0"
    /// );
    /// assert_eq!(
    ///     format!("{:?}", BigExponentFloat::from(Natural::from(12u32))),
    ///     "0xc.0"
    /// );
    /// ```
    fn from(x: Natural) -> Self {
        if x == 0u32 {
            Self::ZERO
        } else {
            let bits = x.significant_bits();
            Self::from_parts(true, Integer::from(bits), x)
        }
    }
}

impl From<&Natural> for BigExponentFloat {
    /// Converts a [`Natural`] to a [`BigExponentFloat`], taking the [`Natural`] by reference.
    ///
    /// The conversion is exact: the exponent is unbounded, so every [`Natural`] is representable.
    /// The result's precision is the number of bits from the most significant 1 of the
    /// [`Natural`]'s absolute value to its least significant 1.
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
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     format!("{:?}", BigExponentFloat::from(&Natural::ZERO)),
    ///     "0x0.0"
    /// );
    /// assert_eq!(
    ///     format!("{:?}", BigExponentFloat::from(&Natural::from(12u32))),
    ///     "0xc.0"
    /// );
    /// ```
    #[inline]
    fn from(x: &Natural) -> Self {
        Self::from(x.clone())
    }
}

impl BigExponentFloat {
    /// Converts a [`Natural`] to a [`BigExponentFloat`], rounding to a given precision with the
    /// specified rounding mode. The [`Natural`] is taken by value. An [`Ordering`] is also
    /// returned, indicating whether the returned value is less than, equal to, or greater than the
    /// original value.
    ///
    /// The exponent is unbounded, so the result never overflows.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `x.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the [`Natural`] cannot be represented
    /// exactly with the given precision.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_nz::natural::Natural;
    /// use std::cmp::Ordering::*;
    ///
    /// let (x, o) = BigExponentFloat::from_natural_prec_round(Natural::from(123u32), 4, Floor);
    /// assert_eq!(format!("{x:?}"), "0x78.0");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn from_natural_prec_round(x: Natural, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        Self::from(x).round_to_prec_round(prec, rm)
    }

    /// Converts a [`Natural`] to a [`BigExponentFloat`], rounding to a given precision with the
    /// specified rounding mode. The [`Natural`] is taken by reference. An [`Ordering`] is also
    /// returned, indicating whether the returned value is less than, equal to, or greater than the
    /// original value.
    ///
    /// The exponent is unbounded, so the result never overflows.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `x.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the [`Natural`] cannot be represented
    /// exactly with the given precision.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_nz::natural::Natural;
    /// use std::cmp::Ordering::*;
    ///
    /// let (x, o) =
    ///     BigExponentFloat::from_natural_prec_round_ref(&Natural::from(123u32), 4, Ceiling);
    /// assert_eq!(format!("{x:?}"), "0x8.0E+1");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn from_natural_prec_round_ref(
        x: &Natural,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        Self::from(x).round_to_prec_round(prec, rm)
    }

    /// Converts a [`Natural`] to a [`BigExponentFloat`], rounding to the nearest value of a given
    /// precision. The [`Natural`] is taken by value. An [`Ordering`] is also returned, indicating
    /// whether the returned value is less than, equal to, or greater than the original value.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `x.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::BigExponentFloat;
    /// use malachite_nz::natural::Natural;
    /// use std::cmp::Ordering::*;
    ///
    /// let (x, o) = BigExponentFloat::from_natural_prec(Natural::from(123u32), 4);
    /// assert_eq!(format!("{x:?}"), "0x78.0");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn from_natural_prec(x: Natural, prec: u64) -> (Self, Ordering) {
        Self::from_natural_prec_round(x, prec, Nearest)
    }

    /// Converts a [`Natural`] to a [`BigExponentFloat`], rounding to the nearest value of a given
    /// precision. The [`Natural`] is taken by reference. An [`Ordering`] is also returned,
    /// indicating whether the returned value is less than, equal to, or greater than the original
    /// value.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `x.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::BigExponentFloat;
    /// use malachite_nz::natural::Natural;
    /// use std::cmp::Ordering::*;
    ///
    /// let (x, o) = BigExponentFloat::from_natural_prec_ref(&Natural::from(123u32), 4);
    /// assert_eq!(format!("{x:?}"), "0x78.0");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn from_natural_prec_ref(x: &Natural, prec: u64) -> (Self, Ordering) {
        Self::from_natural_prec_round_ref(x, prec, Nearest)
    }
}
