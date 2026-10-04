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
use crate::float::arithmetic::cosh::monotone_rational_via_floats;
use crate::float::arithmetic::round_near_x::{float_round_near_x, small_input_shortcut};
use crate::float::arithmetic::sin::{UNDERFLOW_EXPONENT, underflowed};
use crate::float::arithmetic::sinh::sinh_bound;
use crate::{Float, emulate_float_to_float_fn, emulate_rational_to_float_fn};
use core::cmp::Ordering::{self, Equal};
use core::cmp::{max, min};
use malachite_base::num::arithmetic::traits::{
    Abs, CeilingLogBase2, FloorLogBase2, PowerOf2, Square, Tanh, TanhAssign,
};
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::{
    NaN as NaNTrait, NegativeOne, One, Two, Zero as ZeroTrait,
};
use malachite_base::num::conversion::traits::{ExactFrom, RoundingFrom};
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_nz::natural::Natural;
use malachite_nz::natural::arithmetic::float::round::float_can_round;
use malachite_nz::platform::Limb;
use malachite_q::Rational;

// Computes tanh(x) for a finite nonzero x with |x| = `x_abs` so large that tanh(x) is close to ±1:
// MPFR's `set_one` label, which sets the result to ±1 or its neighbor toward zero. That is correct
// only when 1 - tanh(|x|) is below half an ulp of the output, which MPFR takes for granted; here it
// is checked, using 0 < 1 - tanh(|x|) = 2 / (exp(2|x|) + 1) < 2 exp(-2|x|) = 2^(1 - 2|x| log_2(e)),
// and when it fails (at a precision beyond about 2.9|x| bits), the result is computed from expm1.
fn tanh_near_one(x_abs: &Float, positive: bool, prec: u64, rm: RoundingMode) -> (Float, Ordering) {
    // f = floor(2|x|), saturating; an |x| of 2^62 or more gives far more bits than any precision
    let f = if x_abs.get_exponent().unwrap() > 62 {
        u64::MAX
    } else {
        u64::rounding_from(&(x_abs << 1u32), Floor).0
    };
    // err = f + 7 floor(f / 16) <= 2|x| (1 + 7/16) <= 2|x| log_2(e), since log_2(e) = 1.4426... >
    // 1.4375, so 1 - tanh(|x|) < 2^(1 - err)
    let err = f.saturating_add((f >> 4) * 7);
    let one = if positive {
        Float::ONE
    } else {
        Float::NEGATIVE_ONE
    };
    if err > prec + 1
        && let Some(result) = float_round_near_x(&one, min(err, prec + 2), false, prec, rm)
    {
        return result;
    }
    tanh_via_exp_x_minus_1(x_abs, positive, prec, rm)
}

// Computes tanh(x) for a finite nonzero x with |x| = `x_abs` as -expm1(-2|x|) / (2 + expm1(-2|x|)),
// signed. This form has no cancellation for a large |x|, where tanh(x) is close to ±1, and expm1
// handles arguments so negative that exp(-2|x|) underflows. With e = expm1(-2|x|) rounded to
// nearest, the numerator -e has a relative error below 2^-w, the denominator 2 + e, which exceeds 1
// > |e|, one below 2 * 2^-w, and the division adds another 2^-w: in all, below 4 ulps.
fn tanh_via_exp_x_minus_1(
    x_abs: &Float,
    positive: bool,
    prec: u64,
    rm: RoundingMode,
) -> (Float, Ordering) {
    let mut working_prec = prec + prec.ceiling_log_base_2() + 10;
    let mut increment = Limb::WIDTH;
    loop {
        let e = (-(x_abs << 1u32)).exp_x_minus_1_prec(working_prec).0;
        let denominator = e.add_prec_ref_val(Float::TWO, working_prec).0;
        let t = (-e).div_prec(denominator, working_prec).0;
        if float_can_round(t.significand_ref().unwrap(), working_prec - 3, prec, rm) {
            return Float::from_float_prec_round(if positive { t } else { -t }, prec, rm);
        }
        working_prec += increment;
        increment = working_prec >> 1;
    }
}

// This is mpfr_tanh from tanh.c, MPFR 4.2.2, where the input is finite and nonzero.
fn tanh_prec_round_normal_ref(xt: &Float, prec: u64, rm: RoundingMode) -> (Float, Ordering) {
    assert_ne!(rm, Exact, "Inexact tanh");
    let exp_xt = i64::from(xt.get_exponent().unwrap());
    // tanh(x) = x - x^3/3 + ... so the error is < 2^(3*EXP(x)-1)
    //
    // MPFR_FAST_COMPUTE_IF_SMALL_INPUT (y, xt, -2 * MPFR_GET_EXP (xt), 1, 0, rnd_mode, {});
    if let Some(result) = small_input_shortcut(xt, -(exp_xt << 1), 1, false, prec, rm) {
        return result;
    }
    let x = xt.abs();
    let positive = xt.is_sign_positive();
    // First check for BIG overflow of exp(2*x): For x > 0, exp(2*x) > 2^(2*x). If 2 ^(2*x) > 2^emax
    // or x>emax/2, there is an overflow
    if x >= const { Float::MAX_EXPONENT >> 1 } {
        return tanh_near_one(&x, positive, prec, rm);
    }
    // The optimal number of bits: see algorithms.tex
    let mut working_prec = prec + prec.ceiling_log_base_2() + 4;
    // if x is small, there will be a cancellation in exp(2x)-1
    if exp_xt < 0 {
        working_prec += u64::exact_from(-exp_xt);
    }
    // The error analysis in algorithms.tex assumes that 2x is exact. MPFR raises its working
    // precision to the precision of x to make it so, but in Malachite doubling a Float is always
    // exact, so a precise input does not force a precise exponential.
    let two_x = &x << 1u32;
    let mut increment = Limb::WIDTH;
    loop {
        // tanh(x) = (exp(2x)-1)/(exp(2x)+1); since x > 0, exp(2x) can only overflow
        let mut exp_2x = two_x.exp_prec_ref(working_prec).0;
        if exp_2x.is_infinite() {
            return tanh_near_one(&x, positive, prec, rm);
        }
        let exp_exp_2x = i64::from(exp_2x.get_exponent().unwrap());
        let denominator = exp_2x
            .add_prec_round_ref_val(Float::ONE, working_prec, Floor)
            .0;
        exp_2x.sub_prec_round_assign(Float::ONE, working_prec, Ceiling);
        // The subtraction cancels k = EXP(exp(2x)) - EXP(exp(2x) - 1) bits.
        let k = exp_exp_2x - i64::from(exp_2x.get_exponent().unwrap());
        let quotient = exp_2x / denominator;
        // Calculation of the error, see algorithms.tex: below 2^max(3, k + 1) ulps, provided that
        // max(3, k + 1) <= floor(p/2).
        let d = max(3, k + 1);
        let err = i64::exact_from(working_prec) - (d + 1);
        if d <= i64::exact_from(working_prec >> 1)
            && float_can_round(
                quotient.significand_ref().unwrap(),
                u64::exact_from(err),
                prec,
                rm,
            )
        {
            return Float::from_float_prec_round(
                if positive { quotient } else { -quotient },
                prec,
                rm,
            );
        }
        // if the quotient is 1, tanh(x) is close to 1, being below it
        if quotient.get_exponent() == Some(1) {
            return tanh_near_one(&x, positive, prec, rm);
        }
        working_prec += increment;
        increment = working_prec >> 1;
    }
}

