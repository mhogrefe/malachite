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
use crate::float::arithmetic::cosh::{hyperbolic_approx, hyperbolic_can_round};
use crate::float::arithmetic::round_near_x::small_input_shortcut;
use crate::float::conversion::string::set_str::overflow;
use crate::{Float, emulate_float_to_float_pair_fn};
use core::cmp::Ordering::{self, Equal};
use malachite_base::num::arithmetic::traits::{Abs, CeilingLogBase2, SinhCosh, SinhCoshAssign};
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::{Infinity as InfinityTrait, NaN as NaNTrait, One};
use malachite_base::num::conversion::traits::{ExactFrom, RoundingFrom};
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{self, Exact, Nearest};
use malachite_nz::platform::Limb;

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
