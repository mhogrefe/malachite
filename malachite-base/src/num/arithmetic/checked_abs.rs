// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::CheckedAbs;

macro_rules! impl_checked_abs {
    ($t:ident) => {
        impl CheckedAbs for $t {
            type Output = $t;

            /// This is a wrapper over the `checked_abs` functions in the standard library, for
            /// example [this one](i32::checked_abs).
            #[inline]
            fn checked_abs(self) -> Option<$t> {
                $t::checked_abs(self)
            }
        }
    };
}
apply_to_signeds!(impl_checked_abs);
