// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::{CeilingLogBase2, ModPowerOf2Sqrt, Parity};
use crate::num::basic::unsigneds::PrimitiveUnsigned;

// Returns a square root of `u` modulo `2 ^ n`, where `1 <= n <= T::WIDTH`, `u` is reduced modulo `2
// ^ n`, and `u ≡ 1 mod 8` (for `n <= 2` this means that `u` is 1).
//
// This is Newton's iteration for the inverse square root in the 2-adic integers: with `e = 1 -
// uy^2`, the step `y <- y(1 + e / 2)` doubles the precision `j - 2` of an error divisible by `2 ^
// j`, `j >= 3`, so `ceil(log_2(n))` steps from `y = 1` reach precision `n`, and `uy` is a root. The
// wrapping products are correct modulo `2 ^ T::WIDTH`, so masking them to `n` bits gives the
// products modulo `2 ^ n`; `e` is masked before it is halved.
//
// This is `AzZModPow2.oddSqrt` from `Azurite/AzZModPow2/Sqrt.lean`, Azurite, computed modulo `2 ^
// n` rather than modulo the full power.
fn odd_mod_power_of_2_sqrt<T: PrimitiveUnsigned>(u: T, n: u64) -> T {
    let mut y = T::ONE;
    for _ in 0..n.ceiling_log_base_2() {
        let e = T::ONE
            .wrapping_sub(u.wrapping_mul(y).wrapping_mul(y))
            .mod_power_of_2(n);
        y = y.wrapping_add(y.wrapping_mul(e >> 1)).mod_power_of_2(n);
    }
    u.wrapping_mul(y).mod_power_of_2(n)
}

// A nonzero `x = 2 ^ v * u`, with `u` odd, has a root modulo `2 ^ pow` exactly when `v` is even and
// `u ≡ 1 mod 8`; the least root is then `2 ^ (v / 2)` times the least of `±s` and `±s + 2 ^ (n
// - 1)` that squares to `u` modulo `2 ^ n`, where `n = pow - v` and `s` is any root of `u`.
//
// This is equivalent to `AzZModPow2.sqrt?` from `Azurite/AzZModPow2/Sqrt.lean`, Azurite.
fn mod_power_of_2_sqrt<T: PrimitiveUnsigned>(x: T, pow: u64) -> Option<T> {
    assert!(pow <= T::WIDTH);
    assert!(
        x.significant_bits() <= pow,
        "x must be reduced mod 2^pow, but {x} >= 2^{pow}"
    );
    if x == T::ZERO {
        return Some(T::ZERO);
    }
    let v = x.trailing_zeros();
    if v.odd() {
        return None;
    }
    let u = x >> v;
    if u.mod_power_of_2(3) != T::ONE {
        return None;
    }
    // `x` is nonzero and less than `2 ^ pow`, so `v < pow` and `n >= 1`.
    let n = pow - v;
    let s = odd_mod_power_of_2_sqrt(u, n);
    let neg_s = s.wrapping_neg().mod_power_of_2(n);
    let half = T::power_of_2(n - 1);
    // `s` itself is a root, so the minimum exists.
    let least = [
        s,
        neg_s,
        s.wrapping_add(half).mod_power_of_2(n),
        neg_s.wrapping_add(half).mod_power_of_2(n),
    ]
    .into_iter()
    .filter(|&c| c.wrapping_mul(c).mod_power_of_2(n) == u)
    .min()
    .unwrap();
    // `least < 2 ^ n`, so the shifted root is less than `2 ^ (pow - v / 2)` and fits.
    Some(least << (v >> 1))
}

macro_rules! impl_mod_power_of_2_sqrt {
    ($u:ident) => {
        impl ModPowerOf2Sqrt for $u {
            type Output = $u;

            /// Computes the least square root of a number modulo $2^k$. The input must be already
            /// reduced modulo $2^k$.
            ///
            /// Returns `None` if $x$ is not a square modulo $2^k$. A nonzero $x$ is a square modulo
            /// $2^k$ exactly when $x = 4^wu$ with $u \equiv 1 \pmod 8$.
            ///
            /// $f(x, k) = y$, where $x, y < 2^k$, $y^2 \equiv x \mod 2^k$, and $y \leq z$ for every
            /// $z < 2^k$ with $z^2 \equiv x \mod 2^k$.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(\log n)$
            ///
            /// $M(n) = O(1)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `pow`: the Newton iteration
            /// doubles the solved precision each step, so there are $O(\log n)$ steps of
            /// constant-cost word operations.
            ///
            /// # Panics
            /// Panics if `pow` is greater than `Self::WIDTH`, or if `self` is greater than or equal
            /// to $2^k$.
            ///
            /// # Examples
            /// See [here](super::mod_power_of_2_sqrt#mod_power_of_2_sqrt).
            ///
            /// This is equivalent to `AzZModPow2.sqrt?` from `Azurite/AzZModPow2/Sqrt.lean`,
            /// Azurite.
            #[inline]
            fn mod_power_of_2_sqrt(self, pow: u64) -> Option<$u> {
                mod_power_of_2_sqrt(self, pow)
            }
        }
    };
}
apply_to_unsigneds!(impl_mod_power_of_2_sqrt);
