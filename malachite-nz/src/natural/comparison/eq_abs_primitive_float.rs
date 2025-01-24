// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use malachite_base::num::comparison::traits::EqAbs;

macro_rules! impl_eq_abs {
    ($t: ident) => {
        impl EqAbs<$t> for Natural {
            /// Determines whether the absolute values of a [`Natural`] and a primitive float are
            /// equal.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// See [here](super::eq_abs_primitive_float#eq_abs).
            #[inline]
            fn eq_abs(&self, other: &$t) -> bool {
                *self == other.abs()
            }
        }

        impl EqAbs<Natural> for $t {
            /// Determines whether the absolute values of a primitive float and a [`Natural`] are
            /// equal.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// See [here](super::eq_abs_primitive_float#eq_abs).
            #[inline]
            fn eq_abs(&self, other: &Natural) -> bool {
                self.abs() == *other
            }
        }
    };
}
apply_to_primitive_floats!(impl_eq_abs);