// A bound for cosh(t), for a nonzero `Rational` t with |t| < 1/2, from the partial sum C_k of its
// series, 1 + t^2/2! + ... + t^(2k-2)/(2k-2)!, with k chosen from the bit length of t alone so that
// the first omitted term t^(2k)/(2k)! is below 2^-(w+4). Every term is positive, so C_k is a lower
// bound, and the remainder, less than twice the first omitted term, is below 2^-(w+3) <= C_k
// 2^-(w+3), so C_k (1 + 2^-(w+3)) is an upper bound. As in `sinh_bound`, the scaling is a
// multiplication and a shift, and for a tiny t, where one term suffices, t is not even squared.
pub(crate) fn cosh_bound(t: &Rational, w: u64, upper: bool) -> Rational {
    // |t| < 2^(log + 1), with log < 0
    let log = t.floor_log_base_2_abs();
    assert!(log < -1);
    // |t|^(2k) / (2k)! < 2^(2k (log + 1) - log_factorial), where log_factorial <= log2((2k)!)
    let mut k = 1u64;
    let mut log_factorial = 1u64; // floor(log2(2))
    let target = -i128::from(w) - 4;
    while i128::from(k << 1) * i128::from(log + 1) - i128::from(log_factorial) > target {
        k += 1;
        let two_k = k << 1;
        log_factorial += (two_k - 1).floor_log_base_2() + two_k.floor_log_base_2();
    }
    let mut c = Rational::ONE;
    if k > 1 {
        let t_squared = t.square();
        let mut term = Rational::ONE;
        for j in 1..k {
            term *= &t_squared;
            term /= Rational::from(((j << 1) - 1) * (j << 1));
            c += &term;
        }
    }
    if upper {
        let shift = w + 3;
        c *= Rational::from(Natural::power_of_2(shift) + Natural::ONE);
        c >>= shift;
    }
    c
}

