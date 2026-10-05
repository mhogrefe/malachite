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
use crate::float::arithmetic::atan::alternating_odd_series;
use crate::float::arithmetic::cosh::{monotone_rational_via_floats, same_rounding};
use crate::float::arithmetic::round_near_x::{
    LEADING_TERM_MIN_EXPONENT, round_rational_leading_term, small_input_shortcut,
};
use crate::float::arithmetic::sin::{UNDERFLOW_EXPONENT, underflowed};
use crate::{Float, emulate_float_to_float_fn, emulate_rational_to_float_fn};
use core::cmp::Ordering::{self, Equal};
use core::cmp::max;
use malachite_base::fail_on_untested_path;
use malachite_base::num::arithmetic::traits::{Abs, Asinh, AsinhAssign, CeilingLogBase2};
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::{NaN as NaNTrait, One, Zero as ZeroTrait};
use malachite_base::num::conversion::traits::{ExactFrom, RoundingFrom};
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_nz::natural::arithmetic::float::round::float_can_round;
use malachite_nz::platform::Limb;
use malachite_q::Rational;

// The largest working precision at which `ln_of_large_sum` may use ln 2 in place of ln(1 + sqrt(1
// ± (1/x)^2)). The two differ by less than (1/x)^2 / 2 < 2^(-2^30), and the result's exponent is
// at least 29, so the difference stays below half an ulp of the result as long as the working
// precision is at most 2^30 + 28.
const LN_2_SHORTCUT_MAX_PREC: u64 = (1 << 30) + 28;

// asinh(|x|) = ln(sqrt(x^2 + 1) + |x|), evaluated at a working precision of `wp`. x^2 must not
// overflow.
fn asinh_abs_general(x_abs: &Float, wp: u64) -> Float {
    x_abs
        // x^2
        .square_prec_round_ref(wp, Floor)
        .0
        // x^2 + 1
        .add_prec_round(Float::ONE, wp, Floor)
        .0
        // sqrt(x^2 + 1)
        .sqrt_prec_round(wp, Nearest)
        .0
        // sqrt(x^2 + 1) + |x|
        .add_prec_round_val_ref(x_abs, wp, Nearest)
        .0
        // ln(sqrt(x^2 + 1) + |x|)
        .ln_prec_round(wp, Nearest)
        .0
}

// ln(x) + ln(1 + sqrt(1 + (1/x)^2)) if `plus`, which is asinh(x), or ln(x) + ln(1 + sqrt(1 -
// (1/x)^2)) otherwise, which is acosh(x), evaluated at a working precision of `wp` for a positive x
// whose square would overflow. Both identities are exact and cannot overflow, and at any practical
// precision the second logarithm is ln 2 to within half an ulp of the result. The error is at most
// half an ulp each from ln(x), ln 2, the addition, and the replacement of the second logarithm by
// ln 2, so below 2 ulps of the result.
pub(crate) fn ln_of_large_sum(x: &Float, wp: u64, plus: bool) -> Float {
    let ln_x = x.ln_prec_round_ref(wp, Nearest).0;
    let correction = if wp <= LN_2_SHORTCUT_MAX_PREC {
        // ln 2 is needed only to the result's ulp, and the result has the exponent of ln(x) or one
        // more, so wp - EXP(ln(x)) bits suffice, as in MPFR's overflow branch
        let exp_ln_x = u64::from(ln_x.get_exponent().unwrap().unsigned_abs());
        Float::ln_2_prec(wp.saturating_sub(exp_ln_x).max(1)).0
    } else {
        fail_on_untested_path("ln_of_large_sum, full correction");
        let reciprocal_squared = x
            .reciprocal_prec_round_ref(wp, Floor)
            .0
            .square_prec_round(wp, Floor)
            .0;
        let one = Float::ONE;
        let inner = if plus {
            one.add_prec_round(reciprocal_squared, wp, Floor).0
        } else {
            one.sub_prec_round(reciprocal_squared, wp, Floor).0
        };
        inner
            .sqrt_prec_round(wp, Nearest)
            .0
            .add_prec_round(Float::ONE, wp, Nearest)
            .0
            .ln_prec_round(wp, Nearest)
            .0
    };
    ln_x.add_prec_round(correction, wp, Nearest).0
}

// Whether x^2 can overflow: x < 2^EXP(x), so x^2 < 2^(2 EXP(x)), which is in range as long as
// EXP(x) is at most MAX_EXPONENT / 2.
pub(crate) const fn square_may_overflow(x: &Float) -> bool {
    x.get_exponent().unwrap() > Float::MAX_EXPONENT >> 1
}

// The end of a Ziv loop iteration: rounds an approximation t, whose error is below 2^(EXP(t) - wp +
// err), to precision `prec` with rounding mode `rm`, if that error allows it. Like MPFR's, `err` is
// signed, and a nonpositive wp - err means that t cannot be rounded yet. The test does not depend
// on the sign of t, so a caller may negate t first.
pub(crate) fn round_with_error(
    t: Float,
    wp: u64,
    err: i64,
    prec: u64,
    rm: RoundingMode,
) -> Option<(Float, Ordering)> {
    let bits = i64::exact_from(wp) - err;
    (bits > 0
        && float_can_round(
            t.significand_ref().unwrap(),
            u64::exact_from(bits),
            prec,
            rm,
        ))
    .then(|| Float::from_float_prec_round(t, prec, rm))
}

// This is mpfr_asinh from asinh.c, MPFR 4.2.2, where the input is finite and nonzero.
//
// MPFR computes x^2 in an extended exponent range, so it never overflows. Here it overflows once
// EXP(x) exceeds MAX_EXPONENT / 2, and with `Floor` it would saturate to the largest finite `Float`
// and silently give a wrong result, so those inputs go through `ln_of_large_sum`.
fn asinh_prec_round_normal_ref(x: &Float, prec: u64, rm: RoundingMode) -> (Float, Ordering) {
    assert_ne!(rm, Exact, "Inexact asinh");
    let exp_x = i64::from(x.get_exponent().unwrap());
    // asinh(x) = x - x^3/6 + ... so the error is < 2^(3*EXP(x)-2)
    if let Some(result) = small_input_shortcut(x, -(exp_x << 1), 2, false, prec, rm) {
        return result;
    }
    let negative = *x < 0u32;
    let x_abs = x.abs();
    let large = square_may_overflow(x);
    // the optimal number of bits: see algorithms.tex
    let mut working_prec = prec + 4 + prec.ceiling_log_base_2();
    let mut increment = Limb::WIDTH;
    loop {
        let t = if large {
            ln_of_large_sum(&x_abs, working_prec, true)
        } else {
            asinh_abs_general(&x_abs, working_prec)
        };
        if t.is_normal() {
            // error estimate: see algorithms.tex. In the large case the error is below 2 ulps of t,
            // as `ln_of_large_sum` explains, which an err of 2 covers with room to spare.
            let err = if large {
                2
            } else {
                max(4 - i64::from(t.get_exponent().unwrap()), 0) + 1
            };
            if let Some(result) =
                round_with_error(if negative { -t } else { t }, working_prec, err, prec, rm)
            {
                return result;
            }
        }
        working_prec += increment;
        increment = working_prec >> 1;
    }
}

