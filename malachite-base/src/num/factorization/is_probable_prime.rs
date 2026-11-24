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

use crate::num::factorization::is_prime::ll_is_prime;
use crate::num::factorization::traits::{IsLucasProbablePrime, IsPrime, IsProbablePrime};

macro_rules! impl_is_probable_prime {
    ($t:ident) => {
        impl IsProbablePrime for $t {
            /// Tests whether a number is a probable prime.
            ///
            /// For this type the answer is exact: proving primality is as fast as the Baillie–PSW
            /// test, so this is the same as [`is_prime`](IsPrime::is_prime).
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(1)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
            ///
            /// # Examples
            /// See [here](super::is_probable_prime#is_probable_prime).
            #[inline]
            fn is_probable_prime(&self) -> bool {
                self.is_prime()
            }
        }
    };
}
impl_is_probable_prime!(u8);
impl_is_probable_prime!(u16);
impl_is_probable_prime!(u32);
impl_is_probable_prime!(u64);
impl_is_probable_prime!(usize);

impl IsProbablePrime for u128 {
    /// Tests whether a `u128` is a probable prime, using the Baillie–PSW test.
    ///
    /// Inputs less than $2^{64}$ are tested exactly, as `u64`s. Larger inputs are checked for
    /// divisibility by the odd primes up to 137, or up to 313 above $357913941 \cdot 2^{64}$, and
    /// must pass a strong probable prime test to base 2. Below $3317044064679887385961981 \approx
    /// 2^{81.5}$, strong probable prime tests to the bases 2, 3, 5, …, 41 then prove primality,
    /// by a result of Sorenson and Webster. Above it, a Lucas probable prime test completes the
    /// Baillie–PSW test, which no known composite passes.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`: trial
    /// division by a bounded set of small primes, then at most 13 strong probable prime tests, or
    /// one together with a Lucas test, each costing a bounded number of modular multiplications per
    /// bit.
    ///
    /// # Examples
    /// See [here](super::is_probable_prime#is_probable_prime).
    ///
    /// This is equivalent to `fmpz_is_probabprime` from `fmpz.h`, FLINT 3.7.0-dev, for two-limb
    /// inputs.
    fn is_probable_prime(&self) -> bool {
        if let Ok(n) = u64::try_from(*self) {
            return n.is_prime();
        }
        ll_is_prime(*self).unwrap_or_else(|| self.is_lucas_probable_prime())
    }
}
