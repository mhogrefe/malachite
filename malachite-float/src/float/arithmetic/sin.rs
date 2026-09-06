// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the GNU MPFR Library.
//
//      Copyright © 2001-2025 Free Software Foundation, Inc.
//
//      Contributed by the Pascaline and Caramba projects, INRIA.
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

// Port of MPFR's sine. `mpfr_sin` (`sin.c`) reduces an argument with |x| >= 2 modulo 2 pi using
// `mpfr_remainder`, which also settles the sign of the result, and then computes sin(x) = ±sqrt(1
// - cos(x)^2) from the cosine, all inside a Ziv loop. The `mpfr_sin_fast` tier, used for precisions
// at or above `MPFR_SINCOS_THRESHOLD` and built on `mpfr_sincos_fast`, is not ported yet.

use crate::Float;
use crate::InnerFloat::{Finite, Infinity, NaN, Zero};
use crate::float::arithmetic::cos::{NEAR_ZERO_MIN_CANCEL, TrigStep, trig_near_zero};
use crate::float::arithmetic::round_near_x::float_round_near_x;
use core::cmp::Ordering::{self, Equal};
use core::cmp::{max, min};
use malachite_base::num::arithmetic::traits::{Abs, CeilingLogBase2, NegAssign, Sin, SinAssign};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::{NaN as NaNTrait, One};
use malachite_base::num::comparison::traits::PartialOrdAbs;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{self, Ceiling, Down, Exact, Nearest, Up};
use malachite_nz::natural::arithmetic::float::round::float_can_round;
use malachite_nz::platform::Limb;

// One iteration of the Ziv loop at working precision `m`, which the cancellation checks may raise
// for the next iteration (the caller applies the generic increase on `Retry`).
fn sin_ziv_step(
    x: &Float,
    exp_x: i64,
    prec: u64,
    rm: RoundingMode,
    reduce: bool,
    m: &mut u64,
) -> TrigStep {
    // The near-zero path is taken for a cancellation of at least this many bits.
    let near_zero_threshold = max(NEAR_ZERO_MIN_CANCEL, prec >> 4);
    // first perform argument reduction modulo 2*Pi (if needed), also helps to determine the sign of
    // sin(x)
    let xr;
    let xx = if reduce {
        let c_prec = u64::exact_from(exp_x) + *m - 1;
        let pi = Float::pi_prec(c_prec).0;
        xr = x.ieee_remainder_prec_ref_val(&pi << 1u32, *m).0;
        // The analysis is similar to that of cos.c: |xr - x - 2kPi| <= 2^(2-m). Thus we can decide
        // the sign of sin(x) if xr is at distance at least 2^(2-m) of both 0 and +/-Pi.
        //
        // Since c approximates Pi with an error <= 2^(2-expx-m) <= 2^(-m), it suffices to check
        // that c - |xr| >= 2^(2-m).
        let c = pi.sub_prec_round((&xr).abs(), c_prec, Down).0;
        let threshold = 3 - i64::exact_from(*m);
        if xr == 0u32
            || i64::from(xr.get_exponent().unwrap()) < threshold
            || c == 0u32
            || i64::from(c.get_exponent().unwrap()) < threshold
        {
            // x is within 2^(4-m) of a multiple of pi (if |xr| is small, of 2k pi; if c is small,
            // of (2k + 1) pi), so |sin(x)| < 2^(5-m), and with m already above prec by a margin,
            // the near-zero path resolves the result directly. MPFR instead keeps raising m until
            // the reduced argument is resolved.
            let cancel = *m - 4;
            return if cancel >= near_zero_threshold {
                TrigStep::NearZero(cancel)
            } else {
                TrigStep::Retry
            };
        }
        // |xr - x - 2kPi| <= 2^(2-m), thus |sin(xr) - sin(x)| <= 2^(2-m)
        &xr
    } else {
        // the input argument is already reduced
        x
    };
    let sign = *xx < 0u32;
    // now that the argument is reduced, precision m is enough. c = cos(x) rounded away, squared
    // rounding away, then 1 - c^2 and its square root rounding toward zero
    let c = xx
        .cos_prec_round_ref(*m, Up)
        .0
        .square_prec_round(*m, Ceiling)
        .0;
    let mut c = Float::ONE
        .sub_prec_round(c, *m, Down)
        .0
        .sqrt_prec_round(*m, Down)
        .0;
    if sign {
        c.neg_assign();
    }
    // Warning: c may be 0!
    if c == 0u32 {
        // 1 - cos(xx)^2 rounded to zero, so sin(xx)^2 is below 2^(3-m) and |sin(x)| below 2^(3-m)/2
        // + 2^(2-m)
        let cancel = (*m >> 1).saturating_sub(3);
        if reduce && cancel >= near_zero_threshold {
            return TrigStep::NearZero(cancel);
        }
        // Huge cancellation: increase prec a lot!
        *m = max(*m, x.significant_bits()) << 1;
        return TrigStep::Retry;
    }
    // the absolute error on c is at most 2^(3-m-EXP(c)), plus 2^(2-m) if there was an argument
    // reduction. Since EXP(c) <= 1, 3-m-EXP(c) >= 2-m, thus the error is at most 2^(3-m-EXP(c)) in
    // case of argument reduction.
    let exp_c = i64::from(c.get_exponent().unwrap());
    let err = (exp_c << 1) + i64::exact_from(*m) - 3 - i64::from(reduce);
    if err > 0 && float_can_round(c.significand_ref().unwrap(), u64::exact_from(err), prec, rm) {
        return TrigStep::Done(c);
    }
    // |sin(x)| < 2^bound_exp, since |sin(x)| <= |c| + 2^(4-m-EXP(c))
    let bound_exp = max(exp_c, 4 - i64::exact_from(*m) - exp_c) + 1;
    if reduce && bound_exp < 0 {
        let cancel = u64::exact_from(-bound_exp);
        if cancel >= near_zero_threshold {
            return TrigStep::NearZero(cancel);
        }
    }
    // check for huge cancellation (Near 0)
    if err < i64::exact_from(prec) {
        *m += u64::exact_from(i64::exact_from(prec) - err);
    }
    // MPFR also doubles m here "if near 1", when EXP(c) = 1. That cannot happen: the squared cosine
    // is positive, so 1 - c^2 rounded toward zero is below 1, and so is its square root rounded
    // toward zero.
    assert_ne!(exp_c, 1);
    TrigStep::Retry
}

