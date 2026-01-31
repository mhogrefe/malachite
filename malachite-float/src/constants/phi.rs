// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Float;
use core::cmp::Ordering::{self, *};
use malachite_base::num::basic::traits::{One, Two};
use malachite_base::rounding_modes::RoundingMode::{self, *};

impl Float {
    /// Returns an approximation of the golden ratio, with the given precision and rounded using the
    /// given [`RoundingMode`]. An [`Ordering`] is also returned, indicating whether the rounded
    /// value is less than or greater than the exact value of the constant. (Since the constant is
    /// irrational, the rounded value is never equal to the exact value.)
    ///
    /// $$
    /// \varphi = \frac{1+\sqrt{2}}{2}+\varepsilon.
    /// $$
    /// - If $m$ is not `Nearest`, then $|\varepsilon| < 2^{-p}$.
    /// - If $m$ is `Nearest`, then $|\varepsilon| < 2^{-p-1}$.
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
    /// let (phi, o) = Float::phi_prec_round(100, Floor);
    /// assert_eq!(phi.to_string(), "1.618033988749894848204586834364");
    /// assert_eq!(o, Less);
    ///
    /// let (phi, o) = Float::phi_prec_round(100, Ceiling);
    /// assert_eq!(phi.to_string(), "1.618033988749894848204586834366");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn phi_prec_round(prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        if prec == 1 {
            match rm {
                Floor | Down => (Self::ONE, Less),
                Ceiling | Up | Nearest => (Self::TWO, Greater),
                Exact => panic!("Inexact float square root"),
            }
        } else {
            let (sqrt_5, o) =
                Self::sqrt_prec_round(const { Self::const_from_unsigned(5) }, prec, rm);
            ((sqrt_5 + Self::ONE) >> 1u32, o)
        }
    }

    /// Returns an approximation of the golden ratio, with the given precision and rounded to the
    /// nearest [`Float`] of that precision. An [`Ordering`] is also returned, indicating whether
    /// the rounded value is less than or greater than the exact value of the constant. (Since the
    /// constant is irrational, the rounded value is never equal to the exact value.)
    ///
    /// $$
    /// \varphi = \frac{1+\sqrt{2}}{2}+\varepsilon.
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
    /// let (phi, o) = Float::phi_prec(1);
    /// assert_eq!(phi.to_string(), "2.0");
    /// assert_eq!(o, Greater);
    ///
    /// let (phi, o) = Float::phi_prec(10);
    /// assert_eq!(phi.to_string(), "1.617");
    /// assert_eq!(o, Less);
    ///
    /// let (phi, o) = Float::phi_prec(100);
    /// assert_eq!(phi.to_string(), "1.618033988749894848204586834366");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn phi_prec(prec: u64) -> (Self, Ordering) {
        Self::phi_prec_round(prec, Nearest)
    }
}
