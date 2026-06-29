// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the GNU MPFR Library.
//
//      Copyright 2001-2025 Free Software Foundation, Inc.
//
//      Contributed by the Pascaline and Caramba projects, INRIA.
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::InnerFloat::{Finite, Infinity, NaN, Zero};
use crate::arithmetic::exp::{exp_overflow, exp_underflow};
use crate::{Float, emulate_float_to_float_fn};
use core::cmp::Ordering::{self, *};
use malachite_base::num::arithmetic::traits::{
    CeilingLogBase2, IsPowerOf2, PowerOf2, PowerOf2Assign,
};
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::{
    Infinity as InfinityTrait, NaN as NaNTrait, Zero as ZeroTrait,
};
use malachite_base::num::conversion::traits::{ExactFrom, IsInteger, RoundingFrom};
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_nz::integer::Integer;
use malachite_nz::natural::arithmetic::float_extras::float_can_round;
use malachite_nz::platform::{Limb, SignedLimb};

fn power_of_2_of_float_prec_round_normal_helper(
    xfrac: &Float,
    xint: i64,
    precy: u64,
    rm: RoundingMode,
) -> (Float, Ordering) {
    let mut working_prec = precy + 5 + precy.ceiling_log_base_2();
    let mut increment = Limb::WIDTH;
    loop {
        let ln_2 = Float::ln_2_prec_round(working_prec, Up).0;
        let mut t = xfrac.mul_prec_round_ref_val(ln_2, working_prec, Up).0; // xfrac * ln(2)
        // Error estimate (cf. mpfr_exp2): the relative error of t (computed with two roundings) is
        // bounded so that exp(t) is correct to `err` bits.
        let err = u64::exact_from(
            i64::exact_from(working_prec) - (i64::from(t.get_exponent().unwrap()) + 2),
        );
        t.exp_prec_round_assign(working_prec, Nearest); // exp(xfrac * ln(2))
        if float_can_round(t.significand_ref().unwrap(), err, precy, rm) {
            let (y, inexact) = Float::from_float_prec_round(t, precy, rm);
            // Multiply by 2^xint. Special case (mpfr_exp2): if `Nearest` rounded 2^xfrac down to
            // 1/2 and xint = MIN_EXPONENT - 1, the unrounded result is the midpoint between 0 and
            // the smallest positive Float, which is a double-rounding problem: round up to that
            // smallest value instead of underflowing.
            if rm == Nearest
                && xint == const { (Float::MIN_EXPONENT as i64) - 1 }
                && y.get_exponent() == Some(0)
                && (&y).is_power_of_2()
            {
                return (Float::min_positive_value_prec(precy), Greater);
            }
            return (y << xint, inexact);
        }
        working_prec += increment;
        increment = working_prec >> 1;
    }
}

// This is mpfr_exp2 from exp2.c, MPFR 4.2.2, where the input is finite and nonzero and the float is
// taken by reference.
fn power_of_2_of_float_prec_round_normal(
    x: &Float,
    precy: u64,
    rm: RoundingMode,
) -> (Float, Ordering) {
    // 2^x overflows once x >= MAX_EXPONENT, and underflows once x <= MIN_EXPONENT - 2 (the smallest
    // representable positive value is 2^(MIN_EXPONENT - 1)).
    if *x >= const { Float::const_from_signed(Float::MAX_EXPONENT as SignedLimb) } {
        return exp_overflow(precy, rm);
    }
    if *x <= const { Float::const_from_signed((Float::MIN_EXPONENT as SignedLimb) - 2) } {
        return exp_underflow(precy, rm);
    }
    // We now know that MIN_EXPONENT - 2 < x < MAX_EXPONENT, so the integer part fits in an i64.
    let xint = i64::exact_from(&Integer::rounding_from(x, Down).0); // trunc(x), toward zero
    // If x is an integer, 2^x is a power of 2, hence exact.
    if x.is_integer() {
        return Float::power_of_2_prec_round(xint, precy, rm);
    }
    // 2^x for a non-integer Float is transcendental, hence never exactly representable.
    assert_ne!(rm, Exact, "Inexact power_of_2_of_float");
    // 2^x = 2^xint * 2^xfrac, where xfrac = x - xint and |xfrac| < 1. We compute 2^xfrac =
    // exp(xfrac * ln(2)) and then multiply by 2^xint by shifting the result's exponent.
    let p = x.get_prec().unwrap();
    if xint == 0 {
        power_of_2_of_float_prec_round_normal_helper(x, 0, precy, rm)
    } else {
        // x - xint is exact: the difference has fewer significant bits than x.
        let xint_f = Float::from_integer_prec(Integer::from(xint), p).0;
        let xfrac = x.sub_prec_round_ref_val(xint_f, p, Floor).0;
        power_of_2_of_float_prec_round_normal_helper(&xfrac, xint, precy, rm)
    }
}

