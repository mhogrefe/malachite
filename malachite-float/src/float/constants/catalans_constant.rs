// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the GNU MPFR Library.
//
//      Copyright 2005-2025 Free Software Foundation, Inc.
//
//      Contributed by the Pascaline and Caramba projects, INRIA.
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Float;
use core::cmp::Ordering;
use malachite_base::num::arithmetic::traits::{AddMul, CeilingLogBase2, Square};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::{One, Two};
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_nz::natural::Natural;
use malachite_nz::natural::arithmetic::float::round::float_can_round;
use malachite_nz::platform::Limb;

// Returns (T, P, Q) such that T/Q = sum(k!^2/(2k)!/(2k+1)^2, k=n1..n2-1).
//
// This is S from const_catalan.c, MPFR 4.2.2.
fn s(n1: u64, n2: u64) -> (Natural, Natural, Natural) {
    if n2 == n1 + 1 {
        if n1 == 0 {
            (Natural::ONE, Natural::ONE, Natural::ONE)
        } else {
            let p = Natural::from((n1 << 1) - 1) * Natural::from(n1);
            let q = Natural::from((n1 << 1) + 1).square() << 1u64;
            (p.clone(), p, q)
        }
    } else {
        let m = (n1 + n2) >> 1;
        let (t, p, q) = s(n1, m);
        let (t2, p2, q2) = s(m, n2);
        ((t * &q2).add_mul(t2, &p), p * p2, q * q2)
    }
}

impl Float {
    /// Returns an approximation of Catalan's constant, $G=\sum_{k=0}^\infty
    /// \frac{(-1)^k}{(2k+1)^2}$, with the given precision and rounded using the given
    /// [`RoundingMode`]. An [`Ordering`] is also returned, indicating whether the rounded value is
    /// less than or greater than the exact value of the constant. (The rounded value is never equal
    /// to the exact value of the constant. Catalan's constant has not been proven irrational, but
    /// its binary expansion is known to be aperiodic far beyond any precision reachable by this
    /// function.)
    ///
    /// $$
    /// x = G+\varepsilon.
    /// $$
    /// - If $m$ is not `Nearest`, then $|\varepsilon| < 2^{-p+1}$.
    /// - If $m$ is `Nearest`, then $|\varepsilon| < 2^{-p}$.
    ///
    /// The output has precision `prec`.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
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
    /// let (g, o) = Float::catalans_constant_prec_round(100, Floor);
    /// assert_eq!(g.to_string(), "0.91596559417721901505460351493173");
    /// assert_eq!(o, Less);
    ///
    /// let (g, o) = Float::catalans_constant_prec_round(100, Ceiling);
    /// assert_eq!(g.to_string(), "0.91596559417721901505460351493252");
    /// assert_eq!(o, Greater);
    /// ```
    ///
    // Catalan's constant is computed using formula (31) of Victor Adamchik's page "33
    // representations for Catalan's constant":
    //
    // G = Pi/8*log(2+sqrt(3)) + 3/8*sum(k!^2/(2k)!/(2k+1)^2, k=0..infinity)
    //
    // This is mpfr_const_catalan_internal from const_catalan.c, MPFR 4.2.2.
    pub fn catalans_constant_prec_round(prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        const THREE: Natural = Natural::const_from(3);
        let mut working_prec = prec + prec.ceiling_log_base_2() + 7;
        let mut increment = Limb::WIDTH;
        loop {
            // x = (pi * log(2 + sqrt(3)) + 3T/Q) / 8, where T/Q is the series computed by binary
            // splitting. The log's argument and the series numerator are rounded up and the series
            // denominator is rounded down; every operand has the working precision, so the
            // arithmetic operators round to nearest at that precision.
            let log_arg = Self::sqrt_unsigned_prec_round(3, working_prec, Up)
                .0
                .add_round(Self::TWO, Up)
                .0;
            let (t, _, q) = s(0, (working_prec - 1) >> 1);
            let x = (Self::pi_prec_round(working_prec, Up).0 * log_arg.ln_round(Up).0
                + Self::from_natural_prec_round(t * THREE, working_prec, Up).0
                    / Self::from_natural_prec_round(q, working_prec, Down).0)
                >> 3u64;
            if float_can_round(x.significand_ref().unwrap(), working_prec - 5, prec, rm) {
                return Self::from_float_prec_round(x, prec, rm);
            }
            working_prec += increment;
            increment = working_prec >> 1;
        }
    }

    /// Returns an approximation of Catalan's constant, $G=\sum_{k=0}^\infty
    /// \frac{(-1)^k}{(2k+1)^2}$, with the given precision and rounded to the nearest [`Float`] of
    /// that precision. An [`Ordering`] is also returned, indicating whether the rounded value is
    /// less than or greater than the exact value of the constant. (The rounded value is never equal
    /// to the exact value of the constant. Catalan's constant has not been proven irrational, but
    /// its binary expansion is known to be aperiodic far beyond any precision reachable by this
    /// function.)
    ///
    /// $$
    /// x = G+\varepsilon.
    /// $$
    /// - $|\varepsilon| < 2^{-p}$.
    ///
    /// The output has precision `prec`.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
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
    /// let (g, o) = Float::catalans_constant_prec(1);
    /// assert_eq!(g.to_string(), "1.0");
    /// assert_eq!(o, Greater);
    ///
    /// let (g, o) = Float::catalans_constant_prec(10);
    /// assert_eq!(g.to_string(), "0.91602");
    /// assert_eq!(o, Greater);
    ///
    /// let (g, o) = Float::catalans_constant_prec(100);
    /// assert_eq!(g.to_string(), "0.91596559417721901505460351493252");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn catalans_constant_prec(prec: u64) -> (Self, Ordering) {
        Self::catalans_constant_prec_round(prec, Nearest)
    }
}
