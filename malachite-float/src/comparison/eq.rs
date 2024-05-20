// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::InnerFloat::{Finite, Infinity, NaN, Zero};
use crate::{ComparableFloat, ComparableFloatRef, Float};
use core::cmp::Ordering::*;

impl PartialEq for Float {
    /// Compares two [`Float`]s for equality.
    ///
    /// This implementation follows the IEEE 754 standard. `NaN` is not equal to anything, not even
    /// itself. Positive zero is equal to negative zero. [`Float`]s with different precisions are
    /// equal if they represent the same numeric value.
    ///
    /// For different equality behavior, consider using [`ComparableFloat`] or
    /// [`ComparableFloatRef`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{NaN, NegativeZero, One, Two, Zero};
    /// use malachite_float::Float;
    ///
    /// assert_ne!(Float::NAN, Float::NAN);
    /// assert_eq!(Float::ZERO, Float::ZERO);
    /// assert_eq!(Float::NEGATIVE_ZERO, Float::NEGATIVE_ZERO);
    /// assert_eq!(Float::ZERO, Float::NEGATIVE_ZERO);
    ///
    /// assert_eq!(Float::ONE, Float::ONE);
    /// assert_ne!(Float::ONE, Float::TWO);
    /// assert_eq!(Float::ONE, Float::one_prec(100));
    /// ```
    fn eq(&self, other: &Float) -> bool {
        match (self, other) {
            (Float(Infinity { sign: s_x }), Float(Infinity { sign: s_y })) => s_x == s_y,
            (float_either_zero!(), float_either_zero!()) => true,
            (
                Float(Finite {
                    sign: s_x,
                    exponent: e_x,
                    significand: x,
                    ..
                }),
                Float(Finite {
                    sign: s_y,
                    exponent: e_y,
                    significand: y,
                    ..
                }),
            ) => e_x == e_y && s_x == s_y && x.cmp_normalized_no_shift(y) == Equal,
            _ => false,
        }
    }
}

impl PartialEq for ComparableFloat {
    /// Compares two [`ComparableFloat`]s for equality.
    ///
    /// This implementation ignores the IEEE 754 standard in favor of an equality operation that
    /// respects the expected properties of symmetry, reflexivity, and transitivity. Using
    /// [`ComparableFloat`], `NaN`s are equal to themselves. There is a single, unique `NaN`;
    /// there's no concept of signalling `NaN`s. Positive and negative zero are two distinct values,
    /// not equal to each other. [`ComparableFloat`]s with different precisions are unequal.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{NaN, NegativeZero, One, Two, Zero};
    /// use malachite_float::{ComparableFloat, Float};
    ///
    /// assert_eq!(ComparableFloat(Float::NAN), ComparableFloat(Float::NAN));
    /// assert_eq!(ComparableFloat(Float::ZERO), ComparableFloat(Float::ZERO));
    /// assert_eq!(
    ///     ComparableFloat(Float::NEGATIVE_ZERO),
    ///     ComparableFloat(Float::NEGATIVE_ZERO)
    /// );
    /// assert_ne!(
    ///     ComparableFloat(Float::ZERO),
    ///     ComparableFloat(Float::NEGATIVE_ZERO)
    /// );
    ///
    /// assert_eq!(ComparableFloat(Float::ONE), ComparableFloat(Float::ONE));
    /// assert_ne!(ComparableFloat(Float::ONE), ComparableFloat(Float::TWO));
    /// assert_ne!(
    ///     ComparableFloat(Float::ONE),
    ///     ComparableFloat(Float::one_prec(100))
    /// );
    /// ```
    #[inline]
    fn eq(&self, other: &ComparableFloat) -> bool {
        self.as_ref() == other.as_ref()
    }
}

impl Eq for ComparableFloat {}

impl<'a, 'b> PartialEq<ComparableFloatRef<'b>> for ComparableFloatRef<'a> {
    /// Compares two [`ComparableFloatRef`]s for equality.
    ///
    /// This implementation ignores the IEEE 754 standard in favor of an equality operation that
    /// respects the expected properties of symmetry, reflexivity, and transitivity. Using
    /// [`ComparableFloatRef`], `NaN`s are equal to themselves. There is a single, unique `NaN`;
    /// there's no concept of signalling `NaN`s. Positive and negative zero are two distinct values,
    /// not equal to each other. [`ComparableFloatRef`]s with different precisions are unequal.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{NaN, NegativeZero, One, Two, Zero};
    /// use malachite_float::{ComparableFloatRef, Float};
    ///
    /// assert_eq!(
    ///     ComparableFloatRef(&Float::NAN),
    ///     ComparableFloatRef(&Float::NAN)
    /// );
    /// assert_eq!(
    ///     ComparableFloatRef(&Float::ZERO),
    ///     ComparableFloatRef(&Float::ZERO)
    /// );
    /// assert_eq!(
    ///     ComparableFloatRef(&Float::NEGATIVE_ZERO),
    ///     ComparableFloatRef(&Float::NEGATIVE_ZERO)
    /// );
    /// assert_ne!(
    ///     ComparableFloatRef(&Float::ZERO),
    ///     ComparableFloatRef(&Float::NEGATIVE_ZERO)
    /// );
    ///
    /// assert_eq!(
    ///     ComparableFloatRef(&Float::ONE),
    ///     ComparableFloatRef(&Float::ONE)
    /// );
    /// assert_ne!(
    ///     ComparableFloatRef(&Float::ONE),
    ///     ComparableFloatRef(&Float::TWO)
    /// );
    /// assert_ne!(
    ///     ComparableFloatRef(&Float::ONE),
    ///     ComparableFloatRef(&Float::one_prec(100))
    /// );
    /// ```
    fn eq(&self, other: &ComparableFloatRef<'b>) -> bool {
        match (&self.0, &other.0) {
            (float_nan!(), float_nan!()) => true,
            (Float(Infinity { sign: s_x }), Float(Infinity { sign: s_y }))
            | (Float(Zero { sign: s_x }), Float(Zero { sign: s_y })) => s_x == s_y,
            (
                Float(Finite {
                    sign: s_x,
                    exponent: e_x,
                    precision: p_x,
                    significand: x,
                }),
                Float(Finite {
                    sign: s_y,
                    exponent: e_y,
                    precision: p_y,
                    significand: y,
                }),
            ) => s_x == s_y && e_x == e_y && p_x == p_y && x == y,
            _ => false,
        }
    }
}

impl<'a> Eq for ComparableFloatRef<'a> {}
