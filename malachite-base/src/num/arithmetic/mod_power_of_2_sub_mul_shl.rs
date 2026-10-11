// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::{ModPowerOf2SubMulShl, ModPowerOf2SubMulShlAssign};
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

// When `bits` is at least `pow`, the shifted product vanishes modulo `2^pow`. Otherwise `bits` is
// less than `T::WIDTH`, so the shift is valid.
fn mod_power_of_2_sub_mul_shl<T: PrimitiveUnsigned>(x: T, y: T, z: T, bits: u64, pow: u64) -> T {
    assert_reduced(x, y, z, pow);
    if bits >= pow {
        x
    } else {
        x.wrapping_sub(y.wrapping_mul(z) << bits)
            .mod_power_of_2(pow)
    }
}

macro_rules! impl_mod_power_of_2_sub_mul_shl {
    ($t:ident) => {
        impl ModPowerOf2SubMulShl<$t> for $t {
            type Output = $t;

            /// Subtracts the product of two numbers, shifted left by `bits`, from a third number,
            /// modulo $2^k$. All three inputs must be already reduced modulo $2^k$.
            ///
            /// $f(x, y, z, b, k) = w$, where $x, y, z, w < 2^k$ and $x - yz2^b \equiv w \mod 2^k$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `pow` is greater than `Self::WIDTH` or if `self`, `y`, or `z` are greater
            /// than or equal to $2^k$.
            ///
            /// # Examples
            /// See [here](super::mod_power_of_2_sub_mul_shl#mod_power_of_2_sub_mul_shl).
            #[inline]
            fn mod_power_of_2_sub_mul_shl(self, y: $t, z: $t, bits: u64, pow: u64) -> $t {
                mod_power_of_2_sub_mul_shl(self, y, z, bits, pow)
            }
        }

        impl ModPowerOf2SubMulShlAssign<$t> for $t {
            /// Subtracts the product of two numbers, shifted left by `bits`, from a third number
            /// modulo $2^k$, in place. All three inputs must be already reduced modulo $2^k$.
            ///
            /// $x \gets w$, where $x, y, z, w < 2^k$ and $x - yz2^b \equiv w \mod 2^k$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `pow` is greater than `Self::WIDTH` or if `self`, `y`, or `z` are greater
            /// than or equal to $2^k$.
            ///
            /// # Examples
            /// See [here](super::mod_power_of_2_sub_mul_shl#mod_power_of_2_sub_mul_shl_assign).
            #[inline]
            fn mod_power_of_2_sub_mul_shl_assign(&mut self, y: $t, z: $t, bits: u64, pow: u64) {
                *self = mod_power_of_2_sub_mul_shl(*self, y, z, bits, pow);
            }
        }
    };
}
apply_to_unsigneds!(impl_mod_power_of_2_sub_mul_shl);
