// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::big_exponent_float::{
    BigExponentFloat, ComparableBigExponentFloat, ComparableBigExponentFloatRef,
};

impl PartialEq for BigExponentFloat {
    /// Compares two [`BigExponentFloat`]s for equality.
    ///
    /// This implementation follows the IEEE 754 standard: NaN is not equal to anything, not even
    /// itself. Since every value has a single representation, two non-NaN [`BigExponentFloat`]s are
    /// equal exactly when their representations are.
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
    /// use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity, One, Two, Zero};
    /// use malachite_float::BigExponentFloat;
    ///
    /// assert_ne!(BigExponentFloat::NAN, BigExponentFloat::NAN);
    /// assert_eq!(BigExponentFloat::ZERO, BigExponentFloat::ZERO);
    /// assert_eq!(BigExponentFloat::INFINITY, BigExponentFloat::INFINITY);
    /// assert_ne!(
    ///     BigExponentFloat::INFINITY,
    ///     BigExponentFloat::NEGATIVE_INFINITY
    /// );
    /// assert_eq!(BigExponentFloat::ONE, BigExponentFloat::ONE);
    /// assert_ne!(BigExponentFloat::ONE, BigExponentFloat::TWO);
    /// assert_eq!(BigExponentFloat::from(1.5), BigExponentFloat::from(1.5f32));
    /// ```
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        !self.is_nan() && self.0 == other.0
    }
}

impl PartialEq for ComparableBigExponentFloat {
    /// Compares two [`ComparableBigExponentFloat`]s for equality.
    ///
    /// This implementation ignores the IEEE 754 standard in favor of an equality operation that
    /// respects the expected properties of symmetry, reflexivity, and transitivity. Using
    /// [`ComparableBigExponentFloat`], NaN is equal to itself. There is a single, unique NaN;
    /// there's no concept of signalling NaNs. Other values are equal exactly when the wrapped
    /// [`BigExponentFloat`]s are.
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
    /// use malachite_base::num::basic::traits::{NaN, One, Two, Zero};
    /// use malachite_float::BigExponentFloat;
    /// use malachite_float::big_exponent_float::ComparableBigExponentFloat;
    ///
    /// assert_eq!(
    ///     ComparableBigExponentFloat(BigExponentFloat::NAN),
    ///     ComparableBigExponentFloat(BigExponentFloat::NAN)
    /// );
    /// assert_eq!(
    ///     ComparableBigExponentFloat(BigExponentFloat::ZERO),
    ///     ComparableBigExponentFloat(BigExponentFloat::ZERO)
    /// );
    /// assert_eq!(
    ///     ComparableBigExponentFloat(BigExponentFloat::ONE),
    ///     ComparableBigExponentFloat(BigExponentFloat::ONE)
    /// );
    /// assert_ne!(
    ///     ComparableBigExponentFloat(BigExponentFloat::ONE),
    ///     ComparableBigExponentFloat(BigExponentFloat::TWO)
    /// );
    /// ```
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.0.0 == other.0.0
    }
}

impl Eq for ComparableBigExponentFloat {}

impl<'a> PartialEq<ComparableBigExponentFloatRef<'a>> for ComparableBigExponentFloatRef<'_> {
    /// Compares two [`ComparableBigExponentFloatRef`]s for equality.
    ///
    /// This implementation ignores the IEEE 754 standard in favor of an equality operation that
    /// respects the expected properties of symmetry, reflexivity, and transitivity. Using
    /// [`ComparableBigExponentFloatRef`], NaN is equal to itself. There is a single, unique NaN;
    /// there's no concept of signalling NaNs. Other values are equal exactly when the referenced
    /// [`BigExponentFloat`]s are.
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
    /// use malachite_base::num::basic::traits::{NaN, One, Two};
    /// use malachite_float::BigExponentFloat;
    /// use malachite_float::big_exponent_float::ComparableBigExponentFloatRef;
    ///
    /// assert_eq!(
    ///     ComparableBigExponentFloatRef(&BigExponentFloat::NAN),
    ///     ComparableBigExponentFloatRef(&BigExponentFloat::NAN)
    /// );
    /// assert_eq!(
    ///     ComparableBigExponentFloatRef(&BigExponentFloat::ONE),
    ///     ComparableBigExponentFloatRef(&BigExponentFloat::ONE)
    /// );
    /// assert_ne!(
    ///     ComparableBigExponentFloatRef(&BigExponentFloat::ONE),
    ///     ComparableBigExponentFloatRef(&BigExponentFloat::TWO)
    /// );
    /// ```
    #[inline]
    fn eq(&self, other: &ComparableBigExponentFloatRef<'a>) -> bool {
        self.0.0 == other.0.0
    }
}

impl Eq for ComparableBigExponentFloatRef<'_> {}
