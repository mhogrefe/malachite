// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::{WrappingAddMul, WrappingAddMulAssign};
use crate::num::basic::integers::PrimitiveInt;

fn wrapping_add_mul<T: PrimitiveInt>(x: T, y: T, z: T) -> T {
    x.wrapping_add(y.wrapping_mul(z))
}

fn wrapping_add_mul_assign<T: PrimitiveInt>(x: &mut T, y: T, z: T) {
    x.wrapping_add_assign(y.wrapping_mul(z));
}

macro_rules! impl_wrapping_add_mul {
    ($t:ident) => {
        impl WrappingAddMul<$t> for $t {
            type Output = $t;

            /// Adds a number and the product of two other numbers, wrapping around at the boundary
            /// of the type.
            ///
            /// $f(x, y, z) = w$, where $w \equiv x + yz \mod 2^W$ and $W$ is `Self::WIDTH`.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::wrapping_add_mul#wrapping_add_mul).
            #[inline]
            fn wrapping_add_mul(self, y: $t, z: $t) -> $t {
                wrapping_add_mul(self, y, z)
            }
        }

        impl WrappingAddMulAssign<$t> for $t {
            /// Adds a number and the product of two other numbers in place, wrapping around at the
            /// boundary of the type.
            ///
            /// $x \gets w$, where $w \equiv x + yz \mod 2^W$ and $W$ is `Self::WIDTH`.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::wrapping_add_mul#wrapping_add_mul_assign).
            #[inline]
            fn wrapping_add_mul_assign(&mut self, y: $t, z: $t) {
                wrapping_add_mul_assign(self, y, z);
            }
        }
    };
}
apply_to_primitive_ints!(impl_wrapping_add_mul);
