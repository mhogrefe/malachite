// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::{ModPowerOf2SubMul, ModPowerOf2SubMulAssign};
use crate::num::basic::unsigneds::PrimitiveUnsigned;

fn assert_reduced<T: PrimitiveUnsigned>(x: T, y: T, z: T, pow: u64) {
    assert!(pow <= T::WIDTH);
    assert!(
        x.significant_bits() <= pow,
        "x must be reduced mod 2^pow, but {x} >= 2^{pow}"
    );
    assert!(
        y.significant_bits() <= pow,
        "y must be reduced mod 2^pow, but {y} >= 2^{pow}"
    );
    assert!(
        z.significant_bits() <= pow,
        "z must be reduced mod 2^pow, but {z} >= 2^{pow}"
    );
}

#[inline]
fn mod_power_of_2_sub_mul<T: PrimitiveUnsigned>(x: T, y: T, z: T, pow: u64) -> T {
    assert_reduced(x, y, z, pow);
    x.wrapping_sub_mul(y, z).mod_power_of_2(pow)
}

#[inline]
fn mod_power_of_2_sub_mul_assign<T: PrimitiveUnsigned>(x: &mut T, y: T, z: T, pow: u64) {
    assert_reduced(*x, y, z, pow);
    x.wrapping_sub_mul_assign(y, z);
    x.mod_power_of_2_assign(pow);
}

macro_rules! impl_mod_power_of_2_sub_mul {
    ($t:ident) => {
        impl ModPowerOf2SubMul<$t> for $t {
            type Output = $t;

            /// Subtracts the product of two numbers from a third number, modulo a fourth number
            /// $2^k$. All three inputs must be already reduced modulo $2^k$.
            ///
            /// $f(x, y, z, k) = w$, where $x, y, z, w < 2^k$ and $x - yz \equiv w \mod 2^k$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `pow` is greater than `Self::WIDTH` or if `self`, `y`, or `z` are greater
            /// than or equal to $2^k$.
            ///
            /// # Examples
            /// See [here](super::mod_power_of_2_sub_mul#mod_power_of_2_sub_mul).
            #[inline]
            fn mod_power_of_2_sub_mul(self, y: $t, z: $t, pow: u64) -> $t {
                mod_power_of_2_sub_mul(self, y, z, pow)
            }
        }

        impl ModPowerOf2SubMulAssign<$t> for $t {
            /// Subtracts the product of two numbers from a third number, modulo a fourth number
            /// $2^k$, in place. All three inputs must be already reduced modulo $2^k$.
            ///
            /// $x \gets w$, where $x, y, z, w < 2^k$ and $x - yz \equiv w \mod 2^k$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `pow` is greater than `Self::WIDTH` or if `self`, `y`, or `z` are greater
            /// than or equal to $2^k$.
            ///
            /// # Examples
            /// See [here](super::mod_power_of_2_sub_mul#mod_power_of_2_sub_mul_assign).
            #[inline]
            fn mod_power_of_2_sub_mul_assign(&mut self, y: $t, z: $t, pow: u64) {
                mod_power_of_2_sub_mul_assign(self, y, z, pow);
            }
        }
    };
}
apply_to_unsigneds!(impl_mod_power_of_2_sub_mul);
