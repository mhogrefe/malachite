// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the GNU MPFR Library.
//
//      Copyright 2005-2026 Free Software Foundation, Inc.
//
//      Contributed by the Pascaline and Caramba projects, INRIA.
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::InnerFloat::{Finite, Infinity, NaN, Zero};
use crate::float::arithmetic::cos::{cos_rational_tiny, round_bracket, round_scaled_bracket};
use crate::float::arithmetic::exp::one_neighbor;
use crate::float::arithmetic::round_near_x::small_input_shortcut;
use crate::float::arithmetic::sin::underflowed;
use crate::float::arithmetic::tanh::cosh_bound;
use crate::{Float, emulate_float_to_float_fn, emulate_rational_to_float_fn, floor_and_ceiling};
use core::cmp::Ordering::{self, Equal};
use malachite_base::fail_on_untested_path;
use malachite_base::num::arithmetic::traits::{
    Abs, CeilingLogBase2, Reciprocal, ReciprocalAssign, Sech, SechAssign,
};
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::{NaN as NaNTrait, One, Zero as ZeroTrait};
use malachite_base::num::conversion::traits::{ExactFrom, RoundingFrom};
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_nz::natural::arithmetic::float::round::float_can_round;
use malachite_nz::platform::Limb;
use malachite_q::Rational;

// Beyond this |x|, sech(x) < 2 exp(-|x|) < 2^(MIN_EXPONENT - 2) = 2^(-2^30 - 1), half the smallest
// positive Float, since (2^30 + 2) log(2) = 744261119.35... |csch(x)| = 2 exp(-|x|) / (1 -
// exp(-2|x|)) exceeds 2 exp(-|x|) only by a negligible factor, so the same threshold serves it.
pub(crate) const RECIPROCAL_HYPERBOLIC_UNDERFLOW_THRESHOLD: u32 = 744261120;

// Computes sech(x) (if `plus`) or csch(x) (if not) for a finite x with 2^29 <= |x| = `x_abs` <
// `RECIPROCAL_HYPERBOLIC_UNDERFLOW_THRESHOLD`, the result being negative if `negative`. MPFR
// computes 1 / cosh(x) or 1 / sinh(x) in its extended exponent range, and declares underflow when
// the denominator overflows that range. In Malachite's exponent range, cosh(x) and sinh(x) overflow
// at 2^(2^30 - 1), while their reciprocals stay representable down to 2^(-2^30 - 1), so for x in a
// window of width about 2 log(2) the denominator overflows but the result does not; and a result
// near the bottom of the range must be rounded with the underflow rules. So the result 2 exp(-|x|)
// / (1 ± exp(-2|x|)) is computed from a = exp(-|x|/2), which is representable, scaled by an exact
// power of 2 into [1/2, 1]: with A = a 2^S, it is 2^(1 - 2S) A^2 / (1 ± a^4). A^2 is bracketed
// with directed roundings, and the factor is absorbed by nudging one end by an ulp: for sech, 1 -
// 2^(-4S) < 1 / (1 + a^4) < 1 moves the lower end down, and for csch, 1 < 1 / (1 - a^4) < 1 + 2^(1
// - 4S) moves the upper end up. The bracket is then rounded with the final scaling by 2^(1 - 2S).
fn reciprocal_hyperbolic_scaled(
    x_abs: &Float,
    negative: bool,
    plus: bool,
    prec: u64,
    rm: RoundingMode,
) -> (Float, Ordering) {
    let neg_half_x = -(x_abs >> 1u32);
    let mut working_prec = prec + prec.ceiling_log_base_2() + 10;
    let mut increment = Limb::WIDTH;
    loop {
        // exp(-|x|/2) is transcendental, so it is not exact
        let (a_lo, a_hi) = floor_and_ceiling(neg_half_x.exp_prec_round_ref(working_prec, Floor));
        let s = -i64::from(a_lo.get_exponent().unwrap());
        let mut lo = (a_lo << s).square_prec_round(working_prec, Floor).0;
        let mut hi = (a_hi << s).square_prec_round(working_prec, Ceiling).0;
        // a < 2^(-S), so a^4 < 2^(-4S)
        let four_s = u64::exact_from(s << 2);
        if working_prec + 2 < four_s {
            // 1 ulp is at least the value times 2^(-working_prec), more than it times 2^(1 - 4S)
            if plus {
                lo.decrement();
            } else {
                hi.increment();
            }
        } else {
            // only reachable beyond about 2^30 bits of precision
            fail_on_untested_path("reciprocal_hyperbolic_scaled, working precision beyond 4S");
            if plus {
                lo.mul_prec_round_assign(one_neighbor(four_s, false), working_prec, Floor);
            } else {
                hi.mul_prec_round_assign(one_neighbor(four_s - 1, true), working_prec, Ceiling);
            }
        }
        let (lo, hi) = if negative { (-hi, -lo) } else { (lo, hi) };
        if let Some(result) = round_scaled_bracket(&lo, &hi, 1 - (s << 1), prec, rm) {
            return result;
        }
        working_prec += increment;
        increment = working_prec >> 1;
    }
}

// Computes sech(x) (if `plus`) or csch(x) (if not) for a finite x whose exponent exceeds 29, so
// that |x| >= 2^29, returning `None` for any smaller x: past
// `RECIPROCAL_HYPERBOLIC_UNDERFLOW_THRESHOLD` the result underflows, and below it the scaled path
// computes it.
pub(crate) fn reciprocal_hyperbolic_large(
    x: &Float,
    plus: bool,
    prec: u64,
    rm: RoundingMode,
) -> Option<(Float, Ordering)> {
    if x.get_exponent().unwrap() <= 29 {
        return None;
    }
    let negative = !plus && x.is_sign_negative();
    let x_abs = x.abs();
    Some(if x_abs >= RECIPROCAL_HYPERBOLIC_UNDERFLOW_THRESHOLD {
        underflowed(!negative, prec, rm)
    } else {
        reciprocal_hyperbolic_scaled(&x_abs, negative, plus, prec, rm)
    })
}

