// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::{MulSubMul, MulSubMulAssign};

macro_rules! impl_mul_sub_mul_primitive_int {
    ($t:ident) => {
        impl MulSubMul for $t {
            type Output = $t;

            /// Subtracts the product of one pair of numbers from the product of another.
            ///
            /// $f(x, y, z, w) = xy - zw$.
            ///
            /// Both products and their difference wrap on overflow, as they do for
            /// [`sub_mul`](super::traits::SubMul::sub_mul).
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::mul_sub_mul#mul_sub_mul).
            #[inline]
            fn mul_sub_mul(self, y: $t, z: $t, w: $t) -> $t {
                self.wrapping_mul(y).wrapping_sub(z.wrapping_mul(w))
            }
        }

        impl MulSubMulAssign for $t {
            /// Subtracts the product of one pair of numbers from the product of another, in place.
            ///
            /// $x \gets xy - zw$.
            ///
            /// Both products and their difference wrap on overflow, as they do for
            /// [`sub_mul`](super::traits::SubMul::sub_mul).
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::mul_sub_mul#mul_sub_mul_assign).
            #[inline]
            fn mul_sub_mul_assign(&mut self, y: $t, z: $t, w: $t) {
                *self = self.wrapping_mul(y).wrapping_sub(z.wrapping_mul(w));
            }
        }
    };
}
apply_to_primitive_ints!(impl_mul_sub_mul_primitive_int);
