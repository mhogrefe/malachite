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
use crate::float::arithmetic::cosh::monotone_rational_via_floats;
use crate::float::arithmetic::round_near_x::{
    float_round_near_x, round_near_reciprocal, round_rational_reciprocal_leading_term,
};
use crate::float::arithmetic::sech::hyperbolic_series_quotient;
use crate::float::arithmetic::sinh::sinh_bound;
use crate::float::arithmetic::tan::reciprocal_ziv_loop;
use crate::float::arithmetic::tanh::{cosh_bound, two_x_log_2_e_lower_bound};
use crate::{Float, emulate_float_to_float_fn, emulate_rational_to_float_fn};
use core::cmp::Ordering::{self, *};
use core::cmp::{max, min};
use malachite_base::num::arithmetic::traits::{Abs, Coth, CothAssign};
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::traits::{
    Infinity as InfinityTrait, NaN as NaNTrait, NegativeInfinity, NegativeOne, One,
};
use malachite_base::num::conversion::traits::{ExactFrom, RoundingFrom};
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_q::Rational;

// Computes coth(x) for a finite nonzero x so large that coth(x) is close to ±1, the case MPFR's
// ACTION_SPECIAL in coth.c handles inside the Ziv loop, when the reciprocal of the hyperbolic
// tangent lies within 2^-prec of ±1, by rounding that approximation; that can misround when the
// true value lies near the midpoint 1 + 2^-prec. Here the distance from ±1 is bounded from x
// instead, before the loop: 0 < |coth(x)| - 1 = 2 / (exp(2|x|) - 1) <= 4 exp(-2|x|) = 2^(2 - 2|x|
// log_2(e)) once exp(2|x|) >= 2. `x_abs` may be any lower bound on |x|, since |coth(x)| decreases
// in |x|. Returns `None` when the bound is too weak to decide the rounding.
fn coth_near_one(
    x_abs: &Float,
    positive: bool,
    prec: u64,
    rm: RoundingMode,
) -> Option<(Float, Ordering)> {
    let bound = two_x_log_2_e_lower_bound(x_abs);
    // exp(2|x|) >= 2 needs |x| >= log(2)/2 = 0.34..., which a nonzero bound guarantees, since then
    // floor(2|x|) >= 1. |coth(x)| - 1 < 2^(2 - bound) = 2^(EXP(1) - err) with err = bound - 1.
    if bound == 0 {
        return None;
    }
    let err = bound - 1;
    if err <= prec + 1 {
        return None;
    }
    let one = if positive {
        Float::ONE
    } else {
        Float::NEGATIVE_ONE
    };
    float_round_near_x(&one, min(err, prec + 2), true, prec, rm)
}

// This is mpfr_coth from coth.c (an instantiation of gen_inverse.h), MPFR 4.2.2, where the input is
// finite and nonzero, with the bracket path for results near the top of the exponent range.
fn coth_prec_round_normal_ref(x: &Float, prec: u64, rm: RoundingMode) -> (Float, Ordering) {
    assert_ne!(rm, Exact, "Inexact coth");
    let exp_x = i64::from(x.get_exponent().unwrap());
    // ACTION_TINY from coth.c: EXP(x) + 1 <= -2 max(PREC(x), PREC(y)). There coth x = 1/x + x/3 -
    // ..., and |coth x - 1/x| <= 0.32 for |x| <= 1, with the correction sharing the sign of 1/x, so
    // that the hyperbolic cotangent lies just beyond 1/x.
    let n = i64::exact_from(max(x.get_prec().unwrap(), prec));
    if exp_x < -(n << 1) {
        return round_near_reciprocal(x, true, prec, rm);
    }
    if let Some(result) = coth_near_one(&x.abs(), x.is_sign_positive(), prec, rm) {
        return result;
    }
    // |tanh(x)| < 1, so MPFR's overflow check cannot fire. The loop's bracket path, for a
    // reciprocal near the top of the exponent range, is reached only for an x near the bottom of
    // the range that is not tiny, which requires a precision of about 2^29 bits, since |tanh(x)| >=
    // |x| / 2 for |x| <= 1.
    reciprocal_ziv_loop(prec, rm, |m| x.tanh_prec_round_ref(m, Down).0)
}

// Computes coth(x) for a nonzero `Rational` x, rounded to precision `prec` with rounding mode `rm`.
// coth(x) is transcendental for every nonzero rational x, so the result is never exact.
fn coth_rational_helper(x: &Rational, prec: u64, rm: RoundingMode) -> (Float, Ordering) {
    assert_ne!(rm, Exact, "Inexact coth");
    let exp_x = x.floor_log_base_2_abs() + 1; // the MPFR-style exponent of x
    let positive = *x > 0u32;
    // coth(x) = 1/x + x/3 - ..., so |coth x| exceeds 1/|x| by less than |x|/3: for a tiny x the
    // reciprocal's own rounding, nudged away from zero, is the answer.
    if let Some(result) = round_rational_reciprocal_leading_term(x, exp_x, true, prec, rm) {
        return result;
    }
    if exp_x < -1 && u64::exact_from(-exp_x) << 4 >= prec + 10 {
        return hyperbolic_series_quotient(x, !positive, Some(cosh_bound), sinh_bound, prec, rm);
    }
    // A lower bound on |x| serves for the bound on |coth(x)| - 1; rounding |x| down to 64 bits
    // gives one, and an |x| too large to be a `Float` rounds down to the largest finite `Float`.
    let x_abs_lo = Float::from_rational_prec_round_ref(&x.abs(), 64, Floor).0;
    if let Some(result) = coth_near_one(&x_abs_lo, positive, prec, rm) {
        return result;
    }
    // coth is decreasing on each side of 0
    monotone_rational_via_floats(x, prec, rm, coth_prec_round_normal_ref)
}

