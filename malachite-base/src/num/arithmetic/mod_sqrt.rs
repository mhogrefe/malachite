// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2009 William Hart
//
//      Copyright © 2011 Sebastian Pancratz
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::{CheckedSqrt, JacobiSymbol, ModMulPrecomputed, ModSqrt};
use crate::num::basic::integers::USIZE_IS_U32;
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::num::conversion::traits::WrappingFrom;

// Modular exponentiation by binary exponentiation, with an exponent of the full width of `T`. The
// standard `ModPow` implementations take a `u64` exponent, which is too narrow for `u128` moduli.
// The multiplications share the caller's precomputed data.
fn mod_pow_full_width<T: PrimitiveUnsigned>(
    x: T,
    exp: T,
    m: T,
    data: &<T as ModMulPrecomputed<T, T>>::Data,
) -> T {
    let mut result = T::ONE.mod_op(m);
    let mut base = x;
    let mut exp = exp;
    while exp != T::ZERO {
        if exp.odd() {
            result.mod_mul_precomputed_assign(base, m, data);
        }
        exp >>= 1;
        if exp != T::ZERO {
            base.mod_mul_precomputed_assign(base, m, data);
        }
    }
    result
}

// Computes a square root of `x` modulo `m`, where `x` must be reduced modulo `m`.
//
// This has the same behavior as `n_sqrtmod` from `ulong_extras/sqrtmod.c`, FLINT 3.6.0, except
// that:
// - for even moduli between 50 and 600, FLINT consults a Jacobi-symbol routine whose behavior for
//   even moduli is undefined, and this function does not; and
// - for the moduli `T::MAX` and `T::MAX - 2`, FLINT's `(p + 1) / 4` and `(p + 3) / 8` wrap around,
//   while this function computes them exactly as `(p >> 2) + 1` and `(p >> 3) + 1`, agreeing with
//   what `fmpz_sqrtmod` computes for such moduli. (Both moduli are composite at every width, so
//   both behaviors are anyway outside the odd-prime domain.)
//
// See `mod_sqrt_ref_ref` in `malachite-nz` for the structure; this is the same algorithm.
private_test_fn! {mod_sqrt_unsigned<
    T: CheckedSqrt<Output = T> + JacobiSymbol<T> + PrimitiveUnsigned,
>(
    x: T,
    m: T,
) -> Option<T> {
    assert!(x < m, "x must be reduced mod m, but {x} >= {m}");
    if x <= T::ONE {
        return Some(x);
    }
    // Here x >= 2, so m >= 4.
    if m < T::saturating_from(600u16) {
        if m > T::saturating_from(50u8) && m.odd() && x.jacobi_symbol(m) == -1 {
            return None;
        }
        let limit = (m - T::ONE) >> 1;
        let mut t = T::ZERO;
        let mut t_squared = T::ZERO;
        while t < limit {
            // (t + 1) ^ 2 = t ^ 2 + 2t + 1; 2t + 1 < m since t < (m - 1) / 2
            t_squared.mod_add_assign((t << 1) | T::ONE, m);
            t += T::ONE;
            if t_squared == x {
                return Some(t);
            }
        }
        return None;
    }
    if m.even() {
        return None;
    }
    if m.checked_sqrt().is_some() {
        return None;
    }
    if x.jacobi_symbol(m) == -1 {
        return None;
    }
    let data = T::precompute_mod_mul_data(&m);
    if m.mod_power_of_2(2) == T::from(3u8) {
        // (m + 1) / 4, written without overflow
        return Some(mod_pow_full_width(x, (m >> 2) + T::ONE, m, &data));
    }
    if m.mod_power_of_2(3) == T::from(5u8) {
        // (m + 3) / 8, written without overflow
        let root = mod_pow_full_width(x, (m >> 3) + T::ONE, m, &data);
        if root.mod_mul_precomputed(root, m, &data) == x {
            return Some(root);
        }
        let g = mod_pow_full_width(T::TWO, (m - T::ONE) >> 2, m, &data);
        return Some(g.mod_mul_precomputed(root, m, &data));
    }
    // Tonelli-Shanks. Here m == 1 mod 8, so if m is prime, 2 is a quadratic residue and the
    // smallest nonresidue is odd.
    let mut r = 0u64;
    let mut p1 = m - T::ONE;
    loop {
        p1 >>= 1;
        r += 1;
        if p1.odd() {
            break;
        }
    }
    let mut b = mod_pow_full_width(x, p1, m, &data);
    let mut k = T::from(3u8);
    while k.jacobi_symbol(m) != -1 {
        k += T::TWO;
    }
    let mut g = mod_pow_full_width(k, p1, m, &data);
    let mut root = mod_pow_full_width(x, (p1 >> 1) + T::ONE, m, &data);
    // the maximum number of iterations if m is prime
    let mut iter = r - 1;
    while b != T::ONE {
        let mut b_pow = b;
        let mut new_r = 0;
        loop {
            b_pow.mod_mul_precomputed_assign(b_pow, m, &data);
            new_r += 1;
            if new_r >= r || b_pow == T::ONE {
                break;
            }
        }
        let mut g_pow = g;
        for _ in 1..r - new_r {
            g_pow.mod_mul_precomputed_assign(g_pow, m, &data);
        }
        root.mod_mul_precomputed_assign(g_pow, m, &data);
        g = g_pow.mod_mul_precomputed(g_pow, m, &data);
        b.mod_mul_precomputed_assign(g, m, &data);
        r = new_r;
        if iter == 0 {
            // too many iterations; m is not prime
            root = T::ZERO;
            break;
        }
        iter -= 1;
    }
    if root == T::ZERO { None } else { Some(root) }
}}

