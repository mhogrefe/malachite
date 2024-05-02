// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use core::cmp::Ordering;
use malachite_base::num::comparison::traits::PartialOrdAbs;

macro_rules! impl_unsigned {
    ($t: ident) => {
        impl PartialOrdAbs<$t> for Natural {
            /// Compares a [`Natural`] to an unsigned primitive integer.
            ///
            /// Since both values are non-negative, this is the same as ordinary
            /// [`partial_cmp`](PartialOrd::partial_cmp).
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// See [here](super::partial_cmp_abs_primitive_int#partial_cmp_abs).
            #[inline]
            fn partial_cmp_abs(&self, other: &$t) -> Option<Ordering> {
                self.partial_cmp(other)
            }
        }

        impl PartialOrdAbs<Natural> for $t {
            /// Compares a value of unsigned primitive integer type to a [`Natural`].
            ///
            /// Since both values are non-negative, this is the same as ordinary
            /// [`partial_cmp`](PartialOrd::partial_cmp).
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// See [here](super::partial_cmp_abs_primitive_int#partial_cmp_abs).
            #[inline]
            fn partial_cmp_abs(&self, other: &Natural) -> Option<Ordering> {
                self.partial_cmp(other)
            }
        }
    };
}
apply_to_unsigneds!(impl_unsigned);

macro_rules! impl_signed {
    ($t: ident) => {
        impl PartialOrdAbs<$t> for Natural {
            /// Compares a [`Natural`] to the absolute value of a signed primitive integer.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// See [here](super::partial_cmp_abs_primitive_int#partial_cmp_abs).
            fn partial_cmp_abs(&self, other: &$t) -> Option<Ordering> {
                self.partial_cmp(&other.unsigned_abs())
            }
        }

        impl PartialOrdAbs<Natural> for $t {
            /// Compares the absolute value of a signed primitive integer to a [`Natural`].
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// See [here](super::partial_cmp_abs_primitive_int#partial_cmp_abs).
            #[inline]
            fn partial_cmp_abs(&self, other: &Natural) -> Option<Ordering> {
                other.partial_cmp_abs(self).map(Ordering::reverse)
            }
        }
    };
}
apply_to_signeds!(impl_signed);