// Computes asinh(x) for a nonzero `Rational` x with |x| < 1/2 from the series x - x^3/6 + 3 x^5/40
// - ..., whose kth term is c_k x^(2k+1) / (2k+1) with c_k = (2k)! / (4^k (k!)^2) = c_(k-1) (2k - 1)
// / (2k). The terms alternate in sign and decrease in magnitude.
fn asinh_series(x: &Rational, prec: u64, rm: RoundingMode) -> (Float, Ordering) {
    alternating_odd_series(x, prec, rm, |k| {
        Some(Rational::from_unsigneds((k << 1) - 1, k << 1))
    })
}

// Computes ln(2x) + c for a positive `Rational` x too large to be a `Float`, where c is unknown but
// positive if `plus` is true and negative otherwise, and |c| < 2^(2 - 2 EXP(x)). The result exceeds
// 2^29, so |c| is below an ulp of the result at any working precision below 2 EXP(x), more than
// 2^31 bits. ln(2x) rounded down and rounded up, with the bound on c's side moved one more ulp,
// therefore bracket ln(2x) + c.
pub(crate) fn ln_of_large_rational_sum(
    x: &Rational,
    exp_x: i64,
    prec: u64,
    rm: RoundingMode,
    plus: bool,
) -> (Float, Ordering) {
    let two_x = x << 1u32;
    let mut working_prec = prec + 10;
    let mut increment = Limb::WIDTH;
    loop {
        assert!(
            working_prec < u64::exact_from(exp_x) << 1,
            "ln_of_large_rational_sum needs a working precision below 2 EXP(x)"
        );
        let mut lo = Float::ln_rational_prec_round_ref(&two_x, working_prec, Floor).0;
        let mut hi = Float::ln_rational_prec_round_ref(&two_x, working_prec, Ceiling).0;
        if plus {
            hi.increment();
        } else {
            lo.decrement();
        }
        if let Some(result) = same_rounding(
            Float::from_float_prec_round(lo, prec, rm),
            Float::from_float_prec_round(hi, prec, rm),
        ) {
            return result;
        }
        fail_on_untested_path("ln_of_large_rational_sum, retry");
        working_prec += increment;
        increment = working_prec >> 1;
    }
}

// Computes asinh(x) for a `Rational` x too large to be a `Float`. asinh(|x|) = ln(2|x|) + c with 0
// < c < 1/(4x^2) < 2^(-2 EXP(x)).
fn asinh_rational_huge(x: &Rational, exp_x: i64, prec: u64, rm: RoundingMode) -> (Float, Ordering) {
    if *x > 0u32 {
        ln_of_large_rational_sum(x, exp_x, prec, rm, true)
    } else {
        let (y, o) = ln_of_large_rational_sum(&-x, exp_x, prec, -rm, true);
        (-y, o.reverse())
    }
}

// Computes asinh(x) for a nonzero `Rational` x, rounded to precision `prec` with rounding mode
// `rm`. (x = 0 is handled by the caller.) The result is never exactly representable, so `rm` must
// not be `Exact`.
fn asinh_rational_helper(x: &Rational, prec: u64, rm: RoundingMode) -> (Float, Ordering) {
    assert_ne!(rm, Exact, "Inexact asinh");
    let exp_x = x.floor_log_base_2_abs() + 1; // the MPFR-style exponent of x
    if exp_x < UNDERFLOW_EXPONENT {
        // |asinh x| < |x| < 2^(MIN_EXPONENT - 2), below half the smallest positive `Float`, so the
        // result is zero or that `Float` by the rounding mode alone
        return underflowed(*x > 0u32, prec, rm);
    }
    // asinh(x) = x(1 - x^2/6 + ...), so |x| exceeds |asinh x| by less than 2^(3 EXP(x) - 2), and
    // once that is below the distance from x to the nearest (prec + 1)-bit dyadic other than x
    // itself, x's own rounding, nudged toward zero, is the answer; as for `atan_rational`.
    if exp_x > LEADING_TERM_MIN_EXPONENT
        && -(exp_x << 1) > i64::exact_from(prec + x.denominator_ref().significant_bits()) + 4
    {
        return round_rational_leading_term(x.abs(), *x > 0u32, false, prec, rm);
    }
    // For |x| <= 1/2 the partial sums of the series bracket asinh x within a relative width below
    // x^4, which decides the rounding once x^4 is below 2^-(prec + 3); a handful of terms is
    // cheaper than a `Float` inverse hyperbolic sine at the working precision.
    if exp_x < 0 && -(exp_x << 2) > i64::exact_from(prec) + 3 {
        return asinh_series(x, prec, rm);
    }
    if exp_x > Float::MAX_EXPONENT_I64 {
        return asinh_rational_huge(x, exp_x, prec, rm);
    }
    // asinh is increasing, so bracket x between the Floats x_lo <= x <= x_hi, take the inverse
    // hyperbolic sine of both, and increase the working precision until the two round to the same
    // result, which the exact asinh(x), lying between them, must then share.
    monotone_rational_via_floats(x, prec, rm, asinh_prec_round_normal_ref)
}