impl Float {
    /// Computes $2^x$, where $x$ is a [`Float`], rounding the result to the specified precision and
    /// with the specified rounding mode. The [`Float`] is taken by value. An [`Ordering`] is also
    /// returned, indicating whether the rounded power is less than, equal to, or greater than the
    /// exact power. Although `NaN`s are not comparable to any [`Float`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = 2^x+\varepsilon.
    /// $$
    /// - If $2^x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $2^x$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 2^x\rfloor-p+1}$.
    /// - If $2^x$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 2^x\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=\text{NaN}$
    /// - $f(\infty,p,m)=\infty$
    /// - $f(-\infty,p,m)=0.0$
    /// - $f(\pm0.0,p,m)=1.0$
    ///
    /// Overflow and underflow:
    /// - If $f(x,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$ is
    ///   returned instead.
    /// - If $f(x,p,m)<2^{-2^{30}}$ and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $f(x,p,m)<2^{-2^{30}}$ and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned instead.
    /// - If $f(x,p,m)\leq2^{-2^{30}-1}$ and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,p,m)<2^{-2^{30}}$ and $m$ is `Nearest`, $2^{-2^{30}}$ is returned
    ///   instead.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::power_of_2_of_float_prec`]
    /// instead. If you know that your target precision is the precision of the input, consider
    /// using [`Float::power_of_2_of_float_round`] instead. If both of these things are true,
    /// consider using the [`PowerOf2`] implementation instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with the given precision.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (p, o) = Float::power_of_2_of_float_prec_round(Float::from(1.5), 5, Floor);
    /// assert_eq!(p.to_string(), "2.8");
    /// assert_eq!(o, Less);
    ///
    /// let (p, o) = Float::power_of_2_of_float_prec_round(Float::from(1.5), 5, Ceiling);
    /// assert_eq!(p.to_string(), "2.9");
    /// assert_eq!(o, Greater);
    ///
    /// let (p, o) = Float::power_of_2_of_float_prec_round(Float::from(1.5), 5, Nearest);
    /// assert_eq!(p.to_string(), "2.9");
    /// assert_eq!(o, Greater);
    ///
    /// let (p, o) = Float::power_of_2_of_float_prec_round(Float::from(1.5), 20, Floor);
    /// assert_eq!(p.to_string(), "2.828426");
    /// assert_eq!(o, Less);
    ///
    /// let (p, o) = Float::power_of_2_of_float_prec_round(Float::from(1.5), 20, Ceiling);
    /// assert_eq!(p.to_string(), "2.82843");
    /// assert_eq!(o, Greater);
    ///
    /// let (p, o) = Float::power_of_2_of_float_prec_round(Float::from(1.5), 20, Nearest);
    /// assert_eq!(p.to_string(), "2.828426");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn power_of_2_of_float_prec_round(
        pow: Float,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        Self::power_of_2_of_float_prec_round_ref(&pow, prec, rm)
    }

    /// Computes $2^x$, where $x$ is a [`Float`], rounding the result to the specified precision and
    /// with the specified rounding mode. The [`Float`] is taken by reference. An [`Ordering`] is
    /// also returned, indicating whether the rounded power is less than, equal to, or greater than
    /// the exact power. Although `NaN`s are not comparable to any [`Float`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = 2^x+\varepsilon.
    /// $$
    /// - If $2^x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $2^x$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 2^x\rfloor-p+1}$.
    /// - If $2^x$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 2^x\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=\text{NaN}$
    /// - $f(\infty,p,m)=\infty$
    /// - $f(-\infty,p,m)=0.0$
    /// - $f(\pm0.0,p,m)=1.0$
    ///
    /// Overflow and underflow:
    /// - If $f(x,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$ is
    ///   returned instead.
    /// - If $f(x,p,m)<2^{-2^{30}}$ and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $f(x,p,m)<2^{-2^{30}}$ and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned instead.
    /// - If $f(x,p,m)\leq2^{-2^{30}-1}$ and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,p,m)<2^{-2^{30}}$ and $m$ is `Nearest`, $2^{-2^{30}}$ is returned
    ///   instead.
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`Float::power_of_2_of_float_prec_ref`] instead. If you know that your target precision is
    /// the precision of the input, consider using [`Float::power_of_2_of_float_round_ref`] instead.
    /// If both of these things are true, consider using the [`PowerOf2`] implementation instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with the given precision.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(1.5);
    ///
    /// let (p, o) = Float::power_of_2_of_float_prec_round_ref(&x, 5, Floor);
    /// assert_eq!(p.to_string(), "2.8");
    /// assert_eq!(o, Less);
    ///
    /// let (p, o) = Float::power_of_2_of_float_prec_round_ref(&x, 5, Ceiling);
    /// assert_eq!(p.to_string(), "2.9");
    /// assert_eq!(o, Greater);
    ///
    /// let (p, o) = Float::power_of_2_of_float_prec_round_ref(&x, 5, Nearest);
    /// assert_eq!(p.to_string(), "2.9");
    /// assert_eq!(o, Greater);
    ///
    /// let (p, o) = Float::power_of_2_of_float_prec_round_ref(&x, 20, Floor);
    /// assert_eq!(p.to_string(), "2.828426");
    /// assert_eq!(o, Less);
    ///
    /// let (p, o) = Float::power_of_2_of_float_prec_round_ref(&x, 20, Ceiling);
    /// assert_eq!(p.to_string(), "2.82843");
    /// assert_eq!(o, Greater);
    ///
    /// let (p, o) = Float::power_of_2_of_float_prec_round_ref(&x, 20, Nearest);
    /// assert_eq!(p.to_string(), "2.828426");
    /// assert_eq!(o, Less);
    /// ```
    pub fn power_of_2_of_float_prec_round_ref(
        pow: &Float,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        assert_ne!(prec, 0);
        match &pow.0 {
            NaN => (Self::NAN, Equal),
            // 2^(+inf) = +inf; 2^(-inf) = +0
            Infinity { sign } => {
                if *sign {
                    (Self::INFINITY, Equal)
                } else {
                    (Self::ZERO, Equal)
                }
            }
            // 2^(+0) = 2^(-0) = 1
            Zero { .. } => (Self::one_prec(prec), Equal),
            Finite { .. } => power_of_2_of_float_prec_round_normal(pow, prec, rm),
        }
    }

    /// Computes $2^x$, where $x$ is a [`Float`], rounding the result to the nearest value of the
    /// specified precision. The [`Float`] is taken by value. An [`Ordering`] is also returned,
    /// indicating whether the rounded power is less than, equal to, or greater than the exact
    /// power. Although `NaN`s are not comparable to any [`Float`], whenever this function returns a
    /// `NaN` it also returns `Equal`.
    ///
    /// If the power is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = 2^x+\varepsilon.
    /// $$
    /// - If $2^x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $2^x$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2 2^x\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p)=\text{NaN}$
    /// - $f(\infty,p)=\infty$
    /// - $f(-\infty,p)=0.0$
    /// - $f(\pm0.0,p)=1.0$
    ///
    /// Overflow and underflow:
    /// - If $f(x,p)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::power_of_2_of_float_prec_round`] instead. If you know that your target precision is
    /// the precision of the input, consider using the [`PowerOf2`] implementation instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (p, o) = Float::power_of_2_of_float_prec(Float::from(1.5), 5);
    /// assert_eq!(p.to_string(), "2.9");
    /// assert_eq!(o, Greater);
    ///
    /// let (p, o) = Float::power_of_2_of_float_prec(Float::from(1.5), 20);
    /// assert_eq!(p.to_string(), "2.828426");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn power_of_2_of_float_prec(pow: Float, prec: u64) -> (Self, Ordering) {
        Self::power_of_2_of_float_prec_round_ref(&pow, prec, Nearest)
    }

    /// Computes $2^x$, where $x$ is a [`Float`], rounding the result to the nearest value of the
    /// specified precision. The [`Float`] is taken by reference. An [`Ordering`] is also returned,
    /// indicating whether the rounded power is less than, equal to, or greater than the exact
    /// power. Although `NaN`s are not comparable to any [`Float`], whenever this function returns a
    /// `NaN` it also returns `Equal`.
    ///
    /// If the power is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = 2^x+\varepsilon.
    /// $$
    /// - If $2^x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $2^x$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2 2^x\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p)=\text{NaN}$
    /// - $f(\infty,p)=\infty$
    /// - $f(-\infty,p)=0.0$
    /// - $f(\pm0.0,p)=1.0$
    ///
    /// Overflow and underflow:
    /// - If $f(x,p)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::power_of_2_of_float_prec_round_ref`] instead. If you know that your target
    /// precision is the precision of the input, consider using the [`PowerOf2`] implementation
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(1.5);
    ///
    /// let (p, o) = Float::power_of_2_of_float_prec_ref(&x, 5);
    /// assert_eq!(p.to_string(), "2.9");
    /// assert_eq!(o, Greater);
    ///
    /// let (p, o) = Float::power_of_2_of_float_prec_ref(&x, 20);
    /// assert_eq!(p.to_string(), "2.828426");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn power_of_2_of_float_prec_ref(pow: &Float, prec: u64) -> (Self, Ordering) {
        Self::power_of_2_of_float_prec_round_ref(pow, prec, Nearest)
    }

    /// Computes $2^x$, where $x$ is a [`Float`], rounding the result with the specified rounding
    /// mode. The [`Float`] is taken by value. An [`Ordering`] is also returned, indicating whether
    /// the rounded power is less than, equal to, or greater than the exact power. Although `NaN`s
    /// are not comparable to any [`Float`], whenever this function returns a `NaN` it also returns
    /// `Equal`.
    ///
    /// The precision of the output is the precision of the input. See [`RoundingMode`] for a
    /// description of the possible rounding modes.
    ///
    /// $$
    /// f(x,m) = 2^x+\varepsilon.
    /// $$
    /// - If $2^x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $2^x$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 2^x\rfloor-p+1}$, where $p$ is the precision of the input.
    /// - If $2^x$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 2^x\rfloor-p}$, where $p$ is the precision of the input.
    ///
    /// If the output has a precision, it is the precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN},m)=\text{NaN}$
    /// - $f(\infty,m)=\infty$
    /// - $f(-\infty,m)=0.0$
    /// - $f(\pm0.0,m)=1.0$
    ///
    /// See the [`Float::power_of_2_of_float_prec_round`] documentation for information on overflow
    /// and underflow.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::power_of_2_of_float_prec_round`] instead. If you know you'll be using the `Nearest`
    /// rounding mode, consider using the [`PowerOf2`] implementation instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the result cannot be represented exactly with the input
    /// precision.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (p, o) =
    ///     Float::power_of_2_of_float_round(Float::from_unsigned_prec(3u32, 100).0 >> 1u32, Floor);
    /// assert_eq!(p.to_string(), "2.828427124746190097603377448418");
    /// assert_eq!(o, Less);
    ///
    /// let (p, o) = Float::power_of_2_of_float_round(
    ///     Float::from_unsigned_prec(3u32, 100).0 >> 1u32,
    ///     Ceiling,
    /// );
    /// assert_eq!(p.to_string(), "2.828427124746190097603377448422");
    /// assert_eq!(o, Greater);
    ///
    /// let (p, o) = Float::power_of_2_of_float_round(
    ///     Float::from_unsigned_prec(3u32, 100).0 >> 1u32,
    ///     Nearest,
    /// );
    /// assert_eq!(p.to_string(), "2.828427124746190097603377448418");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn power_of_2_of_float_round(pow: Float, rm: RoundingMode) -> (Self, Ordering) {
        let prec = pow.significant_bits();
        Self::power_of_2_of_float_prec_round_ref(&pow, prec, rm)
    }

    /// Computes $2^x$, where $x$ is a [`Float`], rounding the result with the specified rounding
    /// mode. The [`Float`] is taken by reference. An [`Ordering`] is also returned, indicating
    /// whether the rounded power is less than, equal to, or greater than the exact power. Although
    /// `NaN`s are not comparable to any [`Float`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// The precision of the output is the precision of the input. See [`RoundingMode`] for a
    /// description of the possible rounding modes.
    ///
    /// $$
    /// f(x,m) = 2^x+\varepsilon.
    /// $$
    /// - If $2^x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $2^x$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 2^x\rfloor-p+1}$, where $p$ is the precision of the input.
    /// - If $2^x$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 2^x\rfloor-p}$, where $p$ is the precision of the input.
    ///
    /// If the output has a precision, it is the precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN},m)=\text{NaN}$
    /// - $f(\infty,m)=\infty$
    /// - $f(-\infty,m)=0.0$
    /// - $f(\pm0.0,m)=1.0$
    ///
    /// See the [`Float::power_of_2_of_float_prec_round`] documentation for information on overflow
    /// and underflow.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::power_of_2_of_float_prec_round_ref`] instead. If you know you'll be using the
    /// `Nearest` rounding mode, consider using the [`PowerOf2`] implementation instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the result cannot be represented exactly with the input
    /// precision.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from_unsigned_prec(3u32, 100).0 >> 1u32;
    ///
    /// let (p, o) = Float::power_of_2_of_float_round_ref(&x, Floor);
    /// assert_eq!(p.to_string(), "2.828427124746190097603377448418");
    /// assert_eq!(o, Less);
    ///
    /// let (p, o) = Float::power_of_2_of_float_round_ref(&x, Ceiling);
    /// assert_eq!(p.to_string(), "2.828427124746190097603377448422");
    /// assert_eq!(o, Greater);
    ///
    /// let (p, o) = Float::power_of_2_of_float_round_ref(&x, Nearest);
    /// assert_eq!(p.to_string(), "2.828427124746190097603377448418");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn power_of_2_of_float_round_ref(pow: &Float, rm: RoundingMode) -> (Self, Ordering) {
        let prec = pow.significant_bits();
        Self::power_of_2_of_float_prec_round_ref(pow, prec, rm)
    }

    /// Computes $2^x$, where $x$ is a [`Float`], in place, rounding the result to the specified
    /// precision and with the specified rounding mode. An [`Ordering`] is returned, indicating
    /// whether the rounded power is less than, equal to, or greater than the exact power. Although
    /// `NaN`s are not comparable to any [`Float`], whenever this function sets the [`Float`] to
    /// `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets 2^x+\varepsilon.
    /// $$
    /// - If $2^x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $2^x$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 2^x\rfloor-p+1}$.
    /// - If $2^x$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 2^x\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// See the [`Float::power_of_2_of_float_prec_round`] documentation for information on special
    /// cases, overflow, and underflow.
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`Float::power_of_2_of_float_prec_assign`] instead. If you know that your target precision
    /// is the precision of the input, consider using [`Float::power_of_2_of_float_round_assign`]
    /// instead. If both of these things are true, consider using the [`PowerOf2Assign`]
    /// implementation instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with the given precision.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from(1.5);
    /// assert_eq!(x.power_of_2_of_float_prec_round_assign(5, Floor), Less);
    /// assert_eq!(x.to_string(), "2.8");
    ///
    /// let mut x = Float::from(1.5);
    /// assert_eq!(x.power_of_2_of_float_prec_round_assign(5, Ceiling), Greater);
    /// assert_eq!(x.to_string(), "2.9");
    ///
    /// let mut x = Float::from(1.5);
    /// assert_eq!(x.power_of_2_of_float_prec_round_assign(5, Nearest), Greater);
    /// assert_eq!(x.to_string(), "2.9");
    ///
    /// let mut x = Float::from(1.5);
    /// assert_eq!(x.power_of_2_of_float_prec_round_assign(20, Floor), Less);
    /// assert_eq!(x.to_string(), "2.828426");
    ///
    /// let mut x = Float::from(1.5);
    /// assert_eq!(
    ///     x.power_of_2_of_float_prec_round_assign(20, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "2.82843");
    ///
    /// let mut x = Float::from(1.5);
    /// assert_eq!(x.power_of_2_of_float_prec_round_assign(20, Nearest), Less);
    /// assert_eq!(x.to_string(), "2.828426");
    /// ```
    #[inline]
    pub fn power_of_2_of_float_prec_round_assign(
        &mut self,
        prec: u64,
        rm: RoundingMode,
    ) -> Ordering {
        let (result, o) = Self::power_of_2_of_float_prec_round_ref(self, prec, rm);
        *self = result;
        o
    }

    /// Computes $2^x$, where $x$ is a [`Float`], in place, rounding the result to the nearest value
    /// of the specified precision. An [`Ordering`] is returned, indicating whether the rounded
    /// power is less than, equal to, or greater than the exact power. Although `NaN`s are not
    /// comparable to any [`Float`], whenever this function sets the [`Float`] to `NaN` it also
    /// returns `Equal`.
    ///
    /// If the power is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// x \gets 2^x+\varepsilon.
    /// $$
    /// - If $2^x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $2^x$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2 2^x\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// See the [`Float::power_of_2_of_float_prec`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::power_of_2_of_float_prec_round_assign`] instead. If you know that your target
    /// precision is the precision of the input, consider using the [`PowerOf2Assign`]
    /// implementation instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from(1.5);
    /// assert_eq!(x.power_of_2_of_float_prec_assign(5), Greater);
    /// assert_eq!(x.to_string(), "2.9");
    ///
    /// let mut x = Float::from(1.5);
    /// assert_eq!(x.power_of_2_of_float_prec_assign(20), Less);
    /// assert_eq!(x.to_string(), "2.828426");
    /// ```
    #[inline]
    pub fn power_of_2_of_float_prec_assign(&mut self, prec: u64) -> Ordering {
        self.power_of_2_of_float_prec_round_assign(prec, Nearest)
    }

    /// Computes $2^x$, where $x$ is a [`Float`], in place, rounding the result with the specified
    /// rounding mode. An [`Ordering`] is returned, indicating whether the rounded power is less
    /// than, equal to, or greater than the exact power. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function sets the [`Float`] to `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the precision of the input. See [`RoundingMode`] for a
    /// description of the possible rounding modes.
    ///
    /// $$
    /// x \gets 2^x+\varepsilon.
    /// $$
    /// - If $2^x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $2^x$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 2^x\rfloor-p+1}$, where $p$ is the precision of the input.
    /// - If $2^x$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 2^x\rfloor-p}$, where $p$ is the precision of the input.
    ///
    /// If the output has a precision, it is the precision of the input.
    ///
    /// See the [`Float::power_of_2_of_float_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::power_of_2_of_float_prec_round_assign`] instead. If you know you'll be using the
    /// `Nearest` rounding mode, consider using the [`PowerOf2Assign`] implementation instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the result cannot be represented exactly with the input
    /// precision.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from_unsigned_prec(3u32, 100).0 >> 1u32;
    /// assert_eq!(x.power_of_2_of_float_round_assign(Floor), Less);
    /// assert_eq!(x.to_string(), "2.828427124746190097603377448418");
    ///
    /// let mut x = Float::from_unsigned_prec(3u32, 100).0 >> 1u32;
    /// assert_eq!(x.power_of_2_of_float_round_assign(Ceiling), Greater);
    /// assert_eq!(x.to_string(), "2.828427124746190097603377448422");
    ///
    /// let mut x = Float::from_unsigned_prec(3u32, 100).0 >> 1u32;
    /// assert_eq!(x.power_of_2_of_float_round_assign(Nearest), Less);
    /// assert_eq!(x.to_string(), "2.828427124746190097603377448418");
    /// ```
    #[inline]
    pub fn power_of_2_of_float_round_assign(&mut self, rm: RoundingMode) -> Ordering {
        let prec = self.significant_bits();
        self.power_of_2_of_float_prec_round_assign(prec, rm)
    }
}