// This is mpfr_sin from sin.c, MPFR 4.2.2, without the `mpfr_sin_fast` tier for precisions at or
// above `MPFR_SINCOS_THRESHOLD`, which depends on `mpfr_sincos_fast`.
fn sin_prec_round_normal_ref(x: &Float, prec: u64, rm: RoundingMode) -> (Float, Ordering) {
    assert_ne!(rm, Exact, "Inexact sin");
    let exp_x = i64::from(x.get_exponent().unwrap());
    let err1 = -(exp_x << 1);
    // sin(x) = x - x^3/6 + ... so the error is < 2^(3*EXP(x)-2)
    //
    // MPFR_FAST_COMPUTE_IF_SMALL_INPUT (y, x, err1, 2, 0, rnd_mode, {});
    if err1 > 0 {
        let err = u64::exact_from(err1) + 2;
        if err > prec + 1 {
            // The error bound only has to clear prec + 1; passing an enormous err (a tiny x has one
            // around 2^31) would make float_round_near_x do work proportional to it. This can fail
            // to round, for instance for a power of 2 stored at a precision above the error bound,
            // whose bits within the error window are all zero; the general algorithm then takes
            // over, as in MPFR.
            if let Some(result) = float_round_near_x(x, min(err, prec + 2), false, prec, rm) {
                return result;
            }
        }
    }
    // Compute initial precision. For x large, since argument reduction is expensive, we want to
    // avoid any failure in Ziv's strategy, thus we take into account expx too.
    let mut m = prec + max(prec, u64::try_from(exp_x).unwrap_or(0)).ceiling_log_base_2() + 8;
    // since we compute sin(x) as sqrt(1-cos(x)^2), and for x small we have cos(x)^2 ~ 1 - x^2, when
    // subtracting cos(x)^2 from 1 we will lose about -2*expx bits if expx < 0
    if exp_x < 0 {
        m += u64::exact_from(err1);
    }
    // MPFR reduces every |x| >= 2, noting that for 2 <= |x| < pi it could avoid the reduction. For
    // 2 <= |x| < 3, sin(x) has the sign of x and the cosine handles |x| < 4 unreduced, so the
    // reduction (a pi computation and a remainder) is skipped.
    let reduce = exp_x > 2 || (exp_x == 2 && x.ge_abs(&3u32));
    let mut increment = Limb::WIDTH;
    let c = loop {
        match sin_ziv_step(x, exp_x, prec, rm, reduce, &mut m) {
            TrigStep::Done(c) => break c,
            TrigStep::NearZero(cancel) => return trig_near_zero(x, prec, rm, cancel, false),
            TrigStep::Retry => {}
        }
        // ziv_next: Else generic increase
        m += increment;
        increment = m >> 1;
    };
    // inexact cannot be 0, since this would mean that c was representable within the target
    // precision, but in that case mpfr_can_round will fail
    Float::from_float_prec_round(c, prec, rm)
}

