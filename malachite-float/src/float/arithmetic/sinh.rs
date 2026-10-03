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

use crate::InnerFloat::{Finite, Infinity, NaN, Zero};
use crate::float::arithmetic::cos::round_bracket;
use crate::float::arithmetic::cosh::{hyperbolic_approx, hyperbolic_can_round};
use crate::float::arithmetic::round_near_x::small_input_shortcut;
use crate::float::arithmetic::sin::{UNDERFLOW_EXPONENT, underflowed};
use crate::float::conversion::string::set_str::overflow;
use crate::{Float, emulate_float_to_float_fn, emulate_rational_to_float_fn, floor_and_ceiling};
use core::cmp::Ordering::{self, Equal};
use core::cmp::max;
use malachite_base::num::arithmetic::traits::{
    Abs, CeilingLogBase2, FloorLogBase2, PowerOf2, Sinh, SinhAssign, Square,
};
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::{NaN as NaNTrait, One, Zero as ZeroTrait};
use malachite_base::num::conversion::traits::{ExactFrom, RoundingFrom};
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{self, Exact, Floor, Nearest};
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use malachite_q::Rational;

// This is mpfr_sinh from sinh.c, MPFR 4.2.2, where the input is finite and nonzero.
fn sinh_prec_round_normal_ref(x: &Float, prec: u64, rm: RoundingMode) -> (Float, Ordering) {
    assert_ne!(rm, Exact, "Inexact sinh");
    let exp_x = i64::from(x.get_exponent().unwrap());
    // sinh(x) = x + x^3/6 + ..., so the error is < 2^(3*EXP(x)-2).
    if let Some(result) = small_input_shortcut(x, -(exp_x << 1), 2, true, prec, rm) {
        return result;
    }
    let positive = x.is_sign_positive();
    let x_abs = x.abs();
    // The optimal number of bits: see algorithms.tex
    let mut working_prec = max(x_abs.get_prec().unwrap(), prec);
    working_prec += working_prec.ceiling_log_base_2() + 4;
    // If x is near 0, exp(x) - 1/exp(x) = 2*x+x^3/3+O(x^5), so the subtraction below loses about -2
    // EXP(x) bits.
    if exp_x < 0 {
        working_prec += u64::exact_from(-(exp_x << 1));
    }
    let mut increment = Limb::WIDTH;
    let sinh_abs = loop {
        let Some(approx) = hyperbolic_approx(&x_abs, working_prec) else {
            return overflow(positive, prec, rm);
        };
        if hyperbolic_can_round(&approx.sinh, approx.sinh_bits, prec, rm) {
            break approx.sinh;
        }
        working_prec += increment;
        increment = working_prec >> 1;
    };
    Float::from_float_prec_round(if positive { sinh_abs } else { -sinh_abs }, prec, rm)
}

// A bound for sinh(t), for a nonzero `Rational` t with |t| < 1/2, from the partial sum S_k of its
// series, t + t^3/3! + ... + t^(2k-1)/(2k-1)!, with k chosen from the bit length of t alone so that
// the first omitted term t^(2k+1)/(2k+1)! is below |t| 2^-(w+4). Every term has the sign of t, so
// S_k is a bound on the side toward zero, and the remainder, less than twice the first omitted
// term, is below |t| 2^-(w+3) <= |S_k| 2^-(w+3), so S_k moved away from zero by |S_k| 2^-(w+3) is a
// bound on the other side. The move is a multiplication by 2^(w+3) + 1 followed by a shift, which
// only reduces a small integer against the denominator, rather than an addition, which would take a
// GCD of two denominators, ruinous when t has a 2^30-bit one.
fn sinh_bound(t: &Rational, w: u64, away_from_zero: bool) -> Rational {
    // |t| < 2^(log + 1), with log < 0
    let log = t.floor_log_base_2_abs();
    assert!(log < -1);
    // |t|^(2k) / (2k + 1)! < 2^(2k (log + 1) - log_factorial), where log_factorial <= log2((2k +
    // 1)!)
    let mut k = 1u64;
    let mut log_factorial = 2u64; // floor(log2(2)) + floor(log2(3))
    let target = -i128::from(w) - 4;
    while i128::from(k << 1) * i128::from(log + 1) - i128::from(log_factorial) > target {
        k += 1;
        let two_k = k << 1;
        log_factorial += two_k.floor_log_base_2() + (two_k + 1).floor_log_base_2();
    }
    let mut s = t.clone();
    if k > 1 {
        let t_squared = t.square();
        let mut term = t.clone();
        for j in 1..k {
            term *= &t_squared;
            term /= Rational::from((j << 1) * ((j << 1) + 1));
            s += &term;
        }
    }
    if away_from_zero {
        let shift = w + 3;
        s *= Rational::from(Natural::power_of_2(shift) + Natural::ONE);
        s >>= shift;
    }
    s
}

// Brackets sinh(x) for a nonzero `Rational` x, small enough that its series converges in a few
// terms, between bounds from that series, tightening the bracket until both ends round the same
// way. This also covers inputs so small that their hyperbolic sines underflow, since everything is
// done in `Rational` arithmetic.
fn sinh_rational_series(x: &Rational, prec: u64, rm: RoundingMode) -> (Float, Ordering) {
    let mut w = prec + 10;
    let mut increment = Limb::WIDTH;
    loop {
        let toward_zero = sinh_bound(x, w, false);
        let away_from_zero = sinh_bound(x, w, true);
        let (lo, hi) = if *x > 0u32 {
            (toward_zero, away_from_zero)
        } else {
            (away_from_zero, toward_zero)
        };
        if let Some(result) = round_bracket(&lo, &hi, prec, rm) {
            return result;
        }
        w += increment;
        increment = w >> 1;
    }
}

