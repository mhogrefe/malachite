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
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_nz::natural::Natural;

impl PartialOrd<Natural> for Rational {
    /// Compares a [`Rational`] to a [`Natural`].
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
    /// use malachite_nz::natural::Natural;
    /// use malachite_q::Rational;
    ///
    /// assert!(Rational::from_signeds(22, 7) > Natural::from(3u32));
    /// assert!(Rational::from_signeds(22, 7) < Natural::from(4u32));
    /// ```
    fn partial_cmp(&self, other: &Natural) -> Option<Ordering> {
        // First check signs
        let self_sign = self.sign();
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
        let log_cmp = self
            .floor_log_base_2_abs()
            .cmp(&i64::exact_from(other.significant_bits() - 1));
        if log_cmp != Equal {
            return Some(if self.sign {
                log_cmp
            } else {
                log_cmp.reverse()
            });
        }
        // Finally, cross-multiply.
        Some(self.numerator.cmp(&(&self.denominator * other)))
    }
}

impl PartialOrd<Rational> for Natural {
    /// Compares a [`Natural`] to a [`Rational`].
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
    /// use malachite_nz::natural::Natural;
    /// use malachite_q::Rational;
    ///
    /// assert!(Natural::from(3u32) < Rational::from_signeds(22, 7));
    /// assert!(Natural::from(4u32) > Rational::from_signeds(22, 7));
    /// ```
    #[inline]
    fn partial_cmp(&self, other: &Rational) -> Option<Ordering> {
        other.partial_cmp(self).map(Ordering::reverse)
    }
}