impl Float {
    /// Computes $\coth x$, the hyperbolic cotangent of a [`Float`], rounding the result to the
    /// specified precision and with the specified rounding mode. The [`Float`] is taken by value.
    /// An [`Ordering`] is also returned, indicating whether the rounded hyperbolic cotangent is
    /// less than, equal to, or greater than the exact hyperbolic cotangent. Although `NaN`s are not
    /// comparable to any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \coth x+\varepsilon.
    /// $$
    /// - If $\coth x$ is infinite or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $\coth x$ is finite, and $m$ is not `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2
    ///   |\coth x|\rfloor-p+1}$.
    /// - If $\coth x$ is finite, and $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\coth x|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=\text{NaN}$
    /// - $f(\infty,p,m)=1.0$
    /// - $f(-\infty,p,m)=-1.0$
    /// - $f(0.0,p,m)=\infty$
    /// - $f(-0.0,p,m)=-\infty$
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
    /// Underflow is not possible, since $|\coth x| > 1$. Overflow happens only for inputs of
    /// magnitude at most about $2^{-2^{30}+1}$.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::coth_prec`] instead. If you
    /// know that your target precision is the precision of the input, consider using
    /// [`Float::coth_round`] instead. If both of these things are true, consider using
    /// [`Float::coth`] instead.
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
    /// Panics if `rm` is `Exact` and `self` is finite and nonzero, since the hyperbolic cotangent
    /// of a finite nonzero [`Float`] is never exactly representable, or if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .coth_prec_round(5, Floor);
    /// assert_eq!(c.to_string(), "1.31");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .coth_prec_round(5, Ceiling);
    /// assert_eq!(c.to_string(), "1.38");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .coth_prec_round(5, Nearest);
    /// assert_eq!(c.to_string(), "1.31");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .coth_prec_round(20, Floor);
    /// assert_eq!(c.to_string(), "1.3130341");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .coth_prec_round(20, Ceiling);
    /// assert_eq!(c.to_string(), "1.3130360");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .coth_prec_round(20, Nearest);
    /// assert_eq!(c.to_string(), "1.3130360");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn coth_prec_round(self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        self.coth_prec_round_ref(prec, rm)
    }

    /// Computes $\coth x$, the hyperbolic cotangent of a [`Float`], rounding the result to the
    /// specified precision and with the specified rounding mode. The [`Float`] is taken by
    /// reference. An [`Ordering`] is also returned, indicating whether the rounded hyperbolic
    /// cotangent is less than, equal to, or greater than the exact hyperbolic cotangent. Although
    /// `NaN`s are not comparable to any [`Float`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \coth x+\varepsilon.
    /// $$
    /// - If $\coth x$ is infinite or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $\coth x$ is finite, and $m$ is not `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2
    ///   |\coth x|\rfloor-p+1}$.
    /// - If $\coth x$ is finite, and $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\coth x|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=\text{NaN}$
    /// - $f(\infty,p,m)=1.0$
    /// - $f(-\infty,p,m)=-1.0$
    /// - $f(0.0,p,m)=\infty$
    /// - $f(-0.0,p,m)=-\infty$
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
    /// Underflow is not possible, since $|\coth x| > 1$. Overflow happens only for inputs of
    /// magnitude at most about $2^{-2^{30}+1}$.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::coth_prec_ref`] instead. If
    /// you know that your target precision is the precision of the input, consider using
    /// [`Float::coth_round_ref`] instead. If both of these things are true, consider using
    /// `(&Float).coth()` instead.
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
    /// Panics if `rm` is `Exact` and `self` is finite and nonzero, since the hyperbolic cotangent
    /// of a finite nonzero [`Float`] is never exactly representable, or if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .coth_prec_round_ref(5, Floor);
    /// assert_eq!(c.to_string(), "1.31");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .coth_prec_round_ref(5, Ceiling);
    /// assert_eq!(c.to_string(), "1.38");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .coth_prec_round_ref(5, Nearest);
    /// assert_eq!(c.to_string(), "1.31");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .coth_prec_round_ref(20, Floor);
    /// assert_eq!(c.to_string(), "1.3130341");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .coth_prec_round_ref(20, Ceiling);
    /// assert_eq!(c.to_string(), "1.3130360");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .coth_prec_round_ref(20, Nearest);
    /// assert_eq!(c.to_string(), "1.3130360");
    /// assert_eq!(o, Greater);
    /// ```
    pub fn coth_prec_round_ref(&self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        assert_ne!(prec, 0);
        match &self.0 {
            NaN => (Self::NAN, Equal),
            // coth(+Inf) = 1, coth(-Inf) = -1
            Infinity { sign } => (
                if *sign {
                    Self::one_prec(prec)
                } else {
                    -Self::one_prec(prec)
                },
                Equal,
            ),
            // coth(+0) = +Inf, coth(-0) = -Inf
            Zero { sign } => (
                if *sign {
                    Self::INFINITY
                } else {
                    Self::NEGATIVE_INFINITY
                },
                Equal,
            ),
            Finite { .. } => coth_prec_round_normal_ref(self, prec, rm),
        }
    }

    /// Computes $\coth x$, the hyperbolic cotangent of a [`Float`], rounding the result to the
    /// nearest value of the specified precision. The [`Float`] is taken by value. An [`Ordering`]
    /// is also returned, indicating whether the rounded hyperbolic cotangent is less than, equal
    /// to, or greater than the exact hyperbolic cotangent. Although `NaN`s are not comparable to
    /// any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// If the hyperbolic cotangent is equidistant from two [`Float`]s with the specified precision,
    /// the [`Float`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a
    /// description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = \coth x+\varepsilon.
    /// $$
    /// - If $\coth x$ is infinite or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $\coth x$ is finite, then $|\varepsilon| < 2^{\lfloor\log_2 |\coth x|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p)=\text{NaN}$
    /// - $f(\infty,p)=1.0$
    /// - $f(-\infty,p)=-1.0$
    /// - $f(0.0,p)=\infty$
    /// - $f(-0.0,p)=-\infty$
    ///
    /// Overflow:
    /// - If $f(x,p)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,p)\leq -2^{2^{30}-1}$, $-\infty$ is returned instead.
    ///
    /// Underflow is not possible, since $|\coth x| > 1$.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::coth_prec_round`] instead. If you know that your target precision is the precision
    /// of the input, consider using [`Float::coth`] instead.
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
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100).0.coth_prec(5);
    /// assert_eq!(c.to_string(), "1.31");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100).0.coth_prec(20);
    /// assert_eq!(c.to_string(), "1.3130360");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn coth_prec(self, prec: u64) -> (Self, Ordering) {
        self.coth_prec_round(prec, Nearest)
    }

    /// Computes $\coth x$, the hyperbolic cotangent of a [`Float`], rounding the result to the
    /// nearest value of the specified precision. The [`Float`] is taken by reference. An
    /// [`Ordering`] is also returned, indicating whether the rounded hyperbolic cotangent is less
    /// than, equal to, or greater than the exact hyperbolic cotangent. Although `NaN`s are not
    /// comparable to any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// If the hyperbolic cotangent is equidistant from two [`Float`]s with the specified precision,
    /// the [`Float`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a
    /// description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = \coth x+\varepsilon.
    /// $$
    /// - If $\coth x$ is infinite or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $\coth x$ is finite, then $|\varepsilon| < 2^{\lfloor\log_2 |\coth x|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p)=\text{NaN}$
    /// - $f(\infty,p)=1.0$
    /// - $f(-\infty,p)=-1.0$
    /// - $f(0.0,p)=\infty$
    /// - $f(-0.0,p)=-\infty$
    ///
    /// Overflow:
    /// - If $f(x,p)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,p)\leq -2^{2^{30}-1}$, $-\infty$ is returned instead.
    ///
    /// Underflow is not possible, since $|\coth x| > 1$.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::coth_prec_round_ref`] instead. If you know that your target precision is the
    /// precision of the input, consider using `(&Float).coth()` instead.
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
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100).0.coth_prec_ref(5);
    /// assert_eq!(c.to_string(), "1.31");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100).0.coth_prec_ref(20);
    /// assert_eq!(c.to_string(), "1.3130360");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn coth_prec_ref(&self, prec: u64) -> (Self, Ordering) {
        self.coth_prec_round_ref(prec, Nearest)
    }

    /// Computes $\coth x$, the hyperbolic cotangent of a [`Float`], rounding the result with the
    /// specified rounding mode. The [`Float`] is taken by value. An [`Ordering`] is also returned,
    /// indicating whether the rounded hyperbolic cotangent is less than, equal to, or greater than
    /// the exact hyperbolic cotangent. Although `NaN`s are not comparable to any [`Float`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the precision of the input. See [`RoundingMode`] for a
    /// description of the possible rounding modes.
    ///
    /// $$
    /// f(x,m) = \coth x+\varepsilon.
    /// $$
    /// - If $\coth x$ is infinite or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $\coth x$ is finite, and $m$ is not `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2
    ///   |\coth x|\rfloor-p+1}$, where $p$ is the precision of the input.
    /// - If $\coth x$ is finite, and $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\coth x|\rfloor-p}$, where $p$ is the precision of the input.
    ///
    /// If the output has a precision, it is the precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN},m)=\text{NaN}$
    /// - $f(\infty,m)=1.0$
    /// - $f(-\infty,m)=-1.0$
    /// - $f(0.0,m)=\infty$
    /// - $f(-0.0,m)=-\infty$
    ///
    /// See the [`Float::coth_prec_round`] documentation for information on overflow and underflow.
    ///
    /// If you want to specify an output precision, consider using [`Float::coth_prec_round`]
    /// instead. If you know you'll be using the `Nearest` rounding mode, consider using
    /// [`Float::coth`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` and `self` is finite and nonzero, since the hyperbolic cotangent
    /// of a finite nonzero [`Float`] is never exactly representable.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100).0.coth_round(Floor);
    /// assert_eq!(c.to_string(), "1.3130352854993313036361612469298");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100).0.coth_round(Ceiling);
    /// assert_eq!(c.to_string(), "1.3130352854993313036361612469313");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100).0.coth_round(Nearest);
    /// assert_eq!(c.to_string(), "1.3130352854993313036361612469313");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn coth_round(self, rm: RoundingMode) -> (Self, Ordering) {
        let prec = self.significant_bits();
        self.coth_prec_round(prec, rm)
    }

    /// Computes $\coth x$, the hyperbolic cotangent of a [`Float`], rounding the result with the
    /// specified rounding mode. The [`Float`] is taken by reference. An [`Ordering`] is also
    /// returned, indicating whether the rounded hyperbolic cotangent is less than, equal to, or
    /// greater than the exact hyperbolic cotangent. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the precision of the input. See [`RoundingMode`] for a
    /// description of the possible rounding modes.
    ///
    /// $$
    /// f(x,m) = \coth x+\varepsilon.
    /// $$
    /// - If $\coth x$ is infinite or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $\coth x$ is finite, and $m$ is not `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2
    ///   |\coth x|\rfloor-p+1}$, where $p$ is the precision of the input.
    /// - If $\coth x$ is finite, and $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\coth x|\rfloor-p}$, where $p$ is the precision of the input.
    ///
    /// If the output has a precision, it is the precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN},m)=\text{NaN}$
    /// - $f(\infty,m)=1.0$
    /// - $f(-\infty,m)=-1.0$
    /// - $f(0.0,m)=\infty$
    /// - $f(-0.0,m)=-\infty$
    ///
    /// See the [`Float::coth_prec_round`] documentation for information on overflow and underflow.
    ///
    /// If you want to specify an output precision, consider using [`Float::coth_prec_round_ref`]
    /// instead. If you know you'll be using the `Nearest` rounding mode, consider using
    /// `(&Float).coth()` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` and `self` is finite and nonzero, since the hyperbolic cotangent
    /// of a finite nonzero [`Float`] is never exactly representable.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100).0.coth_round_ref(Floor);
    /// assert_eq!(c.to_string(), "1.3130352854993313036361612469298");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .coth_round_ref(Ceiling);
    /// assert_eq!(c.to_string(), "1.3130352854993313036361612469313");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .coth_round_ref(Nearest);
    /// assert_eq!(c.to_string(), "1.3130352854993313036361612469313");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn coth_round_ref(&self, rm: RoundingMode) -> (Self, Ordering) {
        self.coth_prec_round_ref(self.significant_bits(), rm)
    }

    /// Computes $\coth x$, the hyperbolic cotangent of a [`Float`], in place, rounding the result
    /// to the specified precision and with the specified rounding mode. An [`Ordering`] is
    /// returned, indicating whether the rounded hyperbolic cotangent is less than, equal to, or
    /// greater than the exact hyperbolic cotangent. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function sets the [`Float`] to `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets \coth x+\varepsilon.
    /// $$
    /// - If $\coth x$ is infinite or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $\coth x$ is finite, and $m$ is not `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2
    ///   |\coth x|\rfloor-p+1}$.
    /// - If $\coth x$ is finite, and $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\coth x|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// See the [`Float::coth_prec_round`] documentation for information on special cases and
    /// overflow.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::coth_prec_assign`] instead.
    /// If you know that your target precision is the precision of the input, consider using
    /// [`Float::coth_round_assign`] instead. If both of these things are true, consider using
    /// [`Float::coth_assign`] instead.
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
    /// Panics if `rm` is `Exact` and `self` is finite and nonzero, since the hyperbolic cotangent
    /// of a finite nonzero [`Float`] is never exactly representable, or if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.coth_prec_round_assign(5, Floor), Less);
    /// assert_eq!(x.to_string(), "1.31");
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.coth_prec_round_assign(5, Ceiling), Greater);
    /// assert_eq!(x.to_string(), "1.38");
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.coth_prec_round_assign(5, Nearest), Less);
    /// assert_eq!(x.to_string(), "1.31");
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.coth_prec_round_assign(20, Floor), Less);
    /// assert_eq!(x.to_string(), "1.3130341");
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.coth_prec_round_assign(20, Ceiling), Greater);
    /// assert_eq!(x.to_string(), "1.3130360");
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.coth_prec_round_assign(20, Nearest), Greater);
    /// assert_eq!(x.to_string(), "1.3130360");
    /// ```
    #[inline]
    pub fn coth_prec_round_assign(&mut self, prec: u64, rm: RoundingMode) -> Ordering {
        let o;
        (*self, o) = self.coth_prec_round_ref(prec, rm);
        o
    }

    /// Computes $\coth x$, the hyperbolic cotangent of a [`Float`], in place, rounding the result
    /// to the nearest value of the specified precision. An [`Ordering`] is returned, indicating
    /// whether the rounded hyperbolic cotangent is less than, equal to, or greater than the exact
    /// hyperbolic cotangent. Although `NaN`s are not comparable to any [`Float`], whenever this
    /// function sets the [`Float`] to `NaN` it also returns `Equal`.
    ///
    /// If the hyperbolic cotangent is equidistant from two [`Float`]s with the specified precision,
    /// the [`Float`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a
    /// description of the `Nearest` rounding mode.
    ///
    /// $$
    /// x \gets \coth x+\varepsilon.
    /// $$
    /// - If $\coth x$ is infinite or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $\coth x$ is finite, then $|\varepsilon| < 2^{\lfloor\log_2 |\coth x|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// See the [`Float::coth_prec`] documentation for information on special cases, overflow, and
    /// underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::coth_prec_round_assign`] instead. If you know that your target precision is the
    /// precision of the input, consider using [`Float::coth_assign`] instead.
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
    /// assert_eq!(x.coth_prec_assign(5), Less);
    /// assert_eq!(x.to_string(), "1.31");
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.coth_prec_assign(20), Greater);
    /// assert_eq!(x.to_string(), "1.3130360");
    /// ```
    #[inline]
    pub fn coth_prec_assign(&mut self, prec: u64) -> Ordering {
        self.coth_prec_round_assign(prec, Nearest)
    }

    /// Computes $\coth x$, the hyperbolic cotangent of a [`Float`], in place, rounding the result
    /// with the specified rounding mode. An [`Ordering`] is returned, indicating whether the
    /// rounded hyperbolic cotangent is less than, equal to, or greater than the exact hyperbolic
    /// cotangent. Although `NaN`s are not comparable to any [`Float`], whenever this function sets
    /// the [`Float`] to `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the precision of the input. See [`RoundingMode`] for a
    /// description of the possible rounding modes.
    ///
    /// $$
    /// x \gets \coth x+\varepsilon.
    /// $$
    /// - If $\coth x$ is infinite or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $\coth x$ is finite, and $m$ is not `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2
    ///   |\coth x|\rfloor-p+1}$, where $p$ is the precision of the input.
    /// - If $\coth x$ is finite, and $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\coth x|\rfloor-p}$, where $p$ is the precision of the input.
    ///
    /// If the output has a precision, it is the precision of the input.
    ///
    /// See the [`Float::coth_round`] documentation for information on special cases, overflow, and
    /// underflow.
    ///
    /// If you want to specify an output precision, consider using [`Float::coth_prec_round_assign`]
    /// instead. If you know you'll be using the `Nearest` rounding mode, consider using
    /// [`Float::coth_assign`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` and `self` is finite and nonzero, since the hyperbolic cotangent
    /// of a finite nonzero [`Float`] is never exactly representable.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.coth_round_assign(Floor), Less);
    /// assert_eq!(x.to_string(), "1.3130352854993313036361612469298");
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.coth_round_assign(Ceiling), Greater);
    /// assert_eq!(x.to_string(), "1.3130352854993313036361612469313");
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.coth_round_assign(Nearest), Greater);
    /// assert_eq!(x.to_string(), "1.3130352854993313036361612469313");
    /// ```
    #[inline]
    pub fn coth_round_assign(&mut self, rm: RoundingMode) -> Ordering {
        let prec = self.significant_bits();
        self.coth_prec_round_assign(prec, rm)
    }
}