// Computes sinh(x) for a nonzero `Rational` x, rounded to precision `prec` with rounding mode `rm`.
// sinh(x) is transcendental for every nonzero rational x, so the result is never exact.
pub(crate) fn sinh_rational_helper(x: &Rational, prec: u64, rm: RoundingMode) -> (Float, Ordering) {
    assert_ne!(rm, Exact, "Inexact sinh");
    let positive = *x > 0u32;
    let exp_x = x.floor_log_base_2_abs() + 1; // the MPFR-style exponent of x
    if exp_x < const { UNDERFLOW_EXPONENT - 1 } {
        // |x| < 2^(MIN_EXPONENT - 3), so |sinh(x)| < |x| (1 + x^2) is below 2^(MIN_EXPONENT - 2),
        // half the smallest positive Float, and the result is zero or that Float, by the rounding
        // mode alone, with no 2^30-bit arithmetic needed.
        return underflowed(positive, prec, rm);
    }
    // With |x| < 2^exp_x, the kth term of the series is below |x| 2^(2k exp_x), so when -exp_x is
    // at least a sixteenth of the working precision, about 8 terms suffice, which is cheaper than a
    // `Float` hyperbolic sine at that precision. This also covers every x too small to be a
    // `Float`.
    if exp_x < -1 && u64::exact_from(-exp_x) << 4 >= prec + 10 {
        return sinh_rational_series(x, prec, rm);
    }
    // |x| >= 2^(MAX_EXPONENT - 1), so |sinh(x)| > e^|x| / 4 overflows. Smaller x that still
    // overflow are caught by `sinh_prec_round_normal_ref` in the loop below.
    if exp_x >= Float::MAX_EXPONENT_I64 {
        return overflow(positive, prec, rm);
    }
    // sinh is increasing, so bracket x between the Floats x_lo <= x <= x_hi, take the hyperbolic
    // sine of both, and increase the working precision until the two round to the same result,
    // which the exact sinh(x), lying between them, must then share.
    let mut working_prec = prec + 10;
    let mut increment = Limb::WIDTH;
    loop {
        let (x_lo, x_o) = Float::from_rational_prec_round_ref(x, working_prec, Floor);
        if x_o == Equal {
            // x is exactly representable at `working_prec`, so sinh(x) is simply sinh(x_lo).
            return sinh_prec_round_normal_ref(&x_lo, prec, rm);
        }
        let (x_lo, x_hi) = floor_and_ceiling((x_lo, x_o));
        // The hyperbolic sine of a finite nonzero Float is never exact, so both orderings are
        // `Less` or `Greater`, never `Equal`. (x is far from zero here, so neither bound is zero.)
        let (s_lo, o_lo) = sinh_prec_round_normal_ref(&x_lo, prec, rm);
        let (s_hi, o_hi) = sinh_prec_round_normal_ref(&x_hi, prec, rm);
        if o_lo == o_hi && s_lo == s_hi {
            return (s_lo, o_lo);
        }
        working_prec += increment;
        increment = working_prec >> 1;
    }
}

