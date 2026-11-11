// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::big_exponent_float::BigExponentFloat;
use crate::big_exponent_float::InnerBigExponentFloat::{Finite, Infinity, NaN, Zero};
use core::cmp::Ordering::{self, *};
use malachite_base::num::arithmetic::traits::Sign;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;

// Compares the absolute value of a finite nonzero `BigExponentFloat`, given by its exponent and
// significand, with a `Natural`.
pub(crate) fn finite_cmp_abs_natural(
    exponent: &Integer,
    significand: &Natural,
    other: &Natural,
) -> Ordering {
    if *other == 0u32 {
        Greater
    } else if *exponent <= 0u32 {
        Less
    } else {
        exponent
            .partial_cmp(&other.significant_bits())
            .unwrap()
            .then_with(|| significand.cmp_normalized(other))
    }
}

impl PartialOrd<Natural> for BigExponentFloat {
    /// Compares a [`BigExponentFloat`] to a [`Natural`].
    ///
    /// NaN is not comparable to any [`Natural`]. $\infty$ is greater than any [`Natural`], and
    /// $-\infty$ is less. The [`BigExponentFloat`] zero is equal to the [`Natural`] zero.
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
    /// use malachite_base::num::basic::traits::{Infinity, NegativeInfinity, OneHalf, Zero};
    /// use malachite_float::BigExponentFloat;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert!(BigExponentFloat::from(80) < Natural::from(100u32));
    /// assert!(BigExponentFloat::INFINITY > Natural::from(100u32));
    /// assert!(BigExponentFloat::NEGATIVE_INFINITY < Natural::from(100u32));
    /// assert!(BigExponentFloat::ONE_HALF > Natural::ZERO);
    /// ```
    fn partial_cmp(&self, other: &Natural) -> Option<Ordering> {
        match self {
            Self(NaN) => None,
            Self(Infinity { sign }) => Some(if *sign { Greater } else { Less }),
            Self(Zero) => Some(other.sign().reverse()),
            Self(Finite {
                sign,
                exponent,
                significand,
            }) => Some(if *sign {
                finite_cmp_abs_natural(exponent, significand, other)
            } else {
                Less
            }),
        }
    }
}

impl PartialOrd<BigExponentFloat> for Natural {
    /// Compares a [`Natural`] to a [`BigExponentFloat`].
    ///
    /// No [`Natural`] is comparable to NaN. Every [`Natural`] is smaller than $\infty$ and greater
    /// than $-\infty$. The [`Natural`] zero is equal to the [`BigExponentFloat`] zero.
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
    /// use malachite_base::num::basic::traits::{Infinity, NegativeInfinity, OneHalf, Zero};
    /// use malachite_float::BigExponentFloat;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert!(Natural::from(100u32) > BigExponentFloat::from(80));
    /// assert!(Natural::from(100u32) < BigExponentFloat::INFINITY);
    /// assert!(Natural::from(100u32) > BigExponentFloat::NEGATIVE_INFINITY);
    /// assert!(Natural::ZERO < BigExponentFloat::ONE_HALF);
    /// ```
    #[inline]
    fn partial_cmp(&self, other: &BigExponentFloat) -> Option<Ordering> {
        other.partial_cmp(self).map(Ordering::reverse)
    }
}
