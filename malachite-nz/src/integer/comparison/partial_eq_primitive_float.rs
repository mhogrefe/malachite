// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use malachite_base::num::arithmetic::traits::UnsignedAbs;

macro_rules! impl_float {
    ($t: ident) => {
        impl PartialEq<$t> for Integer {
            /// Determines whether an [`Integer`] is equal to a primitive float.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(1)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
            ///
            /// # Examples
            /// See [here](super::partial_eq_primitive_float#partial_eq).
            fn eq(&self, other: &$t) -> bool {
                if self.sign {
                    self.unsigned_abs() == *other
                } else {
                    self.unsigned_abs() == -other
                }
            }
        }

        impl PartialEq<Integer> for $t {
            /// Determines whether a primitive float is equal to an [`Integer`].
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(1)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `other.significant_bits()`.
            ///
            /// # Examples
            /// See [here](super::partial_eq_primitive_float#partial_eq).
            #[inline]
            fn eq(&self, other: &Integer) -> bool {
                other == self
            }
        }
    };
}
apply_to_primitive_floats!(impl_float);