impl Float {
    /// Computes $\sinh x$, the hyperbolic sine of a [`Float`], rounding the result to the specified
    /// precision and with the specified rounding mode. The [`Float`] is taken by value. An
    /// [`Ordering`] is also returned, indicating whether the rounded hyperbolic sine is less than,
    /// equal to, or greater than the exact hyperbolic sine. Although `NaN`s are not comparable to
    /// any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \sinh x+\varepsilon.
    /// $$
    /// - If $\sinh x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $\sinh x$ is finite, nonzero, and nonzero, and $m$ is not `Nearest`, then
    ///   $|\varepsilon| < 2^{\lfloor\log_2 \sinh x\rfloor-p+1}$.
    /// - If $\sinh x$ is finite, nonzero, and nonzero, and $m$ is `Nearest`, then $|\varepsilon|
    ///   \leq 2^{\lfloor\log_2 \sinh x\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=\text{NaN}$
    /// - $f(\infty,p,m)=\infty$
    /// - $f(-\infty,p,m)=-\infty$
    /// - $f(0.0,p,m)=0.0$
    /// - $f(-0.0,p,m)=-0.0$
    ///
    /// Overflow:
    /// - If $f(x,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$ is
    ///   returned instead.
    /// - If $f(x,p,m)\leq -2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is
    ///   returned instead.
    /// - If $f(x,p,m)\leq -2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`, $-(1-(1/2)^p)2^{2^{30}-1}$
    ///   is returned instead.
    ///
    /// Since $|\sinh x|\geq|x|$, the result never underflows.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::sinh_prec`] instead. If you
    /// know that your target precision is the precision of the input, consider using
    /// [`Float::sinh_round`] instead. If both of these things are true, consider using
    /// [`Float::sinh`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O((n+m)^{3/2} \log (n+m) \log\log (n+m))$
    ///
    /// $M(n, m) = O((n+m) \log (n+m))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `self.significant_bits()`: the exponential is computed at a working precision of at least
    /// the larger of `prec` and the input's precision.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` and `self` is finite and nonzero, since the hyperbolic sine of a
    /// finite nonzero [`Float`] is never exactly representable, or if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .sinh_prec_round(5, Floor);
    /// assert_eq!(c.to_string(), "1.12");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .sinh_prec_round(5, Ceiling);
    /// assert_eq!(c.to_string(), "1.19");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .sinh_prec_round(5, Nearest);
    /// assert_eq!(c.to_string(), "1.19");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .sinh_prec_round(20, Floor);
    /// assert_eq!(c.to_string(), "1.1751995");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .sinh_prec_round(20, Ceiling);
    /// assert_eq!(c.to_string(), "1.1752014");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .sinh_prec_round(20, Nearest);
    /// assert_eq!(c.to_string(), "1.1752014");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn sinh_prec_round(self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        self.sinh_prec_round_ref(prec, rm)
    }

    /// Computes $\sinh x$, the hyperbolic sine of a [`Float`], rounding the result to the specified
    /// precision and with the specified rounding mode. The [`Float`] is taken by reference. An
    /// [`Ordering`] is also returned, indicating whether the rounded hyperbolic cosine is less
    /// than, equal to, or greater than the exact hyperbolic sine. Although `NaN`s are not
    /// comparable to any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \sinh x+\varepsilon.
    /// $$
    /// - If $\sinh x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $\sinh x$ is finite, nonzero, and nonzero, and $m$ is not `Nearest`, then
    ///   $|\varepsilon| < 2^{\lfloor\log_2 \sinh x\rfloor-p+1}$.
    /// - If $\sinh x$ is finite, nonzero, and nonzero, and $m$ is `Nearest`, then $|\varepsilon|
    ///   \leq 2^{\lfloor\log_2 \sinh x\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=\text{NaN}$
    /// - $f(\infty,p,m)=\infty$
    /// - $f(-\infty,p,m)=-\infty$
    /// - $f(0.0,p,m)=0.0$
    /// - $f(-0.0,p,m)=-0.0$
    ///
    /// Overflow:
    /// - If $f(x,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$ is
    ///   returned instead.
    /// - If $f(x,p,m)\leq -2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is
    ///   returned instead.
    /// - If $f(x,p,m)\leq -2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`, $-(1-(1/2)^p)2^{2^{30}-1}$
    ///   is returned instead.
    ///
    /// Since $|\sinh x|\geq|x|$, the result never underflows.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::sinh_prec_ref`] instead. If
    /// you know that your target precision is the precision of the input, consider using
    /// [`Float::sinh_round_ref`] instead. If both of these things are true, consider using
    /// `(&Float).sinh()` instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O((n+m)^{3/2} \log (n+m) \log\log (n+m))$
    ///
    /// $M(n, m) = O((n+m) \log (n+m))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `self.significant_bits()`: the exponential is computed at a working precision of at least
    /// the larger of `prec` and the input's precision.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` and `self` is finite and nonzero, since the hyperbolic sine of a
    /// finite nonzero [`Float`] is never exactly representable, or if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .sinh_prec_round_ref(5, Floor);
    /// assert_eq!(c.to_string(), "1.12");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .sinh_prec_round_ref(5, Ceiling);
    /// assert_eq!(c.to_string(), "1.19");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .sinh_prec_round_ref(5, Nearest);
    /// assert_eq!(c.to_string(), "1.19");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .sinh_prec_round_ref(20, Floor);
    /// assert_eq!(c.to_string(), "1.1751995");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .sinh_prec_round_ref(20, Ceiling);
    /// assert_eq!(c.to_string(), "1.1752014");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .sinh_prec_round_ref(20, Nearest);
    /// assert_eq!(c.to_string(), "1.1752014");
    /// assert_eq!(o, Greater);
    /// ```
    pub fn sinh_prec_round_ref(&self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        assert_ne!(prec, 0);
        match &self.0 {
            NaN => (Self::NAN, Equal),
            // sinh(±inf) = ±inf, and sinh(±0) = ±0
            Infinity { .. } | Zero { .. } => (self.clone(), Equal),
            Finite { .. } => sinh_prec_round_normal_ref(self, prec, rm),
        }
    }

    /// Computes $\sinh x$, the hyperbolic sine of a [`Float`], rounding the result to the nearest
    /// value of the specified precision. The [`Float`] is taken by value. An [`Ordering`] is also
    /// returned, indicating whether the rounded hyperbolic sine is less than, equal to, or greater
    /// than the exact hyperbolic sine. Although `NaN`s are not comparable to any [`Float`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// If the hyperbolic sine is equidistant from two [`Float`]s with the specified precision, the
    /// [`Float`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a
    /// description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = \sinh x+\varepsilon.
    /// $$
    /// - If $\sinh x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $\sinh x$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2 |\sinh
    ///   x|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p)=\text{NaN}$
    /// - $f(\infty,p)=\infty$
    /// - $f(-\infty,p)=-\infty$
    /// - $f(0.0,p)=0.0$
    /// - $f(-0.0,p)=-0.0$
    ///
    /// Overflow:
    /// - If $f(x,p)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,p)\leq -2^{2^{30}-1}$, $-\infty$ is returned instead.
    ///
    /// Since $|\sinh x|\geq|x|$, the result never underflows.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::sinh_prec_round`] instead. If you know that your target precision is the precision
    /// of the input, consider using [`Float::sinh`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O((n+m)^{3/2} \log (n+m) \log\log (n+m))$
    ///
    /// $M(n, m) = O((n+m) \log (n+m))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `self.significant_bits()`: the exponential is computed at a working precision of at least
    /// the larger of `prec` and the input's precision.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100).0.sinh_prec(5);
    /// assert_eq!(c.to_string(), "1.19");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100).0.sinh_prec(20);
    /// assert_eq!(c.to_string(), "1.1752014");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn sinh_prec(self, prec: u64) -> (Self, Ordering) {
        self.sinh_prec_round(prec, Nearest)
    }

    /// Computes $\sinh x$, the hyperbolic sine of a [`Float`], rounding the result to the nearest
    /// value of the specified precision. The [`Float`] is taken by reference. An [`Ordering`] is
    /// also returned, indicating whether the rounded hyperbolic sine is less than, equal to, or
    /// greater than the exact hyperbolic sine. Although `NaN`s are not comparable to any [`Float`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// If the hyperbolic sine is equidistant from two [`Float`]s with the specified precision, the
    /// [`Float`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a
    /// description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = \sinh x+\varepsilon.
    /// $$
    /// - If $\sinh x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $\sinh x$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2 |\sinh
    ///   x|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p)=\text{NaN}$
    /// - $f(\infty,p)=\infty$
    /// - $f(-\infty,p)=-\infty$
    /// - $f(0.0,p)=0.0$
    /// - $f(-0.0,p)=-0.0$
    ///
    /// Overflow:
    /// - If $f(x,p)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,p)\leq -2^{2^{30}-1}$, $-\infty$ is returned instead.
    ///
    /// Since $|\sinh x|\geq|x|$, the result never underflows.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::sinh_prec_round_ref`] instead. If you know that your target precision is the
    /// precision of the input, consider using `(&Float).sinh()` instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O((n+m)^{3/2} \log (n+m) \log\log (n+m))$
    ///
    /// $M(n, m) = O((n+m) \log (n+m))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `self.significant_bits()`: the exponential is computed at a working precision of at least
    /// the larger of `prec` and the input's precision.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100).0.sinh_prec_ref(5);
    /// assert_eq!(c.to_string(), "1.19");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100).0.sinh_prec_ref(20);
    /// assert_eq!(c.to_string(), "1.1752014");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn sinh_prec_ref(&self, prec: u64) -> (Self, Ordering) {
        self.sinh_prec_round_ref(prec, Nearest)
    }

    /// Computes $\sinh x$, the hyperbolic sine of a [`Float`], rounding the result with the
    /// specified rounding mode. The [`Float`] is taken by value. An [`Ordering`] is also returned,
    /// indicating whether the rounded hyperbolic sine is less than, equal to, or greater than the
    /// exact hyperbolic sine. Although `NaN`s are not comparable to any [`Float`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the precision of the input. See [`RoundingMode`] for a
    /// description of the possible rounding modes.
    ///
    /// $$
    /// f(x,m) = \sinh x+\varepsilon.
    /// $$
    /// - If $\sinh x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $\sinh x$ is finite, nonzero, and nonzero, and $m$ is not `Nearest`, then
    ///   $|\varepsilon| < 2^{\lfloor\log_2 \sinh x\rfloor-p+1}$, where $p$ is the precision of the
    ///   input.
    /// - If $\sinh x$ is finite, nonzero, and nonzero, and $m$ is `Nearest`, then $|\varepsilon|
    ///   \leq 2^{\lfloor\log_2 \sinh x\rfloor-p}$, where $p$ is the precision of the input.
    ///
    /// If the output has a precision, it is the precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN},m)=\text{NaN}$
    /// - $f(\infty,m)=\infty$
    /// - $f(-\infty,m)=-\infty$
    /// - $f(0.0,m)=0.0$
    /// - $f(-0.0,m)=-0.0$
    ///
    /// See the [`Float::sinh_prec_round`] documentation for information on overflow.
    ///
    /// If you want to specify an output precision, consider using [`Float::sinh_prec_round`]
    /// instead. If you know you'll be using the `Nearest` rounding mode, consider using
    /// [`Float::sinh`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` and `self` is finite and nonzero, since the hyperbolic sine of a
    /// finite nonzero [`Float`] is never exactly representable.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100).0.sinh_round(Floor);
    /// assert_eq!(c.to_string(), "1.1752011936438014568823818505953");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100).0.sinh_round(Ceiling);
    /// assert_eq!(c.to_string(), "1.1752011936438014568823818505969");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100).0.sinh_round(Nearest);
    /// assert_eq!(c.to_string(), "1.1752011936438014568823818505953");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn sinh_round(self, rm: RoundingMode) -> (Self, Ordering) {
        let prec = self.significant_bits();
        self.sinh_prec_round(prec, rm)
    }

    /// Computes $\sinh x$, the hyperbolic sine of a [`Float`], rounding the result with the
    /// specified rounding mode. The [`Float`] is taken by reference. An [`Ordering`] is also
    /// returned, indicating whether the rounded hyperbolic sine is less than, equal to, or greater
    /// than the exact hyperbolic sine. Although `NaN`s are not comparable to any [`Float`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the precision of the input. See [`RoundingMode`] for a
    /// description of the possible rounding modes.
    ///
    /// $$
    /// f(x,m) = \sinh x+\varepsilon.
    /// $$
    /// - If $\sinh x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $\sinh x$ is finite, nonzero, and nonzero, and $m$ is not `Nearest`, then
    ///   $|\varepsilon| < 2^{\lfloor\log_2 \sinh x\rfloor-p+1}$, where $p$ is the precision of the
    ///   input.
    /// - If $\sinh x$ is finite, nonzero, and nonzero, and $m$ is `Nearest`, then $|\varepsilon|
    ///   \leq 2^{\lfloor\log_2 \sinh x\rfloor-p}$, where $p$ is the precision of the input.
    ///
    /// If the output has a precision, it is the precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN},m)=\text{NaN}$
    /// - $f(\infty,m)=\infty$
    /// - $f(-\infty,m)=-\infty$
    /// - $f(0.0,m)=0.0$
    /// - $f(-0.0,m)=-0.0$
    ///
    /// See the [`Float::sinh_prec_round`] documentation for information on overflow.
    ///
    /// If you want to specify an output precision, consider using [`Float::sinh_prec_round_ref`]
    /// instead. If you know you'll be using the `Nearest` rounding mode, consider using
    /// `(&Float).sinh()` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` and `self` is finite and nonzero, since the hyperbolic sine of a
    /// finite nonzero [`Float`] is never exactly representable.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100).0.sinh_round_ref(Floor);
    /// assert_eq!(c.to_string(), "1.1752011936438014568823818505953");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .sinh_round_ref(Ceiling);
    /// assert_eq!(c.to_string(), "1.1752011936438014568823818505969");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .sinh_round_ref(Nearest);
    /// assert_eq!(c.to_string(), "1.1752011936438014568823818505953");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn sinh_round_ref(&self, rm: RoundingMode) -> (Self, Ordering) {
        self.sinh_prec_round_ref(self.significant_bits(), rm)
    }

    /// Computes $\sinh x$, the hyperbolic sine of a [`Float`], in place, rounding the result to the
    /// specified precision and with the specified rounding mode. An [`Ordering`] is returned,
    /// indicating whether the rounded hyperbolic sine is less than, equal to, or greater than the
    /// exact hyperbolic sine. Although `NaN`s are not comparable to any [`Float`], whenever this
    /// function sets the [`Float`] to `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets \sinh x+\varepsilon.
    /// $$
    /// - If $\sinh x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $\sinh x$ is finite, nonzero, and nonzero, and $m$ is not `Nearest`, then
    ///   $|\varepsilon| < 2^{\lfloor\log_2 \sinh x\rfloor-p+1}$.
    /// - If $\sinh x$ is finite, nonzero, and nonzero, and $m$ is `Nearest`, then $|\varepsilon|
    ///   \leq 2^{\lfloor\log_2 \sinh x\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// See the [`Float::sinh_prec_round`] documentation for information on special cases and
    /// overflow.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::sinh_prec_assign`] instead.
    /// If you know that your target precision is the precision of the input, consider using
    /// [`Float::sinh_round_assign`] instead. If both of these things are true, consider using
    /// [`Float::sinh_assign`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O((n+m)^{3/2} \log (n+m) \log\log (n+m))$
    ///
    /// $M(n, m) = O((n+m) \log (n+m))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `self.significant_bits()`: the exponential is computed at a working precision of at least
    /// the larger of `prec` and the input's precision.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` and `self` is finite and nonzero, since the hyperbolic sine of a
    /// finite nonzero [`Float`] is never exactly representable, or if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.sinh_prec_round_assign(5, Floor), Less);
    /// assert_eq!(x.to_string(), "1.12");
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.sinh_prec_round_assign(5, Ceiling), Greater);
    /// assert_eq!(x.to_string(), "1.19");
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.sinh_prec_round_assign(5, Nearest), Greater);
    /// assert_eq!(x.to_string(), "1.19");
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.sinh_prec_round_assign(20, Floor), Less);
    /// assert_eq!(x.to_string(), "1.1751995");
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.sinh_prec_round_assign(20, Ceiling), Greater);
    /// assert_eq!(x.to_string(), "1.1752014");
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.sinh_prec_round_assign(20, Nearest), Greater);
    /// assert_eq!(x.to_string(), "1.1752014");
    /// ```
    #[inline]
    pub fn sinh_prec_round_assign(&mut self, prec: u64, rm: RoundingMode) -> Ordering {
        let o;
        (*self, o) = self.sinh_prec_round_ref(prec, rm);
        o
    }

    /// Computes $\sinh x$, the hyperbolic sine of a [`Float`], in place, rounding the result to the
    /// nearest value of the specified precision. An [`Ordering`] is returned, indicating whether
    /// the rounded hyperbolic sine is less than, equal to, or greater than the exact hyperbolic
    /// sine. Although `NaN`s are not comparable to any [`Float`], whenever this function sets the
    /// [`Float`] to `NaN` it also returns `Equal`.
    ///
    /// If the hyperbolic sine is equidistant from two [`Float`]s with the specified precision, the
    /// [`Float`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a
    /// description of the `Nearest` rounding mode.
    ///
    /// $$
    /// x \gets \sinh x+\varepsilon.
    /// $$
    /// - If $\sinh x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $\sinh x$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2 |\sinh
    ///   x|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// See the [`Float::sinh_prec`] documentation for information on special cases and overflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::sinh_prec_round_assign`] instead. If you know that your target precision is the
    /// precision of the input, consider using [`Float::sinh_assign`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O((n+m)^{3/2} \log (n+m) \log\log (n+m))$
    ///
    /// $M(n, m) = O((n+m) \log (n+m))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `self.significant_bits()`: the exponential is computed at a working precision of at least
    /// the larger of `prec` and the input's precision.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.sinh_prec_assign(5), Greater);
    /// assert_eq!(x.to_string(), "1.19");
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.sinh_prec_assign(20), Greater);
    /// assert_eq!(x.to_string(), "1.1752014");
    /// ```
    #[inline]
    pub fn sinh_prec_assign(&mut self, prec: u64) -> Ordering {
        self.sinh_prec_round_assign(prec, Nearest)
    }

    /// Computes $\sinh x$, the hyperbolic sine of a [`Float`], in place, rounding the result with
    /// the specified rounding mode. An [`Ordering`] is returned, indicating whether the rounded
    /// hyperbolic sine is less than, equal to, or greater than the exact hyperbolic sine. Although
    /// `NaN`s are not comparable to any [`Float`], whenever this function sets the [`Float`] to
    /// `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the precision of the input. See [`RoundingMode`] for a
    /// description of the possible rounding modes.
    ///
    /// $$
    /// x \gets \sinh x+\varepsilon.
    /// $$
    /// - If $\sinh x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $\sinh x$ is finite, nonzero, and nonzero, and $m$ is not `Nearest`, then
    ///   $|\varepsilon| < 2^{\lfloor\log_2 \sinh x\rfloor-p+1}$, where $p$ is the precision of the
    ///   input.
    /// - If $\sinh x$ is finite, nonzero, and nonzero, and $m$ is `Nearest`, then $|\varepsilon|
    ///   \leq 2^{\lfloor\log_2 \sinh x\rfloor-p}$, where $p$ is the precision of the input.
    ///
    /// If the output has a precision, it is the precision of the input.
    ///
    /// See the [`Float::sinh_round`] documentation for information on special cases and overflow.
    ///
    /// If you want to specify an output precision, consider using [`Float::sinh_prec_round_assign`]
    /// instead. If you know you'll be using the `Nearest` rounding mode, consider using
    /// [`Float::sinh_assign`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` and `self` is finite and nonzero, since the hyperbolic sine of a
    /// finite nonzero [`Float`] is never exactly representable.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.sinh_round_assign(Floor), Less);
    /// assert_eq!(x.to_string(), "1.1752011936438014568823818505953");
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.sinh_round_assign(Ceiling), Greater);
    /// assert_eq!(x.to_string(), "1.1752011936438014568823818505969");
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.sinh_round_assign(Nearest), Less);
    /// assert_eq!(x.to_string(), "1.1752011936438014568823818505953");
    /// ```
    #[inline]
    pub fn sinh_round_assign(&mut self, rm: RoundingMode) -> Ordering {
        let prec = self.significant_bits();
        self.sinh_prec_round_assign(prec, rm)
    }
}

