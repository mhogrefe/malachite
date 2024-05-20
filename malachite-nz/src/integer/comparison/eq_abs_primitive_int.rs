// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use malachite_base::num::comparison::traits::EqAbs;

macro_rules! impl_unsigned {
    ($t: ident) => {
        impl EqAbs<$t> for Integer {
            /// Determines whether the absolute values of an [`Integer`] and a primitive unsigned
            /// integer are equal.
            ///
            /// Since both values are non-negative, this is the same as ordinary equality.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// See [here](super::eq_abs_primitive_int#eq_abs).
            #[inline]
            fn eq_abs(&self, other: &$t) -> bool {
                self.unsigned_abs_ref() == other
            }
        }

        impl EqAbs<Integer> for $t {
            /// Determines whether the absolute values of a primitive unsigned integer and an
            /// [`Integer`] are equal.
            ///
            /// Since both values are non-negative, this is the same as ordinary equality.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// See [here](super::eq_abs_primitive_int#eq_abs).
            #[inline]
            fn eq_abs(&self, other: &Integer) -> bool {
                self == other.unsigned_abs_ref()
            }
        }
    };
}
apply_to_unsigneds!(impl_unsigned);

macro_rules! impl_signed {
    ($t: ident) => {
        impl EqAbs<$t> for Integer {
            /// Determines whether the absolute values of an [`Integer`] and a primitive signed
            /// integer are equal.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// See [here](super::eq_abs_primitive_int#eq_abs).
            #[inline]
            fn eq_abs(&self, other: &$t) -> bool {
                *self.unsigned_abs_ref() == other.unsigned_abs()
            }
        }

        impl EqAbs<Integer> for $t {
            /// Determines whether the absolute values of a primitive signed integer and an
            /// [`Integer`] are equal.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// See [here](super::eq_abs_primitive_int#eq_abs).
            #[inline]
            fn eq_abs(&self, other: &Integer) -> bool {
                self.unsigned_abs() == *other.unsigned_abs_ref()
            }
        }
    };
}
apply_to_signeds!(impl_signed);