impl Float {
    /// Computes $\coth x$, the hyperbolic cotangent of a [`Rational`], rounding the result to the
    /// specified precision and with the specified rounding mode and returning the result as a
    /// [`Float`]. The [`Rational`] is taken by value. An [`Ordering`] is also returned, indicating
    /// whether the rounded hyperbolic cotangent is less than, equal to, or greater than the exact
    /// hyperbolic cotangent.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \coth x+\varepsilon.
    /// $$
    /// - If $m$ is not `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2 |\coth x|\rfloor-p+1}$.
    /// - If $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2 |\coth x|\rfloor-p}$.
    ///
    /// These bounds do not apply when the result overflows; see below.
    ///
    /// The output has precision `prec`.
    ///
    /// Special cases:
    /// - $f(0,p,m)=\infty$.
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
    /// Underflow is not possible, since $|\coth x| > 1$. Overflow happens only for inputs of
    /// magnitude at most about $2^{-2^{30}+1}$.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::coth_rational_prec`] instead.
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
    /// let (c, o) = Float::coth_rational_prec_round(Rational::from_unsigneds(3u8, 5), 5, Floor);
    /// assert_eq!(c.to_string(), "1.81");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::coth_rational_prec_round(Rational::from_unsigneds(3u8, 5), 5, Ceiling);
    /// assert_eq!(c.to_string(), "1.88");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = Float::coth_rational_prec_round(Rational::from_unsigneds(3u8, 5), 20, Floor);
    /// assert_eq!(c.to_string(), "1.8620243");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::coth_rational_prec_round(Rational::from_unsigneds(3u8, 5), 20, Ceiling);
    /// assert_eq!(c.to_string(), "1.8620262");
    /// assert_eq!(o, Greater);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn coth_rational_prec_round(x: Rational, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        Self::coth_rational_prec_round_ref(&x, prec, rm)
    }

    /// Computes $\coth x$, the hyperbolic cotangent of a [`Rational`], rounding the result to the
    /// specified precision and with the specified rounding mode and returning the result as a
    /// [`Float`]. The [`Rational`] is taken by reference. An [`Ordering`] is also returned,
    /// indicating whether the rounded hyperbolic cotangent is less than, equal to, or greater than
    /// the exact hyperbolic cotangent.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \coth x+\varepsilon.
    /// $$
    /// - If $m$ is not `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2 |\coth x|\rfloor-p+1}$.
    /// - If $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2 |\coth x|\rfloor-p}$.
    ///
    /// These bounds do not apply when the result overflows; see below.
    ///
    /// The output has precision `prec`.
    ///
    /// Special cases:
    /// - $f(0,p,m)=\infty$.
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
    /// Underflow is not possible, since $|\coth x| > 1$. Overflow happens only for inputs of
    /// magnitude at most about $2^{-2^{30}+1}$.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::coth_rational_prec_ref`]
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
    ///     Float::coth_rational_prec_round_ref(&Rational::from_unsigneds(3u8, 5), 5, Floor);
    /// assert_eq!(c.to_string(), "1.81");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) =
    ///     Float::coth_rational_prec_round_ref(&Rational::from_unsigneds(3u8, 5), 5, Ceiling);
    /// assert_eq!(c.to_string(), "1.88");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) =
    ///     Float::coth_rational_prec_round_ref(&Rational::from_unsigneds(3u8, 5), 20, Floor);
    /// assert_eq!(c.to_string(), "1.8620243");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) =
    ///     Float::coth_rational_prec_round_ref(&Rational::from_unsigneds(3u8, 5), 20, Ceiling);
    /// assert_eq!(c.to_string(), "1.8620262");
    /// assert_eq!(o, Greater);
    /// ```
    pub fn coth_rational_prec_round_ref(
        x: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        assert_ne!(prec, 0);
        if *x == 0u32 {
            // coth(0) = infinity, exactly
            return (Self::INFINITY, Equal);
        }
        coth_rational_helper(x, prec, rm)
    }

    /// Computes $\coth x$, the hyperbolic cotangent of a [`Rational`], rounding the result to the
    /// nearest value of the specified precision and returning the result as a [`Float`]. The
    /// [`Rational`] is taken by value. An [`Ordering`] is also returned, indicating whether the
    /// rounded hyperbolic cotangent is less than, equal to, or greater than the exact hyperbolic
    /// cotangent.
    ///
    /// If the hyperbolic cotangent is equidistant from two [`Float`]s with the specified precision,
    /// the [`Float`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a
    /// description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = \coth x+\varepsilon,
    /// $$
    /// where $|\varepsilon| \leq 2^{\lfloor\log_2 |\coth x|\rfloor-p}$ (unless the result
    /// overflows; see below).
    ///
    /// The output has precision `prec`.
    ///
    /// Special cases:
    /// - $f(0,p)=\infty$.
    ///
    /// Overflow:
    /// - If $f(x,p)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,p)\leq -2^{2^{30}-1}$, $-\infty$ is returned instead.
    ///
    /// Underflow is not possible, since $|\coth x| > 1$.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::coth_rational_prec_round`] instead.
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
    /// let (c, o) = Float::coth_rational_prec(Rational::from_unsigneds(3u8, 5), 5);
    /// assert_eq!(c.to_string(), "1.88");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = Float::coth_rational_prec(Rational::from_unsigneds(3u8, 5), 20);
    /// assert_eq!(c.to_string(), "1.8620262");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = Float::coth_rational_prec(Rational::ZERO, 10);
    /// assert_eq!(c.to_string(), "Infinity");
    /// assert_eq!(o, Equal);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn coth_rational_prec(x: Rational, prec: u64) -> (Self, Ordering) {
        Self::coth_rational_prec_round_ref(&x, prec, Nearest)
    }

    /// Computes $\coth x$, the hyperbolic cotangent of a [`Rational`], rounding the result to the
    /// nearest value of the specified precision and returning the result as a [`Float`]. The
    /// [`Rational`] is taken by reference. An [`Ordering`] is also returned, indicating whether the
    /// rounded hyperbolic cotangent is less than, equal to, or greater than the exact hyperbolic
    /// cosine.
    ///
    /// If the hyperbolic cotangent is equidistant from two [`Float`]s with the specified precision,
    /// the [`Float`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a
    /// description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = \coth x+\varepsilon,
    /// $$
    /// where $|\varepsilon| \leq 2^{\lfloor\log_2 |\coth x|\rfloor-p}$ (unless the result
    /// overflows; see below).
    ///
    /// The output has precision `prec`.
    ///
    /// Special cases:
    /// - $f(0,p)=\infty$.
    ///
    /// Overflow:
    /// - If $f(x,p)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,p)\leq -2^{2^{30}-1}$, $-\infty$ is returned instead.
    ///
    /// Underflow is not possible, since $|\coth x| > 1$.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::coth_rational_prec_round_ref`] instead.
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
    /// let (c, o) = Float::coth_rational_prec_ref(&Rational::from_unsigneds(3u8, 5), 5);
    /// assert_eq!(c.to_string(), "1.88");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = Float::coth_rational_prec_ref(&Rational::from_unsigneds(3u8, 5), 20);
    /// assert_eq!(c.to_string(), "1.8620262");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = Float::coth_rational_prec_ref(&Rational::ZERO, 10);
    /// assert_eq!(c.to_string(), "Infinity");
    /// assert_eq!(o, Equal);
    /// ```
    #[inline]
    pub fn coth_rational_prec_ref(x: &Rational, prec: u64) -> (Self, Ordering) {
        Self::coth_rational_prec_round_ref(x, prec, Nearest)
    }
}

