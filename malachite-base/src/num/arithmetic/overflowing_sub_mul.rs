// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::{
    OverflowingSubAssign, OverflowingSubMul, OverflowingSubMulAssign, UnsignedAbs,
};
use crate::num::basic::signeds::PrimitiveSigned;
use crate::num::basic::unsigneds::PrimitiveUnsigned;

fn overflowing_sub_mul_unsigned<T: PrimitiveUnsigned>(x: T, y: T, z: T) -> (T, bool) {
    let (product, overflow_1) = y.overflowing_mul(z);
    let (result, overflow_2) = x.overflowing_sub(product);
    (result, overflow_1 | overflow_2)
}

macro_rules! impl_overflowing_sub_mul_unsigned {
    ($t:ident) => {
        impl OverflowingSubMul<$t> for $t {
            type Output = $t;

            /// Subtracts a number by the product of two other numbers.
            ///
            /// Returns a tuple containing the result and a boolean indicating whether an arithmetic
            /// overflow would occur. If an overflow would have occurred, then the wrapped value is
            /// returned.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::overflowing_sub_mul#overflowing_sub_mul).
            #[inline]
            fn overflowing_sub_mul(self, y: $t, z: $t) -> ($t, bool) {
                overflowing_sub_mul_unsigned(self, y, z)
            }
        }

        impl OverflowingSubMulAssign<$t> for $t {
            /// Subtracts a number by the product of two other numbers, in place.
            ///
            /// Returns a boolean indicating whether an arithmetic overflow would occur. If an
            /// overflow would have occurred, then the wrapped value is assigned.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::overflowing_sub_mul#overflowing_sub_mul_assign).
            #[inline]
            fn overflowing_sub_mul_assign(&mut self, y: $t, z: $t) -> bool {
                let (product, overflow) = y.overflowing_mul(z);
                self.overflowing_sub_assign(product) | overflow
            }
        }
    };
}
apply_to_unsigneds!(impl_overflowing_sub_mul_unsigned);

fn overflowing_sub_mul<U: PrimitiveUnsigned, S: PrimitiveSigned + UnsignedAbs<Output = U>>(
    x: S,
    y: S,
    z: S,
) -> (S, bool) {
    if y == S::ZERO || z == S::ZERO {
        return (x, false);
    }
    let x_sign = x >= S::ZERO;
    if x_sign == ((y >= S::ZERO) != (z >= S::ZERO)) {
        let (product, overflow_1) = y.overflowing_mul(z);
        let (result, overflow_2) = x.overflowing_sub(product);
        (result, overflow_1 | overflow_2)
    } else {
        let result = x.wrapping_sub(y.wrapping_mul(z));
        let overflow = {
            let x = x.unsigned_abs();
            match y.unsigned_abs().checked_mul(z.unsigned_abs()) {
                Some(product) => {
                    x < product
                        && if x_sign {
                            !x.wrapping_sub(product).get_highest_bit()
                        } else {
                            product.wrapping_sub(x).get_highest_bit()
                        }
                }
                None => true,
            }
        };
        (result, overflow)
    }
}

macro_rules! impl_overflowing_sub_mul_signed {
    ($t:ident) => {
        impl OverflowingSubMul<$t> for $t {
            type Output = $t;

            /// Subtracts a number by the product of two other numbers.
            ///
            /// Returns a tuple containing the result and a boolean indicating whether an arithmetic
            /// overflow occurred. If an overflow occurred, then the wrapped value is returned.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::overflowing_sub_mul#overflowing_sub_mul).
            #[inline]
            fn overflowing_sub_mul(self, y: $t, z: $t) -> ($t, bool) {
                overflowing_sub_mul(self, y, z)
            }
        }

        impl OverflowingSubMulAssign<$t> for $t {
            /// Subtracts a number by the product of two other numbers, in place.
            ///
            /// Returns a boolean indicating whether an arithmetic overflow would occur. If an
            /// overflow would have occurred, then the wrapped value is assigned.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::overflowing_sub_mul#overflowing_sub_mul_assign).
            #[inline]
            fn overflowing_sub_mul_assign(&mut self, y: $t, z: $t) -> bool {
                let overflow;
                (*self, overflow) = self.overflowing_sub_mul(y, z);
                overflow
            }
        }
    };
}
apply_to_signeds!(impl_overflowing_sub_mul_signed);
