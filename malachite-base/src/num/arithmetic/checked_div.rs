// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::CheckedDiv;

macro_rules! impl_checked_div {
    ($t:ident) => {
        impl CheckedDiv<$t> for $t {
            type Output = $t;

            /// This is a wrapper over the `checked_div` functions in the standard library, for
            /// example [this one](u32::checked_div).
            #[inline]
            fn checked_div(self, other: $t) -> Option<$t> {
                $t::checked_div(self, other)
            }
        }
    };
}
apply_to_primitive_ints!(impl_checked_div);