impl Float {
    /// Computes $\sin x$, the sine of a [`Float`], rounding the result to the specified precision
    /// and with the specified rounding mode. The [`Float`] is taken by value. An [`Ordering`] is
    /// also returned, indicating whether the rounded sine is less than, equal to, or greater than
    /// the exact sine. Although `NaN`s are not comparable to any [`Float`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \sin x+\varepsilon.
    /// $$
    /// - If $x$ is not finite, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x$ is finite and $m$ is not `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2 |\sin
    ///   x|\rfloor-p+1}$.
    /// - If $x$ is finite and $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2 |\sin
    ///   x|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=\text{NaN}$
    /// - $f(\pm\infty,p,m)=\text{NaN}$
    /// - $f(\pm0.0,p,m)=\pm0.0$
    ///
    /// Overflow and underflow:
    /// - Since $|\sin x|\leq 1$, the result never overflows.
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
    /// Underflow requires an input within $2^{-2^{30}}$ of a nonzero multiple of $\pi$, which takes
    /// more than $2^{30}$ bits of precision, or an input of magnitude $2^{-2^{30}}$, the smallest
    /// positive [`Float`], rounded toward zero.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::sin_prec`] instead. If you
    /// know that your target precision is the precision of the input, consider using
    /// [`Float::sin_round`] instead. If both of these things are true, consider using
    /// [`Float::sin`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m, e) = O(n^{3/2} \log n \log\log n + (n+m+e) (\log (n+m+e))^2 \log\log (n+m+e))$
    ///
    /// $M(n, m, e) = O((n+m+e) \log (n+m+e))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, $m$ is
    /// `self.significant_bits()`, and $e$ is the exponent of `self` (0 if `self` has no exponent or
    /// a negative one): the cosine at working precision $n$, from which the sine is derived, costs
    /// the first term, and for $|x| \geq 4$ the argument is reduced modulo $2\pi$, which requires
    /// $\pi$ to about $n + e$ bits and a remainder of the $m$-bit input. Unlike most functions,
    /// `sin` therefore gets slower as the magnitude of its input grows, not just as the precision
    /// does.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact`, since the sine of a finite nonzero [`Float`] is never exactly
    /// representable, or if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .sin_prec_round(5, Floor);
    /// assert_eq!(c.to_string(), "0.812");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .sin_prec_round(5, Ceiling);
    /// assert_eq!(c.to_string(), "0.844");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .sin_prec_round(5, Nearest);
    /// assert_eq!(c.to_string(), "0.844");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .sin_prec_round(20, Floor);
    /// assert_eq!(c.to_string(), "0.84147072");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .sin_prec_round(20, Ceiling);
    /// assert_eq!(c.to_string(), "0.84147167");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .sin_prec_round(20, Nearest);
    /// assert_eq!(c.to_string(), "0.84147072");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn sin_prec_round(self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        self.sin_prec_round_ref(prec, rm)
    }

    /// Computes $\sin x$, the sine of a [`Float`], rounding the result to the specified precision
    /// and with the specified rounding mode. The [`Float`] is taken by reference. An [`Ordering`]
    /// is also returned, indicating whether the rounded sine is less than, equal to, or greater
    /// than the exact sine. Although `NaN`s are not comparable to any [`Float`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \sin x+\varepsilon.
    /// $$
    /// - If $x$ is not finite, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x$ is finite and $m$ is not `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2 |\sin
    ///   x|\rfloor-p+1}$.
    /// - If $x$ is finite and $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2 |\sin
    ///   x|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=\text{NaN}$
    /// - $f(\pm\infty,p,m)=\text{NaN}$
    /// - $f(\pm0.0,p,m)=\pm0.0$
    ///
    /// Overflow and underflow:
    /// - Since $|\sin x|\leq 1$, the result never overflows.
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
    /// Underflow requires an input within $2^{-2^{30}}$ of a nonzero multiple of $\pi$, which takes
    /// more than $2^{30}$ bits of precision, or an input of magnitude $2^{-2^{30}}$, the smallest
    /// positive [`Float`], rounded toward zero.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::sin_prec_ref`] instead. If
    /// you know that your target precision is the precision of the input, consider using
    /// [`Float::sin_round_ref`] instead. If both of these things are true, consider using
    /// `(&Float).sin()` instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m, e) = O(n^{3/2} \log n \log\log n + (n+m+e) (\log (n+m+e))^2 \log\log (n+m+e))$
    ///
    /// $M(n, m, e) = O((n+m+e) \log (n+m+e))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, $m$ is
    /// `self.significant_bits()`, and $e$ is the exponent of `self` (0 if `self` has no exponent or
    /// a negative one): the cosine at working precision $n$, from which the sine is derived, costs
    /// the first term, and for $|x| \geq 4$ the argument is reduced modulo $2\pi$, which requires
    /// $\pi$ to about $n + e$ bits and a remainder of the $m$-bit input. Unlike most functions,
    /// `sin` therefore gets slower as the magnitude of its input grows, not just as the precision
    /// does.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact`, since the sine of a finite nonzero [`Float`] is never exactly
    /// representable, or if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = (&Float::from_unsigned_prec(1u32, 100).0).sin_prec_round_ref(5, Floor);
    /// assert_eq!(c.to_string(), "0.812");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = (&Float::from_unsigned_prec(1u32, 100).0).sin_prec_round_ref(5, Ceiling);
    /// assert_eq!(c.to_string(), "0.844");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = (&Float::from_unsigned_prec(1u32, 100).0).sin_prec_round_ref(5, Nearest);
    /// assert_eq!(c.to_string(), "0.844");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = (&Float::from_unsigned_prec(1u32, 100).0).sin_prec_round_ref(20, Floor);
    /// assert_eq!(c.to_string(), "0.84147072");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = (&Float::from_unsigned_prec(1u32, 100).0).sin_prec_round_ref(20, Ceiling);
    /// assert_eq!(c.to_string(), "0.84147167");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = (&Float::from_unsigned_prec(1u32, 100).0).sin_prec_round_ref(20, Nearest);
    /// assert_eq!(c.to_string(), "0.84147072");
    /// assert_eq!(o, Less);
    /// ```
    pub fn sin_prec_round_ref(&self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        assert_ne!(prec, 0);
        match &self.0 {
            NaN | Infinity { .. } => (Self::NAN, Equal),
            // sin(+0) = +0, sin(-0) = -0
            Zero { .. } => (self.clone(), Equal),
            Finite { .. } => sin_prec_round_normal_ref(self, prec, rm),
        }
    }

    /// Computes $\sin x$, the sine of a [`Float`], rounding the result to the nearest value of the
    /// specified precision. The [`Float`] is taken by value. An [`Ordering`] is also returned,
    /// indicating whether the rounded sine is less than, equal to, or greater than the exact sine.
    /// Although `NaN`s are not comparable to any [`Float`], whenever this function returns a `NaN`
    /// it also returns `Equal`.
    ///
    /// If the sine is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = \sin x+\varepsilon.
    /// $$
    /// - If $x$ is not finite, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x$ is finite, then $|\varepsilon| < 2^{\lfloor\log_2 |\sin x|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p)=\text{NaN}$
    /// - $f(\pm\infty,p)=\text{NaN}$
    /// - $f(\pm0.0,p)=1.0$
    ///
    /// Overflow and underflow:
    /// - Since $|\sin x|\leq 1$, the result never overflows.
    /// - If $0<f(x,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// Underflow requires an input within $2^{-2^{30}}$ of a nonzero multiple of $\pi$, which takes
    /// more than $2^{30}$ bits of precision, or an input of magnitude $2^{-2^{30}}$, the smallest
    /// positive [`Float`], rounded toward zero.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::sin_prec_round`] instead. If you know that your target precision is the precision
    /// of the input, consider using [`Float::sin`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m, e) = O(n^{3/2} \log n \log\log n + (n+m+e) (\log (n+m+e))^2 \log\log (n+m+e))$
    ///
    /// $M(n, m, e) = O((n+m+e) \log (n+m+e))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, $m$ is
    /// `self.significant_bits()`, and $e$ is the exponent of `self` (0 if `self` has no exponent or
    /// a negative one): the cosine at working precision $n$, from which the sine is derived, costs
    /// the first term, and for $|x| \geq 4$ the argument is reduced modulo $2\pi$, which requires
    /// $\pi$ to about $n + e$ bits and a remainder of the $m$-bit input. Unlike most functions,
    /// `sin` therefore gets slower as the magnitude of its input grows, not just as the precision
    /// does.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100).0.sin_prec(5);
    /// assert_eq!(c.to_string(), "0.844");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100).0.sin_prec(20);
    /// assert_eq!(c.to_string(), "0.84147072");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn sin_prec(self, prec: u64) -> (Self, Ordering) {
        self.sin_prec_round(prec, Nearest)
    }

    /// Computes $\sin x$, the sine of a [`Float`], rounding the result to the nearest value of the
    /// specified precision. The [`Float`] is taken by reference. An [`Ordering`] is also returned,
    /// indicating whether the rounded sine is less than, equal to, or greater than the exact sine.
    /// Although `NaN`s are not comparable to any [`Float`], whenever this function returns a `NaN`
    /// it also returns `Equal`.
    ///
    /// If the sine is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = \sin x+\varepsilon.
    /// $$
    /// - If $x$ is not finite, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x$ is finite, then $|\varepsilon| < 2^{\lfloor\log_2 |\sin x|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p)=\text{NaN}$
    /// - $f(\pm\infty,p)=\text{NaN}$
    /// - $f(\pm0.0,p)=1.0$
    ///
    /// Overflow and underflow:
    /// - Since $|\sin x|\leq 1$, the result never overflows.
    /// - If $0<f(x,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// Underflow requires an input within $2^{-2^{30}}$ of a nonzero multiple of $\pi$, which takes
    /// more than $2^{30}$ bits of precision, or an input of magnitude $2^{-2^{30}}$, the smallest
    /// positive [`Float`], rounded toward zero.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::sin_prec_round_ref`] instead. If you know that your target precision is the
    /// precision of the input, consider using `(&Float).sin()` instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m, e) = O(n^{3/2} \log n \log\log n + (n+m+e) (\log (n+m+e))^2 \log\log (n+m+e))$
    ///
    /// $M(n, m, e) = O((n+m+e) \log (n+m+e))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, $m$ is
    /// `self.significant_bits()`, and $e$ is the exponent of `self` (0 if `self` has no exponent or
    /// a negative one): the cosine at working precision $n$, from which the sine is derived, costs
    /// the first term, and for $|x| \geq 4$ the argument is reduced modulo $2\pi$, which requires
    /// $\pi$ to about $n + e$ bits and a remainder of the $m$-bit input. Unlike most functions,
    /// `sin` therefore gets slower as the magnitude of its input grows, not just as the precision
    /// does.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = (&Float::from_unsigned_prec(1u32, 100).0).sin_prec_ref(5);
    /// assert_eq!(c.to_string(), "0.844");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = (&Float::from_unsigned_prec(1u32, 100).0).sin_prec_ref(20);
    /// assert_eq!(c.to_string(), "0.84147072");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn sin_prec_ref(&self, prec: u64) -> (Self, Ordering) {
        self.sin_prec_round_ref(prec, Nearest)
    }

    /// Computes $\sin x$, the sine of a [`Float`], rounding the result with the specified rounding
    /// mode. The [`Float`] is taken by value. An [`Ordering`] is also returned, indicating whether
    /// the rounded sine is less than, equal to, or greater than the exact sine. Although `NaN`s are
    /// not comparable to any [`Float`], whenever this function returns a `NaN` it also returns
    /// `Equal`.
    ///
    /// The precision of the output is the precision of the input. See [`RoundingMode`] for a
    /// description of the possible rounding modes.
    ///
    /// $$
    /// f(x,m) = \sin x+\varepsilon.
    /// $$
    /// - If $x$ is not finite, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x$ is finite and $m$ is not `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2 |\sin
    ///   x|\rfloor-p+1}$, where $p$ is the precision of the input.
    /// - If $x$ is finite and $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2 |\sin
    ///   x|\rfloor-p}$, where $p$ is the precision of the input.
    ///
    /// If the output has a precision, it is the precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN},m)=\text{NaN}$
    /// - $f(\pm\infty,m)=\text{NaN}$
    /// - $f(\pm0.0,m)=1.0$
    ///
    /// Overflow and underflow:
    /// - Since $|\sin x|\leq 1$, the result never overflows.
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
    /// Underflow requires an input within $2^{-2^{30}}$ of a nonzero multiple of $\pi$, which takes
    /// more than $2^{30}$ bits of precision, or an input of magnitude $2^{-2^{30}}$, the smallest
    /// positive [`Float`], rounded toward zero.
    ///
    /// If you want to specify an output precision, consider using [`Float::sin_prec_round`]
    /// instead. If you know you'll be using the `Nearest` rounding mode, consider using
    /// [`Float::sin`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, e) = O(n^{3/2} \log n \log\log n + (n+e) (\log (n+e))^2 \log\log (n+e))$
    ///
    /// $M(n, e) = O((n+e) \log (n+e))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits()`, and $e$ is
    /// the exponent of `self` (0 if `self` has no exponent or a negative one): the Taylor series at
    /// working precision $n$ costs the first term, and for $|x| \geq 4$ the argument is reduced
    /// modulo $2\pi$, which requires $\pi$ to about $n + e$ bits. Unlike most functions, `sin`
    /// therefore gets slower as the magnitude of its input grows, not just as the precision does.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact`, since the sine of a finite nonzero [`Float`] is never exactly
    /// representable.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100).0.sin_round(Floor);
    /// assert_eq!(c.to_string(), "0.84147098480789650665250232163005");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100).0.sin_round(Ceiling);
    /// assert_eq!(c.to_string(), "0.84147098480789650665250232163084");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100).0.sin_round(Nearest);
    /// assert_eq!(c.to_string(), "0.84147098480789650665250232163005");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn sin_round(self, rm: RoundingMode) -> (Self, Ordering) {
        let prec = self.significant_bits();
        self.sin_prec_round(prec, rm)
    }

    /// Computes $\sin x$, the sine of a [`Float`], rounding the result with the specified rounding
    /// mode. The [`Float`] is taken by reference. An [`Ordering`] is also returned, indicating
    /// whether the rounded sine is less than, equal to, or greater than the exact sine. Although
    /// `NaN`s are not comparable to any [`Float`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// The precision of the output is the precision of the input. See [`RoundingMode`] for a
    /// description of the possible rounding modes.
    ///
    /// $$
    /// f(x,m) = \sin x+\varepsilon.
    /// $$
    /// - If $x$ is not finite, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x$ is finite and $m$ is not `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2 |\sin
    ///   x|\rfloor-p+1}$, where $p$ is the precision of the input.
    /// - If $x$ is finite and $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2 |\sin
    ///   x|\rfloor-p}$, where $p$ is the precision of the input.
    ///
    /// If the output has a precision, it is the precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN},m)=\text{NaN}$
    /// - $f(\pm\infty,m)=\text{NaN}$
    /// - $f(\pm0.0,m)=1.0$
    ///
    /// Overflow and underflow:
    /// - Since $|\sin x|\leq 1$, the result never overflows.
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
    /// Underflow requires an input within $2^{-2^{30}}$ of a nonzero multiple of $\pi$, which takes
    /// more than $2^{30}$ bits of precision, or an input of magnitude $2^{-2^{30}}$, the smallest
    /// positive [`Float`], rounded toward zero.
    ///
    /// If you want to specify an output precision, consider using [`Float::sin_prec_round_ref`]
    /// instead. If you know you'll be using the `Nearest` rounding mode, consider using
    /// `(&Float).sin()` instead.
    ///
    /// # Worst-case complexity
    /// $T(n, e) = O(n^{3/2} \log n \log\log n + (n+e) (\log (n+e))^2 \log\log (n+e))$
    ///
    /// $M(n, e) = O((n+e) \log (n+e))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits()`, and $e$ is
    /// the exponent of `self` (0 if `self` has no exponent or a negative one): the Taylor series at
    /// working precision $n$ costs the first term, and for $|x| \geq 4$ the argument is reduced
    /// modulo $2\pi$, which requires $\pi$ to about $n + e$ bits. Unlike most functions, `sin`
    /// therefore gets slower as the magnitude of its input grows, not just as the precision does.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact`, since the sine of a finite nonzero [`Float`] is never exactly
    /// representable.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = (&Float::from_unsigned_prec(1u32, 100).0).sin_round_ref(Floor);
    /// assert_eq!(c.to_string(), "0.84147098480789650665250232163005");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = (&Float::from_unsigned_prec(1u32, 100).0).sin_round_ref(Ceiling);
    /// assert_eq!(c.to_string(), "0.84147098480789650665250232163084");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = (&Float::from_unsigned_prec(1u32, 100).0).sin_round_ref(Nearest);
    /// assert_eq!(c.to_string(), "0.84147098480789650665250232163005");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn sin_round_ref(&self, rm: RoundingMode) -> (Self, Ordering) {
        self.sin_prec_round_ref(self.significant_bits(), rm)
    }

    /// Computes $\sin x$, the sine of a [`Float`], rounding the result to the specified precision
    /// and with the specified rounding mode. The [`Float`] is replaced by the result, and an
    /// [`Ordering`] is returned, indicating whether the rounded sine is less than, equal to, or
    /// greater than the exact sine. Although `NaN`s are not comparable to any [`Float`], whenever
    /// this function sets a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets \sin x+\varepsilon.
    /// $$
    /// - If $x$ is not finite, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x$ is finite and $m$ is not `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2 |\sin
    ///   x|\rfloor-p+1}$.
    /// - If $x$ is finite and $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2 |\sin
    ///   x|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// See the [`Float::sin_prec_round`] documentation for information on special cases, overflow,
    /// and underflow.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::sin_prec_assign`] instead. If
    /// you know that your target precision is the precision of the input, consider using
    /// [`Float::sin_round_assign`] instead. If both of these things are true, consider using
    /// [`Float::sin_assign`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m, e) = O(n^{3/2} \log n \log\log n + (n+m+e) (\log (n+m+e))^2 \log\log (n+m+e))$
    ///
    /// $M(n, m, e) = O((n+m+e) \log (n+m+e))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, $m$ is
    /// `self.significant_bits()`, and $e$ is the exponent of `self` (0 if `self` has no exponent or
    /// a negative one): the cosine at working precision $n$, from which the sine is derived, costs
    /// the first term, and for $|x| \geq 4$ the argument is reduced modulo $2\pi$, which requires
    /// $\pi$ to about $n + e$ bits and a remainder of the $m$-bit input. Unlike most functions,
    /// `sin` therefore gets slower as the magnitude of its input grows, not just as the precision
    /// does.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact`, since the sine of a finite nonzero [`Float`] is never exactly
    /// representable, or if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.sin_prec_round_assign(5, Floor), Less);
    /// assert_eq!(x.to_string(), "0.812");
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.sin_prec_round_assign(5, Ceiling), Greater);
    /// assert_eq!(x.to_string(), "0.844");
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.sin_prec_round_assign(5, Nearest), Greater);
    /// assert_eq!(x.to_string(), "0.844");
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.sin_prec_round_assign(20, Floor), Less);
    /// assert_eq!(x.to_string(), "0.84147072");
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.sin_prec_round_assign(20, Ceiling), Greater);
    /// assert_eq!(x.to_string(), "0.84147167");
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.sin_prec_round_assign(20, Nearest), Less);
    /// assert_eq!(x.to_string(), "0.84147072");
    /// ```
    #[inline]
    pub fn sin_prec_round_assign(&mut self, prec: u64, rm: RoundingMode) -> Ordering {
        let o;
        (*self, o) = self.sin_prec_round_ref(prec, rm);
        o
    }

    /// Computes $\sin x$, the sine of a [`Float`], rounding the result to the nearest value of the
    /// specified precision. The [`Float`] is replaced by the result, and an [`Ordering`] is
    /// returned, indicating whether the rounded sine is less than, equal to, or greater than the
    /// exact sine. Although `NaN`s are not comparable to any [`Float`], whenever this function sets
    /// a `NaN` it also returns `Equal`.
    ///
    /// If the sine is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// x \gets \sin x+\varepsilon.
    /// $$
    /// - If $x$ is not finite, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x$ is finite, then $|\varepsilon| < 2^{\lfloor\log_2 |\sin x|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// See the [`Float::sin_prec`] documentation for information on special cases, overflow, and
    /// underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::sin_prec_round_assign`] instead. If you know that your target precision is the
    /// precision of the input, consider using [`Float::sin_assign`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m, e) = O(n^{3/2} \log n \log\log n + (n+m+e) (\log (n+m+e))^2 \log\log (n+m+e))$
    ///
    /// $M(n, m, e) = O((n+m+e) \log (n+m+e))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, $m$ is
    /// `self.significant_bits()`, and $e$ is the exponent of `self` (0 if `self` has no exponent or
    /// a negative one): the cosine at working precision $n$, from which the sine is derived, costs
    /// the first term, and for $|x| \geq 4$ the argument is reduced modulo $2\pi$, which requires
    /// $\pi$ to about $n + e$ bits and a remainder of the $m$-bit input. Unlike most functions,
    /// `sin` therefore gets slower as the magnitude of its input grows, not just as the precision
    /// does.
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
    /// assert_eq!(x.sin_prec_assign(5), Greater);
    /// assert_eq!(x.to_string(), "0.844");
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.sin_prec_assign(20), Less);
    /// assert_eq!(x.to_string(), "0.84147072");
    /// ```
    #[inline]
    pub fn sin_prec_assign(&mut self, prec: u64) -> Ordering {
        self.sin_prec_round_assign(prec, Nearest)
    }

    /// Computes $\sin x$, the sine of a [`Float`], rounding the result with the specified rounding
    /// mode. The [`Float`] is replaced by the result, and an [`Ordering`] is returned, indicating
    /// whether the rounded sine is less than, equal to, or greater than the exact sine. Although
    /// `NaN`s are not comparable to any [`Float`], whenever this function sets a `NaN` it also
    /// returns `Equal`.
    ///
    /// The precision of the output is the precision of the input. See [`RoundingMode`] for a
    /// description of the possible rounding modes.
    ///
    /// $$
    /// x \gets \sin x+\varepsilon.
    /// $$
    /// - If $x$ is not finite, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x$ is finite and $m$ is not `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2 |\sin
    ///   x|\rfloor-p+1}$, where $p$ is the precision of the input.
    /// - If $x$ is finite and $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2 |\sin
    ///   x|\rfloor-p}$, where $p$ is the precision of the input.
    ///
    /// If the output has a precision, it is the precision of the input.
    ///
    /// See the [`Float::sin_round`] documentation for information on special cases, overflow, and
    /// underflow.
    ///
    /// If you want to specify an output precision, consider using [`Float::sin_prec_round_assign`]
    /// instead. If you know you'll be using the `Nearest` rounding mode, consider using
    /// [`Float::sin_assign`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, e) = O(n^{3/2} \log n \log\log n + (n+e) (\log (n+e))^2 \log\log (n+e))$
    ///
    /// $M(n, e) = O((n+e) \log (n+e))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits()`, and $e$ is
    /// the exponent of `self` (0 if `self` has no exponent or a negative one): the Taylor series at
    /// working precision $n$ costs the first term, and for $|x| \geq 4$ the argument is reduced
    /// modulo $2\pi$, which requires $\pi$ to about $n + e$ bits. Unlike most functions, `sin`
    /// therefore gets slower as the magnitude of its input grows, not just as the precision does.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact`, since the sine of a finite nonzero [`Float`] is never exactly
    /// representable.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.sin_round_assign(Floor), Less);
    /// assert_eq!(x.to_string(), "0.84147098480789650665250232163005");
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.sin_round_assign(Ceiling), Greater);
    /// assert_eq!(x.to_string(), "0.84147098480789650665250232163084");
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.sin_round_assign(Nearest), Less);
    /// assert_eq!(x.to_string(), "0.84147098480789650665250232163005");
    /// ```
    #[inline]
    pub fn sin_round_assign(&mut self, rm: RoundingMode) -> Ordering {
        let prec = self.significant_bits();
        self.sin_prec_round_assign(prec, rm)
    }
}

