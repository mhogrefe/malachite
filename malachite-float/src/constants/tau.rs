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
    /// Returns an approximation of $\tau=2\pi$, with the given precision and rounded using the
    /// given [`RoundingMode`]. An [`Ordering`] is also returned, indicating whether the rounded
    /// value is less than or greater than the exact value of the constant. (Since the constant is
    /// irrational, the rounded value is never equal to the exact value.)
    ///
    /// $$
    /// x = \tau=2\pi+\varepsilon.
    /// $$
    /// - If $m$ is not `Nearest`, then $|\varepsilon| < 2^{-p+3}$.
    /// - If $m$ is `Nearest`, then $|\varepsilon| < 2^{-p+2}$.
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
    /// let (tau, o) = Float::tau_prec_round(100, Floor);
    /// assert_eq!(tau.to_string(), "6.283185307179586476925286766559");
    /// assert_eq!(o, Less);
    ///
    /// let (tau, o) = Float::tau_prec_round(100, Ceiling);
    /// assert_eq!(tau.to_string(), "6.283185307179586476925286766565");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn tau_prec_round(prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        let (pi, o) = Self::pi_prec_round(prec, rm);
        (pi << 1u32, o)
    }

    /// Returns an approximation of $\tau=2\pi$, with the given precision and rounded to the nearest
    /// [`Float`] of that precision. An [`Ordering`] is also returned, indicating whether the
    /// rounded value is less than or greater than the exact value of the constant. (Since the
    /// constant is irrational, the rounded value is never equal to the exact value.)
    ///
    /// $$
    /// x = \tau=2\pi+\varepsilon.
    /// $$
    /// - $|\varepsilon| < 2^{-p+2}$.
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
    /// let (tau, o) = Float::tau_prec(1);
    /// assert_eq!(tau.to_string(), "8.0");
    /// assert_eq!(o, Greater);
    ///
    /// let (tau, o) = Float::tau_prec(10);
    /// assert_eq!(tau.to_string(), "6.28");
    /// assert_eq!(o, Less);
    ///
    /// let (tau, o) = Float::tau_prec(100);
    /// assert_eq!(tau.to_string(), "6.283185307179586476925286766559");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn tau_prec(prec: u64) -> (Self, Ordering) {
        Self::tau_prec_round(prec, Nearest)
    }
}