impl Float {
    /// Computes $\sinh x$, the hyperbolic sine of a [`Rational`], rounding the result to the
    /// specified precision and with the specified rounding mode and returning the result as a
    /// [`Float`]. The [`Rational`] is taken by value. An [`Ordering`] is also returned, indicating
    /// whether the rounded hyperbolic sine is less than, equal to, or greater than the exact
    /// hyperbolic sine.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \sinh x+\varepsilon.
    /// $$
    /// - If $m$ is not `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2 |\sinh x|\rfloor-p+1}$.
    /// - If $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2 |\sinh x|\rfloor-p}$.
    ///
    /// These bounds do not apply when the result overflows or underflows; see below.
    ///
    /// The output has precision `prec`.
    ///
    /// Special cases:
    /// - $f(0,p,m)=0.0$.
    ///
    /// Overflow and underflow:
    /// - If $f(x,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$ is
    ///   returned instead.
    /// - If $f(x,p,m)\leq -2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is
    ///   returned instead.
    /// - If $f(x,p,m)\leq -2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`, $-(1-(1/2)^p)2^{2^{30}-1}$
    ///   is returned instead.
    /// - If $0<f(x,p,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,p,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,p,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,p,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,p,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,p,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,p,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,p,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is
    ///   returned instead.
    ///
    /// Underflow requires an input of magnitude below $2^{-2^{30}}$, too small to be a [`Float`].
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::sinh_rational_prec`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{3/2} \log n \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `x.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with the given precision (which is the case for every nonzero input).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = Float::sinh_rational_prec_round(Rational::from_unsigneds(3u8, 5), 5, Floor);
    /// assert_eq!(c.to_string(), "0.625");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::sinh_rational_prec_round(Rational::from_unsigneds(3u8, 5), 5, Ceiling);
    /// assert_eq!(c.to_string(), "0.656");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = Float::sinh_rational_prec_round(Rational::from_signeds(-3i8, 5), 20, Floor);
    /// assert_eq!(c.to_string(), "-0.63665390");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::sinh_rational_prec_round(Rational::from_signeds(-3i8, 5), 20, Ceiling);
    /// assert_eq!(c.to_string(), "-0.63665295");
    /// assert_eq!(o, Greater);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn sinh_rational_prec_round(x: Rational, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        Self::sinh_rational_prec_round_ref(&x, prec, rm)
    }

    /// Computes $\sinh x$, the hyperbolic sine of a [`Rational`], rounding the result to the
    /// specified precision and with the specified rounding mode and returning the result as a
    /// [`Float`]. The [`Rational`] is taken by reference. An [`Ordering`] is also returned,
    /// indicating whether the rounded hyperbolic sine is less than, equal to, or greater than the
    /// exact hyperbolic sine.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \sinh x+\varepsilon.
    /// $$
    /// - If $m$ is not `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2 |\sinh x|\rfloor-p+1}$.
    /// - If $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2 |\sinh x|\rfloor-p}$.
    ///
    /// These bounds do not apply when the result overflows or underflows; see below.
    ///
    /// The output has precision `prec`.
    ///
    /// Special cases:
    /// - $f(0,p,m)=0.0$.
    ///
    /// Overflow and underflow:
    /// - If $f(x,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$ is
    ///   returned instead.
    /// - If $f(x,p,m)\leq -2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is
    ///   returned instead.
    /// - If $f(x,p,m)\leq -2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`, $-(1-(1/2)^p)2^{2^{30}-1}$
    ///   is returned instead.
    /// - If $0<f(x,p,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,p,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,p,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,p,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,p,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,p,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,p,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,p,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is
    ///   returned instead.
    ///
    /// Underflow requires an input of magnitude below $2^{-2^{30}}$, too small to be a [`Float`].
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::sinh_rational_prec_ref`]
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{3/2} \log n \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `x.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with the given precision (which is the case for every nonzero input).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) =
    ///     Float::sinh_rational_prec_round_ref(&Rational::from_unsigneds(3u8, 5), 5, Floor);
    /// assert_eq!(c.to_string(), "0.625");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) =
    ///     Float::sinh_rational_prec_round_ref(&Rational::from_unsigneds(3u8, 5), 5, Ceiling);
    /// assert_eq!(c.to_string(), "0.656");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) =
    ///     Float::sinh_rational_prec_round_ref(&Rational::from_signeds(-3i8, 5), 20, Floor);
    /// assert_eq!(c.to_string(), "-0.63665390");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) =
    ///     Float::sinh_rational_prec_round_ref(&Rational::from_signeds(-3i8, 5), 20, Ceiling);
    /// assert_eq!(c.to_string(), "-0.63665295");
    /// assert_eq!(o, Greater);
    /// ```
    pub fn sinh_rational_prec_round_ref(
        x: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        assert_ne!(prec, 0);
        if *x == 0u32 {
            // sinh(0) = 0, exactly
            return (Self::ZERO, Equal);
        }
        sinh_rational_helper(x, prec, rm)
    }

    /// Computes $\sinh x$, the hyperbolic sine of a [`Rational`], rounding the result to the
    /// nearest value of the specified precision and returning the result as a [`Float`]. The
    /// [`Rational`] is taken by value. An [`Ordering`] is also returned, indicating whether the
    /// rounded hyperbolic sine is less than, equal to, or greater than the exact hyperbolic sine.
    ///
    /// If the hyperbolic sine is equidistant from two [`Float`]s with the specified precision, the
    /// [`Float`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a
    /// description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = \sinh x+\varepsilon,
    /// $$
    /// where $|\varepsilon| \leq 2^{\lfloor\log_2 |\sinh x|\rfloor-p}$ (unless the result overflows
    /// or underflows; see below).
    ///
    /// The output has precision `prec`.
    ///
    /// Special cases:
    /// - $f(0,p)=0.0$.
    ///
    /// Overflow and underflow:
    /// - If $f(x,p)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,p)\leq -2^{2^{30}-1}$, $-\infty$ is returned instead.
    /// - If $0<f(x,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::sinh_rational_prec_round`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{3/2} \log n \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `x.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = Float::sinh_rational_prec(Rational::from_unsigneds(3u8, 5), 5);
    /// assert_eq!(c.to_string(), "0.625");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::sinh_rational_prec(Rational::from_unsigneds(3u8, 5), 20);
    /// assert_eq!(c.to_string(), "0.63665390");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = Float::sinh_rational_prec(Rational::ZERO, 10);
    /// assert_eq!(c.to_string(), "0.0");
    /// assert_eq!(o, Equal);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn sinh_rational_prec(x: Rational, prec: u64) -> (Self, Ordering) {
        Self::sinh_rational_prec_round_ref(&x, prec, Nearest)
    }

    /// Computes $\sinh x$, the hyperbolic sine of a [`Rational`], rounding the result to the
    /// nearest value of the specified precision and returning the result as a [`Float`]. The
    /// [`Rational`] is taken by reference. An [`Ordering`] is also returned, indicating whether the
    /// rounded hyperbolic sine is less than, equal to, or greater than the exact hyperbolic sine.
    ///
    /// If the hyperbolic sine is equidistant from two [`Float`]s with the specified precision, the
    /// [`Float`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a
    /// description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = \sinh x+\varepsilon,
    /// $$
    /// where $|\varepsilon| \leq 2^{\lfloor\log_2 |\sinh x|\rfloor-p}$ (unless the result overflows
    /// or underflows; see below).
    ///
    /// The output has precision `prec`.
    ///
    /// Special cases:
    /// - $f(0,p)=0.0$.
    ///
    /// Overflow and underflow:
    /// - If $f(x,p)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,p)\leq -2^{2^{30}-1}$, $-\infty$ is returned instead.
    /// - If $0<f(x,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::sinh_rational_prec_round_ref`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{3/2} \log n \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `x.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = Float::sinh_rational_prec_ref(&Rational::from_unsigneds(3u8, 5), 5);
    /// assert_eq!(c.to_string(), "0.625");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::sinh_rational_prec_ref(&Rational::from_unsigneds(3u8, 5), 20);
    /// assert_eq!(c.to_string(), "0.63665390");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = Float::sinh_rational_prec_ref(&Rational::ZERO, 10);
    /// assert_eq!(c.to_string(), "0.0");
    /// assert_eq!(o, Equal);
    /// ```
    #[inline]
    pub fn sinh_rational_prec_ref(x: &Rational, prec: u64) -> (Self, Ordering) {
        Self::sinh_rational_prec_round_ref(x, prec, Nearest)
    }
}

