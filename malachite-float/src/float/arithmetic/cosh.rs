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
use crate::float::arithmetic::exp::one_neighbor;
use crate::float::arithmetic::round_near_x::small_input_shortcut;
use crate::float::conversion::string::set_str::overflow;
use crate::{Float, emulate_float_to_float_fn, emulate_rational_to_float_fn, floor_and_ceiling};
use core::cmp::Ordering::{self, Equal, Greater, Less};
use core::cmp::max;
use malachite_base::fail_on_untested_path;
use malachite_base::num::arithmetic::traits::{
    Abs, AddMul, CeilingLogBase2, Cosh, CoshAssign, Reciprocal, Square,
};
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::{Infinity as InfinityTrait, NaN as NaNTrait, One};
use malachite_base::num::conversion::traits::{ExactFrom, RoundingFrom};
use malachite_base::num::logic::traits::{CountOnes, SignificantBits};
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_nz::natural::arithmetic::float::round::float_can_round;
use malachite_nz::platform::Limb;
use malachite_q::Rational;

// Whether `x` is the largest finite `Float` of its precision, the value at which `Floor` and `Down`
// rounding saturate on overflow. Unlike a comparison with `Float::max_finite_value_with_prec`, this
// allocates nothing.
pub(crate) fn is_max_finite(x: &Float) -> bool {
    x.get_exponent() == Some(Float::MAX_EXPONENT)
        && x.significand_ref().unwrap().count_ones() == x.get_prec().unwrap()
}

// Computes an approximation h of exp(x) / 2 for a positive x so large that exp(x), rounded down to
// precision `working_prec`, reached the top binade of finite Floats: x is at least about
// (MAX_EXPONENT - 1) * log(2). MPFR declares overflow when exp(x) overflows, since it runs with an
// extended exponent range in which that implies that cosh(x) and sinh(x), both about exp(x) / 2,
// overflow the ordinary range too. Malachite has no extended range, so exp(x) can overflow while
// exp(x) / 2 is still finite (a window of width log(2) in x). Instead, write exp(x) / 2 = u * (u /
// 2) with u = exp(x / 2), which does not overflow when exp(x) / 2 doesn't.
//
// Returns `None` if cosh(x) and sinh(x) overflow; otherwise h with |h - exp(x) / 2| < 8 ulp(h): u
// has an error below 1 ulp, so u * (u / 2), rounded once more, has a relative error below 2^(3 -
// working_prec).
fn half_exp_near_overflow(x: &Float, working_prec: u64) -> Option<Float> {
    // x is large, so halving it is exact.
    let u = (x >> 1u32).exp_prec_round(working_prec, Floor).0;
    if u.get_exponent() == Some(Float::MAX_EXPONENT) {
        // exp(x / 2) >= 2^(MAX_EXPONENT - 1), so exp(x) / 2 is far beyond the largest finite Float,
        // and so are cosh(x) and sinh(x), which differ from it by less than 1.
        return None;
    }
    let h = (&u >> 1u32).mul_prec_round(u, working_prec, Floor).0; // <= exp(x) / 2
    if is_max_finite(&h) {
        // exp(x) / 2 >= the largest finite Float at precision `working_prec`, which exceeds the
        // midpoint between the largest finite Float at any lower precision and 2^MAX_EXPONENT by
        // far more than exp(-x) / 2, so cosh(x) and sinh(x), which are exp(x) / 2 +/- exp(-x) / 2,
        // overflow (or, with Floor or Down, saturate) at any lower output precision.
        return None;
    }
    Some(h)
}

// Approximates exp(x) / 2 for a positive finite x at precision `working_prec`. Returns `None` if
// exp(x) / 2 is so large that cosh(x) and sinh(x) overflow at any precision below `working_prec`.
// Otherwise returns h and whether it was computed near the overflow threshold: usually |h - exp(x)
// / 2| < 1 ulp(h), but near the threshold the bound is 8 ulps.
fn half_exp(x: &Float, working_prec: u64) -> Option<(Float, bool)> {
    let exp_x = x.exp_prec_round_ref(working_prec, Floor).0;
    if exp_x.get_exponent() == Some(Float::MAX_EXPONENT) {
        // exp(x) is in the top binade, or overflowed and saturated.
        half_exp_near_overflow(x, working_prec).map(|h| (h, true))
    } else {
        // Halving is exact.
        Some((exp_x >> 1u32, false))
    }
}

// Approximations of sinh(x) and cosh(x) for a positive finite x, computed together at precision
// `working_prec`: the core of `sinh`, `cosh`, and `sinh_cosh`, whose Ziv loops differ only in which
// of the two they need to round. With h = exp(x) / 2, cosh(x) = h + 1 / (4 h) and sinh(x) = h - 1 /
// (4 h); away from the overflow threshold the values are those of MPFR's (e + 1 / e) / 2 and (e - 1
// / e) / 2, where e = exp(x) rounded down. Returns `None` if both overflow at any precision below
// `working_prec`; otherwise the two approximations, each with the number of its bits that are
// correct (its error is below 2^(EXP - bits)), which for sinh(x) may be zero after heavy
// cancellation.
pub(crate) struct HyperbolicApprox {
    pub sinh: Float,
    pub sinh_bits: u64,
    pub cosh: Float,
    pub cosh_bits: u64,
}

