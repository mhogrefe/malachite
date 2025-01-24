// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::ModPowerOf2IsReduced;
use crate::num::logic::traits::SignificantBits;

macro_rules! impl_mod_power_of_2_is_reduced {
    ($t:ident) => {
        impl ModPowerOf2IsReduced for $t {
            /// Returns whether a number is reduced modulo another number $2^k$; in other words,
            /// whether it has no more than $k$ significant bits.
            ///
            /// $f(x, k) = (x < 2^k)$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::mod_power_of_2_is_reduced#mod_power_of_2_is_reduced).
            #[inline]
            fn mod_power_of_2_is_reduced(&self, pow: u64) -> bool {
                self.significant_bits() <= pow
            }
        }
    };
}
apply_to_unsigneds!(impl_mod_power_of_2_is_reduced);
