// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Float;
use core::cmp::Ordering;
use malachite_base::num::basic::traits::One;
use malachite_base::rounding_modes::RoundingMode::{self, *};

impl Float {
    /// Returns an approximation of $e$, Euler's number, with the given precision and rounded using
    /// the given [`RoundingMode`]. An [`Ordering`] is also returned, indicating whether the rounded
    /// value is less than or greater than the exact value of the constant. (Since the constant is
    /// irrational, the rounded value is never equal to the exact value.)
    ///
    /// $$
    /// x = e+\varepsilon.
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
    /// let (e, o) = Float::e_prec_round(100, Floor);
    /// assert_eq!(e.to_string(), "2.7182818284590452353602874713512");
    /// assert_eq!(o, Less);
    ///
    /// let (e, o) = Float::e_prec_round(100, Ceiling);
    /// assert_eq!(e.to_string(), "2.7182818284590452353602874713544");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn e_prec_round(prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        Self::exp_prec_round(Self::ONE, prec, rm)
    }

    /// Returns an approximation of $e$, Euler's number, with the given precision and rounded to the
    /// nearest [`Float`] of that precision. An [`Ordering`] is also returned, indicating whether
    /// the rounded value is less than or greater than the exact value of the constant. (Since the
    /// constant is irrational, the rounded value is never equal to the exact value.)
    ///
    /// $$
    /// x = e+\varepsilon.
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
    /// let (e, o) = Float::e_prec(1);
    /// assert_eq!(e.to_string(), "2.0");
    /// assert_eq!(o, Less);
    ///
    /// let (e, o) = Float::e_prec(10);
    /// assert_eq!(e.to_string(), "2.7188");
    /// assert_eq!(o, Greater);
    ///
    /// let (e, o) = Float::e_prec(100);
    /// assert_eq!(e.to_string(), "2.7182818284590452353602874713512");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn e_prec(prec: u64) -> (Self, Ordering) {
        Self::e_prec_round(prec, Nearest)
    }
}
