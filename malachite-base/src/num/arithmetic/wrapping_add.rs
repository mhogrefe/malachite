// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::{WrappingAdd, WrappingAddAssign};

macro_rules! impl_wrapping_add {
    ($t:ident) => {
        impl WrappingAdd<$t> for $t {
            type Output = $t;

            /// This is a wrapper over the `wrapping_add` functions in the standard library, for
            /// example [this one](u32::wrapping_add).
            #[inline]
            fn wrapping_add(self, other: $t) -> $t {
                $t::wrapping_add(self, other)
            }
        }

        impl WrappingAddAssign<$t> for $t {
            /// Adds a number to another number in place, wrapping around at the boundary of the
            /// type.
            ///
            /// $x \gets z$, where $z \equiv x + y \mod 2^W$ and $W$ is `Self::WIDTH`.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::wrapping_add#wrapping_add_assign).
            #[inline]
            fn wrapping_add_assign(&mut self, other: $t) {
                *self = self.wrapping_add(other);
            }
        }
    };
}
apply_to_primitive_ints!(impl_wrapping_add);
