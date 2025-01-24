// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::{OverflowingSub, OverflowingSubAssign};

macro_rules! impl_overflowing_sub {
    ($t:ident) => {
        impl OverflowingSub<$t> for $t {
            type Output = $t;

            /// This is a wrapper over the `overflowing_sub` functions in the standard library, for
            /// example [this one](u32::overflowing_sub).
            #[inline]
            fn overflowing_sub(self, other: $t) -> ($t, bool) {
                $t::overflowing_sub(self, other)
            }
        }

        impl OverflowingSubAssign<$t> for $t {
            /// Subtracts a number by another number, in place.
            ///
            /// Returns a boolean indicating whether an arithmetic overflow occurred. If an overflow
            /// occurred, then the wrapped value is assigned.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::overflowing_sub#overflowing_sub_assign).
            #[inline]
            fn overflowing_sub_assign(&mut self, other: $t) -> bool {
                let overflow;
                (*self, overflow) = self.overflowing_sub(other);
                overflow
            }
        }
    };
}
apply_to_primitive_ints!(impl_overflowing_sub);
