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
use malachite_base::num::logic::traits::SignificantBits;
use malachite_nz::natural::Natural;

impl PartialEq<Natural> for Float {
    /// Determines whether a [`Float`] is equal to a [`Natural`].
    ///
    /// $\infty$, $-\infty$, and NaN are not equal to any [`Natural`]. Both the [`Float`] zero and
    /// the [`Float`] negative zero are equal to the [`Natural`] zero.
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
    /// use malachite_float::Float;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert!(Float::from(123) == Natural::from(123u32));
    /// assert!(Float::ONE_HALF != Natural::ONE);
    /// ```
    fn eq(&self, other: &Natural) -> bool {
        match self {
            float_either_zero!() => *other == 0u32,
            Float(Finite {
                sign,
                exponent,
                significand,
                ..
            }) => {
                *sign
                    && *other != 0u32
                    && *exponent >= 0
                    && other.significant_bits() == u64::from(exponent.unsigned_abs())
                    && significand.cmp_normalized(other) == Equal
            }
            _ => false,
        }
    }
}

impl PartialEq<Float> for Natural {
    /// Determines whether a [`Natural`] is equal to a [`Float`].
    ///
    /// No [`Natural`] is equal to $\infty$, $-\infty$, or NaN. The [`Natural`] zero is equal to
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
    /// use malachite_float::Float;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert!(Natural::from(123u32) == Float::from(123));
    /// assert!(Natural::ONE != Float::ONE_HALF);
    /// ```
    #[inline]
    fn eq(&self, other: &Float) -> bool {
        other == self
    }
}
