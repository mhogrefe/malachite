// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::{Floor, FloorAssign};

macro_rules! impl_floor {
    ($f:ident) => {
        impl Floor for $f {
            type Output = $f;

            /// This is a wrapper over the `floor` functions in [`libm`].
            #[inline]
            fn floor(self) -> $f {
                libm::Libm::<$f>::floor(self)
            }
        }

        impl FloorAssign for $f {
            /// Replaces a number with its floor.
            ///
            /// A number's floor is the largest integer less than or equal to the number.
            ///
            /// $x \gets \lfloor x \rfloor$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::floor#floor_assign).
            #[inline]
            fn floor_assign(&mut self) {
                *self = self.floor();
            }
        }
    };
}
apply_to_primitive_floats!(impl_floor);
