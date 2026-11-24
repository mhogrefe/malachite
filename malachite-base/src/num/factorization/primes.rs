// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the GNU MP Library and the FLINT Library.
//
// Prime sieve code contributed to the GNU project by Marco Bodrato.
//
//      Copyright © 2009 Tom Boothby
//
//      Copyright © 2009 William Hart
//
//      Copyright © 2010 Fredrik Johansson
//
//      Copyright © 2010–2012, 2015, 2016 Free Software Foundation, Inc.
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::DivMod;
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::num::conversion::traits::{ExactFrom, WrappingFrom};
use crate::num::factorization::is_prime_tables::MOD_30_PRIME_LOOKUP;
use crate::num::factorization::prime_sieve::{
    id_to_n, limbs_prime_sieve_size, limbs_prime_sieve_u64, n_to_bit,
};
use crate::num::factorization::traits::Primes;
use crate::num::logic::traits::TrailingZeros;
use alloc::vec::Vec;
use core::marker::PhantomData;

const NUM_SMALL_PRIMES: usize = 172;
// The same count as a `u64`, for comparing against the `u64` index of a prime iterator.
const NUM_SMALL_PRIMES_U64: u64 = NUM_SMALL_PRIMES as u64;

// Primes from 1031 (the next prime after 1021, the last entry of `SMALL_PRIMES`) up to
// `TABLE_LIMIT` are read from `MOD_30_PRIME_LOOKUP`, and only larger ones are sieved.
const TABLE_LIMIT: u64 = 1 << 20;
// 1048573 is the largest prime below `TABLE_LIMIT`.
const LARGEST_TABLE_PRIME: u64 = 1048573;

// The residues mod 30 that the eight bits of each byte of `MOD_30_PRIME_LOOKUP` stand for.
const WHEEL_RESIDUES: [u64; 8] = [1, 7, 11, 13, 17, 19, 23, 29];

// The table state for the first prime greater than n, for n < `LARGEST_TABLE_PRIME`: the byte of
// `MOD_30_PRIME_LOOKUP` to read, and that byte's bits for the numbers greater than n.
fn table_state_after(n: u64) -> (u64, u8) {
    let (byte, r) = n.div_mod(30);
    // the number of wheel residues at most r, which are the bits to clear
    let mut count = 0;
    while count < 8 && WHEEL_RESIDUES[count] <= r {
        count += 1;
    }
    if count == 8 {
        (byte + 1, MOD_30_PRIME_LOOKUP[byte as usize + 1])
    } else {
        (
            byte,
            MOD_30_PRIME_LOOKUP[byte as usize] & (u8::MAX << count),
        )
    }
}

// The sieve bit index to search from for the first prime past the table.
const SIEVE_START: u64 = n_to_bit(LARGEST_TABLE_PRIME) + 1;

// The next prime in `MOD_30_PRIME_LOOKUP` from the table state (`byte`, `bits`), with the table
// state after it, if that prime is less than `TABLE_LIMIT`.
#[inline]
fn table_next(mut byte: u64, mut bits: u8) -> Option<(u64, u8, u64)> {
    while bits == 0 {
        byte += 1;
        bits = *MOD_30_PRIME_LOOKUP.get(usize::exact_from(byte))?;
    }
    let p = byte * 30 + WHEEL_RESIDUES[bits.trailing_zeros() as usize];
    if p < TABLE_LIMIT {
        Some((byte, bits & (bits - 1), p))
    } else {
        None
    }
}

// Where a `PrimesLessThanIterator` is reading its primes from.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Source {
    // `SMALL_PRIMES`; the index is into the array.
    Small,
    // `MOD_30_PRIME_LOOKUP`; the index and `bits` are the table state.
    Table,
    // The sieve; the index is the sieve bit to search from.
    Sieve,
}