pub(crate) fn hyperbolic_approx(x: &Float, working_prec: u64) -> Option<HyperbolicApprox> {
    let (h, near_overflow) = half_exp(x, working_prec)?;
    // exp(-x) / 2 = 1 / (4 h), rounded up. This may underflow, in which case it rounds up to the
    // smallest positive Float, still an upper bound.
    let exp_neg_x_half = h
        .reciprocal_round_ref(Ceiling)
        .0
        .shr_prec_round(2u32, working_prec, Ceiling)
        .0;
    let exp_h = i64::from(h.get_exponent().unwrap());
    let sinh = h.sub_prec_ref_ref(&exp_neg_x_half, working_prec).0;
    // h is not the largest finite Float, so adding a value this small rounds up to at most it.
    let cosh = h.add_round(exp_neg_x_half, Ceiling).0;
    // The difference is not zero: that would need exp(x) to round down to exactly 1, so x < 2^(1 -
    // working_prec), but callers that need sinh(x) raise working_prec above -2 EXP(x).
    let sinh_bits = if sinh == 0u32 {
        0
    } else {
        // The subtraction cancels about EXP(h) - EXP(sinh) bits of h's error, which is below 1 ulp
        // of h, or 8 ulps near the overflow threshold (cf. sinh.c, whose estimate is err = Nt -
        // ceil(log_2(1 + 2^d)) with d = EXP(exp(x)) - EXP(sinh(x)) + 2).
        let d = exp_h - i64::from(sinh.get_exponent().unwrap()) + 3;
        let loss = u64::exact_from(max(d, 0)) + if near_overflow { 4 } else { 1 };
        working_prec.saturating_sub(loss)
    };
    // Away from the threshold, the error of h and the two roundings stay below 8 ulps; near it, h's
    // error is below 8 ulps, and the two roundings add at most 2 more.
    let cosh_bits = working_prec - if near_overflow { 4 } else { 3 };
    Some(HyperbolicApprox {
        sinh,
        sinh_bits,
        cosh,
        cosh_bits,
    })
}

// Whether an approximation with the given number of correct bits can be rounded to `prec` bits.
pub(crate) fn hyperbolic_can_round(f: &Float, bits: u64, prec: u64, rm: RoundingMode) -> bool {
    bits != 0 && float_can_round(f.significand_ref().unwrap(), bits, prec, rm)
}

// This is mpfr_cosh from cosh.c, MPFR 4.2.2, where the input is finite and nonzero.
fn cosh_prec_round_normal_ref(x: &Float, prec: u64, rm: RoundingMode) -> (Float, Ordering) {
    assert_ne!(rm, Exact, "Inexact cosh");
    let exp_x = i64::from(x.get_exponent().unwrap());
    // cosh(x) = 1 + x^2/2 + ... <= 1 + x^2 for x <= 2.9828..., thus the error < 2^(2*EXP(x)). If x
    // >= 1, then EXP(x) >= 1, thus the shortcut always fails.
    if let Some(result) = small_input_shortcut(&Float::ONE, -(exp_x << 1), 0, true, prec, rm) {
        return result;
    }
    let x = x.abs();
    // The optimal number of bits: see algorithms.tex
    let mut working_prec = prec + 3 + prec.ceiling_log_base_2();
    let mut increment = Limb::WIDTH;
    loop {
        let Some(approx) = hyperbolic_approx(&x, working_prec) else {
            return overflow(true, prec, rm);
        };
        if hyperbolic_can_round(&approx.cosh, approx.cosh_bits, prec, rm) {
            return Float::from_float_prec_round(approx.cosh, prec, rm);
        }
        working_prec += increment;
        increment = working_prec >> 1;
    }
}

// Sums the series cosh(x) = 1 + x^2/2! + x^4/4! + ... in `Rational` arithmetic for a nonzero |x| <
// 1 too small to be a `Float`. Every term is positive and each is less than x^2 times the one
// before, so cosh(x) lies strictly between a partial sum S and S + t / (1 - x^2), where t is the
// next term; the bracket is tightened until both ends round the same way. Only reachable for a
// precision beyond 2^31 bits: any smaller precision takes the tiny path.
fn cosh_rational_series(x: &Rational, prec: u64, rm: RoundingMode) -> (Float, Ordering) {
    fail_on_untested_path("cosh_rational_series");
    let x_squared = x.square();
    let tail_factor = (Rational::ONE - &x_squared).reciprocal();
    let mut s = Rational::ONE;
    let mut term = Rational::ONE;
    let mut k = 1u64;
    loop {
        term *= &x_squared;
        term /= Rational::from((k << 1) * ((k << 1) - 1));
        let hi = (&s).add_mul(&term, &tail_factor);
        if let Some(result) = round_bracket(&s, &hi, prec, rm) {
            return result;
        }
        s += &term;
        k += 1;
    }
}

// Computes cosh(x) for a nonzero `Rational` x, rounded to precision `prec` with rounding mode `rm`.
// cosh(x) is transcendental for every nonzero rational x, so the result is never exact.
pub(crate) fn cosh_rational_helper(x: &Rational, prec: u64, rm: RoundingMode) -> (Float, Ordering) {
    assert_ne!(rm, Exact, "Inexact cosh");
    let exp_x = x.floor_log_base_2_abs() + 1; // the MPFR-style exponent of x
    // 0 < cosh(x) - 1 < x^2 < 2^(2 exp_x) for |x| < 1: when that is at most 2^-prec, half an ulp of
    // 1, cosh(x) rounds to 1, or to its successor for rounding away from zero.
    if -(exp_x << 1) >= i64::exact_from(prec) {
        return match rm {
            Ceiling | Up => (one_neighbor(prec, true), Greater),
            _ => (Float::one_prec(prec), Less),
        };
    }
    // x is too small to be a `Float` but `prec` is so large that cosh(x) does not round to 1.
    if exp_x <= Float::MIN_EXPONENT_I64 {
        return cosh_rational_series(x, prec, rm);
    }
    // |x| >= 2^(MAX_EXPONENT - 1), so cosh(x) > e^|x| / 2 overflows. Smaller x that still overflow
    // are caught by `cosh_prec_round_normal_ref` in the loop below.
    if exp_x >= Float::MAX_EXPONENT_I64 {
        return overflow(true, prec, rm);
    }
    // cosh is even and increasing on [0, infinity), so bracket |x| between the Floats x_lo <= |x|
    // <= x_hi, take the hyperbolic cosine of both, and increase the working precision until the two
    // round to the same result, which the exact cosh(x), lying between them, must then share.
    let x_abs = x.abs();
    monotone_rational_via_floats(&x_abs, prec, rm, cosh_prec_round_normal_ref)
}