impl Coth for Float {
    type Output = Self;

    /// Computes $\coth x$, the hyperbolic cotangent of a [`Float`], taking it by value.
    ///
    /// If the output has a precision, it is the precision of the input. If the hyperbolic cotangent
    /// is equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s
    /// in its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// $$
    /// f(x) = \coth x+\varepsilon.
    /// $$
    /// - If $\coth x$ is infinite or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $\coth x$ is finite, then $|\varepsilon| < 2^{\lfloor\log_2 |\coth x|\rfloor-p}$, where
    ///   $p$ is the precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN})=\text{NaN}$
    /// - $f(\infty)=1.0$
    /// - $f(-\infty)=-1.0$
    /// - $f(0.0)=\infty$
    /// - $f(-0.0)=-\infty$
    ///
    /// See the [`Float::coth_round`] documentation for information on overflow and underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::coth_round`] instead. If you want to specify the output precision, consider using
    /// [`Float::coth_prec`]. If you want both of these things, consider using
    /// [`Float::coth_prec_round`].
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
    /// use malachite_base::num::arithmetic::traits::Coth;
    /// use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity};
    /// use malachite_float::Float;
    ///
    /// assert!(Float::NAN.coth().is_nan());
    /// assert_eq!(Float::INFINITY.coth(), 1);
    /// assert_eq!(Float::NEGATIVE_INFINITY.coth(), -1);
    /// assert_eq!(
    ///     Float::from_unsigned_prec(1u32, 100).0.coth().to_string(),
    ///     "1.3130352854993313036361612469313"
    /// );
    /// ```
    #[inline]
    fn coth(self) -> Self {
        let prec = self.significant_bits();
        self.coth_prec_round(prec, Nearest).0
    }
}

