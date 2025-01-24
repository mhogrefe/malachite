// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::{SaturatingMul, SaturatingMulAssign};

macro_rules! impl_saturating_mul {
    ($t:ident) => {
        impl SaturatingMul<$t> for $t {
            type Output = $t;

            /// This is a wrapper over the `saturating_mul` functions in the standard library, for
            /// example [this one](i32::saturating_mul).
            #[inline]
            fn saturating_mul(self, other: $t) -> $t {
                $t::saturating_mul(self, other)
            }
        }

        impl SaturatingMulAssign<$t> for $t {
            /// Multiplies a number by another number, in place, saturating at the numeric bounds
            /// instead of overflowing.
            ///
            /// $$
            /// x \gets \\begin{cases}
            ///     xy & \text{if} \\quad m \leq xy \leq M, \\\\
            ///     M & \text{if} \\quad xy > M, \\\\
            ///     m & \text{if} \\quad xy < m,
            /// \\end{cases}
            /// $$
            /// where $m$ is `Self::MIN` and $M$ is `Self::MAX`.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::saturating_mul#saturating_mul_assign).
            #[inline]
            fn saturating_mul_assign(&mut self, other: $t) {
                *self = self.saturating_mul(other);
            }
        }
    };
}
apply_to_primitive_ints!(impl_saturating_mul);
