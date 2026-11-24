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
use crate::natural::modular::eq_mod::limbs_mod_exact_odd_limb;
use crate::platform::Limb;
use malachite_base::num::arithmetic::traits::Parity;
use malachite_base::num::basic::traits::Two;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::factorization::traits::{
    IsLucasProbablePrime, IsPrime, IsProbablePrime, IsSquare, IsStrongProbablePrime, Primes,
};
use malachite_base::num::logic::traits::SignificantBits;

// Returns the index i, with start <= i < stop, of the first prime p_i (counting from p_0 = 2) that
// divides xs, or 0 if there is none. Requires start >= 1, so that every prime tried is odd.
//
// This is flint_mpn_factor_trial from mpn_extras/factor_trial.c, FLINT 3.7.0-dev.
crate_test_fn! {limbs_factor_trial(xs: &[Limb], start: usize, stop: usize) -> usize {
    for (i, p) in Limb::primes().enumerate().take(stop).skip(start) {
        if limbs_mod_exact_odd_limb(xs, p, 0) == 0 {
            return i;
        }
    }
    0
}}

// This is fmpz_is_probabprime from fmpz/is_prime.c, FLINT 3.7.0-dev.
fn natural_is_probable_prime(n: &Natural) -> bool {
    if let Ok(x) = u64::try_from(n) {
        return x.is_prime();
    }
    if let Ok(x) = u128::try_from(n) {
        return x.is_probable_prime();
    }
    let xs = n.as_limbs_asc();
    if xs[0].even() {
        return false;
    }
    // FLINT tries the first size * FLINT_BITS + FLINT_BIT_COUNT(d[size - 1]) primes, which on a
    // 64-bit machine is n.significant_bits() + 64; this keeps that count for every limb size.
    let trial_primes = usize::exact_from(n.significant_bits() + 64);
    limbs_factor_trial(xs, 1, trial_primes) == 0
        && !n.is_square()
        && n.is_strong_probable_prime(&Natural::TWO)
        && n.is_lucas_probable_prime()
}

impl IsProbablePrime for Natural {
    /// Tests whether a [`Natural`] is a probable prime, using the Baillie–PSW test. The
    /// [`Natural`] is taken by reference.
    ///
    /// Inputs less than $2^{64}$ are tested exactly, and inputs less than $2^{128}$ are tested as
    /// `u128`s. A larger odd input $n$ with $b$ significant bits is checked for divisibility by the
    /// first $b + 63$ odd primes and for being a perfect square, and must then pass a strong
    /// probable prime test to base 2 and a Lucas probable prime test with Selfridge's parameters.
    /// No composite number is known to pass both.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^2 \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::factorization::traits::IsProbablePrime;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert!(Natural::from(97u32).is_probable_prime());
    /// assert!(!Natural::from(91u32).is_probable_prime());
    ///
    /// // 2^128 + 51
    /// let n = Natural::from_str("340282366920938463463374607431768211507").unwrap();
    /// assert!(n.is_probable_prime());
    ///
    /// // 2^128 + 1 = 59649589127497217 * 5704689200685129054721
    /// let n = Natural::from_str("340282366920938463463374607431768211457").unwrap();
    /// assert!(!n.is_probable_prime());
    ///
    /// // A strong pseudoprime to base 2, which the Lucas test rejects
    /// let n = Natural::from_str("1971171711155257004609808164365888160584741").unwrap();
    /// assert!(!n.is_probable_prime());
    /// ```
    ///
    /// This is equivalent to `fmpz_is_probabprime` from `fmpz.h`, FLINT 3.7.0-dev.
    #[inline]
    fn is_probable_prime(&self) -> bool {
        natural_is_probable_prime(self)
    }
}
