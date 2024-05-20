// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::Sign;
use core::cmp::Ordering::{self, *};

macro_rules! impl_sign_primitive_int {
    ($t:ident) => {
        impl Sign for $t {
            /// Compares a number to zero.
            ///
            /// Returns `Greater`, `Equal`, or `Less`, depending on whether the number is positive,
            /// zero, or negative, respectively.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::sign#sign).
            #[inline]
            fn sign(&self) -> Ordering {
                self.cmp(&0)
            }
        }
    };
}
apply_to_primitive_ints!(impl_sign_primitive_int);

macro_rules! impl_sign_primitive_float {
    ($t:ident) => {
        impl Sign for $t {
            /// Compares a number to zero.
            ///
            /// - Positive finite numbers, positive zero, and positive infinity have sign
            /// `Greater`.
            /// - Negative finite numbers, negative zero, and negative infinity have sign `Less`.
            /// - `NaN` has sign `Equal`.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::sign#sign).
            #[inline]
            fn sign(&self) -> Ordering {
                if self.is_nan() {
                    Equal
                } else if self.is_sign_positive() {
                    Greater
                } else {
                    Less
                }
            }
        }
    };
}
apply_to_primitive_floats!(impl_sign_primitive_float);