impl Sin for Float {
    type Output = Self;

    /// Computes $\sin x$, the sine of a [`Float`], taking it by value.
    ///
    /// If the output has a precision, it is the precision of the input. If the sine is equidistant
    /// from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s in its binary
    /// expansion is chosen. See [`RoundingMode`] for a description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x) = \sin x+\varepsilon.
    /// $$
    /// - If $x$ is not finite, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x$ is finite, then $|\varepsilon| < 2^{\lfloor\log_2 |\sin x|\rfloor-p}$, where $p$ is
    ///   the precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN})=\text{NaN}$
    /// - $f(\pm\infty)=\text{NaN}$
    /// - $f(\pm0.0)=\pm0.0$
    ///
    /// See the [`Float::sin_round`] documentation for information on overflow and underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using [`Float::sin_round`]
    /// instead. If you want to specify the output precision, consider using [`Float::sin_prec`]. If
    /// you want both of these things, consider using [`Float::sin_prec_round`].
    ///
    /// # Worst-case complexity
    /// $T(n, e) = O(n^{3/2} \log n \log\log n + (n+e) (\log (n+e))^2 \log\log (n+e))$
    ///
    /// $M(n, e) = O((n+e) \log (n+e))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits()`, and $e$ is
    /// the exponent of `self` (0 if `self` has no exponent or a negative one): the Taylor series at
    /// working precision $n$ costs the first term, and for $|x| \geq 4$ the argument is reduced
    /// modulo $2\pi$, which requires $\pi$ to about $n + e$ bits. Unlike most functions, `sin`
    /// therefore gets slower as the magnitude of its input grows, not just as the precision does.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Sin;
    /// use malachite_base::num::basic::traits::*;
    /// use malachite_float::Float;
    ///
    /// assert!(Float::NAN.sin().is_nan());
    /// assert!(Float::INFINITY.sin().is_nan());
    /// assert!(Float::NEGATIVE_INFINITY.sin().is_nan());
    /// assert_eq!(Float::ZERO.sin().to_string(), "0.0");
    /// assert_eq!(Float::NEGATIVE_ZERO.sin().to_string(), "-0.0");
    /// assert_eq!(
    ///     Float::from_unsigned_prec(1u32, 100).0.sin().to_string(),
    ///     "0.84147098480789650665250232163005"
    /// );
    /// assert_eq!(
    ///     Float::from_unsigned_prec(100u32, 100).0.sin().to_string(),
    ///     "-0.50636564110975879365655761045969"
    /// );
    /// ```
    #[inline]
    fn sin(self) -> Self {
        let prec = self.significant_bits();
        self.sin_prec_round(prec, Nearest).0
    }
}