// Brackets tanh(x) = sinh(x) / cosh(x) for a nonzero `Rational` x, small enough that the series of
// both converge in a few terms, by bounds on the two, tightening the bracket until both ends round
// the same way. This also covers inputs so small that their hyperbolic tangents underflow, since
// everything is done in `Rational` arithmetic.
fn tanh_rational_series(x: &Rational, prec: u64, rm: RoundingMode) -> (Float, Ordering) {
    let mut w = prec + 10;
    let mut increment = Limb::WIDTH;
    loop {
        // |tanh(x)| lies strictly between |sinh| bounded toward zero over cosh bounded above, and
        // |sinh| bounded away from zero over cosh bounded below.
        let toward_zero = sinh_bound(x, w, false) / cosh_bound(x, w, true);
        let away_from_zero = sinh_bound(x, w, true) / cosh_bound(x, w, false);
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

// Computes tanh(x) for a nonzero `Rational` x, rounded to precision `prec` with rounding mode `rm`.
// tanh(x) is transcendental for every nonzero rational x, so the result is never exact.
fn tanh_rational_helper(x: &Rational, prec: u64, rm: RoundingMode) -> (Float, Ordering) {
    assert_ne!(rm, Exact, "Inexact tanh");
    let positive = *x > 0u32;
    let exp_x = x.floor_log_base_2_abs() + 1; // the MPFR-style exponent of x
    if exp_x < UNDERFLOW_EXPONENT {
        // |tanh(x)| < |x| < 2^(MIN_EXPONENT - 2), half the smallest positive Float, so the result
        // is zero or that Float, by the rounding mode alone, with no 2^30-bit arithmetic needed.
        return underflowed(positive, prec, rm);
    }
    // As for `sinh`, a small x is handled by series. This also covers every remaining x too small
    // to be a `Float`.
    if exp_x < -1 && u64::exact_from(-exp_x) << 4 >= prec + 10 {
        return tanh_rational_series(x, prec, rm);
    }
    // |x| >= 2^(MAX_EXPONENT - 1), so 0 < 1 - |tanh(x)| < 2^(1 - 2|x|) is far below half an ulp of
    // 1 at any precision, and the result rounds from ±1.
    if exp_x >= Float::MAX_EXPONENT_I64 {
        let one = if positive {
            Float::ONE
        } else {
            Float::NEGATIVE_ONE
        };
        return float_round_near_x(&one, prec + 2, false, prec, rm).unwrap();
    }
    // tanh is increasing, so bracket x between the Floats x_lo <= x <= x_hi, take the hyperbolic
    // tangent of both, and increase the working precision until the two round to the same result,
    // which the exact tanh(x), lying between them, must then share.
    monotone_rational_via_floats(x, prec, rm, tanh_prec_round_normal_ref)
}

impl Float {
    /// Computes $\tanh x$, the hyperbolic tangent of a [`Float`], rounding the result to the
    /// specified precision and with the specified rounding mode. The [`Float`] is taken by value.
    /// An [`Ordering`] is also returned, indicating whether the rounded hyperbolic tangent is less
    /// than, equal to, or greater than the exact hyperbolic tangent. Although `NaN`s are not
    /// comparable to any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \tanh x+\varepsilon.
    /// $$
    /// - If $\tanh x$ is zero or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $\tanh x$ is finite, nonzero, and nonzero, and $m$ is not `Nearest`, then
    ///   $|\varepsilon| < 2^{\lfloor\log_2 \tanh x\rfloor-p+1}$.
    /// - If $\tanh x$ is finite, nonzero, and nonzero, and $m$ is `Nearest`, then $|\varepsilon|
    ///   \leq 2^{\lfloor\log_2 \tanh x\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=\text{NaN}$
    /// - $f(\infty,p,m)=1.0$
    /// - $f(-\infty,p,m)=-1.0$
    /// - $f(0.0,p,m)=0.0$
    /// - $f(-0.0,p,m)=-0.0$
    ///
    /// Overflow and underflow:
    /// - Since $|\tanh x|<1$, the result never overflows.
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
    /// Since $|\tanh x|<|x|$, underflow requires an input of magnitude $2^{-2^{30}}$, the smallest
    /// positive [`Float`], rounded toward zero.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::tanh_prec`] instead. If you
    /// know that your target precision is the precision of the input, consider using
    /// [`Float::tanh_round`] instead. If both of these things are true, consider using
    /// [`Float::tanh`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O((n+m)^{3/2} \log (n+m) \log\log (n+m))$
    ///
    /// $M(n, m) = O((n+m) \log (n+m))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `self.significant_bits()`: the exponential is computed at a working precision of `prec` plus
    /// the bits lost to cancellation for a small input, which is at most about half the input's
    /// precision when the small-input shortcut does not apply.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` and `self` is finite and nonzero, since the hyperbolic tangent of
    /// a finite nonzero [`Float`] is never exactly representable, or if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .tanh_prec_round(5, Floor);
    /// assert_eq!(c.to_string(), "0.750");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .tanh_prec_round(5, Ceiling);
    /// assert_eq!(c.to_string(), "0.781");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .tanh_prec_round(5, Nearest);
    /// assert_eq!(c.to_string(), "0.750");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .tanh_prec_round(20, Floor);
    /// assert_eq!(c.to_string(), "0.76159382");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .tanh_prec_round(20, Ceiling);
    /// assert_eq!(c.to_string(), "0.76159477");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .tanh_prec_round(20, Nearest);
    /// assert_eq!(c.to_string(), "0.76159382");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn tanh_prec_round(self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        self.tanh_prec_round_ref(prec, rm)
    }

    /// Computes $\tanh x$, the hyperbolic tangent of a [`Float`], rounding the result to the
    /// specified precision and with the specified rounding mode. The [`Float`] is taken by
    /// reference. An [`Ordering`] is also returned, indicating whether the rounded hyperbolic
    /// cosine is less than, equal to, or greater than the exact hyperbolic tangent. Although `NaN`s
    /// are not comparable to any [`Float`], whenever this function returns a `NaN` it also returns
    /// `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \tanh x+\varepsilon.
    /// $$
    /// - If $\tanh x$ is zero or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $\tanh x$ is finite, nonzero, and nonzero, and $m$ is not `Nearest`, then
    ///   $|\varepsilon| < 2^{\lfloor\log_2 \tanh x\rfloor-p+1}$.
    /// - If $\tanh x$ is finite, nonzero, and nonzero, and $m$ is `Nearest`, then $|\varepsilon|
    ///   \leq 2^{\lfloor\log_2 \tanh x\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=\text{NaN}$
    /// - $f(\infty,p,m)=1.0$
    /// - $f(-\infty,p,m)=-1.0$
    /// - $f(0.0,p,m)=0.0$
    /// - $f(-0.0,p,m)=-0.0$
    ///
    /// Overflow and underflow:
    /// - Since $|\tanh x|<1$, the result never overflows.
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
    /// Since $|\tanh x|<|x|$, underflow requires an input of magnitude $2^{-2^{30}}$, the smallest
    /// positive [`Float`], rounded toward zero.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::tanh_prec_ref`] instead. If
    /// you know that your target precision is the precision of the input, consider using
    /// [`Float::tanh_round_ref`] instead. If both of these things are true, consider using
    /// `(&Float).tanh()` instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O((n+m)^{3/2} \log (n+m) \log\log (n+m))$
    ///
    /// $M(n, m) = O((n+m) \log (n+m))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `self.significant_bits()`: the exponential is computed at a working precision of `prec` plus
    /// the bits lost to cancellation for a small input, which is at most about half the input's
    /// precision when the small-input shortcut does not apply.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` and `self` is finite and nonzero, since the hyperbolic tangent of
    /// a finite nonzero [`Float`] is never exactly representable, or if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .tanh_prec_round_ref(5, Floor);
    /// assert_eq!(c.to_string(), "0.750");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .tanh_prec_round_ref(5, Ceiling);
    /// assert_eq!(c.to_string(), "0.781");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .tanh_prec_round_ref(5, Nearest);
    /// assert_eq!(c.to_string(), "0.750");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .tanh_prec_round_ref(20, Floor);
    /// assert_eq!(c.to_string(), "0.76159382");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .tanh_prec_round_ref(20, Ceiling);
    /// assert_eq!(c.to_string(), "0.76159477");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .tanh_prec_round_ref(20, Nearest);
    /// assert_eq!(c.to_string(), "0.76159382");
    /// assert_eq!(o, Less);
    /// ```
    pub fn tanh_prec_round_ref(&self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        assert_ne!(prec, 0);
        match &self.0 {
            NaN => (Self::NAN, Equal),
            // tanh(inf) = 1 && tanh(-inf) = -1
            Infinity { sign } => (
                if *sign {
                    Self::one_prec(prec)
                } else {
                    -Self::one_prec(prec)
                },
                Equal,
            ),
            // tanh (0) = 0
            Zero { .. } => (self.clone(), Equal),
            Finite { .. } => tanh_prec_round_normal_ref(self, prec, rm),
        }
    }

    /// Computes $\tanh x$, the hyperbolic tangent of a [`Float`], rounding the result to the
    /// nearest value of the specified precision. The [`Float`] is taken by value. An [`Ordering`]
    /// is also returned, indicating whether the rounded hyperbolic tangent is less than, equal to,
    /// or greater than the exact hyperbolic tangent. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// If the hyperbolic tangent is equidistant from two [`Float`]s with the specified precision,
    /// the [`Float`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a
    /// description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = \tanh x+\varepsilon.
    /// $$
    /// - If $\tanh x$ is zero or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $\tanh x$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2 |\tanh
    ///   x|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p)=\text{NaN}$
    /// - $f(\infty,p)=1.0$
    /// - $f(-\infty,p)=-1.0$
    /// - $f(0.0,p)=0.0$
    /// - $f(-0.0,p)=-0.0$
    ///
    /// Overflow and underflow:
    /// - Since $|\tanh x|<1$, the result never overflows.
    /// - If $0<f(x,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::tanh_prec_round`] instead. If you know that your target precision is the precision
    /// of the input, consider using [`Float::tanh`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O((n+m)^{3/2} \log (n+m) \log\log (n+m))$
    ///
    /// $M(n, m) = O((n+m) \log (n+m))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `self.significant_bits()`: the exponential is computed at a working precision of `prec` plus
    /// the bits lost to cancellation for a small input, which is at most about half the input's
    /// precision when the small-input shortcut does not apply.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100).0.tanh_prec(5);
    /// assert_eq!(c.to_string(), "0.750");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100).0.tanh_prec(20);
    /// assert_eq!(c.to_string(), "0.76159382");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn tanh_prec(self, prec: u64) -> (Self, Ordering) {
        self.tanh_prec_round(prec, Nearest)
    }

    /// Computes $\tanh x$, the hyperbolic tangent of a [`Float`], rounding the result to the
    /// nearest value of the specified precision. The [`Float`] is taken by reference. An
    /// [`Ordering`] is also returned, indicating whether the rounded hyperbolic tangent is less
    /// than, equal to, or greater than the exact hyperbolic tangent. Although `NaN`s are not
    /// comparable to any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// If the hyperbolic tangent is equidistant from two [`Float`]s with the specified precision,
    /// the [`Float`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a
    /// description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = \tanh x+\varepsilon.
    /// $$
    /// - If $\tanh x$ is zero or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $\tanh x$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2 |\tanh
    ///   x|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p)=\text{NaN}$
    /// - $f(\infty,p)=1.0$
    /// - $f(-\infty,p)=-1.0$
    /// - $f(0.0,p)=0.0$
    /// - $f(-0.0,p)=-0.0$
    ///
    /// Overflow and underflow:
    /// - Since $|\tanh x|<1$, the result never overflows.
    /// - If $0<f(x,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::tanh_prec_round_ref`] instead. If you know that your target precision is the
    /// precision of the input, consider using `(&Float).tanh()` instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O((n+m)^{3/2} \log (n+m) \log\log (n+m))$
    ///
    /// $M(n, m) = O((n+m) \log (n+m))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `self.significant_bits()`: the exponential is computed at a working precision of `prec` plus
    /// the bits lost to cancellation for a small input, which is at most about half the input's
    /// precision when the small-input shortcut does not apply.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100).0.tanh_prec_ref(5);
    /// assert_eq!(c.to_string(), "0.750");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100).0.tanh_prec_ref(20);
    /// assert_eq!(c.to_string(), "0.76159382");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn tanh_prec_ref(&self, prec: u64) -> (Self, Ordering) {
        self.tanh_prec_round_ref(prec, Nearest)
    }

    /// Computes $\tanh x$, the hyperbolic tangent of a [`Float`], rounding the result with the
    /// specified rounding mode. The [`Float`] is taken by value. An [`Ordering`] is also returned,
    /// indicating whether the rounded hyperbolic tangent is less than, equal to, or greater than
    /// the exact hyperbolic tangent. Although `NaN`s are not comparable to any [`Float`], whenever
    /// this function returns a `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the precision of the input. See [`RoundingMode`] for a
    /// description of the possible rounding modes.
    ///
    /// $$
    /// f(x,m) = \tanh x+\varepsilon.
    /// $$
    /// - If $\tanh x$ is zero or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $\tanh x$ is finite, nonzero, and nonzero, and $m$ is not `Nearest`, then
    ///   $|\varepsilon| < 2^{\lfloor\log_2 \tanh x\rfloor-p+1}$, where $p$ is the precision of the
    ///   input.
    /// - If $\tanh x$ is finite, nonzero, and nonzero, and $m$ is `Nearest`, then $|\varepsilon|
    ///   \leq 2^{\lfloor\log_2 \tanh x\rfloor-p}$, where $p$ is the precision of the input.
    ///
    /// If the output has a precision, it is the precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN},m)=\text{NaN}$
    /// - $f(\infty,m)=1.0$
    /// - $f(-\infty,m)=-1.0$
    /// - $f(0.0,m)=0.0$
    /// - $f(-0.0,m)=-0.0$
    ///
    /// See the [`Float::tanh_prec_round`] documentation for information on overflow and underflow.
    ///
    /// If you want to specify an output precision, consider using [`Float::tanh_prec_round`]
    /// instead. If you know you'll be using the `Nearest` rounding mode, consider using
    /// [`Float::tanh`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` and `self` is finite and nonzero, since the hyperbolic tangent of
    /// a finite nonzero [`Float`] is never exactly representable.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100).0.tanh_round(Floor);
    /// assert_eq!(c.to_string(), "0.76159415595576488811945828260469");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100).0.tanh_round(Ceiling);
    /// assert_eq!(c.to_string(), "0.76159415595576488811945828260548");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100).0.tanh_round(Nearest);
    /// assert_eq!(c.to_string(), "0.76159415595576488811945828260469");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn tanh_round(self, rm: RoundingMode) -> (Self, Ordering) {
        let prec = self.significant_bits();
        self.tanh_prec_round(prec, rm)
    }

    /// Computes $\tanh x$, the hyperbolic tangent of a [`Float`], rounding the result with the
    /// specified rounding mode. The [`Float`] is taken by reference. An [`Ordering`] is also
    /// returned, indicating whether the rounded hyperbolic tangent is less than, equal to, or
    /// greater than the exact hyperbolic tangent. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the precision of the input. See [`RoundingMode`] for a
    /// description of the possible rounding modes.
    ///
    /// $$
    /// f(x,m) = \tanh x+\varepsilon.
    /// $$
    /// - If $\tanh x$ is zero or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $\tanh x$ is finite, nonzero, and nonzero, and $m$ is not `Nearest`, then
    ///   $|\varepsilon| < 2^{\lfloor\log_2 \tanh x\rfloor-p+1}$, where $p$ is the precision of the
    ///   input.
    /// - If $\tanh x$ is finite, nonzero, and nonzero, and $m$ is `Nearest`, then $|\varepsilon|
    ///   \leq 2^{\lfloor\log_2 \tanh x\rfloor-p}$, where $p$ is the precision of the input.
    ///
    /// If the output has a precision, it is the precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN},m)=\text{NaN}$
    /// - $f(\infty,m)=1.0$
    /// - $f(-\infty,m)=-1.0$
    /// - $f(0.0,m)=0.0$
    /// - $f(-0.0,m)=-0.0$
    ///
    /// See the [`Float::tanh_prec_round`] documentation for information on overflow and underflow.
    ///
    /// If you want to specify an output precision, consider using [`Float::tanh_prec_round_ref`]
    /// instead. If you know you'll be using the `Nearest` rounding mode, consider using
    /// `(&Float).tanh()` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` and `self` is finite and nonzero, since the hyperbolic tangent of
    /// a finite nonzero [`Float`] is never exactly representable.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100).0.tanh_round_ref(Floor);
    /// assert_eq!(c.to_string(), "0.76159415595576488811945828260469");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .tanh_round_ref(Ceiling);
    /// assert_eq!(c.to_string(), "0.76159415595576488811945828260548");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .tanh_round_ref(Nearest);
    /// assert_eq!(c.to_string(), "0.76159415595576488811945828260469");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn tanh_round_ref(&self, rm: RoundingMode) -> (Self, Ordering) {
        self.tanh_prec_round_ref(self.significant_bits(), rm)
    }

    /// Computes $\tanh x$, the hyperbolic tangent of a [`Float`], in place, rounding the result to
    /// the specified precision and with the specified rounding mode. An [`Ordering`] is returned,
    /// indicating whether the rounded hyperbolic tangent is less than, equal to, or greater than
    /// the exact hyperbolic tangent. Although `NaN`s are not comparable to any [`Float`], whenever
    /// this function sets the [`Float`] to `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets \tanh x+\varepsilon.
    /// $$
    /// - If $\tanh x$ is zero or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $\tanh x$ is finite, nonzero, and nonzero, and $m$ is not `Nearest`, then
    ///   $|\varepsilon| < 2^{\lfloor\log_2 \tanh x\rfloor-p+1}$.
    /// - If $\tanh x$ is finite, nonzero, and nonzero, and $m$ is `Nearest`, then $|\varepsilon|
    ///   \leq 2^{\lfloor\log_2 \tanh x\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// See the [`Float::tanh_prec_round`] documentation for information on special cases and
    /// overflow.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::tanh_prec_assign`] instead.
    /// If you know that your target precision is the precision of the input, consider using
    /// [`Float::tanh_round_assign`] instead. If both of these things are true, consider using
    /// [`Float::tanh_assign`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O((n+m)^{3/2} \log (n+m) \log\log (n+m))$
    ///
    /// $M(n, m) = O((n+m) \log (n+m))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `self.significant_bits()`: the exponential is computed at a working precision of `prec` plus
    /// the bits lost to cancellation for a small input, which is at most about half the input's
    /// precision when the small-input shortcut does not apply.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` and `self` is finite and nonzero, since the hyperbolic tangent of
    /// a finite nonzero [`Float`] is never exactly representable, or if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.tanh_prec_round_assign(5, Floor), Less);
    /// assert_eq!(x.to_string(), "0.750");
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.tanh_prec_round_assign(5, Ceiling), Greater);
    /// assert_eq!(x.to_string(), "0.781");
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.tanh_prec_round_assign(5, Nearest), Less);
    /// assert_eq!(x.to_string(), "0.750");
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.tanh_prec_round_assign(20, Floor), Less);
    /// assert_eq!(x.to_string(), "0.76159382");
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.tanh_prec_round_assign(20, Ceiling), Greater);
    /// assert_eq!(x.to_string(), "0.76159477");
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.tanh_prec_round_assign(20, Nearest), Less);
    /// assert_eq!(x.to_string(), "0.76159382");
    /// ```
    #[inline]
    pub fn tanh_prec_round_assign(&mut self, prec: u64, rm: RoundingMode) -> Ordering {
        let o;
        (*self, o) = self.tanh_prec_round_ref(prec, rm);
        o
    }

    /// Computes $\tanh x$, the hyperbolic tangent of a [`Float`], in place, rounding the result to
    /// the nearest value of the specified precision. An [`Ordering`] is returned, indicating
    /// whether the rounded hyperbolic tangent is less than, equal to, or greater than the exact
    /// hyperbolic sine. Although `NaN`s are not comparable to any [`Float`], whenever this function
    /// sets the [`Float`] to `NaN` it also returns `Equal`.
    ///
    /// If the hyperbolic tangent is equidistant from two [`Float`]s with the specified precision,
    /// the [`Float`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a
    /// description of the `Nearest` rounding mode.
    ///
    /// $$
    /// x \gets \tanh x+\varepsilon.
    /// $$
    /// - If $\tanh x$ is zero or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $\tanh x$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2 |\tanh
    ///   x|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// See the [`Float::tanh_prec`] documentation for information on special cases, overflow, and
    /// underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::tanh_prec_round_assign`] instead. If you know that your target precision is the
    /// precision of the input, consider using [`Float::tanh_assign`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O((n+m)^{3/2} \log (n+m) \log\log (n+m))$
    ///
    /// $M(n, m) = O((n+m) \log (n+m))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `self.significant_bits()`: the exponential is computed at a working precision of `prec` plus
    /// the bits lost to cancellation for a small input, which is at most about half the input's
    /// precision when the small-input shortcut does not apply.
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
    /// assert_eq!(x.tanh_prec_assign(5), Less);
    /// assert_eq!(x.to_string(), "0.750");
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.tanh_prec_assign(20), Less);
    /// assert_eq!(x.to_string(), "0.76159382");
    /// ```
    #[inline]
    pub fn tanh_prec_assign(&mut self, prec: u64) -> Ordering {
        self.tanh_prec_round_assign(prec, Nearest)
    }

    /// Computes $\tanh x$, the hyperbolic tangent of a [`Float`], in place, rounding the result
    /// with the specified rounding mode. An [`Ordering`] is returned, indicating whether the
    /// rounded hyperbolic tangent is less than, equal to, or greater than the exact hyperbolic
    /// tangent. Although `NaN`s are not comparable to any [`Float`], whenever this function sets
    /// the [`Float`] to `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the precision of the input. See [`RoundingMode`] for a
    /// description of the possible rounding modes.
    ///
    /// $$
    /// x \gets \tanh x+\varepsilon.
    /// $$
    /// - If $\tanh x$ is zero or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $\tanh x$ is finite, nonzero, and nonzero, and $m$ is not `Nearest`, then
    ///   $|\varepsilon| < 2^{\lfloor\log_2 \tanh x\rfloor-p+1}$, where $p$ is the precision of the
    ///   input.
    /// - If $\tanh x$ is finite, nonzero, and nonzero, and $m$ is `Nearest`, then $|\varepsilon|
    ///   \leq 2^{\lfloor\log_2 \tanh x\rfloor-p}$, where $p$ is the precision of the input.
    ///
    /// If the output has a precision, it is the precision of the input.
    ///
    /// See the [`Float::tanh_round`] documentation for information on special cases, overflow, and
    /// underflow.
    ///
    /// If you want to specify an output precision, consider using [`Float::tanh_prec_round_assign`]
    /// instead. If you know you'll be using the `Nearest` rounding mode, consider using
    /// [`Float::tanh_assign`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` and `self` is finite and nonzero, since the hyperbolic tangent of
    /// a finite nonzero [`Float`] is never exactly representable.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.tanh_round_assign(Floor), Less);
    /// assert_eq!(x.to_string(), "0.76159415595576488811945828260469");
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.tanh_round_assign(Ceiling), Greater);
    /// assert_eq!(x.to_string(), "0.76159415595576488811945828260548");
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.tanh_round_assign(Nearest), Less);
    /// assert_eq!(x.to_string(), "0.76159415595576488811945828260469");
    /// ```
    #[inline]
    pub fn tanh_round_assign(&mut self, rm: RoundingMode) -> Ordering {
        let prec = self.significant_bits();
        self.tanh_prec_round_assign(prec, rm)
    }
}

