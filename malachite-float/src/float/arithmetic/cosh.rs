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
use crate::float::arithmetic::exp::exp_overflow;
use crate::float::arithmetic::round_near_x::small_input_shortcut;
use crate::{Float, emulate_float_to_float_fn};
use core::cmp::Ordering::{self, Equal};
use malachite_base::num::arithmetic::traits::{Abs, CeilingLogBase2, Cosh, CoshAssign};
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::{Infinity as InfinityTrait, NaN as NaNTrait, One};
use malachite_base::num::conversion::traits::{ExactFrom, RoundingFrom};
use malachite_base::num::logic::traits::{CountOnes, SignificantBits};
use malachite_base::rounding_modes::RoundingMode::{self, Ceiling, Exact, Floor, Nearest};
use malachite_nz::natural::arithmetic::float::round::float_can_round;
use malachite_nz::platform::Limb;

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
// extended exponent range in which that implies that cosh(x) > exp(x) / 2 overflows the ordinary
// range too. Malachite has no extended range, so exp(x) can overflow while cosh(x) ~ exp(x) / 2 is
// still finite (a window of width log(2) in x). Instead, write exp(x) / 2 = u * (u / 2) with u =
// exp(x / 2), which does not overflow when cosh(x) doesn't.
//
// Returns `None` if cosh(x) overflows; otherwise h with |h - exp(x) / 2| < 8 ulp(h): u has an error
// below 1 ulp, so u * (u / 2), rounded once more, has a relative error below 2^(3 - working_prec).
fn half_exp_near_overflow(x: &Float, working_prec: u64) -> Option<Float> {
    // x is large, so halving it is exact.
    let u = (x >> 1u32).exp_prec_round(working_prec, Floor).0;
    if u.get_exponent() == Some(Float::MAX_EXPONENT) {
        // exp(x / 2) >= 2^(MAX_EXPONENT - 1), so cosh(x) > exp(x) / 2 overflows.
        return None;
    }
    let h = (&u >> 1u32).mul_prec_round(u, working_prec, Floor).0; // <= exp(x) / 2
    if is_max_finite(&h) {
        // cosh(x) > exp(x) / 2 >= the largest finite Float at precision `working_prec`, which
        // exceeds the midpoint between the largest finite Float at any lower precision and
        // 2^MAX_EXPONENT, so cosh(x) overflows (or, with Floor or Down, saturates) at the output
        // precision.
        return None;
    }
    Some(h)
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
        // cosh(x) = h + 1 / (4 h), where h = exp(x) / 2.
        let exp_x = x.exp_prec_round_ref(working_prec, Floor).0;
        let (h, err) = if exp_x.get_exponent() == Some(Float::MAX_EXPONENT) {
            // exp(x) is in the top binade, or overflowed and saturated.
            match half_exp_near_overflow(&x, working_prec) {
                None => return exp_overflow(prec, rm),
                // h has an error below 8 ulps, and the two roundings below add at most 2 more.
                Some(h) => (h, working_prec - 4),
            }
        } else {
            // Halving is exact. The error of exp(x) and the two roundings below stay below 8 ulps.
            (exp_x >> 1u32, working_prec - 3)
        };
        // exp(-x) / 2 = 1 / (4 h), rounded up. This may underflow, in which case it rounds up to
        // the smallest positive Float, still an upper bound.
        let exp_neg_x_half = h
            .reciprocal_round_ref(Ceiling)
            .0
            .shr_prec_round(2u32, working_prec, Ceiling)
            .0;
        // h is not the largest finite Float, so adding a value this small rounds up to at most it.
        let cosh_x = h.add_round(exp_neg_x_half, Ceiling).0;
        if float_can_round(cosh_x.significand_ref().unwrap(), err, prec, rm) {
            return Float::from_float_prec_round(cosh_x, prec, rm);
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
