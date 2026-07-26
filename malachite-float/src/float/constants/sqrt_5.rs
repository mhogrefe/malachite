// Copyright © 2026 Mikhail Hogrefe
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
    /// Returns an approximation of the square root of 5, with the given precision and rounded using
    /// the given [`RoundingMode`]. An [`Ordering`] is also returned, indicating whether the rounded
    /// value is less than or greater than the exact value of the constant. (Since the constant is
    /// irrational, the rounded value is never equal to the exact value.)
    ///
    /// $$
    /// x = \sqrt{5}+\varepsilon.
    /// $$
    /// - If $m$ is not `Nearest`, then $|\varepsilon| < 2^{-p+1}$.
    /// - If $m$ is `Nearest`, then $|\varepsilon| < 2^{-p}$.
    ///
    /// The constant is irrational and algebraic.
    ///
    /// The output has precision `prec`.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
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
    /// let (sqrt_5, o) = Float::sqrt_5_prec_round(100, Floor);
    /// assert_eq!(sqrt_5.to_string(), "2.2360679774997896964091736687284");
    /// assert_eq!(o, Less);
    ///
    /// let (sqrt_5, o) = Float::sqrt_5_prec_round(100, Ceiling);
    /// assert_eq!(sqrt_5.to_string(), "2.2360679774997896964091736687316");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn sqrt_5_prec_round(prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        Self::sqrt_prec_round(const { Self::const_from_unsigned(5) }, prec, rm)
    }

    /// Returns an approximation of the square root of 5, with the given precision and rounded to
    /// the nearest [`Float`] of that precision. An [`Ordering`] is also returned, indicating
    /// whether the rounded value is less than or greater than the exact value of the constant.
    /// (Since the constant is irrational, the rounded value is never equal to the exact value.)
    ///
    /// $$
    /// x = \sqrt{5}+\varepsilon.
    /// $$
    /// - $|\varepsilon| < 2^{-p}$.
    ///
    /// The constant is irrational and algebraic.
    ///
    /// The output has precision `prec`.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
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
    /// let (sqrt_5, o) = Float::sqrt_5_prec(1);
    /// assert_eq!(sqrt_5.to_string(), "2.0");
    /// assert_eq!(o, Less);
    ///
    /// let (sqrt_5, o) = Float::sqrt_5_prec(10);
    /// assert_eq!(sqrt_5.to_string(), "2.2344");
    /// assert_eq!(o, Less);
    ///
    /// let (sqrt_5, o) = Float::sqrt_5_prec(100);
    /// assert_eq!(sqrt_5.to_string(), "2.2360679774997896964091736687316");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn sqrt_5_prec(prec: u64) -> (Self, Ordering) {
        Self::sqrt_5_prec_round(prec, Nearest)
    }
}
