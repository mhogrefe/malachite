// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the GNU MPFR Library.
//
//      Copyright 1999, 2001-2024 Free Software Foundation, Inc.
//
//      Contributed by the AriC and Caramba projects, INRIA.
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Float;
use core::cmp::Ordering;
use malachite_base::rounding_modes::RoundingMode::{self, *};

impl Float {
    /// Returns an approximation of $2/\sqrt{pi}$, with the given precision and rounded using the
    /// given [`RoundingMode`]. An [`Ordering`] is also returned, indicating whether the rounded
    /// value is less than or greater than the exact value of the constant. (Since the constant is
    /// irrational, the rounded value is never equal to the exact value.)
    ///
    /// $$
    /// x = 2/\sqrt{pi}+\varepsilon.
    /// $$
    /// - If $m$ is not `Nearest`, then $|\varepsilon| < 2^{-p+1}$.
    /// - If $m$ is `Nearest`, then $|\varepsilon| < 2^{-p}$.
    ///
    /// The constant is irrational and transcendental.
    ///
    /// The output has precision `prec`.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n (\log n)^2)$
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
    /// let (two_over_sqrt_pi, o) = Float::two_over_sqrt_pi_prec_round(100, Floor);
    /// assert_eq!(
    ///     two_over_sqrt_pi.to_string(),
    ///     "1.12837916709551257389615890312"
    /// );
    /// assert_eq!(o, Less);
    ///
    /// let (two_over_sqrt_pi, o) = Float::two_over_sqrt_pi_prec_round(100, Ceiling);
    /// assert_eq!(
    ///     two_over_sqrt_pi.to_string(),
    ///     "1.128379167095512573896158903122"
    /// );
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn two_over_sqrt_pi_prec_round(prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        let (pi, o) = Self::one_over_sqrt_pi_prec_round(prec, rm);
        (pi << 1u32, o)
    }

    /// Returns an approximation of $2/\sqrt{pi}$, with the given precision and rounded to the
    /// nearest [`Float`] of that precision. An [`Ordering`] is also returned, indicating whether
    /// the rounded value is less than or greater than the exact value of the constant. (Since the
    /// constant is irrational, the rounded value is never equal to the exact value.)
    ///
    /// $$
    /// x = 2/\sqrt{pi}+\varepsilon.
    /// $$
    /// - $|\varepsilon| < 2^{-p}$.
    ///
    /// The constant is irrational and transcendental.
    ///
    /// The output has precision `prec`.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n (\log n)^2)$
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
    /// let (two_over_sqrt_pi, o) = Float::two_over_sqrt_pi_prec(1);
    /// assert_eq!(two_over_sqrt_pi.to_string(), "1.0");
    /// assert_eq!(o, Less);
    ///
    /// let (two_over_sqrt_pi, o) = Float::two_over_sqrt_pi_prec(10);
    /// assert_eq!(two_over_sqrt_pi.to_string(), "1.129");
    /// assert_eq!(o, Greater);
    ///
    /// let (two_over_sqrt_pi, o) = Float::two_over_sqrt_pi_prec(100);
    /// assert_eq!(
    ///     two_over_sqrt_pi.to_string(),
    ///     "1.128379167095512573896158903122"
    /// );
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn two_over_sqrt_pi_prec(prec: u64) -> (Self, Ordering) {
        Self::two_over_sqrt_pi_prec_round(prec, Nearest)
    }
}