impl Sin for &Float {
    type Output = Float;

    /// Computes $\sin x$, the sine of a [`Float`], taking it by reference.
    ///
    /// If the output has a precision, it is the precision of the input. If the sine is equidistant
    /// from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s in its binary
    /// expansion is chosen. See [`RoundingMode`] for a description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x) = \sin x+\varepsilon.
    /// $$
    /// - If $x$ is not finite, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x$ is finite, then $|\varepsilon| < 2^{\lfloor\log_2 |\sin x|\rfloor-p}$, where $p$ is
    ///   the precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN})=\text{NaN}$
    /// - $f(\pm\infty)=\text{NaN}$
    /// - $f(\pm0.0)=\pm0.0$
    ///
    /// See the [`Float::sin_round`] documentation for information on overflow and underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::sin_round_ref`] instead. If you want to specify the output precision, consider
    /// using [`Float::sin_prec_ref`]. If you want both of these things, consider using
    /// [`Float::sin_prec_round_ref`].
    ///
    /// # Worst-case complexity
    /// $T(n, e) = O(n^{3/2} \log n \log\log n + (n+e) (\log (n+e))^2 \log\log (n+e))$
    ///
    /// $M(n, e) = O((n+e) \log (n+e))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits()`, and $e$ is
    /// the exponent of `self` (0 if `self` has no exponent or a negative one): the Taylor series at
    /// working precision $n$ costs the first term, and for $|x| \geq 4$ the argument is reduced
    /// modulo $2\pi$, which requires $\pi$ to about $n + e$ bits. Unlike most functions, `sin`
    /// therefore gets slower as the magnitude of its input grows, not just as the precision does.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Sin;
    /// use malachite_base::num::basic::traits::*;
    /// use malachite_float::Float;
    ///
    /// assert!(Float::NAN.sin().is_nan());
    /// assert!(Float::INFINITY.sin().is_nan());
    /// assert!(Float::NEGATIVE_INFINITY.sin().is_nan());
    /// assert_eq!(Float::ZERO.sin().to_string(), "0.0");
    /// assert_eq!(Float::NEGATIVE_ZERO.sin().to_string(), "-0.0");
    /// assert_eq!(
    ///     (&Float::from_unsigned_prec(1u32, 100).0).sin().to_string(),
    ///     "0.84147098480789650665250232163005"
    /// );
    /// assert_eq!(
    ///     (&Float::from_unsigned_prec(100u32, 100).0)
    ///         .sin()
    ///         .to_string(),
    ///     "-0.50636564110975879365655761045969"
    /// );
    /// ```
    #[inline]
    fn sin(self) -> Float {
        self.sin_prec_round_ref(self.significant_bits(), Nearest).0
    }
}

