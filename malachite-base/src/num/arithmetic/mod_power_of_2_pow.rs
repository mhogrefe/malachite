// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::{ModPowerOf2Pow, ModPowerOf2PowAssign};
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::num::logic::traits::BitIterable;

fn mod_power_of_2_pow<T: PrimitiveUnsigned>(x: T, exp: u64, pow: u64) -> T {
    assert!(pow <= T::WIDTH);
    assert!(
        x.significant_bits() <= pow,
        "x must be reduced mod 2^pow, but {x} >= 2^{pow}"
    );
    if pow == 0 {
        return T::ZERO;
    }
    let mut out = T::ONE;
    for bit in exp.bits().rev() {
        out.mod_power_of_2_mul_assign(out, pow);
        if bit {
            out.mod_power_of_2_mul_assign(x, pow);
        }
    }
    out
}

macro_rules! impl_mod_power_of_2_pow {
    ($t:ident) => {
        impl ModPowerOf2Pow<u64> for $t {
            type Output = $t;

            /// Raises a number to a power modulo another number $2^k$. The base must be already
            /// reduced modulo $2^k$.
            ///
            /// $f(x, n, k) = y$, where $x, y < 2^k$ and $x^n \equiv y \mod 2^k$.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(1)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `exp.significant_bits()`.
            ///
            /// # Panics
            /// Panics if `pow` is greater than `Self::WIDTH` or if `self` is greater than or equal
            /// to $2^k$.
            ///
            /// # Examples
            /// See [here](super::mod_power_of_2_pow#mod_power_of_2_pow).
            #[inline]
            fn mod_power_of_2_pow(self, exp: u64, pow: u64) -> $t {
                mod_power_of_2_pow(self, exp, pow)
            }
        }

        impl ModPowerOf2PowAssign<u64> for $t {
            /// Raises a number to a power modulo another number $2^k$, in place. The base must be
            /// already reduced modulo $2^k$.
            ///
            /// $x \gets y$, where $x, y < 2^k$ and $x^n \equiv y \mod 2^k$.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(1)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `exp.significant_bits()`.
            ///
            /// # Panics
            /// Panics if `pow` is greater than `Self::WIDTH` or if `self` is greater than or equal
            /// to $2^k$.
            ///
            /// # Examples
            /// See [here](super::mod_power_of_2_pow#mod_power_of_2_pow_assign).
            #[inline]
            fn mod_power_of_2_pow_assign(&mut self, exp: u64, pow: u64) {
                *self = self.mod_power_of_2_pow(exp, pow);
            }
        }
    };
}
apply_to_unsigneds!(impl_mod_power_of_2_pow);
