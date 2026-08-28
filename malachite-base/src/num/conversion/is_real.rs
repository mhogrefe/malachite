// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::conversion::traits::IsReal;

macro_rules! impl_is_real_primitive_int {
    ($t:ident) => {
        impl IsReal for $t {
            /// Determines whether a value is a real number.
            ///
            /// For primitive integer types this always returns `true`.
            ///
            /// $f(x) = \textrm{true}$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::is_real#is_real).
            #[inline]
            fn is_real(self) -> bool {
                true
            }
        }
    };
}
apply_to_primitive_ints!(impl_is_real_primitive_int);

macro_rules! impl_is_real_primitive_float {
    ($t:ident) => {
        impl IsReal for $t {
            /// Determines whether a value is a real number. `NaN` and the infinities are not real
            /// numbers.
            ///
            /// $f(x) = (x \in \R)$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::is_real#is_real).
            #[inline]
            fn is_real(self) -> bool {
                self.is_finite()
            }
        }
    };
}
apply_to_primitive_floats!(impl_is_real_primitive_float);
