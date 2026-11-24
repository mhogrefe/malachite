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

use crate::num::basic::integers::USIZE_IS_U32;
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::num::conversion::traits::WrappingFrom;
use crate::num::factorization::is_prime::{
    redc_context, redc_context_128, redc_mul, redc_mul_128, to_redc, to_redc_128,
};
use crate::num::factorization::traits::{IsLucasProbablePrime, IsSquare};
use crate::num::number_theory::traits::JacobiSymbol;

// Finds the Selfridge parameters for an odd n > 1 that is not a square: the first D in 5, -7, 9,
// -11, ... with (D / n) = -1, and Q = (1 - D) / 4. Returns `None` if gcd(D, n) is a proper factor
// of n for an earlier D. `d_mod_n` computes D mod n.
//
// FLINT's fmpz_is_probabprime_lucas skips every D with (D / n) = 0, rather than stopping when D
// shares a proper factor with n; it therefore also passes some composites with small factors, like
// 1295 = 5 * 7 * 37, that are not Lucas pseudoprimes. FLINT then needs to check that Q is coprime
// to n, but here that always holds. Suppose an odd prime p divides both. Since p <= |Q| < |D|, the
// search has passed D = 9 (if p = 3) or D = ±p (if p >= 5), where (D / n) = 0; it stopped there
// unless n divides that D, which means n = p. But n = p can't divide Q either: that needs |D| >= 4p
// - 1, while the p - 1 candidates 5, 9, ..., 4p - 3 are distinct mod p, so one of them is a
// nonresidue and the search stops sooner.
#[doc(hidden)]
pub fn selfridge_q<T: PartialEq, R: Fn(i64) -> T, J: Fn(&T) -> i8>(
    zero: &T,
    d_mod_n: &R,
    jacobi: &J,
) -> Option<i64> {
    // -3 becomes 5 after the first step.
    let mut d: i64 = -3;
    loop {
        d = if d > 0 { -(d + 2) } else { -(d - 2) };
        let r = d_mod_n(d);
        match jacobi(&r) {
            // D = 1 mod 4, so the shift is exact.
            -1 => return Some((1 - d) >> 2),
            0 if r != *zero => return None,
            _ => {}
        }
    }
}

// The Lucas test for any n, in Montgomery arithmetic: `context` builds the Montgomery context for
// an odd n > 1, `to_redc` converts a number less than n to Montgomery form, and `mul` multiplies
// two numbers in that form, reducing the product mod n.
//
// This is fmpz_is_probabprime_lucas from fmpz/is_probabprime_lucas.c and fmpz_lucas_chain from
// fmpz/lucas_chain.c, FLINT 3.7.0-dev, for one- or two-limb n.
fn is_lucas_probable_prime_redc<
    T: PrimitiveUnsigned + IsSquare + JacobiSymbol<T> + From<u64>,
    C,
    F: FnOnce(T) -> C,
    R: Fn(T, &C) -> T,
    M: Fn(T, T, &C) -> T,
>(
    n: T,
    context: F,
    to_redc: R,
    mul: M,
) -> bool {
    if n <= T::ONE {
        return false;
    }
    if n.even() {
        return n == T::TWO;
    }
    if n.is_square() {
        return false;
    }
    let d_mod_n = |d: i64| {
        let r = T::from(d.unsigned_abs()) % n;
        if d < 0 && r != T::ZERO { n - r } else { r }
    };
    let Some(q) = selfridge_q(&T::ZERO, &d_mod_n, &|d: &T| d.jacobi_symbol(n)) else {
        return false;
    };
    // A = P^2 / Q - 2 mod n, where P = 1; Q is invertible, as selfridge_q explains.
    let a = d_mod_n(q).mod_inverse(n).unwrap().mod_sub(T::TWO, n);
    let ctx = context(n);
    let a = to_redc(a, &ctx);
    // m = (n - (D / n)) / 2 = (n + 1) / 2
    let m = (n >> 1u64) + T::ONE;
    let two = to_redc(T::TWO, &ctx);
    let mut vm = two;
    let mut vm1 = a;
    for bit in (0..m.significant_bits()).rev() {
        if m.get_bit(bit) {
            vm = mul(vm, vm1, &ctx).mod_sub(a, n);
            vm1 = mul(vm1, vm1, &ctx).mod_sub(two, n);
        } else {
            vm1 = mul(vm, vm1, &ctx).mod_sub(a, n);
            vm = mul(vm, vm, &ctx).mod_sub(two, n);
        }
    }
    mul(vm, a, &ctx) == vm1.mod_add(vm1, n)
}

fn u64_is_lucas_probable_prime(n: u64) -> bool {
    is_lucas_probable_prime_redc(n, redc_context, to_redc, redc_mul)
}

// This works for every n; u128_is_lucas_probable_prime uses it for n >= 2^64.
crate_test_fn! {u128_is_lucas_probable_prime_two_limb(n: u128) -> bool {
    is_lucas_probable_prime_redc(n, redc_context_128, to_redc_128, redc_mul_128)
}}

