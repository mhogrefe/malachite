// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the GNU MPFR Library.
//
//      Copyright 2001-2026 Free Software Foundation, Inc.
//
//      Contributed by the Pascaline and Caramba projects, INRIA.
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::InnerFloat::{Infinity, NaN, Zero};
use crate::arithmetic::round_near_x::float_round_near_x;
use crate::{Float, emulate_float_to_float_fn, float_infinity, float_nan, float_negative_infinity};
use core::cmp::Ordering::{self, *};
use malachite_base::fail_on_untested_path;
use malachite_base::num::arithmetic::traits::{CeilingLogBase2, Ln1PlusX, Ln1PlusXAssign, Parity};
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::One;
use malachite_base::num::conversion::traits::{ExactFrom, RoundingFrom};
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_nz::natural::arithmetic::float_extras::float_can_round;
use malachite_nz::platform::Limb;
use malachite_q::Rational;

// Computes an approximation of ln(1+x) for x small, using the Taylor expansion. Assumes |x| < 1/2
// (that is, EXP(x) <= -1), in which case |x/2| <= |ln(1+x)| <= |2x|. The result has precision
// `prec`. Returns k such that the error is bounded by 2^k ulps of the result.
//
// This is mpfr_log1p_small from log1p.c, MPFR 4.3.0.
fn ln_1_plus_x_small(x: &Float, prec: u64) -> (Float, u64) {
    assert!(x.get_exponent().unwrap() <= -1); // ensures |x| < 1/2
    // In the following, theta represents a value with |theta| <= 2^(1-prec) (might be a different
    // value each time).
    let mut t = Float::from_float_prec_ref(x, prec).0; // t = x * (1 + theta)
    let mut y = t.clone(); // exact
    let y_exp_m_prec = i64::from(y.get_exponent().unwrap()) - i64::exact_from(prec);
    let mut i = 2u32;
    loop {
        t.mul_prec_assign_ref(x, prec); // t = x^i * (1 + theta)^i
        // u = x^i / i * (1 + theta)^(i + 1)
        let u = t.div_prec_ref_val(Float::from(i), prec).0;
        // |u| < ulp(y)
        if i64::from(u.get_exponent().unwrap()) <= y_exp_m_prec {
            break;
        }
        if i.odd() {
            y.add_prec_assign(u, prec); // error <= ulp(y)
        } else {
            y.sub_prec_assign(u, prec); // error <= ulp(y)
        };
        i += 1;
    }
    // The total error is bounded by (2 * i + 8) ulps of y; see the analysis in log1p.c.
    let err = (u64::from(i) << 1) + 8;
    let k = err.ceiling_log_base_2();
    assert!(k < prec);
    (y, k)
}

// This is mpfr_log1p from log1p.c, MPFR 4.3.0, where the input is finite and nonzero.
fn ln_1_plus_x_prec_round_normal(x: &Float, prec: u64, rm: RoundingMode) -> (Float, Ordering) {
    let ex = i64::from(x.get_exponent().unwrap());
    if ex < 0 {
        // -0.5 < x < 0.5. For x > 0, |ln(1+x) - x| < x^2 / 2. For x > -0.5, |ln(1+x) - x| < x^2.
        let (err1, dir) = if *x > 0u32 {
            (-ex - 1, false)
        } else {
            (-ex, true)
        };
        if err1 > 0 {
            let err = u64::exact_from(err1);
            if err > prec + 1
                && let Some(result) = float_round_near_x(x, err, dir, prec, rm)
            {
                return result;
            }
        }
    }
    // ln(1+x) is undefined for x < -1
    match x.partial_cmp(&-1i32).unwrap() {
        Equal => {
            // ln_1_plus_x(-1) = -Infinity
            return (float_negative_infinity!(), Equal);
        }
        Less => {
            return (float_nan!(), Equal);
        }
        _ => {}
    }
    // The result is never exactly representable for finite nonzero x > -1.
    assert_ne!(rm, Exact, "Inexact ln_1_plus_x");
    // General case. Compute the precision of the intermediary variable: the optimal number of bits,
    // see algorithms.tex.
    let mut working_prec = prec + prec.ceiling_log_base_2() + 6;
    // If |x| is smaller than 2^(-e), we will lose about e bits in ln(1+x).
    if ex < 0 {
        working_prec += u64::exact_from(-ex);
    }
    let mut increment = Limb::WIDTH;
    // Assuming the AGM algorithm used by ln uses log2(p) steps for a precision of p bits, we try
    // the Taylor variant whenever EXP(x) <= -p / log2(p). The + 1 avoids a division by 0 when prec
    // = 1.
    let k = 1 + prec.ceiling_log_base_2();
    let small = ex < -i64::exact_from(prec / k);
    loop {
        let (t, err) = if small {
            // This implies EXP(x) <= -1, thus x < 1/2.
            let (t, k_err) = ln_1_plus_x_small(x, working_prec);
            (t, working_prec - k_err)
        } else {
            let (t, o) = x.add_prec_ref_val(Float::ONE, working_prec); // 1 + x
            if o == Equal {
                // t = 1 + x exactly, and the result is simply ln(t).
                return t.ln_prec_round(prec, rm);
            }
            // MPFR computes with an extended exponent range, so its 1 + x cannot overflow or
            // underflow; ours can, and both cases need rescuing.
            let t = if t == 0 {
                // 1 + x underflowed, so x is just above -1 and 1 + x is positive but smaller than
                // 2^MIN_EXPONENT. Reaching this branch requires the precision of x to exceed 2^30,
                // which no generator produces.
                fail_on_untested_path("ln_1_plus_x_prec_round_normal, 1 + x underflows");
                // The sum 1 + x is an exact dyadic rational, so use the Rational implementation of
                // ln.
                return Float::ln_rational_prec_round(
                    Rational::ONE + Rational::try_from(x).unwrap(),
                    prec,
                    rm,
                );
            } else if t.is_infinite() {
                // 1 + x overflowed, so x >= 2^working_prec and ln(1+x) differs from ln(x) by ln(1 +
                // 1/x) < 2^(1-MAX_EXPONENT), far less than an ulp; use ln(x).
                x.ln_prec_ref(working_prec).0
            } else {
                t.ln_prec(working_prec).0 // ln(1+x)
            };
            // The error is bounded by (1/2 + 2^(1-EXP(t))) * ulp(t) (cf algorithms.tex). If EXP(t)
            // >= 2, then error <= ulp(t). If EXP(t) <= 1, then error <= 2^(2-EXP(t)) * ulp(t).
            let t_exp = i64::from(t.get_exponent().unwrap());
            let cancel = u64::exact_from(core::cmp::max(0, 2 - t_exp));
            (t, working_prec - cancel)
        };
        if float_can_round(t.significand_ref().unwrap(), err, prec, rm) {
            return Float::from_float_prec_round(t, prec, rm);
        }
        // Increase the precision.
        working_prec += increment;
        increment = working_prec >> 1;
    }
}

