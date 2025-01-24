// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::{WrappingMul, WrappingMulAssign};

macro_rules! impl_wrapping_mul {
    ($t:ident) => {
        impl WrappingMul<$t> for $t {
            type Output = $t;

            /// This is a wrapper over the `wrapping_mul` functions in the standard library, for
            /// example [this one](u32::wrapping_mul).
            #[inline]
            fn wrapping_mul(self, other: $t) -> $t {
                $t::wrapping_mul(self, other)
            }
        }

        impl WrappingMulAssign<$t> for $t {
            /// Adds a number to another number in place, wrapping around at the boundary of the
            /// type.
            ///
            /// $x \gets z$, where $z \equiv xy \mod 2^W$ and $W$ is `Self::WIDTH`.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::wrapping_mul#wrapping_mul_assign).
            #[inline]
            fn wrapping_mul_assign(&mut self, other: $t) {
                *self = self.wrapping_mul(other);
            }
        }
    };
}
apply_to_primitive_ints!(impl_wrapping_mul);
