// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the GNU MPFR Library.
//
//      Copyright 2001-2025 Free Software Foundation, Inc.
//
//      Contributed by Fredrik Johansson.
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Float;
use core::cmp::Ordering;
use malachite_base::num::arithmetic::traits::{
    AddMul, CeilingLogBase2, DivRound, MulAddMul, Pow, Square,
};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_nz::natural::Natural;
use malachite_nz::natural::arithmetic::float::round::float_can_round;
use malachite_nz::platform::Limb;
use malachite_q::Rational;

// The six binary-splitting components. This is mpfr_const_euler_bs_struct from const_euler.c, MPFR
// 4.2.2.
struct SplitState {
    p: Natural,
    q: Natural,
    t: Natural,
    c: Natural,
    d: Natural,
    v: Natural,
}

// Six-component binary splitting over [n1, n2) for the sums
//
// S = sum_{k=0}^{N-1} H_k n^(2k) / (k!)^2 and I = sum_{k=0}^{N-1} n^(2k) / (k!)^2,
//
// where H_k is the k-th harmonic number; after the top-level call, V/((T+Q)D) = S/I. Every leaf's P
// is the same n^2, so the caller computes it once and passes it in. When `cont` is false (at the
// top level), the P and C components are not needed and are returned as zero.
//
// This is mpfr_const_euler_bs_1 from const_euler.c, MPFR 4.2.2.
fn s1(n1: u64, n2: u64, n_squared: &Natural, cont: bool) -> SplitState {
    if n2 - n1 == 1 {
        let d = Natural::from(n1 + 1);
        let q = (&d).square();
        SplitState {
            p: n_squared.clone(),
            q,
            t: n_squared.clone(),
            c: Natural::ONE,
            d,
            v: n_squared.clone(),
        }
    } else {
        let m = (n1 + n2) >> 1;
        let l = s1(n1, m, n_squared, true);
        let r = s1(m, n2, n_squared, true);
        // t = LP RT is shared between the T and V combinations
        let t = &l.p * r.t;
        SplitState {
            // T = LP RT + RQ LT
            t: (&t).add_mul(&r.q, &l.t),
            // C = LC RD + RC LD
            c: if cont {
                (r.c * &l.d).add_mul(&l.c, &r.d)
            } else {
                Natural::ZERO
            },
            // V = RD (RQ LV + LC LP RT) + LD LP RV
            v: (&r.q * l.v)
                .add_mul(t, l.c)
                .mul_add_mul(&r.d, &l.p * r.v, &l.d),
            p: if cont { l.p * r.p } else { Natural::ZERO },
            q: l.q * r.q,
            d: l.d * r.d,
        }
    }
}

// Three-component binary splitting over [n1, n2) for the sum
//
// U = (1/(4n)) sum_{k=0}^{2n-1} [(2k)!]^3 / ((k!)^4 8^(2k) (2n)^(2k)),
//
// with T/Q = the sum after the top-level call. The leaves' N^2 factor is the same everywhere, so
// the caller computes it once and passes it in. When `cont` is false (at the top level), the P
// component is not needed and is returned as zero.
//
// This is mpfr_const_euler_bs_2 from const_euler.c, MPFR 4.2.2.
fn s2(
    n1: u64,
    n2: u64,
    big_n: u64,
    n_squared: &Natural,
    cont: bool,
) -> (Natural, Natural, Natural) {
    if n2 - n1 == 1 {
        if n1 == 0 {
            (Natural::ONE, Natural::from(big_n) << 2u32, Natural::ONE)
        } else {
            let p = Natural::from((n1 << 1) - 1).pow(3);
            let q = (Natural::from(n1) * n_squared) << 5u32;
            (p.clone(), q, p)
        }
    } else {
        let m = (n1 + n2) >> 1;
        let (p, q, t) = s2(n1, m, big_n, n_squared, true);
        let (p2, q2, t2) = s2(m, n2, big_n, n_squared, true);
        let big_t = (t * &q2).add_mul(t2, &p);
        (if cont { p * p2 } else { Natural::ZERO }, q * q2, big_t)
    }
}

