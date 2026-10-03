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
use crate::float::arithmetic::cosh::{
    cosh_rational_helper, hyperbolic_approx, hyperbolic_can_round,
};
use crate::float::arithmetic::round_near_x::small_input_shortcut;
use crate::float::arithmetic::sinh::sinh_rational_helper;
use crate::float::conversion::string::set_str::overflow;
use crate::{
    Float, emulate_float_to_float_pair_fn, emulate_rational_to_float_pair_fn, floor_and_ceiling,
};
use core::cmp::Ordering::{self, Equal};
use malachite_base::num::arithmetic::traits::{Abs, CeilingLogBase2, SinhCosh, SinhCoshAssign};
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::{
    Infinity as InfinityTrait, NaN as NaNTrait, One, Zero as ZeroTrait,
};
use malachite_base::num::conversion::traits::{ExactFrom, RoundingFrom};
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{self, Exact, Floor, Nearest};
use malachite_nz::platform::Limb;
use malachite_q::Rational;

// This is mpfr_sinh_cosh from sinh_cosh.c, MPFR 4.2.2, where the input is finite and nonzero, with
// two differences. First, MPFR has no small-input shortcut and does not raise its working precision
// for the cancellation in exp(x) - exp(-x) at a small x, so for a tiny x its Ziv loop has to grow
// the precision until it covers about -2 EXP(x) bits, computing exp(x) to about 2^30 bits for an x
// near the smallest Float. Here the shortcuts of `sinh` and `cosh` are tried first, and the working
// precision is raised as in `mpfr_sinh`. Second, MPFR's overflow branch relies on its extended
// exponent range; here the near-overflow handling of `cosh` and `sinh` is used instead. Both
// results have precision `prec`, where MPFR allows two precisions and works at the larger.
fn sinh_cosh_prec_round_normal_ref(
    x: &Float,
    prec: u64,
    rm: RoundingMode,
) -> (Float, Float, Ordering, Ordering) {
    assert_ne!(rm, Exact, "Inexact sinh_cosh");
    let exp_x = i64::from(x.get_exponent().unwrap());
    // For x small, sinh(x) = x + x^3/6 + ... has error below 2^(3 EXP(x) - 2), and cosh(x) = 1 +
    // x^2/2 + ... has error below 2^(2 EXP(x)), so both may round from x and 1 alone. The cosine's
    // bound is the weaker one, and its reference value 1 always rounds once the bound is small
    // enough, so it decides, and is tried first.
    let neg_two_exp = -(exp_x << 1);
    if let Some((c, o_c)) = small_input_shortcut(&Float::ONE, neg_two_exp, 0, true, prec, rm)
        && let Some((s, o_s)) = small_input_shortcut(x, neg_two_exp, 2, true, prec, rm)
    {
        return (s, c, o_s, o_c);
    }
    let positive = x.is_sign_positive();
    let x_abs = x.abs();
    // the optimal number of bits : see algorithms.ps
    let mut working_prec = prec + prec.ceiling_log_base_2() + 4;
    // If x is near 0, exp(x) - 1/exp(x) = 2*x+x^3/3+O(x^5), so the subtraction loses about -2
    // EXP(x) bits.
    if exp_x < 0 {
        working_prec += u64::exact_from(neg_two_exp);
    }
    let mut increment = Limb::WIDTH;
    loop {
        let Some(approx) = hyperbolic_approx(&x_abs, working_prec) else {
            // exp(|x|) / 2 overflows, and so do cosh(x) and sinh(x)
            let (s, o_s) = overflow(positive, prec, rm);
            let (c, o_c) = overflow(true, prec, rm);
            return (s, c, o_s, o_c);
        };
        if hyperbolic_can_round(&approx.sinh, approx.sinh_bits, prec, rm)
            && hyperbolic_can_round(&approx.cosh, approx.cosh_bits, prec, rm)
        {
            let sinh_abs = approx.sinh;
            let (s, o_s) =
                Float::from_float_prec_round(if positive { sinh_abs } else { -sinh_abs }, prec, rm);
            let (c, o_c) = Float::from_float_prec_round(approx.cosh, prec, rm);
            return (s, c, o_s, o_c);
        }
        working_prec += increment;
        increment = working_prec >> 1;
    }
}

