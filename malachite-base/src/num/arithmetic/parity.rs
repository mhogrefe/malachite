// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::Parity;

macro_rules! impl_parity {
    ($t:ident) => {
        impl Parity for $t {
            /// Tests whether a number is even.
            ///
            /// $f(x) = (2|x)$.
            ///
            /// $f(x) = (\exists k \in \N \ x = 2k)$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::parity#even).
            #[inline]
            fn even(self) -> bool {
                (self & 1) == 0
            }

            /// Tests whether a number is odd.
            ///
            /// $f(x) = (2\nmid x)$.
            ///
            /// $f(x) = (\exists k \in \N \ x = 2k+1)$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::parity#odd).
            #[inline]
            fn odd(self) -> bool {
                (self & 1) != 0
            }
        }
    };
}
apply_to_primitive_ints!(impl_parity);
