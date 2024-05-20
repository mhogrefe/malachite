// Copyright © 2024 Mikhail Hogrefe
//
// Uses code adopted from the GNU MP Library.
//
//      Copyright © 1994, 1996, 2001, 2002, 2009-2011 Free Software Foundation, Inc.
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::logic::count_ones::limbs_count_ones;
use crate::natural::InnerNatural::{Large, Small};
use crate::natural::Natural;
use crate::platform::Limb;
use core::cmp::Ordering::*;
use malachite_base::num::logic::traits::HammingDistance;

// Interpreting a slice of `Limb`s as the limbs of a `Natural` in ascending order, returns the
// Hamming distance between that `Natural` and a `Limb`. Both have infinitely many implicit leading
// zeros. `xs` cannot be empty.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(1)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `xs.len()`.
//
// # Panics
// Panics if `xs` is empty.
pub_test! {limbs_hamming_distance_limb(xs: &[Limb], y: Limb) -> u64 {
    xs[0].hamming_distance(y) + limbs_count_ones(&xs[1..])
}}

// Interpreting two equal-length slices of `Limb`s as the limbs of `Natural`s in ascending order,
// returns the Hamming distance between them. Both have infinitely many implicit leading zeros.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(1)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `xs.len()`.
//
// # Panics
// Panics if `xs` and `ys` have different lengths.
//
// This is equivalent to `mpz_hamdist` from `mpz/hamdist.c`, GMP 6.2.1, where both arguments are
// non-negative and have the same length.
pub_crate_test! {limbs_hamming_distance_same_length(xs: &[Limb], ys: &[Limb]) -> u64 {
    assert_eq!(xs.len(), ys.len());
    xs.iter()
        .zip(ys.iter())
        .map(|(x, &y)| x.hamming_distance(y))
        .sum()
}}

// Interpreting two slices of `Limb`s as the limbs of `Natural`s in ascending order, returns the
// Hamming distance between them. Both have infinitely many implicit leading zeros.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(1)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `max(xs.len(), ys.len())`.
//
// This is equivalent to `mpz_hamdist` from `mpz/hamdist.c`, GMP 6.2.1, where both arguments are
// non-negative.
pub_test! {limbs_hamming_distance(xs: &[Limb], ys: &[Limb]) -> u64 {
    let xs_len = xs.len();
    let ys_len = ys.len();
    match xs_len.cmp(&ys_len) {
        Equal => limbs_hamming_distance_same_length(xs, ys),
        Less => {
            limbs_hamming_distance_same_length(xs, &ys[..xs_len]) + limbs_count_ones(&ys[xs_len..])
        }
        Greater => {
            limbs_hamming_distance_same_length(&xs[..ys_len], ys) + limbs_count_ones(&xs[ys_len..])
        }
    }
}}

impl Natural {
    fn hamming_distance_limb(&self, other: Limb) -> u64 {
        match *self {
            Natural(Small(small)) => small.hamming_distance(other),
            Natural(Large(ref limbs)) => limbs_hamming_distance_limb(limbs, other),
        }
    }
}

impl<'a, 'b> HammingDistance<&'a Natural> for &'b Natural {
    /// Determines the Hamming distance between two [`Natural]`s.
    ///
    /// Both [`Natural`]s have infinitely many implicit leading zeros.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_base::num::logic::traits::HammingDistance;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     Natural::from(123u32).hamming_distance(&Natural::from(123u32)),
    ///     0
    /// );
    /// // 105 = 1101001b, 123 = 1111011
    /// assert_eq!(
    ///     Natural::from(105u32).hamming_distance(&Natural::from(123u32)),
    ///     2
    /// );
    /// let n = Natural::ONE << 100u32;
    /// assert_eq!(n.hamming_distance(&(&n - Natural::ONE)), 101);
    /// ```
    fn hamming_distance(self, other: &'a Natural) -> u64 {
        match (self, other) {
            (&Natural(Small(x)), _) => other.hamming_distance_limb(x),
            (_, &Natural(Small(y))) => self.hamming_distance_limb(y),
            (&Natural(Large(ref xs)), &Natural(Large(ref ys))) => limbs_hamming_distance(xs, ys),
        }
    }
}