impl PowerOf2<Float> for Float {
    /// Computes $2^x$, where $x$ is a [`Float`], taking it by value.
    ///
    /// If the output has a precision, it is the precision of the input. If the power is equidistant
    /// from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s in its binary
    /// expansion is chosen. See [`RoundingMode`] for a description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x) = 2^x+\varepsilon.
    /// $$
    /// - If $2^x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $2^x$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2 2^x\rfloor-p}$,
    ///   where $p$ is the precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN})=\text{NaN}$
    /// - $f(\infty)=\infty$
    /// - $f(-\infty)=0.0$
    /// - $f(\pm0.0)=1.0$
    ///
    /// See the [`Float::power_of_2_of_float_round`] documentation for information on overflow and
    /// underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::power_of_2_of_float_round`] instead. If you want to specify the output precision,
    /// consider using [`Float::power_of_2_of_float_prec`]. If you want both of these things,
    /// consider using [`Float::power_of_2_of_float_prec_round`].
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
    /// use malachite_base::num::arithmetic::traits::PowerOf2;
    /// use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity, Zero};
    /// use malachite_float::Float;
    ///
    /// assert!(Float::power_of_2(Float::NAN).is_nan());
    /// assert_eq!(Float::power_of_2(Float::INFINITY), Float::INFINITY);
    /// assert_eq!(Float::power_of_2(Float::NEGATIVE_INFINITY), Float::ZERO);
    /// assert_eq!(
    ///     Float::power_of_2(Float::from_unsigned_prec(3u32, 100).0 >> 1u32).to_string(),
    ///     "2.828427124746190097603377448418"
    /// );
    /// ```
    #[inline]
    fn power_of_2(pow: Float) -> Float {
        Float::power_of_2_of_float_round(pow, Nearest).0
    }
}

