// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::{AbsSquared, Square};

macro_rules! impl_abs_squared {
    ($t:ident) => {
        impl AbsSquared for $t {
            type Output = $t;

            /// Computes the squared absolute value of a number. For real types this is the same as
            /// squaring.
            ///
            /// $f(x) = |x|^2 = x^2$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::abs_squared#abs_squared).
            #[inline]
            fn abs_squared(self) -> $t {
                self.square()
            }
        }
    };
}
apply_to_primitive_ints!(impl_abs_squared);
apply_to_primitive_floats!(impl_abs_squared);