// This is flint_primes_small from ulong_extras/compute_primes.c, FLINT 3.1.2.
pub(crate) const SMALL_PRIMES: [u16; NUM_SMALL_PRIMES] = [
    2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97,
    101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167, 173, 179, 181, 191, 193,
    197, 199, 211, 223, 227, 229, 233, 239, 241, 251, 257, 263, 269, 271, 277, 281, 283, 293, 307,
    311, 313, 317, 331, 337, 347, 349, 353, 359, 367, 373, 379, 383, 389, 397, 401, 409, 419, 421,
    431, 433, 439, 443, 449, 457, 461, 463, 467, 479, 487, 491, 499, 503, 509, 521, 523, 541, 547,
    557, 563, 569, 571, 577, 587, 593, 599, 601, 607, 613, 617, 619, 631, 641, 643, 647, 653, 659,
    661, 673, 677, 683, 691, 701, 709, 719, 727, 733, 739, 743, 751, 757, 761, 769, 773, 787, 797,
    809, 811, 821, 823, 827, 829, 839, 853, 857, 859, 863, 877, 881, 883, 887, 907, 911, 919, 929,
    937, 941, 947, 953, 967, 971, 977, 983, 991, 997, 1009, 1013, 1019, 1021,
];

// This differs from the identically-named function in malachite-nz; this one returns None if there
// are no more false bits.
fn limbs_index_of_next_false_bit<T: PrimitiveUnsigned>(xs: &[T], start: u64) -> Option<u64> {
    let starting_index = usize::exact_from(start >> T::LOG_WIDTH);
    if starting_index >= xs.len() {
        return None;
    }
    if let Some(result) = xs[starting_index].index_of_next_false_bit(start & T::WIDTH_MASK)
        && result != T::WIDTH
    {
        return Some((u64::wrapping_from(starting_index) << T::LOG_WIDTH) + result);
    }
    if starting_index == xs.len() - 1 {
        return None;
    }
    let false_index = starting_index
        + 1
        + xs[starting_index + 1..]
            .iter()
            .take_while(|&&y| y == T::MAX)
            .count();
    if false_index == xs.len() {
        None
    } else {
        Some(
            (u64::exact_from(false_index) << T::LOG_WIDTH)
                + TrailingZeros::trailing_zeros(!xs[false_index]),
        )
    }
}

/// An iterator over that generates all primes less than a given value.
///
/// This `struct` is created by [`Primes::primes_less_than`] and
/// [`Primes::primes_less_than_or_equal_to`]; see their documentation for more.
#[derive(Clone, Debug)]
pub struct PrimesLessThanIterator<T: PrimitiveUnsigned> {
    source: Source,
    i: u64,
    bits: u8,
    limit: T,
    sieve: Vec<u64>,
    phantom: PhantomData<*const T>,
}

impl<T: PrimitiveUnsigned> PrimesLessThanIterator<T> {
    fn new(n: T) -> Self {
        let limit = n;
        let n: u64 = n.saturating_into();
        let mut sieve;
        if n < TABLE_LIMIT {
            sieve = Vec::with_capacity(0);
        } else {
            sieve = alloc::vec![0; limbs_prime_sieve_size::<u64>(n)];
            limbs_prime_sieve_u64(&mut sieve, n);
        }
        Self {
            source: Source::Small,
            i: 0,
            bits: 0,
            limit,
            sieve,
            phantom: PhantomData,
        }
    }

    /// Moves the iterator to just after a given value, returning whether the iterator will return
    /// any more values after that point. If `false` is returned, calling `next` will return `None`;
    /// if `true` is returned, calling `next` will return smallest prime greater than $n$.
    ///
    /// # Worst-case complexity (amortized)
    /// $T(n) = O(n\log \log n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `n`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::factorization::traits::Primes;
    ///
    /// let mut primes = u32::primes_less_than(&10_000);
    /// assert_eq!(primes.jump_after(1000), true);
    /// assert_eq!(primes.next(), Some(1009));
    ///
    /// assert_eq!(primes.jump_after(10_000), false);
    /// assert_eq!(primes.next(), None);
    /// ```
    pub fn jump_after(&mut self, n: T) -> bool {
        let n: u64 = if let Ok(n) = n.try_into() {
            n
        } else {
            return false;
        };
        // 1021 is the greatest prime smaller than 2^10.
        if n < 1021 {
            self.source = Source::Small;
            self.i = u64::wrapping_from(match SMALL_PRIMES.binary_search(&u16::wrapping_from(n)) {
                Ok(i) => i + 1,
                Err(i) => i,
            });
        } else if n < LARGEST_TABLE_PRIME {
            self.source = Source::Table;
            (self.i, self.bits) = table_state_after(n);
        } else {
            self.source = Source::Sieve;
            self.i = n_to_bit(n) + 1;
        }
        self.peek().is_some()
    }

