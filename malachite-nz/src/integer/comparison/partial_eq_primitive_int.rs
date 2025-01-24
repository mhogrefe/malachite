// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::natural::Natural;
use malachite_base::num::arithmetic::traits::UnsignedAbs;
use malachite_base::num::basic::traits::Zero;

macro_rules! impl_unsigned {
    ($t: ident) => {
        impl PartialEq<$t> for Integer {
            /// Determines whether an [`Integer`] is equal to an unsigned primitive integer.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::partial_eq_primitive_int#partial_eq).
            #[inline]
            fn eq(&self, other: &$t) -> bool {
                self.sign && self.abs == *other
            }
        }

        impl PartialEq<Integer> for $t {
            /// Determines whether an unsigned primitive integer is equal to an [`Integer`].
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::partial_eq_primitive_int#partial_eq).
            #[inline]
            fn eq(&self, other: &Integer) -> bool {
                other == self
            }
        }
    };
}
apply_to_unsigneds!(impl_unsigned);

fn eq_signed<U, S: Copy + Ord + UnsignedAbs<Output = U> + Zero>(x: &Integer, other: &S) -> bool
where
    Natural: PartialEq<U>,
{
    x.sign == (*other >= S::ZERO) && x.abs == other.unsigned_abs()
}

macro_rules! impl_signed {
    ($t: ident) => {
        impl PartialEq<$t> for Integer {
            /// Determines whether an [`Integer`] is equal to a signed primitive integer.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::partial_eq_primitive_int#partial_eq).
            fn eq(&self, other: &$t) -> bool {
                eq_signed(self, other)
            }
        }

        impl PartialEq<Integer> for $t {
            /// Determines whether a signed primitive integer is equal to an [`Integer`].
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::partial_eq_primitive_int#partial_eq).
            #[inline]
            fn eq(&self, other: &Integer) -> bool {
                other == self
            }
        }
    };
}
apply_to_signeds!(impl_signed);
