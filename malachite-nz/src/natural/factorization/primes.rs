// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use malachite_base::num::conversion::traits::SaturatingFrom;
use malachite_base::num::factorization::primes::{PrimesIterator, PrimesLessThanIterator};
use malachite_base::num::factorization::traits::Primes;

/// An iterator over that generates all prime [`Natural`]s less than a given value.
///
/// This `struct` is created by [`Natural::primes_less_than`] and
/// [`Natural::primes_less_than_or_equal_to`]; see their documentation for more.
#[derive(Clone, Debug)]
pub struct NaturalPrimesLessThanIterator(PrimesLessThanIterator<u64>);

impl Iterator for NaturalPrimesLessThanIterator {
    type Item = Natural;

    #[inline]
    fn next(&mut self) -> Option<Natural> {
        self.0.next().map(Natural::from)
    }
}

/// An iterator over that generates all prime [`Natural`]s.
///
/// This `struct` is created by [`Natural::primes`]; see its documentation for more.
#[derive(Clone, Debug)]
pub struct NaturalPrimesIterator(PrimesIterator<u64>);

impl Iterator for NaturalPrimesIterator {
    type Item = Natural;

    #[inline]
    fn next(&mut self) -> Option<Natural> {
        self.0.next().map(Natural::from)
    }
}

impl Primes for Natural {
    type I = NaturalPrimesIterator;
    type LI = NaturalPrimesLessThanIterator;

    /// Returns an iterator that generates all primes less than a given value.
    ///
    /// The iterator produced by `primes_less_than(n)` generates the same primes as the iterator
    /// produced by `primes().take_while(|&p| p < n)`, but the latter would be slower because it
    /// doesn't know in advance how large its prime sieve should be, and might have to create larger
    /// and larger prime sieves.
    ///
    /// # Worst-case complexity (amortized)
    /// $T(i) = O(\log \log i)$
    ///
    /// $M(i) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $i$ is the iteration index.
    ///
    /// # Examples
    /// See [here](super::primes#primes_less_than).
    #[inline]
    fn primes_less_than(n: &Natural) -> NaturalPrimesLessThanIterator {
        NaturalPrimesLessThanIterator(u64::primes_less_than(&u64::saturating_from(n)))
    }

    /// Returns an iterator that generates all primes less than or equal to a given value.
    ///
    /// The iterator produced by `primes_less_than_or_equal_to(n)` generates the same primes as the
    /// iterator produced by `primes().take_while(|&p| p <= n)`, but the latter would be slower
    /// because it doesn't know in advance how large its prime sieve should be, and might have to
    /// create larger and larger prime sieves.
    ///
    /// # Worst-case complexity (amortized)
    /// $T(i) = O(\log \log i)$
    ///
    /// $M(i) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $i$ is the iteration index.
    ///
    /// # Examples
    /// See [here](super::primes#primes_less_than_or_equal_to).
    #[inline]
    fn primes_less_than_or_equal_to(n: &Natural) -> NaturalPrimesLessThanIterator {
        NaturalPrimesLessThanIterator(u64::primes_less_than_or_equal_to(&u64::saturating_from(n)))
    }

    /// Returns all [`Natural`] primes.
    ///
    /// # Worst-case complexity (amortized)
    /// $T(i) = O(\log \log i)$
    ///
    /// $M(i) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $i$ is the iteration index.
    ///
    /// # Examples
    /// See [here](super::primes#primes).
    #[inline]
    fn primes() -> NaturalPrimesIterator {
        NaturalPrimesIterator(u64::primes())
    }
}
