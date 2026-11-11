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
use malachite_base::num::comparison::traits::PartialOrdAbs;
use malachite_nz::natural::Natural;

impl PartialOrdAbs<Natural> for BigExponentFloat {
    /// Compares the absolute value of a [`BigExponentFloat`] to a [`Natural`].
    ///
    /// NaN is not comparable to any [`Natural`]. $\infty$ and $-\infty$ are greater than any
    /// [`Natural`] in absolute value. The [`BigExponentFloat`] zero is equal to the [`Natural`]
    /// zero.
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
    /// use malachite_base::num::basic::traits::NegativeInfinity;
    /// use malachite_base::num::comparison::traits::PartialOrdAbs;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert!(BigExponentFloat::from(80).lt_abs(&Natural::from(100u32)));
    /// assert!(BigExponentFloat::from(-120).gt_abs(&Natural::from(100u32)));
    /// assert!(BigExponentFloat::NEGATIVE_INFINITY.gt_abs(&Natural::from(100u32)));
    /// ```
    fn partial_cmp_abs(&self, other: &Natural) -> Option<Ordering> {
        match self {
            Self(NaN) => None,
            Self(Infinity { .. }) => Some(Greater),
            Self(Zero) => Some(other.sign().reverse()),
            Self(Finite {
                exponent,
                significand,
                ..
            }) => Some(finite_cmp_abs_natural(exponent, significand, other)),
        }
    }
}

impl PartialOrdAbs<BigExponentFloat> for Natural {
    /// Compares a [`Natural`] to the absolute value of a [`BigExponentFloat`].
    ///
    /// No [`Natural`] is comparable to NaN. Every [`Natural`] is smaller than $\infty$ and
    /// $-\infty$ in absolute value. The [`Natural`] zero is equal to the [`BigExponentFloat`] zero.
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
    /// use malachite_base::num::basic::traits::NegativeInfinity;
    /// use malachite_base::num::comparison::traits::PartialOrdAbs;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert!(Natural::from(100u32).gt_abs(&BigExponentFloat::from(80)));
    /// assert!(Natural::from(100u32).lt_abs(&BigExponentFloat::from(-120)));
    /// assert!(Natural::from(100u32).lt_abs(&BigExponentFloat::NEGATIVE_INFINITY));
    /// ```
    #[inline]
    fn partial_cmp_abs(&self, other: &BigExponentFloat) -> Option<Ordering> {
        other.partial_cmp_abs(self).map(Ordering::reverse)
    }
}
