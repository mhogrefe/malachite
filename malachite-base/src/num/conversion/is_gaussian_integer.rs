// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::conversion::traits::{IsGaussianInteger, IsInteger};

macro_rules! impl_is_gaussian_integer_primitive_int {
    ($t:ident) => {
        impl IsGaussianInteger for $t {
            /// Determines whether a value is a Gaussian integer.
            ///
            /// For primitive integer types this always returns `true`.
            ///
            /// $f(x) = \textrm{true}$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::is_gaussian_integer#is_gaussian_integer).
            #[inline]
            fn is_gaussian_integer(self) -> bool {
                true
            }
        }
    };
}
apply_to_primitive_ints!(impl_is_gaussian_integer_primitive_int);

macro_rules! impl_is_gaussian_integer_primitive_float {
    ($t:ident) => {
        impl IsGaussianInteger for $t {
            /// Determines whether a value is a Gaussian integer.
            ///
            /// A primitive float is real (or `NaN` or infinite), so it is a Gaussian integer if and
            /// only if it is an integer.
            ///
            /// $f(x) = (x \in \Z\[i\])$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::is_gaussian_integer#is_gaussian_integer).
            #[inline]
            fn is_gaussian_integer(self) -> bool {
                self.is_integer()
            }
        }
    };
}
apply_to_primitive_floats!(impl_is_gaussian_integer_primitive_float);
