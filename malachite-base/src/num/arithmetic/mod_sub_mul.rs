// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::{ModSubMul, ModSubMulAssign};
use crate::num::basic::unsigneds::PrimitiveUnsigned;

fn assert_reduced<T: PrimitiveUnsigned>(x: T, y: T, z: T, m: T) {
    assert!(x < m, "x must be reduced mod m, but {x} >= {m}");
    assert!(y < m, "y must be reduced mod m, but {y} >= {m}");
    assert!(z < m, "z must be reduced mod m, but {z} >= {m}");
}

#[inline]
fn mod_sub_mul<T: PrimitiveUnsigned>(x: T, y: T, z: T, m: T) -> T {
    assert_reduced(x, y, z, m);
    x.mod_sub(y.mod_mul(z, m), m)
}

#[inline]
fn mod_sub_mul_assign<T: PrimitiveUnsigned>(x: &mut T, y: T, z: T, m: T) {
    assert_reduced(*x, y, z, m);
    x.mod_sub_assign(y.mod_mul(z, m), m);
}

macro_rules! impl_mod_sub_mul {
    ($t:ident) => {
        impl ModSubMul<$t> for $t {
            type Output = $t;

            /// Subtracts the product of two numbers from a third number, modulo a fourth number
            /// $m$. All three inputs must be already reduced modulo $m$.
            ///
            /// $f(x, y, z, m) = w$, where $x, y, z, w < m$ and $x - yz \equiv w \mod m$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `self`, `y`, or `z` are greater than or equal to `m`.
            ///
            /// # Examples
            /// See [here](super::mod_sub_mul#mod_sub_mul).
            #[inline]
            fn mod_sub_mul(self, y: $t, z: $t, m: $t) -> $t {
                mod_sub_mul(self, y, z, m)
            }
        }

        impl ModSubMulAssign<$t> for $t {
            /// Subtracts the product of two numbers from a third number, modulo a fourth number
            /// $m$, in place. All three inputs must be already reduced modulo $m$.
            ///
            /// $x \gets w$, where $x, y, z, w < m$ and $x - yz \equiv w \mod m$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `self`, `y`, or `z` are greater than or equal to `m`.
            ///
            /// # Examples
            /// See [here](super::mod_sub_mul#mod_sub_mul_assign).
            #[inline]
            fn mod_sub_mul_assign(&mut self, y: $t, z: $t, m: $t) {
                mod_sub_mul_assign(self, y, z, m);
            }
        }
    };
}
apply_to_unsigneds!(impl_mod_sub_mul);