// Computes sinh(x) and cosh(x) for a nonzero `Rational` x, rounded to precision `prec` with
// rounding mode `rm`. Both are transcendental for every nonzero rational x, so neither result is
// exact.
fn sinh_cosh_rational_helper(
    x: &Rational,
    prec: u64,
    rm: RoundingMode,
) -> (Float, Float, Ordering, Ordering) {
    assert_ne!(rm, Exact, "Inexact sinh_cosh");
    let exp_x = x.floor_log_base_2_abs() + 1; // the MPFR-style exponent of x
    // For an x small enough that the hyperbolic sine comes from its series (or underflows), both
    // results come cheaply from the separate paths, which share no exponential: the hyperbolic
    // cosine then rounds from 1, or, at a high precision, brackets a small x. This also covers
    // every x too small to be a `Float`.
    if exp_x < -1 && u64::exact_from(-exp_x) << 4 >= prec + 10 {
        let (s, o_s) = sinh_rational_helper(x, prec, rm);
        let (c, o_c) = cosh_rational_helper(x, prec, rm);
        return (s, c, o_s, o_c);
    }
    // |x| >= 2^(MAX_EXPONENT - 1), so both results overflow
    if exp_x >= Float::MAX_EXPONENT_I64 {
        let (s, o_s) = overflow(*x > 0u32, prec, rm);
        let (c, o_c) = overflow(true, prec, rm);
        return (s, c, o_s, o_c);
    }
    // Bracket x between the Floats x_lo <= x <= x_hi, which have the sign of x since x is not
    // small. sinh is increasing, and cosh is monotonic on any interval not containing 0, so the
    // exact values lie between those at the two ends; increase the working precision until both
    // pairs of ends round to the same results.
    let mut working_prec = prec + 10;
    let mut increment = Limb::WIDTH;
    loop {
        let (x_lo, x_o) = Float::from_rational_prec_round_ref(x, working_prec, Floor);
        if x_o == Equal {
            // x is exactly representable at `working_prec`
            return sinh_cosh_prec_round_normal_ref(&x_lo, prec, rm);
        }
        let (x_lo, x_hi) = floor_and_ceiling((x_lo, x_o));
        // The hyperbolic sine and cosine of a finite nonzero Float are never exact, so the
        // orderings are `Less` or `Greater`, never `Equal`.
        let (s_lo, c_lo, o_s_lo, o_c_lo) = sinh_cosh_prec_round_normal_ref(&x_lo, prec, rm);
        let (s_hi, c_hi, o_s_hi, o_c_hi) = sinh_cosh_prec_round_normal_ref(&x_hi, prec, rm);
        if o_s_lo == o_s_hi && o_c_lo == o_c_hi && s_lo == s_hi && c_lo == c_hi {
            return (s_lo, c_lo, o_s_lo, o_c_lo);
        }
        working_prec += increment;
        increment = working_prec >> 1;
    }
}

impl Float {
    /// Computes $\sinh x$ and $\cosh x$, the hyperbolic sine and cosine of a [`Float`], together,
    /// rounding both results to the specified precision and with the specified rounding mode. The
    /// [`Float`] is taken by value. Two [`Ordering`]s are also returned, indicating whether the
    /// rounded hyperbolic sine and cosine are less than, equal to, or greater than the exact
    /// values. Although `NaN`s are not comparable to any [`Float`], whenever this function returns
    /// a `NaN` it also returns `Equal` for it.
    ///
    /// The results are the same as those of [`Float::sinh_prec_round`] and
    /// [`Float::cosh_prec_round`], but they share a single exponential, so this is faster than the
    /// two calls when both values are needed.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = (\sinh x+\varepsilon_s, \cosh x+\varepsilon_c).
    /// $$
    /// - If a result is infinite, zero, or `NaN`, its $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If the results are finite and nonzero and $m$ is not `Nearest`, then $|\varepsilon_s| <
    ///   2^{\lfloor\log_2 |\sinh x|\rfloor-p+1}$ and $|\varepsilon_c| < 2^{\lfloor\log_2 \cosh
    ///   x\rfloor-p+1}$.
    /// - If the results are finite and nonzero and $m$ is `Nearest`, then $|\varepsilon_s| \leq
    ///   2^{\lfloor\log_2 |\sinh x|\rfloor-p}$ and $|\varepsilon_c| \leq 2^{\lfloor\log_2 \cosh
    ///   x\rfloor-p}$.
    ///
    /// If the outputs have a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=(\text{NaN},\text{NaN})$
    /// - $f(\infty,p,m)=(\infty,\infty)$
    /// - $f(-\infty,p,m)=(-\infty,\infty)$
    /// - $f(\pm0.0,p,m)=(\pm0.0,1.0)$
    ///
    /// Overflow:
    /// - Each result overflows exactly as [`Float::sinh_prec_round`] or [`Float::cosh_prec_round`]
    ///   does, which happens when $|x|$ exceeds about $(2^{30}-1)\log 2$. See those functions for
    ///   the values returned.
    /// - Since $|\sinh x|\geq|x|$ and $\cosh x\geq 1$, neither result underflows.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::sinh_cosh_prec`] instead. If
    /// you know that your target precision is the precision of the input, consider using
    /// [`Float::sinh_cosh_round`] instead. If both of these things are true, consider using
    /// [`Float::sinh_cosh`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O((n+m)^{3/2} \log (n+m) \log\log (n+m))$
    ///
    /// $M(n, m) = O((n+m) \log (n+m))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `self.significant_bits()`: the exponential is computed at a working precision of `prec` plus
    /// the bits lost to cancellation in the hyperbolic sine of a small input, which is at most
    /// about the input's precision when the small-input shortcuts do not apply.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` and `self` is finite and nonzero, since the hyperbolic sine and
    /// cosine of a finite nonzero [`Float`] are never exactly representable, or if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (s, c, o_s, o_c) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .sinh_cosh_prec_round(5, Floor);
    /// assert_eq!(s.to_string(), "1.12");
    /// assert_eq!(c.to_string(), "1.50");
    /// assert_eq!(o_s, Less);
    /// assert_eq!(o_c, Less);
    ///
    /// let (s, c, o_s, o_c) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .sinh_cosh_prec_round(5, Ceiling);
    /// assert_eq!(s.to_string(), "1.19");
    /// assert_eq!(c.to_string(), "1.56");
    /// assert_eq!(o_s, Greater);
    /// assert_eq!(o_c, Greater);
    ///
    /// let (s, c, o_s, o_c) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .sinh_cosh_prec_round(20, Nearest);
    /// assert_eq!(s.to_string(), "1.1752014");
    /// assert_eq!(c.to_string(), "1.5430813");
    /// assert_eq!(o_s, Greater);
    /// assert_eq!(o_c, Greater);
    /// ```
    #[inline]
    pub fn sinh_cosh_prec_round(
        self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Self, Ordering, Ordering) {
        self.sinh_cosh_prec_round_ref(prec, rm)
    }