impl Float {
    /// Computes $\tanh x$, the hyperbolic tangent of a [`Rational`], rounding the result to the
    /// specified precision and with the specified rounding mode and returning the result as a
    /// [`Float`]. The [`Rational`] is taken by value. An [`Ordering`] is also returned, indicating
    /// whether the rounded hyperbolic tangent is less than, equal to, or greater than the exact
    /// hyperbolic tangent.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \tanh x+\varepsilon.
    /// $$
    /// - If $m$ is not `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2 |\tanh x|\rfloor-p+1}$.
    /// - If $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2 |\tanh x|\rfloor-p}$.
    ///
    /// These bounds do not apply when the result underflows; see below.
    ///
    /// The output has precision `prec`.
    ///
    /// Special cases:
    /// - $f(0,p,m)=0.0$.
    ///
    /// Overflow and underflow:
    /// - Since $|\tanh x|<1$, the result never overflows.
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
    /// Underflow requires an input of magnitude below about $2^{-2^{30}}$.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::tanh_rational_prec`] instead.
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
    /// let (t, o) = Float::tanh_rational_prec_round(Rational::from_unsigneds(3u8, 5), 5, Floor);
    /// assert_eq!(t.to_string(), "0.531");
    /// assert_eq!(o, Less);
    ///
    /// let (t, o) = Float::tanh_rational_prec_round(Rational::from_unsigneds(3u8, 5), 5, Ceiling);
    /// assert_eq!(t.to_string(), "0.562");
    /// assert_eq!(o, Greater);
    ///
    /// let (t, o) = Float::tanh_rational_prec_round(Rational::from_signeds(-3i8, 5), 20, Floor);
    /// assert_eq!(t.to_string(), "-0.53705025");
    /// assert_eq!(o, Less);
    ///
    /// let (t, o) = Float::tanh_rational_prec_round(Rational::from_signeds(-3i8, 5), 20, Ceiling);
    /// assert_eq!(t.to_string(), "-0.53704929");
    /// assert_eq!(o, Greater);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn tanh_rational_prec_round(x: Rational, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        Self::tanh_rational_prec_round_ref(&x, prec, rm)
    }

    /// Computes $\tanh x$, the hyperbolic tangent of a [`Rational`], rounding the result to the
    /// specified precision and with the specified rounding mode and returning the result as a
    /// [`Float`]. The [`Rational`] is taken by reference. An [`Ordering`] is also returned,
    /// indicating whether the rounded hyperbolic tangent is less than, equal to, or greater than
    /// the exact hyperbolic tangent.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \tanh x+\varepsilon.
    /// $$
    /// - If $m$ is not `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2 |\tanh x|\rfloor-p+1}$.
    /// - If $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2 |\tanh x|\rfloor-p}$.
    ///
    /// These bounds do not apply when the result underflows; see below.
    ///
    /// The output has precision `prec`.
    ///
    /// Special cases:
    /// - $f(0,p,m)=0.0$.
    ///
    /// Overflow and underflow:
    /// - Since $|\tanh x|<1$, the result never overflows.
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
    /// Underflow requires an input of magnitude below about $2^{-2^{30}}$.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::tanh_rational_prec_ref`]
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
    /// let (t, o) =
    ///     Float::tanh_rational_prec_round_ref(&Rational::from_unsigneds(3u8, 5), 5, Floor);
    /// assert_eq!(t.to_string(), "0.531");
    /// assert_eq!(o, Less);
    ///
    /// let (t, o) =
    ///     Float::tanh_rational_prec_round_ref(&Rational::from_unsigneds(3u8, 5), 5, Ceiling);
    /// assert_eq!(t.to_string(), "0.562");
    /// assert_eq!(o, Greater);
    ///
    /// let (t, o) =
    ///     Float::tanh_rational_prec_round_ref(&Rational::from_signeds(-3i8, 5), 20, Floor);
    /// assert_eq!(t.to_string(), "-0.53705025");
    /// assert_eq!(o, Less);
    ///
    /// let (t, o) =
    ///     Float::tanh_rational_prec_round_ref(&Rational::from_signeds(-3i8, 5), 20, Ceiling);
    /// assert_eq!(t.to_string(), "-0.53704929");
    /// assert_eq!(o, Greater);
    /// ```
    pub fn tanh_rational_prec_round_ref(
        x: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        assert_ne!(prec, 0);
        if *x == 0u32 {
            // tanh(0) = 0, exactly
            return (Self::ZERO, Equal);
        }
        tanh_rational_helper(x, prec, rm)
    }

    /// Computes $\tanh x$, the hyperbolic tangent of a [`Rational`], rounding the result to the
    /// nearest value of the specified precision and returning the result as a [`Float`]. The
    /// [`Rational`] is taken by value. An [`Ordering`] is also returned, indicating whether the
    /// rounded hyperbolic tangent is less than, equal to, or greater than the exact hyperbolic
    /// tangent.
    ///
    /// If the hyperbolic tangent is equidistant from two [`Float`]s with the specified precision,
    /// the [`Float`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a
    /// description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = \tanh x+\varepsilon,
    /// $$
    /// where $|\varepsilon| \leq 2^{\lfloor\log_2 |\tanh x|\rfloor-p}$ (unless the result
    /// underflows; see below).
    ///
    /// The output has precision `prec`.
    ///
    /// Special cases:
    /// - $f(0,p)=0.0$.
    ///
    /// Overflow and underflow:
    /// - Since $|\tanh x|<1$, the result never overflows.
    /// - If $0<f(x,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::tanh_rational_prec_round`] instead.
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
    /// let (t, o) = Float::tanh_rational_prec(Rational::from_unsigneds(3u8, 5), 20);
    /// assert_eq!(t.to_string(), "0.53704929");
    /// assert_eq!(o, Less);
    ///
    /// let (t, o) = Float::tanh_rational_prec(Rational::ZERO, 10);
    /// assert_eq!(t.to_string(), "0.0");
    /// assert_eq!(o, Equal);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn tanh_rational_prec(x: Rational, prec: u64) -> (Self, Ordering) {
        Self::tanh_rational_prec_round_ref(&x, prec, Nearest)
    }

    /// Computes $\tanh x$, the hyperbolic tangent of a [`Rational`], rounding the result to the
    /// nearest value of the specified precision and returning the result as a [`Float`]. The
    /// [`Rational`] is taken by reference. An [`Ordering`] is also returned, indicating whether the
    /// rounded hyperbolic tangent is less than, equal to, or greater than the exact hyperbolic
    /// tangent.
    ///
    /// If the hyperbolic tangent is equidistant from two [`Float`]s with the specified precision,
    /// the [`Float`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a
    /// description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = \tanh x+\varepsilon,
    /// $$
    /// where $|\varepsilon| \leq 2^{\lfloor\log_2 |\tanh x|\rfloor-p}$ (unless the result
    /// underflows; see below).
    ///
    /// The output has precision `prec`.
    ///
    /// Special cases:
    /// - $f(0,p)=0.0$.
    ///
    /// Overflow and underflow:
    /// - Since $|\tanh x|<1$, the result never overflows.
    /// - If $0<f(x,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::tanh_rational_prec_round_ref`] instead.
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
    /// let (t, o) = Float::tanh_rational_prec_ref(&Rational::from_unsigneds(3u8, 5), 20);
    /// assert_eq!(t.to_string(), "0.53704929");
    /// assert_eq!(o, Less);
    ///
    /// let (t, o) = Float::tanh_rational_prec_ref(&Rational::ZERO, 10);
    /// assert_eq!(t.to_string(), "0.0");
    /// assert_eq!(o, Equal);
    /// ```
    #[inline]
    pub fn tanh_rational_prec_ref(x: &Rational, prec: u64) -> (Self, Ordering) {
        Self::tanh_rational_prec_round_ref(x, prec, Nearest)
    }
}

