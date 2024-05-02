// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::{AbsDiff, AbsDiffAssign};

macro_rules! impl_abs_diff_unsigned {
    ($t:ident) => {
        impl AbsDiff for $t {
            type Output = $t;

            /// This is a wrapper over the `abs_diff` functions in the standard library, for example
            /// [this one](u32::abs_diff).
            #[inline]
            fn abs_diff(self, other: $t) -> $t {
                self.abs_diff(other)
            }
        }

        impl AbsDiffAssign for $t {
            /// Subtracts a number by another and takes the absolute value, in place. The output
            /// type is the unsigned type with the same width.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::abs_diff#abs_diff_assign).
            #[inline]
            fn abs_diff_assign(&mut self, other: $t) {
                *self = self.abs_diff(other);
            }
        }
    };
}
apply_to_unsigneds!(impl_abs_diff_unsigned);

macro_rules! impl_abs_diff_signed {
    ($u:ident, $s:ident) => {
        impl AbsDiff for $s {
            type Output = $u;

            /// This is a wrapper over the `abs_diff` functions in the standard library, for example
            /// [this one](i32::abs_diff).
            #[inline]
            fn abs_diff(self, other: $s) -> $u {
                self.abs_diff(other)
            }
        }
    };
}
apply_to_unsigned_signed_pairs!(impl_abs_diff_signed);
