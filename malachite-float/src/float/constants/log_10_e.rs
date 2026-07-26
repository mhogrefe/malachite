// Copyright © 2026 Mikhail Hogrefe
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
    /// Returns an approximation of the base-10 logarithm of $e$, with the given precision and
    /// rounded using the given [`RoundingMode`]. An [`Ordering`] is also returned, indicating
    /// whether the rounded value is less than or greater than the exact value of the constant.
    /// (Since the constant is irrational, the rounded value is never equal to the exact value.)
    ///
    /// $$
    /// x = \log_{10} e+\varepsilon.
    /// $$
    /// - If $m$ is not `Nearest`, then $|\varepsilon| < 2^{-p-1}$.
    /// - If $m$ is `Nearest`, then $|\varepsilon| < 2^{-p-2}$.
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
    /// let (l, o) = Float::log_10_e_prec_round(100, Floor);
    /// assert_eq!(l.to_string(), "0.43429448190325182765112891891628");
    /// assert_eq!(o, Less);
    ///
    /// let (l, o) = Float::log_10_e_prec_round(100, Ceiling);
    /// assert_eq!(l.to_string(), "0.43429448190325182765112891891667");
    /// assert_eq!(o, Greater);
    /// ```
    pub fn log_10_e_prec_round(prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        let mut working_prec = prec + 10;
        let mut increment = Limb::WIDTH;
        loop {
            let log_10_e = Self::ln_10_prec_round(working_prec, Floor).0.reciprocal();
            // As with log_2_e: since we rounded ln_10 down, the absolute error of the inverse is
            // bounded by (c_w + 2c_uk_u)ulp(log_e(10)) <= 4ulp(log_e(10)).
            if float_can_round(
                log_10_e.significand_ref().unwrap(),
                working_prec - 2,
                prec,
                rm,
            ) {
                return Self::from_float_prec_round(log_10_e, prec, rm);
            }
            working_prec += increment;
            increment = working_prec >> 1;
        }
    }

    /// Returns an approximation of the base-10 logarithm of $e$, with the given precision and
    /// rounded to the nearest [`Float`] of that precision. An [`Ordering`] is also returned,
    /// indicating whether the rounded value is less than or greater than the exact value of the
    /// constant. (Since the constant is irrational, the rounded value is never equal to the exact
    /// value.)
    ///
    /// $$
    /// x = \log_{10} e+\varepsilon.
    /// $$
    /// - $|\varepsilon| < 2^{-p-2}$.
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
    /// let (l, o) = Float::log_10_e_prec(1);
    /// assert_eq!(l.to_string(), "0.50");
    /// assert_eq!(o, Greater);
    ///
    /// let (l, o) = Float::log_10_e_prec(10);
    /// assert_eq!(l.to_string(), "0.43408");
    /// assert_eq!(o, Less);
    ///
    /// let (l, o) = Float::log_10_e_prec(100);
    /// assert_eq!(l.to_string(), "0.43429448190325182765112891891667");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn log_10_e_prec(prec: u64) -> (Self, Ordering) {
        Self::log_10_e_prec_round(prec, Nearest)
    }
}