impl Tanh for Float {
    type Output = Self;

    /// Computes $\tanh x$, the hyperbolic tangent of a [`Float`], taking it by value.
    ///
    /// If the output has a precision, it is the precision of the input. If the hyperbolic tangent
    /// is equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s
    /// in its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// $$
    /// f(x) = \tanh x+\varepsilon.
    /// $$
    /// - If $\tanh x$ is zero or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $\tanh x$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2 |\tanh
    ///   x|\rfloor-p}$, where $p$ is the precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN})=\text{NaN}$
    /// - $f(\infty)=1.0$
    /// - $f(-\infty)=-1.0$
    /// - $f(0.0)=0.0$
    /// - $f(-0.0)=-0.0$
    ///
    /// See the [`Float::tanh_round`] documentation for information on overflow and underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::tanh_round`] instead. If you want to specify the output precision, consider using
    /// [`Float::tanh_prec`]. If you want both of these things, consider using
    /// [`Float::tanh_prec_round`].
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
    /// use malachite_base::num::arithmetic::traits::Tanh;
    /// use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity};
    /// use malachite_float::Float;
    ///
    /// assert!(Float::NAN.tanh().is_nan());
    /// assert_eq!(Float::INFINITY.tanh(), 1);
    /// assert_eq!(Float::NEGATIVE_INFINITY.tanh(), -1);
    /// assert_eq!(
    ///     Float::from_unsigned_prec(1u32, 100).0.tanh().to_string(),
    ///     "0.76159415595576488811945828260469"
    /// );
    /// ```
    #[inline]
    fn tanh(self) -> Self {
        let prec = self.significant_bits();
        self.tanh_prec_round(prec, Nearest).0
    }
}

