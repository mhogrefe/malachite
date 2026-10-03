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
use crate::float::arithmetic::cosh::{half_exp, quarter_reciprocal};
use crate::float::arithmetic::round_near_x::small_input_shortcut;
use crate::float::conversion::string::set_str::overflow;
use crate::{Float, emulate_float_to_float_fn};
use core::cmp::Ordering::{self, Equal};
use core::cmp::max;
use malachite_base::num::arithmetic::traits::{Abs, CeilingLogBase2, Sinh, SinhAssign};
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::NaN as NaNTrait;
use malachite_base::num::conversion::traits::{ExactFrom, RoundingFrom};
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{self, Exact, Nearest};
use malachite_nz::natural::arithmetic::float::round::float_can_round;
use malachite_nz::platform::Limb;

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
        // sinh(|x|) = h - 1 / (4 h), where h = exp(|x|) / 2.
        let Some((h, near_overflow)) = half_exp(&x_abs, working_prec) else {
            return overflow(positive, prec, rm);
        };
        let sinh_abs = h
            .sub_prec_ref_val(quarter_reciprocal(&h, working_prec), working_prec)
            .0;
        // The difference is not zero: that would need exp(|x|) to round down to exactly 1, so |x| <
        // 2^(1 - working_prec), but working_prec exceeds -2 EXP(x).
        debug_assert_ne!(sinh_abs, 0u32);
        // The subtraction cancels about EXP(h) - EXP(sinh_abs) bits of h's error, which is below 1
        // ulp of h, or 8 ulps near the overflow threshold (cf. sinh.c, whose estimate is err = Nt -
        // ceil(log_2(1 + 2^d)) with d = EXP(exp(x)) - EXP(sinh(x)) + 2).
        let d =
            i64::from(h.get_exponent().unwrap()) - i64::from(sinh_abs.get_exponent().unwrap()) + 3;
        let loss = u64::exact_from(max(d, 0)) + if near_overflow { 4 } else { 1 };
        if loss < working_prec
            && float_can_round(
                sinh_abs.significand_ref().unwrap(),
                working_prec - loss,
                prec,
                rm,
            )
        {
            break sinh_abs;
        }
        working_prec += increment;
        increment = working_prec >> 1;
    };
    Float::from_float_prec_round(if positive { sinh_abs } else { -sinh_abs }, prec, rm)
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
