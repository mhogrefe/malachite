// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::IsUnit;

macro_rules! impl_is_unit_unsigned {
    ($t:ident) => {
        impl IsUnit for $t {
            /// Determines whether a number is a unit: whether it is 1, the only unsigned integer
            /// with a multiplicative inverse.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::is_unit#is_unit).
            #[inline]
            fn is_unit(&self) -> bool {
                *self == 1
            }
        }
    };
}
apply_to_unsigneds!(impl_is_unit_unsigned);

macro_rules! impl_is_unit_signed {
    ($t:ident) => {
        impl IsUnit for $t {
            /// Determines whether a number is a unit: whether it is 1 or $-1$, the only integers
            /// with a multiplicative inverse.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::is_unit#is_unit).
            #[inline]
            fn is_unit(&self) -> bool {
                *self == 1 || *self == -1
            }
        }
    };
}
apply_to_signeds!(impl_is_unit_signed);

macro_rules! impl_is_unit_primitive_float {
    ($t:ident) => {
        impl IsUnit for $t {
            /// Determines whether a number is a unit: whether it is finite and nonzero, so that it
            /// has a multiplicative inverse. NaN and the infinities are not units.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::is_unit#is_unit).
            #[inline]
            fn is_unit(&self) -> bool {
                self.is_finite() && *self != 0.0
            }
        }
    };
}
apply_to_primitive_floats!(impl_is_unit_primitive_float);
