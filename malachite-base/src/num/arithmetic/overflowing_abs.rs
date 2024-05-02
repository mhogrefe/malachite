// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::{OverflowingAbs, OverflowingAbsAssign};

macro_rules! impl_overflowing_abs {
    ($t:ident) => {
        impl OverflowingAbs for $t {
            type Output = $t;

            /// This is a wrapper over the `overflowing_abs` functions in the standard library, for
            /// example [this one](i32::overflowing_abs).
            #[inline]
            fn overflowing_abs(self) -> ($t, bool) {
                $t::overflowing_abs(self)
            }
        }

        impl OverflowingAbsAssign for $t {
            /// Replaces a number with its absolute value.
            ///
            /// Returns a boolean indicating whether an arithmetic overflow occurred. If an overflow
            /// occurred, then the wrapped value is assigned.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::overflowing_abs#overflowing_abs_assign).
            #[inline]
            fn overflowing_abs_assign(&mut self) -> bool {
                let overflow;
                (*self, overflow) = self.overflowing_abs();
                overflow
            }
        }
    };
}
apply_to_signeds!(impl_overflowing_abs);