impl Coth for &Float {
    type Output = Float;

    /// Computes $\coth x$, the hyperbolic cotangent of a [`Float`], taking it by reference.
    ///
    /// If the output has a precision, it is the precision of the input. If the hyperbolic cotangent
    /// is equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s
    /// in its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// $$
    /// f(x) = \coth x+\varepsilon.
    /// $$
    /// - If $\coth x$ is infinite or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $\coth x$ is finite, then $|\varepsilon| < 2^{\lfloor\log_2 |\coth x|\rfloor-p}$, where
    ///   $p$ is the precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN})=\text{NaN}$
    /// - $f(\infty)=1.0$
    /// - $f(-\infty)=-1.0$
    /// - $f(0.0)=\infty$
    /// - $f(-0.0)=-\infty$
    ///
    /// See the [`Float::coth_round`] documentation for information on overflow and underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::coth_round_ref`] instead. If you want to specify the output precision, consider
    /// using [`Float::coth_prec_ref`]. If you want both of these things, consider using
    /// [`Float::coth_prec_round_ref`].
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
    /// use malachite_base::num::arithmetic::traits::Coth;
    /// use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity};
    /// use malachite_float::Float;
    ///
    /// assert!((&Float::NAN).coth().is_nan());
    /// assert_eq!((&Float::INFINITY).coth(), 1);
    /// assert_eq!((&Float::NEGATIVE_INFINITY).coth(), -1);
    /// assert_eq!(
    ///     (&Float::from_unsigned_prec(1u32, 100).0).coth().to_string(),
    ///     "1.3130352854993313036361612469313"
    /// );
    /// ```
    #[inline]
    fn coth(self) -> Float {
        self.coth_prec_round_ref(self.significant_bits(), Nearest).0
    }
}

