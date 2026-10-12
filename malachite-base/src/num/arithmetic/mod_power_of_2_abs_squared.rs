// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::{
    ModPowerOf2AbsSquared, ModPowerOf2AbsSquaredAssign, ModPowerOf2Square, ModPowerOf2SquareAssign,
};

macro_rules! impl_mod_power_of_2_abs_squared {
    ($t:ident) => {
        impl ModPowerOf2AbsSquared for $t {
            type Output = $t;

            /// Computes the squared absolute value of a number modulo $2^k$, which for an unsigned
            /// number is its square modulo $2^k$. The input must be already reduced modulo $2^k$.
            ///
            /// $f(x, k) = y$, where $x, y < 2^k$ and $|x|^2 = x^2 \equiv y \mod 2^k$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `pow` is greater than `Self::WIDTH` or if `self` is greater than or equal
            /// to $2^k$.
            ///
            /// # Examples
            /// See [here](super::mod_power_of_2_abs_squared#mod_power_of_2_abs_squared).
            #[inline]
            fn mod_power_of_2_abs_squared(self, pow: u64) -> $t {
                self.mod_power_of_2_square(pow)
            }
        }

        impl ModPowerOf2AbsSquaredAssign for $t {
            /// Replaces a number with its squared absolute value modulo $2^k$, which for an
            /// unsigned number is its square modulo $2^k$. The input must be already reduced modulo
            /// $2^k$.
            ///
            /// $x \gets y$, where $x, y < 2^k$ and $|x|^2 = x^2 \equiv y \mod 2^k$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `pow` is greater than `Self::WIDTH` or if `self` is greater than or equal
            /// to $2^k$.
            ///
            /// # Examples
            /// See [here](super::mod_power_of_2_abs_squared#mod_power_of_2_abs_squared_assign).
            #[inline]
            fn mod_power_of_2_abs_squared_assign(&mut self, pow: u64) {
                self.mod_power_of_2_square_assign(pow);
            }
        }
    };
}
apply_to_unsigneds!(impl_mod_power_of_2_abs_squared);