// This is mpfr_sech from sech.c (an instantiation of gen_inverse.h), MPFR 4.2.2, where the input is
// finite and nonzero, with the scaled path for large inputs.
fn sech_prec_round_normal_ref(x: &Float, prec: u64, rm: RoundingMode) -> (Float, Ordering) {
    assert_ne!(rm, Exact, "Inexact sech");
    let exp_x = i64::from(x.get_exponent().unwrap());
    // for x near 0, sech(x) = 1 - x^2/2 + ..., more precisely |sech(x)-1| <= x^2/2 for |x| <= 1.
    // The tiny action is the same as for cos(x).
    //
    // MPFR_FAST_COMPUTE_IF_SMALL_INPUT(y, __gmpfr_one, -2 * MPFR_GET_EXP (x), 1, 0, r, ...)
    if let Some(result) = small_input_shortcut(&Float::ONE, -(exp_x << 1), 1, false, prec, rm) {
        return result;
    }
    if let Some(result) = reciprocal_hyperbolic_large(x, true, prec, rm) {
        return result;
    }
    // |x| < 2^29, so cosh(x) < exp(2^29) < 2^(2^30 - 1) cannot overflow, and sech(x) > 2^(-2^30) is
    // well above the bottom of the exponent range.
    let mut working_prec = prec + prec.ceiling_log_base_2() + 3;
    let mut increment = Limb::WIDTH;
    loop {
        // the cosh has an error below 1 ulp, and rounding toward zero fixes its sign
        let mut sech_x = x.cosh_prec_round_ref(working_prec, Down).0;
        sech_x.reciprocal_assign();
        // the error is less than c_w + 2*c_u*k_u (see algorithms.tex), where c_w = 1/2, c_u = 1
        // since the cosh was rounded toward zero, thus 1/2 + 2 < 4
        if float_can_round(
            sech_x.significand_ref().unwrap(),
            working_prec - 2,
            prec,
            rm,
        ) {
            return Float::from_float_prec_round(sech_x, prec, rm);
        }
        working_prec += increment;
        increment = working_prec >> 1;
    }
}

// Computes sech(x) for a nonzero `Rational` x, rounded to precision `prec` with rounding mode `rm`.
// sech(x) is transcendental for every nonzero rational x, so the result is never exact.
fn sech_rational_helper(x: &Rational, prec: u64, rm: RoundingMode) -> (Float, Ordering) {
    assert_ne!(rm, Exact, "Inexact sech");
    let exp_x = x.floor_log_base_2_abs() + 1; // the MPFR-style exponent of x
    // 0 < 1 - sech(x) < x^2/2 < 2^(2 exp_x - 1): when that is at most 2^(-prec - 1), half an ulp
    // below 1, sech(x) rounds to 1, or to its predecessor for rounding toward zero, as cos(x) does.
    if 1 - (exp_x << 1) > i64::exact_from(prec) {
        return cos_rational_tiny(prec, rm);
    }
    // A small x is handled by bracketing sech(x) = 1 / cosh(x) with series bounds on cosh(x). This
    // also covers every remaining x too small to be a `Float`.
    if exp_x < -1 && u64::exact_from(-exp_x) << 4 >= prec + 10 {
        return sech_rational_series(x, prec, rm);
    }
    let x_abs = x.abs();
    if x_abs >= RECIPROCAL_HYPERBOLIC_UNDERFLOW_THRESHOLD {
        return underflowed(true, prec, rm);
    }
    // sech is even and decreasing on [0, infinity), so bracket |x| between the Floats x_lo <= |x|
    // <= x_hi, take the hyperbolic secant of both, and increase the working precision until the two
    // round to the same result, which the exact sech(x), lying between them, must then share.
    let mut working_prec = prec + 10;
    let mut increment = Limb::WIDTH;
    loop {
        let (x_lo, x_o) = Float::from_rational_prec_round_ref(&x_abs, working_prec, Floor);
        if x_o == Equal {
            // |x| is exactly representable at `working_prec`, so sech(x) is simply sech(x_lo).
            return sech_prec_round_normal_ref(&x_lo, prec, rm);
        }
        let (x_lo, x_hi) = floor_and_ceiling((x_lo, x_o));
        // The hyperbolic secant of a finite nonzero Float is never exact, so both orderings are
        // `Less` or `Greater`, never `Equal`.
        let (s_lo, o_lo) = sech_prec_round_normal_ref(&x_lo, prec, rm);
        let (s_hi, o_hi) = sech_prec_round_normal_ref(&x_hi, prec, rm);
        if o_lo == o_hi && s_lo == s_hi {
            return (s_lo, o_lo);
        }
        working_prec += increment;
        increment = working_prec >> 1;
    }
}

// Brackets sech(x) = 1 / cosh(x) for a nonzero `Rational` x, small enough that the series of
// cosh(x) converges in a few terms, by the reciprocals of bounds on cosh(x), tightening the bracket
// until both ends round the same way.
fn sech_rational_series(x: &Rational, prec: u64, rm: RoundingMode) -> (Float, Ordering) {
    let mut w = prec + 10;
    let mut increment = Limb::WIDTH;
    loop {
        let lo = cosh_bound(x, w, true).reciprocal();
        let hi = cosh_bound(x, w, false).reciprocal();
        if let Some(result) = round_bracket(&lo, &hi, prec, rm) {
            return result;
        }
        w += increment;
        increment = w >> 1;
    }
}

