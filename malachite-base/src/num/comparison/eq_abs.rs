// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::comparison::traits::EqAbs;

macro_rules! impl_eq_abs_signed {
    ($t:ident) => {
        impl EqAbs<$t> for $t {
            /// Compares the absolute values of two numbers for equality, taking both by reference.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::eq_abs#eq_abs).
            #[inline]
            fn eq_abs(&self, other: &Self) -> bool {
                self.unsigned_abs() == other.unsigned_abs()
            }
        }
    };
}
apply_to_signeds!(impl_eq_abs_signed);

macro_rules! impl_eq_abs_float {
    ($t:ident) => {
        impl EqAbs<$t> for $t {
            /// Compares the absolute values of two numbers for equality, taking both by reference.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::eq_abs#eq_abs).
            #[inline]
            fn eq_abs(&self, other: &Self) -> bool {
                self.abs() == other.abs()
            }
        }
    };
}
apply_to_primitive_floats!(impl_eq_abs_float);
