// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::{WrappingNeg, WrappingNegAssign};

macro_rules! impl_wrapping_neg {
    ($t:ident) => {
        impl WrappingNeg for $t {
            type Output = $t;

            /// This is a wrapper over the `wrapping_neg` functions in the standard library, for
            /// example [this one](u32::wrapping_neg).
            #[inline]
            fn wrapping_neg(self) -> $t {
                $t::wrapping_neg(self)
            }
        }

        impl WrappingNegAssign for $t {
            /// Negates a number in place, wrapping around at the boundary of the type.
            ///
            /// $x \gets y$, where $y \equiv -x \mod 2^W$ and $W$ is `Self::WIDTH`.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::wrapping_neg#wrapping_neg_assign).
            #[inline]
            fn wrapping_neg_assign(&mut self) {
                *self = self.wrapping_neg();
            }
        }
    };
}
apply_to_primitive_ints!(impl_wrapping_neg);