impl Float {
    /// Computes $\operatorname{asinh} x$, the inverse hyperbolic sine of a [`Float`], rounding the
    /// result to the specified precision and with the specified rounding mode. The [`Float`] is
    /// taken by value. An [`Ordering`] is also returned, indicating whether the rounded inverse
    /// hyperbolic sine is less than, equal to, or greater than the exact inverse hyperbolic sine.
    /// Although `NaN`s are not comparable to any [`Float`], whenever this function returns a `NaN`
    /// it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \operatorname{asinh} x+\varepsilon.
    /// $$
    /// - If $x$ is infinite, zero, or NaN, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\operatorname{asinh} x|\rfloor-p+1}$.
    /// - If $x$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\operatorname{asinh} x|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=\text{NaN}$
    /// - $f(\pm\infty,p,m)=\pm\infty$
    /// - $f(\pm0.0,p,m)=\pm0.0$
    ///
    /// Overflow and underflow:
    /// - Since $|\operatorname{asinh} x| < |x|$ for nonzero $x$, the result never overflows.
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
    /// Underflow requires an input of magnitude $2^{-2^{30}}$, the smallest positive [`Float`],
    /// rounded toward zero: since $|\operatorname{asinh} x| < |x|$ for nonzero $x$, no other input
    /// can reach it.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::asinh_prec`] instead. If you
    /// know that your target precision is the precision of the input, consider using
    /// [`Float::asinh_round`] instead. If both of these things are true, consider using
    /// [`Float::asinh`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^2 \log\log n + m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `self.significant_bits()`: the logarithm is computed at a working precision of about $n$,
    /// and the input is first squared at its own precision.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` and `self` is finite and nonzero, since the inverse hyperbolic
    /// sine of a finite nonzero [`Float`] is never exactly representable, or if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .asinh_prec_round(5, Floor);
    /// assert_eq!(c.to_string(), "0.875");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .asinh_prec_round(5, Ceiling);
    /// assert_eq!(c.to_string(), "0.906");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .asinh_prec_round(5, Nearest);
    /// assert_eq!(c.to_string(), "0.875");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .asinh_prec_round(20, Floor);
    /// assert_eq!(c.to_string(), "0.88137341");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .asinh_prec_round(20, Ceiling);
    /// assert_eq!(c.to_string(), "0.88137436");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .asinh_prec_round(20, Nearest);
    /// assert_eq!(c.to_string(), "0.88137341");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn asinh_prec_round(self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        self.asinh_prec_round_ref(prec, rm)
    }

    /// Computes $\operatorname{asinh} x$, the inverse hyperbolic sine of a [`Float`], rounding the
    /// result to the specified precision and with the specified rounding mode. The [`Float`] is
    /// taken by reference. An [`Ordering`] is also returned, indicating whether the rounded inverse
    /// hyperbolic sine is less than, equal to, or greater than the exact inverse hyperbolic sine.
    /// Although `NaN`s are not comparable to any [`Float`], whenever this function returns a `NaN`
    /// it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \operatorname{asinh} x+\varepsilon.
    /// $$
    /// - If $x$ is infinite, zero, or NaN, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\operatorname{asinh} x|\rfloor-p+1}$.
    /// - If $x$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\operatorname{asinh} x|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=\text{NaN}$
    /// - $f(\pm\infty,p,m)=\pm\infty$
    /// - $f(\pm0.0,p,m)=\pm0.0$
    ///
    /// Overflow and underflow:
    /// - Since $|\operatorname{asinh} x| < |x|$ for nonzero $x$, the result never overflows.
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
    /// Underflow requires an input of magnitude $2^{-2^{30}}$, the smallest positive [`Float`],
    /// rounded toward zero: since $|\operatorname{asinh} x| < |x|$ for nonzero $x$, no other input
    /// can reach it.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::asinh_prec_ref`] instead. If
    /// you know that your target precision is the precision of the input, consider using
    /// [`Float::asinh_round_ref`] instead. If both of these things are true, consider using
    /// `(&Float).asinh()` instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^2 \log\log n + m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `self.significant_bits()`: the logarithm is computed at a working precision of about $n$,
    /// and the input is first squared at its own precision.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` and `self` is finite and nonzero, since the inverse hyperbolic
    /// sine of a finite nonzero [`Float`] is never exactly representable, or if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = (&Float::from_unsigned_prec(1u32, 100).0).asinh_prec_round_ref(5, Floor);
    /// assert_eq!(c.to_string(), "0.875");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = (&Float::from_unsigned_prec(1u32, 100).0).asinh_prec_round_ref(5, Ceiling);
    /// assert_eq!(c.to_string(), "0.906");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = (&Float::from_unsigned_prec(1u32, 100).0).asinh_prec_round_ref(5, Nearest);
    /// assert_eq!(c.to_string(), "0.875");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = (&Float::from_unsigned_prec(1u32, 100).0).asinh_prec_round_ref(20, Floor);
    /// assert_eq!(c.to_string(), "0.88137341");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = (&Float::from_unsigned_prec(1u32, 100).0).asinh_prec_round_ref(20, Ceiling);
    /// assert_eq!(c.to_string(), "0.88137436");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = (&Float::from_unsigned_prec(1u32, 100).0).asinh_prec_round_ref(20, Nearest);
    /// assert_eq!(c.to_string(), "0.88137341");
    /// assert_eq!(o, Less);
    /// ```
    pub fn asinh_prec_round_ref(&self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        assert_ne!(prec, 0);
        match &self.0 {
            NaN => (Self::NAN, Equal),
            // asinh(±inf) = ±inf, and asinh(±0) = ±0
            Infinity { .. } | Zero { .. } => (self.clone(), Equal),
            Finite { .. } => asinh_prec_round_normal_ref(self, prec, rm),
        }
    }

    /// Computes $\operatorname{asinh} x$, the inverse hyperbolic sine of a [`Float`], rounding the
    /// result to the nearest value of the specified precision. The [`Float`] is taken by value. An
    /// [`Ordering`] is also returned, indicating whether the rounded inverse hyperbolic sine is
    /// less than, equal to, or greater than the exact inverse hyperbolic sine. Although `NaN`s are
    /// not comparable to any [`Float`], whenever this function returns a `NaN` it also returns
    /// `Equal`.
    ///
    /// If the inverse hyperbolic sine is equidistant from two [`Float`]s with the specified
    /// precision, the [`Float`] with fewer 1s in its binary expansion is chosen. See
    /// [`RoundingMode`] for a description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = \operatorname{asinh} x+\varepsilon.
    /// $$
    /// - If $x$ is infinite, zero, or NaN, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2 |\operatorname{asinh}
    ///   x|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p)=\text{NaN}$
    /// - $f(\pm\infty,p)=\pm\infty$
    /// - $f(\pm0.0,p)=1.0$
    ///
    /// Overflow and underflow:
    /// - Since $|\operatorname{asinh} x| < |x|$ for nonzero $x$, the result never overflows.
    /// - If $0<f(x,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// Underflow requires an input of magnitude $2^{-2^{30}}$, the smallest positive [`Float`],
    /// rounded toward zero: since $|\operatorname{asinh} x| < |x|$ for nonzero $x$, no other input
    /// can reach it.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::asinh_prec_round`] instead. If you know that your target precision is the precision
    /// of the input, consider using [`Float::asinh`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^2 \log\log n + m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `self.significant_bits()`: the logarithm is computed at a working precision of about $n$,
    /// and the input is first squared at its own precision.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100).0.asinh_prec(5);
    /// assert_eq!(c.to_string(), "0.875");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100).0.asinh_prec(20);
    /// assert_eq!(c.to_string(), "0.88137341");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn asinh_prec(self, prec: u64) -> (Self, Ordering) {
        self.asinh_prec_round(prec, Nearest)
    }

    /// Computes $\operatorname{asinh} x$, the inverse hyperbolic sine of a [`Float`], rounding the
    /// result to the nearest value of the specified precision. The [`Float`] is taken by reference.
    /// An [`Ordering`] is also returned, indicating whether the rounded inverse hyperbolic sine is
    /// less than, equal to, or greater than the exact inverse hyperbolic sine. Although `NaN`s are
    /// not comparable to any [`Float`], whenever this function returns a `NaN` it also returns
    /// `Equal`.
    ///
    /// If the inverse hyperbolic sine is equidistant from two [`Float`]s with the specified
    /// precision, the [`Float`] with fewer 1s in its binary expansion is chosen. See
    /// [`RoundingMode`] for a description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = \operatorname{asinh} x+\varepsilon.
    /// $$
    /// - If $x$ is infinite, zero, or NaN, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2 |\operatorname{asinh}
    ///   x|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p)=\text{NaN}$
    /// - $f(\pm\infty,p)=\pm\infty$
    /// - $f(\pm0.0,p)=1.0$
    ///
    /// Overflow and underflow:
    /// - Since $|\operatorname{asinh} x| < |x|$ for nonzero $x$, the result never overflows.
    /// - If $0<f(x,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// Underflow requires an input of magnitude $2^{-2^{30}}$, the smallest positive [`Float`],
    /// rounded toward zero: since $|\operatorname{asinh} x| < |x|$ for nonzero $x$, no other input
    /// can reach it.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::asinh_prec_round_ref`] instead. If you know that your target precision is the
    /// precision of the input, consider using `(&Float).asinh()` instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^2 \log\log n + m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `self.significant_bits()`: the logarithm is computed at a working precision of about $n$,
    /// and the input is first squared at its own precision.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = (&Float::from_unsigned_prec(1u32, 100).0).asinh_prec_ref(5);
    /// assert_eq!(c.to_string(), "0.875");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = (&Float::from_unsigned_prec(1u32, 100).0).asinh_prec_ref(20);
    /// assert_eq!(c.to_string(), "0.88137341");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn asinh_prec_ref(&self, prec: u64) -> (Self, Ordering) {
        self.asinh_prec_round_ref(prec, Nearest)
    }

    /// Computes $\operatorname{asinh} x$, the inverse hyperbolic sine of a [`Float`], rounding the
    /// result with the specified rounding mode. The [`Float`] is taken by value. An [`Ordering`] is
    /// also returned, indicating whether the rounded inverse hyperbolic sine is less than, equal
    /// to, or greater than the exact inverse hyperbolic sine. Although `NaN`s are not comparable to
    /// any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the precision of the input. See [`RoundingMode`] for a
    /// description of the possible rounding modes.
    ///
    /// $$
    /// f(x,m) = \operatorname{asinh} x+\varepsilon.
    /// $$
    /// - If $x$ is infinite, zero, or NaN, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\operatorname{asinh} x|\rfloor-p+1}$, where $p$ is the precision of the
    ///   input.
    /// - If $x$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\operatorname{asinh} x|\rfloor-p}$, where $p$ is the precision of the
    ///   input.
    ///
    /// If the output has a precision, it is the precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN},m)=\text{NaN}$
    /// - $f(\pm\infty,m)=\pm\infty$
    /// - $f(\pm0.0,m)=1.0$
    ///
    /// Overflow and underflow:
    /// - Since $|\operatorname{asinh} x| < |x|$ for nonzero $x$, the result never overflows.
    /// - If $0<f(x,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is returned
    ///   instead.
    ///
    /// Underflow requires an input of magnitude $2^{-2^{30}}$, the smallest positive [`Float`],
    /// rounded toward zero: since $|\operatorname{asinh} x| < |x|$ for nonzero $x$, no other input
    /// can reach it.
    ///
    /// If you want to specify an output precision, consider using [`Float::asinh_prec_round`]
    /// instead. If you know you'll be using the `Nearest` rounding mode, consider using
    /// [`Float::asinh`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` and `self` is finite and nonzero, since the inverse hyperbolic
    /// sine of a finite nonzero [`Float`] is never exactly representable.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100).0.asinh_round(Floor);
    /// assert_eq!(c.to_string(), "0.88137358701954302523260932497968");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100).0.asinh_round(Ceiling);
    /// assert_eq!(c.to_string(), "0.88137358701954302523260932498047");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100).0.asinh_round(Nearest);
    /// assert_eq!(c.to_string(), "0.88137358701954302523260932497968");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn asinh_round(self, rm: RoundingMode) -> (Self, Ordering) {
        let prec = self.significant_bits();
        self.asinh_prec_round(prec, rm)
    }

    /// Computes $\operatorname{asinh} x$, the inverse hyperbolic sine of a [`Float`], rounding the
    /// result with the specified rounding mode. The [`Float`] is taken by reference. An
    /// [`Ordering`] is also returned, indicating whether the rounded inverse hyperbolic sine is
    /// less than, equal to, or greater than the exact inverse hyperbolic sine. Although `NaN`s are
    /// not comparable to any [`Float`], whenever this function returns a `NaN` it also returns
    /// `Equal`.
    ///
    /// The precision of the output is the precision of the input. See [`RoundingMode`] for a
    /// description of the possible rounding modes.
    ///
    /// $$
    /// f(x,m) = \operatorname{asinh} x+\varepsilon.
    /// $$
    /// - If $x$ is infinite, zero, or NaN, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\operatorname{asinh} x|\rfloor-p+1}$, where $p$ is the precision of the
    ///   input.
    /// - If $x$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\operatorname{asinh} x|\rfloor-p}$, where $p$ is the precision of the
    ///   input.
    ///
    /// If the output has a precision, it is the precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN},m)=\text{NaN}$
    /// - $f(\pm\infty,m)=\pm\infty$
    /// - $f(\pm0.0,m)=1.0$
    ///
    /// Overflow and underflow:
    /// - Since $|\operatorname{asinh} x| < |x|$ for nonzero $x$, the result never overflows.
    /// - If $0<f(x,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is returned
    ///   instead.
    ///
    /// Underflow requires an input of magnitude $2^{-2^{30}}$, the smallest positive [`Float`],
    /// rounded toward zero: since $|\operatorname{asinh} x| < |x|$ for nonzero $x$, no other input
    /// can reach it.
    ///
    /// If you want to specify an output precision, consider using [`Float::asinh_prec_round_ref`]
    /// instead. If you know you'll be using the `Nearest` rounding mode, consider using
    /// `(&Float).asinh()` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` and `self` is finite and nonzero, since the inverse hyperbolic
    /// sine of a finite nonzero [`Float`] is never exactly representable.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = (&Float::from_unsigned_prec(1u32, 100).0).asinh_round_ref(Floor);
    /// assert_eq!(c.to_string(), "0.88137358701954302523260932497968");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = (&Float::from_unsigned_prec(1u32, 100).0).asinh_round_ref(Ceiling);
    /// assert_eq!(c.to_string(), "0.88137358701954302523260932498047");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = (&Float::from_unsigned_prec(1u32, 100).0).asinh_round_ref(Nearest);
    /// assert_eq!(c.to_string(), "0.88137358701954302523260932497968");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn asinh_round_ref(&self, rm: RoundingMode) -> (Self, Ordering) {
        self.asinh_prec_round_ref(self.significant_bits(), rm)
    }

    /// Computes $\operatorname{asinh} x$, the inverse hyperbolic sine of a [`Float`], rounding the
    /// result to the specified precision and with the specified rounding mode. The [`Float`] is
    /// replaced by the result, and an [`Ordering`] is returned, indicating whether the rounded
    /// inverse hyperbolic sine is less than, equal to, or greater than the exact inverse hyperbolic
    /// sine. Although `NaN`s are not comparable to any [`Float`], whenever this function sets a
    /// `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets \operatorname{asinh} x+\varepsilon.
    /// $$
    /// - If $x$ is infinite, zero, or NaN, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\operatorname{asinh} x|\rfloor-p+1}$.
    /// - If $x$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\operatorname{asinh} x|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// See the [`Float::asinh_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::asinh_prec_assign`] instead.
    /// If you know that your target precision is the precision of the input, consider using
    /// [`Float::asinh_round_assign`] instead. If both of these things are true, consider using
    /// [`Float::asinh_assign`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^2 \log\log n + m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `self.significant_bits()`: the logarithm is computed at a working precision of about $n$,
    /// and the input is first squared at its own precision.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` and `self` is finite and nonzero, since the inverse hyperbolic
    /// sine of a finite nonzero [`Float`] is never exactly representable, or if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.asinh_prec_round_assign(5, Floor), Less);
    /// assert_eq!(x.to_string(), "0.875");
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.asinh_prec_round_assign(5, Ceiling), Greater);
    /// assert_eq!(x.to_string(), "0.906");
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.asinh_prec_round_assign(5, Nearest), Less);
    /// assert_eq!(x.to_string(), "0.875");
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.asinh_prec_round_assign(20, Floor), Less);
    /// assert_eq!(x.to_string(), "0.88137341");
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.asinh_prec_round_assign(20, Ceiling), Greater);
    /// assert_eq!(x.to_string(), "0.88137436");
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.asinh_prec_round_assign(20, Nearest), Less);
    /// assert_eq!(x.to_string(), "0.88137341");
    /// ```
    #[inline]
    pub fn asinh_prec_round_assign(&mut self, prec: u64, rm: RoundingMode) -> Ordering {
        let o;
        (*self, o) = self.asinh_prec_round_ref(prec, rm);
        o
    }

    /// Computes $\operatorname{asinh} x$, the inverse hyperbolic sine of a [`Float`], rounding the
    /// result to the nearest value of the specified precision. The [`Float`] is replaced by the
    /// result, and an [`Ordering`] is returned, indicating whether the rounded inverse hyperbolic
    /// sine is less than, equal to, or greater than the exact inverse hyperbolic sine. Although
    /// `NaN`s are not comparable to any [`Float`], whenever this function sets a `NaN` it also
    /// returns `Equal`.
    ///
    /// If the inverse hyperbolic sine is equidistant from two [`Float`]s with the specified
    /// precision, the [`Float`] with fewer 1s in its binary expansion is chosen. See
    /// [`RoundingMode`] for a description of the `Nearest` rounding mode.
    ///
    /// $$
    /// x \gets \operatorname{asinh} x+\varepsilon.
    /// $$
    /// - If $x$ is infinite, zero, or NaN, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2 |\operatorname{asinh}
    ///   x|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// See the [`Float::asinh_prec`] documentation for information on special cases, overflow, and
    /// underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::asinh_prec_round_assign`] instead. If you know that your target precision is the
    /// precision of the input, consider using [`Float::asinh_assign`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^2 \log\log n + m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `self.significant_bits()`: the logarithm is computed at a working precision of about $n$,
    /// and the input is first squared at its own precision.
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
    /// assert_eq!(x.asinh_prec_assign(5), Less);
    /// assert_eq!(x.to_string(), "0.875");
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.asinh_prec_assign(20), Less);
    /// assert_eq!(x.to_string(), "0.88137341");
    /// ```
    #[inline]
    pub fn asinh_prec_assign(&mut self, prec: u64) -> Ordering {
        self.asinh_prec_round_assign(prec, Nearest)
    }

    /// Computes $\operatorname{asinh} x$, the inverse hyperbolic sine of a [`Float`], rounding the
    /// result with the specified rounding mode. The [`Float`] is replaced by the result, and an
    /// [`Ordering`] is returned, indicating whether the rounded inverse hyperbolic sine is less
    /// than, equal to, or greater than the exact inverse hyperbolic sine. Although `NaN`s are not
    /// comparable to any [`Float`], whenever this function sets a `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the precision of the input. See [`RoundingMode`] for a
    /// description of the possible rounding modes.
    ///
    /// $$
    /// x \gets \operatorname{asinh} x+\varepsilon.
    /// $$
    /// - If $x$ is infinite, zero, or NaN, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\operatorname{asinh} x|\rfloor-p+1}$, where $p$ is the precision of the
    ///   input.
    /// - If $x$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\operatorname{asinh} x|\rfloor-p}$, where $p$ is the precision of the
    ///   input.
    ///
    /// If the output has a precision, it is the precision of the input.
    ///
    /// See the [`Float::asinh_round`] documentation for information on special cases, overflow, and
    /// underflow.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::asinh_prec_round_assign`] instead. If you know you'll be using the `Nearest`
    /// rounding mode, consider using [`Float::asinh_assign`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` and `self` is finite and nonzero, since the inverse hyperbolic
    /// sine of a finite nonzero [`Float`] is never exactly representable.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.asinh_round_assign(Floor), Less);
    /// assert_eq!(x.to_string(), "0.88137358701954302523260932497968");
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.asinh_round_assign(Ceiling), Greater);
    /// assert_eq!(x.to_string(), "0.88137358701954302523260932498047");
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.asinh_round_assign(Nearest), Less);
    /// assert_eq!(x.to_string(), "0.88137358701954302523260932497968");
    /// ```
    #[inline]
    pub fn asinh_round_assign(&mut self, rm: RoundingMode) -> Ordering {
        let prec = self.significant_bits();
        self.asinh_prec_round_assign(prec, rm)
    }
}

