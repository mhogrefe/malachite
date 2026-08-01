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
use crate::num::arithmetic::traits::{SaturatingMulSubMul, SaturatingMulSubMulAssign};

macro_rules! impl_saturating_mul_sub_mul {
    ($t:ident, $wide:ident) => {
        impl SaturatingMulSubMul for $t {
            type Output = $t;

            /// Subtracts the product of one pair of numbers from the product of another, saturating
            /// at the numeric bounds instead of overflowing.
            ///
            /// $$
            /// f(x, y, z, w) = \\begin{cases}
            ///     xy - zw & \\text{if} \\quad m \\leq xy - zw \\leq M \\\\
            ///     M & \\text{if} \\quad xy - zw > M \\\\
            ///     m & \\text{if} \\quad xy - zw < m,
            /// \\end{cases}
            /// $$
            /// where $m$ is `Self::MIN` and $M$ is `Self::MAX`.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::saturating_mul_sub_mul#saturating_mul_sub_mul).
            #[inline]
            fn saturating_mul_sub_mul(self, y: $t, z: $t, w: $t) -> $t {
                match $wide(self, y, z, w, true) {
                    Wide::Fits(v) => v,
                    Wide::Above => $t::MAX,
                    Wide::Below => $t::MIN,
                }
            }
        }

        impl SaturatingMulSubMulAssign for $t {
            /// Subtracts the product of one pair of numbers from the product of another, in place,
            /// saturating at the numeric bounds instead of overflowing.
            ///
            /// $$
            /// x \\gets \\begin{cases}
            ///     xy - zw & \\text{if} \\quad m \\leq xy - zw \\leq M \\\\
            ///     M & \\text{if} \\quad xy - zw > M \\\\
            ///     m & \\text{if} \\quad xy - zw < m,
            /// \\end{cases}
            /// $$
            /// where $m$ is `Self::MIN` and $M$ is `Self::MAX`.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::saturating_mul_sub_mul#saturating_mul_sub_mul_assign).
            #[inline]
            fn saturating_mul_sub_mul_assign(&mut self, y: $t, z: $t, w: $t) {
                *self = self.saturating_mul_sub_mul(y, z, w);
            }
        }
    };
}

macro_rules! impl_saturating_mul_sub_mul_unsigned {
    ($t:ident) => {
        impl_saturating_mul_sub_mul!($t, mul_add_mul_wide_unsigned);
    };
}
apply_to_unsigneds!(impl_saturating_mul_sub_mul_unsigned);

macro_rules! impl_saturating_mul_sub_mul_signed {
    ($t:ident) => {
        impl_saturating_mul_sub_mul!($t, mul_add_mul_wide_signed);
    };
}
apply_to_signeds!(impl_saturating_mul_sub_mul_signed);
