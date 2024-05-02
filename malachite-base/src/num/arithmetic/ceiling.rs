// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::{Ceiling, CeilingAssign};

macro_rules! impl_ceiling {
    ($f:ident) => {
        impl Ceiling for $f {
            type Output = $f;

            /// This is a wrapper over the `ceil` functions in [`libm`]
            #[inline]
            fn ceiling(self) -> $f {
                libm::Libm::<$f>::ceil(self)
            }
        }

        impl CeilingAssign for $f {
            /// Replaces a number with its ceiling.
            ///
            /// A number's ceiling is the smallest integer greater than or equal to the number.
            ///
            /// $x \gets \lceil x \rceil$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::ceiling#ceiling_assign).
            #[inline]
            fn ceiling_assign(&mut self) {
                *self = self.ceiling();
            }
        }
    };
}
apply_to_primitive_floats!(impl_ceiling);