impl Float {
    /// Computes $\ln(1+x)$, where $x$ is a [`Float`], rounding the result to the specified
    /// precision and with the specified rounding mode. The [`Float`] is taken by value. An
    /// [`Ordering`] is also returned, indicating whether the rounded value is less than, equal to,
    /// or greater than the exact value. Although `NaN`s are not comparable to any [`Float`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $\ln(1+x)$ is undefined for $x<-1$, so whenever $x<-1$, `NaN` is returned.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \ln(1+x)+\varepsilon.
    /// $$
    /// - If $\ln(1+x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\ln(1+x)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\ln(1+x)|\rfloor-p+1}$.
    /// - If $\ln(1+x)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\ln(1+x)|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=\text{NaN}$
    /// - $f(\infty,p,m)=\infty$
    /// - $f(-\infty,p,m)=\text{NaN}$
    /// - $f(\pm0.0,p,m)=\pm0.0$
    /// - $f(-1,p,m)=-\infty$
    /// - $f(x,p,m)=\text{NaN}$ for $x<-1$
    ///
    /// This function cannot overflow, but it can underflow:
    /// - If $0<f(x,p,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,p,m)<2^{-2^{30}}$, and $m$ is `Ceiling`, `Up`, or `Nearest`, $2^{-2^{30}}$ is
    ///   returned instead.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::ln_1_plus_x_prec`] instead.
    /// If you know that your target precision is the precision of the input, consider using
    /// [`Float::ln_1_plus_x_round`] instead. If both of these things are true, consider using
    /// [`Float::ln_1_plus_x`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n (\log n)^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with the given precision. (The result cannot be represented exactly whenever the input is
    /// finite, nonzero, and greater than $-1$.)
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (ln, o) = Float::from_unsigned_prec(10u32, 100)
    ///     .0
    ///     .ln_1_plus_x_prec_round(5, Floor);
    /// assert_eq!(ln.to_string(), "2.4");
    /// assert_eq!(o, Less);
    ///
    /// let (ln, o) = Float::from_unsigned_prec(10u32, 100)
    ///     .0
    ///     .ln_1_plus_x_prec_round(5, Ceiling);
    /// assert_eq!(ln.to_string(), "2.5");
    /// assert_eq!(o, Greater);
    ///
    /// let (ln, o) = Float::from_unsigned_prec(10u32, 100)
    ///     .0
    ///     .ln_1_plus_x_prec_round(5, Nearest);
    /// assert_eq!(ln.to_string(), "2.4");
    /// assert_eq!(o, Less);
    ///
    /// let (ln, o) = Float::from_unsigned_prec(10u32, 100)
    ///     .0
    ///     .ln_1_plus_x_prec_round(20, Floor);
    /// assert_eq!(ln.to_string(), "2.397892");
    /// assert_eq!(o, Less);
    ///
    /// let (ln, o) = Float::from_unsigned_prec(10u32, 100)
    ///     .0
    ///     .ln_1_plus_x_prec_round(20, Ceiling);
    /// assert_eq!(ln.to_string(), "2.397896");
    /// assert_eq!(o, Greater);
    ///
    /// let (ln, o) = Float::from_unsigned_prec(10u32, 100)
    ///     .0
    ///     .ln_1_plus_x_prec_round(20, Nearest);
    /// assert_eq!(ln.to_string(), "2.397896");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn ln_1_plus_x_prec_round(self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        assert_ne!(prec, 0);
        match self {
            Self(NaN | Infinity { sign: false }) => (float_nan!(), Equal),
            float_infinity!() => (float_infinity!(), Equal),
            // ln_1_plus_x(±0) = ±0
            Self(Zero { .. }) => (self, Equal),
            _ => ln_1_plus_x_prec_round_normal(&self, prec, rm),
        }
    }

    /// Computes $\ln(1+x)$, where $x$ is a [`Float`], rounding the result to the specified
    /// precision and with the specified rounding mode. The [`Float`] is taken by reference. An
    /// [`Ordering`] is also returned, indicating whether the rounded value is less than, equal to,
    /// or greater than the exact value. Although `NaN`s are not comparable to any [`Float`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $\ln(1+x)$ is undefined for $x<-1$, so whenever $x<-1$, `NaN` is returned.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \ln(1+x)+\varepsilon.
    /// $$
    /// - If $\ln(1+x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\ln(1+x)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\ln(1+x)|\rfloor-p+1}$.
    /// - If $\ln(1+x)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\ln(1+x)|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=\text{NaN}$
    /// - $f(\infty,p,m)=\infty$
    /// - $f(-\infty,p,m)=\text{NaN}$
    /// - $f(\pm0.0,p,m)=\pm0.0$
    /// - $f(-1,p,m)=-\infty$
    /// - $f(x,p,m)=\text{NaN}$ for $x<-1$
    ///
    /// This function cannot overflow, but it can underflow:
    /// - If $0<f(x,p,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,p,m)<2^{-2^{30}}$, and $m$ is `Ceiling`, `Up`, or `Nearest`, $2^{-2^{30}}$ is
    ///   returned instead.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::ln_1_plus_x_prec_ref`]
    /// instead. If you know that your target precision is the precision of the input, consider
    /// using [`Float::ln_1_plus_x_round_ref`] instead. If both of these things are true, consider
    /// using `(&Float).ln_1_plus_x()` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n (\log n)^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with the given precision. (The result cannot be represented exactly whenever the input is
    /// finite, nonzero, and greater than $-1$.)
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (ln, o) = Float::from_unsigned_prec(10u32, 100)
    ///     .0
    ///     .ln_1_plus_x_prec_round_ref(5, Floor);
    /// assert_eq!(ln.to_string(), "2.4");
    /// assert_eq!(o, Less);
    ///
    /// let (ln, o) = Float::from_unsigned_prec(10u32, 100)
    ///     .0
    ///     .ln_1_plus_x_prec_round_ref(5, Ceiling);
    /// assert_eq!(ln.to_string(), "2.5");
    /// assert_eq!(o, Greater);
    ///
    /// let (ln, o) = Float::from_unsigned_prec(10u32, 100)
    ///     .0
    ///     .ln_1_plus_x_prec_round_ref(5, Nearest);
    /// assert_eq!(ln.to_string(), "2.4");
    /// assert_eq!(o, Less);
    ///
    /// let (ln, o) = Float::from_unsigned_prec(10u32, 100)
    ///     .0
    ///     .ln_1_plus_x_prec_round_ref(20, Floor);
    /// assert_eq!(ln.to_string(), "2.397892");
    /// assert_eq!(o, Less);
    ///
    /// let (ln, o) = Float::from_unsigned_prec(10u32, 100)
    ///     .0
    ///     .ln_1_plus_x_prec_round_ref(20, Ceiling);
    /// assert_eq!(ln.to_string(), "2.397896");
    /// assert_eq!(o, Greater);
    ///
    /// let (ln, o) = Float::from_unsigned_prec(10u32, 100)
    ///     .0
    ///     .ln_1_plus_x_prec_round_ref(20, Nearest);
    /// assert_eq!(ln.to_string(), "2.397896");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn ln_1_plus_x_prec_round_ref(&self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        assert_ne!(prec, 0);
        match self {
            Self(NaN | Infinity { sign: false }) => (float_nan!(), Equal),
            float_infinity!() => (float_infinity!(), Equal),
            Self(Zero { sign }) => (Self(Zero { sign: *sign }), Equal),
            _ => ln_1_plus_x_prec_round_normal(self, prec, rm),
        }
    }

    /// Computes $\ln(1+x)$, where $x$ is a [`Float`], rounding the result to the nearest value of
    /// the specified precision. The [`Float`] is taken by value. An [`Ordering`] is also returned,
    /// indicating whether the rounded value is less than, equal to, or greater than the exact
    /// value. Although `NaN`s are not comparable to any [`Float`], whenever this function returns a
    /// `NaN` it also returns `Equal`.
    ///
    /// $\ln(1+x)$ is undefined for $x<-1$, so whenever $x<-1$, `NaN` is returned.
    ///
    /// If the result is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = \ln(1+x)+\varepsilon.
    /// $$
    /// - If $\ln(1+x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\ln(1+x)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\ln(1+x)|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p)=\text{NaN}$
    /// - $f(\infty,p)=\infty$
    /// - $f(-\infty,p)=\text{NaN}$
    /// - $f(\pm0.0,p)=\pm0.0$
    /// - $f(-1,p)=-\infty$
    /// - $f(x,p)=\text{NaN}$ for $x<-1$
    ///
    /// This function cannot overflow, but it can underflow: if $0<f(x,p)<2^{-2^{30}}$,
    /// $2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::ln_1_plus_x_prec_round`] instead. If you know that your target precision is the
    /// precision of the input, consider using [`Float::ln_1_plus_x`] instead.
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
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (ln, o) = Float::from_unsigned_prec(10u32, 100).0.ln_1_plus_x_prec(5);
    /// assert_eq!(ln.to_string(), "2.4");
    /// assert_eq!(o, Less);
    ///
    /// let (ln, o) = Float::from_unsigned_prec(10u32, 100).0.ln_1_plus_x_prec(20);
    /// assert_eq!(ln.to_string(), "2.397896");
    /// assert_eq!(o, Greater);
    ///
    /// let (ln, o) = Float::ONE.ln_1_plus_x_prec(20);
    /// assert_eq!(ln.to_string(), "0.693147");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn ln_1_plus_x_prec(self, prec: u64) -> (Self, Ordering) {
        self.ln_1_plus_x_prec_round(prec, Nearest)
    }

    /// Computes $\ln(1+x)$, where $x$ is a [`Float`], rounding the result to the nearest value of
    /// the specified precision. The [`Float`] is taken by reference. An [`Ordering`] is also
    /// returned, indicating whether the rounded value is less than, equal to, or greater than the
    /// exact value. Although `NaN`s are not comparable to any [`Float`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// $\ln(1+x)$ is undefined for $x<-1$, so whenever $x<-1$, `NaN` is returned.
    ///
    /// If the result is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = \ln(1+x)+\varepsilon.
    /// $$
    /// - If $\ln(1+x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\ln(1+x)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\ln(1+x)|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p)=\text{NaN}$
    /// - $f(\infty,p)=\infty$
    /// - $f(-\infty,p)=\text{NaN}$
    /// - $f(\pm0.0,p)=\pm0.0$
    /// - $f(-1,p)=-\infty$
    /// - $f(x,p)=\text{NaN}$ for $x<-1$
    ///
    /// This function cannot overflow, but it can underflow: if $0<f(x,p)<2^{-2^{30}}$,
    /// $2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::ln_1_plus_x_prec_round_ref`] instead. If you know that your target precision is the
    /// precision of the input, consider using `(&Float).ln_1_plus_x()` instead.
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
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (ln, o) = Float::from_unsigned_prec(10u32, 100)
    ///     .0
    ///     .ln_1_plus_x_prec_ref(5);
    /// assert_eq!(ln.to_string(), "2.4");
    /// assert_eq!(o, Less);
    ///
    /// let (ln, o) = Float::from_unsigned_prec(10u32, 100)
    ///     .0
    ///     .ln_1_plus_x_prec_ref(20);
    /// assert_eq!(ln.to_string(), "2.397896");
    /// assert_eq!(o, Greater);
    ///
    /// let (ln, o) = Float::ONE.ln_1_plus_x_prec_ref(20);
    /// assert_eq!(ln.to_string(), "0.693147");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn ln_1_plus_x_prec_ref(&self, prec: u64) -> (Self, Ordering) {
        self.ln_1_plus_x_prec_round_ref(prec, Nearest)
    }

    /// Computes $\ln(1+x)$, where $x$ is a [`Float`], rounding the result with the specified
    /// rounding mode. The [`Float`] is taken by value. An [`Ordering`] is also returned, indicating
    /// whether the rounded value is less than, equal to, or greater than the exact value. Although
    /// `NaN`s are not comparable to any [`Float`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// $\ln(1+x)$ is undefined for $x<-1$, so whenever $x<-1$, `NaN` is returned.
    ///
    /// The precision of the output is the precision of the input. See [`RoundingMode`] for a
    /// description of the possible rounding modes.
    ///
    /// $$
    /// f(x,m) = \ln(1+x)+\varepsilon.
    /// $$
    /// - If $\ln(1+x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\ln(1+x)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\ln(1+x)|\rfloor-p+1}$, where $p$ is the precision of the input.
    /// - If $\ln(1+x)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\ln(1+x)|\rfloor-p}$, where $p$ is the precision of the input.
    ///
    /// If the output has a precision, it is the precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN},m)=\text{NaN}$
    /// - $f(\infty,m)=\infty$
    /// - $f(-\infty,m)=\text{NaN}$
    /// - $f(\pm0.0,m)=\pm0.0$
    /// - $f(-1,m)=-\infty$
    /// - $f(x,m)=\text{NaN}$ for $x<-1$
    ///
    /// This function cannot overflow, but it can underflow:
    /// - If $0<f(x,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,m)<2^{-2^{30}}$, and $m$ is `Ceiling`, `Up`, or `Nearest`, $2^{-2^{30}}$ is
    ///   returned instead.
    ///
    /// If you want to specify an output precision, consider using [`Float::ln_1_plus_x_prec_round`]
    /// instead. If you know you'll be using the `Nearest` rounding mode, consider using
    /// [`Float::ln_1_plus_x`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n (\log n)^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.get_prec()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the result cannot be represented exactly with the input
    /// precision. (The result cannot be represented exactly whenever the input is finite, nonzero,
    /// and greater than $-1$.)
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (ln, o) = Float::from_unsigned_prec(10u32, 100)
    ///     .0
    ///     .ln_1_plus_x_round(Floor);
    /// assert_eq!(ln.to_string(), "2.397895272798370544061943577962");
    /// assert_eq!(o, Less);
    ///
    /// let (ln, o) = Float::from_unsigned_prec(10u32, 100)
    ///     .0
    ///     .ln_1_plus_x_round(Ceiling);
    /// assert_eq!(ln.to_string(), "2.397895272798370544061943577965");
    /// assert_eq!(o, Greater);
    ///
    /// let (ln, o) = Float::from_unsigned_prec(10u32, 100)
    ///     .0
    ///     .ln_1_plus_x_round(Nearest);
    /// assert_eq!(ln.to_string(), "2.397895272798370544061943577965");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn ln_1_plus_x_round(self, rm: RoundingMode) -> (Self, Ordering) {
        let prec = self.significant_bits();
        self.ln_1_plus_x_prec_round(prec, rm)
    }

    /// Computes $\ln(1+x)$, where $x$ is a [`Float`], rounding the result with the specified
    /// rounding mode. The [`Float`] is taken by reference. An [`Ordering`] is also returned,
    /// indicating whether the rounded value is less than, equal to, or greater than the exact
    /// value. Although `NaN`s are not comparable to any [`Float`], whenever this function returns a
    /// `NaN` it also returns `Equal`.
    ///
    /// $\ln(1+x)$ is undefined for $x<-1$, so whenever $x<-1$, `NaN` is returned.
    ///
    /// The precision of the output is the precision of the input. See [`RoundingMode`] for a
    /// description of the possible rounding modes.
    ///
    /// $$
    /// f(x,m) = \ln(1+x)+\varepsilon.
    /// $$
    /// - If $\ln(1+x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\ln(1+x)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\ln(1+x)|\rfloor-p+1}$, where $p$ is the precision of the input.
    /// - If $\ln(1+x)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\ln(1+x)|\rfloor-p}$, where $p$ is the precision of the input.
    ///
    /// If the output has a precision, it is the precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN},m)=\text{NaN}$
    /// - $f(\infty,m)=\infty$
    /// - $f(-\infty,m)=\text{NaN}$
    /// - $f(\pm0.0,m)=\pm0.0$
    /// - $f(-1,m)=-\infty$
    /// - $f(x,m)=\text{NaN}$ for $x<-1$
    ///
    /// This function cannot overflow, but it can underflow:
    /// - If $0<f(x,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,m)<2^{-2^{30}}$, and $m$ is `Ceiling`, `Up`, or `Nearest`, $2^{-2^{30}}$ is
    ///   returned instead.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::ln_1_plus_x_prec_round_ref`] instead. If you know you'll be using the `Nearest`
    /// rounding mode, consider using `(&Float).ln_1_plus_x()` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n (\log n)^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.get_prec()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the result cannot be represented exactly with the input
    /// precision. (The result cannot be represented exactly whenever the input is finite, nonzero,
    /// and greater than $-1$.)
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (ln, o) = Float::from_unsigned_prec(10u32, 100)
    ///     .0
    ///     .ln_1_plus_x_round_ref(Floor);
    /// assert_eq!(ln.to_string(), "2.397895272798370544061943577962");
    /// assert_eq!(o, Less);
    ///
    /// let (ln, o) = Float::from_unsigned_prec(10u32, 100)
    ///     .0
    ///     .ln_1_plus_x_round_ref(Ceiling);
    /// assert_eq!(ln.to_string(), "2.397895272798370544061943577965");
    /// assert_eq!(o, Greater);
    ///
    /// let (ln, o) = Float::from_unsigned_prec(10u32, 100)
    ///     .0
    ///     .ln_1_plus_x_round_ref(Nearest);
    /// assert_eq!(ln.to_string(), "2.397895272798370544061943577965");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn ln_1_plus_x_round_ref(&self, rm: RoundingMode) -> (Self, Ordering) {
        self.ln_1_plus_x_prec_round_ref(self.significant_bits(), rm)
    }

    /// Computes $\ln(1+x)$, where $x$ is a [`Float`], in place, rounding the result to the
    /// specified precision and with the specified rounding mode. An [`Ordering`] is returned,
    /// indicating whether the rounded value is less than, equal to, or greater than the exact
    /// value. Although `NaN`s are not comparable to any [`Float`], whenever this function sets the
    /// [`Float`] to `NaN` it also returns `Equal`.
    ///
    /// $\ln(1+x)$ is undefined for $x<-1$, so whenever $x<-1$, the [`Float`] is set to `NaN`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets \ln(1+x)+\varepsilon.
    /// $$
    /// - If $\ln(1+x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\ln(1+x)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\ln(1+x)|\rfloor-p+1}$.
    /// - If $\ln(1+x)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\ln(1+x)|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// See the [`Float::ln_1_plus_x_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::ln_1_plus_x_prec_assign`]
    /// instead. If you know that your target precision is the precision of the input, consider
    /// using [`Float::ln_1_plus_x_round_assign`] instead. If both of these things are true,
    /// consider using [`Float::ln_1_plus_x_assign`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n (\log n)^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with the given precision. (The result cannot be represented exactly whenever the input is
    /// finite, nonzero, and greater than $-1$.)
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from_unsigned_prec(10u32, 100).0;
    /// assert_eq!(x.ln_1_plus_x_prec_round_assign(5, Floor), Less);
    /// assert_eq!(x.to_string(), "2.4");
    ///
    /// let mut x = Float::from_unsigned_prec(10u32, 100).0;
    /// assert_eq!(x.ln_1_plus_x_prec_round_assign(5, Ceiling), Greater);
    /// assert_eq!(x.to_string(), "2.5");
    ///
    /// let mut x = Float::from_unsigned_prec(10u32, 100).0;
    /// assert_eq!(x.ln_1_plus_x_prec_round_assign(5, Nearest), Less);
    /// assert_eq!(x.to_string(), "2.4");
    ///
    /// let mut x = Float::from_unsigned_prec(10u32, 100).0;
    /// assert_eq!(x.ln_1_plus_x_prec_round_assign(20, Floor), Less);
    /// assert_eq!(x.to_string(), "2.397892");
    ///
    /// let mut x = Float::from_unsigned_prec(10u32, 100).0;
    /// assert_eq!(x.ln_1_plus_x_prec_round_assign(20, Ceiling), Greater);
    /// assert_eq!(x.to_string(), "2.397896");
    ///
    /// let mut x = Float::from_unsigned_prec(10u32, 100).0;
    /// assert_eq!(x.ln_1_plus_x_prec_round_assign(20, Nearest), Greater);
    /// assert_eq!(x.to_string(), "2.397896");
    /// ```
    #[inline]
    pub fn ln_1_plus_x_prec_round_assign(&mut self, prec: u64, rm: RoundingMode) -> Ordering {
        let (result, o) = core::mem::take(self).ln_1_plus_x_prec_round(prec, rm);
        *self = result;
        o
    }

    /// Computes $\ln(1+x)$, where $x$ is a [`Float`], in place, rounding the result to the nearest
    /// value of the specified precision. An [`Ordering`] is returned, indicating whether the
    /// rounded value is less than, equal to, or greater than the exact value. Although `NaN`s are
    /// not comparable to any [`Float`], whenever this function sets the [`Float`] to `NaN` it also
    /// returns `Equal`.
    ///
    /// $\ln(1+x)$ is undefined for $x<-1$, so whenever $x<-1$, the [`Float`] is set to `NaN`.
    ///
    /// If the result is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// x \gets \ln(1+x)+\varepsilon.
    /// $$
    /// - If $\ln(1+x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\ln(1+x)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\ln(1+x)|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// See the [`Float::ln_1_plus_x_prec`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::ln_1_plus_x_prec_round_assign`] instead. If you know that your target precision is
    /// the precision of the input, consider using [`Float::ln_1_plus_x_assign`] instead.
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
    /// let mut x = Float::from_unsigned_prec(10u32, 100).0;
    /// assert_eq!(x.ln_1_plus_x_prec_assign(5), Less);
    /// assert_eq!(x.to_string(), "2.4");
    ///
    /// let mut x = Float::from_unsigned_prec(10u32, 100).0;
    /// assert_eq!(x.ln_1_plus_x_prec_assign(20), Greater);
    /// assert_eq!(x.to_string(), "2.397896");
    /// ```
    #[inline]
    pub fn ln_1_plus_x_prec_assign(&mut self, prec: u64) -> Ordering {
        self.ln_1_plus_x_prec_round_assign(prec, Nearest)
    }

    /// Computes $\ln(1+x)$, where $x$ is a [`Float`], in place, rounding the result with the
    /// specified rounding mode. An [`Ordering`] is returned, indicating whether the rounded value
    /// is less than, equal to, or greater than the exact value. Although `NaN`s are not comparable
    /// to any [`Float`], whenever this function sets the [`Float`] to `NaN` it also returns
    /// `Equal`.
    ///
    /// $\ln(1+x)$ is undefined for $x<-1$, so whenever $x<-1$, the [`Float`] is set to `NaN`.
    ///
    /// The precision of the output is the precision of the input. See [`RoundingMode`] for a
    /// description of the possible rounding modes.
    ///
    /// $$
    /// x \gets \ln(1+x)+\varepsilon.
    /// $$
    /// - If $\ln(1+x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\ln(1+x)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\ln(1+x)|\rfloor-p+1}$, where $p$ is the precision of the input.
    /// - If $\ln(1+x)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\ln(1+x)|\rfloor-p}$, where $p$ is the precision of the input.
    ///
    /// If the output has a precision, it is the precision of the input.
    ///
    /// See the [`Float::ln_1_plus_x_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::ln_1_plus_x_prec_round_assign`] instead. If you know you'll be using the `Nearest`
    /// rounding mode, consider using [`Float::ln_1_plus_x_assign`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n (\log n)^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.get_prec()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the result cannot be represented exactly with the input
    /// precision. (The result cannot be represented exactly whenever the input is finite, nonzero,
    /// and greater than $-1$.)
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from_unsigned_prec(10u32, 100).0;
    /// assert_eq!(x.ln_1_plus_x_round_assign(Floor), Less);
    /// assert_eq!(x.to_string(), "2.397895272798370544061943577962");
    ///
    /// let mut x = Float::from_unsigned_prec(10u32, 100).0;
    /// assert_eq!(x.ln_1_plus_x_round_assign(Ceiling), Greater);
    /// assert_eq!(x.to_string(), "2.397895272798370544061943577965");
    ///
    /// let mut x = Float::from_unsigned_prec(10u32, 100).0;
    /// assert_eq!(x.ln_1_plus_x_round_assign(Nearest), Greater);
    /// assert_eq!(x.to_string(), "2.397895272798370544061943577965");
    /// ```
    #[inline]
    pub fn ln_1_plus_x_round_assign(&mut self, rm: RoundingMode) -> Ordering {
        let prec = self.significant_bits();
        self.ln_1_plus_x_prec_round_assign(prec, rm)
    }
}