impl Float {
    /// Returns an approximation of Euler's constant (also known as the Euler–Mascheroni
    /// constant), $\gamma=\lim_{n\to\infty}\left(\sum_{k=1}^n\frac{1}{k}-\log n\right)$, with the
    /// given precision and rounded using the given [`RoundingMode`]. An [`Ordering`] is also
    /// returned, indicating whether the rounded value is less than or greater than the exact value
    /// of the constant. (The rounded value is never equal to the exact value of the constant.
    /// Euler's constant has not been proven irrational, but its binary expansion is known to be
    /// aperiodic far beyond any precision reachable by this function.)
    ///
    /// $$
    /// x = \gamma+\varepsilon.
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
    /// let (g, o) = Float::eulers_constant_prec_round(100, Floor);
    /// assert_eq!(g.to_string(), "0.57721566490153286060651209008234");
    /// assert_eq!(o, Less);
    ///
    /// let (g, o) = Float::eulers_constant_prec_round(100, Ceiling);
    /// assert_eq!(g.to_string(), "0.57721566490153286060651209008313");
    /// assert_eq!(o, Greater);
    /// ```
    ///
    // Euler's constant is computed using the Brent-McMillan algorithm with binary splitting, as
    // gamma = S/I - U/I^2 - log(n), with the approximation error bounded using Theorem 1 and Remark
    // 2 of Fredrik Johansson's "Evaluation of the Bessel function sum ..." paper
    // (https://arxiv.org/pdf/1312.0039v1.pdf).
    //
    // MPFR computes v - log(n) 2^wp over integers scaled by 2^wp, with the log and the subtraction
    // rounded toward zero; here the scaling is folded away by subtracting the exact Rational v/2^wp
    // from the log and negating, toward-zero rounding being symmetric under negation.
    //
    // This is mpfr_const_euler_internal from const_euler.c, MPFR 4.2.2.
    pub fn eulers_constant_prec_round(prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        let mut wp = prec + prec.ceiling_log_base_2() + 5;
        let mut increment = Limb::WIDTH;
        loop {
            // The approximation error is bounded by 24 exp(-8n) when n > 1, which is smaller than
            // 2^-wp if n > (wp + log_2(24)) * (log(2)/8). Note log2(24) < 5 and log(2)/8 < 866434 /
            // 10000000.
            let n = u64::exact_from(
                ((u128::from(wp) + 5) * 866434)
                    .div_round(10000000, Ceiling)
                    .0,
            );
            // It is sufficient to take N >= alpha*n + 1 where alpha = 3/LambertW(3/e) =
            // 4.970625759544...
            let big_n =
                u64::exact_from((u128::from(n) * 4970626).div_round(1000000, Ceiling).0) + 1;
            let n_squared = Natural::from(n).square();
            // V / ((T + Q) * D) = S / I
            let sum = s1(0, big_n, &n_squared, false);
            let t = sum.t + &sum.q;
            // s_over_i * 2^-wp = S/I with error < 1
            let s_over_i = (sum.v << wp) / (&t * sum.d);
            // T2/Q2 = 4n U after the top-level call, and u_over_i_squared * 2^-wp = U/I^2 with
            // error < 1
            let (_, q2, t2) = s2(0, n << 1, n, &n_squared, false);
            let u_over_i_squared = ((sum.q.square() * t2) << wp) / (t.square() * q2);
            // v * 2^-wp = gamma + log(n) with error at most 3*2^-wp
            let v = s_over_i - u_over_i_squared;
            // log(n) < 2^ceil(log2(n))
            let magn = n.ceiling_log_base_2();
            // y = gamma with error < 5*2^-wp
            let y = -(Self::ln_unsigned_prec_round(n, wp + magn, Down)
                .0
                .sub_rational_prec_round(Rational::from(v) >> wp, wp + magn, Down)
                .0);
            if float_can_round(y.significand_ref().unwrap(), wp - 3, prec, rm) {
                return Self::from_float_prec_round(y, prec, rm);
            }
            wp += increment;
            increment = wp >> 1;
        }
    }

    /// Returns an approximation of Euler's constant (also known as the Euler–Mascheroni
    /// constant), $\gamma=\lim_{n\to\infty}\left(\sum_{k=1}^n\frac{1}{k}-\log n\right)$, with the
    /// given precision and rounded to the nearest [`Float`] of that precision. An [`Ordering`] is
    /// also returned, indicating whether the rounded value is less than or greater than the exact
    /// value of the constant. (The rounded value is never equal to the exact value of the constant.
    /// Euler's constant has not been proven irrational, but its binary expansion is known to be
    /// aperiodic far beyond any precision reachable by this function.)
    ///
    /// $$
    /// x = \gamma+\varepsilon.
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
    /// let (g, o) = Float::eulers_constant_prec(1);
    /// assert_eq!(g.to_string(), "0.50");
    /// assert_eq!(o, Less);
    ///
    /// let (g, o) = Float::eulers_constant_prec(10);
    /// assert_eq!(g.to_string(), "0.57715");
    /// assert_eq!(o, Less);
    ///
    /// let (g, o) = Float::eulers_constant_prec(100);
    /// assert_eq!(g.to_string(), "0.57721566490153286060651209008234");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn eulers_constant_prec(prec: u64) -> (Self, Ordering) {
        Self::eulers_constant_prec_round(prec, Nearest)
    }
}