impl Float {
    /// Computes $\operatorname{sech} x$, the hyperbolic secant of a [`Float`], rounding the result
    /// to the specified precision and with the specified rounding mode. The [`Float`] is taken by
    /// value. An [`Ordering`] is also returned, indicating whether the rounded hyperbolic secant is
    /// less than, equal to, or greater than the exact hyperbolic secant. Although `NaN`s are not
    /// comparable to any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \operatorname{sech} x+\varepsilon.
    /// $$
    /// - If $\operatorname{sech} x$ is zero or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\operatorname{sech} x$ is nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 \operatorname{sech} x\rfloor-p+1}$.
    /// - If $\operatorname{sech} x$ is nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 \operatorname{sech} x\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=\text{NaN}$
    /// - $f(\infty,p,m)=0.0$
    /// - $f(-\infty,p,m)=0.0$
    /// - $f(\pm0.0,p,m)=1.0$
    ///
    /// Overflow and underflow:
    /// - Since $\operatorname{sech} x\leq 1$, the result never overflows.
    /// - If $0<f(x,p,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,p,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,p,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,p,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is returned
    ///   instead.
    ///
    /// Underflow happens for inputs of magnitude above about $7.4\times10^8$.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::sech_prec`] instead. If you
    /// know that your target precision is the precision of the input, consider using
    /// [`Float::sech_round`] instead. If both of these things are true, consider using
    /// [`Float::sech`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{3/2} \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` and `self` is finite and nonzero, since the hyperbolic secant of a
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
    ///     .sech_prec_round(5, Floor);
    /// assert_eq!(c.to_string(), "0.625");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .sech_prec_round(5, Ceiling);
    /// assert_eq!(c.to_string(), "0.656");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .sech_prec_round(5, Nearest);
    /// assert_eq!(c.to_string(), "0.656");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .sech_prec_round(20, Floor);
    /// assert_eq!(c.to_string(), "0.64805412");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .sech_prec_round(20, Ceiling);
    /// assert_eq!(c.to_string(), "0.64805508");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .sech_prec_round(20, Nearest);
    /// assert_eq!(c.to_string(), "0.64805412");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn sech_prec_round(self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        self.sech_prec_round_ref(prec, rm)
    }

    /// Computes $\operatorname{sech} x$, the hyperbolic secant of a [`Float`], rounding the result
    /// to the specified precision and with the specified rounding mode. The [`Float`] is taken by
    /// reference. An [`Ordering`] is also returned, indicating whether the rounded hyperbolic
    /// secant is less than, equal to, or greater than the exact hyperbolic secant. Although `NaN`s
    /// are not comparable to any [`Float`], whenever this function returns a `NaN` it also returns
    /// `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \operatorname{sech} x+\varepsilon.
    /// $$
    /// - If $\operatorname{sech} x$ is zero or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\operatorname{sech} x$ is nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 \operatorname{sech} x\rfloor-p+1}$.
    /// - If $\operatorname{sech} x$ is nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 \operatorname{sech} x\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=\text{NaN}$
    /// - $f(\infty,p,m)=0.0$
    /// - $f(-\infty,p,m)=0.0$
    /// - $f(\pm0.0,p,m)=1.0$
    ///
    /// Overflow and underflow:
    /// - Since $\operatorname{sech} x\leq 1$, the result never overflows.
    /// - If $0<f(x,p,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,p,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,p,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,p,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is returned
    ///   instead.
    ///
    /// Underflow happens for inputs of magnitude above about $7.4\times10^8$.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::sech_prec_ref`] instead. If
    /// you know that your target precision is the precision of the input, consider using
    /// [`Float::sech_round_ref`] instead. If both of these things are true, consider using
    /// `(&Float).sech()` instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{3/2} \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` and `self` is finite and nonzero, since the hyperbolic secant of a
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
    ///     .sech_prec_round_ref(5, Floor);
    /// assert_eq!(c.to_string(), "0.625");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .sech_prec_round_ref(5, Ceiling);
    /// assert_eq!(c.to_string(), "0.656");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .sech_prec_round_ref(5, Nearest);
    /// assert_eq!(c.to_string(), "0.656");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .sech_prec_round_ref(20, Floor);
    /// assert_eq!(c.to_string(), "0.64805412");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .sech_prec_round_ref(20, Ceiling);
    /// assert_eq!(c.to_string(), "0.64805508");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .sech_prec_round_ref(20, Nearest);
    /// assert_eq!(c.to_string(), "0.64805412");
    /// assert_eq!(o, Less);
    /// ```
    pub fn sech_prec_round_ref(&self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        assert_ne!(prec, 0);
        match &self.0 {
            NaN => (Self::NAN, Equal),
            // sech(+Inf) = sech(-Inf) = 0+
            Infinity { .. } => (Self::ZERO, Equal),
            // sech(+0) = sech(-0) = 1
            Zero { .. } => (Self::one_prec(prec), Equal),
            Finite { .. } => sech_prec_round_normal_ref(self, prec, rm),
        }
    }

    /// Computes $\operatorname{sech} x$, the hyperbolic secant of a [`Float`], rounding the result
    /// to the nearest value of the specified precision. The [`Float`] is taken by value. An
    /// [`Ordering`] is also returned, indicating whether the rounded hyperbolic secant is less
    /// than, equal to, or greater than the exact hyperbolic secant. Although `NaN`s are not
    /// comparable to any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// If the hyperbolic secant is equidistant from two [`Float`]s with the specified precision,
    /// the [`Float`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a
    /// description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = \operatorname{sech} x+\varepsilon.
    /// $$
    /// - If $\operatorname{sech} x$ is zero or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\operatorname{sech} x$ is nonzero, then $|\varepsilon| < 2^{\lfloor\log_2
    ///   \operatorname{sech} x\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p)=\text{NaN}$
    /// - $f(\infty,p)=0.0$
    /// - $f(-\infty,p)=0.0$
    /// - $f(\pm0.0,p)=1.0$
    ///
    /// Overflow and underflow:
    /// - Since $\operatorname{sech} x\leq 1$, the result never overflows.
    /// - If $0<f(x,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::sech_prec_round`] instead. If you know that your target precision is the precision
    /// of the input, consider using [`Float::sech`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{3/2} \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100).0.sech_prec(5);
    /// assert_eq!(c.to_string(), "0.656");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100).0.sech_prec(20);
    /// assert_eq!(c.to_string(), "0.64805412");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn sech_prec(self, prec: u64) -> (Self, Ordering) {
        self.sech_prec_round(prec, Nearest)
    }

    /// Computes $\operatorname{sech} x$, the hyperbolic secant of a [`Float`], rounding the result
    /// to the nearest value of the specified precision. The [`Float`] is taken by reference. An
    /// [`Ordering`] is also returned, indicating whether the rounded hyperbolic secant is less
    /// than, equal to, or greater than the exact hyperbolic secant. Although `NaN`s are not
    /// comparable to any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// If the hyperbolic secant is equidistant from two [`Float`]s with the specified precision,
    /// the [`Float`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a
    /// description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = \operatorname{sech} x+\varepsilon.
    /// $$
    /// - If $\operatorname{sech} x$ is zero or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\operatorname{sech} x$ is nonzero, then $|\varepsilon| < 2^{\lfloor\log_2
    ///   \operatorname{sech} x\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p)=\text{NaN}$
    /// - $f(\infty,p)=0.0$
    /// - $f(-\infty,p)=0.0$
    /// - $f(\pm0.0,p)=1.0$
    ///
    /// Overflow and underflow:
    /// - Since $\operatorname{sech} x\leq 1$, the result never overflows.
    /// - If $0<f(x,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::sech_prec_round_ref`] instead. If you know that your target precision is the
    /// precision of the input, consider using `(&Float).sech()` instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{3/2} \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100).0.sech_prec_ref(5);
    /// assert_eq!(c.to_string(), "0.656");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100).0.sech_prec_ref(20);
    /// assert_eq!(c.to_string(), "0.64805412");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn sech_prec_ref(&self, prec: u64) -> (Self, Ordering) {
        self.sech_prec_round_ref(prec, Nearest)
    }

    /// Computes $\operatorname{sech} x$, the hyperbolic secant of a [`Float`], rounding the result
    /// with the specified rounding mode. The [`Float`] is taken by value. An [`Ordering`] is also
    /// returned, indicating whether the rounded hyperbolic secant is less than, equal to, or
    /// greater than the exact hyperbolic secant. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the precision of the input. See [`RoundingMode`] for a
    /// description of the possible rounding modes.
    ///
    /// $$
    /// f(x,m) = \operatorname{sech} x+\varepsilon.
    /// $$
    /// - If $\operatorname{sech} x$ is zero or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\operatorname{sech} x$ is nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 \operatorname{sech} x\rfloor-p+1}$, where $p$ is the precision of the
    ///   input.
    /// - If $\operatorname{sech} x$ is nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 \operatorname{sech} x\rfloor-p}$, where $p$ is the precision of the
    ///   input.
    ///
    /// If the output has a precision, it is the precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN},m)=\text{NaN}$
    /// - $f(\infty,m)=0.0$
    /// - $f(-\infty,m)=0.0$
    /// - $f(\pm0.0,m)=1.0$
    ///
    /// See the [`Float::sech_prec_round`] documentation for information on overflow and underflow.
    ///
    /// If you want to specify an output precision, consider using [`Float::sech_prec_round`]
    /// instead. If you know you'll be using the `Nearest` rounding mode, consider using
    /// [`Float::sech`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` and `self` is finite and nonzero, since the hyperbolic secant of a
    /// finite nonzero [`Float`] is never exactly representable.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100).0.sech_round(Floor);
    /// assert_eq!(c.to_string(), "0.64805427366388539957497735322564");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100).0.sech_round(Ceiling);
    /// assert_eq!(c.to_string(), "0.64805427366388539957497735322643");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100).0.sech_round(Nearest);
    /// assert_eq!(c.to_string(), "0.64805427366388539957497735322643");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn sech_round(self, rm: RoundingMode) -> (Self, Ordering) {
        let prec = self.significant_bits();
        self.sech_prec_round(prec, rm)
    }

    /// Computes $\operatorname{sech} x$, the hyperbolic secant of a [`Float`], rounding the result
    /// with the specified rounding mode. The [`Float`] is taken by reference. An [`Ordering`] is
    /// also returned, indicating whether the rounded hyperbolic secant is less than, equal to, or
    /// greater than the exact hyperbolic secant. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the precision of the input. See [`RoundingMode`] for a
    /// description of the possible rounding modes.
    ///
    /// $$
    /// f(x,m) = \operatorname{sech} x+\varepsilon.
    /// $$
    /// - If $\operatorname{sech} x$ is zero or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\operatorname{sech} x$ is nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 \operatorname{sech} x\rfloor-p+1}$, where $p$ is the precision of the
    ///   input.
    /// - If $\operatorname{sech} x$ is nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 \operatorname{sech} x\rfloor-p}$, where $p$ is the precision of the
    ///   input.
    ///
    /// If the output has a precision, it is the precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN},m)=\text{NaN}$
    /// - $f(\infty,m)=0.0$
    /// - $f(-\infty,m)=0.0$
    /// - $f(\pm0.0,m)=1.0$
    ///
    /// See the [`Float::sech_prec_round`] documentation for information on overflow and underflow.
    ///
    /// If you want to specify an output precision, consider using [`Float::sech_prec_round_ref`]
    /// instead. If you know you'll be using the `Nearest` rounding mode, consider using
    /// `(&Float).sech()` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` and `self` is finite and nonzero, since the hyperbolic secant of a
    /// finite nonzero [`Float`] is never exactly representable.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100).0.sech_round_ref(Floor);
    /// assert_eq!(c.to_string(), "0.64805427366388539957497735322564");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .sech_round_ref(Ceiling);
    /// assert_eq!(c.to_string(), "0.64805427366388539957497735322643");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .sech_round_ref(Nearest);
    /// assert_eq!(c.to_string(), "0.64805427366388539957497735322643");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn sech_round_ref(&self, rm: RoundingMode) -> (Self, Ordering) {
        self.sech_prec_round_ref(self.significant_bits(), rm)
    }

    /// Computes $\operatorname{sech} x$, the hyperbolic secant of a [`Float`], in place, rounding
    /// the result to the specified precision and with the specified rounding mode. An [`Ordering`]
    /// is returned, indicating whether the rounded hyperbolic secant is less than, equal to, or
    /// greater than the exact hyperbolic secant. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function sets the [`Float`] to `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets \operatorname{sech} x+\varepsilon.
    /// $$
    /// - If $\operatorname{sech} x$ is zero or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\operatorname{sech} x$ is nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 \operatorname{sech} x\rfloor-p+1}$.
    /// - If $\operatorname{sech} x$ is nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 \operatorname{sech} x\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// See the [`Float::sech_prec_round`] documentation for information on special cases and
    /// overflow.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::sech_prec_assign`] instead.
    /// If you know that your target precision is the precision of the input, consider using
    /// [`Float::sech_round_assign`] instead. If both of these things are true, consider using
    /// [`Float::sech_assign`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{3/2} \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` and `self` is finite and nonzero, since the hyperbolic secant of a
    /// finite nonzero [`Float`] is never exactly representable, or if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.sech_prec_round_assign(5, Floor), Less);
    /// assert_eq!(x.to_string(), "0.625");
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.sech_prec_round_assign(5, Ceiling), Greater);
    /// assert_eq!(x.to_string(), "0.656");
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.sech_prec_round_assign(5, Nearest), Greater);
    /// assert_eq!(x.to_string(), "0.656");
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.sech_prec_round_assign(20, Floor), Less);
    /// assert_eq!(x.to_string(), "0.64805412");
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.sech_prec_round_assign(20, Ceiling), Greater);
    /// assert_eq!(x.to_string(), "0.64805508");
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.sech_prec_round_assign(20, Nearest), Less);
    /// assert_eq!(x.to_string(), "0.64805412");
    /// ```
    #[inline]
    pub fn sech_prec_round_assign(&mut self, prec: u64, rm: RoundingMode) -> Ordering {
        let o;
        (*self, o) = self.sech_prec_round_ref(prec, rm);
        o
    }

    /// Computes $\operatorname{sech} x$, the hyperbolic secant of a [`Float`], in place, rounding
    /// the result to the nearest value of the specified precision. An [`Ordering`] is returned,
    /// indicating whether the rounded hyperbolic secant is less than, equal to, or greater than the
    /// exact hyperbolic secant. Although `NaN`s are not comparable to any [`Float`], whenever this
    /// function sets the [`Float`] to `NaN` it also returns `Equal`.
    ///
    /// If the hyperbolic secant is equidistant from two [`Float`]s with the specified precision,
    /// the [`Float`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a
    /// description of the `Nearest` rounding mode.
    ///
    /// $$
    /// x \gets \operatorname{sech} x+\varepsilon.
    /// $$
    /// - If $\operatorname{sech} x$ is zero or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\operatorname{sech} x$ is nonzero, then $|\varepsilon| < 2^{\lfloor\log_2
    ///   \operatorname{sech} x\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// See the [`Float::sech_prec`] documentation for information on special cases, overflow, and
    /// underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::sech_prec_round_assign`] instead. If you know that your target precision is the
    /// precision of the input, consider using [`Float::sech_assign`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{3/2} \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `self.significant_bits()`.
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
    /// assert_eq!(x.sech_prec_assign(5), Greater);
    /// assert_eq!(x.to_string(), "0.656");
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.sech_prec_assign(20), Less);
    /// assert_eq!(x.to_string(), "0.64805412");
    /// ```
    #[inline]
    pub fn sech_prec_assign(&mut self, prec: u64) -> Ordering {
        self.sech_prec_round_assign(prec, Nearest)
    }

    /// Computes $\operatorname{sech} x$, the hyperbolic secant of a [`Float`], in place, rounding
    /// the result with the specified rounding mode. An [`Ordering`] is returned, indicating whether
    /// the rounded hyperbolic secant is less than, equal to, or greater than the exact hyperbolic
    /// secant. Although `NaN`s are not comparable to any [`Float`], whenever this function sets the
    /// [`Float`] to `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the precision of the input. See [`RoundingMode`] for a
    /// description of the possible rounding modes.
    ///
    /// $$
    /// x \gets \operatorname{sech} x+\varepsilon.
    /// $$
    /// - If $\operatorname{sech} x$ is zero or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\operatorname{sech} x$ is nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 \operatorname{sech} x\rfloor-p+1}$, where $p$ is the precision of the
    ///   input.
    /// - If $\operatorname{sech} x$ is nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 \operatorname{sech} x\rfloor-p}$, where $p$ is the precision of the
    ///   input.
    ///
    /// If the output has a precision, it is the precision of the input.
    ///
    /// See the [`Float::sech_round`] documentation for information on special cases, overflow, and
    /// underflow.
    ///
    /// If you want to specify an output precision, consider using [`Float::sech_prec_round_assign`]
    /// instead. If you know you'll be using the `Nearest` rounding mode, consider using
    /// [`Float::sech_assign`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` and `self` is finite and nonzero, since the hyperbolic secant of a
    /// finite nonzero [`Float`] is never exactly representable.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.sech_round_assign(Floor), Less);
    /// assert_eq!(x.to_string(), "0.64805427366388539957497735322564");
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.sech_round_assign(Ceiling), Greater);
    /// assert_eq!(x.to_string(), "0.64805427366388539957497735322643");
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.sech_round_assign(Nearest), Greater);
    /// assert_eq!(x.to_string(), "0.64805427366388539957497735322643");
    /// ```
    #[inline]
    pub fn sech_round_assign(&mut self, rm: RoundingMode) -> Ordering {
        let prec = self.significant_bits();
        self.sech_prec_round_assign(prec, rm)
    }
}

