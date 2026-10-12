// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::{
    ModAbsSquared, ModAbsSquaredAssign, ModSquare, ModSquareAssign,
};

macro_rules! impl_mod_abs_squared {
    ($t:ident) => {
        impl ModAbsSquared for $t {
            type Output = $t;

            /// Computes the squared absolute value of a number modulo another number $m$, which for
            /// an unsigned number is its square modulo $m$. The input must be already reduced
            /// modulo $m$.
            ///
            /// $f(x, m) = y$, where $x, y < m$ and $|x|^2 = x^2 \equiv y \mod m$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `self` is greater than or equal to `m`.
            ///
            /// # Examples
            /// See [here](super::mod_abs_squared#mod_abs_squared).
            #[inline]
            fn mod_abs_squared(self, m: $t) -> $t {
                self.mod_square(m)
            }
        }

        impl ModAbsSquaredAssign for $t {
            /// Replaces a number with its squared absolute value modulo another number $m$, which
            /// for an unsigned number is its square modulo $m$. The input must be already reduced
            /// modulo $m$.
            ///
            /// $x \gets y$, where $x, y < m$ and $|x|^2 = x^2 \equiv y \mod m$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `self` is greater than or equal to `m`.
            ///
            /// # Examples
            /// See [here](super::mod_abs_squared#mod_abs_squared_assign).
            #[inline]
            fn mod_abs_squared_assign(&mut self, m: $t) {
                self.mod_square_assign(m);
            }
        }
    };
}
apply_to_unsigneds!(impl_mod_abs_squared);