impl Float {
    /// Computes $\operatorname{asinh} x$, the inverse hyperbolic sine of a [`Rational`], rounding
    /// the result to the specified precision and with the specified rounding mode and returning the
    /// result as a [`Float`]. The [`Rational`] is taken by value. An [`Ordering`] is also returned,
    /// indicating whether the rounded inverse hyperbolic sine is less than, equal to, or greater
    /// than the exact inverse hyperbolic sine.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \operatorname{asinh} x+\varepsilon.
    /// $$
    /// - If $m$ is not `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2 |\operatorname{asinh}
    ///   x|\rfloor-p+1}$.
    /// - If $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2 |\operatorname{asinh}
    ///   x|\rfloor-p}$.
    ///
    /// These bounds do not apply when the result overflows or underflows; see below.
    ///
    /// The output has precision `prec`.
    ///
    /// Special cases:
    /// - $f(0,p,m)=0.0$.
    ///
    /// Overflow and underflow:
    /// - Since $|\operatorname{asinh} x| < \ln(2|x|+1)$, the result never overflows.
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
    /// Underflow requires an input of magnitude at most $2^{-2^{30}}$, the smallest positive
    /// [`Float`]: since $|\operatorname{asinh} x| < |x|$ for nonzero $x$, no larger input can reach
    /// it.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::asinh_rational_prec`]
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^2 \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `x.significant_bits()`: the logarithm is computed at a working precision of about $n$, and
    /// the input is handled with `Rational` arithmetic.
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
    /// let (c, o) = Float::asinh_rational_prec_round(Rational::from_unsigneds(3u8, 5), 5, Floor);
    /// assert_eq!(c.to_string(), "0.562");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::asinh_rational_prec_round(Rational::from_unsigneds(3u8, 5), 5, Ceiling);
    /// assert_eq!(c.to_string(), "0.594");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = Float::asinh_rational_prec_round(Rational::from_signeds(-3i8, 5), 20, Floor);
    /// assert_eq!(c.to_string(), "-0.56882572");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::asinh_rational_prec_round(Rational::from_signeds(-3i8, 5), 20, Ceiling);
    /// assert_eq!(c.to_string(), "-0.56882477");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    #[allow(clippy::needless_pass_by_value)]
    pub fn asinh_rational_prec_round(x: Rational, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        Self::asinh_rational_prec_round_ref(&x, prec, rm)
    }