// Given the roundings of the two ends of a bracket known to contain a value strictly inside it, the
// rounding of the value, if the ends round to the same `Float` on the same side. An end whose
// rounding is exact settles nothing, since the value lies beyond it.
pub(crate) fn same_rounding(
    (y_lo, o_lo): (Float, Ordering),
    (y_hi, o_hi): (Float, Ordering),
) -> Option<(Float, Ordering)> {
    (o_lo == o_hi && o_lo != Equal && y_lo == y_hi).then_some((y_lo, o_lo))
}

// Computes f(x) for a `Rational` x, rounded to precision `prec` with rounding mode `rm`, where `f`
// computes f for a finite nonzero `Float`, f is monotonic on an interval containing x and the
// `Float`s next to it, and f of a finite nonzero `Float` is never exact. x is bracketed between the
// `Float`s x_lo <= x <= x_hi, f is taken at both, and the working precision is increased until the
// two round to the same result, which the exact f(x), lying between them, must then share. An x
// that is exactly representable at the working precision is passed to `f` directly.
pub(crate) fn monotone_rational_via_floats<
    F: Fn(&Float, u64, RoundingMode) -> (Float, Ordering),
>(
    x: &Rational,
    prec: u64,
    rm: RoundingMode,
    f: F,
) -> (Float, Ordering) {
    let mut working_prec = prec + 10;
    let mut increment = Limb::WIDTH;
    loop {
        let (x_lo, x_o) = Float::from_rational_prec_round_ref(x, working_prec, Floor);
        if x_o == Equal {
            return f(&x_lo, prec, rm);
        }
        let (x_lo, x_hi) = floor_and_ceiling((x_lo, x_o));
        // Both orderings are `Less` or `Greater`, never `Equal`.
        if let Some(result) = same_rounding(f(&x_lo, prec, rm), f(&x_hi, prec, rm)) {
            return result;
        }
        working_prec += increment;
        increment = working_prec >> 1;
    }
}

