// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::InnerFloat::{Finite, Infinity, NaN, Zero};
use crate::{ComparableFloat, ComparableFloatRef, Float};
use core::cmp::Ordering::{self, *};
use malachite_base::num::comparison::traits::{OrdAbs, PartialOrdAbs};

impl PartialOrdAbs for Float {
    /// Compares the absolute values of two [`Float`]s.
    ///
    /// This implementation follows the IEEE 754 standard. `NaN` is not comparable to anything, not
    /// even itself. [`Float`]s with different precisions are equal if they represent the same
    /// numeric value.
    ///
    /// For different comparison behavior that provides a total order, consider using
    /// [`ComparableFloat`] or [`ComparableFloatRef`].
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
    /// use malachite_base::num::basic::traits::{
    ///     Infinity, NaN, NegativeInfinity, NegativeOne, NegativeZero, One, OneHalf, Zero,
    /// };
    /// use malachite_base::num::comparison::traits::PartialOrdAbs;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// assert_eq!(Float::NAN.partial_cmp_abs(&Float::NAN), None);
    /// assert_eq!(
    ///     Float::ZERO.partial_cmp_abs(&Float::NEGATIVE_ZERO),
    ///     Some(Equal)
    /// );
    /// assert_eq!(
    ///     Float::ONE.partial_cmp_abs(&Float::one_prec(100)),
    ///     Some(Equal)
    /// );
    /// assert!(Float::INFINITY.gt_abs(&Float::ONE));
    /// assert!(Float::NEGATIVE_INFINITY.gt_abs(&Float::ONE));
    /// assert!(Float::ONE_HALF.lt_abs(&Float::ONE));
    /// assert!(Float::ONE_HALF.lt_abs(&Float::NEGATIVE_ONE));
    /// ```
    fn partial_cmp_abs(&self, other: &Float) -> Option<Ordering> {
        match (self, other) {
            (float_nan!(), _) | (_, float_nan!()) => None,
            (float_either_infinity!(), float_either_infinity!())
            | (float_either_zero!(), float_either_zero!()) => Some(Equal),
            (float_either_infinity!(), _) | (_, float_either_zero!()) => Some(Greater),
            (_, float_either_infinity!()) | (float_either_zero!(), _) => Some(Less),
            (
                Float(Finite {
                    exponent: e_x,
                    significand: x,
                    ..
                }),
                Float(Finite {
                    exponent: e_y,
                    significand: y,
                    ..
                }),
            ) => Some(e_x.cmp(e_y).then_with(|| x.cmp_normalized_no_shift(y))),
        }
    }
}

impl<'a> OrdAbs for ComparableFloatRef<'a> {
    /// Compares the absolute values of two [`ComparableFloatRef`]s.
    ///
    /// This implementation does not follow the IEEE 754 standard. This is how
    /// [`ComparableFloatRef`]s are ordered by absolute value, from least to greatest:
    ///   - NaN
    ///   - Positive and negative zero
    ///   - Nonzero finite floats
    ///   - $\infty$ and $-\infty$
    ///
    /// For different comparison behavior that follows the IEEE 754 standard, consider just using
    /// [`Float`].
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
    /// use malachite_base::num::basic::traits::{
    ///     Infinity, NaN, NegativeInfinity, NegativeOne, NegativeZero, One, OneHalf, Zero,
    /// };
    /// use malachite_base::num::comparison::traits::PartialOrdAbs;
    /// use malachite_float::{ComparableFloatRef, Float};
    /// use std::cmp::Ordering::*;
    ///
    /// assert_eq!(
    ///     ComparableFloatRef(&Float::NAN).partial_cmp_abs(&ComparableFloatRef(&Float::NAN)),
    ///     Some(Equal)
    /// );
    /// assert_eq!(
    ///     ComparableFloatRef(&Float::ZERO)
    ///         .partial_cmp_abs(&ComparableFloatRef(&Float::NEGATIVE_ZERO)),
    ///     Some(Equal)
    /// );
    /// assert!(ComparableFloatRef(&Float::ONE).lt_abs(&ComparableFloatRef(&Float::one_prec(100))));
    /// assert!(ComparableFloatRef(&Float::INFINITY).gt_abs(&ComparableFloatRef(&Float::ONE)));
    /// assert!(
    ///     ComparableFloatRef(&Float::NEGATIVE_INFINITY).gt_abs(&ComparableFloatRef(&Float::ONE))
    /// );
    /// assert!(ComparableFloatRef(&Float::ONE_HALF).lt_abs(&ComparableFloatRef(&Float::ONE)));
    /// assert!(
    ///     ComparableFloatRef(&Float::ONE_HALF).lt_abs(&ComparableFloatRef(&Float::NEGATIVE_ONE))
    /// );
    /// ```
    #[allow(clippy::match_same_arms)]
    fn cmp_abs(&self, other: &ComparableFloatRef<'a>) -> Ordering {
        match (&self.0, &other.0) {
            (float_nan!(), float_nan!())
            | (float_either_infinity!(), float_either_infinity!())
            | (float_either_zero!(), float_either_zero!()) => Equal,
            (float_either_infinity!(), _) | (_, float_nan!()) => Greater,
            (_, float_either_infinity!()) | (float_nan!(), _) => Less,
            (float_either_zero!(), _) => Less,
            (_, float_either_zero!()) => Greater,
            (
                Float(Finite {
                    exponent: e_x,
                    precision: p_x,
                    significand: x,
                    ..
                }),
                Float(Finite {
                    exponent: e_y,
                    precision: p_y,
                    significand: y,
                    ..
                }),
            ) => e_x
                .cmp(e_y)
                .then_with(|| x.cmp_normalized_no_shift(y))
                .then_with(|| p_x.cmp(p_y)),
        }
    }
}