impl Float {
    /// Computes $\operatorname{sech} x$, the hyperbolic secant of a [`Rational`], rounding the
    /// result to the specified precision and with the specified rounding mode and returning the
    /// result as a [`Float`]. The [`Rational`] is taken by value. An [`Ordering`] is also returned,
    /// indicating whether the rounded hyperbolic secant is less than, equal to, or greater than the
    /// exact hyperbolic secant.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \operatorname{sech} x+\varepsilon.
    /// $$
    /// - If $m$ is not `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2 \operatorname{sech}
    ///   x\rfloor-p+1}$.
    /// - If $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2 \operatorname{sech}
    ///   x\rfloor-p}$.
    ///
    /// These bounds do not apply when the result underflows; see below.
    ///
    /// The output has precision `prec`.
    ///
    /// Special cases:
    /// - $f(0,p,m)=1$.
    ///
    /// Overflow and underflow:
    /// - Since $\operatorname{sech} x\leq 1$, the result never overflows.
    /// - If $0<f(x,p,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,p,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,p,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,p,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is returned
    ///   instead.
    ///
    /// Underflow happens for inputs of magnitude above about $7.4\times10^8$.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::sech_rational_prec`] instead.
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
    /// let (c, o) = Float::sech_rational_prec_round(Rational::from_unsigneds(3u8, 5), 5, Floor);
    /// assert_eq!(c.to_string(), "0.812");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::sech_rational_prec_round(Rational::from_unsigneds(3u8, 5), 5, Ceiling);
    /// assert_eq!(c.to_string(), "0.844");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = Float::sech_rational_prec_round(Rational::from_unsigneds(3u8, 5), 20, Floor);
    /// assert_eq!(c.to_string(), "0.84355068");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::sech_rational_prec_round(Rational::from_unsigneds(3u8, 5), 20, Ceiling);
    /// assert_eq!(c.to_string(), "0.84355164");
    /// assert_eq!(o, Greater);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn sech_rational_prec_round(x: Rational, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        Self::sech_rational_prec_round_ref(&x, prec, rm)
    }

    /// Computes $\operatorname{sech} x$, the hyperbolic secant of a [`Rational`], rounding the
    /// result to the specified precision and with the specified rounding mode and returning the
    /// result as a [`Float`]. The [`Rational`] is taken by reference. An [`Ordering`] is also
    /// returned, indicating whether the rounded hyperbolic secant is less than, equal to, or
    /// greater than the exact hyperbolic secant.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \operatorname{sech} x+\varepsilon.
    /// $$
    /// - If $m$ is not `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2 \operatorname{sech}
    ///   x\rfloor-p+1}$.
    /// - If $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2 \operatorname{sech}
    ///   x\rfloor-p}$.
    ///
    /// These bounds do not apply when the result underflows; see below.
    ///
    /// The output has precision `prec`.
    ///
    /// Special cases:
    /// - $f(0,p,m)=1$.
    ///
    /// Overflow and underflow:
    /// - Since $\operatorname{sech} x\leq 1$, the result never overflows.
    /// - If $0<f(x,p,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,p,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,p,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,p,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is returned
    ///   instead.
    ///
    /// Underflow happens for inputs of magnitude above about $7.4\times10^8$.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::sech_rational_prec_ref`]
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
    ///     Float::sech_rational_prec_round_ref(&Rational::from_unsigneds(3u8, 5), 5, Floor);
    /// assert_eq!(c.to_string(), "0.812");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) =
    ///     Float::sech_rational_prec_round_ref(&Rational::from_unsigneds(3u8, 5), 5, Ceiling);
    /// assert_eq!(c.to_string(), "0.844");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) =
    ///     Float::sech_rational_prec_round_ref(&Rational::from_unsigneds(3u8, 5), 20, Floor);
    /// assert_eq!(c.to_string(), "0.84355068");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) =
    ///     Float::sech_rational_prec_round_ref(&Rational::from_unsigneds(3u8, 5), 20, Ceiling);
    /// assert_eq!(c.to_string(), "0.84355164");
    /// assert_eq!(o, Greater);
    /// ```
    pub fn sech_rational_prec_round_ref(
        x: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        assert_ne!(prec, 0);
        if *x == 0u32 {
            // sech(0) = 1, exactly
            return (Self::one_prec(prec), Equal);
        }
        sech_rational_helper(x, prec, rm)
    }

    /// Computes $\operatorname{sech} x$, the hyperbolic secant of a [`Rational`], rounding the
    /// result to the nearest value of the specified precision and returning the result as a
    /// [`Float`]. The [`Rational`] is taken by value. An [`Ordering`] is also returned, indicating
    /// whether the rounded hyperbolic secant is less than, equal to, or greater than the exact
    /// hyperbolic cosine.
    ///
    /// If the hyperbolic secant is equidistant from two [`Float`]s with the specified precision,
    /// the [`Float`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a
    /// description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = \operatorname{sech} x+\varepsilon,
    /// $$
    /// where $|\varepsilon| \leq 2^{\lfloor\log_2 \operatorname{sech} x\rfloor-p}$ (unless the
    /// result underflows; see below).
    ///
    /// The output has precision `prec`.
    ///
    /// Special cases:
    /// - $f(0,p)=1$.
    ///
    /// Overflow and underflow:
    /// - Since $\operatorname{sech} x\leq 1$, the result never overflows.
    /// - If $0<f(x,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::sech_rational_prec_round`] instead.
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
    /// let (c, o) = Float::sech_rational_prec(Rational::from_unsigneds(3u8, 5), 5);
    /// assert_eq!(c.to_string(), "0.844");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = Float::sech_rational_prec(Rational::from_unsigneds(3u8, 5), 20);
    /// assert_eq!(c.to_string(), "0.84355068");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::sech_rational_prec(Rational::ZERO, 10);
    /// assert_eq!(c.to_string(), "1.0000");
    /// assert_eq!(o, Equal);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn sech_rational_prec(x: Rational, prec: u64) -> (Self, Ordering) {
        Self::sech_rational_prec_round_ref(&x, prec, Nearest)
    }

    /// Computes $\operatorname{sech} x$, the hyperbolic secant of a [`Rational`], rounding the
    /// result to the nearest value of the specified precision and returning the result as a
    /// [`Float`]. The [`Rational`] is taken by reference. An [`Ordering`] is also returned,
    /// indicating whether the rounded hyperbolic secant is less than, equal to, or greater than the
    /// exact hyperbolic cosine.
    ///
    /// If the hyperbolic secant is equidistant from two [`Float`]s with the specified precision,
    /// the [`Float`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a
    /// description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = \operatorname{sech} x+\varepsilon,
    /// $$
    /// where $|\varepsilon| \leq 2^{\lfloor\log_2 \operatorname{sech} x\rfloor-p}$ (unless the
    /// result underflows; see below).
    ///
    /// The output has precision `prec`.
    ///
    /// Special cases:
    /// - $f(0,p)=1$.
    ///
    /// Overflow and underflow:
    /// - Since $\operatorname{sech} x\leq 1$, the result never overflows.
    /// - If $0<f(x,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::sech_rational_prec_round_ref`] instead.
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
    /// let (c, o) = Float::sech_rational_prec_ref(&Rational::from_unsigneds(3u8, 5), 5);
    /// assert_eq!(c.to_string(), "0.844");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = Float::sech_rational_prec_ref(&Rational::from_unsigneds(3u8, 5), 20);
    /// assert_eq!(c.to_string(), "0.84355068");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::sech_rational_prec_ref(&Rational::ZERO, 10);
    /// assert_eq!(c.to_string(), "1.0000");
    /// assert_eq!(o, Equal);
    /// ```
    #[inline]
    pub fn sech_rational_prec_ref(x: &Rational, prec: u64) -> (Self, Ordering) {
        Self::sech_rational_prec_round_ref(x, prec, Nearest)
    }
}

