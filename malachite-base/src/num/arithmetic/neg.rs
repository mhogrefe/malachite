// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::NegAssign;

macro_rules! impl_neg_signed {
    ($t:ident) => {
        impl NegAssign for $t {
            /// Negates a number in place.
            ///
            /// Assumes that the negative can be represented.
            ///
            /// $x \gets -x$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::neg#neg_assign).
            #[inline]
            fn neg_assign(&mut self) {
                *self = -*self;
            }
        }
    };
}
apply_to_signeds!(impl_neg_signed);

macro_rules! impl_neg_float {
    ($t:ident) => {
        impl NegAssign for $t {
            /// Negates a number in place.
            ///
            /// Assumes that the negative can be represented.
            ///
            /// $x \gets -x$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::neg#neg_assign).
            #[inline]
            fn neg_assign(&mut self) {
                *self = -*self;
            }
        }
    };
}
apply_to_primitive_floats!(impl_neg_float);
