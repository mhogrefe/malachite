// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
#[cfg(not(any(feature = "test_build", feature = "random")))]
use crate::malachite_base::num::arithmetic::traits::Abs;
use malachite_base::num::comparison::traits::EqAbs;

macro_rules! impl_eq_abs {
    ($t: ident) => {
        impl EqAbs<$t> for Integer {
            /// Determines whether the absolute values of an [`Integer`] and a primitive float are
            /// equal.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// See [here](super::eq_abs_primitive_float#eq_abs).
            #[inline]
            fn eq_abs(&self, other: &$t) -> bool {
                *self.unsigned_abs_ref() == other.abs()
            }
        }

        impl EqAbs<Integer> for $t {
            /// Determines whether the absolute values of a primitive float and an [`Integer`] are
            /// equal.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// See [here](super::eq_abs_primitive_float#eq_abs).
            #[inline]
            fn eq_abs(&self, other: &Integer) -> bool {
                self.abs() == *other.unsigned_abs_ref()
            }
        }
    };
}
apply_to_primitive_floats!(impl_eq_abs);