impl Sinh for Float {
    type Output = Self;

    /// Computes $\sinh x$, the hyperbolic sine of a [`Float`], taking it by value.
    ///
    /// If the output has a precision, it is the precision of the input. If the hyperbolic sine is
    /// equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s in
    /// its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// $$
    /// f(x) = \sinh x+\varepsilon.
    /// $$
    /// - If $\sinh x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $\sinh x$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2 |\sinh
    ///   x|\rfloor-p}$, where $p$ is the precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN})=\text{NaN}$
    /// - $f(\infty)=\infty$
    /// - $f(-\infty)=-\infty$
    /// - $f(0.0)=0.0$
    /// - $f(-0.0)=-0.0$
    ///
    /// See the [`Float::sinh_round`] documentation for information on overflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::sinh_round`] instead. If you want to specify the output precision, consider using
    /// [`Float::sinh_prec`]. If you want both of these things, consider using
    /// [`Float::sinh_prec_round`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Sinh;
    /// use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity};
    /// use malachite_float::Float;
    ///
    /// assert!(Float::NAN.sinh().is_nan());
    /// assert_eq!(Float::INFINITY.sinh(), Float::INFINITY);
    /// assert_eq!(Float::NEGATIVE_INFINITY.sinh(), Float::NEGATIVE_INFINITY);
    /// assert_eq!(
    ///     Float::from_unsigned_prec(1u32, 100).0.sinh().to_string(),
    ///     "1.1752011936438014568823818505953"
    /// );
    /// ```
    #[inline]
    fn sinh(self) -> Self {
        let prec = self.significant_bits();
        self.sinh_prec_round(prec, Nearest).0
    }
}