    /// Computes $\operatorname{asinh} x$, the inverse hyperbolic sine of a [`Rational`], rounding
    /// the result to the specified precision and with the specified rounding mode and returning the
    /// result as a [`Float`]. The [`Rational`] is taken by reference. An [`Ordering`] is also
    /// returned, indicating whether the rounded inverse hyperbolic sine is less than, equal to, or
    /// greater than the exact inverse hyperbolic sine.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \operatorname{asinh} x+\varepsilon.
    /// $$
    /// - If $m$ is not `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2 |\operatorname{asinh}
    ///   x|\rfloor-p+1}$.
    /// - If $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2 |\operatorname{asinh}
    ///   x|\rfloor-p}$.
    ///
    /// These bounds do not apply when the result overflows or underflows; see below.
    ///
    /// The output has precision `prec`.
    ///
    /// Special cases:
    /// - $f(0,p,m)=0.0$.
    ///
    /// Overflow and underflow:
    /// - Since $|\operatorname{asinh} x| < \ln(2|x|+1)$, the result never overflows.
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
    /// Underflow requires an input of magnitude at most $2^{-2^{30}}$, the smallest positive
    /// [`Float`]: since $|\operatorname{asinh} x| < |x|$ for nonzero $x$, no larger input can reach
    /// it.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::asinh_rational_prec_ref`]
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^2 \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `x.significant_bits()`: the logarithm is computed at a working precision of about $n$, and
    /// the input is handled with `Rational` arithmetic.
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
    ///     Float::asinh_rational_prec_round_ref(&Rational::from_unsigneds(3u8, 5), 5, Floor);
    /// assert_eq!(c.to_string(), "0.562");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) =
    ///     Float::asinh_rational_prec_round_ref(&Rational::from_unsigneds(3u8, 5), 5, Ceiling);
    /// assert_eq!(c.to_string(), "0.594");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) =
    ///     Float::asinh_rational_prec_round_ref(&Rational::from_signeds(-3i8, 5), 20, Floor);
    /// assert_eq!(c.to_string(), "-0.56882572");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) =
    ///     Float::asinh_rational_prec_round_ref(&Rational::from_signeds(-3i8, 5), 20, Ceiling);
    /// assert_eq!(c.to_string(), "-0.56882477");
    /// assert_eq!(o, Greater);
    /// ```
    pub fn asinh_rational_prec_round_ref(
        x: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        assert_ne!(prec, 0);
        if *x == 0u32 {
            // asinh(0) = 0, exactly
            return (Self::ZERO, Equal);
        }
        asinh_rational_helper(x, prec, rm)
    }

