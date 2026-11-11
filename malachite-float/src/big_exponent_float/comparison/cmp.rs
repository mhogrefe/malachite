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

impl PartialOrd for BigExponentFloat {
    /// Compares two [`BigExponentFloat`]s.
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
    /// use core::str::FromStr;
    /// use malachite_base::num::basic::traits::{
    ///     Infinity, NaN, NegativeInfinity, NegativeOne, One, OneHalf, Zero,
    /// };
    /// use malachite_base::num::conversion::traits::IntegerMantissaAndExponent;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::natural::Natural;
    /// use std::cmp::Ordering::*;
    ///
    /// assert_eq!(
    ///     BigExponentFloat::NAN.partial_cmp(&BigExponentFloat::NAN),
    ///     None
    /// );
    /// assert_eq!(
    ///     BigExponentFloat::ZERO.partial_cmp(&BigExponentFloat::ZERO),
    ///     Some(Equal)
    /// );
    /// assert!(BigExponentFloat::INFINITY > BigExponentFloat::ONE);
    /// assert!(BigExponentFloat::NEGATIVE_INFINITY < BigExponentFloat::ONE);
    /// assert!(BigExponentFloat::ONE_HALF < BigExponentFloat::ONE);
    /// assert!(BigExponentFloat::ONE_HALF > BigExponentFloat::NEGATIVE_ONE);
    ///
    /// let huge = BigExponentFloat::from_integer_mantissa_and_exponent(
    ///     Natural::ONE,
    ///     Integer::from_str("100000000000000000000").unwrap(),
    /// )
    /// .unwrap();
    /// assert!(huge > BigExponentFloat::ONE);
    /// assert!(huge < BigExponentFloat::INFINITY);
    /// ```
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Self(NaN), _) | (_, Self(NaN)) => None,
            (Self(Infinity { sign: s_x }), Self(Infinity { sign: s_y })) => Some(s_x.cmp(s_y)),
            (Self(Zero), Self(Zero)) => Some(Equal),
            (Self(Infinity { sign }), _) | (Self(Finite { sign, .. }), Self(Zero)) => {
                Some(if *sign { Greater } else { Less })
            }
            (_, Self(Infinity { sign })) | (Self(Zero), Self(Finite { sign, .. })) => {
                Some(if *sign { Less } else { Greater })
            }
            (
                Self(Finite {
                    sign: s_x,
                    exponent: e_x,
                    significand: x,
                }),
                Self(Finite {
                    sign: s_y,
                    exponent: e_y,
                    significand: y,
                }),
            ) => Some(s_x.cmp(s_y).then_with(|| {
                let abs_cmp = e_x.cmp(e_y).then_with(|| x.cmp_normalized_no_shift(y));
                if *s_x { abs_cmp } else { abs_cmp.reverse() }
            })),
        }
    }
}

impl Ord for ComparableBigExponentFloatRef<'_> {
    /// Compares two [`ComparableBigExponentFloatRef`]s.
    ///
    /// This implementation does not follow the IEEE 754 standard. This is how
    /// [`ComparableBigExponentFloatRef`]s are ordered, least to greatest:
    ///   - $-\infty$
    ///   - Negative finite values
    ///   - NaN
    ///   - Zero
    ///   - Positive finite values
    ///   - $\infty$
    ///
    /// This places NaN where [`ComparableFloat`](crate::ComparableFloat) does, between the two
    /// zeros, in a type with only one zero.
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
    /// use malachite_base::num::basic::traits::{Infinity, NaN, NegativeOne, One, OneHalf, Zero};
    /// use malachite_float::BigExponentFloat;
    /// use malachite_float::big_exponent_float::ComparableBigExponentFloatRef;
    /// use std::cmp::Ordering::*;
    ///
    /// assert_eq!(
    ///     ComparableBigExponentFloatRef(&BigExponentFloat::NAN)
    ///         .partial_cmp(&ComparableBigExponentFloatRef(&BigExponentFloat::NAN)),
    ///     Some(Equal)
    /// );
    /// assert!(
    ///     ComparableBigExponentFloatRef(&BigExponentFloat::NAN)
    ///         < ComparableBigExponentFloatRef(&BigExponentFloat::ZERO)
    /// );
    /// assert!(
    ///     ComparableBigExponentFloatRef(&BigExponentFloat::NAN)
    ///         > ComparableBigExponentFloatRef(&BigExponentFloat::NEGATIVE_ONE)
    /// );
    /// assert!(
    ///     ComparableBigExponentFloatRef(&BigExponentFloat::INFINITY)
    ///         > ComparableBigExponentFloatRef(&BigExponentFloat::ONE)
    /// );
    /// assert!(
    ///     ComparableBigExponentFloatRef(&BigExponentFloat::ONE_HALF)
    ///         < ComparableBigExponentFloatRef(&BigExponentFloat::ONE)
    /// );
    /// ```
    fn cmp(&self, other: &Self) -> Ordering {
        match (self.0.is_nan(), other.0.is_nan()) {
            (true, true) => Equal,
            (true, false) => {
                if other.0.is_sign_negative() {
                    Greater
                } else {
                    Less
                }
            }
            (false, true) => {
                if self.0.is_sign_negative() {
                    Less
                } else {
                    Greater
                }
            }
            (false, false) => self.0.partial_cmp(other.0).unwrap(),
        }
    }
}

impl PartialOrd for ComparableBigExponentFloatRef<'_> {
    /// Compares two [`ComparableBigExponentFloatRef`]s.
    ///
    /// See the documentation for the [`Ord`] implementation.
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ComparableBigExponentFloat {
    /// Compares two [`ComparableBigExponentFloat`]s.
    ///
    /// This implementation does not follow the IEEE 754 standard. This is how
    /// [`ComparableBigExponentFloat`]s are ordered, least to greatest:
    ///   - $-\infty$
    ///   - Negative finite values
    ///   - NaN
    ///   - Zero
    ///   - Positive finite values
    ///   - $\infty$
    ///
    /// This places NaN where [`ComparableFloat`](crate::ComparableFloat) does, between the two
    /// zeros, in a type with only one zero.
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
    /// use malachite_base::num::basic::traits::{Infinity, NaN, NegativeOne, One, OneHalf, Zero};
    /// use malachite_float::BigExponentFloat;
    /// use malachite_float::big_exponent_float::ComparableBigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// assert_eq!(
    ///     ComparableBigExponentFloat(BigExponentFloat::NAN)
    ///         .partial_cmp(&ComparableBigExponentFloat(BigExponentFloat::NAN)),
    ///     Some(Equal)
    /// );
    /// assert!(
    ///     ComparableBigExponentFloat(BigExponentFloat::NAN)
    ///         < ComparableBigExponentFloat(BigExponentFloat::ZERO)
    /// );
    /// assert!(
    ///     ComparableBigExponentFloat(BigExponentFloat::NAN)
    ///         > ComparableBigExponentFloat(BigExponentFloat::NEGATIVE_ONE)
    /// );
    /// assert!(
    ///     ComparableBigExponentFloat(BigExponentFloat::INFINITY)
    ///         > ComparableBigExponentFloat(BigExponentFloat::ONE)
    /// );
    /// assert!(
    ///     ComparableBigExponentFloat(BigExponentFloat::ONE_HALF)
    ///         < ComparableBigExponentFloat(BigExponentFloat::ONE)
    /// );
    /// ```
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.as_ref().cmp(&other.as_ref())
    }
}

impl PartialOrd for ComparableBigExponentFloat {
    /// Compares two [`ComparableBigExponentFloat`]s.
    ///
    /// See the documentation for the [`Ord`] implementation.
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
