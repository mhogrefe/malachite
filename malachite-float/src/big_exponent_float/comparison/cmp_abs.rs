// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::big_exponent_float::InnerBigExponentFloat::{Finite, Infinity, NaN, Zero};
use crate::big_exponent_float::{
    BigExponentFloat, ComparableBigExponentFloat, ComparableBigExponentFloatRef,
};
use core::cmp::Ordering::{self, *};
use malachite_base::num::comparison::traits::{OrdAbs, PartialOrdAbs};

impl PartialOrdAbs for BigExponentFloat {
    /// Compares the absolute values of two [`BigExponentFloat`]s.
    ///
    /// This implementation follows the IEEE 754 standard. NaN is not comparable to anything, not
    /// even itself.
    ///
    /// For different comparison behavior that provides a total order, consider using
    /// [`ComparableBigExponentFloat`] or [`ComparableBigExponentFloatRef`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.complexity())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{
    ///     Infinity, NaN, NegativeInfinity, NegativeOne, One, OneHalf, Zero,
    /// };
    /// use malachite_base::num::comparison::traits::{EqAbs, PartialOrdAbs};
    /// use malachite_float::BigExponentFloat;
    ///
    /// assert_eq!(
    ///     BigExponentFloat::NAN.partial_cmp_abs(&BigExponentFloat::NAN),
    ///     None
    /// );
    /// assert!(BigExponentFloat::INFINITY.eq_abs(&BigExponentFloat::NEGATIVE_INFINITY));
    /// assert!(BigExponentFloat::NEGATIVE_INFINITY.gt_abs(&BigExponentFloat::ONE));
    /// assert!(BigExponentFloat::ONE_HALF.lt_abs(&BigExponentFloat::NEGATIVE_ONE));
    /// assert!(BigExponentFloat::ZERO.lt_abs(&BigExponentFloat::ONE_HALF));
    /// ```
    fn partial_cmp_abs(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Self(NaN), _) | (_, Self(NaN)) => None,
            (Self(Infinity { .. }), Self(Infinity { .. })) | (Self(Zero), Self(Zero)) => {
                Some(Equal)
            }
            (Self(Infinity { .. }), _) | (_, Self(Zero)) => Some(Greater),
            (_, Self(Infinity { .. })) | (Self(Zero), _) => Some(Less),
            (
                Self(Finite {
                    exponent: e_x,
                    significand: x,
                    ..
                }),
                Self(Finite {
                    exponent: e_y,
                    significand: y,
                    ..
                }),
            ) => Some(e_x.cmp(e_y).then_with(|| x.cmp_normalized_no_shift(y))),
        }
    }
}

impl OrdAbs for ComparableBigExponentFloatRef<'_> {
    /// Compares the absolute values of two [`ComparableBigExponentFloatRef`]s.
    ///
    /// This implementation does not follow the IEEE 754 standard. This is how
    /// [`ComparableBigExponentFloatRef`]s are ordered by absolute value, least to greatest:
    ///   - NaN
    ///   - Zero
    ///   - Nonzero finite values
    ///   - $\infty$ and $-\infty$
    ///
    /// For different comparison behavior that follows the IEEE 754 standard, consider just using
    /// [`BigExponentFloat`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.complexity())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{
    ///     NaN, NegativeInfinity, NegativeOne, One, OneHalf, Zero,
    /// };
    /// use malachite_base::num::comparison::traits::PartialOrdAbs;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_float::big_exponent_float::ComparableBigExponentFloatRef;
    /// use std::cmp::Ordering::*;
    ///
    /// assert_eq!(
    ///     ComparableBigExponentFloatRef(&BigExponentFloat::NAN)
    ///         .partial_cmp_abs(&ComparableBigExponentFloatRef(&BigExponentFloat::NAN)),
    ///     Some(Equal)
    /// );
    /// assert!(
    ///     ComparableBigExponentFloatRef(&BigExponentFloat::NAN)
    ///         .lt_abs(&ComparableBigExponentFloatRef(&BigExponentFloat::ZERO))
    /// );
    /// assert!(
    ///     ComparableBigExponentFloatRef(&BigExponentFloat::NEGATIVE_ONE)
    ///         .gt_abs(&ComparableBigExponentFloatRef(&BigExponentFloat::ONE_HALF))
    /// );
    /// assert!(
    ///     ComparableBigExponentFloatRef(&BigExponentFloat::NEGATIVE_INFINITY)
    ///         .gt_abs(&ComparableBigExponentFloatRef(&BigExponentFloat::ONE))
    /// );
    /// ```
    fn cmp_abs(&self, other: &Self) -> Ordering {
        match (self.0.is_nan(), other.0.is_nan()) {
            (true, true) => Equal,
            (true, false) => Less,
            (false, true) => Greater,
            (false, false) => self.0.partial_cmp_abs(other.0).unwrap(),
        }
    }
}

impl PartialOrdAbs for ComparableBigExponentFloatRef<'_> {
    /// Compares the absolute values of two [`ComparableBigExponentFloatRef`]s.
    ///
    /// See the documentation for the [`OrdAbs`] implementation.
    #[inline]
    fn partial_cmp_abs(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp_abs(other))
    }
}

impl OrdAbs for ComparableBigExponentFloat {
    /// Compares the absolute values of two [`ComparableBigExponentFloat`]s.
    ///
    /// This implementation does not follow the IEEE 754 standard. This is how
    /// [`ComparableBigExponentFloat`]s are ordered by absolute value, least to greatest:
    ///   - NaN
    ///   - Zero
    ///   - Nonzero finite values
    ///   - $\infty$ and $-\infty$
    ///
    /// For different comparison behavior that follows the IEEE 754 standard, consider just using
    /// [`BigExponentFloat`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.complexity())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{
    ///     NaN, NegativeInfinity, NegativeOne, One, OneHalf, Zero,
    /// };
    /// use malachite_base::num::comparison::traits::PartialOrdAbs;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_float::big_exponent_float::ComparableBigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// assert_eq!(
    ///     ComparableBigExponentFloat(BigExponentFloat::NAN)
    ///         .partial_cmp_abs(&ComparableBigExponentFloat(BigExponentFloat::NAN)),
    ///     Some(Equal)
    /// );
    /// assert!(
    ///     ComparableBigExponentFloat(BigExponentFloat::NAN)
    ///         .lt_abs(&ComparableBigExponentFloat(BigExponentFloat::ZERO))
    /// );
    /// assert!(
    ///     ComparableBigExponentFloat(BigExponentFloat::NEGATIVE_ONE)
    ///         .gt_abs(&ComparableBigExponentFloat(BigExponentFloat::ONE_HALF))
    /// );
    /// assert!(
    ///     ComparableBigExponentFloat(BigExponentFloat::NEGATIVE_INFINITY)
    ///         .gt_abs(&ComparableBigExponentFloat(BigExponentFloat::ONE))
    /// );
    /// ```
    #[inline]
    fn cmp_abs(&self, other: &Self) -> Ordering {
        self.as_ref().cmp_abs(&other.as_ref())
    }
}

impl PartialOrdAbs for ComparableBigExponentFloat {
    /// Compares the absolute values of two [`ComparableBigExponentFloat`]s.
    ///
    /// See the documentation for the [`OrdAbs`] implementation.
    #[inline]
    fn partial_cmp_abs(&self, other: &Self) -> Option<Ordering> {
        Some(self.as_ref().cmp_abs(&other.as_ref()))
    }
}