impl Ln1PlusX for Float {
    type Output = Self;

    /// Computes $\ln(1+x)$, where $x$ is a [`Float`], taking the [`Float`] by value.
    ///
    /// If the output has a precision, it is the precision of the input. If the result is
    /// equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s in
    /// its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// $\ln(1+x)$ is undefined for $x<-1$, so whenever $x<-1$, `NaN` is returned.
    ///
    /// $$
    /// f(x) = \ln(1+x)+\varepsilon.
    /// $$
    /// - If $\ln(1+x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\ln(1+x)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\ln(1+x)|\rfloor-p}$, where $p$ is the precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN})=\text{NaN}$
    /// - $f(\infty)=\infty$
    /// - $f(-\infty)=\text{NaN}$
    /// - $f(\pm0.0)=\pm0.0$
    /// - $f(-1)=-\infty$
    /// - $f(x)=\text{NaN}$ for $x<-1$
    ///
    /// This function cannot overflow, but it can underflow: if $0<f(x)<2^{-2^{30}}$, $2^{-2^{30}}$
    /// is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::ln_1_plus_x_round`] instead. If you want to specify the output precision, consider
    /// using [`Float::ln_1_plus_x_prec`]. If you want both of these things, consider using
    /// [`Float::ln_1_plus_x_prec_round`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n (\log n)^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.get_prec()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Ln1PlusX;
    /// use malachite_base::num::basic::traits::{
    ///     Infinity, NaN, NegativeInfinity, NegativeOne, One,
    /// };
    /// use malachite_float::Float;
    ///
    /// assert!(Float::NAN.ln_1_plus_x().is_nan());
    /// assert_eq!(Float::INFINITY.ln_1_plus_x(), Float::INFINITY);
    /// assert!(Float::NEGATIVE_INFINITY.ln_1_plus_x().is_nan());
    /// assert_eq!(Float::ONE.ln_1_plus_x().to_string(), "0.5");
    /// assert_eq!(
    ///     Float::from_unsigned_prec(10u32, 100)
    ///         .0
    ///         .ln_1_plus_x()
    ///         .to_string(),
    ///     "2.397895272798370544061943577965"
    /// );
    /// assert_eq!(Float::NEGATIVE_ONE.ln_1_plus_x(), Float::NEGATIVE_INFINITY);
    /// assert!(Float::from_signed_prec(-10, 100).0.ln_1_plus_x().is_nan());
    /// ```
    #[inline]
    fn ln_1_plus_x(self) -> Self {
        let prec = self.significant_bits();
        self.ln_1_plus_x_prec(prec).0
    }
}