impl SinAssign for Float {
    /// Computes $\sin x$, the sine of a [`Float`], in place.
    ///
    /// If the output has a precision, it is the precision of the input. If the sine is equidistant
    /// from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s in its binary
    /// expansion is chosen. See [`RoundingMode`] for a description of the `Nearest` rounding mode.
    ///
    /// $$
    /// x \gets \sin x+\varepsilon.
    /// $$
    /// - If $x$ is not finite, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x$ is finite, then $|\varepsilon| < 2^{\lfloor\log_2 |\sin x|\rfloor-p}$, where $p$ is
    ///   the precision of the input.
    ///
    /// See the [`Float::sin`] documentation for information on special cases, overflow, and
    /// underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::sin_round_assign`] instead. If you want to specify the output precision, consider
    /// using [`Float::sin_prec_assign`]. If you want both of these things, consider using
    /// [`Float::sin_prec_round_assign`].
    ///
    /// # Worst-case complexity
    /// $T(n, e) = O(n^{3/2} \log n \log\log n + (n+e) (\log (n+e))^2 \log\log (n+e))$
    ///
    /// $M(n, e) = O((n+e) \log (n+e))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits()`, and $e$ is
    /// the exponent of `self` (0 if `self` has no exponent or a negative one): the Taylor series at
    /// working precision $n$ costs the first term, and for $|x| \geq 4$ the argument is reduced
    /// modulo $2\pi$, which requires $\pi$ to about $n + e$ bits. Unlike most functions, `sin`
    /// therefore gets slower as the magnitude of its input grows, not just as the precision does.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::SinAssign;
    /// use malachite_base::num::basic::traits::*;
    /// use malachite_float::Float;
    ///
    /// let mut x = Float::NAN;
    /// x.sin_assign();
    /// assert!(x.is_nan());
    ///
    /// let mut x = Float::INFINITY;
    /// x.sin_assign();
    /// assert!(x.is_nan());
    ///
    /// let mut x = Float::NEGATIVE_INFINITY;
    /// x.sin_assign();
    /// assert!(x.is_nan());
    ///
    /// let mut x = Float::ZERO;
    /// x.sin_assign();
    /// assert_eq!(x.to_string(), "0.0");
    ///
    /// let mut x = Float::NEGATIVE_ZERO;
    /// x.sin_assign();
    /// assert_eq!(x.to_string(), "-0.0");
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// x.sin_assign();
    /// assert_eq!(x.to_string(), "0.84147098480789650665250232163005");
    ///
    /// let mut x = Float::from_unsigned_prec(100u32, 100).0;
    /// x.sin_assign();
    /// assert_eq!(x.to_string(), "-0.50636564110975879365655761045969");
    /// ```
    #[inline]
    fn sin_assign(&mut self) {
        let prec = self.significant_bits();
        self.sin_prec_round_assign(prec, Nearest);
    }
}
