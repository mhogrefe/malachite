// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::big_exponent_float::BigExponentFloat;
use crate::big_exponent_float::InnerBigExponentFloat::{Finite, Infinity, NaN, Zero};
use crate::big_exponent_float::comparison::partial_cmp_natural::finite_cmp_abs_natural;
use core::cmp::Ordering::{self, *};
use malachite_base::num::arithmetic::traits::Sign;
use malachite_nz::integer::Integer;

impl PartialOrd<Integer> for BigExponentFloat {
    /// Compares a [`BigExponentFloat`] to an [`Integer`].
    ///
    /// NaN is not comparable to any [`Integer`]. $\infty$ is greater than any [`Integer`], and
    /// $-\infty$ is less. The [`BigExponentFloat`] zero is equal to the [`Integer`] zero.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{Infinity, NegativeInfinity};
    /// use malachite_float::BigExponentFloat;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert!(BigExponentFloat::from(80) < Integer::from(100));
    /// assert!(BigExponentFloat::from(-80) > Integer::from(-100));
    /// assert!(BigExponentFloat::INFINITY > Integer::from(100));
    /// assert!(BigExponentFloat::NEGATIVE_INFINITY < Integer::from(-100));
    /// ```
    fn partial_cmp(&self, other: &Integer) -> Option<Ordering> {
        match self {
            Self(NaN) => None,
            Self(Infinity { sign }) => Some(if *sign { Greater } else { Less }),
            Self(Zero) => Some(other.sign().reverse()),
            Self(Finite {
                sign,
                exponent,
                significand,
            }) => {
                let s_x = if *sign { Greater } else { Less };
                let s_cmp = s_x.cmp(&other.sign());
                if s_cmp != Equal {
                    return Some(s_cmp);
                }
                let abs_cmp =
                    finite_cmp_abs_natural(exponent, significand, other.unsigned_abs_ref());
                Some(if *sign { abs_cmp } else { abs_cmp.reverse() })
            }
        }
    }
}

impl PartialOrd<BigExponentFloat> for Integer {
    /// Compares an [`Integer`] to a [`BigExponentFloat`].
    ///
    /// No [`Integer`] is comparable to NaN. Every [`Integer`] is smaller than $\infty$ and greater
    /// than $-\infty$. The [`Integer`] zero is equal to the [`BigExponentFloat`] zero.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.complexity())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{Infinity, NegativeInfinity};
    /// use malachite_float::BigExponentFloat;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert!(Integer::from(100) > BigExponentFloat::from(80));
    /// assert!(Integer::from(-100) < BigExponentFloat::from(-80));
    /// assert!(Integer::from(-100) < BigExponentFloat::INFINITY);
    /// assert!(Integer::from(-100) > BigExponentFloat::NEGATIVE_INFINITY);
    /// ```
    #[inline]
    fn partial_cmp(&self, other: &BigExponentFloat) -> Option<Ordering> {
        other.partial_cmp(self).map(Ordering::reverse)
    }
}