fn u128_is_lucas_probable_prime(n: u128) -> bool {
    if let Ok(n) = u64::try_from(n) {
        u64_is_lucas_probable_prime(n)
    } else {
        u128_is_lucas_probable_prime_two_limb(n)
    }
}

macro_rules! impl_is_lucas_probable_prime {
    ($t:ident) => {
        impl IsLucasProbablePrime for $t {
            /// Tests whether a number is a Lucas probable prime, with Selfridge's parameters.
            ///
            /// Let $D$ be the first of $5, -7, 9, -11, \ldots$ whose Jacobi symbol $(D / n)$ is
            /// $-1$, and let $P = 1$ and $Q = (1 - D) / 4$. Then $n$ is a Lucas probable prime if
            /// $U_{n+1}(P, Q) \equiv 0 \pmod n$, where $U$ is the Lucas sequence. If an earlier $D$
            /// shares a proper factor with $n$, or if $n$ is a perfect square, so that no such $D$
            /// exists, $n$ is not a Lucas probable prime. Of the even numbers, only 2 is one. Every
            /// prime is a Lucas probable prime.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(1)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`: a
            /// perfect-square test, a search for $D$ that ends after a few steps on average, and a
            /// Lucas chain of two modular multiplications per bit.
            ///
            /// # Examples
            /// See [here](super::is_lucas_probable_prime#is_lucas_probable_prime).
            ///
            /// This is equivalent to `fmpz_is_probabprime_lucas` from `fmpz.h`, FLINT 3.7.0-dev,
            /// except that FLINT skips any $D$ with $(D / n) = 0$, and therefore also accepts some
            /// composites with small factors, such as $1295 = 5 \cdot 7 \cdot 37$.
            #[inline]
            fn is_lucas_probable_prime(&self) -> bool {
                u64_is_lucas_probable_prime(u64::from(*self))
            }
        }
    };
}
impl_is_lucas_probable_prime!(u8);
impl_is_lucas_probable_prime!(u16);
impl_is_lucas_probable_prime!(u32);
impl_is_lucas_probable_prime!(u64);

impl IsLucasProbablePrime for u128 {
    /// Tests whether a `u128` is a Lucas probable prime, with Selfridge's parameters.
    ///
    /// Let $D$ be the first of $5, -7, 9, -11, \ldots$ whose Jacobi symbol $(D / n)$ is $-1$, and
    /// let $P = 1$ and $Q = (1 - D) / 4$. Then $n$ is a Lucas probable prime if $U_{n+1}(P, Q)
    /// \equiv 0 \pmod n$, where $U$ is the Lucas sequence. If an earlier $D$ shares a proper factor
    /// with $n$, or if $n$ is a perfect square, so that no such $D$ exists, $n$ is not a Lucas
    /// probable prime. Of the even numbers, only 2 is one. Every prime is a Lucas probable prime.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`: a
    /// perfect-square test, a search for $D$ that ends after a few steps on average, and a Lucas
    /// chain of two modular multiplications per bit.
    ///
    /// # Examples
    /// See [here](super::is_lucas_probable_prime#is_lucas_probable_prime).
    ///
    /// This is equivalent to `fmpz_is_probabprime_lucas` from `fmpz.h`, FLINT 3.7.0-dev, except
    /// that FLINT skips any $D$ with $(D / n) = 0$, and therefore also accepts some composites with
    /// small factors, such as $1295 = 5 \cdot 7 \cdot 37$.
    #[inline]
    fn is_lucas_probable_prime(&self) -> bool {
        u128_is_lucas_probable_prime(*self)
    }
}

impl IsLucasProbablePrime for usize {
    /// Tests whether a `usize` is a Lucas probable prime, with Selfridge's parameters.
    ///
    /// Let $D$ be the first of $5, -7, 9, -11, \ldots$ whose Jacobi symbol $(D / n)$ is $-1$, and
    /// let $P = 1$ and $Q = (1 - D) / 4$. Then $n$ is a Lucas probable prime if $U_{n+1}(P, Q)
    /// \equiv 0 \pmod n$, where $U$ is the Lucas sequence. If an earlier $D$ shares a proper factor
    /// with $n$, or if $n$ is a perfect square, so that no such $D$ exists, $n$ is not a Lucas
    /// probable prime. Of the even numbers, only 2 is one. Every prime is a Lucas probable prime.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`: a
    /// perfect-square test, a search for $D$ that ends after a few steps on average, and a Lucas
    /// chain of two modular multiplications per bit.
    ///
    /// # Examples
    /// See [here](super::is_lucas_probable_prime#is_lucas_probable_prime).
    ///
    /// This is equivalent to `fmpz_is_probabprime_lucas` from `fmpz.h`, FLINT 3.7.0-dev, except
    /// that FLINT skips any $D$ with $(D / n) = 0$, and therefore also accepts some composites with
    /// small factors, such as $1295 = 5 \cdot 7 \cdot 37$.
    #[inline]
    fn is_lucas_probable_prime(&self) -> bool {
        if USIZE_IS_U32 {
            u32::wrapping_from(*self).is_lucas_probable_prime()
        } else {
            u64::wrapping_from(*self).is_lucas_probable_prime()
        }
    }
}