    /// Computes $\sinh x$ and $\cosh x$, the hyperbolic sine and cosine of a [`Float`], together,
    /// rounding both results to the specified precision and with the specified rounding mode. The
    /// [`Float`] is taken by reference. Two [`Ordering`]s are also returned, indicating whether the
    /// rounded hyperbolic sine and cosine are less than, equal to, or greater than the exact
    /// values.
    ///
    /// See [`Float::sinh_cosh_prec_round`] for the error bounds, the special cases, overflow, and
    /// the complexity; this function behaves the same way.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` and `self` is finite and nonzero, since the hyperbolic sine and
    /// cosine of a finite nonzero [`Float`] are never exactly representable, or if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from_unsigned_prec(1u32, 100).0;
    /// let (s, c, o_s, o_c) = x.sinh_cosh_prec_round_ref(5, Floor);
    /// assert_eq!(s.to_string(), "1.12");
    /// assert_eq!(c.to_string(), "1.50");
    /// assert_eq!(o_s, Less);
    /// assert_eq!(o_c, Less);
    ///
    /// let (s, c, o_s, o_c) = x.sinh_cosh_prec_round_ref(20, Nearest);
    /// assert_eq!(s.to_string(), "1.1752014");
    /// assert_eq!(c.to_string(), "1.5430813");
    /// assert_eq!(o_s, Greater);
    /// assert_eq!(o_c, Greater);
    /// ```
    pub fn sinh_cosh_prec_round_ref(
        &self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Self, Ordering, Ordering) {
        assert_ne!(prec, 0);
        match &self.0 {
            NaN => (Self::NAN, Self::NAN, Equal, Equal),
            // sinh(±inf) = ±inf and cosh(±inf) = inf, exactly
            Infinity { .. } => (self.clone(), Self::INFINITY, Equal, Equal),
            // sinh(±0) = ±0 and cosh(±0) = 1, exactly
            Zero { .. } => (self.clone(), Self::one_prec(prec), Equal, Equal),
            Finite { .. } => sinh_cosh_prec_round_normal_ref(self, prec, rm),
        }
    }

    /// Computes $\sinh x$ and $\cosh x$, the hyperbolic sine and cosine of a [`Float`], together,
    /// rounding both results to the nearest value of the specified precision. The [`Float`] is
    /// taken by value. Two [`Ordering`]s are also returned, indicating whether the rounded
    /// hyperbolic sine and cosine are less than, equal to, or greater than the exact values.
    ///
    /// If a result is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// See [`Float::sinh_cosh_prec_round`] for the error bounds, the special cases, overflow, and
    /// the complexity; this function behaves the same way with `Nearest`.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::sinh_cosh_prec_round`] instead. If you know that your target precision is the
    /// precision of the input, consider using [`Float::sinh_cosh`] instead.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (s, c, o_s, o_c) = Float::from_unsigned_prec(1u32, 100).0.sinh_cosh_prec(5);
    /// assert_eq!(s.to_string(), "1.19");
    /// assert_eq!(c.to_string(), "1.56");
    /// assert_eq!(o_s, Greater);
    /// assert_eq!(o_c, Greater);
    ///
    /// let (s, c, o_s, o_c) = Float::from_unsigned_prec(1u32, 100).0.sinh_cosh_prec(20);
    /// assert_eq!(s.to_string(), "1.1752014");
    /// assert_eq!(c.to_string(), "1.5430813");
    /// assert_eq!(o_s, Greater);
    /// assert_eq!(o_c, Greater);
    /// ```
    #[inline]
    pub fn sinh_cosh_prec(self, prec: u64) -> (Self, Self, Ordering, Ordering) {
        self.sinh_cosh_prec_round_ref(prec, Nearest)
    }

    /// Computes $\sinh x$ and $\cosh x$, the hyperbolic sine and cosine of a [`Float`], together,
    /// rounding both results to the nearest value of the specified precision. The [`Float`] is
    /// taken by reference. Two [`Ordering`]s are also returned, indicating whether the rounded
    /// hyperbolic sine and cosine are less than, equal to, or greater than the exact values.
    ///
    /// If a result is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// See [`Float::sinh_cosh_prec_round`] for the error bounds, the special cases, overflow, and
    /// the complexity; this function behaves the same way with `Nearest`.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::sinh_cosh_prec_round_ref`] instead. If you know that your target precision is the
    /// precision of the input, consider using `(&Float).sinh_cosh()` instead.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (s, c, o_s, o_c) = Float::from_unsigned_prec(1u32, 100).0.sinh_cosh_prec_ref(5);
    /// assert_eq!(s.to_string(), "1.19");
    /// assert_eq!(c.to_string(), "1.56");
    /// assert_eq!(o_s, Greater);
    /// assert_eq!(o_c, Greater);
    ///
    /// let (s, c, o_s, o_c) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .sinh_cosh_prec_ref(20);
    /// assert_eq!(s.to_string(), "1.1752014");
    /// assert_eq!(c.to_string(), "1.5430813");
    /// assert_eq!(o_s, Greater);
    /// assert_eq!(o_c, Greater);
    /// ```
    #[inline]
    pub fn sinh_cosh_prec_ref(&self, prec: u64) -> (Self, Self, Ordering, Ordering) {
        self.sinh_cosh_prec_round_ref(prec, Nearest)
    }