impl Tanh for &Float {
    type Output = Float;

    /// Computes $\tanh x$, the hyperbolic tangent of a [`Float`], taking it by reference.
    ///
    /// If the output has a precision, it is the precision of the input. If the hyperbolic tangent
    /// is equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s
    /// in its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// $$
    /// f(x) = \tanh x+\varepsilon.
    /// $$
    /// - If $\tanh x$ is zero or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $\tanh x$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2 |\tanh
    ///   x|\rfloor-p}$, where $p$ is the precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN})=\text{NaN}$
    /// - $f(\infty)=1.0$
    /// - $f(-\infty)=-1.0$
    /// - $f(0.0)=0.0$
    /// - $f(-0.0)=-0.0$
    ///
    /// See the [`Float::tanh_round`] documentation for information on overflow and underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::tanh_round_ref`] instead. If you want to specify the output precision, consider
    /// using [`Float::tanh_prec_ref`]. If you want both of these things, consider using
    /// [`Float::tanh_prec_round_ref`].
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
    /// use malachite_base::num::arithmetic::traits::Tanh;
    /// use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity};
    /// use malachite_float::Float;
    ///
    /// assert!((&Float::NAN).tanh().is_nan());
    /// assert_eq!((&Float::INFINITY).tanh(), 1);
    /// assert_eq!((&Float::NEGATIVE_INFINITY).tanh(), -1);
    /// assert_eq!(
    ///     (&Float::from_unsigned_prec(1u32, 100).0).tanh().to_string(),
    ///     "0.76159415595576488811945828260469"
    /// );
    /// ```
    #[inline]
    fn tanh(self) -> Float {
        self.tanh_prec_round_ref(self.significant_bits(), Nearest).0
    }
}

