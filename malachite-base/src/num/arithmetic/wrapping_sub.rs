// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::{WrappingSub, WrappingSubAssign};

macro_rules! impl_wrapping_sub {
    ($t:ident) => {
        impl WrappingSub<$t> for $t {
            type Output = $t;

            /// This is a wrapper over the `wrapping_sub` functions in the standard library, for
            /// example [this one](u32::wrapping_sub).
            #[inline]
            fn wrapping_sub(self, other: $t) -> $t {
                $t::wrapping_sub(self, other)
            }
        }

        impl WrappingSubAssign<$t> for $t {
            /// Subtracts a number by another number in place, wrapping around at the boundary of
            /// the type.
            ///
            /// $x \gets z$, where $z \equiv x - y \mod 2^W$ and $W$ is `Self::WIDTH`.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::wrapping_sub#wrapping_sub_assign).
            #[inline]
            fn wrapping_sub_assign(&mut self, other: $t) {
                *self = self.wrapping_sub(other);
            }
        }
    };
}
apply_to_primitive_ints!(impl_wrapping_sub);
