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
use crate::float::arithmetic::asinh::{
    ln_of_large_rational_sum, ln_of_large_sum, round_with_error, square_may_overflow,
};
use crate::float::arithmetic::cosh::{monotone_rational_via_floats, same_rounding};
use crate::{Float, emulate_float_to_float_fn, emulate_rational_to_float_fn};
use core::cmp::Ordering::{self, *};
use core::cmp::max;
use malachite_base::fail_on_untested_path;
use malachite_base::num::arithmetic::traits::{
    Acosh, AcoshAssign, CeilingLogBase2, Ln, Sqrt, Square,
};
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::{
    Infinity as InfinityTrait, NaN as NaNTrait, One, Zero as ZeroTrait,
};
use malachite_base::num::conversion::traits::{ExactFrom, RoundingFrom};
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_nz::platform::Limb;
use malachite_q::Rational;

// This is mpfr_acosh from acosh.c, MPFR 4.2.2, where the input is finite and greater than 1.
//
// MPFR squares x in an extended exponent range, where its check for an overflowing x^2 never fires
// for an input in the ordinary range. Here x^2 overflows once EXP(x) exceeds MAX_EXPONENT / 2, and
// with `Floor` it would saturate to the largest finite `Float` and silently give a wrong result, so
// those inputs are detected by their exponent and go through `ln_of_large_sum`, as MPFR's overflow
// branch would.
fn acosh_prec_round_normal_ref(x: &Float, prec: u64, rm: RoundingMode) -> (Float, Ordering) {
    assert_ne!(rm, Exact, "Inexact acosh");
    let large = square_may_overflow(x);
    // the optimal number of bits: see algorithms.tex
    let mut working_prec = prec + 4 + prec.ceiling_log_base_2();
    let mut increment = Limb::WIDTH;
    loop {
        let (t, err) = if large {
            // As x is very large, acosh(x) is ln(2x) to well within the working precision, computed
            // as ln(x) + ln(2), since 2x can overflow. The error is below 2 ulps of t.
            (ln_of_large_sum(x, working_prec, false), 1)
        } else {
            // x^2
            let sq = x.square_prec_round_ref(working_prec, Floor).0;
            let exp_te = i64::from(sq.get_exponent().unwrap());
            // x^2 - 1
            let t = sq.sub_round(Float::ONE, Floor).0;
            if t == 0u32 {
                // This means that x is very close to 1: x = 1 + t with t < 2^(-working_prec). We
                // have acosh(x) = sqrt(2t) (1 - eps(t)) with 0 < eps(t) < t / 12.
                let t = x.sub_prec_round_ref_val(Float::ONE, working_prec, Floor).0;
                // sqrt(2t)
                ((t << 1u32).sqrt(), 1)
            } else {
                let d = exp_te - i64::from(t.get_exponent().unwrap());
                let t = t.sqrt().add_prec_val_ref(x, working_prec).0.ln();
                // error estimate: see algorithms.tex. The error is bounded by 1/2 + 2^err <=
                // 2^max(0, 1 + err).
                let err = 3 + max(1, d) - i64::from(t.get_exponent().unwrap());
                (t, max(0, 1 + err))
            }
        };
        if let Some(result) = round_with_error(t, working_prec, err, prec, rm) {
            return result;
        }
        working_prec += increment;
        increment = working_prec >> 1;
    }
}

// Computes acosh(1 + t) for a positive `Rational` t below 2^-(prec + 3), if the bracket sqrt(2t -
// t^2/3) < acosh(1 + t) < sqrt(2t) decides the rounding. The series acosh(1 + t) = sqrt(2t) (1 -
// t/12 + 3t^2/160 - ...), whose terms alternate in sign and decrease in magnitude for t < 2, gives
// sqrt(2t) (1 - t/12) < acosh(1 + t), and 2t - t^2/3 = 2t (1 - t/6) < 2t (1 - t/12)^2. Both ends
// are square roots of `Rational`s, so they are rounded directly, underflow included.
fn acosh_rational_near_one(t: &Rational, prec: u64, rm: RoundingMode) -> Option<(Float, Ordering)> {
    let two_t = t << 1u32;
    let lower = &two_t - t.square() / const { Rational::const_from_unsigned(3) };
    let (y, o) = Float::sqrt_rational_prec_round(two_t, prec, rm);
    let hi = if o == Equal {
        // sqrt(2t) = y is exactly representable, and acosh(1 + t) lies just below it, so it rounds
        // like any number in the half-ulp below y, such as y minus a quarter-ulp. (The predecessor
        // of the smallest positive `Float` is not available, so that case falls back.)
        if y.get_exponent().unwrap() <= Float::MIN_EXPONENT {
            fail_on_untested_path(
                "acosh_rational_near_one, exact sqrt(2t) at the bottom of the range",
            );
            return None;
        }
        let mut z = y;
        z.set_prec(prec + 2);
        z.decrement();
        Float::from_float_prec_round(z, prec, rm)
    } else {
        (y, o)
    };
    same_rounding(Float::sqrt_rational_prec_round(lower, prec, rm), hi)
}

// Computes acosh(x) for a `Rational` x greater than 1, rounded to precision `prec` with rounding
// mode `rm`. The result is never exactly representable, so `rm` must not be `Exact`.
fn acosh_rational_helper(x: &Rational, prec: u64, rm: RoundingMode) -> (Float, Ordering) {
    assert_ne!(rm, Exact, "Inexact acosh");
    let exp_x = x.floor_log_base_2_abs() + 1; // the MPFR-style exponent of x
    if exp_x > Float::MAX_EXPONENT_I64 {
        // acosh(x) = ln(2x) + c with -1/x^2 < c < 0, since 1 - u/2 <= (1 + sqrt(1 - u)) / 2 < 1 for
        // u = 1/x^2
        return ln_of_large_rational_sum(x, exp_x, prec, rm, false);
    }
    // x < 2 is necessary for x - 1 < 2^-(prec + 3)
    if exp_x == 1 {
        let t = x - Rational::ONE;
        if -(t.floor_log_base_2_abs() + 1) > i64::exact_from(prec) + 3
            && let Some(result) = acosh_rational_near_one(&t, prec, rm)
        {
            return result;
        }
    }
    // acosh is increasing, so bracket x between the Floats x_lo <= x <= x_hi, take the inverse
    // hyperbolic cosine of both, and increase the working precision until the two round to the same
    // result, which the exact acosh(x), lying between them, must then share. x_lo may be 1, whose
    // exact result 0 never matches.
    monotone_rational_via_floats(x, prec, rm, Float::acosh_prec_round_ref)
}