impl CothAssign for Float {
    /// Computes $\coth x$, the hyperbolic cotangent of a [`Float`], in place.
    ///
    /// If the output has a precision, it is the precision of the input. If the hyperbolic cotangent
    /// is equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s
    /// in its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// $$
    /// x \gets \coth x+\varepsilon.
    /// $$
    /// - If $\coth x$ is infinite or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $\coth x$ is finite, then $|\varepsilon| < 2^{\lfloor\log_2 |\coth x|\rfloor-p}$, where
    ///   $p$ is the precision of the input.
    ///
    /// See the [`Float::coth`] documentation for information on special cases, overflow, and
    /// underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::coth_round_assign`] instead. If you want to specify the output precision, consider
    /// using [`Float::coth_prec_assign`]. If you want both of these things, consider using
    /// [`Float::coth_prec_round_assign`].
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
    /// use malachite_base::num::arithmetic::traits::CothAssign;
    /// use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity};
    /// use malachite_float::Float;
    ///
    /// let mut x = Float::NAN;
    /// x.coth_assign();
    /// assert!(x.is_nan());
    ///
    /// let mut x = Float::INFINITY;
    /// x.coth_assign();
    /// assert_eq!(x, 1);
    ///
    /// let mut x = Float::NEGATIVE_INFINITY;
    /// x.coth_assign();
    /// assert_eq!(x, -1);
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// x.coth_assign();
    /// assert_eq!(x.to_string(), "1.3130352854993313036361612469313");
    /// ```
    #[inline]
    fn coth_assign(&mut self) {
        let prec = self.significant_bits();
        self.coth_prec_round_assign(prec, Nearest);
    }
}

