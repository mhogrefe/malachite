// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::{SaturatingSub, SaturatingSubAssign};

macro_rules! impl_saturating_sub {
    ($t:ident) => {
        impl SaturatingSub<$t> for $t {
            type Output = $t;

            /// This is a wrapper over the `saturating_sub` functions in the standard library, for
            /// example [this one](i32::saturating_sub).
            #[inline]
            fn saturating_sub(self, other: $t) -> $t {
                $t::saturating_sub(self, other)
            }
        }

        impl SaturatingSubAssign<$t> for $t {
            /// Subtracts a number by another number in place, saturating at the numeric bounds
            /// instead of overflowing.
            ///
            /// $$
            /// x \gets \\begin{cases}
            ///     x - y & \text{if} \\quad m \leq x - y \leq M, \\\\
            ///     M & \text{if} \\quad x - y > M, \\\\
            ///     m & \text{if} \\quad x - y < m,
            /// \\end{cases}
            /// $$
            /// where $m$ is `Self::MIN` and $M$ is `Self::MAX`.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::saturating_sub#saturating_sub_assign).
            #[inline]
            fn saturating_sub_assign(&mut self, other: $t) {
                *self = self.saturating_sub(other);
            }
        }
    };
}
apply_to_primitive_ints!(impl_saturating_sub);
