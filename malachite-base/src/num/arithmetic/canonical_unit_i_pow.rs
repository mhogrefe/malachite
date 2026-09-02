// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::CanonicalUnitIPow;

macro_rules! impl_canonical_unit_i_pow_unsigned {
    ($t:ident) => {
        impl CanonicalUnitIPow for $t {
            /// Finds the power of $i$ that brings a number into canonical unit form. An unsigned
            /// number is already in canonical form, so this is always 0.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::canonical_unit_i_pow#canonical_unit_i_pow).
            #[inline]
            fn canonical_unit_i_pow(&self) -> u64 {
                0
            }
        }
    };
}
apply_to_unsigneds!(impl_canonical_unit_i_pow_unsigned);

macro_rules! impl_canonical_unit_i_pow_signed {
    ($t:ident) => {
        impl CanonicalUnitIPow for $t {
            /// Finds the power of $i$ that brings a number into canonical unit form. The canonical
            /// form of a real number is its absolute value, so this is 2 for negative numbers,
            /// since $x i^2 = -x$, and 0 otherwise.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::canonical_unit_i_pow#canonical_unit_i_pow).
            #[inline]
            fn canonical_unit_i_pow(&self) -> u64 {
                if *self < 0 { 2 } else { 0 }
            }
        }
    };
}
apply_to_signeds!(impl_canonical_unit_i_pow_signed);

macro_rules! impl_canonical_unit_i_pow_primitive_float {
    ($t:ident) => {
        impl CanonicalUnitIPow for $t {
            /// Finds the power of $i$ that brings a number into canonical unit form. The canonical
            /// form of a real number is its absolute value, so this is 2 for numbers with the sign
            /// bit set, negative zero and negative infinity included, since $x i^2 = -x$, and 0
            /// otherwise, NaN included.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::canonical_unit_i_pow#canonical_unit_i_pow).
            #[inline]
            fn canonical_unit_i_pow(&self) -> u64 {
                if self.is_sign_negative() && !self.is_nan() {
                    2
                } else {
                    0
                }
            }
        }
    };
}
apply_to_primitive_floats!(impl_canonical_unit_i_pow_primitive_float);