impl Sech for Float {
    type Output = Self;

    /// Computes $\operatorname{sech} x$, the hyperbolic secant of a [`Float`], taking it by value.
    ///
    /// If the output has a precision, it is the precision of the input. If the hyperbolic secant is
    /// equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s in
    /// its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// $$
    /// f(x) = \operatorname{sech} x+\varepsilon.
    /// $$
    /// - If $\operatorname{sech} x$ is zero or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\operatorname{sech} x$ is nonzero, then $|\varepsilon| < 2^{\lfloor\log_2
    ///   \operatorname{sech} x\rfloor-p}$, where $p$ is the precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN})=\text{NaN}$
    /// - $f(\infty)=0.0$
    /// - $f(-\infty)=0.0$
    /// - $f(\pm0.0)=1.0$
    ///
    /// See the [`Float::sech_round`] documentation for information on overflow and underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::sech_round`] instead. If you want to specify the output precision, consider using
    /// [`Float::sech_prec`]. If you want both of these things, consider using
    /// [`Float::sech_prec_round`].
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
    /// use malachite_base::num::arithmetic::traits::Sech;
    /// use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity};
    /// use malachite_float::Float;
    ///
    /// assert!(Float::NAN.sech().is_nan());
    /// assert_eq!(Float::INFINITY.sech(), 0);
    /// assert_eq!(Float::NEGATIVE_INFINITY.sech(), 0);
    /// assert_eq!(
    ///     Float::from_unsigned_prec(1u32, 100).0.sech().to_string(),
    ///     "0.64805427366388539957497735322643"
    /// );
    /// ```
    #[inline]
    fn sech(self) -> Self {
        let prec = self.significant_bits();
        self.sech_prec_round(prec, Nearest).0
    }
}

