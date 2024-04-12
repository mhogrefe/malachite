// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::{SaturatingNeg, SaturatingNegAssign};

macro_rules! impl_saturating_neg {
    ($t:ident) => {
        impl SaturatingNeg for $t {
            type Output = $t;

            /// This is a wrapper over the `saturating_neg` functions in the standard library, for
            /// example [this one](i32::saturating_neg).
            #[inline]
            fn saturating_neg(self) -> $t {
                $t::saturating_neg(self)
            }
        }

        impl SaturatingNegAssign for $t {
            /// Negates a number in place, saturating at the numeric bounds instead of overflowing.
            ///
            /// $$
            /// x \gets \\begin{cases}
            ///     -x & \text{if} \\quad x^2 > -2^{W-1}, \\\\
            ///     2^{W-1} - 1 & \text{if} \\quad x = -2^{W-1},
            /// \\end{cases}
            /// $$
            /// where $W$ is `Self::WIDTH`.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::saturating_neg#saturating_neg_assign).
            #[inline]
            fn saturating_neg_assign(&mut self) {
                *self = self.saturating_neg();
            }
        }
    };
}
apply_to_signeds!(impl_saturating_neg);