impl Float {
    /// Computes $\operatorname{acosh} x$, the inverse hyperbolic cosine of a [`Float`], rounding
    /// the result to the specified precision and with the specified rounding mode. The [`Float`] is
    /// taken by value. An [`Ordering`] is also returned, indicating whether the rounded inverse
    /// hyperbolic cosine is less than, equal to, or greater than the exact inverse hyperbolic
    /// cosine. Although `NaN`s are not comparable to any [`Float`], whenever this function returns
    /// a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \operatorname{acosh} x+\varepsilon.
    /// $$
    /// - If $\operatorname{acosh} x$ is infinite, zero, or NaN, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $\operatorname{acosh} x$ is finite and nonzero, and $m$ is not `Nearest`, then
    ///   $|\varepsilon| < 2^{\lfloor\log_2 \operatorname{acosh} x\rfloor-p+1}$.
    /// - If $\operatorname{acosh} x$ is finite and nonzero, and $m$ is `Nearest`, then
    ///   $|\varepsilon| \leq 2^{\lfloor\log_2 \operatorname{acosh} x\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=\text{NaN}$
    /// - $f(\infty,p,m)=\infty$
    /// - $f(-\infty,p,m)=\text{NaN}$
    /// - $f(\pm0.0,p,m)=\text{NaN}$
    /// - $f(1,p,m)=0.0$
    /// - $f(x,p,m)=\text{NaN}$ if $x<1$
    ///
    /// The result never overflows, since $\operatorname{acosh} x < \ln 2x$. It underflows only for
    /// an $x$ within $2^{-2^{31}}$ of 1, since $\operatorname{acosh}(1+t) > \sqrt t$, and such an
    /// $x$ needs a precision above $2^{31}$.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::acosh_prec`] instead. If you
    /// know that your target precision is the precision of the input, consider using
    /// [`Float::acosh_round`] instead. If both of these things are true, consider using
    /// [`Float::acosh`] instead.
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
    /// Panics if `rm` is `Exact` and `self` is finite and greater than 1, since the inverse
    /// hyperbolic cosine of such a [`Float`] is never exactly representable, or if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = Float::from_unsigned_prec(2u32, 100)
    ///     .0
    ///     .acosh_prec_round(5, Floor);
    /// assert_eq!(c.to_string(), "1.31");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::from_unsigned_prec(2u32, 100)
    ///     .0
    ///     .acosh_prec_round(5, Ceiling);
    /// assert_eq!(c.to_string(), "1.38");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = Float::from_unsigned_prec(2u32, 100)
    ///     .0
    ///     .acosh_prec_round(5, Nearest);
    /// assert_eq!(c.to_string(), "1.31");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::from_unsigned_prec(2u32, 100)
    ///     .0
    ///     .acosh_prec_round(20, Floor);
    /// assert_eq!(c.to_string(), "1.3169575");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::from_unsigned_prec(2u32, 100)
    ///     .0
    ///     .acosh_prec_round(20, Ceiling);
    /// assert_eq!(c.to_string(), "1.3169594");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = Float::from_unsigned_prec(2u32, 100)
    ///     .0
    ///     .acosh_prec_round(20, Nearest);
    /// assert_eq!(c.to_string(), "1.3169575");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn acosh_prec_round(self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        self.acosh_prec_round_ref(prec, rm)
    }

    /// Computes $\operatorname{acosh} x$, the inverse hyperbolic cosine of a [`Float`], rounding
    /// the result to the specified precision and with the specified rounding mode. The [`Float`] is
    /// taken by reference. An [`Ordering`] is also returned, indicating whether the rounded inverse
    /// hyperbolic cosine is less than, equal to, or greater than the exact inverse hyperbolic
    /// cosine. Although `NaN`s are not comparable to any [`Float`], whenever this function returns
    /// a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \operatorname{acosh} x+\varepsilon.
    /// $$
    /// - If $\operatorname{acosh} x$ is infinite, zero, or NaN, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $\operatorname{acosh} x$ is finite and nonzero, and $m$ is not `Nearest`, then
    ///   $|\varepsilon| < 2^{\lfloor\log_2 \operatorname{acosh} x\rfloor-p+1}$.
    /// - If $\operatorname{acosh} x$ is finite and nonzero, and $m$ is `Nearest`, then
    ///   $|\varepsilon| \leq 2^{\lfloor\log_2 \operatorname{acosh} x\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=\text{NaN}$
    /// - $f(\infty,p,m)=\infty$
    /// - $f(-\infty,p,m)=\text{NaN}$
    /// - $f(\pm0.0,p,m)=\text{NaN}$
    /// - $f(1,p,m)=0.0$
    /// - $f(x,p,m)=\text{NaN}$ if $x<1$
    ///
    /// The result never overflows, since $\operatorname{acosh} x < \ln 2x$. It underflows only for
    /// an $x$ within $2^{-2^{31}}$ of 1, since $\operatorname{acosh}(1+t) > \sqrt t$, and such an
    /// $x$ needs a precision above $2^{31}$.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::acosh_prec_ref`] instead. If
    /// you know that your target precision is the precision of the input, consider using
    /// [`Float::acosh_round_ref`] instead. If both of these things are true, consider using
    /// `(&Float).acosh()` instead.
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
    /// Panics if `rm` is `Exact` and `self` is finite and greater than 1, since the inverse
    /// hyperbolic cosine of such a [`Float`] is never exactly representable, or if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = (&Float::from_unsigned_prec(2u32, 100).0).acosh_prec_round_ref(5, Floor);
    /// assert_eq!(c.to_string(), "1.31");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = (&Float::from_unsigned_prec(2u32, 100).0).acosh_prec_round_ref(5, Ceiling);
    /// assert_eq!(c.to_string(), "1.38");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = (&Float::from_unsigned_prec(2u32, 100).0).acosh_prec_round_ref(5, Nearest);
    /// assert_eq!(c.to_string(), "1.31");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = (&Float::from_unsigned_prec(2u32, 100).0).acosh_prec_round_ref(20, Floor);
    /// assert_eq!(c.to_string(), "1.3169575");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = (&Float::from_unsigned_prec(2u32, 100).0).acosh_prec_round_ref(20, Ceiling);
    /// assert_eq!(c.to_string(), "1.3169594");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = (&Float::from_unsigned_prec(2u32, 100).0).acosh_prec_round_ref(20, Nearest);
    /// assert_eq!(c.to_string(), "1.3169575");
    /// assert_eq!(o, Less);
    /// ```
    pub fn acosh_prec_round_ref(&self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        assert_ne!(prec, 0);
        match &self.0 {
            // acosh(inf) = inf
            Infinity { sign: true } => (Self::INFINITY, Equal),
            // acosh is NaN for NaN, -inf, ±0, and any x < 1
            NaN | Infinity { .. } | Zero { .. } => (Self::NAN, Equal),
            Finite { .. } => match self.partial_cmp(&1u32).unwrap() {
                Less => (Self::NAN, Equal),
                // acosh(1) = +0
                Equal => (Self::ZERO, Equal),
                Greater => acosh_prec_round_normal_ref(self, prec, rm),
            },
        }
    }

    /// Computes $\operatorname{acosh} x$, the inverse hyperbolic cosine of a [`Float`], rounding
    /// the result to the nearest value of the specified precision. The [`Float`] is taken by value.
    /// An [`Ordering`] is also returned, indicating whether the rounded inverse hyperbolic cosine
    /// is less than, equal to, or greater than the exact inverse hyperbolic cosine. Although `NaN`s
    /// are not comparable to any [`Float`], whenever this function returns a `NaN` it also returns
    /// `Equal`.
    ///
    /// If the inverse hyperbolic cosine is equidistant from two [`Float`]s with the specified
    /// precision, the [`Float`] with fewer 1s in its binary expansion is chosen. See
    /// [`RoundingMode`] for a description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = \operatorname{acosh} x+\varepsilon.
    /// $$
    /// - If $\operatorname{acosh} x$ is infinite, zero, or NaN, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $\operatorname{acosh} x$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2
    ///   |\operatorname{acosh} x|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p)=\text{NaN}$
    /// - $f(\infty,p)=\infty$
    /// - $f(-\infty,p)=\text{NaN}$
    /// - $f(\pm0.0,p)=\text{NaN}$
    /// - $f(1,p)=0.0$
    /// - $f(x,p)=\text{NaN}$ if $x<1$
    ///
    /// The result never overflows, since $\operatorname{acosh} x < \ln 2x$. It underflows only for
    /// an $x$ within $2^{-2^{31}}$ of 1, since $\operatorname{acosh}(1+t) > \sqrt t$, and such an
    /// $x$ needs a precision above $2^{31}$.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::acosh_prec_round`] instead. If you know that your target precision is the precision
    /// of the input, consider using [`Float::acosh`] instead.
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
    /// let (c, o) = Float::from_unsigned_prec(2u32, 100).0.acosh_prec(5);
    /// assert_eq!(c.to_string(), "1.31");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::from_unsigned_prec(2u32, 100).0.acosh_prec(20);
    /// assert_eq!(c.to_string(), "1.3169575");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn acosh_prec(self, prec: u64) -> (Self, Ordering) {
        self.acosh_prec_round(prec, Nearest)
    }

    /// Computes $\operatorname{acosh} x$, the inverse hyperbolic cosine of a [`Float`], rounding
    /// the result to the nearest value of the specified precision. The [`Float`] is taken by
    /// reference. An [`Ordering`] is also returned, indicating whether the rounded inverse
    /// hyperbolic cosine is less than, equal to, or greater than the exact inverse hyperbolic
    /// cosine. Although `NaN`s are not comparable to any [`Float`], whenever this function returns
    /// a `NaN` it also returns `Equal`.
    ///
    /// If the inverse hyperbolic cosine is equidistant from two [`Float`]s with the specified
    /// precision, the [`Float`] with fewer 1s in its binary expansion is chosen. See
    /// [`RoundingMode`] for a description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = \operatorname{acosh} x+\varepsilon.
    /// $$
    /// - If $\operatorname{acosh} x$ is infinite, zero, or NaN, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $\operatorname{acosh} x$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2
    ///   |\operatorname{acosh} x|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p)=\text{NaN}$
    /// - $f(\infty,p)=\infty$
    /// - $f(-\infty,p)=\text{NaN}$
    /// - $f(\pm0.0,p)=\text{NaN}$
    /// - $f(1,p)=0.0$
    /// - $f(x,p)=\text{NaN}$ if $x<1$
    ///
    /// The result never overflows, since $\operatorname{acosh} x < \ln 2x$. It underflows only for
    /// an $x$ within $2^{-2^{31}}$ of 1, since $\operatorname{acosh}(1+t) > \sqrt t$, and such an
    /// $x$ needs a precision above $2^{31}$.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::acosh_prec_round_ref`] instead. If you know that your target precision is the
    /// precision of the input, consider using `(&Float).acosh()` instead.
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
    /// let (c, o) = (&Float::from_unsigned_prec(2u32, 100).0).acosh_prec_ref(5);
    /// assert_eq!(c.to_string(), "1.31");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = (&Float::from_unsigned_prec(2u32, 100).0).acosh_prec_ref(20);
    /// assert_eq!(c.to_string(), "1.3169575");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn acosh_prec_ref(&self, prec: u64) -> (Self, Ordering) {
        self.acosh_prec_round_ref(prec, Nearest)
    }

    /// Computes $\operatorname{acosh} x$, the inverse hyperbolic cosine of a [`Float`], rounding
    /// the result with the specified rounding mode. The [`Float`] is taken by value. An
    /// [`Ordering`] is also returned, indicating whether the rounded inverse hyperbolic cosine is
    /// less than, equal to, or greater than the exact inverse hyperbolic cosine. Although `NaN`s
    /// are not comparable to any [`Float`], whenever this function returns a `NaN` it also returns
    /// `Equal`.
    ///
    /// The precision of the output is the precision of the input. See [`RoundingMode`] for a
    /// description of the possible rounding modes.
    ///
    /// $$
    /// f(x,m) = \operatorname{acosh} x+\varepsilon.
    /// $$
    /// - If $\operatorname{acosh} x$ is infinite, zero, or NaN, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $\operatorname{acosh} x$ is finite and nonzero, and $m$ is not `Nearest`, then
    ///   $|\varepsilon| < 2^{\lfloor\log_2 \operatorname{acosh} x\rfloor-p+1}$, where $p$ is the
    ///   precision of the input.
    /// - If $\operatorname{acosh} x$ is finite and nonzero, and $m$ is `Nearest`, then
    ///   $|\varepsilon| \leq 2^{\lfloor\log_2 \operatorname{acosh} x\rfloor-p}$, where $p$ is the
    ///   precision of the input.
    ///
    /// If the output has a precision, it is the precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN},m)=\text{NaN}$
    /// - $f(\infty,m)=\infty$
    /// - $f(-\infty,m)=\text{NaN}$
    /// - $f(\pm0.0,m)=\text{NaN}$
    /// - $f(1,m)=0.0$
    /// - $f(x,m)=\text{NaN}$ if $x<1$
    ///
    /// The result never overflows, since $\operatorname{acosh} x < \ln 2x$. It underflows only for
    /// an $x$ within $2^{-2^{31}}$ of 1, since $\operatorname{acosh}(1+t) > \sqrt t$, and such an
    /// $x$ needs a precision above $2^{31}$.
    ///
    /// If you want to specify an output precision, consider using [`Float::acosh_prec_round`]
    /// instead. If you know you'll be using the `Nearest` rounding mode, consider using
    /// [`Float::acosh`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` and `self` is finite and greater than 1, since the inverse
    /// hyperbolic cosine of such a [`Float`] is never exactly representable.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = Float::from_unsigned_prec(2u32, 100).0.acosh_round(Floor);
    /// assert_eq!(c.to_string(), "1.3169578969248167086250463473073");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::from_unsigned_prec(2u32, 100).0.acosh_round(Ceiling);
    /// assert_eq!(c.to_string(), "1.3169578969248167086250463473089");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = Float::from_unsigned_prec(2u32, 100).0.acosh_round(Nearest);
    /// assert_eq!(c.to_string(), "1.3169578969248167086250463473073");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn acosh_round(self, rm: RoundingMode) -> (Self, Ordering) {
        let prec = self.significant_bits();
        self.acosh_prec_round(prec, rm)
    }

    /// Computes $\operatorname{acosh} x$, the inverse hyperbolic cosine of a [`Float`], rounding
    /// the result with the specified rounding mode. The [`Float`] is taken by reference. An
    /// [`Ordering`] is also returned, indicating whether the rounded inverse hyperbolic cosine is
    /// less than, equal to, or greater than the exact inverse hyperbolic cosine. Although `NaN`s
    /// are not comparable to any [`Float`], whenever this function returns a `NaN` it also returns
    /// `Equal`.
    ///
    /// The precision of the output is the precision of the input. See [`RoundingMode`] for a
    /// description of the possible rounding modes.
    ///
    /// $$
    /// f(x,m) = \operatorname{acosh} x+\varepsilon.
    /// $$
    /// - If $\operatorname{acosh} x$ is infinite, zero, or NaN, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $\operatorname{acosh} x$ is finite and nonzero, and $m$ is not `Nearest`, then
    ///   $|\varepsilon| < 2^{\lfloor\log_2 \operatorname{acosh} x\rfloor-p+1}$, where $p$ is the
    ///   precision of the input.
    /// - If $\operatorname{acosh} x$ is finite and nonzero, and $m$ is `Nearest`, then
    ///   $|\varepsilon| \leq 2^{\lfloor\log_2 \operatorname{acosh} x\rfloor-p}$, where $p$ is the
    ///   precision of the input.
    ///
    /// If the output has a precision, it is the precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN},m)=\text{NaN}$
    /// - $f(\infty,m)=\infty$
    /// - $f(-\infty,m)=\text{NaN}$
    /// - $f(\pm0.0,m)=\text{NaN}$
    /// - $f(1,m)=0.0$
    /// - $f(x,m)=\text{NaN}$ if $x<1$
    ///
    /// The result never overflows, since $\operatorname{acosh} x < \ln 2x$. It underflows only for
    /// an $x$ within $2^{-2^{31}}$ of 1, since $\operatorname{acosh}(1+t) > \sqrt t$, and such an
    /// $x$ needs a precision above $2^{31}$.
    ///
    /// If you want to specify an output precision, consider using [`Float::acosh_prec_round_ref`]
    /// instead. If you know you'll be using the `Nearest` rounding mode, consider using
    /// `(&Float).acosh()` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` and `self` is finite and greater than 1, since the inverse
    /// hyperbolic cosine of such a [`Float`] is never exactly representable.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = (&Float::from_unsigned_prec(2u32, 100).0).acosh_round_ref(Floor);
    /// assert_eq!(c.to_string(), "1.3169578969248167086250463473073");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = (&Float::from_unsigned_prec(2u32, 100).0).acosh_round_ref(Ceiling);
    /// assert_eq!(c.to_string(), "1.3169578969248167086250463473089");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = (&Float::from_unsigned_prec(2u32, 100).0).acosh_round_ref(Nearest);
    /// assert_eq!(c.to_string(), "1.3169578969248167086250463473073");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn acosh_round_ref(&self, rm: RoundingMode) -> (Self, Ordering) {
        self.acosh_prec_round_ref(self.significant_bits(), rm)
    }

    /// Computes $\operatorname{acosh} x$, the inverse hyperbolic cosine of a [`Float`], rounding
    /// the result to the specified precision and with the specified rounding mode. The [`Float`] is
    /// replaced by the result, and an [`Ordering`] is returned, indicating whether the rounded
    /// inverse hyperbolic cosine is less than, equal to, or greater than the exact inverse
    /// hyperbolic cosine. Although `NaN`s are not comparable to any [`Float`], whenever this
    /// function sets a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets \operatorname{acosh} x+\varepsilon.
    /// $$
    /// - If $\operatorname{acosh} x$ is infinite, zero, or NaN, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $\operatorname{acosh} x$ is finite and nonzero, and $m$ is not `Nearest`, then
    ///   $|\varepsilon| < 2^{\lfloor\log_2 \operatorname{acosh} x\rfloor-p+1}$.
    /// - If $\operatorname{acosh} x$ is finite and nonzero, and $m$ is `Nearest`, then
    ///   $|\varepsilon| \leq 2^{\lfloor\log_2 \operatorname{acosh} x\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// See the [`Float::acosh_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::acosh_prec_assign`] instead.
    /// If you know that your target precision is the precision of the input, consider using
    /// [`Float::acosh_round_assign`] instead. If both of these things are true, consider using
    /// [`Float::acosh_assign`] instead.
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
    /// Panics if `rm` is `Exact` and `self` is finite and greater than 1, since the inverse
    /// hyperbolic cosine of such a [`Float`] is never exactly representable, or if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from_unsigned_prec(2u32, 100).0;
    /// assert_eq!(x.acosh_prec_round_assign(5, Floor), Less);
    /// assert_eq!(x.to_string(), "1.31");
    ///
    /// let mut x = Float::from_unsigned_prec(2u32, 100).0;
    /// assert_eq!(x.acosh_prec_round_assign(5, Ceiling), Greater);
    /// assert_eq!(x.to_string(), "1.38");
    ///
    /// let mut x = Float::from_unsigned_prec(2u32, 100).0;
    /// assert_eq!(x.acosh_prec_round_assign(5, Nearest), Less);
    /// assert_eq!(x.to_string(), "1.31");
    ///
    /// let mut x = Float::from_unsigned_prec(2u32, 100).0;
    /// assert_eq!(x.acosh_prec_round_assign(20, Floor), Less);
    /// assert_eq!(x.to_string(), "1.3169575");
    ///
    /// let mut x = Float::from_unsigned_prec(2u32, 100).0;
    /// assert_eq!(x.acosh_prec_round_assign(20, Ceiling), Greater);
    /// assert_eq!(x.to_string(), "1.3169594");
    ///
    /// let mut x = Float::from_unsigned_prec(2u32, 100).0;
    /// assert_eq!(x.acosh_prec_round_assign(20, Nearest), Less);
    /// assert_eq!(x.to_string(), "1.3169575");
    /// ```
    #[inline]
    pub fn acosh_prec_round_assign(&mut self, prec: u64, rm: RoundingMode) -> Ordering {
        let o;
        (*self, o) = self.acosh_prec_round_ref(prec, rm);
        o
    }

    /// Computes $\operatorname{acosh} x$, the inverse hyperbolic cosine of a [`Float`], rounding
    /// the result to the nearest value of the specified precision. The [`Float`] is replaced by the
    /// result, and an [`Ordering`] is returned, indicating whether the rounded inverse hyperbolic
    /// cosine is less than, equal to, or greater than the exact inverse hyperbolic cosine. Although
    /// `NaN`s are not comparable to any [`Float`], whenever this function sets a `NaN` it also
    /// returns `Equal`.
    ///
    /// If the inverse hyperbolic cosine is equidistant from two [`Float`]s with the specified
    /// precision, the [`Float`] with fewer 1s in its binary expansion is chosen. See
    /// [`RoundingMode`] for a description of the `Nearest` rounding mode.
    ///
    /// $$
    /// x \gets \operatorname{acosh} x+\varepsilon.
    /// $$
    /// - If $\operatorname{acosh} x$ is infinite, zero, or NaN, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $\operatorname{acosh} x$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2
    ///   |\operatorname{acosh} x|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// See the [`Float::acosh_prec`] documentation for information on special cases, overflow, and
    /// underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::acosh_prec_round_assign`] instead. If you know that your target precision is the
    /// precision of the input, consider using [`Float::acosh_assign`] instead.
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
    /// let mut x = Float::from_unsigned_prec(2u32, 100).0;
    /// assert_eq!(x.acosh_prec_assign(5), Less);
    /// assert_eq!(x.to_string(), "1.31");
    ///
    /// let mut x = Float::from_unsigned_prec(2u32, 100).0;
    /// assert_eq!(x.acosh_prec_assign(20), Less);
    /// assert_eq!(x.to_string(), "1.3169575");
    /// ```
    #[inline]
    pub fn acosh_prec_assign(&mut self, prec: u64) -> Ordering {
        self.acosh_prec_round_assign(prec, Nearest)
    }

    /// Computes $\operatorname{acosh} x$, the inverse hyperbolic cosine of a [`Float`], rounding
    /// the result with the specified rounding mode. The [`Float`] is replaced by the result, and an
    /// [`Ordering`] is returned, indicating whether the rounded inverse hyperbolic cosine is less
    /// than, equal to, or greater than the exact inverse hyperbolic cosine. Although `NaN`s are not
    /// comparable to any [`Float`], whenever this function sets a `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the precision of the input. See [`RoundingMode`] for a
    /// description of the possible rounding modes.
    ///
    /// $$
    /// x \gets \operatorname{acosh} x+\varepsilon.
    /// $$
    /// - If $\operatorname{acosh} x$ is infinite, zero, or NaN, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $\operatorname{acosh} x$ is finite and nonzero, and $m$ is not `Nearest`, then
    ///   $|\varepsilon| < 2^{\lfloor\log_2 \operatorname{acosh} x\rfloor-p+1}$, where $p$ is the
    ///   precision of the input.
    /// - If $\operatorname{acosh} x$ is finite and nonzero, and $m$ is `Nearest`, then
    ///   $|\varepsilon| \leq 2^{\lfloor\log_2 \operatorname{acosh} x\rfloor-p}$, where $p$ is the
    ///   precision of the input.
    ///
    /// If the output has a precision, it is the precision of the input.
    ///
    /// See the [`Float::acosh_round`] documentation for information on special cases, overflow, and
    /// underflow.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::acosh_prec_round_assign`] instead. If you know you'll be using the `Nearest`
    /// rounding mode, consider using [`Float::acosh_assign`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` and `self` is finite and greater than 1, since the inverse
    /// hyperbolic cosine of such a [`Float`] is never exactly representable.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from_unsigned_prec(2u32, 100).0;
    /// assert_eq!(x.acosh_round_assign(Floor), Less);
    /// assert_eq!(x.to_string(), "1.3169578969248167086250463473073");
    ///
    /// let mut x = Float::from_unsigned_prec(2u32, 100).0;
    /// assert_eq!(x.acosh_round_assign(Ceiling), Greater);
    /// assert_eq!(x.to_string(), "1.3169578969248167086250463473089");
    ///
    /// let mut x = Float::from_unsigned_prec(2u32, 100).0;
    /// assert_eq!(x.acosh_round_assign(Nearest), Less);
    /// assert_eq!(x.to_string(), "1.3169578969248167086250463473073");
    /// ```
    #[inline]
    pub fn acosh_round_assign(&mut self, rm: RoundingMode) -> Ordering {
        let prec = self.significant_bits();
        self.acosh_prec_round_assign(prec, rm)
    }

    /// Computes $\operatorname{acosh} x$, the inverse hyperbolic cosine of a [`Rational`], rounding
    /// the result to the specified precision and with the specified rounding mode and returning the
    /// result as a [`Float`]. The [`Rational`] is taken by value. An [`Ordering`] is also returned,
    /// indicating whether the rounded inverse hyperbolic cosine is less than, equal to, or greater
    /// than the exact inverse hyperbolic cosine. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \operatorname{acosh} x+\varepsilon.
    /// $$
    /// - If $\operatorname{acosh} x$ is zero or NaN, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\operatorname{acosh} x$ is nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 \operatorname{acosh} x\rfloor-p+1}$.
    /// - If $\operatorname{acosh} x$ is nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 \operatorname{acosh} x\rfloor-p}$.
    ///
    /// These bounds do not apply when the result underflows; see below.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(1,p,m)=0.0$
    /// - $f(x,p,m)=\text{NaN}$ if $x<1$
    ///
    /// Overflow and underflow:
    /// - Since $\operatorname{acosh} x < \ln 2x$, the result never overflows.
    /// - If $0<f(x,p,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,p,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,p,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,p,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is returned
    ///   instead.
    ///
    /// Underflow requires an $x$ within $2^{-2^{31}}$ of 1: since $\operatorname{acosh}(1+t) >
    /// \sqrt t$, no other input can reach it.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::acosh_rational_prec`]
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
    /// with the given precision (which is the case for every input greater than 1).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = Float::acosh_rational_prec_round(Rational::from_unsigneds(3u8, 2), 5, Floor);
    /// assert_eq!(c.to_string(), "0.938");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::acosh_rational_prec_round(Rational::from_unsigneds(3u8, 2), 5, Ceiling);
    /// assert_eq!(c.to_string(), "0.969");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = Float::acosh_rational_prec_round(Rational::from_unsigneds(3u8, 2), 20, Floor);
    /// assert_eq!(c.to_string(), "0.96242332");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) =
    ///     Float::acosh_rational_prec_round(Rational::from_unsigneds(3u8, 2), 20, Ceiling);
    /// assert_eq!(c.to_string(), "0.96242428");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    #[allow(clippy::needless_pass_by_value)]
    pub fn acosh_rational_prec_round(x: Rational, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        Self::acosh_rational_prec_round_ref(&x, prec, rm)
    }

    /// Computes $\operatorname{acosh} x$, the inverse hyperbolic cosine of a [`Rational`], rounding
    /// the result to the specified precision and with the specified rounding mode and returning the
    /// result as a [`Float`]. The [`Rational`] is taken by reference. An [`Ordering`] is also
    /// returned, indicating whether the rounded inverse hyperbolic cosine is less than, equal to,
    /// or greater than the exact inverse hyperbolic cosine. Although `NaN`s are not comparable to
    /// any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \operatorname{acosh} x+\varepsilon.
    /// $$
    /// - If $\operatorname{acosh} x$ is zero or NaN, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\operatorname{acosh} x$ is nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 \operatorname{acosh} x\rfloor-p+1}$.
    /// - If $\operatorname{acosh} x$ is nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 \operatorname{acosh} x\rfloor-p}$.
    ///
    /// These bounds do not apply when the result underflows; see below.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(1,p,m)=0.0$
    /// - $f(x,p,m)=\text{NaN}$ if $x<1$
    ///
    /// Overflow and underflow:
    /// - Since $\operatorname{acosh} x < \ln 2x$, the result never overflows.
    /// - If $0<f(x,p,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,p,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,p,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,p,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is returned
    ///   instead.
    ///
    /// Underflow requires an $x$ within $2^{-2^{31}}$ of 1: since $\operatorname{acosh}(1+t) >
    /// \sqrt t$, no other input can reach it.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::acosh_rational_prec_ref`]
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
    /// with the given precision (which is the case for every input greater than 1).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) =
    ///     Float::acosh_rational_prec_round_ref(&Rational::from_unsigneds(3u8, 2), 5, Floor);
    /// assert_eq!(c.to_string(), "0.938");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) =
    ///     Float::acosh_rational_prec_round_ref(&Rational::from_unsigneds(3u8, 2), 5, Ceiling);
    /// assert_eq!(c.to_string(), "0.969");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) =
    ///     Float::acosh_rational_prec_round_ref(&Rational::from_unsigneds(3u8, 2), 20, Floor);
    /// assert_eq!(c.to_string(), "0.96242332");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) =
    ///     Float::acosh_rational_prec_round_ref(&Rational::from_unsigneds(3u8, 2), 20, Ceiling);
    /// assert_eq!(c.to_string(), "0.96242428");
    /// assert_eq!(o, Greater);
    /// ```
    pub fn acosh_rational_prec_round_ref(
        x: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        assert_ne!(prec, 0);
        match x.partial_cmp(&1u32).unwrap() {
            Less => (Self::NAN, Equal),
            // acosh(1) = 0, exactly
            Equal => (Self::ZERO, Equal),
            Greater => acosh_rational_helper(x, prec, rm),
        }
    }

    /// Computes $\operatorname{acosh} x$, the inverse hyperbolic cosine of a [`Rational`], rounding
    /// the result to the nearest value of the specified precision and returning the result as a
    /// [`Float`]. The [`Rational`] is taken by value. An [`Ordering`] is also returned, indicating
    /// whether the rounded inverse hyperbolic cosine is less than, equal to, or greater than the
    /// exact inverse hyperbolic cosine. Although `NaN`s are not comparable to any [`Float`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// If the inverse hyperbolic cosine is equidistant from two [`Float`]s with the specified
    /// precision, the [`Float`] with fewer 1s in its binary expansion is chosen. See
    /// [`RoundingMode`] for a description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = \operatorname{acosh} x+\varepsilon,
    /// $$
    /// where, if $\operatorname{acosh} x$ is nonzero, $|\varepsilon| \leq 2^{\lfloor\log_2
    /// \operatorname{acosh} x\rfloor-p}$ (unless the result underflows; see below).
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(1,p)=0.0$
    /// - $f(x,p)=\text{NaN}$ if $x<1$
    ///
    /// Overflow and underflow:
    /// - Since $\operatorname{acosh} x < \ln 2x$, the result never overflows.
    /// - If $0<f(x,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    ///
    /// Underflow requires an $x$ within $2^{-2^{31}}$ of 1: since $\operatorname{acosh}(1+t) >
    /// \sqrt t$, no other input can reach it.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::acosh_rational_prec_round`] instead.
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
    /// use malachite_base::num::basic::traits::{One, OneHalf};
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = Float::acosh_rational_prec(Rational::from_unsigneds(3u8, 2), 5);
    /// assert_eq!(c.to_string(), "0.969");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = Float::acosh_rational_prec(Rational::from_unsigneds(3u8, 2), 20);
    /// assert_eq!(c.to_string(), "0.96242332");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::acosh_rational_prec(Rational::ONE, 10);
    /// assert_eq!(c.to_string(), "0.0");
    /// assert_eq!(o, Equal);
    ///
    /// let (c, o) = Float::acosh_rational_prec(Rational::ONE_HALF, 10);
    /// assert!(c.is_nan());
    /// assert_eq!(o, Equal);
    /// ```
    #[inline]
    #[allow(clippy::needless_pass_by_value)]
    pub fn acosh_rational_prec(x: Rational, prec: u64) -> (Self, Ordering) {
        Self::acosh_rational_prec_round_ref(&x, prec, Nearest)
    }

    /// Computes $\operatorname{acosh} x$, the inverse hyperbolic cosine of a [`Rational`], rounding
    /// the result to the nearest value of the specified precision and returning the result as a
    /// [`Float`]. The [`Rational`] is taken by reference. An [`Ordering`] is also returned,
    /// indicating whether the rounded inverse hyperbolic cosine is less than, equal to, or greater
    /// than the exact inverse hyperbolic cosine. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// If the inverse hyperbolic cosine is equidistant from two [`Float`]s with the specified
    /// precision, the [`Float`] with fewer 1s in its binary expansion is chosen. See
    /// [`RoundingMode`] for a description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = \operatorname{acosh} x+\varepsilon,
    /// $$
    /// where, if $\operatorname{acosh} x$ is nonzero, $|\varepsilon| \leq 2^{\lfloor\log_2
    /// \operatorname{acosh} x\rfloor-p}$ (unless the result underflows; see below).
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(1,p)=0.0$
    /// - $f(x,p)=\text{NaN}$ if $x<1$
    ///
    /// Overflow and underflow:
    /// - Since $\operatorname{acosh} x < \ln 2x$, the result never overflows.
    /// - If $0<f(x,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    ///
    /// Underflow requires an $x$ within $2^{-2^{31}}$ of 1: since $\operatorname{acosh}(1+t) >
    /// \sqrt t$, no other input can reach it.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::acosh_rational_prec_round_ref`] instead.
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
    /// use malachite_base::num::basic::traits::{One, OneHalf};
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = Float::acosh_rational_prec_ref(&Rational::from_unsigneds(3u8, 2), 5);
    /// assert_eq!(c.to_string(), "0.969");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = Float::acosh_rational_prec_ref(&Rational::from_unsigneds(3u8, 2), 20);
    /// assert_eq!(c.to_string(), "0.96242332");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::acosh_rational_prec_ref(&Rational::ONE, 10);
    /// assert_eq!(c.to_string(), "0.0");
    /// assert_eq!(o, Equal);
    ///
    /// let (c, o) = Float::acosh_rational_prec_ref(&Rational::ONE_HALF, 10);
    /// assert!(c.is_nan());
    /// assert_eq!(o, Equal);
    /// ```
    #[inline]
    pub fn acosh_rational_prec_ref(x: &Rational, prec: u64) -> (Self, Ordering) {
        Self::acosh_rational_prec_round_ref(x, prec, Nearest)
    }
}