    /// Computes $\sinh x$ and $\cosh x$, the hyperbolic sine and cosine of a [`Float`], together,
    /// rounding both results to the precision of the input and with the specified rounding mode.
    /// The [`Float`] is taken by value. Two [`Ordering`]s are also returned, indicating whether the
    /// rounded hyperbolic sine and cosine are less than, equal to, or greater than the exact
    /// values.
    ///
    /// See [`Float::sinh_cosh_prec_round`] for the error bounds, the special cases, and overflow;
    /// this function behaves the same way, with the precision of the input.
    ///
    /// If you want to specify an output precision, consider using [`Float::sinh_cosh_prec_round`]
    /// instead. If you know you'll be using the `Nearest` rounding mode, consider using
    /// [`Float::sinh_cosh`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` and `self` is finite and nonzero, since the hyperbolic sine and
    /// cosine of a finite nonzero [`Float`] are never exactly representable.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (s, c, o_s, o_c) = Float::from_unsigned_prec(1u32, 5).0.sinh_cosh_round(Floor);
    /// assert_eq!(s.to_string(), "1.12");
    /// assert_eq!(c.to_string(), "1.50");
    /// assert_eq!(o_s, Less);
    /// assert_eq!(o_c, Less);
    ///
    /// let (s, c, o_s, o_c) = Float::from_unsigned_prec(1u32, 5)
    ///     .0
    ///     .sinh_cosh_round(Ceiling);
    /// assert_eq!(s.to_string(), "1.19");
    /// assert_eq!(c.to_string(), "1.56");
    /// assert_eq!(o_s, Greater);
    /// assert_eq!(o_c, Greater);
    /// ```
    #[inline]
    pub fn sinh_cosh_round(self, rm: RoundingMode) -> (Self, Self, Ordering, Ordering) {
        let prec = self.significant_bits();
        self.sinh_cosh_prec_round_ref(prec, rm)
    }

    /// Computes $\sinh x$ and $\cosh x$, the hyperbolic sine and cosine of a [`Float`], together,
    /// rounding both results to the precision of the input and with the specified rounding mode.
    /// The [`Float`] is taken by reference. Two [`Ordering`]s are also returned, indicating whether
    /// the rounded hyperbolic sine and cosine are less than, equal to, or greater than the exact
    /// values.
    ///
    /// See [`Float::sinh_cosh_prec_round`] for the error bounds, the special cases, and overflow;
    /// this function behaves the same way, with the precision of the input.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::sinh_cosh_prec_round_ref`] instead. If you know you'll be using the `Nearest`
    /// rounding mode, consider using `(&Float).sinh_cosh()` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` and `self` is finite and nonzero, since the hyperbolic sine and
    /// cosine of a finite nonzero [`Float`] are never exactly representable.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (s, c, o_s, o_c) = Float::from_unsigned_prec(1u32, 5)
    ///     .0
    ///     .sinh_cosh_round_ref(Floor);
    /// assert_eq!(s.to_string(), "1.12");
    /// assert_eq!(c.to_string(), "1.50");
    /// assert_eq!(o_s, Less);
    /// assert_eq!(o_c, Less);
    ///
    /// let (s, c, o_s, o_c) = Float::from_unsigned_prec(1u32, 5)
    ///     .0
    ///     .sinh_cosh_round_ref(Ceiling);
    /// assert_eq!(s.to_string(), "1.19");
    /// assert_eq!(c.to_string(), "1.56");
    /// assert_eq!(o_s, Greater);
    /// assert_eq!(o_c, Greater);
    /// ```
    #[inline]
    pub fn sinh_cosh_round_ref(&self, rm: RoundingMode) -> (Self, Self, Ordering, Ordering) {
        self.sinh_cosh_prec_round_ref(self.significant_bits(), rm)
    }

    /// Replaces a [`Float`] with its hyperbolic sine and writes its hyperbolic cosine to `cosh`,
    /// rounding both results to the specified precision and with the specified rounding mode. The
    /// previous value of `cosh` is discarded. Two [`Ordering`]s are returned, indicating whether
    /// the rounded hyperbolic sine and cosine are less than, equal to, or greater than the exact
    /// values.
    ///
    /// See [`Float::sinh_cosh_prec_round`]; this function behaves the same way.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` and `self` is finite and nonzero, since the hyperbolic sine and
    /// cosine of a finite nonzero [`Float`] are never exactly representable, or if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::NaN;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// let mut c = Float::NAN;
    /// assert_eq!(
    ///     x.sinh_cosh_prec_round_assign(&mut c, 5, Floor),
    ///     (Less, Less)
    /// );
    /// assert_eq!(x.to_string(), "1.12");
    /// assert_eq!(c.to_string(), "1.50");
    /// ```
    #[inline]
    pub fn sinh_cosh_prec_round_assign(
        &mut self,
        cosh: &mut Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Ordering, Ordering) {
        let (s, c, o_s, o_c) = self.sinh_cosh_prec_round_ref(prec, rm);
        *self = s;
        *cosh = c;
        (o_s, o_c)
    }