impl PartialOrdAbs for ComparableFloatRef<'_> {
    /// Compares the absolute values of two [`ComparableFloatRef`]s.
    ///
    /// See the documentation for the [`Ord`] implementation.
    #[inline]
    fn partial_cmp_abs(&self, other: &ComparableFloatRef) -> Option<Ordering> {
        Some(self.cmp_abs(other))
    }
}

impl OrdAbs for ComparableFloat {
    /// Compares the absolute values of two [`ComparableFloat`]s.
    ///
    /// This implementation does not follow the IEEE 754 standard. This is how [`ComparableFloat`]s
    /// are ordered by absolute value, from least to greatest:
    ///   - NaN
    ///   - Positive and negative zero
    ///   - Nonzero finite floats
    ///   - $\infty$ and $-\infty$
    ///
    /// For different comparison behavior that follows the IEEE 754 standard, consider just using
    /// [`Float`].
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
    /// use malachite_base::num::basic::traits::{
    ///     Infinity, NaN, NegativeInfinity, NegativeOne, NegativeZero, One, OneHalf, Zero,
    /// };
    /// use malachite_base::num::comparison::traits::PartialOrdAbs;
    /// use malachite_float::{ComparableFloat, Float};
    /// use std::cmp::Ordering::*;
    ///
    /// assert_eq!(
    ///     ComparableFloat(Float::NAN).partial_cmp_abs(&ComparableFloat(Float::NAN)),
    ///     Some(Equal)
    /// );
    /// assert_eq!(
    ///     ComparableFloat(Float::ZERO).partial_cmp_abs(&ComparableFloat(Float::NEGATIVE_ZERO)),
    ///     Some(Equal)
    /// );
    /// assert!(ComparableFloat(Float::ONE).lt_abs(&ComparableFloat(Float::one_prec(100))));
    /// assert!(ComparableFloat(Float::INFINITY).gt_abs(&ComparableFloat(Float::ONE)));
    /// assert!(ComparableFloat(Float::NEGATIVE_INFINITY).gt_abs(&ComparableFloat(Float::ONE)));
    /// assert!(ComparableFloat(Float::ONE_HALF).lt_abs(&ComparableFloat(Float::ONE)));
    /// assert!(ComparableFloat(Float::ONE_HALF).lt_abs(&ComparableFloat(Float::NEGATIVE_ONE)));
    /// ```
    #[inline]
    fn cmp_abs(&self, other: &ComparableFloat) -> Ordering {
        self.as_ref().cmp_abs(&other.as_ref())
    }
}

impl PartialOrdAbs for ComparableFloat {
    /// Compares the absolute values of two [`ComparableFloatRef`]s.
    ///
    /// See the documentation for the [`Ord`] implementation.
    #[inline]
    fn partial_cmp_abs(&self, other: &ComparableFloat) -> Option<Ordering> {
        Some(self.as_ref().cmp_abs(&other.as_ref()))
    }
}