impl Acosh for Float {
    type Output = Self;

    /// Computes $\operatorname{acosh} x$, the inverse hyperbolic cosine of a [`Float`], taking it
    /// by value.
    ///
    /// If the output has a precision, it is the precision of the input. If the inverse hyperbolic
    /// cosine is equidistant from two [`Float`]s with the specified precision, the [`Float`] with
    /// fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of the
    /// `Nearest` rounding mode.
    ///
    /// $$
    /// f(x) = \operatorname{acosh} x+\varepsilon.
    /// $$
    /// - If $\operatorname{acosh} x$ is infinite, zero, or NaN, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $\operatorname{acosh} x$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2
    ///   |\operatorname{acosh} x|\rfloor-p}$, where $p$ is the precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN})=\text{NaN}$
    /// - $f(\infty)=\infty$
    /// - $f(-\infty)=\text{NaN}$
    /// - $f(\pm0.0)=\text{NaN}$
    /// - $f(1)=0.0$
    /// - $f(x)=\text{NaN}$ if $x<1$
    ///
    /// See the [`Float::acosh_round`] documentation for information on overflow and underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::acosh_round`] instead. If you want to specify the output precision, consider using
    /// [`Float::acosh_prec`]. If you want both of these things, consider using
    /// [`Float::acosh_prec_round`].
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
    /// use malachite_base::num::arithmetic::traits::Acosh;
    /// use malachite_base::num::basic::traits::*;
    /// use malachite_float::Float;
    ///
    /// assert!(Float::NAN.acosh().is_nan());
    /// assert_eq!(Float::INFINITY.acosh().to_string(), "Infinity");
    /// assert!(Float::NEGATIVE_INFINITY.acosh().is_nan());
    /// assert!(Float::ZERO.acosh().is_nan());
    /// assert!(Float::NEGATIVE_ZERO.acosh().is_nan());
    /// assert_eq!(Float::ONE.acosh().to_string(), "0.0");
    /// assert_eq!(
    ///     Float::from_unsigned_prec(2u32, 100).0.acosh().to_string(),
    ///     "1.3169578969248167086250463473073"
    /// );
    /// assert_eq!(
    ///     Float::from_unsigned_prec(100u32, 100).0.acosh().to_string(),
    ///     "5.2982923656104845907016668349453"
    /// );
    /// ```
    #[inline]
    fn acosh(self) -> Self {
        let prec = self.significant_bits();
        self.acosh_prec_round(prec, Nearest).0
    }
}