impl Sech for &Float {
    type Output = Float;

    /// Computes $\operatorname{sech} x$, the hyperbolic secant of a [`Float`], taking it by
    /// reference.
    ///
    /// If the output has a precision, it is the precision of the input. If the hyperbolic secant is
    /// equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s in
    /// its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// $$
    /// f(x) = \operatorname{sech} x+\varepsilon.
    /// $$
    /// - If $\operatorname{sech} x$ is zero or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\operatorname{sech} x$ is nonzero, then $|\varepsilon| < 2^{\lfloor\log_2
    ///   \operatorname{sech} x\rfloor-p}$, where $p$ is the precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN})=\text{NaN}$
    /// - $f(\infty)=0.0$
    /// - $f(-\infty)=0.0$
    /// - $f(\pm0.0)=1.0$
    ///
    /// See the [`Float::sech_round`] documentation for information on overflow and underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::sech_round_ref`] instead. If you want to specify the output precision, consider
    /// using [`Float::sech_prec_ref`]. If you want both of these things, consider using
    /// [`Float::sech_prec_round_ref`].
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
    /// use malachite_base::num::arithmetic::traits::Sech;
    /// use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity};
    /// use malachite_float::Float;
    ///
    /// assert!((&Float::NAN).sech().is_nan());
    /// assert_eq!((&Float::INFINITY).sech(), 0);
    /// assert_eq!((&Float::NEGATIVE_INFINITY).sech(), 0);
    /// assert_eq!(
    ///     (&Float::from_unsigned_prec(1u32, 100).0).sech().to_string(),
    ///     "0.64805427366388539957497735322643"
    /// );
    /// ```
    #[inline]
    fn sech(self) -> Float {
        self.sech_prec_round_ref(self.significant_bits(), Nearest).0
    }
}

