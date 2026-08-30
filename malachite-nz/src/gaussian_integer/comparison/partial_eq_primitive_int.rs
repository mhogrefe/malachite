// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::gaussian_integer::GaussianInteger;

macro_rules! impl_unsigned {
    ($t: ident) => {
        impl PartialEq<$t> for GaussianInteger {
            /// Determines whether a [`GaussianInteger`] is equal to an unsigned primitive integer.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::partial_eq_primitive_int#partial_eq).
            #[inline]
            fn eq(&self, other: &$t) -> bool {
                self.imaginary == 0u32 && self.real == *other
            }
        }

        impl PartialEq<GaussianInteger> for $t {
            /// Determines whether an unsigned primitive integer is equal to a [`GaussianInteger`].
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::partial_eq_primitive_int#partial_eq).
            #[inline]
            fn eq(&self, other: &GaussianInteger) -> bool {
                other == self
            }
        }
    };
}
apply_to_unsigneds!(impl_unsigned);

macro_rules! impl_signed {
    ($t: ident) => {
        impl PartialEq<$t> for GaussianInteger {
            /// Determines whether a [`GaussianInteger`] is equal to a signed primitive integer.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::partial_eq_primitive_int#partial_eq).
            #[inline]
            fn eq(&self, other: &$t) -> bool {
                self.imaginary == 0u32 && self.real == *other
            }
        }

        impl PartialEq<GaussianInteger> for $t {
            /// Determines whether a signed primitive integer is equal to a [`GaussianInteger`].
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::partial_eq_primitive_int#partial_eq).
            #[inline]
            fn eq(&self, other: &GaussianInteger) -> bool {
                other == self
            }
        }
    };
}
apply_to_signeds!(impl_signed);