impl Acosh for &Float {
    type Output = Float;

    /// Computes $\operatorname{acosh} x$, the inverse hyperbolic cosine of a [`Float`], taking it
    /// by reference.
    ///
    /// If the output has a precision, it is the precision of the input. If the inverse hyperbolic
    /// cosine is equidistant from two [`Float`]s with the specified precision, the [`Float`] with
    /// fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of the
    /// `Nearest` rounding mode.
    ///
    /// $$
    /// f(x) = \operatorname{acosh} x+\varepsilon.
    /// $$
    /// - If $\operatorname{acosh} x$ is infinite, zero, or NaN, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $\operatorname{acosh} x$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2
    ///   |\operatorname{acosh} x|\rfloor-p}$, where $p$ is the precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN})=\text{NaN}$
    /// - $f(\infty)=\infty$
    /// - $f(-\infty)=\text{NaN}$
    /// - $f(\pm0.0)=\text{NaN}$
    /// - $f(1)=0.0$
    /// - $f(x)=\text{NaN}$ if $x<1$
    ///
    /// See the [`Float::acosh_round`] documentation for information on overflow and underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::acosh_round_ref`] instead. If you want to specify the output precision, consider
    /// using [`Float::acosh_prec_ref`]. If you want both of these things, consider using
    /// [`Float::acosh_prec_round_ref`].
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
    /// use malachite_base::num::arithmetic::traits::Acosh;
    /// use malachite_base::num::basic::traits::*;
    /// use malachite_float::Float;
    ///
    /// assert!(Float::NAN.acosh().is_nan());
    /// assert_eq!(Float::INFINITY.acosh().to_string(), "Infinity");
    /// assert!(Float::NEGATIVE_INFINITY.acosh().is_nan());
    /// assert!(Float::ZERO.acosh().is_nan());
    /// assert!(Float::NEGATIVE_ZERO.acosh().is_nan());
    /// assert_eq!(Float::ONE.acosh().to_string(), "0.0");
    /// assert_eq!(
    ///     (&Float::from_unsigned_prec(2u32, 100).0)
    ///         .acosh()
    ///         .to_string(),
    ///     "1.3169578969248167086250463473073"
    /// );
    /// assert_eq!(
    ///     (&Float::from_unsigned_prec(100u32, 100).0)
    ///         .acosh()
    ///         .to_string(),
    ///     "5.2982923656104845907016668349453"
    /// );
    /// ```
    #[inline]
    fn acosh(self) -> Float {
        self.acosh_prec_round_ref(self.significant_bits(), Nearest)
            .0
    }
}

