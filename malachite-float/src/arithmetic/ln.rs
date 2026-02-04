// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the GNU MPFR Library.
//
//      Copyright 1999-2026 Free Software Foundation, Inc.
//
//      Contributed by the Pascaline and Caramba projects, INRIA.
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::InnerFloat::{Finite, Infinity, NaN, Zero};
use crate::{
    Float, emulate_float_to_float_fn, float_either_zero, float_infinity, float_nan,
    float_negative_infinity,
};
use core::cmp::Ordering::{self, *};
use core::mem::swap;
use malachite_base::num::arithmetic::traits::{Agm, CeilingLogBase2, Ln, LnAssign};
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::{One, Zero as ZeroTrait};
use malachite_base::num::conversion::traits::{ExactFrom, RoundingFrom, SaturatingFrom};
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_nz::natural::arithmetic::float_extras::float_can_round;
use malachite_nz::platform::Limb;

// The computation of log(x) is done using the formula: if we want p bits of the result,
// ```
//                    pi
//      log(x) ~ ------------- - m log 2
//               2 AG(1,4 / s)
// ```
// where s = x 2^m > 2^(p/2).
//
// More precisely, if F(x) = int(1 / sqrt(1 - (1 - x ^ 2) * sin(t) ^ 2), t = 0..pi / 2), then for s
// >= 1.26 we have log(s) < F(4 / s) < log(s) * (1 + 4 / s ^ 2) from which we deduce pi / 2 / AG(1,
// 4 / s) * (1 - 4 / s ^ 2) < log(s) < pi / 2 / AG(1, 4 / s) so the relative error 4 / s ^ 2 is < 4
// / 2 ^ p i.e. 4 ulps.
//
// This is mpfr_log from log.c, MPFR 4.2.0.
fn ln_prec_round_normal_ref(x: &Float, prec: u64, rm: RoundingMode) -> (Float, Ordering) {
    if *x == 1u32 {
        return (Float::ZERO, Equal);
    }
    assert_ne!(rm, Exact, "Inexact ln");
    let exp_a = x.get_exponent().unwrap();
    // use initial precision about q + 2 * lg(q) + cte
    let mut working_prec = prec + (prec.ceiling_log_base_2() << 1) + 10;
    let mut increment = Limb::WIDTH;
    let mut previous_m = 0;
    let mut x = x.clone();
    loop {
        // Calculus of m (depends on p)
        let m = i64::exact_from((working_prec + 3) >> 1)
            .checked_sub(i64::from(exp_a))
            .unwrap();
        x <<= m - previous_m;
        previous_m = m;
        assert!(x.is_normal());
        let tmp2 = Float::pi_prec(working_prec).0
            / (Float::ONE.agm(
                const { Float::const_from_unsigned(4) }
                    .div_prec_round_val_ref(&x, working_prec, Floor)
                    .0,
            ) << 1u32);
        let exp2 = tmp2.get_exponent();
        let tmp1 = tmp2
            - Float::ln_2_prec(working_prec)
                .0
                .mul_prec(Float::from(m), working_prec)
                .0;
        if let (Some(exp1), Some(exp2)) = (tmp1.get_exponent(), exp2) {
            let cancel = u64::saturating_from(exp2 - exp1);
            // we have 7 ulps of error from the above roundings, 4 ulps from the 4/s^2 second order
            // term, plus the canceled bits
            if float_can_round(
                tmp1.significand_ref().unwrap(),
                working_prec.saturating_sub(cancel).saturating_sub(4),
                prec,
                rm,
            ) {
                return Float::from_float_prec_round(tmp1, prec, rm);
            }
            working_prec += cancel + working_prec.ceiling_log_base_2();
        } else {
            working_prec += working_prec.ceiling_log_base_2();
        }
        working_prec += increment;
        increment = working_prec >> 1;
    }
}