/// Computes $\coth x$, the hyperbolic cotangent of a primitive float. The result is correctly
/// rounded.
///
/// $$
/// f(x) = \coth x+\varepsilon.
/// $$
/// - If $\coth x$ is infinite or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
/// - If $\coth x$ is finite, then $|\varepsilon| < 2^{\lfloor\log_2 |\coth x|\rfloor-p}$, where $p$
///   is the precision of the output (24 if `T` is a [`f32`] and 53 if `T` is a [`f64`]).
///
/// Special cases:
/// - $f(\text{NaN})=\text{NaN}$
/// - $f(\infty)=1.0$
/// - $f(-\infty)=-1.0$
/// - $f(0.0)=\infty$
/// - $f(-0.0)=-\infty$
///
/// An `x` of magnitude below the reciprocal of the largest finite value, such as a subnormal, gives
/// a result that overflows to $\pm\infty$. Underflow is not possible, since $|\coth x| > 1$.
///
/// # Worst-case complexity
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use malachite_base::num::basic::traits::NegativeInfinity;
/// use malachite_base::num::float::NiceFloat;
/// use malachite_float::float::arithmetic::coth::primitive_float_coth;
///
/// assert!(primitive_float_coth(f32::NAN).is_nan());
/// assert_eq!(
///     NiceFloat(primitive_float_coth(f32::INFINITY)),
///     NiceFloat(1.0)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_coth(-0.0f32)),
///     NiceFloat(f32::NEGATIVE_INFINITY)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_coth(1.0f32)),
///     NiceFloat(1.3130352)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_coth(-1.0f64)),
///     NiceFloat(-1.3130352854993312)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_coth(10.0f64)),
///     NiceFloat(1.0000000041223072)
/// );
/// assert_eq!(NiceFloat(primitive_float_coth(20.0f64)), NiceFloat(1.0));
/// assert_eq!(
///     NiceFloat(primitive_float_coth(5.0e-309f64)),
///     NiceFloat(f64::INFINITY)
/// );
/// ```
#[inline]
#[allow(clippy::type_repetition_in_bounds)]
pub fn primitive_float_coth<T: PrimitiveFloat>(x: T) -> T
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    emulate_float_to_float_fn(Float::coth_prec, x)
}

