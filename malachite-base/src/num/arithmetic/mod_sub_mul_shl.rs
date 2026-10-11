// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::{ModShl, ModSubMulShl, ModSubMulShlAssign};
use crate::num::basic::unsigneds::PrimitiveUnsigned;

fn assert_reduced<T: PrimitiveUnsigned>(x: T, y: T, z: T, m: T) {
    assert!(x < m, "x must be reduced mod m, but {x} >= {m}");
    assert!(y < m, "y must be reduced mod m, but {y} >= {m}");
    assert!(z < m, "z must be reduced mod m, but {z} >= {m}");
}

fn mod_sub_mul_shl<T: PrimitiveUnsigned + ModShl<u64, T, Output = T>>(
    x: T,
    y: T,
    z: T,
    bits: u64,
    m: T,
) -> T {
    assert_reduced(x, y, z, m);
    x.mod_sub(y.mod_mul(z, m).mod_shl(bits, m), m)
}

macro_rules! impl_mod_sub_mul_shl {
    ($t:ident) => {
        impl ModSubMulShl<$t> for $t {
            type Output = $t;

            /// Subtracts the product of two numbers, shifted left by `bits`, from a third number,
            /// modulo a fourth number $m$. All three inputs must be already reduced modulo $m$.
            ///
            /// $f(x, y, z, b, m) = w$, where $x, y, z, w < m$ and $x - yz2^b \equiv w \mod m$.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(1)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `bits.significant_bits()`.
            ///
            /// # Panics
            /// Panics if `self`, `y`, or `z` are greater than or equal to `m`.
            ///
            /// # Examples
            /// See [here](super::mod_sub_mul_shl#mod_sub_mul_shl).
            #[inline]
            fn mod_sub_mul_shl(self, y: $t, z: $t, bits: u64, m: $t) -> $t {
                mod_sub_mul_shl(self, y, z, bits, m)
            }
        }

        impl ModSubMulShlAssign<$t> for $t {
            /// Subtracts the product of two numbers, shifted left by `bits`, from a third number
            /// modulo a fourth number $m$, in place. All three inputs must be already reduced
            /// modulo $m$.
            ///
            /// $x \gets w$, where $x, y, z, w < m$ and $x - yz2^b \equiv w \mod m$.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(1)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `bits.significant_bits()`.
            ///
            /// # Panics
            /// Panics if `self`, `y`, or `z` are greater than or equal to `m`.
            ///
            /// # Examples
            /// See [here](super::mod_sub_mul_shl#mod_sub_mul_shl_assign).
            #[inline]
            fn mod_sub_mul_shl_assign(&mut self, y: $t, z: $t, bits: u64, m: $t) {
                *self = mod_sub_mul_shl(*self, y, z, bits, m);
            }
        }
    };
}
apply_to_unsigneds!(impl_mod_sub_mul_shl);
