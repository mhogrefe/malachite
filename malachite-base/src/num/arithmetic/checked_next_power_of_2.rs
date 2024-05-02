// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::CheckedNextPowerOf2;

macro_rules! impl_checked_next_power_of_2 {
    ($t:ident) => {
        impl CheckedNextPowerOf2 for $t {
            type Output = $t;

            /// This is a wrapper over the `checked_next_power_of_two` functions in the standard
            /// library, for example [this one](u32::checked_next_power_of_two).
            #[inline]
            fn checked_next_power_of_2(self) -> Option<$t> {
                $t::checked_next_power_of_two(self)
            }
        }
    };
}
apply_to_unsigneds!(impl_checked_next_power_of_2);
