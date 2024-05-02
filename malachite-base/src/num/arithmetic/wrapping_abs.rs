// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::{WrappingAbs, WrappingAbsAssign};

macro_rules! impl_wrapping_abs {
    ($t:ident) => {
        impl WrappingAbs for $t {
            type Output = $t;

            /// This is a wrapper over the `wrapping_abs` functions in the standard library, for
            /// example [this one](i32::wrapping_abs).
            #[inline]
            fn wrapping_abs(self) -> $t {
                $t::wrapping_abs(self)
            }
        }

        impl WrappingAbsAssign for $t {
            /// Replaces a number with its absolute value, wrapping around at the boundary of the
            /// type.
            ///
            /// $$
            /// x \gets \\begin{cases}
            ///     |x| & \text{if} \\quad x > -2^{W-1}, \\\\
            ///     -2^{W-1} & \text{if} \\quad x = -2^{W-1},
            /// \\end{cases}
            /// $$
            /// where $W$ is `Self::WIDTH`.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::wrapping_abs#wrapping_abs_assign).
            #[inline]
            fn wrapping_abs_assign(&mut self) {
                *self = self.wrapping_abs();
            }
        }
    };
}
apply_to_signeds!(impl_wrapping_abs);
