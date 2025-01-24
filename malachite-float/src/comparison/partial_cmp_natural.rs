// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Float;
use crate::InnerFloat::{Finite, Infinity, NaN, Zero};
use core::cmp::Ordering::{self, *};
use malachite_base::num::logic::traits::SignificantBits;
use malachite_nz::natural::Natural;

impl PartialOrd<Natural> for Float {
    /// Compares a [`Float`] to a [`Natural`].
    ///
    /// NaN is not comparable to any [`Natural`]. $\infty$ is greater than any [`Natural`], and
    /// $-\infty$ is less. Both the [`Float`] zero and the [`Float`] negative zero are equal to the
    /// [`Natural`] zero.
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
    /// use malachite_base::num::basic::traits::{Infinity, NegativeInfinity};
    /// use malachite_float::Float;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert!(Float::from(80) < Natural::from(100u32));
    /// assert!(Float::INFINITY > Natural::from(100u32));
    /// assert!(Float::NEGATIVE_INFINITY < Natural::from(100u32));
    /// ```
    fn partial_cmp(&self, other: &Natural) -> Option<Ordering> {
        match (self, other) {
            (float_nan!(), _) => None,
            (float_infinity!(), _) => Some(Greater),
            (float_negative_infinity!(), _) => Some(Less),
            (float_either_zero!(), _) => Some(if *other == 0u32 { Equal } else { Less }),
            (
                Float(Finite {
                    sign: s_x,
                    exponent: e_x,
                    significand: x,
                    ..
                }),
                y,
            ) => Some(if !s_x {
                Less
            } else if *other == 0u32 {
                Greater
            } else if *e_x <= 0 {
                Less
            } else {
                u64::from(e_x.unsigned_abs())
                    .cmp(&other.significant_bits())
                    .then_with(|| x.cmp_normalized(y))
            }),
        }
    }
}

impl PartialOrd<Float> for Natural {
    /// Compares a [`Natural`] to a [`Float`].
    ///
    /// No [`Natural`] is comparable to NaN. Every [`Natural`] is smaller than $\infty$ and greater
    /// than $-\infty$. The [`Natural`] zero is equal to both the [`Float`] zero and the [`Float`]
    /// negative zero.
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
    /// use malachite_base::num::basic::traits::{Infinity, NegativeInfinity};
    /// use malachite_float::Float;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert!(Natural::from(100u32) > Float::from(80));
    /// assert!(Natural::from(100u32) < Float::INFINITY);
    /// assert!(Natural::from(100u32) > Float::NEGATIVE_INFINITY);
    /// ```
    #[inline]
    fn partial_cmp(&self, other: &Float) -> Option<Ordering> {
        other.partial_cmp(self).map(Ordering::reverse)
    }
}
