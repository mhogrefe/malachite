// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::big_exponent_float::BigExponentFloat;
use crate::big_exponent_float::InnerBigExponentFloat::{Finite, Infinity, NaN, Zero};
use crate::{Float, InnerFloat};
use core::cmp::Ordering::{self, *};
use malachite_base::num::comparison::traits::PartialOrdAbs;

impl PartialOrdAbs<Float> for BigExponentFloat {
    /// Compares the absolute values of a [`BigExponentFloat`] and a [`Float`].
    ///
    /// This implementation follows the IEEE 754 standard: NaN is not comparable to anything. A
    /// [`BigExponentFloat`] has no stored precision and a [`Float`] does; values with different
    /// precisions are equal if they represent the same number. Both [`Float`] zeros are equal to
    /// the [`BigExponentFloat`] zero.
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
    /// use malachite_base::num::basic::traits::{
    ///     NaN, NegativeInfinity, NegativeOne, One, OneHalf,
    /// };
    /// use malachite_base::num::comparison::traits::PartialOrdAbs;
    /// use malachite_float::{BigExponentFloat, Float};
    ///
    /// assert!(BigExponentFloat::ONE_HALF.lt_abs(&Float::NEGATIVE_ONE));
    /// assert!(BigExponentFloat::NEGATIVE_INFINITY.gt_abs(&Float::max_finite_value_with_prec(10)));
    /// assert_eq!(BigExponentFloat::NAN.partial_cmp_abs(&Float::ONE), None);
    /// ```
    fn partial_cmp_abs(&self, other: &Float) -> Option<Ordering> {
        match (self, other) {
            (Self(NaN), _) | (_, Float(InnerFloat::NaN)) => None,
            (Self(Infinity { .. }), Float(InnerFloat::Infinity { .. }))
            | (Self(Zero), Float(InnerFloat::Zero { .. })) => Some(Equal),
            (Self(Infinity { .. }), _) | (_, Float(InnerFloat::Zero { .. })) => Some(Greater),
            (_, Float(InnerFloat::Infinity { .. })) | (Self(Zero), _) => Some(Less),
            (
                Self(Finite {
                    exponent: e_x,
                    significand: x,
                    ..
                }),
                Float(InnerFloat::Finite {
                    exponent: e_y,
                    significand: y,
                    ..
                }),
            ) => Some(
                e_x.partial_cmp(e_y)
                    .unwrap()
                    .then_with(|| x.cmp_normalized_no_shift(y)),
            ),
        }
    }
}

impl PartialOrdAbs<BigExponentFloat> for Float {
    /// Compares the absolute values of a [`Float`] and a [`BigExponentFloat`].
    ///
    /// This implementation follows the IEEE 754 standard: NaN is not comparable to anything. A
    /// [`BigExponentFloat`] has no stored precision and a [`Float`] does; values with different
    /// precisions are equal if they represent the same number. Both [`Float`] zeros are equal to
    /// the [`BigExponentFloat`] zero.
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
    /// use malachite_base::num::basic::traits::{
    ///     NaN, NegativeInfinity, NegativeOne, One, OneHalf,
    /// };
    /// use malachite_base::num::comparison::traits::PartialOrdAbs;
    /// use malachite_float::{BigExponentFloat, Float};
    ///
    /// assert!(Float::NEGATIVE_ONE.gt_abs(&BigExponentFloat::ONE_HALF));
    /// assert!(Float::max_finite_value_with_prec(10).lt_abs(&BigExponentFloat::NEGATIVE_INFINITY));
    /// assert_eq!(Float::ONE.partial_cmp_abs(&BigExponentFloat::NAN), None);
    /// ```
    #[inline]
    fn partial_cmp_abs(&self, other: &BigExponentFloat) -> Option<Ordering> {
        other.partial_cmp_abs(self).map(Ordering::reverse)
    }
}