impl Float {
    /// Computes $\cosh x$, the hyperbolic cosine of a [`Float`], rounding the result to the
    /// specified precision and with the specified rounding mode. The [`Float`] is taken by value.
    /// An [`Ordering`] is also returned, indicating whether the rounded hyperbolic cosine is less
    /// than, equal to, or greater than the exact hyperbolic cosine. Although `NaN`s are not
    /// comparable to any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \cosh x+\varepsilon.
    /// $$
    /// - If $\cosh x$ is infinite or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $\cosh x$ is finite, and $m$ is not `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2
    ///   \cosh x\rfloor-p+1}$.
    /// - If $\cosh x$ is finite, and $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   \cosh x\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=\text{NaN}$
    /// - $f(\infty,p,m)=\infty$
    /// - $f(-\infty,p,m)=\infty$
    /// - $f(\pm0.0,p,m)=1.0$
    ///
    /// Overflow:
    /// - If $f(x,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$ is
    ///   returned instead.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::cosh_prec`] instead. If you
    /// know that your target precision is the precision of the input, consider using
    /// [`Float::cosh_round`] instead. If both of these things are true, consider using
    /// [`Float::cosh`] instead.
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
    /// Panics if `rm` is `Exact` and `self` is finite and nonzero, since the hyperbolic cosine of a
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
    ///     .cosh_prec_round(5, Floor);
    /// assert_eq!(c.to_string(), "1.50");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .cosh_prec_round(5, Ceiling);
    /// assert_eq!(c.to_string(), "1.56");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .cosh_prec_round(5, Nearest);
    /// assert_eq!(c.to_string(), "1.56");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .cosh_prec_round(20, Floor);
    /// assert_eq!(c.to_string(), "1.5430794");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .cosh_prec_round(20, Ceiling);
    /// assert_eq!(c.to_string(), "1.5430813");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .cosh_prec_round(20, Nearest);
    /// assert_eq!(c.to_string(), "1.5430813");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn cosh_prec_round(self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        self.cosh_prec_round_ref(prec, rm)
    }

    /// Computes $\cosh x$, the hyperbolic cosine of a [`Float`], rounding the result to the
    /// specified precision and with the specified rounding mode. The [`Float`] is taken by
    /// reference. An [`Ordering`] is also returned, indicating whether the rounded hyperbolic
    /// cosine is less than, equal to, or greater than the exact hyperbolic cosine. Although `NaN`s
    /// are not comparable to any [`Float`], whenever this function returns a `NaN` it also returns
    /// `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \cosh x+\varepsilon.
    /// $$
    /// - If $\cosh x$ is infinite or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $\cosh x$ is finite, and $m$ is not `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2
    ///   \cosh x\rfloor-p+1}$.
    /// - If $\cosh x$ is finite, and $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   \cosh x\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=\text{NaN}$
    /// - $f(\infty,p,m)=\infty$
    /// - $f(-\infty,p,m)=\infty$
    /// - $f(\pm0.0,p,m)=1.0$
    ///
    /// Overflow:
    /// - If $f(x,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$ is
    ///   returned instead.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::cosh_prec_ref`] instead. If
    /// you know that your target precision is the precision of the input, consider using
    /// [`Float::cosh_round_ref`] instead. If both of these things are true, consider using
    /// `(&Float).cosh()` instead.
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
    /// Panics if `rm` is `Exact` and `self` is finite and nonzero, since the hyperbolic cosine of a
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
    ///     .cosh_prec_round_ref(5, Floor);
    /// assert_eq!(c.to_string(), "1.50");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .cosh_prec_round_ref(5, Ceiling);
    /// assert_eq!(c.to_string(), "1.56");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .cosh_prec_round_ref(5, Nearest);
    /// assert_eq!(c.to_string(), "1.56");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .cosh_prec_round_ref(20, Floor);
    /// assert_eq!(c.to_string(), "1.5430794");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .cosh_prec_round_ref(20, Ceiling);
    /// assert_eq!(c.to_string(), "1.5430813");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .cosh_prec_round_ref(20, Nearest);
    /// assert_eq!(c.to_string(), "1.5430813");
    /// assert_eq!(o, Greater);
    /// ```
    pub fn cosh_prec_round_ref(&self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        assert_ne!(prec, 0);
        match &self.0 {
            NaN => (Self::NAN, Equal),
            Infinity { .. } => (Self::INFINITY, Equal),
            // cosh(+0) = cosh(-0) = 1
            Zero { .. } => (Self::one_prec(prec), Equal),
            Finite { .. } => cosh_prec_round_normal_ref(self, prec, rm),
        }
    }

    /// Computes $\cosh x$, the hyperbolic cosine of a [`Float`], rounding the result to the nearest
    /// value of the specified precision. The [`Float`] is taken by value. An [`Ordering`] is also
    /// returned, indicating whether the rounded hyperbolic cosine is less than, equal to, or
    /// greater than the exact hyperbolic cosine. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// If the hyperbolic cosine is equidistant from two [`Float`]s with the specified precision,
    /// the [`Float`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a
    /// description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = \cosh x+\varepsilon.
    /// $$
    /// - If $\cosh x$ is infinite or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $\cosh x$ is finite, then $|\varepsilon| < 2^{\lfloor\log_2 \cosh x\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p)=\text{NaN}$
    /// - $f(\infty,p)=\infty$
    /// - $f(-\infty,p)=\infty$
    /// - $f(\pm0.0,p)=1.0$
    ///
    /// Overflow:
    /// - If $f(x,p)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::cosh_prec_round`] instead. If you know that your target precision is the precision
    /// of the input, consider using [`Float::cosh`] instead.
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
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100).0.cosh_prec(5);
    /// assert_eq!(c.to_string(), "1.56");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100).0.cosh_prec(20);
    /// assert_eq!(c.to_string(), "1.5430813");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn cosh_prec(self, prec: u64) -> (Self, Ordering) {
        self.cosh_prec_round(prec, Nearest)
    }

    /// Computes $\cosh x$, the hyperbolic cosine of a [`Float`], rounding the result to the nearest
    /// value of the specified precision. The [`Float`] is taken by reference. An [`Ordering`] is
    /// also returned, indicating whether the rounded hyperbolic cosine is less than, equal to, or
    /// greater than the exact hyperbolic cosine. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// If the hyperbolic cosine is equidistant from two [`Float`]s with the specified precision,
    /// the [`Float`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a
    /// description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = \cosh x+\varepsilon.
    /// $$
    /// - If $\cosh x$ is infinite or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $\cosh x$ is finite, then $|\varepsilon| < 2^{\lfloor\log_2 \cosh x\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p)=\text{NaN}$
    /// - $f(\infty,p)=\infty$
    /// - $f(-\infty,p)=\infty$
    /// - $f(\pm0.0,p)=1.0$
    ///
    /// Overflow:
    /// - If $f(x,p)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::cosh_prec_round_ref`] instead. If you know that your target precision is the
    /// precision of the input, consider using `(&Float).cosh()` instead.
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
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100).0.cosh_prec_ref(5);
    /// assert_eq!(c.to_string(), "1.56");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100).0.cosh_prec_ref(20);
    /// assert_eq!(c.to_string(), "1.5430813");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn cosh_prec_ref(&self, prec: u64) -> (Self, Ordering) {
        self.cosh_prec_round_ref(prec, Nearest)
    }

    /// Computes $\cosh x$, the hyperbolic cosine of a [`Float`], rounding the result with the
    /// specified rounding mode. The [`Float`] is taken by value. An [`Ordering`] is also returned,
    /// indicating whether the rounded hyperbolic cosine is less than, equal to, or greater than the
    /// exact hyperbolic cosine. Although `NaN`s are not comparable to any [`Float`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the precision of the input. See [`RoundingMode`] for a
    /// description of the possible rounding modes.
    ///
    /// $$
    /// f(x,m) = \cosh x+\varepsilon.
    /// $$
    /// - If $\cosh x$ is infinite or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $\cosh x$ is finite, and $m$ is not `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2
    ///   \cosh x\rfloor-p+1}$, where $p$ is the precision of the input.
    /// - If $\cosh x$ is finite, and $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   \cosh x\rfloor-p}$, where $p$ is the precision of the input.
    ///
    /// If the output has a precision, it is the precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN},m)=\text{NaN}$
    /// - $f(\infty,m)=\infty$
    /// - $f(-\infty,m)=\infty$
    /// - $f(\pm0.0,m)=1.0$
    ///
    /// See the [`Float::cosh_prec_round`] documentation for information on overflow.
    ///
    /// If you want to specify an output precision, consider using [`Float::cosh_prec_round`]
    /// instead. If you know you'll be using the `Nearest` rounding mode, consider using
    /// [`Float::cosh`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` and `self` is finite and nonzero, since the hyperbolic cosine of a
    /// finite nonzero [`Float`] is never exactly representable.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100).0.cosh_round(Floor);
    /// assert_eq!(c.to_string(), "1.5430806348152437784779056207559");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100).0.cosh_round(Ceiling);
    /// assert_eq!(c.to_string(), "1.5430806348152437784779056207575");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100).0.cosh_round(Nearest);
    /// assert_eq!(c.to_string(), "1.5430806348152437784779056207575");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn cosh_round(self, rm: RoundingMode) -> (Self, Ordering) {
        let prec = self.significant_bits();
        self.cosh_prec_round(prec, rm)
    }

    /// Computes $\cosh x$, the hyperbolic cosine of a [`Float`], rounding the result with the
    /// specified rounding mode. The [`Float`] is taken by reference. An [`Ordering`] is also
    /// returned, indicating whether the rounded hyperbolic cosine is less than, equal to, or
    /// greater than the exact hyperbolic cosine. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the precision of the input. See [`RoundingMode`] for a
    /// description of the possible rounding modes.
    ///
    /// $$
    /// f(x,m) = \cosh x+\varepsilon.
    /// $$
    /// - If $\cosh x$ is infinite or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $\cosh x$ is finite, and $m$ is not `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2
    ///   \cosh x\rfloor-p+1}$, where $p$ is the precision of the input.
    /// - If $\cosh x$ is finite, and $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   \cosh x\rfloor-p}$, where $p$ is the precision of the input.
    ///
    /// If the output has a precision, it is the precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN},m)=\text{NaN}$
    /// - $f(\infty,m)=\infty$
    /// - $f(-\infty,m)=\infty$
    /// - $f(\pm0.0,m)=1.0$
    ///
    /// See the [`Float::cosh_prec_round`] documentation for information on overflow.
    ///
    /// If you want to specify an output precision, consider using [`Float::cosh_prec_round_ref`]
    /// instead. If you know you'll be using the `Nearest` rounding mode, consider using
    /// `(&Float).cosh()` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` and `self` is finite and nonzero, since the hyperbolic cosine of a
    /// finite nonzero [`Float`] is never exactly representable.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100).0.cosh_round_ref(Floor);
    /// assert_eq!(c.to_string(), "1.5430806348152437784779056207559");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .cosh_round_ref(Ceiling);
    /// assert_eq!(c.to_string(), "1.5430806348152437784779056207575");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .cosh_round_ref(Nearest);
    /// assert_eq!(c.to_string(), "1.5430806348152437784779056207575");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn cosh_round_ref(&self, rm: RoundingMode) -> (Self, Ordering) {
        self.cosh_prec_round_ref(self.significant_bits(), rm)
    }

    /// Computes $\cosh x$, the hyperbolic cosine of a [`Float`], in place, rounding the result to
    /// the specified precision and with the specified rounding mode. An [`Ordering`] is returned,
    /// indicating whether the rounded hyperbolic cosine is less than, equal to, or greater than the
    /// exact hyperbolic cosine. Although `NaN`s are not comparable to any [`Float`], whenever this
    /// function sets the [`Float`] to `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets \cosh x+\varepsilon.
    /// $$
    /// - If $\cosh x$ is infinite or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $\cosh x$ is finite, and $m$ is not `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2
    ///   \cosh x\rfloor-p+1}$.
    /// - If $\cosh x$ is finite, and $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   \cosh x\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// See the [`Float::cosh_prec_round`] documentation for information on special cases and
    /// overflow.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::cosh_prec_assign`] instead.
    /// If you know that your target precision is the precision of the input, consider using
    /// [`Float::cosh_round_assign`] instead. If both of these things are true, consider using
    /// [`Float::cosh_assign`] instead.
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
    /// Panics if `rm` is `Exact` and `self` is finite and nonzero, since the hyperbolic cosine of a
    /// finite nonzero [`Float`] is never exactly representable, or if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.cosh_prec_round_assign(5, Floor), Less);
    /// assert_eq!(x.to_string(), "1.50");
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.cosh_prec_round_assign(5, Ceiling), Greater);
    /// assert_eq!(x.to_string(), "1.56");
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.cosh_prec_round_assign(5, Nearest), Greater);
    /// assert_eq!(x.to_string(), "1.56");
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.cosh_prec_round_assign(20, Floor), Less);
    /// assert_eq!(x.to_string(), "1.5430794");
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.cosh_prec_round_assign(20, Ceiling), Greater);
    /// assert_eq!(x.to_string(), "1.5430813");
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.cosh_prec_round_assign(20, Nearest), Greater);
    /// assert_eq!(x.to_string(), "1.5430813");
    /// ```
    #[inline]
    pub fn cosh_prec_round_assign(&mut self, prec: u64, rm: RoundingMode) -> Ordering {
        let o;
        (*self, o) = self.cosh_prec_round_ref(prec, rm);
        o
    }

    /// Computes $\cosh x$, the hyperbolic cosine of a [`Float`], in place, rounding the result to
    /// the nearest value of the specified precision. An [`Ordering`] is returned, indicating
    /// whether the rounded hyperbolic cosine is less than, equal to, or greater than the exact
    /// hyperbolic cosine. Although `NaN`s are not comparable to any [`Float`], whenever this
    /// function sets the [`Float`] to `NaN` it also returns `Equal`.
    ///
    /// If the hyperbolic cosine is equidistant from two [`Float`]s with the specified precision,
    /// the [`Float`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a
    /// description of the `Nearest` rounding mode.
    ///
    /// $$
    /// x \gets \cosh x+\varepsilon.
    /// $$
    /// - If $\cosh x$ is infinite or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $\cosh x$ is finite, then $|\varepsilon| < 2^{\lfloor\log_2 \cosh x\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// See the [`Float::cosh_prec`] documentation for information on special cases and overflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::cosh_prec_round_assign`] instead. If you know that your target precision is the
    /// precision of the input, consider using [`Float::cosh_assign`] instead.
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
    /// assert_eq!(x.cosh_prec_assign(5), Greater);
    /// assert_eq!(x.to_string(), "1.56");
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.cosh_prec_assign(20), Greater);
    /// assert_eq!(x.to_string(), "1.5430813");
    /// ```
    #[inline]
    pub fn cosh_prec_assign(&mut self, prec: u64) -> Ordering {
        self.cosh_prec_round_assign(prec, Nearest)
    }

    /// Computes $\cosh x$, the hyperbolic cosine of a [`Float`], in place, rounding the result with
    /// the specified rounding mode. An [`Ordering`] is returned, indicating whether the rounded
    /// hyperbolic cosine is less than, equal to, or greater than the exact hyperbolic cosine.
    /// Although `NaN`s are not comparable to any [`Float`], whenever this function sets the
    /// [`Float`] to `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the precision of the input. See [`RoundingMode`] for a
    /// description of the possible rounding modes.
    ///
    /// $$
    /// x \gets \cosh x+\varepsilon.
    /// $$
    /// - If $\cosh x$ is infinite or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $\cosh x$ is finite, and $m$ is not `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2
    ///   \cosh x\rfloor-p+1}$, where $p$ is the precision of the input.
    /// - If $\cosh x$ is finite, and $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   \cosh x\rfloor-p}$, where $p$ is the precision of the input.
    ///
    /// If the output has a precision, it is the precision of the input.
    ///
    /// See the [`Float::cosh_round`] documentation for information on special cases and overflow.
    ///
    /// If you want to specify an output precision, consider using [`Float::cosh_prec_round_assign`]
    /// instead. If you know you'll be using the `Nearest` rounding mode, consider using
    /// [`Float::cosh_assign`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` and `self` is finite and nonzero, since the hyperbolic cosine of a
    /// finite nonzero [`Float`] is never exactly representable.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.cosh_round_assign(Floor), Less);
    /// assert_eq!(x.to_string(), "1.5430806348152437784779056207559");
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.cosh_round_assign(Ceiling), Greater);
    /// assert_eq!(x.to_string(), "1.5430806348152437784779056207575");
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.cosh_round_assign(Nearest), Greater);
    /// assert_eq!(x.to_string(), "1.5430806348152437784779056207575");
    /// ```
    #[inline]
    pub fn cosh_round_assign(&mut self, rm: RoundingMode) -> Ordering {
        let prec = self.significant_bits();
        self.cosh_prec_round_assign(prec, rm)
    }
}

