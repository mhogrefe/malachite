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
use crate::num::arithmetic::traits::{
    OverflowingMulAddMul, OverflowingMulAddMulAssign, WrappingMulAddMul,
};

macro_rules! impl_overflowing_mul_add_mul {
    ($t:ident, $wide:ident) => {
        impl OverflowingMulAddMul for $t {
            type Output = $t;

            /// Adds the products of two pairs of numbers.
            ///
            /// Returns a tuple of the result along with a boolean indicating whether an arithmetic
            /// overflow occurred. If an overflow occurred, then the wrapped result is returned.
            ///
            /// $$
            /// f(x, y, z, w) = (xy + zw \\mod 2^W, m),
            /// $$
            /// where $W$ is `Self::WIDTH` and $m$ is true if and only if $xy + zw$ is not
            /// representable.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::overflowing_mul_add_mul#overflowing_mul_add_mul).
            #[inline]
            fn overflowing_mul_add_mul(self, y: $t, z: $t, w: $t) -> ($t, bool) {
                (
                    self.wrapping_mul_add_mul(y, z, w),
                    !matches!($wide(self, y, z, w, false), Wide::Fits(_)),
                )
            }
        }

        impl OverflowingMulAddMulAssign for $t {
            /// Adds the products of two pairs of numbers, in place.
            ///
            /// Returns a boolean indicating whether an arithmetic overflow occurred. If an overflow
            /// occurred, then the wrapped result is assigned.
            ///
            /// $$
            /// x \\gets xy + zw \\mod 2^W,
            /// $$
            /// where $W$ is `Self::WIDTH`; the return value is true if and only if $xy + zw$ is not
            /// representable.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::overflowing_mul_add_mul#overflowing_mul_add_mul_assign).
            #[inline]
            fn overflowing_mul_add_mul_assign(&mut self, y: $t, z: $t, w: $t) -> bool {
                let (result, overflow) = self.overflowing_mul_add_mul(y, z, w);
                *self = result;
                overflow
            }
        }
    };
}

macro_rules! impl_overflowing_mul_add_mul_unsigned {
    ($t:ident) => {
        impl_overflowing_mul_add_mul!($t, mul_add_mul_wide_unsigned);
    };
}
apply_to_unsigneds!(impl_overflowing_mul_add_mul_unsigned);

macro_rules! impl_overflowing_mul_add_mul_signed {
    ($t:ident) => {
        impl_overflowing_mul_add_mul!($t, mul_add_mul_wide_signed);
    };
}
apply_to_signeds!(impl_overflowing_mul_add_mul_signed);