impl SechAssign for Float {
    /// Computes $\operatorname{sech} x$, the hyperbolic secant of a [`Float`], in place.
    ///
    /// If the output has a precision, it is the precision of the input. If the hyperbolic secant is
    /// equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s in
    /// its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// $$
    /// x \gets \operatorname{sech} x+\varepsilon.
    /// $$
    /// - If $\operatorname{sech} x$ is zero or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\operatorname{sech} x$ is nonzero, then $|\varepsilon| < 2^{\lfloor\log_2
    ///   \operatorname{sech} x\rfloor-p}$, where $p$ is the precision of the input.
    ///
    /// See the [`Float::sech`] documentation for information on special cases, overflow, and
    /// underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::sech_round_assign`] instead. If you want to specify the output precision, consider
    /// using [`Float::sech_prec_assign`]. If you want both of these things, consider using
    /// [`Float::sech_prec_round_assign`].
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
    /// use malachite_base::num::arithmetic::traits::SechAssign;
    /// use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity};
    /// use malachite_float::Float;
    ///
    /// let mut x = Float::NAN;
    /// x.sech_assign();
    /// assert!(x.is_nan());
    ///
    /// let mut x = Float::INFINITY;
    /// x.sech_assign();
    /// assert_eq!(x, 0);
    ///
    /// let mut x = Float::NEGATIVE_INFINITY;
    /// x.sech_assign();
    /// assert_eq!(x, 0);
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// x.sech_assign();
    /// assert_eq!(x.to_string(), "0.64805427366388539957497735322643");
    /// ```
    #[inline]
    fn sech_assign(&mut self) {
        let prec = self.significant_bits();
        self.sech_prec_round_assign(prec, Nearest);
    }
}