    /// Computes $\operatorname{asinh} x$, the inverse hyperbolic sine of a [`Rational`], rounding
    /// the result to the nearest value of the specified precision and returning the result as a
    /// [`Float`]. The [`Rational`] is taken by value. An [`Ordering`] is also returned, indicating
    /// whether the rounded inverse hyperbolic sine is less than, equal to, or greater than the
    /// exact inverse hyperbolic sine.
    ///
    /// If the inverse hyperbolic sine is equidistant from two [`Float`]s with the specified
    /// precision, the [`Float`] with fewer 1s in its binary expansion is chosen. See
    /// [`RoundingMode`] for a description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = \operatorname{asinh} x+\varepsilon,
    /// $$
    /// where $|\varepsilon| \leq 2^{\lfloor\log_2 |\operatorname{asinh} x|\rfloor-p}$ (unless the
    /// result overflows or underflows; see below).
    ///
    /// The output has precision `prec`.
    ///
    /// Special cases:
    /// - $f(0,p)=0.0$.
    ///
    /// Overflow and underflow:
    /// - Since $|\operatorname{asinh} x| < \ln(2|x|+1)$, the result never overflows.
    /// - If $0<f(x,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::asinh_rational_prec_round`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^2 \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `x.significant_bits()`: the logarithm is computed at a working precision of about $n$, and
    /// the input is handled with `Rational` arithmetic.
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
    /// let (c, o) = Float::asinh_rational_prec(Rational::from_unsigneds(3u8, 5), 5);
    /// assert_eq!(c.to_string(), "0.562");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::asinh_rational_prec(Rational::from_unsigneds(3u8, 5), 20);
    /// assert_eq!(c.to_string(), "0.56882477");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::asinh_rational_prec(Rational::ZERO, 10);
    /// assert_eq!(c.to_string(), "0.0");
    /// assert_eq!(o, Equal);
    /// ```
    #[inline]
    #[allow(clippy::needless_pass_by_value)]
    pub fn asinh_rational_prec(x: Rational, prec: u64) -> (Self, Ordering) {
        Self::asinh_rational_prec_round_ref(&x, prec, Nearest)
    }