macro_rules! impl_mod_sqrt {
    ($t:ident) => {
        impl ModSqrt<$t> for $t {
            type Output = $t;

            /// Computes a square root of a number modulo another number $m$: a $y$ with $y^2 \equiv
            /// x \pmod m$. The input must be already reduced modulo $m$.
            ///
            /// If $m$ is an odd prime, a root is returned whenever one exists, and `None` is
            /// returned exactly when $x$ is a quadratic nonresidue. For other moduli the function
            /// still terminates and is deterministic, but it may return `None` even though a root
            /// exists, and it may return a value that is not a root, so if $m$ is not known to be
            /// prime, a returned root should be verified by squaring. The behavior for such moduli
            /// matches FLINT's, with two exceptions, both involving only composite moduli: for even
            /// moduli between 50 and 600 FLINT consults a Jacobi-symbol routine whose behavior for
            /// even moduli is undefined, and for the two largest odd moduli of a width FLINT's
            /// exponent computations wrap, while this function computes them exactly, as FLINT's
            /// own multiprecision path does.
            ///
            /// $f(x, m) = y$, where $x, y < m$ and $y^2 \equiv x \mod m$, if such a $y$ is found.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `self` is greater than or equal to `m`.
            ///
            /// # Examples
            /// See [here](super::mod_sqrt#mod_sqrt).
            ///
            /// This is equivalent to `n_sqrtmod` from `ulong_extras/sqrtmod.c`, FLINT 3.6.0,
            /// returning an `Option` where FLINT returns 0 for both a failure and a root of 0.
            #[inline]
            fn mod_sqrt(self, m: $t) -> Option<$t> {
                mod_sqrt_unsigned(self, m)
            }
        }
    };
}
impl_mod_sqrt!(u32);
impl_mod_sqrt!(u64);
impl_mod_sqrt!(u128);

macro_rules! impl_mod_sqrt_promoted {
    ($t:ident) => {
        impl ModSqrt<$t> for $t {
            type Output = $t;

            /// Computes a square root of a number modulo another number $m$: a $y$ with $y^2 \equiv
            /// x \pmod m$. The input must be already reduced modulo $m$.
            ///
            /// If $m$ is an odd prime, a root is returned whenever one exists, and `None` is
            /// returned exactly when $x$ is a quadratic nonresidue. For other moduli the function
            /// still terminates and is deterministic, but it may return `None` even though a root
            /// exists, and it may return a value that is not a root, so if $m$ is not known to be
            /// prime, a returned root should be verified by squaring. The behavior for such moduli
            /// matches FLINT's, with two exceptions, both involving only composite moduli: for even
            /// moduli between 50 and 600 FLINT consults a Jacobi-symbol routine whose behavior for
            /// even moduli is undefined, and for the two largest odd moduli of a width FLINT's
            /// exponent computations wrap, while this function computes them exactly, as FLINT's
            /// own multiprecision path does.
            ///
            /// $f(x, m) = y$, where $x, y < m$ and $y^2 \equiv x \mod m$, if such a $y$ is found.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `self` is greater than or equal to `m`.
            ///
            /// # Examples
            /// See [here](super::mod_sqrt#mod_sqrt).
            ///
            /// This is equivalent to `n_sqrtmod` from `ulong_extras/sqrtmod.c`, FLINT 3.6.0,
            /// returning an `Option` where FLINT returns 0 for both a failure and a root of 0.
            #[inline]
            fn mod_sqrt(self, m: $t) -> Option<$t> {
                u32::from(self)
                    .mod_sqrt(u32::from(m))
                    .map($t::wrapping_from)
            }
        }
    };
}
impl_mod_sqrt_promoted!(u8);
impl_mod_sqrt_promoted!(u16);

impl ModSqrt<Self> for usize {
    type Output = Self;

    /// Computes a square root of a number modulo another number $m$: a $y$ with $y^2 \equiv x \pmod
    /// m$. The input must be already reduced modulo $m$.
    ///
    /// If $m$ is an odd prime, a root is returned whenever one exists, and `None` is returned
    /// exactly when $x$ is a quadratic nonresidue. For other moduli the function still terminates
    /// and is deterministic, but it may return `None` even though a root exists, and it may return
    /// a value that is not a root, so if $m$ is not known to be prime, a returned root should be
    /// verified by squaring. The behavior for such moduli matches FLINT's, with two exceptions,
    /// both involving only composite moduli: for even moduli between 50 and 600 FLINT consults a
    /// Jacobi-symbol routine whose behavior for even moduli is undefined, and for the two largest
    /// odd moduli of a width FLINT's exponent computations wrap, while this function computes them
    /// exactly, as FLINT's own multiprecision path does.
    ///
    /// $f(x, m) = y$, where $x, y < m$ and $y^2 \equiv x \mod m$, if such a $y$ is found.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Panics
    /// Panics if `self` is greater than or equal to `m`.
    ///
    /// # Examples
    /// See [here](super::mod_sqrt#mod_sqrt).
    ///
    /// This is equivalent to `n_sqrtmod` from `ulong_extras/sqrtmod.c`, FLINT 3.6.0, returning an
    /// `Option` where FLINT returns 0 for both a failure and a root of 0.
    #[inline]
    fn mod_sqrt(self, m: Self) -> Option<Self> {
        if USIZE_IS_U32 {
            u32::wrapping_from(self)
                .mod_sqrt(u32::wrapping_from(m))
                .map(Self::wrapping_from)
        } else {
            u64::wrapping_from(self)
                .mod_sqrt(u64::wrapping_from(m))
                .map(Self::wrapping_from)
        }
    }
}
