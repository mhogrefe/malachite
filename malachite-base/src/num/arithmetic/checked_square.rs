// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::CheckedSquare;

macro_rules! impl_checked_square {
    ($t:ident) => {
        impl CheckedSquare for $t {
            type Output = $t;

            /// Squares a number, returning `None` if the result cannot be represented.
            ///
            /// $$
            /// f(x) = \\begin{cases}
            ///     \operatorname{Some}(x^2) & \text{if} \\quad x^2 < 2^W, \\\\
            ///     \operatorname{None} & \text{if} \\quad x^2 \geq 2^W,
            /// \\end{cases}
            /// $$
            /// where $W$ is `Self::WIDTH`.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::checked_square#checked_square).
            #[inline]
            fn checked_square(self) -> Option<$t> {
                self.checked_mul(self)
            }
        }
    };
}
apply_to_primitive_ints!(impl_checked_square);