    // The next prime, without advancing.
    fn peek(&self) -> Option<T> {
        let p = match self.source {
            Source::Small => u64::from(SMALL_PRIMES[self.i as usize]),
            Source::Table => {
                if let Some((_, _, p)) = table_next(self.i, self.bits) {
                    p
                } else {
                    id_to_n(limbs_index_of_next_false_bit(&self.sieve, SIEVE_START)? + 1)
                }
            }
            Source::Sieve => id_to_n(limbs_index_of_next_false_bit(&self.sieve, self.i)? + 1),
        };
        T::try_from(p).ok().filter(|&p| p <= self.limit)
    }

    // `next` for primes past `SMALL_PRIMES`.
    #[inline]
    fn next_large(&mut self) -> Option<T> {
        if self.source == Source::Table {
            if let Some((byte, bits, p)) = table_next(self.i, self.bits) {
                let p = T::try_from(p).ok()?;
                if p > self.limit {
                    return None;
                }
                self.i = byte;
                self.bits = bits;
                return Some(p);
            }
            self.source = Source::Sieve;
            self.i = SIEVE_START;
        }
        self.i = limbs_index_of_next_false_bit(&self.sieve, self.i)? + 1;
        let p = T::exact_from(id_to_n(self.i));
        if p > self.limit { None } else { Some(p) }
    }
}

impl<T: PrimitiveUnsigned> Iterator for PrimesLessThanIterator<T> {
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<T> {
        if self.source != Source::Small {
            return self.next_large();
        }
        let p = T::try_from(SMALL_PRIMES[self.i as usize]).ok()?;
        if p > self.limit {
            return None;
        }
        self.i += 1;
        if self.i == NUM_SMALL_PRIMES_U64 {
            self.source = Source::Table;
            // continue from 1031, the first prime past `SMALL_PRIMES`
            (self.i, self.bits) = table_state_after(1021);
        }
        Some(p)
    }
}

/// An iterator over that generates all primes.
///
/// This `struct` is created by [`Primes::primes`]; see its documentation for more.
#[derive(Clone, Debug)]
pub struct PrimesIterator<T: PrimitiveUnsigned> {
    limit: T,
    xs: PrimesLessThanIterator<T>,
}

impl<T: PrimitiveUnsigned> PrimesIterator<T> {
    fn new() -> Self {
        let limit = T::saturating_from(const { TABLE_LIMIT - 1 });
        Self {
            limit,
            xs: PrimesLessThanIterator::new(limit),
        }
    }

    /// Moves the iterator to just after a given value, returning whether the iterator will return
    /// any more values after that point. If `false` is returned, calling `next` will return `None`
    /// (which only happens if $n$ is very close to the maximum value of `T`); if `true` is
    /// returned, calling `next` will return smallest prime greater than $n$.
    ///
    /// # Worst-case complexity (amortized)
    /// $T(n) = O(n\log \log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `n`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::factorization::traits::Primes;
    ///
    /// let mut primes = u16::primes();
    /// assert_eq!(primes.jump_after(1000), true);
    /// assert_eq!(primes.next(), Some(1009));
    ///
    /// assert_eq!(primes.jump_after(u16::MAX), false);
    /// assert_eq!(primes.next(), None);
    /// ```
    pub fn jump_after(&mut self, n: T) -> bool {
        loop {
            if self.xs.jump_after(n) {
                return true;
            } else if self.limit == T::MAX {
                return false;
            }
            self.limit.saturating_mul_assign(T::TWO);
            while self.limit != T::MAX && self.limit <= n {
                self.limit.saturating_mul_assign(T::TWO);
            }
            let i = self.xs.i;
            self.xs = T::primes_less_than_or_equal_to(&self.limit);
            self.xs.i = i;
        }
    }
}