impl TanhAssign for Float {
    /// Computes $\tanh x$, the hyperbolic tangent of a [`Float`], in place.
    ///
    /// If the output has a precision, it is the precision of the input. If the hyperbolic tangent
    /// is equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s
    /// in its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// $$
    /// x \gets \tanh x+\varepsilon.
    /// $$
    /// - If $\tanh x$ is zero or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $\tanh x$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2 |\tanh
    ///   x|\rfloor-p}$, where $p$ is the precision of the input.
    ///
    /// See the [`Float::tanh`] documentation for information on special cases, overflow, and
    /// underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::tanh_round_assign`] instead. If you want to specify the output precision, consider
    /// using [`Float::tanh_prec_assign`]. If you want both of these things, consider using
    /// [`Float::tanh_prec_round_assign`].
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
    /// use malachite_base::num::arithmetic::traits::TanhAssign;
    /// use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity};
    /// use malachite_float::Float;
    ///
    /// let mut x = Float::NAN;
    /// x.tanh_assign();
    /// assert!(x.is_nan());
    ///
    /// let mut x = Float::INFINITY;
    /// x.tanh_assign();
    /// assert_eq!(x, 1);
    ///
    /// let mut x = Float::NEGATIVE_INFINITY;
    /// x.tanh_assign();
    /// assert_eq!(x, -1);
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// x.tanh_assign();
    /// assert_eq!(x.to_string(), "0.76159415595576488811945828260469");
    /// ```
    #[inline]
    fn tanh_assign(&mut self) {
        let prec = self.significant_bits();
        self.tanh_prec_round_assign(prec, Nearest);
    }
}

