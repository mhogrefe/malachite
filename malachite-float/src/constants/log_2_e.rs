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
use malachite_base::num::arithmetic::traits::Reciprocal;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_nz::natural::arithmetic::float_extras::float_can_round;
use malachite_nz::platform::Limb;

impl Float {
    /// Returns an approximation to the base-2 logarithm of $e$, with the given precision and
    /// rounded using the given [`RoundingMode`]. An [`Ordering`] is also returned, indicating
    /// whether the rounded value is less than or greater than the exact value of the constant.
    /// (Since the constant is irrational, the rounded value is never equal to the exact value.)
    ///
    /// $$
    /// x = \log_2 e+\varepsilon.
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
    /// let (log_2_e, o) = Float::log_2_e_prec_round(100, Floor);
    /// assert_eq!(log_2_e.to_string(), "1.442695040888963407359924681001");
    /// assert_eq!(o, Less);
    ///
    /// let (log_2_e, o) = Float::log_2_e_prec_round(100, Ceiling);
    /// assert_eq!(log_2_e.to_string(), "1.442695040888963407359924681003");
    /// assert_eq!(o, Greater);
    /// ```
    pub fn log_2_e_prec_round(prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        let mut working_prec = prec + 10;
        let mut increment = Limb::WIDTH;
        loop {
            let log_2_e = Self::ln_2_prec_round(working_prec, Floor).0.reciprocal();
            // See algorithms.tex. Since we rounded down when computing ln_2, the absolute error of
            // the inverse is bounded by (c_w + 2c_uk_u)ulp(log_e(2)) <= 4ulp(log_e(2)).
            if float_can_round(
                log_2_e.significand_ref().unwrap(),
                working_prec - 2,
                prec,
                rm,
            ) {
                return Self::from_float_prec_round(log_2_e, prec, rm);
            }
            working_prec += increment;
            increment = working_prec >> 1;
        }
    }

    /// Returns an approximation to the base-2 logarithm of $e$, with the given precision and
    /// rounded to the nearest [`Float`] of that precision. An [`Ordering`] is also returned,
    /// indicating whether the rounded value is less than or greater than the exact value of the
    /// constant. (Since the constant is irrational, the rounded value is never equal to the exact
    /// value.)
    ///
    /// $$
    /// x = \log_2 e+\varepsilon.
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
    /// let (log_2_e, o) = Float::log_2_e_prec(1);
    /// assert_eq!(log_2_e.to_string(), "1.0");
    /// assert_eq!(o, Less);
    ///
    /// let (log_2_e, o) = Float::log_2_e_prec(10);
    /// assert_eq!(log_2_e.to_string(), "1.443");
    /// assert_eq!(o, Greater);
    ///
    /// let (log_2_e, o) = Float::log_2_e_prec(100);
    /// assert_eq!(log_2_e.to_string(), "1.442695040888963407359924681003");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn log_2_e_prec(prec: u64) -> (Self, Ordering) {
        Self::log_2_e_prec_round(prec, Nearest)
    }
}
