// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::{DivisibleByPowerOf2, EqModPowerOf2};

macro_rules! impl_eq_mod_power_of_2 {
    ($t:ident) => {
        impl EqModPowerOf2<$t> for $t {
            /// Returns whether one number is equal to another modulo $2^k$.
            ///
            /// $f(x, y, k) = (x \equiv y \mod 2^k)$.
            ///
            /// $f(x, y, k) = (\exists n \in \Z : x - y = n2^k)$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::eq_mod_power_of_2#eq_mod_power_of_2).
            #[inline]
            fn eq_mod_power_of_2(self, other: $t, pow: u64) -> bool {
                (self ^ other).divisible_by_power_of_2(pow)
            }
        }
    };
}
apply_to_primitive_ints!(impl_eq_mod_power_of_2);