impl<T: PrimitiveUnsigned> Iterator for PrimesIterator<T> {
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<T> {
        loop {
            let p = self.xs.next();
            if p.is_some() || !self.extend() {
                return p;
            }
        }
    }
}

impl<T: PrimitiveUnsigned> PrimesIterator<T> {
    // Doubles the limit and re-sieves, continuing after the last prime generated. Returns `false`
    // if the limit is already `T::MAX`.
    #[cold]
    #[inline(never)]
    fn extend(&mut self) -> bool {
        if self.limit == T::MAX {
            return false;
        }
        self.limit.saturating_mul_assign(T::TWO);
        let i = if self.xs.source == Source::Sieve {
            self.xs.i
        } else {
            SIEVE_START
        };
        self.xs = T::primes_less_than_or_equal_to(&self.limit);
        self.xs.source = Source::Sieve;
        self.xs.i = i;
        true
    }
}

macro_rules! impl_primes {
    ($t:ident) => {
        impl Primes for $t {
            type I = PrimesIterator<$t>;
            type LI = PrimesLessThanIterator<$t>;

            /// Returns an iterator that generates all primes less than a given value.
            ///
            /// The iterator produced by `primes_less_than(n)` generates the same primes as the
            /// iterator produced by `primes().take_while(|&p| p < n)`, but the latter would be
            /// slower because it doesn't know in advance how large its prime sieve should be, and
            /// might have to create larger and larger prime sieves.
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
            fn primes_less_than(n: &$t) -> PrimesLessThanIterator<$t> {
                PrimesLessThanIterator::new(n.saturating_sub(1))
            }

            /// Returns an iterator that generates all primes less than or equal to a given value.
            ///
            /// The iterator produced by `primes_less_than_or_equal_to(n)` generates the same primes
            /// as the iterator produced by `primes().take_while(|&p| p <= n)`, but the latter would
            /// be slower because it doesn't know in advance how large its prime sieve should be,
            /// and might have to create larger and larger prime sieves.
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
            fn primes_less_than_or_equal_to(&n: &$t) -> PrimesLessThanIterator<$t> {
                PrimesLessThanIterator::new(n)
            }

            /// Returns all primes that fit into the specified type.
            ///
            /// The iterator produced by `primes(n)` generates the same primes as the iterator
            /// produced by `primes_less_than_or_equal_to(T::MAX)`. If you really need to generate
            /// _every_ prime, and `T` is `u32` or smaller, then you should use the latter, as it
            /// will allocate all the needed memory at once. If `T` is `u64` or larger, or if you
            /// probably don't need every prime, then `primes()` will be faster as it won't allocate
            /// too much memory right away.
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
            fn primes() -> PrimesIterator<$t> {
                PrimesIterator::new()
            }
        }
    };
}
apply_to_unsigneds!(impl_primes);

/// An iterator that generates `bool`s up to a certain limit, where the $n$th `bool` is `true` if
/// and only if $n$ is prime. See [`prime_indicator_sequence_less_than`] for more information.
#[derive(Clone, Debug)]
pub struct PrimeIndicatorSequenceLessThan {
    primes: PrimesLessThanIterator<u64>,
    limit: u64,
    i: u64,
    next_prime: u64,
}

impl Iterator for PrimeIndicatorSequenceLessThan {
    type Item = bool;

    fn next(&mut self) -> Option<bool> {
        if self.i >= self.limit {
            None
        } else if self.i == self.next_prime {
            self.i += 1;
            self.next_prime = self.primes.next().unwrap_or(0);
            Some(true)
        } else {
            self.i += 1;
            Some(false)
        }
    }
}

