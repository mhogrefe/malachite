// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::big_exponent_float::BigExponentFloat;
use crate::big_exponent_float::InnerBigExponentFloat::{Finite, Infinity, NaN, Zero};
use crate::big_exponent_float::comparison::partial_cmp_rational::finite_cmp_abs_nonzero_rational;
use core::cmp::Ordering::{self, *};
use malachite_base::num::comparison::traits::PartialOrdAbs;
use malachite_q::Rational;

impl PartialOrdAbs<Rational> for BigExponentFloat {
    /// Compares the absolute values of a [`BigExponentFloat`] and a [`Rational`].
    ///
    /// NaN is not comparable to any [`Rational`]. $\infty$ and $-\infty$ are greater than any
    /// [`Rational`] in absolute value. The [`BigExponentFloat`] zero is equal to the [`Rational`]
    /// zero. The comparison is exact, whatever the size of the [`BigExponentFloat`]'s exponent.
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
    /// use malachite_base::num::basic::traits::NegativeInfinity;
    /// use malachite_base::num::comparison::traits::PartialOrdAbs;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    ///
    /// assert!(BigExponentFloat::from(2).gt_abs(&Rational::from_signeds(-3, 2)));
    /// assert!(BigExponentFloat::from(-1).lt_abs(&Rational::from_signeds(3, 2)));
    /// assert!(BigExponentFloat::NEGATIVE_INFINITY.gt_abs(&Rational::from(100)));
    /// ```
    fn partial_cmp_abs(&self, other: &Rational) -> Option<Ordering> {
        match self {
            Self(NaN) => None,
            Self(Infinity { .. }) => Some(Greater),
            Self(Zero) => Some(if *other == 0u32 { Equal } else { Less }),
            Self(Finite {
                exponent,
                significand,
                ..
            }) => Some(if *other == 0u32 {
                Greater
            } else {
                finite_cmp_abs_nonzero_rational(exponent, significand, other)
            }),
        }
    }
}

impl PartialOrdAbs<BigExponentFloat> for Rational {
    /// Compares the absolute values of a [`Rational`] and a [`BigExponentFloat`].
    ///
    /// No [`Rational`] is comparable to NaN. Every [`Rational`] is smaller than $\infty$ and
    /// $-\infty$ in absolute value. The [`Rational`] zero is equal to the [`BigExponentFloat`]
    /// zero. The comparison is exact, whatever the size of the [`BigExponentFloat`]'s exponent.
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
    /// use malachite_base::num::basic::traits::NegativeInfinity;
    /// use malachite_base::num::comparison::traits::PartialOrdAbs;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    ///
    /// assert!(Rational::from_signeds(-3, 2).lt_abs(&BigExponentFloat::from(2)));
    /// assert!(Rational::from_signeds(3, 2).gt_abs(&BigExponentFloat::from(-1)));
    /// assert!(Rational::from(100).lt_abs(&BigExponentFloat::NEGATIVE_INFINITY));
    /// ```
    #[inline]
    fn partial_cmp_abs(&self, other: &BigExponentFloat) -> Option<Ordering> {
        other.partial_cmp_abs(self).map(Ordering::reverse)
    }
}