    /// Replaces a [`Float`] with its hyperbolic sine and writes its hyperbolic cosine to `cosh`,
    /// rounding both results to the nearest value of the specified precision. The previous value of
    /// `cosh` is discarded. Two [`Ordering`]s are returned, indicating whether the rounded
    /// hyperbolic sine and cosine are less than, equal to, or greater than the exact values.
    ///
    /// See [`Float::sinh_cosh_prec`]; this function behaves the same way.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::NaN;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// let mut c = Float::NAN;
    /// assert_eq!(x.sinh_cosh_prec_assign(&mut c, 20), (Greater, Greater));
    /// assert_eq!(x.to_string(), "1.1752014");
    /// assert_eq!(c.to_string(), "1.5430813");
    /// ```
    #[inline]
    pub fn sinh_cosh_prec_assign(&mut self, cosh: &mut Self, prec: u64) -> (Ordering, Ordering) {
        self.sinh_cosh_prec_round_assign(cosh, prec, Nearest)
    }

    /// Replaces a [`Float`] with its hyperbolic sine and writes its hyperbolic cosine to `cosh`,
    /// rounding both results to the precision of the input and with the specified rounding mode.
    /// The previous value of `cosh` is discarded. Two [`Ordering`]s are returned, indicating
    /// whether the rounded hyperbolic sine and cosine are less than, equal to, or greater than the
    /// exact values.
    ///
    /// See [`Float::sinh_cosh_round`] and [`Float::sinh_cosh_prec_round`]; this function behaves
    /// the same way.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` and `self` is finite and nonzero, since the hyperbolic sine and
    /// cosine of a finite nonzero [`Float`] are never exactly representable.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::NaN;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 5).0;
    /// let mut c = Float::NAN;
    /// assert_eq!(x.sinh_cosh_round_assign(&mut c, Floor), (Less, Less));
    /// assert_eq!(x.to_string(), "1.12");
    /// assert_eq!(c.to_string(), "1.50");
    /// ```
    #[inline]
    pub fn sinh_cosh_round_assign(
        &mut self,
        cosh: &mut Self,
        rm: RoundingMode,
    ) -> (Ordering, Ordering) {
        let prec = self.significant_bits();
        self.sinh_cosh_prec_round_assign(cosh, prec, rm)
    }
}

