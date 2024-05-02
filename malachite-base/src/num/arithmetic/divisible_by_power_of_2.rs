// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::{DivisibleByPowerOf2, ModPowerOf2};
use crate::num::conversion::traits::WrappingFrom;

macro_rules! impl_divisible_by_power_of_2_unsigned {
    ($t:ident) => {
        impl DivisibleByPowerOf2 for $t {
            /// Returns whether a number is divisible by $2^k$.
            ///
            /// $f(x, k) = (2^k|x)$.
            ///
            /// $f(x, k) = (\exists n \in \N : \ x = n2^k)$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::divisible_by_power_of_2#divisible_by_power_of_2).
            #[inline]
            fn divisible_by_power_of_2(self, pow: u64) -> bool {
                self.mod_power_of_2(pow) == 0
            }
        }
    };
}
apply_to_unsigneds!(impl_divisible_by_power_of_2_unsigned);

macro_rules! impl_divisible_by_power_of_2_signed {
    ($u:ident, $s:ident) => {
        impl DivisibleByPowerOf2 for $s {
            /// Returns whether a number is divisible by $2^k$.
            ///
            /// $f(x, k) = (2^k|x)$.
            ///
            /// $f(x, k) = (\exists n \in \N : x = n2^k)$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::divisible_by_power_of_2#divisible_by_power_of_2).
            #[inline]
            fn divisible_by_power_of_2(self, pow: u64) -> bool {
                $u::wrapping_from(self).divisible_by_power_of_2(pow)
            }
        }
    };
}
apply_to_unsigned_signed_pairs!(impl_divisible_by_power_of_2_signed);