    /// Computes $\operatorname{asinh} x$, the inverse hyperbolic sine of a [`Rational`], rounding
    /// the result to the nearest value of the specified precision and returning the result as a
    /// [`Float`]. The [`Rational`] is taken by reference. An [`Ordering`] is also returned,
    /// indicating whether the rounded inverse hyperbolic sine is less than, equal to, or greater
    /// than the exact inverse hyperbolic sine.
    ///
    /// If the inverse hyperbolic sine is equidistant from two [`Float`]s with the specified
    /// precision, the [`Float`] with fewer 1s in its binary expansion is chosen. See
    /// [`RoundingMode`] for a description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = \operatorname{asinh} x+\varepsilon,
    /// $$
    /// where $|\varepsilon| \leq 2^{\lfloor\log_2 |\operatorname{asinh} x|\rfloor-p}$ (unless the
    /// result overflows or underflows; see below).
    ///
    /// The output has precision `prec`.
    ///
    /// Special cases:
    /// - $f(0,p)=0.0$.
    ///
    /// Overflow and underflow:
    /// - Since $|\operatorname{asinh} x| < \ln(2|x|+1)$, the result never overflows.
    /// - If $0<f(x,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::asinh_rational_prec_round_ref`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^2 \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `x.significant_bits()`: the logarithm is computed at a working precision of about $n$, and
    /// the input is handled with `Rational` arithmetic.
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
    /// let (c, o) = Float::asinh_rational_prec_ref(&Rational::from_unsigneds(3u8, 5), 5);
    /// assert_eq!(c.to_string(), "0.562");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::asinh_rational_prec_ref(&Rational::from_unsigneds(3u8, 5), 20);
    /// assert_eq!(c.to_string(), "0.56882477");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::asinh_rational_prec_ref(&Rational::ZERO, 10);
    /// assert_eq!(c.to_string(), "0.0");
    /// assert_eq!(o, Equal);
    /// ```
    #[inline]
    pub fn asinh_rational_prec_ref(x: &Rational, prec: u64) -> (Self, Ordering) {
        Self::asinh_rational_prec_round_ref(x, prec, Nearest)
    }
}

impl Asinh for Float {
    type Output = Self;

    /// Computes $\operatorname{asinh} x$, the inverse hyperbolic sine of a [`Float`], taking it by
    /// value.
    ///
    /// If the output has a precision, it is the precision of the input. If the inverse hyperbolic
    /// sine is equidistant from two [`Float`]s with the specified precision, the [`Float`] with
    /// fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of the
    /// `Nearest` rounding mode.
    ///
    /// $$
    /// f(x) = \operatorname{asinh} x+\varepsilon.
    /// $$
    /// - If $x$ is infinite, zero, or NaN, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2 |\operatorname{asinh}
    ///   x|\rfloor-p}$, where $p$ is the precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN})=\text{NaN}$
    /// - $f(\pm\infty)=\pm\infty$
    /// - $f(\pm0.0)=\pm0.0$
    ///
    /// See the [`Float::asinh_round`] documentation for information on overflow and underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::asinh_round`] instead. If you want to specify the output precision, consider using
    /// [`Float::asinh_prec`]. If you want both of these things, consider using
    /// [`Float::asinh_prec_round`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Asinh;
    /// use malachite_base::num::basic::traits::*;
    /// use malachite_float::Float;
    ///
    /// assert!(Float::NAN.asinh().is_nan());
    /// assert_eq!(Float::INFINITY.asinh().to_string(), "Infinity");
    /// assert_eq!(Float::NEGATIVE_INFINITY.asinh().to_string(), "-Infinity");
    /// assert_eq!(Float::ZERO.asinh().to_string(), "0.0");
    /// assert_eq!(Float::NEGATIVE_ZERO.asinh().to_string(), "-0.0");
    /// assert_eq!(
    ///     Float::from_unsigned_prec(1u32, 100).0.asinh().to_string(),
    ///     "0.88137358701954302523260932497968"
    /// );
    /// assert_eq!(
    ///     Float::from_unsigned_prec(100u32, 100).0.asinh().to_string(),
    ///     "5.2983423656105887573688256891151"
    /// );
    /// ```
    #[inline]
    fn asinh(self) -> Self {
        let prec = self.significant_bits();
        self.asinh_prec_round(prec, Nearest).0
    }
}

impl Asinh for &Float {
    type Output = Float;

    /// Computes $\operatorname{asinh} x$, the inverse hyperbolic sine of a [`Float`], taking it by
    /// reference.
    ///
    /// If the output has a precision, it is the precision of the input. If the inverse hyperbolic
    /// sine is equidistant from two [`Float`]s with the specified precision, the [`Float`] with
    /// fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of the
    /// `Nearest` rounding mode.
    ///
    /// $$
    /// f(x) = \operatorname{asinh} x+\varepsilon.
    /// $$
    /// - If $x$ is infinite, zero, or NaN, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2 |\operatorname{asinh}
    ///   x|\rfloor-p}$, where $p$ is the precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN})=\text{NaN}$
    /// - $f(\pm\infty)=\pm\infty$
    /// - $f(\pm0.0)=\pm0.0$
    ///
    /// See the [`Float::asinh_round`] documentation for information on overflow and underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::asinh_round_ref`] instead. If you want to specify the output precision, consider
    /// using [`Float::asinh_prec_ref`]. If you want both of these things, consider using
    /// [`Float::asinh_prec_round_ref`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Asinh;
    /// use malachite_base::num::basic::traits::*;
    /// use malachite_float::Float;
    ///
    /// assert!(Float::NAN.asinh().is_nan());
    /// assert_eq!(Float::INFINITY.asinh().to_string(), "Infinity");
    /// assert_eq!(Float::NEGATIVE_INFINITY.asinh().to_string(), "-Infinity");
    /// assert_eq!(Float::ZERO.asinh().to_string(), "0.0");
    /// assert_eq!(Float::NEGATIVE_ZERO.asinh().to_string(), "-0.0");
    /// assert_eq!(
    ///     (&Float::from_unsigned_prec(1u32, 100).0).asinh().to_string(),
    ///     "0.88137358701954302523260932497968"
    /// );
    /// assert_eq!(
    ///     (&Float::from_unsigned_prec(100u32, 100).0)
    ///         .asinh()
    ///         .to_string(),
    ///     "5.2983423656105887573688256891151"
    /// );
    /// ```
    #[inline]
    fn asinh(self) -> Float {
        self.asinh_prec_round_ref(self.significant_bits(), Nearest)
            .0
    }
}

