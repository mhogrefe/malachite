// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use malachite_base::num::comparison::traits::EqAbs;

macro_rules! impl_unsigned {
    ($t: ident) => {
        impl EqAbs<$t> for Natural {
            /// Determines whether the absolute values of a [`Natural`] and a primitive unsigned
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
                self == other
            }
        }

        impl EqAbs<Natural> for $t {
            /// Determines whether the absolute values of a primitive unsigned integer and a
            /// [`Natural`] are equal.
            ///
            /// Since both values are non-negative, this is the same as ordinary equality.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// See [here](super::eq_abs_primitive_int#eq_abs).
            #[inline]
            fn eq_abs(&self, other: &Natural) -> bool {
                self == other
            }
        }
    };
}
apply_to_unsigneds!(impl_unsigned);

macro_rules! impl_signed {
    ($t: ident) => {
        impl EqAbs<$t> for Natural {
            /// Determines whether the absolute values of a [`Natural`] and a primitive signed
            /// integer are equal.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// See [here](super::eq_abs_primitive_int#eq_abs).
            #[inline]
            fn eq_abs(&self, other: &$t) -> bool {
                *self == other.unsigned_abs()
            }
        }

        impl EqAbs<Natural> for $t {
            /// Determines whether the absolute values of a primitive signed integer and a
            /// [`Natural`] are equal.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// See [here](super::eq_abs_primitive_int#eq_abs).
            #[inline]
            fn eq_abs(&self, other: &Natural) -> bool {
                self.unsigned_abs() == *other
            }
        }
    };
}
apply_to_signeds!(impl_signed);