impl AcoshAssign for Float {
    /// Computes $\operatorname{acosh} x$, the inverse hyperbolic cosine of a [`Float`], in place.
    ///
    /// If the output has a precision, it is the precision of the input. If the inverse hyperbolic
    /// cosine is equidistant from two [`Float`]s with the specified precision, the [`Float`] with
    /// fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of the
    /// `Nearest` rounding mode.
    ///
    /// $$
    /// x \gets \operatorname{acosh} x+\varepsilon.
    /// $$
    /// - If $\operatorname{acosh} x$ is infinite, zero, or NaN, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $\operatorname{acosh} x$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2
    ///   |\operatorname{acosh} x|\rfloor-p}$, where $p$ is the precision of the input.
    ///
    /// See the [`Float::acosh`] documentation for information on special cases, overflow, and
    /// underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::acosh_round_assign`] instead. If you want to specify the output precision, consider
    /// using [`Float::acosh_prec_assign`]. If you want both of these things, consider using
    /// [`Float::acosh_prec_round_assign`].
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
    /// use malachite_base::num::arithmetic::traits::AcoshAssign;
    /// use malachite_base::num::basic::traits::*;
    /// use malachite_float::Float;
    ///
    /// let mut x = Float::NAN;
    /// x.acosh_assign();
    /// assert!(x.is_nan());
    ///
    /// let mut x = Float::INFINITY;
    /// x.acosh_assign();
    /// assert_eq!(x.to_string(), "Infinity");
    ///
    /// let mut x = Float::NEGATIVE_INFINITY;
    /// x.acosh_assign();
    /// assert!(x.is_nan());
    ///
    /// let mut x = Float::ZERO;
    /// x.acosh_assign();
    /// assert!(x.is_nan());
    ///
    /// let mut x = Float::NEGATIVE_ZERO;
    /// x.acosh_assign();
    /// assert!(x.is_nan());
    ///
    /// let mut x = Float::ONE;
    /// x.acosh_assign();
    /// assert_eq!(x.to_string(), "0.0");
    ///
    /// let mut x = Float::from_unsigned_prec(2u32, 100).0;
    /// x.acosh_assign();
    /// assert_eq!(x.to_string(), "1.3169578969248167086250463473073");
    ///
    /// let mut x = Float::from_unsigned_prec(100u32, 100).0;
    /// x.acosh_assign();
    /// assert_eq!(x.to_string(), "5.2982923656104845907016668349453");
    /// ```
    #[inline]
    fn acosh_assign(&mut self) {
        let prec = self.significant_bits();
        self.acosh_prec_round_assign(prec, Nearest);
    }
}