impl Float {
    /// Computes $\sinh x$ and $\cosh x$, the hyperbolic sine and cosine of a [`Rational`],
    /// together, rounding both results to the specified precision and with the specified rounding
    /// mode, and returning the results as [`Float`]s. The [`Rational`] is taken by value. Two
    /// [`Ordering`]s are also returned, indicating whether the rounded hyperbolic sine and cosine
    /// are less than, equal to, or greater than the exact values.
    ///
    /// The results are the same as those of [`Float::sinh_rational_prec_round`] and
    /// [`Float::cosh_rational_prec_round`], but they share their exponentials, so this is faster
    /// than the two calls when both values are needed.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = (\sinh x+\varepsilon_s, \cosh x+\varepsilon_c).
    /// $$
    /// - If $m$ is not `Nearest`, then $|\varepsilon_s| < 2^{\lfloor\log_2 |\sinh x|\rfloor-p+1}$
    ///   and $|\varepsilon_c| < 2^{\lfloor\log_2 \cosh x\rfloor-p+1}$.
    /// - If $m$ is `Nearest`, then $|\varepsilon_s| \leq 2^{\lfloor\log_2 |\sinh x|\rfloor-p}$ and
    ///   $|\varepsilon_c| \leq 2^{\lfloor\log_2 \cosh x\rfloor-p}$.
    ///
    /// These bounds do not apply to a result that overflows or underflows; see below.
    ///
    /// The outputs have precision `prec`.
    ///
    /// Special cases:
    /// - $f(0,p,m)=(0.0,1.0)$.
    ///
    /// Overflow and underflow:
    /// - Each result overflows exactly as [`Float::sinh_rational_prec_round`] or
    ///   [`Float::cosh_rational_prec_round`] does, which happens when $|x|$ exceeds about
    ///   $(2^{30}-1)\log 2$. See those functions for the values returned.
    /// - The hyperbolic sine underflows exactly as [`Float::sinh_rational_prec_round`] does, which
    ///   requires an input of magnitude below $2^{-2^{30}}$; the hyperbolic cosine never
    ///   underflows.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::sinh_cosh_rational_prec`]
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
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the results cannot be represented
    /// exactly with the given precision (which is the case for every nonzero input).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (s, c, o_s, o_c) =
    ///     Float::sinh_cosh_rational_prec_round(Rational::from_unsigneds(3u8, 5), 5, Floor);
    /// assert_eq!(s.to_string(), "0.625");
    /// assert_eq!(c.to_string(), "1.12");
    /// assert_eq!(o_s, Less);
    /// assert_eq!(o_c, Less);
    ///
    /// let (s, c, o_s, o_c) =
    ///     Float::sinh_cosh_rational_prec_round(Rational::from_signeds(-3i8, 5), 20, Ceiling);
    /// assert_eq!(s.to_string(), "-0.63665295");
    /// assert_eq!(c.to_string(), "1.1854668");
    /// assert_eq!(o_s, Greater);
    /// assert_eq!(o_c, Greater);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn sinh_cosh_rational_prec_round(
        x: Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Self, Ordering, Ordering) {
        Self::sinh_cosh_rational_prec_round_ref(&x, prec, rm)
    }

    /// Computes $\sinh x$ and $\cosh x$, the hyperbolic sine and cosine of a [`Rational`],
    /// together, rounding both results to the specified precision and with the specified rounding
    /// mode, and returning the results as [`Float`]s. The [`Rational`] is taken by reference. Two
    /// [`Ordering`]s are also returned, indicating whether the rounded hyperbolic sine and cosine
    /// are less than, equal to, or greater than the exact values.
    ///
    /// See [`Float::sinh_cosh_rational_prec_round`] for the error bounds, the special cases,
    /// overflow and underflow, and the complexity; this function behaves the same way.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the results cannot be represented
    /// exactly with the given precision (which is the case for every nonzero input).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (s, c, o_s, o_c) =
    ///     Float::sinh_cosh_rational_prec_round_ref(&Rational::from_unsigneds(3u8, 5), 5, Floor);
    /// assert_eq!(s.to_string(), "0.625");
    /// assert_eq!(c.to_string(), "1.12");
    /// assert_eq!(o_s, Less);
    /// assert_eq!(o_c, Less);
    /// ```
    pub fn sinh_cosh_rational_prec_round_ref(
        x: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Self, Ordering, Ordering) {
        assert_ne!(prec, 0);
        if *x == 0u32 {
            // sinh(0) = 0 and cosh(0) = 1, exactly
            return (Self::ZERO, Self::one_prec(prec), Equal, Equal);
        }
        sinh_cosh_rational_helper(x, prec, rm)
    }

    /// Computes $\sinh x$ and $\cosh x$, the hyperbolic sine and cosine of a [`Rational`],
    /// together, rounding both results to the nearest value of the specified precision, and
    /// returning the results as [`Float`]s. The [`Rational`] is taken by value. Two [`Ordering`]s
    /// are also returned, indicating whether the rounded hyperbolic sine and cosine are less than,
    /// equal to, or greater than the exact values.
    ///
    /// If a result is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// See [`Float::sinh_cosh_rational_prec_round`] for the error bounds, the special cases,
    /// overflow and underflow, and the complexity; this function behaves the same way with
    /// `Nearest`.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::sinh_cosh_rational_prec_round`] instead.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (s, c, o_s, o_c) = Float::sinh_cosh_rational_prec(Rational::from_unsigneds(3u8, 5), 20);
    /// assert_eq!(s.to_string(), "0.63665390");
    /// assert_eq!(c.to_string(), "1.1854649");
    /// assert_eq!(o_s, Greater);
    /// assert_eq!(o_c, Less);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn sinh_cosh_rational_prec(x: Rational, prec: u64) -> (Self, Self, Ordering, Ordering) {
        Self::sinh_cosh_rational_prec_round_ref(&x, prec, Nearest)
    }

    /// Computes $\sinh x$ and $\cosh x$, the hyperbolic sine and cosine of a [`Rational`],
    /// together, rounding both results to the nearest value of the specified precision, and
    /// returning the results as [`Float`]s. The [`Rational`] is taken by reference. Two
    /// [`Ordering`]s are also returned, indicating whether the rounded hyperbolic sine and cosine
    /// are less than, equal to, or greater than the exact values.
    ///
    /// If a result is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// See [`Float::sinh_cosh_rational_prec_round`] for the error bounds, the special cases,
    /// overflow and underflow, and the complexity; this function behaves the same way with
    /// `Nearest`.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::sinh_cosh_rational_prec_round_ref`] instead.
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
    /// let (s, c, o_s, o_c) =
    ///     Float::sinh_cosh_rational_prec_ref(&Rational::from_unsigneds(3u8, 5), 20);
    /// assert_eq!(s.to_string(), "0.63665390");
    /// assert_eq!(c.to_string(), "1.1854649");
    /// assert_eq!(o_s, Greater);
    /// assert_eq!(o_c, Less);
    ///
    /// let (s, c, o_s, o_c) = Float::sinh_cosh_rational_prec_ref(&Rational::ZERO, 10);
    /// assert_eq!(s.to_string(), "0.0");
    /// assert_eq!(c.to_string(), "1.0000");
    /// assert_eq!(o_s, Equal);
    /// assert_eq!(o_c, Equal);
    /// ```
    #[inline]
    pub fn sinh_cosh_rational_prec_ref(
        x: &Rational,
        prec: u64,
    ) -> (Self, Self, Ordering, Ordering) {
        Self::sinh_cosh_rational_prec_round_ref(x, prec, Nearest)
    }
}

impl SinhCosh for Float {
    type Output = Self;