// This is mpfr_log from log.c, MPFR 4.2.0.
fn ln_prec_round_normal(mut x: Float, prec: u64, rm: RoundingMode) -> (Float, Ordering) {
    if x == 1u32 {
        return (Float::ZERO, Equal);
    }
    assert_ne!(rm, Exact, "Inexact ln");
    let exp_a = x.get_exponent().unwrap();
    // use initial precision about q + 2 * lg(q) + cte
    let mut working_prec = prec + (prec.ceiling_log_base_2() << 1) + 10;
    let mut increment = Limb::WIDTH;
    let mut previous_m = 0;
    loop {
        // Calculus of m (depends on p)
        let m = i64::exact_from((working_prec + 3) >> 1)
            .checked_sub(i64::from(exp_a))
            .unwrap();
        x <<= m - previous_m;
        previous_m = m;
        assert!(x.is_normal());
        let tmp2 = Float::pi_prec(working_prec).0
            / (Float::ONE.agm(
                const { Float::const_from_unsigned(4) }
                    .div_prec_round_val_ref(&x, working_prec, Floor)
                    .0,
            ) << 1u32);
        let exp2 = tmp2.get_exponent();
        let tmp1 = tmp2
            - Float::ln_2_prec(working_prec)
                .0
                .mul_prec(Float::from(m), working_prec)
                .0;
        if let (Some(exp1), Some(exp2)) = (tmp1.get_exponent(), exp2) {
            let cancel = u64::saturating_from(exp2 - exp1);
            // we have 7 ulps of error from the above roundings, 4 ulps from the 4/s^2 second order
            // term, plus the canceled bits
            if float_can_round(
                tmp1.significand_ref().unwrap(),
                working_prec.saturating_sub(cancel).saturating_sub(4),
                prec,
                rm,
            ) {
                return Float::from_float_prec_round(tmp1, prec, rm);
            }
            working_prec += cancel + working_prec.ceiling_log_base_2();
        } else {
            working_prec += working_prec.ceiling_log_base_2();
        }
        working_prec += increment;
        increment = working_prec >> 1;
    }
}

