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

use crate::natural::Natural;
use malachite_base::num::basic::traits::One;
use malachite_base::num::factorization::traits::IsStrongProbablePrime;
use malachite_base::num::modular::traits::{ModPow, ModSquareAssign};

// This is fmpz_is_strong_probabprime from fmpz/is_strong_probabprime.c, FLINT 3.7.0-dev.
fn natural_is_strong_probable_prime(n: &Natural, base: &Natural) -> bool {
    if *n <= 1u32 {
        return false;
    }
    let nm1 = n - Natural::ONE;
    let a = base % n;
    if a == 1u32 || a == nm1 || a == 0u32 {
        return true;
    }
    let s = nm1.trailing_zeros().unwrap();
    let t = &nm1 >> s;
    let mut y = a.mod_pow(t, n);
    if y == 1u32 || y == nm1 {
        return true;
    }
    for _ in 1..s {
        y.mod_square_assign(n);
        if y == nm1 {
            return true;
        }
    }
    false
}

impl IsStrongProbablePrime<&Self> for Natural {
    /// Tests whether a [`Natural`] is a strong probable prime to a given base. Both [`Natural`]s
    /// are taken by reference.
    ///
    /// Let $a$ be the base reduced mod $n$. If $a$ is 0, 1, or $n - 1$, the result is `true`.
    /// Otherwise, writing $n - 1 = 2^s d$ with $d$ odd, $n$ is a strong probable prime to base $a$
    /// if $a^d \equiv 1 \pmod n$, or if $a^{2^r d} \equiv -1 \pmod n$ for some $r$ with $0 \leq r <
    /// s$. Every prime is a strong probable prime to every base, and 0 and 1 are not to any base.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^2 \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// base.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::basic::traits::Two;
    /// use malachite_base::num::factorization::traits::IsStrongProbablePrime;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert!(Natural::from(97u32).is_strong_probable_prime(&Natural::from(5u32)));
    /// // 2047 = 23 * 89 is the smallest strong pseudoprime to base 2.
    /// assert!(Natural::from(2047u32).is_strong_probable_prime(&Natural::TWO));
    /// assert!(!Natural::from(2047u32).is_strong_probable_prime(&Natural::from(3u32)));
    ///
    /// // 1971171711155257004609808164365888160584741 is a strong pseudoprime to base 2, but not to
    /// // base 3.
    /// let n = Natural::from_str("1971171711155257004609808164365888160584741").unwrap();
    /// assert!(n.is_strong_probable_prime(&Natural::TWO));
    /// assert!(!n.is_strong_probable_prime(&Natural::from(3u32)));
    /// ```
    ///
    /// This is equivalent to `fmpz_is_strong_probabprime` from `fmpz.h`, FLINT 3.7.0-dev.
    #[inline]
    fn is_strong_probable_prime(&self, base: &Self) -> bool {
        natural_is_strong_probable_prime(self, base)
    }
}

impl IsStrongProbablePrime for Natural {
    /// Tests whether a [`Natural`] is a strong probable prime to a given base. The first
    /// [`Natural`] is taken by reference and the base by value.
    ///
    /// Let $a$ be the base reduced mod $n$. If $a$ is 0, 1, or $n - 1$, the result is `true`.
    /// Otherwise, writing $n - 1 = 2^s d$ with $d$ odd, $n$ is a strong probable prime to base $a$
    /// if $a^d \equiv 1 \pmod n$, or if $a^{2^r d} \equiv -1 \pmod n$ for some $r$ with $0 \leq r <
    /// s$. Every prime is a strong probable prime to every base, and 0 and 1 are not to any base.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^2 \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// base.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::basic::traits::Two;
    /// use malachite_base::num::factorization::traits::IsStrongProbablePrime;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert!(Natural::from(97u32).is_strong_probable_prime(Natural::from(5u32)));
    /// // 2047 = 23 * 89 is the smallest strong pseudoprime to base 2.
    /// assert!(Natural::from(2047u32).is_strong_probable_prime(Natural::TWO));
    /// assert!(!Natural::from(2047u32).is_strong_probable_prime(Natural::from(3u32)));
    ///
    /// // 1971171711155257004609808164365888160584741 is a strong pseudoprime to base 2, but not to
    /// // base 3.
    /// let n = Natural::from_str("1971171711155257004609808164365888160584741").unwrap();
    /// assert!(n.is_strong_probable_prime(Natural::TWO));
    /// assert!(!n.is_strong_probable_prime(Natural::from(3u32)));
    /// ```
    ///
    /// This is equivalent to `fmpz_is_strong_probabprime` from `fmpz.h`, FLINT 3.7.0-dev.
    #[inline]
    fn is_strong_probable_prime(&self, base: Self) -> bool {
        natural_is_strong_probable_prime(self, &base)
    }
}
