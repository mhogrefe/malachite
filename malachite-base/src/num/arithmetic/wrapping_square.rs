// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::{WrappingMulAssign, WrappingSquare, WrappingSquareAssign};

macro_rules! impl_wrapping_square {
    ($t:ident) => {
        impl WrappingSquare for $t {
            type Output = $t;

            /// Squares a number, wrapping around at the boundary of the type.
            ///
            /// $f(x) = y$, where $y \equiv x^2 \mod 2^W$ and $W$ is `Self::WIDTH`.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::wrapping_square#wrapping_square).
            #[inline]
            fn wrapping_square(self) -> $t {
                self.wrapping_mul(self)
            }
        }

        impl WrappingSquareAssign for $t {
            /// Squares a number in place, wrapping around at the boundary of the type.
            ///
            /// $x \gets y$, where $y \equiv x^2 \mod 2^W$ and $W$ is `Self::WIDTH`.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::wrapping_square#wrapping_square_assign).
            #[inline]
            fn wrapping_square_assign(&mut self) {
                self.wrapping_mul_assign(*self);
            }
        }
    };
}
apply_to_primitive_ints!(impl_wrapping_square);