impl Float {
    /// Computes the natural logarithm of a [`Float`], rounding the result to the specified
    /// precision and with the specified rounding mode. The [`Float`] is taken by value. An
    /// [`Ordering`] is also returned, indicating whether the rounded logarithm is less than, equal
    /// to, or greater than the exact logarithm. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The logarithm of any nonzero negative number is `NaN`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \ln{x}+\varepsilon.
    /// $$
    /// - If $\ln{x}$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $\ln{x}$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 \|ln{x}|\rfloor-p+1}$.
    /// - If $\ln{x}$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 \|ln{x}|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=\text{NaN}$
    /// - $f(\infty,p,m)=\infty$
    /// - $f(-\infty,p,m)=\text{NaN}$
    /// - $f(\pm0.0,p,m)=-\infty$
    ///
    /// Neither overflow nor underflow is possible.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::ln_prec`] instead. If you
    /// know that your target precision is the precision of the input, consider using
    /// [`Float::ln_round`] instead. If both of these things are true, consider using [`Float::ln`]
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n (\log n)^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the result cannot be represented exactly with the given
    /// precision.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (ln, o) = Float::from_unsigned_prec(10u32, 100)
    ///     .0
    ///     .ln_prec_round(5, Floor);
    /// assert_eq!(ln.to_string(), "2.2");
    /// assert_eq!(o, Less);
    ///
    /// let (ln, o) = Float::from_unsigned_prec(10u32, 100)
    ///     .0
    ///     .ln_prec_round(5, Ceiling);
    /// assert_eq!(ln.to_string(), "2.4");
    /// assert_eq!(o, Greater);
    ///
    /// let (ln, o) = Float::from_unsigned_prec(10u32, 100)
    ///     .0
    ///     .ln_prec_round(5, Nearest);
    /// assert_eq!(ln.to_string(), "2.2");
    /// assert_eq!(o, Less);
    ///
    /// let (ln, o) = Float::from_unsigned_prec(10u32, 100)
    ///     .0
    ///     .ln_prec_round(20, Floor);
    /// assert_eq!(ln.to_string(), "2.302582");
    /// assert_eq!(o, Less);
    ///
    /// let (ln, o) = Float::from_unsigned_prec(10u32, 100)
    ///     .0
    ///     .ln_prec_round(20, Ceiling);
    /// assert_eq!(ln.to_string(), "2.302586");
    /// assert_eq!(o, Greater);
    ///
    /// let (ln, o) = Float::from_unsigned_prec(10u32, 100)
    ///     .0
    ///     .ln_prec_round(20, Nearest);
    /// assert_eq!(ln.to_string(), "2.302586");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn ln_prec_round(self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        assert_ne!(prec, 0);
        match self {
            float_nan!() | float_negative_infinity!() => (float_nan!(), Equal),
            float_either_zero!() => (float_negative_infinity!(), Equal),
            float_infinity!() => (float_infinity!(), Equal),
            Self(Finite { sign, .. }) => {
                if !sign {
                    return (float_nan!(), Equal);
                }
                ln_prec_round_normal(self, prec, rm)
            }
        }
    }

    /// Computes the natural logarithm of a [`Float`], rounding the result to the specified
    /// precision and with the specified rounding mode. The [`Float`] is taken by reference. An
    /// [`Ordering`] is also returned, indicating whether the rounded logarithm is less than, equal
    /// to, or greater than the exact logarithm. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The logarithm of any nonzero negative number is `NaN`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \ln{x}+\varepsilon.
    /// $$
    /// - If $\ln{x}$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $\ln{x}$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 \|ln{x}|\rfloor-p+1}$.
    /// - If $\ln{x}$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 \|ln{x}|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=\text{NaN}$
    /// - $f(\infty,p,m)=\infty$
    /// - $f(-\infty,p,m)=\text{NaN}$
    /// - $f(\pm0.0,p,m)=-\infty$
    ///
    /// Neither overflow nor underflow is possible.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::ln_prec_ref`] instead. If you
    /// know that your target precision is the precision of the input, consider using
    /// [`Float::ln_round_ref`] instead. If both of these things are true, consider using
    /// `(&Float).ln()`instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n (\log n)^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the result cannot be represented exactly with the given
    /// precision.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (ln, o) = Float::from_unsigned_prec(10u32, 100)
    ///     .0
    ///     .ln_prec_round_ref(5, Floor);
    /// assert_eq!(ln.to_string(), "2.2");
    /// assert_eq!(o, Less);
    ///
    /// let (ln, o) = Float::from_unsigned_prec(10u32, 100)
    ///     .0
    ///     .ln_prec_round_ref(5, Ceiling);
    /// assert_eq!(ln.to_string(), "2.4");
    /// assert_eq!(o, Greater);
    ///
    /// let (ln, o) = Float::from_unsigned_prec(10u32, 100)
    ///     .0
    ///     .ln_prec_round_ref(5, Nearest);
    /// assert_eq!(ln.to_string(), "2.2");
    /// assert_eq!(o, Less);
    ///
    /// let (ln, o) = Float::from_unsigned_prec(10u32, 100)
    ///     .0
    ///     .ln_prec_round_ref(20, Floor);
    /// assert_eq!(ln.to_string(), "2.302582");
    /// assert_eq!(o, Less);
    ///
    /// let (ln, o) = Float::from_unsigned_prec(10u32, 100)
    ///     .0
    ///     .ln_prec_round_ref(20, Ceiling);
    /// assert_eq!(ln.to_string(), "2.302586");
    /// assert_eq!(o, Greater);
    ///
    /// let (ln, o) = Float::from_unsigned_prec(10u32, 100)
    ///     .0
    ///     .ln_prec_round_ref(20, Nearest);
    /// assert_eq!(ln.to_string(), "2.302586");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn ln_prec_round_ref(&self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        assert_ne!(prec, 0);
        match self {
            float_nan!() | float_negative_infinity!() => (float_nan!(), Equal),
            float_either_zero!() => (float_negative_infinity!(), Equal),
            float_infinity!() => (float_infinity!(), Equal),
            Self(Finite { sign, .. }) => {
                if !sign {
                    return (float_nan!(), Equal);
                }
                ln_prec_round_normal_ref(self, prec, rm)
            }
        }
    }

    /// Computes the natural logarithm of a [`Float`], rounding the result to the nearest value of
    /// the specified precision. The [`Float`] is taken by value. An [`Ordering`] is also returned,
    /// indicating whether the rounded logarithm is less than, equal to, or greater than the exact
    /// logarithm. Although `NaN`s are not comparable to any [`Float`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// The logarithm of any nonzero negative number is `NaN`.
    ///
    /// If the logarithm is equidistant from two [`Float`]s with the specified precision, the
    /// [`Float`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a
    /// description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = \ln{x}+\varepsilon.
    /// $$
    /// - If $\ln{x}$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $\ln{x}$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2
    ///   \ln{x}\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=\text{NaN}$
    /// - $f(\infty,p,m)=\infty$
    /// - $f(-\infty,p,m)=\text{NaN}$
    /// - $f(\pm0.0,p,m)=-\infty$
    ///
    /// Neither overflow nor underflow is possible.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::ln_prec_round`] instead. If you know that your target precision is the precision of
    /// the input, consider using [`Float::ln`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n (\log n)^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (ln, o) = Float::from_unsigned_prec(10u32, 100).0.ln_prec(5);
    /// assert_eq!(ln.to_string(), "2.2");
    /// assert_eq!(o, Less);
    ///
    /// let (ln, o) = Float::from_unsigned_prec(10u32, 100).0.ln_prec(20);
    /// assert_eq!(ln.to_string(), "2.302586");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn ln_prec(self, prec: u64) -> (Self, Ordering) {
        self.ln_prec_round(prec, Nearest)
    }

    /// Computes the natural logarithm of a [`Float`], rounding the result to the nearest value of
    /// the specified precision. The [`Float`] is taken by reference. An [`Ordering`] is also
    /// returned, indicating whether the rounded logarithm is less than, equal to, or greater than
    /// the exact logarithm. Although `NaN`s are not comparable to any [`Float`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// The logarithm of any nonzero negative number is `NaN`.
    ///
    /// If the logarithm is equidistant from two [`Float`]s with the specified precision, the
    /// [`Float`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a
    /// description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = \ln{x}+\varepsilon.
    /// $$
    /// - If $\ln{x}$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $\ln{x}$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2
    ///   \ln{x}\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p)=\text{NaN}$
    /// - $f(\infty,p)=\infty$
    /// - $f(-\infty,p)=\text{NaN}$
    /// - $f(\pm0.0,p)=-\infty$
    ///
    /// Neither overflow nor underflow is possible.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::ln_prec_round_ref`] instead. If you know that your target precision is the
    /// precision of the input, consider using `(&Float).ln()` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n (\log n)^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (ln, o) = Float::from_unsigned_prec(10u32, 100).0.ln_prec_ref(5);
    /// assert_eq!(ln.to_string(), "2.2");
    /// assert_eq!(o, Less);
    ///
    /// let (ln, o) = Float::from_unsigned_prec(10u32, 100).0.ln_prec_ref(20);
    /// assert_eq!(ln.to_string(), "2.302586");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn ln_prec_ref(&self, prec: u64) -> (Self, Ordering) {
        self.ln_prec_round_ref(prec, Nearest)
    }

    /// Computes the natural logarithm of a [`Float`], rounding the result with the specified
    /// rounding mode. The [`Float`] is taken by value. An [`Ordering`] is also returned, indicating
    /// whether the rounded logarithm is less than, equal to, or greater than the exact logarithm.
    /// Although `NaN`s are not comparable to any [`Float`], whenever this function returns a `NaN`
    /// it also returns `Equal`.
    ///
    /// The logarithm of any nonzero negative number is `NaN`.
    ///
    /// The precision of the output is the precision of the input. See [`RoundingMode`] for a
    /// description of the possible rounding modes.
    ///
    /// $$
    /// f(x,m) = \ln{x}+\varepsilon.
    /// $$
    /// - If $\ln{x}$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $\ln{x}$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 \|ln{x}|\rfloor-p+1}$, where $p$ is the precision of the input.
    /// - If $\ln{x}$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 \|ln{x}|\rfloor-p}$, where $p$ is the precision of the input.
    ///
    /// If the output has a precision, it is the precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN},m)=\text{NaN}$
    /// - $f(\infty,m)=\infty$
    /// - $f(-\infty,m)=\text{NaN}$
    /// - $f(\pm0.0,m)=-\infty$
    ///
    /// Neither overflow nor underflow is possible.
    ///
    /// If you want to specify an output precision, consider using [`Float::ln_prec_round`] instead.
    /// If you know you'll be using the `Nearest` rounding mode, consider using [`Float::ln`]
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n (\log n)^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.get_prec()`.
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
    /// let (ln, o) = Float::from_unsigned_prec(10u32, 100).0.ln_round(Floor);
    /// assert_eq!(ln.to_string(), "2.302585092994045684017991454684");
    /// assert_eq!(o, Less);
    ///
    /// let (ln, o) = Float::from_unsigned_prec(10u32, 100).0.ln_round(Ceiling);
    /// assert_eq!(ln.to_string(), "2.302585092994045684017991454687");
    /// assert_eq!(o, Greater);
    ///
    /// let (ln, o) = Float::from_unsigned_prec(10u32, 100).0.ln_round(Nearest);
    /// assert_eq!(ln.to_string(), "2.302585092994045684017991454684");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn ln_round(self, rm: RoundingMode) -> (Self, Ordering) {
        let prec = self.significant_bits();
        self.ln_prec_round(prec, rm)
    }

    /// Computes the natural logarithm of a [`Float`], rounding the result with the specified
    /// rounding mode. The [`Float`] is taken by reference. An [`Ordering`] is also returned,
    /// indicating whether the rounded logarithm is less than, equal to, or greater than the exact
    /// square root. Although `NaN`s are not comparable to any [`Float`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// The logarithm of any nonzero negative number is `NaN`.
    ///
    /// The precision of the output is the precision of the input. See [`RoundingMode`] for a
    /// description of the possible rounding modes.
    ///
    /// $$
    /// f(x,m) = \ln{x}+\varepsilon.
    /// $$
    /// - If $\ln{x}$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $\ln{x}$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 \|ln{x}|\rfloor-p+1}$, where $p$ is the precision of the input.
    /// - If $\ln{x}$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 \|ln{x}|\rfloor-p}$, where $p$ is the precision of the input.
    ///
    /// If the output has a precision, it is the precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN},m)=\text{NaN}$
    /// - $f(\infty,m)=\infty$
    /// - $f(-\infty,m)=\text{NaN}$
    /// - $f(\pm0.0,m)=-\infty$
    ///
    /// Neither overflow nor underflow is possible.
    ///
    /// If you want to specify an output precision, consider using [`Float::ln_prec_round_ref`]
    /// instead. If you know you'll be using the `Nearest` rounding mode, consider using
    /// `(&Float).ln()` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n (\log n)^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.get_prec()`.
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
    /// let (ln, o) = Float::from_unsigned_prec(10u32, 100).0.ln_round_ref(Floor);
    /// assert_eq!(ln.to_string(), "2.302585092994045684017991454684");
    /// assert_eq!(o, Less);
    ///
    /// let (ln, o) = Float::from_unsigned_prec(10u32, 100)
    ///     .0
    ///     .ln_round_ref(Ceiling);
    /// assert_eq!(ln.to_string(), "2.302585092994045684017991454687");
    /// assert_eq!(o, Greater);
    ///
    /// let (ln, o) = Float::from_unsigned_prec(10u32, 100)
    ///     .0
    ///     .ln_round_ref(Nearest);
    /// assert_eq!(ln.to_string(), "2.302585092994045684017991454684");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn ln_round_ref(&self, rm: RoundingMode) -> (Self, Ordering) {
        let prec = self.significant_bits();
        self.ln_prec_round_ref(prec, rm)
    }

    /// Computes the natural logarithm of a [`Float`] in place, rounding the result to the specified
    /// precision and with the specified rounding mode. An [`Ordering`] is returned, indicating
    /// whether the rounded logarithm is less than, equal to, or greater than the exact square root.
    /// Although `NaN`s are not comparable to any [`Float`], whenever this function sets the
    /// [`Float`] to `NaN` it also returns `Equal`.
    ///
    /// The logarithm of any nonzero negative number is `NaN`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets \ln{x}+\varepsilon.
    /// $$
    /// - If $\ln{x}$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $\ln{x}$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy|\rfloor-p+1}$.
    /// - If $\ln{x}$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 \|ln{x}|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// See the [`Float::ln_prec_round`] documentation for information on special cases, overflow,
    /// and underflow.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::ln_prec_assign`] instead. If
    /// you know that your target precision is the precision of the input, consider using
    /// [`Float::ln_round_assign`] instead. If both of these things are true, consider using
    /// [`Float::ln_assign`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n (\log n)^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the result cannot be represented exactly with the given
    /// precision.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from_unsigned_prec(10u32, 100).0;
    /// assert_eq!(x.ln_prec_round_assign(5, Floor), Less);
    /// assert_eq!(x.to_string(), "2.2");
    ///
    /// let mut x = Float::from_unsigned_prec(10u32, 100).0;
    /// assert_eq!(x.ln_prec_round_assign(5, Ceiling), Greater);
    /// assert_eq!(x.to_string(), "2.4");
    ///
    /// let mut x = Float::from_unsigned_prec(10u32, 100).0;
    /// assert_eq!(x.ln_prec_round_assign(5, Nearest), Less);
    /// assert_eq!(x.to_string(), "2.2");
    ///
    /// let mut x = Float::from_unsigned_prec(10u32, 100).0;
    /// assert_eq!(x.ln_prec_round_assign(20, Floor), Less);
    /// assert_eq!(x.to_string(), "2.302582");
    ///
    /// let mut x = Float::from_unsigned_prec(10u32, 100).0;
    /// assert_eq!(x.ln_prec_round_assign(20, Ceiling), Greater);
    /// assert_eq!(x.to_string(), "2.302586");
    ///
    /// let mut x = Float::from_unsigned_prec(10u32, 100).0;
    /// assert_eq!(x.ln_prec_round_assign(20, Nearest), Greater);
    /// assert_eq!(x.to_string(), "2.302586");
    /// ```
    #[inline]
    pub fn ln_prec_round_assign(&mut self, prec: u64, rm: RoundingMode) -> Ordering {
        let mut x = Float::ZERO;
        swap(self, &mut x);
        let o;
        (*self, o) = x.ln_prec_round(prec, rm);
        o
    }

    /// Computes the natural logarithm of a [`Float`] in place, rounding the result to the nearest
    /// value of the specified precision. An [`Ordering`] is returned, indicating whether the
    /// rounded logarithm is less than, equal to, or greater than the exact logarithm. Although
    /// `NaN`s are not comparable to any [`Float`], whenever this function sets the [`Float`] to
    /// `NaN` it also returns `Equal`.
    ///
    /// The logarithm of any nonzero negative number is `NaN`.
    ///
    /// If the logarithm is equidistant from two [`Float`]s with the specified precision, the
    /// [`Float`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a
    /// description of the `Nearest` rounding mode.
    ///
    /// $$
    /// x \gets \ln{x}+\varepsilon.
    /// $$
    /// - If $\ln{x}$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $\ln{x}$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2
    ///   \ln{x}\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// See the [`Float::ln_prec`] documentation for information on special cases, overflow, and
    /// underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::ln_prec_round_assign`] instead. If you know that your target precision is the
    /// precision of the input, consider using [`Float::ln`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n (\log n)^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from_unsigned_prec(10u32, 100).0;
    /// assert_eq!(x.ln_prec_assign(5), Less);
    /// assert_eq!(x.to_string(), "2.2");
    ///
    /// let mut x = Float::from_unsigned_prec(10u32, 100).0;
    /// assert_eq!(x.ln_prec_assign(20), Greater);
    /// assert_eq!(x.to_string(), "2.302586");
    /// ```
    #[inline]
    pub fn ln_prec_assign(&mut self, prec: u64) -> Ordering {
        self.ln_prec_round_assign(prec, Nearest)
    }

    /// Computes the natural logarithm of a [`Float`] in place, rounding the result with the
    /// specified rounding mode. An [`Ordering`] is returned, indicating whether the rounded
    /// logarithm is less than, equal to, or greater than the exact logarithm. Although `NaN`s are
    /// not comparable to any [`Float`], whenever this function sets the [`Float`] to `NaN` it also
    /// returns `Equal`.
    ///
    /// The logarithm of any nonzero negative number is `NaN`.
    ///
    /// The precision of the output is the precision of the input. See [`RoundingMode`] for a
    /// description of the possible rounding modes.
    ///
    /// $$
    /// x \gets \ln{x}+\varepsilon.
    /// $$
    /// - If $\ln{x}$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $\ln{x}$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 \|ln{x}|\rfloor-p+1}$, where $p$ is the maximum precision of the inputs.
    /// - If $\ln{x}$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 \|ln{x}|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// If the output has a precision, it is the precision of the input.
    ///
    /// See the [`Float::ln_round`] documentation for information on special cases, overflow, and
    /// underflow.
    ///
    /// If you want to specify an output precision, consider using [`Float::ln_prec_round_assign`]
    /// instead. If you know you'll be using the `Nearest` rounding mode, consider using
    /// [`Float::ln_assign`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n (\log n)^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.get_prec()`.
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
    /// let mut x = Float::from_unsigned_prec(10u32, 100).0;
    /// assert_eq!(x.ln_round_assign(Floor), Less);
    /// assert_eq!(x.to_string(), "2.302585092994045684017991454684");
    ///
    /// let mut x = Float::from_unsigned_prec(10u32, 100).0;
    /// assert_eq!(x.ln_round_assign(Ceiling), Greater);
    /// assert_eq!(x.to_string(), "2.302585092994045684017991454687");
    ///
    /// let mut x = Float::from_unsigned_prec(10u32, 100).0;
    /// assert_eq!(x.ln_round_assign(Nearest), Less);
    /// assert_eq!(x.to_string(), "2.302585092994045684017991454684");
    /// ```
    #[inline]
    pub fn ln_round_assign(&mut self, rm: RoundingMode) -> Ordering {
        let prec = self.significant_bits();
        self.ln_prec_round_assign(prec, rm)
    }
}