impl PowerOf2<&Float> for Float {
    /// Computes $2^x$, where $x$ is a [`Float`], taking it by reference.
    ///
    /// If the output has a precision, it is the precision of the input. If the power is equidistant
    /// from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s in its binary
    /// expansion is chosen. See [`RoundingMode`] for a description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x) = 2^x+\varepsilon.
    /// $$
    /// - If $2^x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $2^x$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2 2^x\rfloor-p}$,
    ///   where $p$ is the precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN})=\text{NaN}$
    /// - $f(\infty)=\infty$
    /// - $f(-\infty)=0.0$
    /// - $f(\pm0.0)=1.0$
    ///
    /// See the [`Float::power_of_2_of_float_round`] documentation for information on overflow and
    /// underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::power_of_2_of_float_round_ref`] instead. If you want to specify the output
    /// precision, consider using [`Float::power_of_2_of_float_prec_ref`]. If you want both of these
    /// things, consider using [`Float::power_of_2_of_float_prec_round_ref`].
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
    /// use malachite_base::num::arithmetic::traits::PowerOf2;
    /// use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity, Zero};
    /// use malachite_float::Float;
    ///
    /// assert!(Float::power_of_2(&Float::NAN).is_nan());
    /// assert_eq!(Float::power_of_2(&Float::INFINITY), Float::INFINITY);
    /// assert_eq!(Float::power_of_2(&Float::NEGATIVE_INFINITY), Float::ZERO);
    /// assert_eq!(
    ///     Float::power_of_2(&(Float::from_unsigned_prec(3u32, 100).0 >> 1u32)).to_string(),
    ///     "2.828427124746190097603377448418"
    /// );
    /// ```
    #[inline]
    fn power_of_2(pow: &Float) -> Float {
        Float::power_of_2_of_float_round_ref(pow, Nearest).0
    }
}

