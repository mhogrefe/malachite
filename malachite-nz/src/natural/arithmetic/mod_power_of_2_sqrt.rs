// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use crate::platform::Limb;
use malachite_base::num::arithmetic::traits::{
    CeilingLogBase2, ModPowerOf2, ModPowerOf2Add, ModPowerOf2AddAssign, ModPowerOf2Mul,
    ModPowerOf2Neg, ModPowerOf2Sqrt, ModPowerOf2Square, ModPowerOf2Sub, Parity, PowerOf2,
};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;

// Returns a square root of `u` modulo `2 ^ n`, where `u` is reduced modulo `2 ^ n` and `u ≡ 1 mod
// 8` (for `n <= 2` this means that `u` is 1).
//
// This is Newton's iteration for the inverse square root in the 2-adic integers. With `e = 1 -
// uy^2`, the step `y <- y(1 + e / 2)` (`e` is even) gives `1 - uy'^2 = e^2(3 + e) / 4`, so if `2 ^
// j` divides `e` with `j >= 3`, then `2 ^ (2j - 2)` divides the new error: the precision `j - 2`
// doubles at each step. Starting from `y = 1`, which is valid since `u ≡ 1 mod 8`,
// `ceil(log_2(n))` steps reach precision `n`, and `uy` is a root, since `(uy)^2 = u(uy^2) = u`.
// Halving `e` on its canonical residue determines `y` only modulo `2 ^ (n - 1)`, which is harmless,
// since the top bit of `y` does not affect `uy^2` modulo `2 ^ n`.
//
// This is `AzZModPow2.oddSqrt` from `Azurite/AzZModPow2/Sqrt.lean`, Azurite, computed modulo `2 ^
// n` rather than modulo the full power.
fn odd_mod_power_of_2_sqrt(u: &Natural, n: u64) -> Natural {
    let mut y = Natural::ONE;
    for _ in 0..n.ceiling_log_base_2() {
        let e = Natural::ONE
            .mod_power_of_2_sub(u.mod_power_of_2_mul((&y).mod_power_of_2_square(n), n), n);
        let correction = (&y).mod_power_of_2_mul(e >> 1u32, n);
        y.mod_power_of_2_add_assign(correction, n);
    }
    u.mod_power_of_2_mul(y, n)
}

// A nonzero residue `a = 2 ^ v * u`, with `u` odd and `v < k`, has a root `r` modulo `2 ^ k`
// exactly when `v = 2w` is even and `u` is a square modulo `2 ^ n`, `n = k - v`, which for odd `u`
// means `u ≡ 1 mod 8`; the roots are then `r = 2 ^ w * t` with `t ^ 2 ≡ u mod 2 ^ n`. The roots
// of `u` modulo `2 ^ n` are `±s` and `±s + 2 ^ (n - 1)` for any one root `s`, so the least root
// of `a` is `2 ^ w` times the least of those four residues that squares to `u` modulo `2 ^ n`.
//
// The input must be reduced modulo `2 ^ pow`. This is the general path, used when `pow` is greater
// than `Limb::WIDTH`; for smaller powers, `mod_power_of_2_sqrt_ref` uses the `Limb` implementation.
//
// This is equivalent to `AzZModPow2.sqrt?` from `Azurite/AzZModPow2/Sqrt.lean`, Azurite.
crate_test_fn! {mod_power_of_2_sqrt_natural(a: &Natural, pow: u64) -> Option<Natural> {
    let Some(v) = a.trailing_zeros() else {
        return Some(Natural::ZERO);
    };
    if v.odd() {
        return None;
    }
    let u = a >> v;
    if (&u).mod_power_of_2(3) != 1u32 {
        return None;
    }
    // `a` is nonzero and less than `2 ^ pow`, so `v < pow` and `n >= 1`.
    let n = pow - v;
    let s = odd_mod_power_of_2_sqrt(&u, n);
    let neg_s = (&s).mod_power_of_2_neg(n);
    let half = Natural::power_of_2(n - 1);
    let s_plus_half = (&s).mod_power_of_2_add(&half, n);
    let neg_s_plus_half = (&neg_s).mod_power_of_2_add(half, n);
    // `s` itself is a root, so the minimum exists.
    let least = [s, neg_s, s_plus_half, neg_s_plus_half]
        .into_iter()
        .filter(|c| c.mod_power_of_2_square(n) == u)
        .min()
        .unwrap();
    Some(least << (v >> 1))
}}

fn mod_power_of_2_sqrt_ref(a: &Natural, pow: u64) -> Option<Natural> {
    assert!(
        a.significant_bits() <= pow,
        "self must be reduced mod 2^pow, but {a} >= 2^{pow}"
    );
    if pow <= Limb::WIDTH {
        // A reduced `a` fits in a limb.
        Limb::exact_from(a)
            .mod_power_of_2_sqrt(pow)
            .map(Natural::from)
    } else {
        mod_power_of_2_sqrt_natural(a, pow)
    }
}

impl ModPowerOf2Sqrt for Natural {
    type Output = Self;

    /// Computes the least square root of a [`Natural`] modulo $2^k$. The input must be already
    /// reduced modulo $2^k$. The [`Natural`] is taken by value.
    ///
    /// Returns `None` if $x$ is not a square modulo $2^k$. A nonzero $x$ is a square modulo $2^k$
    /// exactly when $x = 4^wu$ with $u \equiv 1 \pmod 8$.
    ///
    /// $f(x, k) = y$, where $x, y < 2^k$, $y^2 \equiv x \mod 2^k$, and $y \leq z$ for every $z <
    /// 2^k$ with $z^2 \equiv x \mod 2^k$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `pow`.
    ///
    /// # Panics
    /// Panics if `self` is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// See [here](super::mod_power_of_2_sqrt#mod_power_of_2_sqrt).
    ///
    /// This is equivalent to `AzZModPow2.sqrt?` from `Azurite/AzZModPow2/Sqrt.lean`, Azurite.
    #[inline]
    fn mod_power_of_2_sqrt(self, pow: u64) -> Option<Self> {
        mod_power_of_2_sqrt_ref(&self, pow)
    }
}

impl ModPowerOf2Sqrt for &Natural {
    type Output = Natural;

    /// Computes the least square root of a [`Natural`] modulo $2^k$. The input must be already
    /// reduced modulo $2^k$. The [`Natural`] is taken by reference.
    ///
    /// Returns `None` if $x$ is not a square modulo $2^k$. A nonzero $x$ is a square modulo $2^k$
    /// exactly when $x = 4^wu$ with $u \equiv 1 \pmod 8$.
    ///
    /// $f(x, k) = y$, where $x, y < 2^k$, $y^2 \equiv x \mod 2^k$, and $y \leq z$ for every $z <
    /// 2^k$ with $z^2 \equiv x \mod 2^k$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `pow`.
    ///
    /// # Panics
    /// Panics if `self` is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// See [here](super::mod_power_of_2_sqrt#mod_power_of_2_sqrt).
    ///
    /// This is equivalent to `AzZModPow2.sqrt?` from `Azurite/AzZModPow2/Sqrt.lean`, Azurite.
    #[inline]
    fn mod_power_of_2_sqrt(self, pow: u64) -> Option<Natural> {
        mod_power_of_2_sqrt_ref(self, pow)
    }
}
