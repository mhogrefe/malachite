// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2014 William Hart
//
//      Copyright © 2020 Fredrik Johansson
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use crate::natural::factorization::is_prime_aprcl::natural_is_prime_aprcl;
use crate::natural::factorization::is_prime_n_pm_1::natural_is_prime_n_pm_1;
use crate::natural::factorization::is_probable_prime::limbs_factor_trial;
use malachite_base::num::arithmetic::traits::Parity;
use malachite_base::num::basic::traits::Two;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::factorization::traits::{IsPrime, IsSquare, IsStrongProbablePrime};
use malachite_base::num::logic::traits::SignificantBits;

// This is fmpz_is_prime from fmpz/is_prime.c, FLINT 3.7.0-dev, with APR-CL as the full primality
// proof.
fn natural_is_prime(n: &Natural) -> bool {
    if let Ok(x) = u128::try_from(n) {
        return x.is_prime();
    }
    let xs = n.as_limbs_asc();
    if xs[0].even() {
        return false;
    }
    let trial_primes = usize::exact_from(n.significant_bits() + 64);
    // Trial division, a square test, and a single base-2 test rule out most composites.
    if limbs_factor_trial(xs, 1, trial_primes) != 0
        || n.is_square()
        || !n.is_strong_probable_prime(&Natural::TWO)
    {
        return false;
    }
    natural_is_prime_n_pm_1(n, |m| Some(m.is_prime())).unwrap_or_else(|| natural_is_prime_aprcl(n))
}

impl IsPrime for Natural {
    /// Tests whether a [`Natural`] is prime. The [`Natural`] is taken by reference.
    ///
    /// Inputs less than $2^{128}$ are tested as `u128`s. A larger odd input $n$ with $b$
    /// significant bits is checked for divisibility by the first $b + 63$ odd primes and for being
    /// a perfect square, and must pass a strong probable prime test to base 2. It is then proven
    /// prime or composite by the $n - 1$ and $n + 1$ tests of Pocklington, Morrison, and Brillhart,
    /// Lehmer, and Selfridge, which use the primes found by trial division of $n - 1$ and $n + 1$,
    /// together with Lenstra's search for divisors in residue classes. If those tests cannot settle
    /// it, the APR-CL test with Jacobi sums, of Adleman, Pomerance, Rumely, Cohen, and Lenstra,
    /// does.
    ///
    /// # Worst-case complexity
    /// $T(n) = n^{O(\log\log n)}$
    ///
    /// $M(n) = n^{O(\log\log n)}$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::factorization::traits::IsPrime;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert!(Natural::from(97u32).is_prime());
    /// assert!(!Natural::from(91u32).is_prime());
    ///
    /// // 2^128 + 51
    /// let n = Natural::from_str("340282366920938463463374607431768211507").unwrap();
    /// assert!(n.is_prime());
    ///
    /// // 2^128 + 1 = 59649589127497217 * 5704689200685129054721
    /// let n = Natural::from_str("340282366920938463463374607431768211457").unwrap();
    /// assert!(!n.is_prime());
    ///
    /// // 2^521 - 1
    /// let n = Natural::from_str(
    ///     "68647976601306097149819007990813932172694353001433054093944634591855431833976560521225\
    ///     59640661454554977296311391480858037121987999716643812574028291115057151",
    /// )
    /// .unwrap();
    /// assert!(n.is_prime());
    /// ```
    ///
    /// This is equivalent to `fmpz_is_prime` from `fmpz.h`, FLINT 3.7.0-dev, except that FLINT uses
    /// the ECPP test instead of APR-CL for large inputs.
    #[inline]
    fn is_prime(&self) -> bool {
        natural_is_prime(self)
    }
}