impl Ln for Float {
    type Output = Self;

    /// Computes the natural logarithm of a [`Float`], taking it by value.
    ///
    /// If the output has a precision, it is the precision of the input. If the logarithm is
    /// equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s in
    /// its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// The logarithm of any nonzero negative number is `NaN`.
    ///
    /// $$
    /// f(x) = \ln{x}+\varepsilon.
    /// $$
    /// - If $\ln{x}$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $\ln{x}$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2
    ///   \ln{x}\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// Special cases:
    /// - $f(\text{NaN})=\text{NaN}$
    /// - $f(\infty)=\infty$
    /// - $f(-\infty)=\text{NaN}$
    /// - $f(\pm0.0)=-\infty$
    ///
    /// Neither overflow nor underflow is possible.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using [`Float::ln_prec`]
    /// instead. If you want to specify the output precision, consider using [`Float::ln_round`]. If
    /// you want both of these things, consider using [`Float::ln_prec_round`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n (\log n)^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.get_prec()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Ln;
    /// use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity};
    /// use malachite_float::Float;
    ///
    /// assert!(Float::NAN.ln().is_nan());
    /// assert_eq!(Float::INFINITY.ln(), Float::INFINITY);
    /// assert!(Float::NEGATIVE_INFINITY.ln().is_nan());
    /// assert_eq!(
    ///     Float::from_unsigned_prec(10u32, 100).0.ln().to_string(),
    ///     "2.302585092994045684017991454684"
    /// );
    /// assert!(Float::from_signed_prec(-10, 100).0.ln().is_nan());
    /// ```
    #[inline]
    fn ln(self) -> Self {
        let prec = self.significant_bits();
        self.ln_prec_round(prec, Nearest).0
    }
}

