// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use core::cmp::Ordering::{self, *};
use malachite_base::num::arithmetic::traits::Sign;
use malachite_base::num::comparison::traits::{OrdAbs, PartialOrdAbs};

impl PartialOrdAbs for Rational {
    /// Compares the absolute values of two [`Rational`]s.
    ///
    /// See the documentation for the [`OrdAbs`] implementation.
    #[inline]
    fn partial_cmp_abs(&self, other: &Rational) -> Option<Ordering> {
        Some(self.cmp_abs(other))
    }
}

impl OrdAbs for Rational {
    /// Compares the absolute values of two [`Rational`]s.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::OneHalf;
    /// use malachite_base::num::comparison::traits::OrdAbs;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(
    ///     Rational::from_str("2/3")
    ///         .unwrap()
    ///         .cmp_abs(&Rational::ONE_HALF),
    ///     Greater
    /// );
    /// assert_eq!(
    ///     Rational::from_str("-2/3")
    ///         .unwrap()
    ///         .cmp_abs(&Rational::ONE_HALF),
    ///     Greater
    /// );
    /// ```
    fn cmp_abs(&self, other: &Rational) -> Ordering {
        if core::ptr::eq(self, other) {
            return Equal;
        }
        // First check if either value is zero
        let self_sign = self.numerator_ref().sign();
        let other_sign = other.numerator_ref().sign();
        let sign_cmp = self_sign.cmp(&other_sign);
        if sign_cmp != Equal || self_sign == Equal {
            return sign_cmp;
        }
        // Then check if one is < 1 and the other is > 1
        let self_cmp_one = self.numerator.cmp(&self.denominator);
        let other_cmp_one = other.numerator.cmp(&other.denominator);
        let one_cmp = self_cmp_one.cmp(&other_cmp_one);
        if one_cmp != Equal {
            return one_cmp;
        }
        // Then compare numerators and denominators
        let n_cmp = self.numerator.cmp(&other.numerator);
        let d_cmp = self.denominator.cmp(&other.denominator);
        if n_cmp == Equal && d_cmp == Equal {
            return Equal;
        }
        let nd_cmp = n_cmp.cmp(&d_cmp);
        if nd_cmp != Equal {
            return nd_cmp;
        }
        // Then compare floor ∘ log_2 ∘ abs
        let log_cmp = self
            .floor_log_base_2_abs()
            .cmp(&other.floor_log_base_2_abs());
        if log_cmp != Equal {
            return log_cmp;
        }
        // Finally, cross-multiply.
        (&self.numerator * &other.denominator).cmp(&(&self.denominator * &other.numerator))
    }
}
