// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::big_exponent_float::BigExponentFloat;
use core::cmp::Ordering::{self, *};
use malachite_base::num::arithmetic::traits::AbsSquared;
use malachite_base::num::comparison::traits::PartialOrdAbs;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_nz::gaussian_integer::GaussianInteger;
use malachite_q::Rational;

impl PartialOrdAbs<GaussianInteger> for BigExponentFloat {
    /// Compares the absolute value of a [`BigExponentFloat`] to the modulus of a
    /// [`GaussianInteger`].
    ///
    /// NaN is not comparable to any [`GaussianInteger`]. $\infty$ and $-\infty$ are greater than
    /// any [`GaussianInteger`] in absolute value. The comparison is exact, whatever the size of the
    /// [`BigExponentFloat`]'s exponent.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::comparison::traits::{EqAbs, PartialOrdAbs};
    /// use malachite_float::BigExponentFloat;
    /// use malachite_nz::gaussian_integer::GaussianInteger;
    /// use std::str::FromStr;
    ///
    /// assert!(BigExponentFloat::from(-5).eq_abs(&GaussianInteger::from_str("3+4i").unwrap()));
    /// assert!(BigExponentFloat::from(4.9).lt_abs(&GaussianInteger::from_str("3+4i").unwrap()));
    /// assert!(BigExponentFloat::from(5.1).gt_abs(&GaussianInteger::from_str("-3+4i").unwrap()));
    /// ```
    fn partial_cmp_abs(&self, other: &GaussianInteger) -> Option<Ordering> {
        if self.is_nan() {
            None
        } else if !self.is_finite() {
            Some(Greater)
        } else if other.imaginary == 0u32 {
            self.partial_cmp_abs(&other.real)
        } else if other.real == 0u32 {
            self.partial_cmp_abs(&other.imaginary)
        } else if !self.gt_abs(&other.real) || !self.gt_abs(&other.imaginary) {
            Some(Less)
        } else {
            // Now `self` is finite and larger than both parts in absolute value, so its exponent is
            // bounded below. With |re|, |im| < 2^b, the modulus is less than 2^(b + 1); if `self`
            // has an exponent above b + 1, it is at least 2^(b + 1) and wins. Otherwise its
            // exponent is bounded above as well, and the squares are compared exactly.
            let b = core::cmp::max(
                other.real.significant_bits(),
                other.imaginary.significant_bits(),
            );
            if *self.get_exponent().unwrap() > b + 1 {
                Some(Greater)
            } else {
                Some(
                    Rational::exact_from(self)
                        .abs_squared()
                        .cmp(&Rational::from(other.abs_squared())),
                )
            }
        }
    }
}

impl PartialOrdAbs<BigExponentFloat> for GaussianInteger {
    /// Compares the modulus of a [`GaussianInteger`] to the absolute value of a
    /// [`BigExponentFloat`].
    ///
    /// No [`GaussianInteger`] is comparable to NaN. Every [`GaussianInteger`] is smaller than
    /// $\infty$ and $-\infty$ in absolute value. The comparison is exact, whatever the size of the
    /// [`BigExponentFloat`]'s exponent.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.complexity())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::comparison::traits::{EqAbs, PartialOrdAbs};
    /// use malachite_float::BigExponentFloat;
    /// use malachite_nz::gaussian_integer::GaussianInteger;
    /// use std::str::FromStr;
    ///
    /// assert!(
    ///     GaussianInteger::from_str("3+4i")
    ///         .unwrap()
    ///         .eq_abs(&BigExponentFloat::from(-5))
    /// );
    /// assert!(
    ///     GaussianInteger::from_str("3+4i")
    ///         .unwrap()
    ///         .gt_abs(&BigExponentFloat::from(4.9))
    /// );
    /// assert!(
    ///     GaussianInteger::from_str("-3+4i")
    ///         .unwrap()
    ///         .lt_abs(&BigExponentFloat::from(5.1))
    /// );
    /// ```
    #[inline]
    fn partial_cmp_abs(&self, other: &BigExponentFloat) -> Option<Ordering> {
        other.partial_cmp_abs(self).map(Ordering::reverse)
    }
}