impl Ln for &Float {
    type Output = Float;

    /// Computes the natural logarithm of a [`Float`], taking it by reference.
    ///
    /// If the output has a precision, it is the precision of the input. If the logarithm is
    /// equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s in
    /// its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// The logarithm of any nonzero negative number is `NaN`.
    ///
    /// $$
    /// f(x) = \ln{x}+\varepsilon.
    /// $$
    /// - If $\ln{x}$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $\ln{x}$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2
    ///   \ln{x}\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// Special cases:
    /// - $f(\text{NaN})=\text{NaN}$
    /// - $f(\infty)=\infty$
    /// - $f(-\infty)=\text{NaN}$
    /// - $f(\pm0.0)=-\infty$
    ///
    /// Neither overflow nor underflow is possible.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::ln_prec_ref`] instead. If you want to specify the output precision, consider using
    /// [`Float::ln_round_ref`]. If you want both of these things, consider using
    /// [`Float::ln_prec_round_ref`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n (\log n)^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.get_prec()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Ln;
    /// use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity};
    /// use malachite_float::Float;
    ///
    /// assert!((&Float::NAN).ln().is_nan());
    /// assert_eq!((&Float::INFINITY).ln(), Float::INFINITY);
    /// assert!((&Float::NEGATIVE_INFINITY).ln().is_nan());
    /// assert_eq!(
    ///     (&Float::from_unsigned_prec(10u32, 100).0).ln().to_string(),
    ///     "2.302585092994045684017991454684"
    /// );
    /// assert!((&Float::from_signed_prec(-10, 100).0).ln().is_nan());
    /// ```
    #[inline]
    fn ln(self) -> Float {
        let prec = self.significant_bits();
        self.ln_prec_round_ref(prec, Nearest).0
    }
}