/// Computes $\coth x$, the hyperbolic cotangent of a [`Rational`], returning the result as a
/// primitive float. The result is correctly rounded.
///
/// $$
/// f(x) = \coth x+\varepsilon.
/// $$
/// - If $\coth x$ is infinite, $\varepsilon$ may be ignored or assumed to be 0.
/// - If $\coth x$ is finite, then $|\varepsilon| < 2^{\lfloor\log_2 |\coth x|\rfloor-p}$, where $p$
///   is the precision of the output (24 if `T` is a [`f32`] and 53 if `T` is a [`f64`]).
///
/// Special cases:
/// - $f(0)=\infty$
///
/// An `x` of magnitude below the reciprocal of the largest finite value gives a result that
/// overflows to $\pm\infty$. Underflow is not possible, since $|\coth x| > 1$.
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
/// use malachite_float::float::arithmetic::coth::primitive_float_coth_rational;
/// use malachite_q::Rational;
///
/// assert_eq!(
///     NiceFloat(primitive_float_coth_rational::<f64>(&Rational::ZERO)),
///     NiceFloat(f64::INFINITY)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_coth_rational::<f64>(
///         &Rational::from_unsigneds(1u8, 3)
///     )),
///     NiceFloat(3.110296679619444)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_coth_rational::<f64>(
///         &Rational::from_signeds(-1i8, 3)
///     )),
///     NiceFloat(-3.110296679619444)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_coth_rational::<f64>(&Rational::from(10000))),
///     NiceFloat(1.0)
/// );
/// ```
#[inline]
#[allow(clippy::type_repetition_in_bounds)]
pub fn primitive_float_coth_rational<T: PrimitiveFloat>(x: &Rational) -> T
where
    Float: PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    emulate_rational_to_float_fn(Float::coth_rational_prec_ref, x)
}
