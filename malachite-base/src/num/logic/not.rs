// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::logic::traits::NotAssign;

macro_rules! impl_not {
    ($t:ident) => {
        impl NotAssign for $t {
            /// Replaces a number with its bitwise negation.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::not#not_assign).
            #[inline]
            fn not_assign(&mut self) {
                *self = !*self;
            }
        }
    };
}
apply_to_primitive_ints!(impl_not);