/// Computes $\tanh x$, the hyperbolic tangent of a primitive float. The result is correctly
/// rounded.
///
/// $$
/// f(x) = \tanh x+\varepsilon.
/// $$
/// - If $\tanh x$ is zero or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
/// - If $\tanh x$ is nonzero, then $|\varepsilon| < 2^{\lfloor\log_2 |\tanh x|\rfloor-p}$, where
///   $p$ is the precision of the output (typically 24 if `T` is a [`f32`] and 53 if `T` is a
///   [`f64`], but less if the output is subnormal).
///
/// Special cases:
/// - $f(\text{NaN})=\text{NaN}$
/// - $f(\infty)=1.0$
/// - $f(-\infty)=-1.0$
/// - $f(0.0)=0.0$
/// - $f(-0.0)=-0.0$
///
/// Neither overflow nor underflow is possible. The result is subnormal only when $x$ is, and then
/// it is $x$ itself.
///
/// # Worst-case complexity
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use malachite_base::num::basic::traits::NegativeInfinity;
/// use malachite_base::num::float::NiceFloat;
/// use malachite_float::float::arithmetic::tanh::primitive_float_tanh;
///
/// assert!(primitive_float_tanh(f32::NAN).is_nan());
/// assert_eq!(
///     NiceFloat(primitive_float_tanh(f32::INFINITY)),
///     NiceFloat(1.0)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_tanh(f32::NEGATIVE_INFINITY)),
///     NiceFloat(-1.0)
/// );
/// assert_eq!(NiceFloat(primitive_float_tanh(-0.0f32)), NiceFloat(-0.0));
/// assert_eq!(
///     NiceFloat(primitive_float_tanh(1.0f32)),
///     NiceFloat(0.7615942)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_tanh(-1.0f64)),
///     NiceFloat(-0.7615941559557649)
/// );
/// assert_eq!(NiceFloat(primitive_float_tanh(20.0f64)), NiceFloat(1.0));
/// ```
#[inline]
#[allow(clippy::type_repetition_in_bounds)]
pub fn primitive_float_tanh<T: PrimitiveFloat>(x: T) -> T
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    emulate_float_to_float_fn(Float::tanh_prec, x)
}

/// Computes $\tanh x$, the hyperbolic tangent of a [`Rational`], returning the result as a
/// primitive float. The result is correctly rounded.
///
/// $$
/// f(x) = \tanh x+\varepsilon.
/// $$
/// - If $\tanh x$ is zero, $\varepsilon$ may be ignored or assumed to be 0.
/// - If $\tanh x$ is nonzero, then $|\varepsilon| < 2^{\lfloor\log_2 |\tanh x|\rfloor-p}$, where
///   $p$ is the precision of the output (typically 24 if `T` is a [`f32`] and 53 if `T` is a
///   [`f64`], but less if the output is subnormal).
///
/// Special cases:
/// - $f(0)=0.0$
///
/// Overflow is not possible, since the result lies in $(-1, 1)$. An `x` of small enough magnitude
/// underflows to `0.0` or `-0.0`.
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
/// use malachite_base::num::basic::traits::Zero;
/// use malachite_base::num::float::NiceFloat;
/// use malachite_float::float::arithmetic::tanh::primitive_float_tanh_rational;
/// use malachite_q::Rational;
///
/// assert_eq!(
///     NiceFloat(primitive_float_tanh_rational::<f64>(&Rational::ZERO)),
///     NiceFloat(0.0)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_tanh_rational::<f64>(
///         &Rational::from_unsigneds(1u8, 3)
///     )),
///     NiceFloat(0.32151273753163434)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_tanh_rational::<f64>(&Rational::from(
///         -10000
///     ))),
///     NiceFloat(-1.0)
/// );
/// ```
#[inline]
#[allow(clippy::type_repetition_in_bounds)]
pub fn primitive_float_tanh_rational<T: PrimitiveFloat>(x: &Rational) -> T
where
    Float: PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    emulate_rational_to_float_fn(Float::tanh_rational_prec_ref, x)
}
