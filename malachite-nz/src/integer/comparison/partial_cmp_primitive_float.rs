// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use core::cmp::Ordering;
use malachite_base::num::arithmetic::traits::UnsignedAbs;

macro_rules! impl_float {
    ($t: ident) => {
        impl PartialOrd<$t> for Integer {
            /// Compares an [`Integer`] to a primitive float.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(1)$
            ///
            /// where n = `self.significant_bits()`.
            ///
            /// # Examples
            /// See [here](super::partial_cmp_primitive_float#partial_cmp).
            fn partial_cmp(&self, other: &$t) -> Option<Ordering> {
                if self.sign {
                    self.unsigned_abs().partial_cmp(other)
                } else {
                    self.unsigned_abs()
                        .partial_cmp(&-other)
                        .map(Ordering::reverse)
                }
            }
        }

        impl PartialOrd<Integer> for $t {
            /// Compares a primitive float to an [`Integer`].
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(1)$
            ///
            /// where n = `other.significant_bits()`
            ///
            /// # Examples
            /// See [here](super::partial_cmp_primitive_float#partial_cmp).
            #[inline]
            fn partial_cmp(&self, other: &Integer) -> Option<Ordering> {
                other.partial_cmp(self).map(Ordering::reverse)
            }
        }
    };
}
apply_to_primitive_floats!(impl_float);
