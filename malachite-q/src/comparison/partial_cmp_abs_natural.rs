// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use core::cmp::Ordering::{self, *};
use malachite_base::num::arithmetic::traits::Sign;
use malachite_base::num::basic::traits::One;
use malachite_base::num::comparison::traits::PartialOrdAbs;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_nz::natural::Natural;

impl PartialOrdAbs<Natural> for Rational {
    /// Compares the absolute values of a [`Rational`] and a [`Natural`].
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
    /// use malachite_base::num::comparison::traits::PartialOrdAbs;
    /// use malachite_nz::natural::Natural;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// assert_eq!(
    ///     Rational::from_signeds(22, 7).partial_cmp_abs(&Natural::from(3u32)),
    ///     Some(Greater)
    /// );
    /// assert_eq!(
    ///     Rational::from_signeds(-22, 7).partial_cmp_abs(&Natural::from(3u32)),
    ///     Some(Greater)
    /// );
    /// ```
    fn partial_cmp_abs(&self, other: &Natural) -> Option<Ordering> {
        // First check if either value is zero
        let self_sign = self.numerator_ref().sign();
        let other_sign = other.sign();
        let sign_cmp = self_sign.cmp(&other_sign);
        if sign_cmp != Equal || self_sign == Equal {
            return Some(sign_cmp);
        }
        // Then check if one is < 1 and the other is > 1
        let self_cmp_one = self.numerator.cmp(&self.denominator);
        let other_cmp_one = other.cmp(&Natural::ONE);
        let one_cmp = self_cmp_one.cmp(&other_cmp_one);
        if one_cmp != Equal {
            return Some(one_cmp);
        }
        // Then compare numerators and denominators
        let n_cmp = self.numerator.cmp(other);
        let d_cmp = self.denominator.cmp(&Natural::ONE);
        if n_cmp == Equal && d_cmp == Equal {
            return Some(Equal);
        }
        let nd_cmp = n_cmp.cmp(&d_cmp);
        if nd_cmp != Equal {
            return Some(nd_cmp);
        }
        // Then compare floor ∘ log_2 ∘ abs
        let log_cmp = self
            .floor_log_base_2_abs()
            .cmp(&i64::exact_from(other.significant_bits() - 1));
        if log_cmp != Equal {
            return Some(log_cmp);
        }
        // Finally, cross-multiply.
        Some(self.numerator.cmp(&(&self.denominator * other)))
    }
}

impl PartialOrdAbs<Rational> for Natural {
    /// Compares the absolute values of a [`Natural`] and a [`Rational`].
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
    /// use malachite_base::num::comparison::traits::PartialOrdAbs;
    /// use malachite_nz::natural::Natural;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// assert_eq!(
    ///     Natural::from(3u32).partial_cmp_abs(&Rational::from_signeds(22, 7)),
    ///     Some(Less)
    /// );
    /// assert_eq!(
    ///     Natural::from(3u32).partial_cmp_abs(&Rational::from_signeds(-22, 7)),
    ///     Some(Less)
    /// );
    /// ```
    #[inline]
    fn partial_cmp_abs(&self, other: &Rational) -> Option<Ordering> {
        other.partial_cmp_abs(self).map(Ordering::reverse)
    }
}