impl Ln1PlusX for &Float {
    type Output = Float;

    /// Computes $\ln(1+x)$, where $x$ is a [`Float`], taking the [`Float`] by reference.
    ///
    /// If the output has a precision, it is the precision of the input. If the result is
    /// equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s in
    /// its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// $\ln(1+x)$ is undefined for $x<-1$, so whenever $x<-1$, `NaN` is returned.
    ///
    /// $$
    /// f(x) = \ln(1+x)+\varepsilon.
    /// $$
    /// - If $\ln(1+x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\ln(1+x)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\ln(1+x)|\rfloor-p}$, where $p$ is the precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN})=\text{NaN}$
    /// - $f(\infty)=\infty$
    /// - $f(-\infty)=\text{NaN}$
    /// - $f(\pm0.0)=\pm0.0$
    /// - $f(-1)=-\infty$
    /// - $f(x)=\text{NaN}$ for $x<-1$
    ///
    /// This function cannot overflow, but it can underflow: if $0<f(x)<2^{-2^{30}}$, $2^{-2^{30}}$
    /// is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::ln_1_plus_x_round_ref`] instead. If you want to specify the output precision,
    /// consider using [`Float::ln_1_plus_x_prec_ref`]. If you want both of these things, consider
    /// using [`Float::ln_1_plus_x_prec_round_ref`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n (\log n)^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.get_prec()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Ln1PlusX;
    /// use malachite_base::num::basic::traits::{
    ///     Infinity, NaN, NegativeInfinity, NegativeOne, One,
    /// };
    /// use malachite_float::Float;
    ///
    /// assert!((&Float::NAN).ln_1_plus_x().is_nan());
    /// assert_eq!((&Float::INFINITY).ln_1_plus_x(), Float::INFINITY);
    /// assert!((&Float::NEGATIVE_INFINITY).ln_1_plus_x().is_nan());
    /// assert_eq!((&Float::ONE).ln_1_plus_x().to_string(), "0.5");
    /// assert_eq!(
    ///     (&Float::from_unsigned_prec(10u32, 100).0)
    ///         .ln_1_plus_x()
    ///         .to_string(),
    ///     "2.397895272798370544061943577965"
    /// );
    /// assert_eq!(
    ///     (&Float::NEGATIVE_ONE).ln_1_plus_x(),
    ///     Float::NEGATIVE_INFINITY
    /// );
    /// assert!((&Float::from_signed_prec(-10, 100).0)
    ///     .ln_1_plus_x()
    ///     .is_nan());
    /// ```
    #[inline]
    fn ln_1_plus_x(self) -> Float {
        self.ln_1_plus_x_prec_round_ref(self.significant_bits(), Nearest)
            .0
    }
}

