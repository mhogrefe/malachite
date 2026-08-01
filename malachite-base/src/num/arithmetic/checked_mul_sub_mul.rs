// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::mul_add_mul::{
    Wide, mul_add_mul_wide_signed, mul_add_mul_wide_unsigned,
};
use crate::num::arithmetic::traits::CheckedMulSubMul;

macro_rules! impl_checked_mul_sub_mul_unsigned {
    ($t:ident) => {
        impl CheckedMulSubMul for $t {
            type Output = $t;

            /// Subtracts the product of one pair of numbers from the product of another, returning
            /// `None` if the result cannot be represented.
            ///
            /// $$
            /// f(x, y, z, w) = \\begin{cases}
            ///     xy - zw & \\text{if} \\quad xy - zw \\ \\text{is representable} \\\\
            ///     \\operatorname{None} & \\text{otherwise}
            /// \\end{cases}
            /// $$
            ///
            /// The products are formed at double width, so a product that does not fit does not by
            /// itself make the result unrepresentable.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::checked_mul_sub_mul#checked_mul_sub_mul).
            #[inline]
            fn checked_mul_sub_mul(self, y: $t, z: $t, w: $t) -> Option<$t> {
                match mul_add_mul_wide_unsigned(self, y, z, w, true) {
                    Wide::Fits(v) => Some(v),
                    _ => None,
                }
            }
        }
    };
}
apply_to_unsigneds!(impl_checked_mul_sub_mul_unsigned);

macro_rules! impl_checked_mul_sub_mul_signed {
    ($t:ident) => {
        impl CheckedMulSubMul for $t {
            type Output = $t;

            /// Subtracts the product of one pair of numbers from the product of another, returning
            /// `None` if the result cannot be represented.
            ///
            /// $$
            /// f(x, y, z, w) = \\begin{cases}
            ///     xy - zw & \\text{if} \\quad xy - zw \\ \\text{is representable} \\\\
            ///     \\operatorname{None} & \\text{otherwise}
            /// \\end{cases}
            /// $$
            ///
            /// The products are formed at double width, so a product that does not fit does not by
            /// itself make the result unrepresentable.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::checked_mul_sub_mul#checked_mul_sub_mul).
            #[inline]
            fn checked_mul_sub_mul(self, y: $t, z: $t, w: $t) -> Option<$t> {
                match mul_add_mul_wide_signed(self, y, z, w, true) {
                    Wide::Fits(v) => Some(v),
                    _ => None,
                }
            }
        }
    };
}
apply_to_signeds!(impl_checked_mul_sub_mul_signed);
