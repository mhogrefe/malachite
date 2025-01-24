// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::ModPowerOf2Inverse;
use crate::num::basic::unsigneds::PrimitiveUnsigned;

// Uses Newton's method, as described by Colin Plumb in
// https://groups.google.com/g/sci.crypt/c/UI-UMbUnYGk/m/hX2-wQVyE3oJ.
pub_test! {mod_power_of_2_inverse_fast<T: PrimitiveUnsigned>(x: T, pow: u64) -> Option<T> {
    assert_ne!(x, T::ZERO);
    assert!(pow <= T::WIDTH);
    assert!(x.significant_bits() <= pow, "x must be reduced mod 2^pow, but {x} >= 2^{pow}");
    if x.even() {
        return None;
    } else if x == T::ONE {
        return Some(T::ONE);
    }
    let mut small_pow = 2;
    let mut inverse = x.mod_power_of_2(2);
    while small_pow < pow {
        small_pow <<= 1;
        if small_pow > pow {
            small_pow = pow;
        }
        // inverse <- inverse * (2 - inverse * x) mod 2^small_pow
        inverse.mod_power_of_2_mul_assign(
            T::TWO.mod_power_of_2_sub(
                inverse.mod_power_of_2_mul(x.mod_power_of_2(small_pow), small_pow),
                small_pow,
            ),
            small_pow,
        );
    }
    Some(inverse)
}}

macro_rules! impl_mod_power_of_2_inverse {
    ($u:ident) => {
        impl ModPowerOf2Inverse for $u {
            type Output = $u;

            /// Computes the multiplicative inverse of a number modulo $2^k$. The input must be
            /// already reduced modulo $2^k$.
            ///
            /// Returns `None` if $x$ is even.
            ///
            /// $f(x, k) = y$, where $x, y < 2^k$, $x$ is odd, and $xy \equiv 1 \mod 2^k$.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(1)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `pow`.
            ///
            /// # Panics
            /// Panics if `pow` is greater than `Self::WIDTH`, if `self` is zero, or if `self` is
            /// greater than or equal to $2^k$.
            ///
            /// # Examples
            /// See [here](super::mod_power_of_2_inverse#mod_power_of_2_inverse).
            #[inline]
            fn mod_power_of_2_inverse(self, pow: u64) -> Option<$u> {
                mod_power_of_2_inverse_fast(self, pow)
            }
        }
    };
}
apply_to_unsigneds!(impl_mod_power_of_2_inverse);