    /// Computes $\sinh x$ and $\cosh x$, the hyperbolic sine and cosine of a [`Float`], together,
    /// taking it by value.
    ///
    /// If the outputs have a precision, it is the precision of the input. If a result is
    /// equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s in
    /// its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// $$
    /// f(x) = (\sinh x+\varepsilon_s, \cosh x+\varepsilon_c).
    /// $$
    /// - If a result is infinite, zero, or `NaN`, its $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If the results are finite and nonzero, then $|\varepsilon_s| < 2^{\lfloor\log_2 |\sinh
    ///   x|\rfloor-p}$ and $|\varepsilon_c| < 2^{\lfloor\log_2 \cosh x\rfloor-p}$, where $p$ is the
    ///   precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN})=(\text{NaN},\text{NaN})$
    /// - $f(\infty)=(\infty,\infty)$
    /// - $f(-\infty)=(-\infty,\infty)$
    /// - $f(\pm0.0)=(\pm0.0,1.0)$
    ///
    /// See [`Float::sinh_cosh_prec_round`] for overflow and the complexity.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::sinh_cosh_round`] instead. If you want to specify an output precision, consider
    /// using [`Float::sinh_cosh_prec`] instead. If you want both of these things, consider using
    /// [`Float::sinh_cosh_prec_round`] instead.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::SinhCosh;
    /// use malachite_base::num::basic::traits::{NaN, NegativeInfinity, NegativeZero};
    /// use malachite_float::Float;
    ///
    /// let (s, c) = Float::NAN.sinh_cosh();
    /// assert!(s.is_nan());
    /// assert!(c.is_nan());
    ///
    /// let (s, c) = Float::NEGATIVE_INFINITY.sinh_cosh();
    /// assert_eq!(s.to_string(), "-Infinity");
    /// assert_eq!(c.to_string(), "Infinity");
    ///
    /// let (s, c) = Float::NEGATIVE_ZERO.sinh_cosh();
    /// assert_eq!(s.to_string(), "-0.0");
    /// assert_eq!(c.to_string(), "1.0");
    ///
    /// let (s, c) = Float::from_unsigned_prec(1u32, 100).0.sinh_cosh();
    /// assert_eq!(s.to_string(), "1.1752011936438014568823818505953");
    /// assert_eq!(c.to_string(), "1.5430806348152437784779056207575");
    /// ```
    #[inline]
    fn sinh_cosh(self) -> (Self, Self) {
        let prec = self.significant_bits();
        let (s, c, _, _) = self.sinh_cosh_prec_round_ref(prec, Nearest);
        (s, c)
    }
}

impl SinhCosh for &Float {
    type Output = Float;

    /// Computes $\sinh x$ and $\cosh x$, the hyperbolic sine and cosine of a [`Float`], together,
    /// taking it by reference.
    ///
    /// If the outputs have a precision, it is the precision of the input. If a result is
    /// equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s in
    /// its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// $$
    /// f(x) = (\sinh x+\varepsilon_s, \cosh x+\varepsilon_c).
    /// $$
    /// - If a result is infinite, zero, or `NaN`, its $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If the results are finite and nonzero, then $|\varepsilon_s| < 2^{\lfloor\log_2 |\sinh
    ///   x|\rfloor-p}$ and $|\varepsilon_c| < 2^{\lfloor\log_2 \cosh x\rfloor-p}$, where $p$ is the
    ///   precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN})=(\text{NaN},\text{NaN})$
    /// - $f(\infty)=(\infty,\infty)$
    /// - $f(-\infty)=(-\infty,\infty)$
    /// - $f(\pm0.0)=(\pm0.0,1.0)$
    ///
    /// See [`Float::sinh_cosh_prec_round`] for overflow and the complexity.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::sinh_cosh_round_ref`] instead. If you want to specify an output precision, consider
    /// using [`Float::sinh_cosh_prec_ref`] instead. If you want both of these things, consider
    /// using [`Float::sinh_cosh_prec_round_ref`] instead.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::SinhCosh;
    /// use malachite_base::num::basic::traits::{NaN, NegativeInfinity, NegativeZero};
    /// use malachite_float::Float;
    ///
    /// let (s, c) = (&Float::NAN).sinh_cosh();
    /// assert!(s.is_nan());
    /// assert!(c.is_nan());
    ///
    /// let (s, c) = (&Float::NEGATIVE_INFINITY).sinh_cosh();
    /// assert_eq!(s.to_string(), "-Infinity");
    /// assert_eq!(c.to_string(), "Infinity");
    ///
    /// let (s, c) = (&Float::NEGATIVE_ZERO).sinh_cosh();
    /// assert_eq!(s.to_string(), "-0.0");
    /// assert_eq!(c.to_string(), "1.0");
    ///
    /// let (s, c) = (&Float::from_unsigned_prec(1u32, 100).0).sinh_cosh();
    /// assert_eq!(s.to_string(), "1.1752011936438014568823818505953");
    /// assert_eq!(c.to_string(), "1.5430806348152437784779056207575");
    /// ```
    #[inline]
    fn sinh_cosh(self) -> (Float, Float) {
        let (s, c, _, _) = self.sinh_cosh_prec_round_ref(self.significant_bits(), Nearest);
        (s, c)
    }
}

impl SinhCoshAssign for Float {
    /// Replaces a [`Float`] with its hyperbolic sine and writes its hyperbolic cosine to `cosh`,
    /// rounding both results to the nearest value of the input's precision. The previous value of
    /// `cosh` is discarded.
    ///
    /// See [`Float::sinh_cosh`]; this function behaves the same way.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::SinhCoshAssign;
    /// use malachite_base::num::basic::traits::NaN;
    /// use malachite_float::Float;
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// let mut c = Float::NAN;
    /// x.sinh_cosh_assign(&mut c);
    /// assert_eq!(x.to_string(), "1.1752011936438014568823818505953");
    /// assert_eq!(c.to_string(), "1.5430806348152437784779056207575");
    /// ```
    #[inline]
    fn sinh_cosh_assign(&mut self, cosh: &mut Self) {
        let prec = self.significant_bits();
        self.sinh_cosh_prec_round_assign(cosh, prec, Nearest);
    }
}

