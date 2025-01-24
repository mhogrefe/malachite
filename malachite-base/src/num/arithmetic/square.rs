// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::{Square, SquareAssign};

macro_rules! impl_square {
    ($t:ident) => {
        impl Square for $t {
            type Output = $t;

            /// Squares a number.
            ///
            /// $f(x) = x^2$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::square#square).
            #[inline]
            fn square(self) -> $t {
                self * self
            }
        }

        impl SquareAssign for $t {
            /// Squares a number in place.
            ///
            /// $x \gets x^2$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::square#square_assign).
            #[inline]
            fn square_assign(&mut self) {
                *self *= *self;
            }
        }
    };
}
apply_to_primitive_ints!(impl_square);
apply_to_primitive_floats!(impl_square);