impl Float {
    /// Computes $\cosh x$, the hyperbolic cosine of a [`Rational`], rounding the result to the
    /// specified precision and with the specified rounding mode and returning the result as a
    /// [`Float`]. The [`Rational`] is taken by value. An [`Ordering`] is also returned, indicating
    /// whether the rounded hyperbolic cosine is less than, equal to, or greater than the exact
    /// hyperbolic cosine.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \cosh x+\varepsilon.
    /// $$
    /// - If $m$ is not `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2 \cosh x\rfloor-p+1}$.
    /// - If $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2 \cosh x\rfloor-p}$.
    ///
    /// These bounds do not apply when the result overflows; see below.
    ///
    /// The output has precision `prec`.
    ///
    /// Special cases:
    /// - $f(0,p,m)=1$.
    ///
    /// Overflow:
    /// - If $f(x,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$ is
    ///   returned instead.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::cosh_rational_prec`] instead.
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
    /// let (c, o) = Float::cosh_rational_prec_round(Rational::from_unsigneds(3u8, 5), 5, Floor);
    /// assert_eq!(c.to_string(), "1.12");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::cosh_rational_prec_round(Rational::from_unsigneds(3u8, 5), 5, Ceiling);
    /// assert_eq!(c.to_string(), "1.19");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = Float::cosh_rational_prec_round(Rational::from_unsigneds(3u8, 5), 20, Floor);
    /// assert_eq!(c.to_string(), "1.1854649");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::cosh_rational_prec_round(Rational::from_unsigneds(3u8, 5), 20, Ceiling);
    /// assert_eq!(c.to_string(), "1.1854668");
    /// assert_eq!(o, Greater);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn cosh_rational_prec_round(x: Rational, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        Self::cosh_rational_prec_round_ref(&x, prec, rm)
    }

    /// Computes $\cosh x$, the hyperbolic cosine of a [`Rational`], rounding the result to the
    /// specified precision and with the specified rounding mode and returning the result as a
    /// [`Float`]. The [`Rational`] is taken by reference. An [`Ordering`] is also returned,
    /// indicating whether the rounded hyperbolic cosine is less than, equal to, or greater than the
    /// exact hyperbolic cosine.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \cosh x+\varepsilon.
    /// $$
    /// - If $m$ is not `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2 \cosh x\rfloor-p+1}$.
    /// - If $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2 \cosh x\rfloor-p}$.
    ///
    /// These bounds do not apply when the result overflows; see below.
    ///
    /// The output has precision `prec`.
    ///
    /// Special cases:
    /// - $f(0,p,m)=1$.
    ///
    /// Overflow:
    /// - If $f(x,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$ is
    ///   returned instead.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::cosh_rational_prec_ref`]
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
    ///     Float::cosh_rational_prec_round_ref(&Rational::from_unsigneds(3u8, 5), 5, Floor);
    /// assert_eq!(c.to_string(), "1.12");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) =
    ///     Float::cosh_rational_prec_round_ref(&Rational::from_unsigneds(3u8, 5), 5, Ceiling);
    /// assert_eq!(c.to_string(), "1.19");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) =
    ///     Float::cosh_rational_prec_round_ref(&Rational::from_unsigneds(3u8, 5), 20, Floor);
    /// assert_eq!(c.to_string(), "1.1854649");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) =
    ///     Float::cosh_rational_prec_round_ref(&Rational::from_unsigneds(3u8, 5), 20, Ceiling);
    /// assert_eq!(c.to_string(), "1.1854668");
    /// assert_eq!(o, Greater);
    /// ```
    pub fn cosh_rational_prec_round_ref(
        x: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        assert_ne!(prec, 0);
        if *x == 0u32 {
            // cosh(0) = 1, exactly
            return (Self::one_prec(prec), Equal);
        }
        cosh_rational_helper(x, prec, rm)
    }

    /// Computes $\cosh x$, the hyperbolic cosine of a [`Rational`], rounding the result to the
    /// nearest value of the specified precision and returning the result as a [`Float`]. The
    /// [`Rational`] is taken by value. An [`Ordering`] is also returned, indicating whether the
    /// rounded hyperbolic cosine is less than, equal to, or greater than the exact hyperbolic
    /// cosine.
    ///
    /// If the hyperbolic cosine is equidistant from two [`Float`]s with the specified precision,
    /// the [`Float`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a
    /// description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = \cosh x+\varepsilon,
    /// $$
    /// where $|\varepsilon| \leq 2^{\lfloor\log_2 \cosh x\rfloor-p}$ (unless the result overflows;
    /// see below).
    ///
    /// The output has precision `prec`.
    ///
    /// Special cases:
    /// - $f(0,p)=1$.
    ///
    /// Overflow:
    /// - If $f(x,p)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::cosh_rational_prec_round`] instead.
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
    /// let (c, o) = Float::cosh_rational_prec(Rational::from_unsigneds(3u8, 5), 5);
    /// assert_eq!(c.to_string(), "1.19");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = Float::cosh_rational_prec(Rational::from_unsigneds(3u8, 5), 20);
    /// assert_eq!(c.to_string(), "1.1854649");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::cosh_rational_prec(Rational::ZERO, 10);
    /// assert_eq!(c.to_string(), "1.0000");
    /// assert_eq!(o, Equal);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn cosh_rational_prec(x: Rational, prec: u64) -> (Self, Ordering) {
        Self::cosh_rational_prec_round_ref(&x, prec, Nearest)
    }

    /// Computes $\cosh x$, the hyperbolic cosine of a [`Rational`], rounding the result to the
    /// nearest value of the specified precision and returning the result as a [`Float`]. The
    /// [`Rational`] is taken by reference. An [`Ordering`] is also returned, indicating whether the
    /// rounded hyperbolic cosine is less than, equal to, or greater than the exact hyperbolic
    /// cosine.
    ///
    /// If the hyperbolic cosine is equidistant from two [`Float`]s with the specified precision,
    /// the [`Float`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a
    /// description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = \cosh x+\varepsilon,
    /// $$
    /// where $|\varepsilon| \leq 2^{\lfloor\log_2 \cosh x\rfloor-p}$ (unless the result overflows;
    /// see below).
    ///
    /// The output has precision `prec`.
    ///
    /// Special cases:
    /// - $f(0,p)=1$.
    ///
    /// Overflow:
    /// - If $f(x,p)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::cosh_rational_prec_round_ref`] instead.
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
    /// let (c, o) = Float::cosh_rational_prec_ref(&Rational::from_unsigneds(3u8, 5), 5);
    /// assert_eq!(c.to_string(), "1.19");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = Float::cosh_rational_prec_ref(&Rational::from_unsigneds(3u8, 5), 20);
    /// assert_eq!(c.to_string(), "1.1854649");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::cosh_rational_prec_ref(&Rational::ZERO, 10);
    /// assert_eq!(c.to_string(), "1.0000");
    /// assert_eq!(o, Equal);
    /// ```
    #[inline]
    pub fn cosh_rational_prec_ref(x: &Rational, prec: u64) -> (Self, Ordering) {
        Self::cosh_rational_prec_round_ref(x, prec, Nearest)
    }
}

