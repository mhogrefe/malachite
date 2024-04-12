// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::{OverflowingDiv, OverflowingDivAssign};

macro_rules! impl_overflowing_div {
    ($t:ident) => {
        impl OverflowingDiv<$t> for $t {
            type Output = $t;

            /// This is a wrapper over the `overflowing_div` functions in the standard library, for
            /// example [this one](u32::overflowing_div).
            #[inline]
            fn overflowing_div(self, other: $t) -> ($t, bool) {
                $t::overflowing_div(self, other)
            }
        }

        impl OverflowingDivAssign<$t> for $t {
            /// Divides a number by another number, in place.
            ///
            /// Returns a boolean indicating whether an arithmetic overflow occurred. If an overflow
            /// occurred, then the wrapped value is assigned. Overflow only occurs when `Self` is
            /// signed, `self` is `Self::MIN`, and `other` is -1. The "actual" result, `-Self::MIN`,
            /// can't be represented and is wrapped back to `Self::MIN`.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::overflowing_div#overflowing_div_assign).
            #[inline]
            fn overflowing_div_assign(&mut self, other: $t) -> bool {
                let overflow;
                (*self, overflow) = self.overflowing_div(other);
                overflow
            }
        }
    };
}
apply_to_primitive_ints!(impl_overflowing_div);