/// Computes $\sinh x$ and $\cosh x$, the hyperbolic sine and cosine of a primitive float, together.
/// The results are correctly rounded.
///
/// The results are those of
/// [`primitive_float_sinh`](crate::float::arithmetic::sinh::primitive_float_sinh) and
/// [`primitive_float_cosh`](crate::float::arithmetic::cosh::primitive_float_cosh), but they share a
/// single exponential, so this is faster than the two calls when both values are needed.
///
/// $$
/// f(x) = (\sinh x+\varepsilon_s, \cosh x+\varepsilon_c).
/// $$
/// - If a result is infinite, zero, or `NaN`, its $\varepsilon$ may be ignored or assumed to be 0.
/// - If the results are finite and nonzero, then $|\varepsilon_s| < 2^{\lfloor\log_2 |\sinh
///   x|\rfloor-p}$ and $|\varepsilon_c| < 2^{\lfloor\log_2 \cosh x\rfloor-p}$, where $p$ is the
///   precision of the output (typically 24 if `T` is a [`f32`] and 53 if `T` is a [`f64`], but less
///   for a subnormal hyperbolic sine).
///
/// Special cases:
/// - $f(\text{NaN})=(\text{NaN},\text{NaN})$
/// - $f(\infty)=(\infty,\infty)$
/// - $f(-\infty)=(-\infty,\infty)$
/// - $f(\pm0.0)=(\pm0.0,1.0)$
///
/// Overflow is possible: an `x` of large magnitude gives infinite results. Neither result
/// underflows. The hyperbolic sine is subnormal only when $x$ is, and then it is $x$ itself.
///
/// # Worst-case complexity
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use malachite_base::num::float::NiceFloat;
/// use malachite_float::float::arithmetic::sinh_cosh::primitive_float_sinh_cosh;
///
/// let (s, c) = primitive_float_sinh_cosh(f32::NAN);
/// assert!(s.is_nan());
/// assert!(c.is_nan());
///
/// let (s, c) = primitive_float_sinh_cosh(-0.0f32);
/// assert_eq!(NiceFloat(s), NiceFloat(-0.0));
/// assert_eq!(NiceFloat(c), NiceFloat(1.0));
///
/// let (s, c) = primitive_float_sinh_cosh(1.0f32);
/// assert_eq!(NiceFloat(s), NiceFloat(1.1752012));
/// assert_eq!(NiceFloat(c), NiceFloat(1.5430807));
///
/// let (s, c) = primitive_float_sinh_cosh(-1.0f64);
/// assert_eq!(NiceFloat(s), NiceFloat(-1.1752011936438014));
/// assert_eq!(NiceFloat(c), NiceFloat(1.5430806348152437));
///
/// let (s, c) = primitive_float_sinh_cosh(1000.0f64);
/// assert_eq!(NiceFloat(s), NiceFloat(f64::INFINITY));
/// assert_eq!(NiceFloat(c), NiceFloat(f64::INFINITY));
/// ```
#[inline]
#[allow(clippy::type_repetition_in_bounds)]
pub fn primitive_float_sinh_cosh<T: PrimitiveFloat>(x: T) -> (T, T)
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    emulate_float_to_float_pair_fn(Float::sinh_cosh_prec, x)
}

/// Computes $\sinh x$ and $\cosh x$, the hyperbolic sine and cosine of a [`Rational`], together,
/// returning the results as primitive floats. The results are correctly rounded.
///
/// The results are those of
/// [`primitive_float_sinh_rational`](crate::float::arithmetic::sinh::primitive_float_sinh_rational)
/// and
/// [`primitive_float_cosh_rational`](
/// crate::float::arithmetic::cosh::primitive_float_cosh_rational),
/// but they share their exponentials, so this is faster than the two calls when both values are
/// needed.
///
/// $$
/// f(x) = (\sinh x+\varepsilon_s, \cosh x+\varepsilon_c).
/// $$
/// - If a result is infinite or zero, its $\varepsilon$ may be ignored or assumed to be 0.
/// - If the results are finite and nonzero, then $|\varepsilon_s| < 2^{\lfloor\log_2 |\sinh
///   x|\rfloor-p}$ and $|\varepsilon_c| < 2^{\lfloor\log_2 \cosh x\rfloor-p}$, where $p$ is the
///   precision of the output (typically 24 if `T` is a [`f32`] and 53 if `T` is a [`f64`], but less
///   for a subnormal hyperbolic sine).
///
/// Special cases:
/// - $f(0)=(0.0,1.0)$
///
/// Overflow is possible: an `x` of large magnitude gives infinite results. The hyperbolic sine of
/// an `x` of small enough magnitude underflows to `0.0` or `-0.0`; the hyperbolic cosine never
/// underflows.
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
/// use malachite_base::num::basic::traits::NegativeInfinity;
/// use malachite_base::num::float::NiceFloat;
/// use malachite_float::float::arithmetic::sinh_cosh::primitive_float_sinh_cosh_rational;
/// use malachite_q::Rational;
///
/// let (s, c) = primitive_float_sinh_cosh_rational::<f64>(&Rational::from_unsigneds(1u8, 3));
/// assert_eq!(NiceFloat(s), NiceFloat(0.3395405572561501));
/// assert_eq!(NiceFloat(c), NiceFloat(1.0560718678299394));
///
/// let (s, c) = primitive_float_sinh_cosh_rational::<f64>(&Rational::from(-10000));
/// assert_eq!(NiceFloat(s), NiceFloat(f64::NEGATIVE_INFINITY));
/// assert_eq!(NiceFloat(c), NiceFloat(f64::INFINITY));
/// ```
#[inline]
#[allow(clippy::type_repetition_in_bounds)]
pub fn primitive_float_sinh_cosh_rational<T: PrimitiveFloat>(x: &Rational) -> (T, T)
where
    Float: PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    emulate_rational_to_float_pair_fn(Float::sinh_cosh_rational_prec_ref, x)
}