impl Cosh for Float {
    type Output = Self;

    /// Computes $\cosh x$, the hyperbolic cosine of a [`Float`], taking it by value.
    ///
    /// If the output has a precision, it is the precision of the input. If the hyperbolic cosine is
    /// equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s in
    /// its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// $$
    /// f(x) = \cosh x+\varepsilon.
    /// $$
    /// - If $\cosh x$ is infinite or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $\cosh x$ is finite, then $|\varepsilon| < 2^{\lfloor\log_2 \cosh x\rfloor-p}$, where
    ///   $p$ is the precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN})=\text{NaN}$
    /// - $f(\infty)=\infty$
    /// - $f(-\infty)=\infty$
    /// - $f(\pm0.0)=1.0$
    ///
    /// See the [`Float::cosh_round`] documentation for information on overflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::cosh_round`] instead. If you want to specify the output precision, consider using
    /// [`Float::cosh_prec`]. If you want both of these things, consider using
    /// [`Float::cosh_prec_round`].
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
    /// use malachite_base::num::arithmetic::traits::Cosh;
    /// use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity};
    /// use malachite_float::Float;
    ///
    /// assert!(Float::NAN.cosh().is_nan());
    /// assert_eq!(Float::INFINITY.cosh(), Float::INFINITY);
    /// assert_eq!(Float::NEGATIVE_INFINITY.cosh(), Float::INFINITY);
    /// assert_eq!(
    ///     Float::from_unsigned_prec(1u32, 100).0.cosh().to_string(),
    ///     "1.5430806348152437784779056207575"
    /// );
    /// ```
    #[inline]
    fn cosh(self) -> Self {
        let prec = self.significant_bits();
        self.cosh_prec_round(prec, Nearest).0
    }
}

