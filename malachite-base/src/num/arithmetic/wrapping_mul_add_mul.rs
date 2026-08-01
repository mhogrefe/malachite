// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::{WrappingMulAddMul, WrappingMulAddMulAssign};

macro_rules! impl_wrapping_mul_add_mul {
    ($t:ident) => {
        impl WrappingMulAddMul for $t {
            type Output = $t;

            /// Adds the products of two pairs of numbers, wrapping around at the boundary of the
            /// type.
            ///
            /// $f(x, y, z, w) = z$, where $z \\equiv xy + zw \\mod 2^W$ and $W$ is `Self::WIDTH`.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::wrapping_mul_add_mul#wrapping_mul_add_mul).
            #[inline]
            fn wrapping_mul_add_mul(self, y: $t, z: $t, w: $t) -> $t {
                self.wrapping_mul(y).wrapping_add(z.wrapping_mul(w))
            }
        }

        impl WrappingMulAddMulAssign for $t {
            /// Adds the products of two pairs of numbers, in place, wrapping around at the boundary
            /// of the type.
            ///
            /// $x \\gets z$, where $z \\equiv xy + zw \\mod 2^W$ and $W$ is `Self::WIDTH`.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::wrapping_mul_add_mul#wrapping_mul_add_mul_assign).
            #[inline]
            fn wrapping_mul_add_mul_assign(&mut self, y: $t, z: $t, w: $t) {
                *self = self.wrapping_mul(y).wrapping_add(z.wrapping_mul(w));
            }
        }
    };
}
apply_to_primitive_ints!(impl_wrapping_mul_add_mul);
