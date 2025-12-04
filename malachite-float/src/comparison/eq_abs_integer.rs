// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Float;
use crate::InnerFloat::{Finite, Zero};
use core::cmp::Ordering::*;
use malachite_base::num::comparison::traits::EqAbs;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_nz::integer::Integer;

impl EqAbs<Integer> for Float {
    /// Determines whether the absolute value of a [`Float`] is equal to the absolute value of an
    /// [`Integer`].
    ///
    /// $\infty$, $-\infty$, and NaN are not equal to any [`Integer`]. Both the [`Float`] zero and
    /// the [`Float`] negative zero are equal to the [`Integer`] zero.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `min(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{One, OneHalf};
    /// use malachite_base::num::comparison::traits::EqAbs;
    /// use malachite_float::Float;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert!(Float::from(123).eq_abs(&Integer::from(123)));
    /// assert!(Float::from(-123).eq_abs(&Integer::from(-123)));
    /// assert!(Float::ONE_HALF.ne_abs(&Integer::ONE));
    /// ```
    fn eq_abs(&self, other: &Integer) -> bool {
        match self {
            float_either_zero!() => *other == 0u32,
            Self(Finite {
                exponent,
                significand,
                ..
            }) => {
                *other != 0u32
                    && *exponent >= 0
                    && other.significant_bits() == u64::from(exponent.unsigned_abs())
                    && significand.cmp_normalized(other.unsigned_abs_ref()) == Equal
            }
            _ => false,
        }
    }
}

impl EqAbs<Float> for Integer {
    /// Determines whether the absolute value of an [`Integer`] is equal to the absolute value of a
    /// [`Float`].
    ///
    /// No [`Integer`] is equal to $\infty$, $-\infty$, or NaN. The [`Integer`] zero is equal to
    /// both the [`Float`] zero and the [`Float`] negative zero.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `min(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{One, OneHalf};
    /// use malachite_base::num::comparison::traits::EqAbs;
    /// use malachite_float::Float;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert!(Integer::from(123).eq_abs(&Float::from(123)));
    /// assert!(Integer::from(-123).eq_abs(&Float::from(-123)));
    /// assert!(Integer::ONE.ne_abs(&Float::ONE_HALF));
    /// ```
    #[inline]
    fn eq_abs(&self, other: &Float) -> bool {
        other.eq_abs(self)
    }
}