impl AsinhAssign for Float {
    /// Computes $\operatorname{asinh} x$, the inverse hyperbolic sine of a [`Float`], in place.
    ///
    /// If the output has a precision, it is the precision of the input. If the inverse hyperbolic
    /// sine is equidistant from two [`Float`]s with the specified precision, the [`Float`] with
    /// fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of the
    /// `Nearest` rounding mode.
    ///
    /// $$
    /// x \gets \operatorname{asinh} x+\varepsilon.
    /// $$
    /// - If $x$ is infinite, zero, or NaN, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2 |\operatorname{asinh}
    ///   x|\rfloor-p}$, where $p$ is the precision of the input.
    ///
    /// See the [`Float::asinh`] documentation for information on special cases, overflow, and
    /// underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::asinh_round_assign`] instead. If you want to specify the output precision, consider
    /// using [`Float::asinh_prec_assign`]. If you want both of these things, consider using
    /// [`Float::asinh_prec_round_assign`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::AsinhAssign;
    /// use malachite_base::num::basic::traits::*;
    /// use malachite_float::Float;
    ///
    /// let mut x = Float::NAN;
    /// x.asinh_assign();
    /// assert!(x.is_nan());
    ///
    /// let mut x = Float::INFINITY;
    /// x.asinh_assign();
    /// assert_eq!(x.to_string(), "Infinity");
    ///
    /// let mut x = Float::NEGATIVE_INFINITY;
    /// x.asinh_assign();
    /// assert_eq!(x.to_string(), "-Infinity");
    ///
    /// let mut x = Float::ZERO;
    /// x.asinh_assign();
    /// assert_eq!(x.to_string(), "0.0");
    ///
    /// let mut x = Float::NEGATIVE_ZERO;
    /// x.asinh_assign();
    /// assert_eq!(x.to_string(), "-0.0");
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// x.asinh_assign();
    /// assert_eq!(x.to_string(), "0.88137358701954302523260932497968");
    ///
    /// let mut x = Float::from_unsigned_prec(100u32, 100).0;
    /// x.asinh_assign();
    /// assert_eq!(x.to_string(), "5.2983423656105887573688256891151");
    /// ```
    #[inline]
    fn asinh_assign(&mut self) {
        let prec = self.significant_bits();
        self.asinh_prec_round_assign(prec, Nearest);
    }
}

/// Computes $\operatorname{asinh} x$, the inverse hyperbolic sine of a primitive float. Using this
/// function is more accurate than using the default `asinh` function or the one provided by `libm`.
///
/// $$
/// f(x) = \operatorname{asinh} x+\varepsilon.
/// $$
/// - If $x$ is infinite, zero, or NaN, $\varepsilon$ may be ignored or assumed to be 0.
/// - If $x$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2 |\operatorname{asinh}
///   x|\rfloor-p}$, where $p$ is the precision of the output (24 if `T` is a [`f32`] and 53 if `T`
///   is a [`f64`]).
///
/// Special cases:
/// - $f(\text{NaN})=\text{NaN}$
/// - $f(\pm\infty)=\pm\infty$
/// - $f(\pm0.0)=\pm0.0$
///
/// Overflow is not possible, since $|\operatorname{asinh} x| \leq |x|$. The result is subnormal
/// only when $x$ is, and then it is $x$ itself, since $|\operatorname{asinh} x - x| < |x|^3/6$.
///
/// # Worst-case complexity
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use malachite_base::num::basic::traits::NegativeInfinity;
/// use malachite_base::num::float::NiceFloat;
/// use malachite_float::float::arithmetic::asinh::primitive_float_asinh;
///
/// assert!(primitive_float_asinh(f32::NAN).is_nan());
/// assert_eq!(
///     NiceFloat(primitive_float_asinh(f32::INFINITY)),
///     NiceFloat(f32::INFINITY)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_asinh(f32::NEGATIVE_INFINITY)),
///     NiceFloat(f32::NEGATIVE_INFINITY)
/// );
/// assert_eq!(NiceFloat(primitive_float_asinh(0.0f32)), NiceFloat(0.0));
/// assert_eq!(NiceFloat(primitive_float_asinh(-0.0f32)), NiceFloat(-0.0));
/// assert_eq!(
///     NiceFloat(primitive_float_asinh(1.0f32)),
///     NiceFloat(0.8813736)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_asinh(1.0f64)),
///     NiceFloat(0.881373587019543)
/// );
/// ```
#[inline]
#[allow(clippy::type_repetition_in_bounds)]
pub fn primitive_float_asinh<T: PrimitiveFloat>(x: T) -> T
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    emulate_float_to_float_fn(Float::asinh_prec, x)
}

/// Computes $\operatorname{asinh} x$, the inverse hyperbolic sine of a [`Rational`], returning the
/// result as a primitive float. The result is correctly rounded.
///
/// $$
/// f(x) = \operatorname{asinh} x+\varepsilon.
/// $$
/// - If $\operatorname{asinh} x$ is infinite or zero, $\varepsilon$ may be ignored or assumed to be
///   0.
/// - If $\operatorname{asinh} x$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2
///   |\operatorname{asinh} x|\rfloor-p}$, where $p$ is the precision of the output (typically 24 if
///   `T` is a [`f32`] and 53 if `T` is a [`f64`], but less if the output is subnormal).
///
/// Special cases:
/// - $f(0)=0.0$
///
/// Overflow is not possible. Underflow is: an `x` of small enough magnitude gives `0.0` or `-0.0`.
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
/// use malachite_float::float::arithmetic::asinh::primitive_float_asinh_rational;
/// use malachite_q::Rational;
///
/// assert_eq!(
///     NiceFloat(primitive_float_asinh_rational::<f64>(&Rational::ZERO)),
///     NiceFloat(0.0)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_asinh_rational::<f64>(
///         &Rational::from_unsigneds(1u8, 3)
///     )),
///     NiceFloat(0.32745015023725843)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_asinh_rational::<f64>(
///         &Rational::from_unsigneds(22u8, 7)
///     )),
///     NiceFloat(1.86267921113461)
/// );
/// ```
#[inline]
#[allow(clippy::type_repetition_in_bounds)]
pub fn primitive_float_asinh_rational<T: PrimitiveFloat>(x: &Rational) -> T
where
    Float: PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    emulate_rational_to_float_fn(Float::asinh_rational_prec_ref, x)
}
