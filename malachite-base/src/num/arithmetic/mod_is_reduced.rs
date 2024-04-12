// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::ModIsReduced;

macro_rules! impl_mod_is_reduced {
    ($t:ident) => {
        impl ModIsReduced for $t {
            /// Returns whether a number is reduced modulo another number $m$; in other words,
            /// whether it is less than $m$. $m$ cannot be zero.
            ///
            /// $f(x, m) = (x < m)$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if $m$ is 0.
            ///
            /// # Examples
            /// See [here](super::mod_is_reduced#mod_is_reduced).
            #[inline]
            fn mod_is_reduced(&self, m: &$t) -> bool {
                assert_ne!(*m, 0);
                self < m
            }
        }
    };
}
apply_to_unsigneds!(impl_mod_is_reduced);