/// Computes $\operatorname{acosh} x$, the inverse hyperbolic cosine of a primitive float. Using
/// this function is more accurate than using the default `acosh` function or the one provided by
/// `libm`.
///
/// $$
/// f(x) = \operatorname{acosh} x+\varepsilon.
/// $$
/// - If $\operatorname{acosh} x$ is infinite, zero, or NaN, $\varepsilon$ may be ignored or assumed
///   to be 0.
/// - If $\operatorname{acosh} x$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2
///   \operatorname{acosh} x\rfloor-p}$, where $p$ is the precision of the output (24 if `T` is a
///   [`f32`] and 53 if `T` is a [`f64`]).
///
/// Special cases:
/// - $f(\text{NaN})=\text{NaN}$
/// - $f(\infty)=\infty$
/// - $f(-\infty)=\text{NaN}$
/// - $f(\pm0.0)=\text{NaN}$
/// - $f(1)=0.0$
/// - $f(x)=\text{NaN}$ if $x<1$
///
/// Overflow and underflow are not possible: the result is less than $\ln 2x$, and for $x>1$ it is
/// at least $\operatorname{acosh}(1+2^{1-p})$, far above the subnormal range.
///
/// # Worst-case complexity
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use malachite_base::num::basic::traits::NegativeInfinity;
/// use malachite_base::num::float::NiceFloat;
/// use malachite_float::float::arithmetic::acosh::primitive_float_acosh;
///
/// assert!(primitive_float_acosh(f32::NAN).is_nan());
/// assert_eq!(
///     NiceFloat(primitive_float_acosh(f32::INFINITY)),
///     NiceFloat(f32::INFINITY)
/// );
/// assert!(primitive_float_acosh(f32::NEGATIVE_INFINITY).is_nan());
/// assert!(primitive_float_acosh(0.0f32).is_nan());
/// assert!(primitive_float_acosh(-0.0f32).is_nan());
/// assert_eq!(NiceFloat(primitive_float_acosh(1.0f32)), NiceFloat(0.0));
/// assert_eq!(
///     NiceFloat(primitive_float_acosh(2.0f32)),
///     NiceFloat(1.316958)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_acosh(2.0f64)),
///     NiceFloat(1.3169578969248168)
/// );
/// ```
#[inline]
#[allow(clippy::type_repetition_in_bounds)]
pub fn primitive_float_acosh<T: PrimitiveFloat>(x: T) -> T
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    emulate_float_to_float_fn(Float::acosh_prec, x)
}

