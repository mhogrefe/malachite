// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::ModPowerOf2Div;
use crate::num::basic::unsigneds::PrimitiveUnsigned;

fn mod_power_of_2_div<T: PrimitiveUnsigned>(x: T, y: T, pow: u64) -> Option<T> {
    assert!(pow <= T::WIDTH);
    assert!(
        x.significant_bits() <= pow,
        "x must be reduced mod 2^pow, but {x} >= 2^{pow}"
    );
    assert!(
        y.significant_bits() <= pow,
        "y must be reduced mod 2^pow, but {y} >= 2^{pow}"
    );
    if x == T::ZERO {
        return Some(T::ZERO);
    }
    if y == T::ZERO {
        return None;
    }
    // y is nonzero and less than 2^pow, so s < pow.
    let s = y.trailing_zeros();
    if x.trailing_zeros() < s {
        return None;
    }
    let k = pow - s;
    Some((x >> s).mod_power_of_2_mul((y >> s).mod_power_of_2_inverse(k).unwrap(), k))
}

macro_rules! impl_mod_power_of_2_div {
    ($t:ident) => {
        impl ModPowerOf2Div<$t> for $t {
            type Output = $t;

            /// Divides a number by another number modulo $2^k$, returning `None` if no quotient
            /// exists. The inputs must be already reduced modulo $2^k$.
            ///
            /// Let $s$ be the number of trailing zeros of $y$. A quotient exists if and only if $x
            /// = 0$, or $y \neq 0$ and $2^s$ divides $x$. If $y$ is even, the quotient is not
            /// unique; all quotients differ by multiples of $2^{k-s}$, and this function returns
            /// the smallest, $(x/2^s)(y/2^s)^{-1} \bmod 2^{k-s}$.
            ///
            /// $f(x, y, k) = \operatorname{Some}(q)$, where $q$ is the least $q < 2^k$ with $qy
            /// \equiv x \mod 2^k$, if such a $q$ exists.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `pow` is greater than `Self::WIDTH`, or if `self` or `other` are greater
            /// than or equal to $2^k$.
            ///
            /// # Examples
            /// See [here](super::mod_power_of_2_div#mod_power_of_2_div).
            #[inline]
            fn mod_power_of_2_div(self, other: $t, pow: u64) -> Option<$t> {
                mod_power_of_2_div(self, other, pow)
            }
        }
    };
}
apply_to_unsigneds!(impl_mod_power_of_2_div);
