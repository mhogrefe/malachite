// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Float;
use core::cmp::Ordering;
use malachite_base::num::factorization::primes::prime_indicator_sequence_less_than_or_equal_to;
use malachite_base::rounding_modes::RoundingMode::{self, *};

impl Float {
    /// Returns an approximation to the prime constant, with the given precision and rounded using
    /// the given [`RoundingMode`]. An [`Ordering`] is also returned, indicating whether the rounded
    /// value is less than or greater than the exact value of the constant. (Since the constant is
    /// irrational, the rounded value is never equal to the exact value.)
    ///
    /// The prime constant is the real number whose $n$th bit is prime if and only if $n$ is prime.
    /// That is,
    /// $$
    /// P = \sum_{p\ text{prime}\}2^{-p}.
    /// $$
    ///
    /// The constant is irrational.
    ///
    /// The output has precision `prec`.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Panics
    /// Panics if `prec` is zero or if `rm` is `Exact`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (pc, o) = Float::prime_constant_prec_round(100, Floor);
    /// assert_eq!(pc.to_string(), "0.4146825098511116602481096221542");
    /// assert_eq!(o, Less);
    ///
    /// let (pc, o) = Float::prime_constant_prec_round(100, Ceiling);
    /// assert_eq!(pc.to_string(), "0.4146825098511116602481096221546");
    /// assert_eq!(o, Greater);
    /// ```
    pub fn prime_constant_prec_round(prec: u64, rm: RoundingMode) -> (Float, Ordering) {
        // Strictly speaking, this call violates the preconditions for
        // `non_dyadic_from_bits_prec_round`, because the iterator passed in is finite. But since we
        // know exactly how many bits `non_dyadic_from_bits_prec_round` will read, we can get away
        // with this.
        Float::non_dyadic_from_bits_prec_round(
            prime_indicator_sequence_less_than_or_equal_to(if rm == Nearest {
                prec + 2
            } else {
                prec + 1
            }),
            prec,
            rm,
        )
    }

    /// Returns an approximation to the prime constant, with the given precision and rounded to the
    /// nearest [`Float`] of that precision. An [`Ordering`] is also returned, indicating whether
    /// the rounded value is less than or greater than the exact value of the constant. (Since the
    /// constant is irrational, the rounded value is never equal to the exact value.)
    ///
    /// The prime constant is the real number whose $n$th bit is prime if and only if $n$ is prime.
    /// That is,
    /// $$
    /// P = \sum_{p\ text{prime}\}2^{-p}.
    /// $$
    ///
    /// The constant is irrational.
    ///
    /// The output has precision `prec`.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (pc, o) = Float::prime_constant_prec(1);
    /// assert_eq!(pc.to_string(), "0.5");
    /// assert_eq!(o, Greater);
    ///
    /// let (pc, o) = Float::prime_constant_prec(10);
    /// assert_eq!(pc.to_string(), "0.4146");
    /// assert_eq!(o, Less);
    ///
    /// let (pc, o) = Float::prime_constant_prec(100);
    /// assert_eq!(pc.to_string(), "0.4146825098511116602481096221542");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn prime_constant_prec(prec: u64) -> (Float, Ordering) {
        Float::prime_constant_prec_round(prec, Nearest)
    }
}
