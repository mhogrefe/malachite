// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2021 Daniel Schultz
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use malachite_base::num::arithmetic::traits::{DivExact, Mod, ModInverse, UnsignedAbs};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;

impl Rational {
    /// Returns the two [`Rational`]s adjacent to this one in the Farey sequence of order
    /// `max_denominator`: the largest fraction below it and the smallest fraction above it, among
    /// those whose denominators are at most `max_denominator`.
    ///
    /// The Farey sequence of order $n$ is classically the ascending sequence of fractions in $[0,
    /// 1]$ whose denominators are at most $n$; this function uses its extension to all of
    /// $\mathbb{Q}$, so `self` need not lie in $[0, 1]$.
    ///
    /// $f(x, n) = (a/b, c/d)$, where $b, d \leq n$, $a/b < x < c/d$, and no fraction with
    /// denominator at most $n$ lies in $(a/b, x)$ or in $(x, c/d)$.
    ///
    /// Both neighbors are in lowest terms, and each is a best approximation of `self` from its
    /// side. The nearer of the two is the value returned by
    /// [`approximate`](crate::rational::arithmetic::traits::Approximate::approximate).
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// max_denominator.significant_bits())`: the modular inverse dominates, and the rest is a
    /// constant number of multiplications and divisions.
    ///
    /// # Panics
    /// Panics if `max_denominator` is less than the denominator of `self`.
    ///
    /// # Examples
    /// ```
    /// use malachite_nz::natural::Natural;
    /// use malachite_q::Rational;
    ///
    /// // In the Farey sequence of order 5, 1/2 sits between 2/5 and 3/5.
    /// let (l, r) = Rational::from_signeds(1, 2).farey_neighbors(&Natural::from(5u32));
    /// assert_eq!(l.to_string(), "2/5");
    /// assert_eq!(r.to_string(), "3/5");
    ///
    /// // The sequence extends beyond [0, 1], and to negative numbers.
    /// let (l, r) = Rational::from_signeds(-3, 4).farey_neighbors(&Natural::from(7u32));
    /// assert_eq!(l.to_string(), "-4/5");
    /// assert_eq!(r.to_string(), "-5/7");
    ///
    /// // The neighbors of an integer are one unit of the largest denominator away.
    /// let (l, r) = Rational::from(2).farey_neighbors(&Natural::from(3u32));
    /// assert_eq!(l.to_string(), "5/3");
    /// assert_eq!(r.to_string(), "7/3");
    /// ```
    ///
    /// This is fmpq_farey_neighbors from fmpq/farey_neighbors.c, FLINT 3.6.0.
    pub fn farey_neighbors(&self, max_denominator: &Natural) -> (Self, Self) {
        assert!(
            *max_denominator >= self.denominator,
            "max_denominator must be at least the denominator of self, but {max_denominator} < {}",
            self.denominator
        );
        let d = &self.denominator;
        let n = Integer::from_sign_and_abs_ref(self.sign, &self.numerator);
        // The left denominator is the largest b <= max_denominator with n * b = -1 mod d, reached
        // by stepping down from max_denominator into the right residue class. Modulo 1 every
        // residue is zero, and `mod_inverse` does not accept a zero input, so that case is taken
        // directly.
        let inverse = if *d == 1u32 {
            Natural::ZERO
        } else {
            Natural::exact_from((&n).mod_op(Integer::from(d)))
                .mod_inverse(d)
                .unwrap()
        };
        let l_den = max_denominator - (max_denominator - inverse) % d;
        // n * l_den - 1 is divisible by d, and the quotient is the left numerator.
        let l_num = (&n * Integer::from(&l_den) - Integer::ONE).div_exact(Integer::from(d));
        // The index of the Farey recurrence: the right neighbor is this many times self, minus the
        // left neighbor, in both components.
        let v = (max_denominator + &l_den) / d;
        let r_den = d * &v - &l_den;
        let r_num = n * Integer::from(v) - &l_num;
        // Adjacent fractions in a Farey sequence satisfy |ad - bc| = 1, so both neighbors are
        // already in lowest terms and need no reduction.
        (
            Self {
                sign: l_num >= 0,
                numerator: l_num.unsigned_abs(),
                denominator: l_den,
            },
            Self {
                sign: r_num >= 0,
                numerator: r_num.unsigned_abs(),
                denominator: r_den,
            },
        )
    }
}
