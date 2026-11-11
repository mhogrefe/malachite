// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::big_exponent_float::InnerBigExponentFloat::{Finite, Infinity, Zero};
use crate::big_exponent_float::{
    BigExponentFloat, ComparableBigExponentFloat, ComparableBigExponentFloatRef,
};
use malachite_base::num::comparison::traits::EqAbs;

impl EqAbs for BigExponentFloat {
    /// Compares the absolute values of two [`BigExponentFloat`]s for equality.
    ///
    /// This implementation follows the IEEE 754 standard: NaN is not equal to anything, not even
    /// itself. $\infty$ and $-\infty$ have equal absolute values.
    ///
    /// For different equality behavior, consider using [`ComparableBigExponentFloat`] or
    /// [`ComparableBigExponentFloatRef`].
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
    ///     Infinity, NaN, NegativeInfinity, NegativeOne, One, Two,
    /// };
    /// use malachite_base::num::comparison::traits::EqAbs;
    /// use malachite_float::BigExponentFloat;
    ///
    /// assert!(BigExponentFloat::NAN.ne_abs(&BigExponentFloat::NAN));
    /// assert!(BigExponentFloat::INFINITY.eq_abs(&BigExponentFloat::NEGATIVE_INFINITY));
    /// assert!(BigExponentFloat::ONE.eq_abs(&BigExponentFloat::NEGATIVE_ONE));
    /// assert!(BigExponentFloat::ONE.ne_abs(&BigExponentFloat::TWO));
    /// ```
    fn eq_abs(&self, other: &Self) -> bool {
        match (self, other) {
            (Self(Infinity { .. }), Self(Infinity { .. })) | (Self(Zero), Self(Zero)) => true,
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
            ) => e_x == e_y && x == y,
            _ => false,
        }
    }
}

impl EqAbs for ComparableBigExponentFloat {
    /// Compares the absolute values of two [`ComparableBigExponentFloat`]s for equality.
    ///
    /// This implementation ignores the IEEE 754 standard in favor of an equality operation that
    /// respects the expected properties of symmetry, reflexivity, and transitivity: NaN's absolute
    /// value is equal to itself.
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
    /// use malachite_base::num::basic::traits::{NaN, NegativeOne, One};
    /// use malachite_base::num::comparison::traits::EqAbs;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_float::big_exponent_float::ComparableBigExponentFloat;
    ///
    /// assert!(
    ///     ComparableBigExponentFloat(BigExponentFloat::NAN)
    ///         .eq_abs(&ComparableBigExponentFloat(BigExponentFloat::NAN))
    /// );
    /// assert!(
    ///     ComparableBigExponentFloat(BigExponentFloat::ONE)
    ///         .eq_abs(&ComparableBigExponentFloat(BigExponentFloat::NEGATIVE_ONE))
    /// );
    /// ```
    #[inline]
    fn eq_abs(&self, other: &Self) -> bool {
        self.as_ref().eq_abs(&other.as_ref())
    }
}

impl<'a> EqAbs<ComparableBigExponentFloatRef<'a>> for ComparableBigExponentFloatRef<'_> {
    /// Compares the absolute values of two [`ComparableBigExponentFloatRef`]s for equality.
    ///
    /// This implementation ignores the IEEE 754 standard in favor of an equality operation that
    /// respects the expected properties of symmetry, reflexivity, and transitivity: NaN's absolute
    /// value is equal to itself.
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
    /// use malachite_base::num::basic::traits::{NaN, NegativeOne, One};
    /// use malachite_base::num::comparison::traits::EqAbs;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_float::big_exponent_float::ComparableBigExponentFloatRef;
    ///
    /// assert!(
    ///     ComparableBigExponentFloatRef(&BigExponentFloat::NAN)
    ///         .eq_abs(&ComparableBigExponentFloatRef(&BigExponentFloat::NAN))
    /// );
    /// assert!(
    ///     ComparableBigExponentFloatRef(&BigExponentFloat::ONE).eq_abs(
    ///         &ComparableBigExponentFloatRef(&BigExponentFloat::NEGATIVE_ONE)
    ///     )
    /// );
    /// ```
    #[inline]
    fn eq_abs(&self, other: &ComparableBigExponentFloatRef<'a>) -> bool {
        (self.0.is_nan() && other.0.is_nan()) || self.0.eq_abs(other.0)
    }
}
