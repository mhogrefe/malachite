// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::{SaturatingAbs, SaturatingAbsAssign};

macro_rules! impl_saturating_abs {
    ($t:ident) => {
        impl SaturatingAbs for $t {
            type Output = $t;

            /// This is a wrapper over the `saturating_abs` functions in the standard library, for
            /// example [this one](i32::saturating_abs).
            #[inline]
            fn saturating_abs(self) -> $t {
                $t::saturating_abs(self)
            }
        }

        impl SaturatingAbsAssign for $t {
            /// Replaces a number with its absolute value, saturating at the numeric bounds instead
            /// of overflowing.
            ///
            /// $$
            /// x \gets \\begin{cases}
            ///     |x| & \text{if} \\quad x > -2^{W-1}, \\\\
            ///     2^{W-1} - 1 & \text{if} \\quad x = -2^{W-1},
            /// \\end{cases}
            /// $$
            /// where $W$ is `Self::WIDTH`.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::saturating_abs#saturating_abs_assign).
            #[inline]
            fn saturating_abs_assign(&mut self) {
                *self = self.saturating_abs();
            }
        }
    };
}
apply_to_signeds!(impl_saturating_abs);
