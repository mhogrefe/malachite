// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::logic::traits::TrailingZeros;

macro_rules! impl_trailing_zeros {
    ($t:ident) => {
        impl TrailingZeros for $t {
            /// This is a wrapper over the `trailing_zeros` functions in the standard library, for
            /// example [this one](u32::trailing_zeros).
            #[inline]
            fn trailing_zeros(self) -> u64 {
                u64::from($t::trailing_zeros(self))
            }
        }
    };
}
apply_to_primitive_ints!(impl_trailing_zeros);