/// Returns an iterator that generates an sequence of `bool`s, where the $n$th `bool` is `true` if
/// and only if $n$ is prime. The first `bool` generated has index 1, and the last one has index
/// $\max(0,\ell-1)$, where $\ell$ is `limit`.
///
/// The output length is $max(0,\ell-1)$, where $\ell$ is `limit`.
///
/// # Worst-case complexity (amortized)
/// $T(i) = O(\log \log \log i)$
///
/// $M(i) = O(1)$
///
/// where $T$ is time, $M$ is additional memory, and $i$ is the iteration index.
///
/// # Examples
/// ```
/// use malachite_base::num::factorization::primes::prime_indicator_sequence_less_than;
///
/// let s: String = prime_indicator_sequence_less_than(101)
///     .map(|b| if b { '1' } else { '0' })
///     .collect();
/// assert_eq!(
///     s,
///     "01101010001010001010001000001010000010001010001000001000001010000010001010000010001000001\
///     00000001000"
/// )
/// ```
pub fn prime_indicator_sequence_less_than(limit: u64) -> PrimeIndicatorSequenceLessThan {
    let mut primes = u64::primes_less_than(&limit);
    primes.next(); // skip 2
    PrimeIndicatorSequenceLessThan {
        primes,
        limit,
        i: 1,
        next_prime: 2,
    }
}

/// Returns an iterator that generates an sequence of `bool`s, where the $n$th `bool` is `true` if
/// and only if $n$ is prime. The first `bool` generated has index 1, and the last one has index
/// `limit`.
///
/// The output length is `limit`.
///
/// # Worst-case complexity (amortized)
/// $T(i) = O(\log \log \log i)$
///
/// $M(i) = O(1)$
///
/// where $T$ is time, $M$ is additional memory, and $i$ is the iteration index.
///
/// # Examples
/// ```
/// use malachite_base::num::factorization::primes::prime_indicator_sequence_less_than_or_equal_to;
///
/// let s: String = prime_indicator_sequence_less_than_or_equal_to(100)
///     .map(|b| if b { '1' } else { '0' })
///     .collect();
/// assert_eq!(
///     s,
///     "01101010001010001010001000001010000010001010001000001000001010000010001010000010001000001\
///     00000001000"
/// )
/// ```
#[inline]
pub fn prime_indicator_sequence_less_than_or_equal_to(
    limit: u64,
) -> PrimeIndicatorSequenceLessThan {
    prime_indicator_sequence_less_than(limit.checked_add(1).unwrap())
}

/// An iterator that generates `bool`s, where the $n$th `bool` is `true` if and only if $n$ is
/// prime. See [`prime_indicator_sequence`] for more information.
#[derive(Clone, Debug)]
pub struct PrimeIndicatorSequence {
    primes: PrimesIterator<u64>,
    i: u64,
    next_prime: u64,
}

impl Iterator for PrimeIndicatorSequence {
    type Item = bool;

    fn next(&mut self) -> Option<bool> {
        Some(if self.i == self.next_prime {
            self.i += 1;
            self.next_prime = self.primes.next().unwrap();
            true
        } else {
            self.i += 1;
            false
        })
    }
}

/// Returns an iterator that generates an infinite sequence of `bool`s, where the $n$th `bool` is
/// `true` if and only if $n$ is prime. The first `bool` generated has index 1.
///
/// The output length is infinite.
///
/// # Worst-case complexity (amortized)
/// $T(i) = O(\log \log \log i)$
///
/// $M(i) = O(1)$
///
/// where $T$ is time, $M$ is additional memory, and $i$ is the iteration index.
///
/// # Examples
/// ```
/// use malachite_base::num::factorization::primes::prime_indicator_sequence;
///
/// let s: String = prime_indicator_sequence()
///     .take(100)
///     .map(|b| if b { '1' } else { '0' })
///     .collect();
/// assert_eq!(
///     s,
///     "01101010001010001010001000001010000010001010001000001000001010000010001010000010001000001\
///     00000001000"
/// )
/// ```
pub fn prime_indicator_sequence() -> PrimeIndicatorSequence {
    let mut primes = u64::primes();
    primes.next(); // skip 2
    PrimeIndicatorSequence {
        primes,
        i: 1,
        next_prime: 2,
    }
}