impl PowerOf2Assign for Float {
    /// Computes $2^x$, where $x$ is a [`Float`], in place.
    ///
    /// If the output has a precision, it is the precision of the input. If the power is equidistant
    /// from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s in its binary
    /// expansion is chosen. See [`RoundingMode`] for a description of the `Nearest` rounding mode.
    ///
    /// $$
    /// x \gets 2^x+\varepsilon.
    /// $$
    /// - If $2^x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $2^x$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2 2^x\rfloor-p}$,
    ///   where $p$ is the precision of the input.
    ///
    /// See the [`Float::power_of_2_of_float_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::power_of_2_of_float_round_assign`] instead. If you want to specify the output
    /// precision, consider using [`Float::power_of_2_of_float_prec_assign`]. If you want both of
    /// these things, consider using [`Float::power_of_2_of_float_prec_round_assign`].
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
    /// use malachite_base::num::arithmetic::traits::PowerOf2Assign;
    /// use malachite_float::Float;
    ///
    /// let mut x = Float::from_unsigned_prec(3u32, 100).0 >> 1u32;
    /// x.power_of_2_assign();
    /// assert_eq!(x.to_string(), "2.828427124746190097603377448418");
    /// ```
    #[inline]
    fn power_of_2_assign(&mut self) {
        self.power_of_2_of_float_round_assign(Nearest);
    }
}

