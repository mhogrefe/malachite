// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the GNU MPFR Library.
//
//      Copyright 1999-2024 Free Software Foundation, Inc.
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
use malachite_base::num::arithmetic::traits::{SqrtAssign, Square};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_nz::natural::arithmetic::float_extras::float_can_round;
use malachite_nz::platform::Limb;

impl Float {
    /// Returns an approximation to $\pi$, with the given precision and rounded using the given
    /// [`RoundingMode`]. An [`Ordering`] is also returned, indicating whether the rounded value is
    /// less than or greater than the exact value of the constant. (Since the constant is
    /// irrational, the rounded value is never equal to the exact value.)
    ///
    /// $$
    /// x = \pi+\varepsilon.
    /// $$
    /// - If $m$ is not `Nearest`, then $|\varepsilon| < 2^{-p+2}$.
    /// - If $m$ is `Nearest`, then $|\varepsilon| < 2^{-p+1}$.
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
    /// let (pi, o) = Float::pi_prec_round(100, Floor);
    /// assert_eq!(pi.to_string(), "3.141592653589793238462643383279");
    /// assert_eq!(o, Less);
    ///
    /// let (pi, o) = Float::pi_prec_round(100, Ceiling);
    /// assert_eq!(pi.to_string(), "3.141592653589793238462643383282");
    /// assert_eq!(o, Greater);
    /// ```
    ///
    // This is mpfr_const_pi_internal from const_pi.c, MPFR 4.2.0.
    #[inline]
    pub fn pi_prec_round(prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        let px = prec;
        // we need 9 * 2 ^ kmax - 4 >= px + 2 * kmax + 8
        let mut kmax = 2;
        while ((px + 2 * kmax + 12) / 9) >> kmax != 0 {
            kmax += 1;
        }
        // guarantees no recomputation for px <= 10000
        let mut p = px + 3 * kmax + 14;
        let mut increment = Limb::WIDTH;
        let mut big_a;
        loop {
            let mut a = Float::one_prec(p); // a = 1
            big_a = a.clone(); // A = a^2 = 1
            let mut big_b = Float::one_half_prec(p); // B = b^2 = 1/2
            let mut big_d = Float::one_prec(p) >> 2u32; // D = 1/4
            let mut k = 0;
            loop {
                let mut s = &big_a + &big_b;
                s >>= 2;
                big_b.sqrt_assign();
                a += big_b;
                a >>= 1;
                big_a = (&a).square();
                big_b = &big_a - s;
                big_b <<= 1;
                let mut s = &big_a - &big_b;
                assert!(s < 1u32);
                let cancel = if s != 0 {
                    i64::from(-s.get_exponent().unwrap())
                } else {
                    i64::exact_from(p)
                };
                s <<= k;
                big_d -= s;
                // stop when |A_k - B_k| <= 2 ^ (k - p) i.e. cancel >= p - k
                if cancel >= i64::exact_from(p) - i64::exact_from(k) {
                    break;
                }
                k += 1;
            }
            big_a = big_b / big_d;
            if float_can_round(big_a.significand_ref().unwrap(), p - 2 * k - 8, px, rm) {
                break;
            }
            p += kmax;
            p += increment;
            increment = p >> 1;
        }
        Float::from_float_prec_round(big_a, prec, rm)
    }

    /// Returns an approximation to $\pi$, with the given precision and rounded to the nearest
    /// [`Float`] of that precision. An [`Ordering`] is also returned, indicating whether the
    /// rounded value is less than or greater than the exact value of the constant. (Since the
    /// constant is irrational, the rounded value is never equal to the exact value.)
    ///
    /// $$
    /// x = \pi+\varepsilon.
    /// $$
    /// - $|\varepsilon| < 2^{-p+1}$.
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
    /// let (pi, o) = Float::pi_prec(1);
    /// assert_eq!(pi.to_string(), "4.0");
    /// assert_eq!(o, Greater);
    ///
    /// let (pi, o) = Float::pi_prec(10);
    /// assert_eq!(pi.to_string(), "3.141");
    /// assert_eq!(o, Less);
    ///
    /// let (pi, o) = Float::pi_prec(100);
    /// assert_eq!(pi.to_string(), "3.141592653589793238462643383279");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn pi_prec(prec: u64) -> (Self, Ordering) {
        Self::pi_prec_round(prec, Nearest)
    }
}
