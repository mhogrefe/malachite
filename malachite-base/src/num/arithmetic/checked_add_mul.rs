// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::{CheckedAddMul, UnsignedAbs};
use crate::num::basic::signeds::PrimitiveSigned;
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::num::conversion::traits::WrappingFrom;

fn checked_add_mul_unsigned<T: PrimitiveUnsigned>(x: T, y: T, z: T) -> Option<T> {
    y.checked_mul(z).and_then(|yz| x.checked_add(yz))
}

macro_rules! impl_checked_add_mul_unsigned {
    ($t:ident) => {
        impl CheckedAddMul<$t> for $t {
            type Output = $t;

            /// Adds a number and the product of two other numbers, returning `None` if the result
            /// cannot be represented.
            ///
            /// $$
            /// f(x, y, z) = \\begin{cases}
            ///     \operatorname{Some}(x + yz) & \text{if} \\quad x + yz < 2^W, \\\\
            ///     \operatorname{None} & \text{if} \\quad x + yz \geq 2^W,
            /// \\end{cases}
            /// $$
            /// where $W$ is `Self::WIDTH`.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::checked_add_mul#checked_add_mul).
            #[inline]
            fn checked_add_mul(self, y: $t, z: $t) -> Option<$t> {
                checked_add_mul_unsigned(self, y, z)
            }
        }
    };
}
apply_to_unsigneds!(impl_checked_add_mul_unsigned);

fn checked_add_mul_signed<
    U: PrimitiveUnsigned,
    T: PrimitiveSigned + UnsignedAbs<Output = U> + WrappingFrom<U>,
>(
    x: T,
    y: T,
    z: T,
) -> Option<T> {
    if y == T::ZERO || z == T::ZERO {
        return Some(x);
    }
    let x_sign = x >= T::ZERO;
    if x_sign == ((y >= T::ZERO) == (z >= T::ZERO)) {
        x.checked_add(y.checked_mul(z)?)
    } else {
        let x = x.unsigned_abs();
        let product = y.unsigned_abs().checked_mul(z.unsigned_abs())?;
        let result = T::wrapping_from(if x_sign {
            x.wrapping_sub(product)
        } else {
            product.wrapping_sub(x)
        });
        if x >= product || (x_sign == (result < T::ZERO)) {
            Some(result)
        } else {
            None
        }
    }
}

macro_rules! impl_checked_add_mul_signed {
    ($t:ident) => {
        impl CheckedAddMul<$t> for $t {
            type Output = $t;

            /// Adds a number and the product of two other numbers, returning `None` if the result
            /// cannot be represented.
            ///
            /// $$
            /// f(x, y, z) = \\begin{cases}
            ///     \operatorname{Some}(x + yz) &
            ///         \text{if} \\quad -2^{W-1} \leq x + yz < 2^{W-1}, \\\\
            ///     \operatorname{None} &
            ///         \text{if} \\quad x + yz < -2^{W-1} \\ \mathrm{or}
            ///         \\ x + yz \geq 2^{W-1}, \\\\
            /// \\end{cases}
            /// $$
            /// where $W$ is `Self::WIDTH`.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::checked_add_mul#checked_add_mul).
            #[inline]
            fn checked_add_mul(self, y: $t, z: $t) -> Option<$t> {
                checked_add_mul_signed(self, y, z)
            }
        }
    };
}
apply_to_signeds!(impl_checked_add_mul_signed);