/// Computes $\operatorname{sech} x$, the hyperbolic secant of a primitive float. The result is
/// correctly rounded.
///
/// $$
/// f(x) = \operatorname{sech} x+\varepsilon.
/// $$
/// - If $\operatorname{sech} x$ is zero or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
/// - If $\operatorname{sech} x$ is nonzero, then $|\varepsilon| < 2^{\lfloor\log_2
///   \operatorname{sech} x\rfloor-p}$, where $p$ is the precision of the output (typically 24 if
///   `T` is a [`f32`] and 53 if `T` is a [`f64`], but less if the output is subnormal).
///
/// Special cases:
/// - $f(\text{NaN})=\text{NaN}$
/// - $f(\pm\infty)=0.0$
/// - $f(\pm0.0)=1.0$
///
/// Overflow is not possible, since the result lies in $[0, 1]$. An `x` of large magnitude gives a
/// subnormal result, or underflows to `0.0`.
///
/// # Worst-case complexity
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use malachite_base::num::float::NiceFloat;
/// use malachite_float::float::arithmetic::sech::primitive_float_sech;
///
/// assert!(primitive_float_sech(f32::NAN).is_nan());
/// assert_eq!(
///     NiceFloat(primitive_float_sech(f32::INFINITY)),
///     NiceFloat(0.0)
/// );
/// assert_eq!(NiceFloat(primitive_float_sech(-0.0f32)), NiceFloat(1.0));
/// assert_eq!(
///     NiceFloat(primitive_float_sech(1.0f32)),
///     NiceFloat(0.6480543)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_sech(-1.0f64)),
///     NiceFloat(0.6480542736638853)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_sech(720.0f64)),
///     NiceFloat(4.06446160484e-313)
/// );
/// assert_eq!(NiceFloat(primitive_float_sech(746.0f64)), NiceFloat(0.0));
/// ```
#[inline]
#[allow(clippy::type_repetition_in_bounds)]
pub fn primitive_float_sech<T: PrimitiveFloat>(x: T) -> T
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    emulate_float_to_float_fn(Float::sech_prec, x)
}

/// Computes $\operatorname{sech} x$, the hyperbolic secant of a [`Rational`], returning the result
/// as a primitive float. The result is correctly rounded.
///
/// $$
/// f(x) = \operatorname{sech} x+\varepsilon.
/// $$
/// - If $\operatorname{sech} x$ is zero, $\varepsilon$ may be ignored or assumed to be 0.
/// - If $\operatorname{sech} x$ is nonzero, then $|\varepsilon| < 2^{\lfloor\log_2
///   \operatorname{sech} x\rfloor-p}$, where $p$ is the precision of the output (typically 24 if
///   `T` is a [`f32`] and 53 if `T` is a [`f64`], but less if the output is subnormal).
///
/// Special cases:
/// - $f(0)=1$
///
/// Overflow is not possible, since the result lies in $(0, 1]$. An `x` of large magnitude gives a
/// subnormal result, or underflows to `0.0`.
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
/// use malachite_float::float::arithmetic::sech::primitive_float_sech_rational;
/// use malachite_q::Rational;
///
/// assert_eq!(
///     NiceFloat(primitive_float_sech_rational::<f64>(&Rational::ZERO)),
///     NiceFloat(1.0)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_sech_rational::<f64>(
///         &Rational::from_unsigneds(1u8, 3)
///     )),
///     NiceFloat(0.9469052537634979)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_sech_rational::<f64>(
///         &Rational::from_signeds(-1i8, 3)
///     )),
///     NiceFloat(0.9469052537634979)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_sech_rational::<f64>(&Rational::from(10000))),
///     NiceFloat(0.0)
/// );
/// ```
#[inline]
#[allow(clippy::type_repetition_in_bounds)]
pub fn primitive_float_sech_rational<T: PrimitiveFloat>(x: &Rational) -> T
where
    Float: PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    emulate_rational_to_float_fn(Float::sech_rational_prec_ref, x)
}
