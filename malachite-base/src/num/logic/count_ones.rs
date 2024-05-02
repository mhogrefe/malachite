// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::logic::traits::CountOnes;

macro_rules! impl_count_ones {
    ($t:ident) => {
        impl CountOnes for $t {
            /// This is a wrapper over the `count_ones` functions in the standard library, for
            /// example [this one](u32::count_ones).
            #[inline]
            fn count_ones(self) -> u64 {
                u64::from($t::count_ones(self))
            }
        }
    };
}
apply_to_primitive_ints!(impl_count_ones);
