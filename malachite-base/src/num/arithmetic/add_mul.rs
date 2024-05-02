// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::{AddMul, AddMulAssign, WrappingAddMul, WrappingAddMulAssign};

macro_rules! impl_add_mul_primitive_int {
    ($t:ident) => {
        impl AddMul for $t {
            type Output = $t;

            /// Adds a number and the product of two other numbers.
            ///
            /// $f(x, y, z) = x + yz$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::add_mul#add_mul).
            #[inline]
            fn add_mul(self, y: $t, z: $t) -> $t {
                self.wrapping_add_mul(y, z)
            }
        }

        impl AddMulAssign<$t> for $t {
            /// Adds the product of two other numbers to a number in place.
            ///
            /// $x \gets x + yz$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::add_mul#add_mul_assign).
            #[inline]
            fn add_mul_assign(&mut self, y: $t, z: $t) {
                self.wrapping_add_mul_assign(y, z)
            }
        }
    };
}
apply_to_primitive_ints!(impl_add_mul_primitive_int);

macro_rules! impl_add_mul_primitive_float {
    ($t:ident) => {
        impl AddMul for $t {
            type Output = $t;

            /// Adds a number and the product of two other numbers.
            ///
            /// $f(x, y, z) = x + yz$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::add_mul#add_mul).
            #[inline]
            fn add_mul(self, y: $t, z: $t) -> $t {
                self + y * z
            }
        }

        impl AddMulAssign<$t> for $t {
            /// Adds the product of two other numbers to a number in place.
            ///
            /// $x \gets x + yz$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::add_mul#add_mul_assign).
            #[inline]
            fn add_mul_assign(&mut self, y: $t, z: $t) {
                *self += y * z;
            }
        }
    };
}
apply_to_primitive_floats!(impl_add_mul_primitive_float);