impl LnAssign for Float {
    /// Computes the natural logarithm of a [`Float`] in place.
    ///
    /// If the output has a precision, it is the precision of the input. If the logarithm is
    /// equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s in
    /// its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// The logarithm of any nonzero negative number is `NaN`.
    ///
    /// $$
    /// x\gets = \ln{x}+\varepsilon.
    /// $$
    /// - If $\ln{x}$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $\ln{x}$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2
    ///   \ln{x}\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// See the [`Float::ln`] documentation for information on special cases, overflow, and
    /// underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::ln_prec_assign`] instead. If you want to specify the output precision, consider
    /// using [`Float::ln_round_assign`]. If you want both of these things, consider using
    /// [`Float::ln_prec_round_assign`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n (\log n)^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.get_prec()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::LnAssign;
    /// use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity};
    /// use malachite_float::Float;
    ///
    /// let mut x = Float::NAN;
    /// x.ln_assign();
    /// assert!(x.is_nan());
    ///
    /// let mut x = Float::INFINITY;
    /// x.ln_assign();
    /// assert_eq!(x, Float::INFINITY);
    ///
    /// let mut x = Float::NEGATIVE_INFINITY;
    /// x.ln_assign();
    /// assert!(x.is_nan());
    ///
    /// let mut x = Float::from_unsigned_prec(10u32, 100).0;
    /// x.ln_assign();
    /// assert_eq!(x.to_string(), "2.302585092994045684017991454684");
    ///
    /// let mut x = Float::from_signed_prec(-10, 100).0;
    /// x.ln_assign();
    /// assert!(x.is_nan());
    /// ```
    #[inline]
    fn ln_assign(&mut self) {
        let prec = self.significant_bits();
        self.ln_prec_round_assign(prec, Nearest);
    }
}