impl Sinh for &Float {
    type Output = Float;

    /// Computes $\sinh x$, the hyperbolic sine of a [`Float`], taking it by reference.
    ///
    /// If the output has a precision, it is the precision of the input. If the hyperbolic sine is
    /// equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s in
    /// its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// $$
    /// f(x) = \sinh x+\varepsilon.
    /// $$
    /// - If $\sinh x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $\sinh x$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2 |\sinh
    ///   x|\rfloor-p}$, where $p$ is the precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN})=\text{NaN}$
    /// - $f(\infty)=\infty$
    /// - $f(-\infty)=-\infty$
    /// - $f(0.0)=0.0$
    /// - $f(-0.0)=-0.0$
    ///
    /// See the [`Float::sinh_round`] documentation for information on overflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::sinh_round_ref`] instead. If you want to specify the output precision, consider
    /// using [`Float::sinh_prec_ref`]. If you want both of these things, consider using
    /// [`Float::sinh_prec_round_ref`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Sinh;
    /// use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity};
    /// use malachite_float::Float;
    ///
    /// assert!((&Float::NAN).sinh().is_nan());
    /// assert_eq!((&Float::INFINITY).sinh(), Float::INFINITY);
    /// assert_eq!((&Float::NEGATIVE_INFINITY).sinh(), Float::NEGATIVE_INFINITY);
    /// assert_eq!(
    ///     (&Float::from_unsigned_prec(1u32, 100).0).sinh().to_string(),
    ///     "1.1752011936438014568823818505953"
    /// );
    /// ```
    #[inline]
    fn sinh(self) -> Float {
        self.sinh_prec_round_ref(self.significant_bits(), Nearest).0
    }
}