impl Cosh for &Float {
    type Output = Float;

    /// Computes $\cosh x$, the hyperbolic cosine of a [`Float`], taking it by reference.
    ///
    /// If the output has a precision, it is the precision of the input. If the hyperbolic cosine is
    /// equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s in
    /// its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// $$
    /// f(x) = \cosh x+\varepsilon.
    /// $$
    /// - If $\cosh x$ is infinite or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $\cosh x$ is finite, then $|\varepsilon| < 2^{\lfloor\log_2 \cosh x\rfloor-p}$, where
    ///   $p$ is the precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN})=\text{NaN}$
    /// - $f(\infty)=\infty$
    /// - $f(-\infty)=\infty$
    /// - $f(\pm0.0)=1.0$
    ///
    /// See the [`Float::cosh_round`] documentation for information on overflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::cosh_round_ref`] instead. If you want to specify the output precision, consider
    /// using [`Float::cosh_prec_ref`]. If you want both of these things, consider using
    /// [`Float::cosh_prec_round_ref`].
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
    /// use malachite_base::num::arithmetic::traits::Cosh;
    /// use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity};
    /// use malachite_float::Float;
    ///
    /// assert!((&Float::NAN).cosh().is_nan());
    /// assert_eq!((&Float::INFINITY).cosh(), Float::INFINITY);
    /// assert_eq!((&Float::NEGATIVE_INFINITY).cosh(), Float::INFINITY);
    /// assert_eq!(
    ///     (&Float::from_unsigned_prec(1u32, 100).0).cosh().to_string(),
    ///     "1.5430806348152437784779056207575"
    /// );
    /// ```
    #[inline]
    fn cosh(self) -> Float {
        self.cosh_prec_round_ref(self.significant_bits(), Nearest).0
    }
}