/// Computes the natural logarithm of a primitive float. Using this function is more accurate than
/// using the default `log` function or the one provided by `libm`.
///
/// The reciprocal square root of any nonzero negative number is `NaN`.
///
/// $$
/// f(x) = \ln x+\varepsilon.
/// $$
/// - If $\ln x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
/// - If $\ln x$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2 \ln x\rfloor-p}$,
///   where $p$ is precision of the output (typically 24 if `T` is a [`f32`] and 53 if `T` is a
///   [`f64`], but less if the output is subnormal).
///
/// Special cases:
/// - $f(\text{NaN})=\text{NaN}$
/// - $f(\infty)=\infty$
/// - $f(-\infty)=\text{NaN}$
/// - $f(\pm0.0)=-\infty$
///
/// Neither overflow nor underflow is possible.
///
/// # Worst-case complexity
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use malachite_base::num::basic::traits::NegativeInfinity;
/// use malachite_base::num::float::NiceFloat;
/// use malachite_float::arithmetic::ln::primitive_float_ln;
///
/// assert!(primitive_float_ln(f32::NAN).is_nan());
/// assert_eq!(
///     NiceFloat(primitive_float_ln(f32::INFINITY)),
///     NiceFloat(f32::INFINITY)
/// );
/// assert!(primitive_float_ln(f32::NEGATIVE_INFINITY).is_nan());
/// assert_eq!(NiceFloat(primitive_float_ln(10.0f32)), NiceFloat(2.3025851));
/// assert!(primitive_float_ln(-10.0f32).is_nan());
/// ```
#[inline]
#[allow(clippy::type_repetition_in_bounds)]
pub fn primitive_float_ln<T: PrimitiveFloat>(x: T) -> T
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    emulate_float_to_float_fn(Float::ln_prec, x)
}