impl SinhAssign for Float {
    /// Computes $\sinh x$, the hyperbolic sine of a [`Float`], in place.
    ///
    /// If the output has a precision, it is the precision of the input. If the hyperbolic sine is
    /// equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s in
    /// its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// $$
    /// x \gets \sinh x+\varepsilon.
    /// $$
    /// - If $\sinh x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $\sinh x$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2 |\sinh
    ///   x|\rfloor-p}$, where $p$ is the precision of the input.
    ///
    /// See the [`Float::sinh`] documentation for information on special cases and overflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::sinh_round_assign`] instead. If you want to specify the output precision, consider
    /// using [`Float::sinh_prec_assign`]. If you want both of these things, consider using
    /// [`Float::sinh_prec_round_assign`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::SinhAssign;
    /// use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity};
    /// use malachite_float::Float;
    ///
    /// let mut x = Float::NAN;
    /// x.sinh_assign();
    /// assert!(x.is_nan());
    ///
    /// let mut x = Float::INFINITY;
    /// x.sinh_assign();
    /// assert_eq!(x, Float::INFINITY);
    ///
    /// let mut x = Float::NEGATIVE_INFINITY;
    /// x.sinh_assign();
    /// assert_eq!(x, Float::NEGATIVE_INFINITY);
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// x.sinh_assign();
    /// assert_eq!(x.to_string(), "1.1752011936438014568823818505953");
    /// ```
    #[inline]
    fn sinh_assign(&mut self) {
        let prec = self.significant_bits();
        self.sinh_prec_round_assign(prec, Nearest);
    }
}