impl CoshAssign for Float {
    /// Computes $\cosh x$, the hyperbolic cosine of a [`Float`], in place.
    ///
    /// If the output has a precision, it is the precision of the input. If the hyperbolic cosine is
    /// equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s in
    /// its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// $$
    /// x \gets \cosh x+\varepsilon.
    /// $$
    /// - If $\cosh x$ is infinite or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $\cosh x$ is finite, then $|\varepsilon| < 2^{\lfloor\log_2 \cosh x\rfloor-p}$, where
    ///   $p$ is the precision of the input.
    ///
    /// See the [`Float::cosh`] documentation for information on special cases and overflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::cosh_round_assign`] instead. If you want to specify the output precision, consider
    /// using [`Float::cosh_prec_assign`]. If you want both of these things, consider using
    /// [`Float::cosh_prec_round_assign`].
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
    /// use malachite_base::num::arithmetic::traits::CoshAssign;
    /// use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity};
    /// use malachite_float::Float;
    ///
    /// let mut x = Float::NAN;
    /// x.cosh_assign();
    /// assert!(x.is_nan());
    ///
    /// let mut x = Float::INFINITY;
    /// x.cosh_assign();
    /// assert_eq!(x, Float::INFINITY);
    ///
    /// let mut x = Float::NEGATIVE_INFINITY;
    /// x.cosh_assign();
    /// assert_eq!(x, Float::INFINITY);
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// x.cosh_assign();
    /// assert_eq!(x.to_string(), "1.5430806348152437784779056207575");
    /// ```
    #[inline]
    fn cosh_assign(&mut self) {
        let prec = self.significant_bits();
        self.cosh_prec_round_assign(prec, Nearest);
    }
}

/// Computes $\cosh x$, the hyperbolic cosine of a primitive float. The result is correctly rounded.
///
/// $$
/// f(x) = \cosh x+\varepsilon.
/// $$
/// - If $\cosh x$ is infinite or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
/// - If $\cosh x$ is finite, then $|\varepsilon| < 2^{\lfloor\log_2 \cosh x\rfloor-p}$, where $p$
///   is the precision of the output (24 if `T` is a [`f32`] and 53 if `T` is a [`f64`]).
///
/// Special cases:
/// - $f(\text{NaN})=\text{NaN}$
/// - $f(\infty)=\infty$
/// - $f(-\infty)=\infty$
/// - $f(\pm0.0)=1.0$
///
/// Overflow is possible: an `x` of large magnitude gives $\infty$. Since $\cosh x\geq 1$, the
/// result never underflows.
///
/// # Worst-case complexity
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use malachite_base::num::basic::traits::NegativeInfinity;
/// use malachite_base::num::float::NiceFloat;
/// use malachite_float::float::arithmetic::cosh::primitive_float_cosh;
///
/// assert!(primitive_float_cosh(f32::NAN).is_nan());
/// assert_eq!(
///     NiceFloat(primitive_float_cosh(f32::INFINITY)),
///     NiceFloat(f32::INFINITY)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_cosh(f32::NEGATIVE_INFINITY)),
///     NiceFloat(f32::INFINITY)
/// );
/// assert_eq!(NiceFloat(primitive_float_cosh(0.0f32)), NiceFloat(1.0));
/// assert_eq!(
///     NiceFloat(primitive_float_cosh(1.0f32)),
///     NiceFloat(1.5430807)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_cosh(-1.0f32)),
///     NiceFloat(1.5430807)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_cosh(100.0f32)),
///     NiceFloat(f32::INFINITY)
/// );
/// ```
#[inline]
#[allow(clippy::type_repetition_in_bounds)]
pub fn primitive_float_cosh<T: PrimitiveFloat>(x: T) -> T
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    emulate_float_to_float_fn(Float::cosh_prec, x)
}

/// Computes $\cosh x$, the hyperbolic cosine of a [`Rational`], returning the result as a primitive
/// float. The result is correctly rounded.
///
/// $$
/// f(x) = \cosh x+\varepsilon.
/// $$
/// - If $\cosh x$ is infinite, $\varepsilon$ may be ignored or assumed to be 0.
/// - If $\cosh x$ is finite, then $|\varepsilon| < 2^{\lfloor\log_2 \cosh x\rfloor-p}$, where $p$
///   is the precision of the output (24 if `T` is a [`f32`] and 53 if `T` is a [`f64`]).
///
/// Special cases:
/// - $f(0)=1$
///
/// Overflow is possible: an `x` of large magnitude gives $\infty$. Since $\cosh x\geq 1$, the
/// result never underflows.
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
/// use malachite_float::float::arithmetic::cosh::primitive_float_cosh_rational;
/// use malachite_q::Rational;
///
/// assert_eq!(
///     NiceFloat(primitive_float_cosh_rational::<f64>(&Rational::ZERO)),
///     NiceFloat(1.0)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_cosh_rational::<f64>(
///         &Rational::from_unsigneds(1u8, 3)
///     )),
///     NiceFloat(1.0560718678299394)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_cosh_rational::<f64>(
///         &Rational::from_signeds(-1i8, 3)
///     )),
///     NiceFloat(1.0560718678299394)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_cosh_rational::<f64>(&Rational::from(10000))),
///     NiceFloat(f64::INFINITY)
/// );
/// ```
#[inline]
#[allow(clippy::type_repetition_in_bounds)]
pub fn primitive_float_cosh_rational<T: PrimitiveFloat>(x: &Rational) -> T
where
    Float: PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    emulate_rational_to_float_fn(Float::cosh_rational_prec_ref, x)
}