/// Computes $2^x$, where $x$ is a primitive float, returning the result as a primitive float of the
/// same type. Using this function is more accurate than using `x.exp2()` or the `exp2` function
/// provided by `libm`.
///
/// $$
/// f(x) = 2^x+\varepsilon.
/// $$
/// - If $2^x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
/// - If $2^x$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2 2^x\rfloor-p}$, where
///   $p$ is the precision of the output (typically 24 if `T` is a [`f32`] and 53 if `T` is a
///   [`f64`], but less if the output is subnormal).
///
/// Special cases:
/// - $f(\text{NaN})=\text{NaN}$
/// - $f(\infty)=\infty$
/// - $f(-\infty)=0.0$
/// - $f(\pm0.0)=1.0$
///
/// Overflow and underflow are possible: a large positive `x` gives $\infty$, and a large negative
/// `x` gives `0.0`.
///
/// # Worst-case complexity
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use malachite_base::num::basic::traits::NegativeInfinity;
/// use malachite_base::num::float::NiceFloat;
/// use malachite_float::arithmetic::power_of_2_of_float::primitive_float_power_of_2;
///
/// assert!(primitive_float_power_of_2(f32::NAN).is_nan());
/// assert_eq!(
///     NiceFloat(primitive_float_power_of_2(f32::INFINITY)),
///     NiceFloat(f32::INFINITY)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_power_of_2(f32::NEGATIVE_INFINITY)),
///     NiceFloat(0.0)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_power_of_2(0.0f32)),
///     NiceFloat(1.0)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_power_of_2(1.0f32)),
///     NiceFloat(2.0)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_power_of_2(0.5f32)),
///     NiceFloat(1.4142135)
/// );
/// ```
#[inline]
#[allow(clippy::type_repetition_in_bounds)]
pub fn primitive_float_power_of_2<T: PrimitiveFloat>(x: T) -> T
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    emulate_float_to_float_fn(Float::power_of_2_of_float_prec, x)
}