impl Ln1PlusXAssign for Float {
    /// Computes $\ln(1+x)$, where $x$ is a [`Float`], in place.
    ///
    /// If the output has a precision, it is the precision of the input. If the result is
    /// equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s in
    /// its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// $\ln(1+x)$ is undefined for $x<-1$, so whenever $x<-1$, the [`Float`] is set to `NaN`.
    ///
    /// $$
    /// x \gets \ln(1+x)+\varepsilon.
    /// $$
    /// - If $\ln(1+x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\ln(1+x)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\ln(1+x)|\rfloor-p}$, where $p$ is the precision of the input.
    ///
    /// See the [`Float::ln_1_plus_x`] documentation for information on special cases, overflow, and
    /// underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::ln_1_plus_x_round_assign`] instead. If you want to specify the output precision,
    /// consider using [`Float::ln_1_plus_x_prec_assign`]. If you want both of these things,
    /// consider using [`Float::ln_1_plus_x_prec_round_assign`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n (\log n)^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.get_prec()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Ln1PlusXAssign;
    /// use malachite_base::num::basic::traits::{
    ///     Infinity, NaN, NegativeInfinity, NegativeOne, One,
    /// };
    /// use malachite_float::Float;
    ///
    /// let mut x = Float::NAN;
    /// x.ln_1_plus_x_assign();
    /// assert!(x.is_nan());
    ///
    /// let mut x = Float::INFINITY;
    /// x.ln_1_plus_x_assign();
    /// assert_eq!(x, Float::INFINITY);
    ///
    /// let mut x = Float::NEGATIVE_INFINITY;
    /// x.ln_1_plus_x_assign();
    /// assert!(x.is_nan());
    ///
    /// let mut x = Float::ONE;
    /// x.ln_1_plus_x_assign();
    /// assert_eq!(x.to_string(), "0.5");
    ///
    /// let mut x = Float::from_unsigned_prec(10u32, 100).0;
    /// x.ln_1_plus_x_assign();
    /// assert_eq!(x.to_string(), "2.397895272798370544061943577965");
    ///
    /// let mut x = Float::NEGATIVE_ONE;
    /// x.ln_1_plus_x_assign();
    /// assert_eq!(x, Float::NEGATIVE_INFINITY);
    ///
    /// let mut x = Float::from_signed_prec(-10, 100).0;
    /// x.ln_1_plus_x_assign();
    /// assert!(x.is_nan());
    /// ```
    #[inline]
    fn ln_1_plus_x_assign(&mut self) {
        let prec = self.significant_bits();
        self.ln_1_plus_x_prec_round_assign(prec, Nearest);
    }
}

