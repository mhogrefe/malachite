// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2014 William Hart
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::Parity;
use crate::num::basic::integers::USIZE_IS_U32;
use crate::num::conversion::traits::{ExactFrom, WrappingFrom};
use crate::num::factorization::is_prime::{
    is_strong_probable_prime_redc, redc_context, to_redc_unreduced,
    u128_is_strong_probable_prime_odd,
};
use crate::num::factorization::traits::IsStrongProbablePrime;
use crate::num::logic::traits::{BitAccess, SignificantBits};
use crate::num::modular::traits::{ModMulAssign, ModPow, ModSquareAssign};

// This is fmpz_is_strong_probabprime from fmpz/is_strong_probabprime.c, FLINT 3.7.0-dev, for
// single-limb n.
fn u64_is_strong_probable_prime(n: u64, base: u64) -> bool {
    if n <= 1 {
        return false;
    }
    let a = base % n;
    let nm1 = n - 1;
    if a == 0 || a == 1 || a == nm1 {
        return true;
    }
    if n.even() {
        // n - 1 is odd, so there is no squaring stage.
        let y = a.mod_pow(nm1, n);
        return y == 1 || y == nm1;
    }
    let ctx = redc_context(n);
    let one_redc = to_redc_unreduced(1, &ctx);
    is_strong_probable_prime_redc(a, nm1 >> nm1.trailing_zeros(), one_redc, &ctx)
}

// Computes a^exp mod n, for a < n.
fn u128_mod_pow(a: u128, exp: u128, n: u128) -> u128 {
    let mut y = 1 % n;
    for bit in (0..exp.significant_bits()).rev() {
        y.mod_square_assign(n);
        if exp.get_bit(bit) {
            y.mod_mul_assign(a, n);
        }
    }
    y
}

// This is fmpz_is_strong_probabprime from fmpz/is_strong_probabprime.c, FLINT 3.7.0-dev, for
// two-limb n.
fn u128_is_strong_probable_prime(n: u128, base: u128) -> bool {
    if n <= 1 {
        return false;
    }
    if let Ok(n) = u64::try_from(n) {
        return u64_is_strong_probable_prime(n, u64::exact_from(base % u128::from(n)));
    }
    let a = base % n;
    let nm1 = n - 1;
    if a == 0 || a == 1 || a == nm1 {
        return true;
    }
    if n.even() {
        // n - 1 is odd, so there is no squaring stage.
        let y = u128_mod_pow(a, nm1, n);
        return y == 1 || y == nm1;
    }
    u128_is_strong_probable_prime_odd(n, a)
}

macro_rules! impl_is_strong_probable_prime {
    ($t:ident) => {
        impl IsStrongProbablePrime for $t {
            /// Tests whether a number is a strong probable prime to a given base.
            ///
            /// Let $a$ be the base reduced mod $n$. If $a$ is 0, 1, or $n - 1$, the result is
            /// `true`. Otherwise, writing $n - 1 = 2^s d$ with $d$ odd, $n$ is a strong probable
            /// prime to base $a$ if $a^d \equiv 1 \pmod n$, or if $a^{2^r d} \equiv -1 \pmod n$ for
            /// some $r$ with $0 \leq r < s$. Every prime is a strong probable prime to every base,
            /// and 0 and 1 are not to any base.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(1)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`:
            /// one modular powering, with one or two modular multiplications per exponent bit.
            ///
            /// # Examples
            /// See [here](super::is_strong_probable_prime#is_strong_probable_prime).
            ///
            /// This is equivalent to `fmpz_is_strong_probabprime` from `fmpz.h`, FLINT 3.7.0-dev.
            #[inline]
            fn is_strong_probable_prime(&self, base: $t) -> bool {
                u64_is_strong_probable_prime(u64::from(*self), u64::from(base))
            }
        }
    };
}
impl_is_strong_probable_prime!(u8);
impl_is_strong_probable_prime!(u16);
impl_is_strong_probable_prime!(u32);
impl_is_strong_probable_prime!(u64);

impl IsStrongProbablePrime for u128 {
    /// Tests whether a `u128` is a strong probable prime to a given base.
    ///
    /// Let $a$ be the base reduced mod $n$. If $a$ is 0, 1, or $n - 1$, the result is `true`.
    /// Otherwise, writing $n - 1 = 2^s d$ with $d$ odd, $n$ is a strong probable prime to base $a$
    /// if $a^d \equiv 1 \pmod n$, or if $a^{2^r d} \equiv -1 \pmod n$ for some $r$ with $0 \leq r <
    /// s$. Every prime is a strong probable prime to every base, and 0 and 1 are not to any base.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`: one
    /// modular powering, with one or two modular multiplications per exponent bit.
    ///
    /// # Examples
    /// See [here](super::is_strong_probable_prime#is_strong_probable_prime).
    ///
    /// This is equivalent to `fmpz_is_strong_probabprime` from `fmpz.h`, FLINT 3.7.0-dev.
    #[inline]
    fn is_strong_probable_prime(&self, base: Self) -> bool {
        u128_is_strong_probable_prime(*self, base)
    }
}

impl IsStrongProbablePrime for usize {
    /// Tests whether a `usize` is a strong probable prime to a given base.
    ///
    /// Let $a$ be the base reduced mod $n$. If $a$ is 0, 1, or $n - 1$, the result is `true`.
    /// Otherwise, writing $n - 1 = 2^s d$ with $d$ odd, $n$ is a strong probable prime to base $a$
    /// if $a^d \equiv 1 \pmod n$, or if $a^{2^r d} \equiv -1 \pmod n$ for some $r$ with $0 \leq r <
    /// s$. Every prime is a strong probable prime to every base, and 0 and 1 are not to any base.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`: one
    /// modular powering, with one or two modular multiplications per exponent bit.
    ///
    /// # Examples
    /// See [here](super::is_strong_probable_prime#is_strong_probable_prime).
    ///
    /// This is equivalent to `fmpz_is_strong_probabprime` from `fmpz.h`, FLINT 3.7.0-dev.
    #[inline]
    fn is_strong_probable_prime(&self, base: Self) -> bool {
        if USIZE_IS_U32 {
            u32::wrapping_from(*self).is_strong_probable_prime(u32::wrapping_from(base))
        } else {
            u64::wrapping_from(*self).is_strong_probable_prime(u64::wrapping_from(base))
        }
    }
}