/// Computes $\sinh x$, the hyperbolic sine of a primitive float. The result is correctly rounded.
///
/// $$
/// f(x) = \sinh x+\varepsilon.
/// $$
/// - If $\sinh x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
/// - If $\sinh x$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2 |\sinh
///   x|\rfloor-p}$, where $p$ is the precision of the output (typically 24 if `T` is a [`f32`] and
///   53 if `T` is a [`f64`], but less if the output is subnormal).
///
/// Special cases:
/// - $f(\text{NaN})=\text{NaN}$
/// - $f(\infty)=\infty$
/// - $f(-\infty)=-\infty$
/// - $f(0.0)=0.0$
/// - $f(-0.0)=-0.0$
///
/// Overflow is possible: a large positive `x` gives $\infty$, and a large negative `x` gives
/// $-\infty$. Since $|\sinh x|\geq|x|$, the result never underflows.
///
/// # Worst-case complexity
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use malachite_base::num::basic::traits::NegativeInfinity;
/// use malachite_base::num::float::NiceFloat;
/// use malachite_float::float::arithmetic::sinh::primitive_float_sinh;
///
/// assert!(primitive_float_sinh(f32::NAN).is_nan());
/// assert_eq!(
///     NiceFloat(primitive_float_sinh(f32::INFINITY)),
///     NiceFloat(f32::INFINITY)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_sinh(f32::NEGATIVE_INFINITY)),
///     NiceFloat(f32::NEGATIVE_INFINITY)
/// );
/// assert_eq!(NiceFloat(primitive_float_sinh(-0.0f32)), NiceFloat(-0.0));
/// assert_eq!(
///     NiceFloat(primitive_float_sinh(1.0f32)),
///     NiceFloat(1.1752012)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_sinh(-1.0f32)),
///     NiceFloat(-1.1752012)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_sinh(-100.0f32)),
///     NiceFloat(f32::NEGATIVE_INFINITY)
/// );
/// ```
#[inline]
#[allow(clippy::type_repetition_in_bounds)]
pub fn primitive_float_sinh<T: PrimitiveFloat>(x: T) -> T
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    emulate_float_to_float_fn(Float::sinh_prec, x)
}

/// Computes $\sinh x$, the hyperbolic sine of a [`Rational`], returning the result as a primitive
/// float. The result is correctly rounded.
///
/// $$
/// f(x) = \sinh x+\varepsilon.
/// $$
/// - If $\sinh x$ is infinite or zero, $\varepsilon$ may be ignored or assumed to be 0.
/// - If $\sinh x$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2 |\sinh
///   x|\rfloor-p}$, where $p$ is the precision of the output (typically 24 if `T` is a [`f32`] and
///   53 if `T` is a [`f64`], but less if the output is subnormal).
///
/// Special cases:
/// - $f(0)=0.0$
///
/// Overflow and underflow are possible: an `x` of large magnitude gives $\infty$ or $-\infty$, and
/// an `x` of small enough magnitude gives `0.0` or `-0.0`.
///
/// # Worst-case complexity
/// $T(m) = O(m (\log m)^2 \log\log m)$
///
/// $M(m) = O(m \log m)$
///
/// where $T$ is time, $M$ is additional memory, and $m$ is `x.significant_bits()`.
///
/// # Examples
/// ```
/// use malachite_base::num::basic::traits::{NegativeInfinity, Zero};
/// use malachite_base::num::float::NiceFloat;
/// use malachite_float::float::arithmetic::sinh::primitive_float_sinh_rational;
/// use malachite_q::Rational;
///
/// assert_eq!(
///     NiceFloat(primitive_float_sinh_rational::<f64>(&Rational::ZERO)),
///     NiceFloat(0.0)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_sinh_rational::<f64>(
///         &Rational::from_unsigneds(1u8, 3)
///     )),
///     NiceFloat(0.3395405572561501)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_sinh_rational::<f64>(&Rational::from(
///         -10000
///     ))),
///     NiceFloat(f64::NEGATIVE_INFINITY)
/// );
/// ```
#[inline]
#[allow(clippy::type_repetition_in_bounds)]
pub fn primitive_float_sinh_rational<T: PrimitiveFloat>(x: &Rational) -> T
where
    Float: PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    emulate_rational_to_float_fn(Float::sinh_rational_prec_ref, x)
}