/// Computes $\operatorname{acosh} x$, the inverse hyperbolic cosine of a [`Rational`], returning
/// the result as a primitive float. The result is correctly rounded.
///
/// $$
/// f(x) = \operatorname{acosh} x+\varepsilon.
/// $$
/// - If $\operatorname{acosh} x$ is zero or NaN, $\varepsilon$ may be ignored or assumed to be 0.
/// - If $\operatorname{acosh} x$ is nonzero, then $|\varepsilon| < 2^{\lfloor\log_2
///   \operatorname{acosh} x\rfloor-p}$, where $p$ is the precision of the output (typically 24 if
///   `T` is a [`f32`] and 53 if `T` is a [`f64`], but less if the output is subnormal).
///
/// Special cases:
/// - $f(1)=0.0$
/// - $f(x)=\text{NaN}$ if $x<1$
///
/// Overflow is not possible. Underflow is: an `x` close enough to 1 gives `0.0`.
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
/// use malachite_base::num::basic::traits::{One, OneHalf};
/// use malachite_base::num::float::NiceFloat;
/// use malachite_float::float::arithmetic::acosh::primitive_float_acosh_rational;
/// use malachite_q::Rational;
///
/// assert!(primitive_float_acosh_rational::<f64>(&Rational::ONE_HALF).is_nan());
/// assert_eq!(
///     NiceFloat(primitive_float_acosh_rational::<f64>(&Rational::ONE)),
///     NiceFloat(0.0)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_acosh_rational::<f64>(
///         &Rational::from_unsigneds(3u8, 2)
///     )),
///     NiceFloat(0.9624236501192069)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_acosh_rational::<f64>(
///         &Rational::from_unsigneds(22u8, 7)
///     )),
///     NiceFloat(1.8119507608214136)
/// );
/// ```
#[inline]
#[allow(clippy::type_repetition_in_bounds)]
pub fn primitive_float_acosh_rational<T: PrimitiveFloat>(x: &Rational) -> T
where
    Float: PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    emulate_rational_to_float_fn(Float::acosh_rational_prec_ref, x)
}
