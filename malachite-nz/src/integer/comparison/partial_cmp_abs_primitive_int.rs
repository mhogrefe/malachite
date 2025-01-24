// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use core::cmp::Ordering;
use malachite_base::num::comparison::traits::PartialOrdAbs;

macro_rules! impl_unsigned {
    ($t: ident) => {
        impl PartialOrdAbs<$t> for Integer {
            /// Compares the absolute values of an [`Integer`] and an unsigned primitive integer.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::partial_cmp_abs_primitive_int#partial_cmp_abs).
            #[inline]
            fn partial_cmp_abs(&self, other: &$t) -> Option<Ordering> {
                self.abs.partial_cmp(other)
            }
        }

        impl PartialOrdAbs<Integer> for $t {
            /// Compares the absolute values of an unsigned primitive integer and an [`Integer`].
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// See [here](super::partial_cmp_abs_primitive_int#partial_cmp_abs).
            #[inline]
            fn partial_cmp_abs(&self, other: &Integer) -> Option<Ordering> {
                other.partial_cmp_abs(self).map(Ordering::reverse)
            }
        }
    };
}
apply_to_unsigneds!(impl_unsigned);

macro_rules! impl_signed {
    ($t: ident) => {
        impl PartialOrdAbs<$t> for Integer {
            /// Compares the absolute values of an [`Integer`] and a signed primitive integer.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::partial_cmp_abs_primitive_int#partial_cmp_abs).
            fn partial_cmp_abs(&self, other: &$t) -> Option<Ordering> {
                self.abs.partial_cmp(&other.unsigned_abs())
            }
        }

        impl PartialOrdAbs<Integer> for $t {
            /// Compares the absolute values of a signed primitive integer and an [`Integer`].
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// See [here](super::partial_cmp_abs_primitive_int#partial_cmp_abs).
            #[inline]
            fn partial_cmp_abs(&self, other: &Integer) -> Option<Ordering> {
                other.partial_cmp_abs(self).map(Ordering::reverse)
            }
        }
    };
}
apply_to_signeds!(impl_signed);
