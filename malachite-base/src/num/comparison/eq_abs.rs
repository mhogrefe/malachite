// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::UnsignedAbs;
use crate::num::comparison::traits::EqAbs;

macro_rules! impl_eq_abs_unsigned {
    ($t:ident) => {
        impl EqAbs<$t> for $t {
            /// Compares the absolute values of two numbers for equality, taking both by reference.
            ///
            /// For unsigned values, this is the same as ordinary equality.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::eq_abs#eq_abs).
            #[inline]
            fn eq_abs(&self, other: &Self) -> bool {
                self == other
            }
        }
    };
}
apply_to_unsigneds!(impl_eq_abs_unsigned);

fn eq_abs_signed<U: Eq, S: Copy + UnsignedAbs<Output = U>>(x: &S, y: &S) -> bool {
    x.unsigned_abs() == y.unsigned_abs()
}

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
                eq_abs_signed(self, other)
            }
        }
    };
}
apply_to_signeds!(impl_eq_abs_signed);