/// Computes the natural logarithm of one plus a primitive float, $\ln(1+x)$. Using this function is
/// more accurate than using the primitive float `ln_1p` function (the standard library's `ln_1p` is
/// not correctly rounded).
///
/// $\ln(1+x)$ is undefined for $x<-1$, so whenever $x<-1$, `NaN` is returned.
///
/// $$
/// f(x) = \ln(1+x)+\varepsilon.
/// $$
/// - If $\ln(1+x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
/// - If $\ln(1+x)$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2
///   |\ln(1+x)|\rfloor-p}$, where $p$ is precision of the output (typically 24 if `T` is a [`f32`]
///   and 53 if `T` is a [`f64`], but less if the output is subnormal).
///
/// Special cases:
/// - $f(\text{NaN})=\text{NaN}$
/// - $f(\infty)=\infty$
/// - $f(-\infty)=\text{NaN}$
/// - $f(\pm0.0)=\pm0.0$
/// - $f(-1.0)=-\infty$
/// - $f(x)=\text{NaN}$ for $x<-1$
///
/// Neither overflow nor underflow is possible.
///
/// # Worst-case complexity
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use malachite_base::num::basic::traits::NegativeInfinity;
/// use malachite_base::num::float::NiceFloat;
/// use malachite_float::arithmetic::ln_1_plus_x::primitive_float_ln_1_plus_x;
///
/// assert!(primitive_float_ln_1_plus_x(f32::NAN).is_nan());
/// assert_eq!(
///     NiceFloat(primitive_float_ln_1_plus_x(f32::INFINITY)),
///     NiceFloat(f32::INFINITY)
/// );
/// assert!(primitive_float_ln_1_plus_x(f32::NEGATIVE_INFINITY).is_nan());
/// assert_eq!(
///     NiceFloat(primitive_float_ln_1_plus_x(-1.0f32)),
///     NiceFloat(f32::NEGATIVE_INFINITY)
/// );
/// assert!(primitive_float_ln_1_plus_x(-2.0f32).is_nan());
/// assert_eq!(
///     NiceFloat(primitive_float_ln_1_plus_x(1.0f32)),
///     NiceFloat(0.6931472)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_ln_1_plus_x(7.0f32)),
///     NiceFloat(2.0794415)
/// );
/// ```
#[inline]
#[allow(clippy::type_repetition_in_bounds)]
pub fn primitive_float_ln_1_plus_x<T: PrimitiveFloat>(x: T) -> T
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    emulate_float_to_float_fn(Float::ln_1_plus_x_prec, x)
}
