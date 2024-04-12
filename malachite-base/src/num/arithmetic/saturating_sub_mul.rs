// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::{SaturatingSubMul, SaturatingSubMulAssign, UnsignedAbs};
use crate::num::basic::signeds::PrimitiveSigned;
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::num::conversion::traits::WrappingFrom;

fn saturating_sub_mul_unsigned<T: PrimitiveUnsigned>(x: T, y: T, z: T) -> T {
    x.saturating_sub(y.saturating_mul(z))
}

fn saturating_sub_mul_assign_unsigned<T: PrimitiveUnsigned>(x: &mut T, y: T, z: T) {
    x.saturating_sub_assign(y.saturating_mul(z));
}

macro_rules! impl_saturating_sub_mul_unsigned {
    ($t:ident) => {
        impl SaturatingSubMul<$t> for $t {
            type Output = $t;

            /// Subtracts a number by the product of two other numbers, saturating at the numeric
            /// bounds instead of overflowing.
            ///
            /// $$
            /// f(x, y, z) = \\begin{cases}
            ///     x - yz & \text{if} \\quad m \leq x - yz \leq M, \\\\
            ///     M & \text{if} \\quad x - yz > M, \\\\
            ///     m & \text{if} \\quad x - yz < m,
            /// \\end{cases}
            /// $$
            /// where $m$ is `Self::MIN` and $M$ is `Self::MAX`.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::saturating_sub_mul#saturating_sub_mul_assign).
            #[inline]
            fn saturating_sub_mul(self, y: $t, z: $t) -> $t {
                saturating_sub_mul_unsigned(self, y, z)
            }
        }

        impl SaturatingSubMulAssign<$t> for $t {
            /// Subtracts a number by the product of two other numbers in place, saturating at the
            /// numeric bounds instead of overflowing.
            ///
            /// $$
            /// x \gets \\begin{cases}
            ///     x - yz & \text{if} \\quad m \leq x - yz \leq M, \\\\
            ///     M & \text{if} \\quad x - yz > M, \\\\
            ///     m & \text{if} \\quad x - yz < m,
            /// \\end{cases}
            /// $$
            /// where $m$ is `Self::MIN` and $M$ is `Self::MAX`.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::saturating_sub_mul#saturating_sub_mul_assign).
            #[inline]
            fn saturating_sub_mul_assign(&mut self, y: $t, z: $t) {
                saturating_sub_mul_assign_unsigned(self, y, z);
            }
        }
    };
}
apply_to_unsigneds!(impl_saturating_sub_mul_unsigned);

fn saturating_sub_mul_signed<
    U: PrimitiveUnsigned,
    S: PrimitiveSigned + UnsignedAbs<Output = U> + WrappingFrom<U>,
>(
    x: S,
    y: S,
    z: S,
) -> S {
    if y == S::ZERO || z == S::ZERO {
        return x;
    }
    let x_sign = x >= S::ZERO;
    if x_sign == ((y >= S::ZERO) != (z >= S::ZERO)) {
        x.saturating_sub(y.saturating_mul(z))
    } else {
        let x = x.unsigned_abs();
        let product = if let Some(product) = y.unsigned_abs().checked_mul(z.unsigned_abs()) {
            product
        } else {
            return if x_sign { S::MIN } else { S::MAX };
        };
        let result = S::wrapping_from(if x_sign {
            x.wrapping_sub(product)
        } else {
            product.wrapping_sub(x)
        });
        if x >= product || (x_sign == (result < S::ZERO)) {
            result
        } else if x_sign {
            S::MIN
        } else {
            S::MAX
        }
    }
}

macro_rules! impl_saturating_sub_mul_signed {
    ($t:ident) => {
        impl SaturatingSubMul<$t> for $t {
            type Output = $t;

            /// Subtracts a number by the product of two other numbers, saturating at the numeric
            /// bounds instead of overflowing.
            ///
            /// $$
            /// f(x, y, z) = \\begin{cases}
            ///     x - yz & \text{if} \\quad m \leq x - yz \leq M, \\\\
            ///     M & \text{if} \\quad x - yz > M, \\\\
            ///     m & \text{if} \\quad x - yz < m,
            /// \\end{cases}
            /// $$
            /// where $m$ is `Self::MIN` and $M$ is `Self::MAX`.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::saturating_sub_mul#saturating_sub_mul).
            #[inline]
            fn saturating_sub_mul(self, y: $t, z: $t) -> $t {
                saturating_sub_mul_signed(self, y, z)
            }
        }

        impl SaturatingSubMulAssign<$t> for $t {
            /// Subtracts a number by the product of two other numbers in place, saturating at the
            /// numeric bounds instead of overflowing.
            ///
            /// $$
            /// x \gets \\begin{cases}
            ///     x - yz & \text{if} \\quad m \leq x - yz \leq M, \\\\
            ///     M & \text{if} \\quad x - yz > M, \\\\
            ///     m & \text{if} \\quad x - yz < m,
            /// \\end{cases}
            /// $$
            /// where $m$ is `Self::MIN` and $M$ is `Self::MAX`.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::saturating_sub_mul#saturating_sub_mul_assign).
            #[inline]
            fn saturating_sub_mul_assign(&mut self, y: $t, z: $t) {
                *self = self.saturating_sub_mul(y, z);
            }
        }
    };
}
apply_to_signeds!(impl_saturating_sub_mul_signed);
