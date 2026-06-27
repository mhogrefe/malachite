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

use crate::InnerFloat::{Infinity, NaN, Zero};
use crate::arithmetic::exp::exp_overflow;
use crate::arithmetic::round_near_x::float_round_near_x;
use crate::{Float, emulate_float_to_float_fn, float_infinity, float_nan};
use core::cmp::Ordering::{self, *};
use core::cmp::max;
use malachite_base::num::arithmetic::traits::{CeilingLogBase2, ExpXMinus1, ExpXMinus1Assign};
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::{NegativeOne, One};
use malachite_base::num::conversion::traits::{ExactFrom, RoundingFrom};
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_nz::integer::Integer;
use malachite_nz::natural::arithmetic::float_extras::float_can_round;
use malachite_nz::platform::{Limb, SignedLimb};

// This is mpfr_expm1 from expm1.c, MPFR 4.2.2, where the input is finite and nonzero.
fn exp_x_minus_1_prec_round_normal(x: &Float, prec: u64, rm: RoundingMode) -> (Float, Ordering) {
    let ex = i64::from(x.get_exponent().unwrap());
    if ex < 0 {
        // -0.5 < x < 0.5. For 0 < x < 1, |expm1(x) - x| < x^2. For -1 < x < 0, |expm1(x) - x| < x^2
        // / 2. In both cases the error term is positive (expm1(x) > x), so it brings the result
        // away from zero for x > 0 and toward zero for x < 0.
        let (err, dir) = if *x > 0u32 {
            (-ex, true)
        } else {
            (-ex + 1, false)
        };
        let err = u64::exact_from(err);
        if err > prec + 1
            && let Some(result) = float_round_near_x(x, err, dir, prec, rm)
        {
            return result;
        }
    }
    // The result is never exactly representable for finite nonzero x.
    assert_ne!(rm, Exact, "Inexact exp_x_minus_1");
    const BP: u64 = 64;
    if x.is_sign_negative() && ex > 5 {
        // x <= -32, so exp(x) is tiny and expm1(x) = exp(x) - 1 is very close to -1 (slightly
        // toward zero). Since exp(x) = 2^(x / ln(2)), an upper bound on x / ln(2) (obtained by
        // dividing the negative x by an upper bound on ln(2)) gives an err with exp(x) < 2^(1 -
        // err), so -1 can be rounded directly. This also handles the regime where exp(x) would
        // underflow.
        let log2_up = Float::ln_2_prec_round(BP, Up).0;
        // Round the (negative) quotient toward +infinity to get an upper bound on x / ln(2). This
        // must be `Ceiling`, not `Up`: for hugely negative x, rounding away from zero would push
        // the magnitude past `MAX_EXPONENT` and overflow to -infinity, whereas `Ceiling` saturates
        // to the largest finite value.
        let t = x.div_prec_round_ref_val(log2_up, BP, Ceiling).0; // > x / ln(2)
        // err = -ceil(t), clamped to at most MAX_EXPONENT (avoiding overflow for huge |x|).
        let neg_ceil = -Integer::rounding_from(&t, Ceiling).0;
        const MAX_EXP: Integer = Integer::const_from_signed(Float::MAX_EXPONENT as SignedLimb);
        let err = u64::exact_from(&if neg_ceil > MAX_EXP {
            MAX_EXP
        } else {
            neg_ceil
        });
        if let Some(result) = float_round_near_x(&Float::NEGATIVE_ONE, err, false, prec, rm) {
            return result;
        }
    }
    // General case. Compute the precision of the intermediary variable: the optimal number of bits,
    // see algorithms.tex.
    let mut working_prec = prec + prec.ceiling_log_base_2() + 6;
    // If |x| is smaller than 2^(-e), we lose about e bits in the subtraction exp(x) - 1.
    if ex < 0 {
        working_prec += u64::exact_from(-ex);
    }
    let mut increment = Limb::WIDTH;
    loop {
        // exp(x) may overflow.
        let mut t = x.exp_prec_ref(working_prec).0;
        if t.is_infinite() {
            return exp_overflow(prec, rm);
        }
        // exp(x) cannot underflow here: that would require x / ln(2) < MIN_EXPONENT - 1, but then
        // the large-negative case above would already have returned.
        let exp_te = i64::from(t.get_exponent().unwrap());
        t.sub_prec_assign(Float::ONE, working_prec); // exp(x) - 1
        let t_exp = i64::from(t.get_exponent().unwrap());
        // The error estimate (cf. expm1.c). The cancellation `max(exp_te - t_exp, 0)` never reaches
        // `working_prec`: when |x| is small the cancellation is about -ex bits, which
        // `working_prec` already absorbs via the `+= -ex` above, so `err` stays positive.
        let err = working_prec - u64::exact_from(max(exp_te - t_exp, 0) + 1);
        if float_can_round(t.significand_ref().unwrap(), err, prec, rm) {
            return Float::from_float_prec_round(t, prec, rm);
        }
        // Increase the precision.
        working_prec += increment;
        increment = working_prec >> 1;
    }
}

impl Float {
    /// Computes $e^x-1$, where $x$ is a [`Float`], rounding the result to the specified precision
    /// and with the specified rounding mode. The [`Float`] is taken by value. An [`Ordering`] is
    /// also returned, indicating whether the rounded value is less than, equal to, or greater than
    /// the exact value. Although `NaN`s are not comparable to any [`Float`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = e^x-1+\varepsilon.
    /// $$
    /// - If $e^x-1$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $e^x-1$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |e^x-1|\rfloor-p+1}$.
    /// - If $e^x-1$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |e^x-1|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=\text{NaN}$
    /// - $f(\infty,p,m)=\infty$
    /// - $f(-\infty,p,m)=-1$
    /// - $f(\pm0.0,p,m)=\pm0.0$
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::exp_x_minus_1_prec`] instead.
    /// If you know that your target precision is the precision of the input, consider using
    /// [`Float::exp_x_minus_1_round`] instead. If both of these things are true, consider using
    /// [`Float::exp_x_minus_1`] instead.
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
    /// with the given precision. (The result cannot be represented exactly whenever the input is
    /// finite and nonzero.)
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (e, o) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .exp_x_minus_1_prec_round(20, Floor);
    /// assert_eq!(e.to_string(), "1.718281");
    /// assert_eq!(o, Less);
    ///
    /// let (e, o) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .exp_x_minus_1_prec_round(20, Ceiling);
    /// assert_eq!(e.to_string(), "1.718283");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn exp_x_minus_1_prec_round(self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        self.exp_x_minus_1_prec_round_ref(prec, rm)
    }

    /// Computes $e^x-1$, where $x$ is a [`Float`], rounding the result to the specified precision
    /// and with the specified rounding mode. The [`Float`] is taken by reference. An [`Ordering`]
    /// is also returned, indicating whether the rounded value is less than, equal to, or greater
    /// than the exact value. Although `NaN`s are not comparable to any [`Float`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = e^x-1+\varepsilon.
    /// $$
    /// - If $e^x-1$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $e^x-1$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |e^x-1|\rfloor-p+1}$.
    /// - If $e^x-1$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |e^x-1|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=\text{NaN}$
    /// - $f(\infty,p,m)=\infty$
    /// - $f(-\infty,p,m)=-1$
    /// - $f(\pm0.0,p,m)=\pm0.0$
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::exp_x_minus_1_prec_ref`]
    /// instead. If you know that your target precision is the precision of the input, consider
    /// using [`Float::exp_x_minus_1_round_ref`] instead. If both of these things are true, consider
    /// using `(&Float).exp_x_minus_1()` instead.
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
    /// with the given precision. (The result cannot be represented exactly whenever the input is
    /// finite and nonzero.)
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (e, o) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .exp_x_minus_1_prec_round_ref(20, Floor);
    /// assert_eq!(e.to_string(), "1.718281");
    /// assert_eq!(o, Less);
    ///
    /// let (e, o) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .exp_x_minus_1_prec_round_ref(20, Ceiling);
    /// assert_eq!(e.to_string(), "1.718283");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn exp_x_minus_1_prec_round_ref(&self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        assert_ne!(prec, 0);
        match self {
            Self(NaN) => (float_nan!(), Equal),
            float_infinity!() => (float_infinity!(), Equal),
            // expm1(-inf) = -1
            Self(Infinity { sign: false }) => (Float::from_signed_prec(-1i32, prec).0, Equal),
            // expm1(±0) = ±0
            Self(Zero { sign }) => (Self(Zero { sign: *sign }), Equal),
            _ => exp_x_minus_1_prec_round_normal(self, prec, rm),
        }
    }

    /// Computes $e^x-1$, where $x$ is a [`Float`], rounding the result to the nearest value of the
    /// specified precision. The [`Float`] is taken by value. An [`Ordering`] is also returned,
    /// indicating whether the rounded value is less than, equal to, or greater than the exact
    /// value. Although `NaN`s are not comparable to any [`Float`], whenever this function returns a
    /// `NaN` it also returns `Equal`.
    ///
    /// If the result is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = e^x-1+\varepsilon.
    /// $$
    /// - If $e^x-1$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $e^x-1$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |e^x-1|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p)=\text{NaN}$
    /// - $f(\infty,p)=\infty$
    /// - $f(-\infty,p)=-1$
    /// - $f(\pm0.0,p)=\pm0.0$
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::exp_x_minus_1_prec_round`] instead. If you know that your target precision is the
    /// precision of the input, consider using [`Float::exp_x_minus_1`] instead.
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
    /// let (e, o) = Float::from_unsigned_prec(1u32, 100).0.exp_x_minus_1_prec(20);
    /// assert_eq!(e.to_string(), "1.718283");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn exp_x_minus_1_prec(self, prec: u64) -> (Self, Ordering) {
        self.exp_x_minus_1_prec_round(prec, Nearest)
    }

    /// Computes $e^x-1$, where $x$ is a [`Float`], rounding the result to the nearest value of the
    /// specified precision. The [`Float`] is taken by reference. An [`Ordering`] is also returned,
    /// indicating whether the rounded value is less than, equal to, or greater than the exact
    /// value. Although `NaN`s are not comparable to any [`Float`], whenever this function returns a
    /// `NaN` it also returns `Equal`.
    ///
    /// If the result is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = e^x-1+\varepsilon.
    /// $$
    /// - If $e^x-1$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $e^x-1$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |e^x-1|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p)=\text{NaN}$
    /// - $f(\infty,p)=\infty$
    /// - $f(-\infty,p)=-1$
    /// - $f(\pm0.0,p)=\pm0.0$
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::exp_x_minus_1_prec_round_ref`] instead. If you know that your target precision is
    /// the precision of the input, consider using `(&Float).exp_x_minus_1()` instead.
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
    /// let (e, o) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .exp_x_minus_1_prec_ref(20);
    /// assert_eq!(e.to_string(), "1.718283");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn exp_x_minus_1_prec_ref(&self, prec: u64) -> (Self, Ordering) {
        self.exp_x_minus_1_prec_round_ref(prec, Nearest)
    }

    /// Computes $e^x-1$, where $x$ is a [`Float`], rounding the result with the specified rounding
    /// mode. The [`Float`] is taken by value. An [`Ordering`] is also returned, indicating whether
    /// the rounded value is less than, equal to, or greater than the exact value. Although `NaN`s
    /// are not comparable to any [`Float`], whenever this function returns a `NaN` it also returns
    /// `Equal`.
    ///
    /// The precision of the output is the precision of the input. See [`RoundingMode`] for a
    /// description of the possible rounding modes.
    ///
    /// $$
    /// f(x,m) = e^x-1+\varepsilon.
    /// $$
    /// - If $e^x-1$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $e^x-1$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |e^x-1|\rfloor-p+1}$, where $p$ is the precision of the input.
    /// - If $e^x-1$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |e^x-1|\rfloor-p}$, where $p$ is the precision of the input.
    ///
    /// If the output has a precision, it is the precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN},m)=\text{NaN}$
    /// - $f(\infty,m)=\infty$
    /// - $f(-\infty,m)=-1$
    /// - $f(\pm0.0,m)=\pm0.0$
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::exp_x_minus_1_prec_round`] instead. If you know you'll be using the `Nearest`
    /// rounding mode, consider using [`Float::exp_x_minus_1`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.get_prec()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the result cannot be represented exactly with the input
    /// precision. (The result cannot be represented exactly whenever the input is finite and
    /// nonzero.)
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (e, o) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .exp_x_minus_1_round(Floor);
    /// assert_eq!(e.to_string(), "1.718281828459045235360287471351");
    /// assert_eq!(o, Less);
    ///
    /// let (e, o) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .exp_x_minus_1_round(Ceiling);
    /// assert_eq!(e.to_string(), "1.718281828459045235360287471353");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn exp_x_minus_1_round(self, rm: RoundingMode) -> (Self, Ordering) {
        let prec = self.significant_bits();
        self.exp_x_minus_1_prec_round(prec, rm)
    }

    /// Computes $e^x-1$, where $x$ is a [`Float`], rounding the result with the specified rounding
    /// mode. The [`Float`] is taken by reference. An [`Ordering`] is also returned, indicating
    /// whether the rounded value is less than, equal to, or greater than the exact value. Although
    /// `NaN`s are not comparable to any [`Float`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// The precision of the output is the precision of the input. See [`RoundingMode`] for a
    /// description of the possible rounding modes.
    ///
    /// $$
    /// f(x,m) = e^x-1+\varepsilon.
    /// $$
    /// - If $e^x-1$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $e^x-1$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |e^x-1|\rfloor-p+1}$, where $p$ is the precision of the input.
    /// - If $e^x-1$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |e^x-1|\rfloor-p}$, where $p$ is the precision of the input.
    ///
    /// If the output has a precision, it is the precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN},m)=\text{NaN}$
    /// - $f(\infty,m)=\infty$
    /// - $f(-\infty,m)=-1$
    /// - $f(\pm0.0,m)=\pm0.0$
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::exp_x_minus_1_prec_round_ref`] instead. If you know you'll be using the `Nearest`
    /// rounding mode, consider using `(&Float).exp_x_minus_1()` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.get_prec()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the result cannot be represented exactly with the input
    /// precision. (The result cannot be represented exactly whenever the input is finite and
    /// nonzero.)
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (e, o) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .exp_x_minus_1_round_ref(Floor);
    /// assert_eq!(e.to_string(), "1.718281828459045235360287471351");
    /// assert_eq!(o, Less);
    ///
    /// let (e, o) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .exp_x_minus_1_round_ref(Ceiling);
    /// assert_eq!(e.to_string(), "1.718281828459045235360287471353");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn exp_x_minus_1_round_ref(&self, rm: RoundingMode) -> (Self, Ordering) {
        self.exp_x_minus_1_prec_round_ref(self.significant_bits(), rm)
    }

    /// Computes $e^x-1$, where $x$ is a [`Float`], in place, rounding the result to the specified
    /// precision and with the specified rounding mode. An [`Ordering`] is returned, indicating
    /// whether the rounded value is less than, equal to, or greater than the exact value. Although
    /// `NaN`s are not comparable to any [`Float`], whenever this function sets the [`Float`] to
    /// `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets e^x-1+\varepsilon.
    /// $$
    /// - If $e^x-1$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $e^x-1$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |e^x-1|\rfloor-p+1}$.
    /// - If $e^x-1$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |e^x-1|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// See the [`Float::exp_x_minus_1_prec_round`] documentation for information on special cases.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::exp_x_minus_1_prec_assign`]
    /// instead. If you know that your target precision is the precision of the input, consider
    /// using [`Float::exp_x_minus_1_round_assign`] instead. If both of these things are true,
    /// consider using [`Float::exp_x_minus_1_assign`] instead.
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
    /// with the given precision. (The result cannot be represented exactly whenever the input is
    /// finite and nonzero.)
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.exp_x_minus_1_prec_round_assign(20, Floor), Less);
    /// assert_eq!(x.to_string(), "1.718281");
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.exp_x_minus_1_prec_round_assign(20, Ceiling), Greater);
    /// assert_eq!(x.to_string(), "1.718283");
    /// ```
    #[inline]
    pub fn exp_x_minus_1_prec_round_assign(&mut self, prec: u64, rm: RoundingMode) -> Ordering {
        let (result, o) = core::mem::take(self).exp_x_minus_1_prec_round(prec, rm);
        *self = result;
        o
    }

    /// Computes $e^x-1$, where $x$ is a [`Float`], in place, rounding the result to the nearest
    /// value of the specified precision. An [`Ordering`] is returned, indicating whether the
    /// rounded value is less than, equal to, or greater than the exact value. Although `NaN`s are
    /// not comparable to any [`Float`], whenever this function sets the [`Float`] to `NaN` it also
    /// returns `Equal`.
    ///
    /// If the result is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// x \gets e^x-1+\varepsilon.
    /// $$
    /// - If $e^x-1$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $e^x-1$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |e^x-1|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// See the [`Float::exp_x_minus_1_prec`] documentation for information on special cases.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::exp_x_minus_1_prec_round_assign`] instead. If you know that your target precision
    /// is the precision of the input, consider using [`Float::exp_x_minus_1_assign`] instead.
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
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.exp_x_minus_1_prec_assign(20), Greater);
    /// assert_eq!(x.to_string(), "1.718283");
    /// ```
    #[inline]
    pub fn exp_x_minus_1_prec_assign(&mut self, prec: u64) -> Ordering {
        self.exp_x_minus_1_prec_round_assign(prec, Nearest)
    }

    /// Computes $e^x-1$, where $x$ is a [`Float`], in place, rounding the result with the specified
    /// rounding mode. An [`Ordering`] is returned, indicating whether the rounded value is less
    /// than, equal to, or greater than the exact value. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function sets the [`Float`] to `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the precision of the input. See [`RoundingMode`] for a
    /// description of the possible rounding modes.
    ///
    /// $$
    /// x \gets e^x-1+\varepsilon.
    /// $$
    /// - If $e^x-1$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $e^x-1$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |e^x-1|\rfloor-p+1}$, where $p$ is the precision of the input.
    /// - If $e^x-1$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |e^x-1|\rfloor-p}$, where $p$ is the precision of the input.
    ///
    /// If the output has a precision, it is the precision of the input.
    ///
    /// See the [`Float::exp_x_minus_1_round`] documentation for information on special cases.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::exp_x_minus_1_prec_round_assign`] instead. If you know you'll be using the
    /// `Nearest` rounding mode, consider using [`Float::exp_x_minus_1_assign`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.get_prec()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the result cannot be represented exactly with the input
    /// precision. (The result cannot be represented exactly whenever the input is finite and
    /// nonzero.)
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.exp_x_minus_1_round_assign(Floor), Less);
    /// assert_eq!(x.to_string(), "1.718281828459045235360287471351");
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.exp_x_minus_1_round_assign(Ceiling), Greater);
    /// assert_eq!(x.to_string(), "1.718281828459045235360287471353");
    /// ```
    #[inline]
    pub fn exp_x_minus_1_round_assign(&mut self, rm: RoundingMode) -> Ordering {
        let prec = self.significant_bits();
        self.exp_x_minus_1_prec_round_assign(prec, rm)
    }
}

impl ExpXMinus1 for Float {
    type Output = Self;

    /// Computes $e^x-1$, where $x$ is a [`Float`], taking the [`Float`] by value.
    ///
    /// If the output has a precision, it is the precision of the input. If the result is
    /// equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s in
    /// its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// $$
    /// f(x) = e^x-1+\varepsilon.
    /// $$
    /// - If $e^x-1$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $e^x-1$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |e^x-1|\rfloor-p}$, where $p$ is the precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN})=\text{NaN}$
    /// - $f(\infty)=\infty$
    /// - $f(-\infty)=-1$
    /// - $f(\pm0.0)=\pm0.0$
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::exp_x_minus_1_round`] instead. If you want to specify the output precision,
    /// consider using [`Float::exp_x_minus_1_prec`]. If you want both of these things, consider
    /// using [`Float::exp_x_minus_1_prec_round`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.get_prec()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::ExpXMinus1;
    /// use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity, One};
    /// use malachite_float::Float;
    ///
    /// assert!(Float::NAN.exp_x_minus_1().is_nan());
    /// assert_eq!(Float::INFINITY.exp_x_minus_1(), Float::INFINITY);
    /// assert_eq!(Float::NEGATIVE_INFINITY.exp_x_minus_1().to_string(), "-1.0");
    /// assert_eq!(Float::ONE.exp_x_minus_1().to_string(), "2.0");
    /// ```
    #[inline]
    fn exp_x_minus_1(self) -> Self {
        let prec = self.significant_bits();
        self.exp_x_minus_1_prec(prec).0
    }
}

impl ExpXMinus1 for &Float {
    type Output = Float;

    /// Computes $e^x-1$, where $x$ is a [`Float`], taking the [`Float`] by reference.
    ///
    /// If the output has a precision, it is the precision of the input. If the result is
    /// equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s in
    /// its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// $$
    /// f(x) = e^x-1+\varepsilon.
    /// $$
    /// - If $e^x-1$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $e^x-1$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |e^x-1|\rfloor-p}$, where $p$ is the precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN})=\text{NaN}$
    /// - $f(\infty)=\infty$
    /// - $f(-\infty)=-1$
    /// - $f(\pm0.0)=\pm0.0$
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::exp_x_minus_1_round_ref`] instead. If you want to specify the output precision,
    /// consider using [`Float::exp_x_minus_1_prec_ref`]. If you want both of these things, consider
    /// using [`Float::exp_x_minus_1_prec_round_ref`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.get_prec()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::ExpXMinus1;
    /// use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity, One};
    /// use malachite_float::Float;
    ///
    /// assert!((&Float::NAN).exp_x_minus_1().is_nan());
    /// assert_eq!((&Float::INFINITY).exp_x_minus_1(), Float::INFINITY);
    /// assert_eq!(
    ///     (&Float::NEGATIVE_INFINITY).exp_x_minus_1().to_string(),
    ///     "-1.0"
    /// );
    /// assert_eq!((&Float::ONE).exp_x_minus_1().to_string(), "2.0");
    /// ```
    #[inline]
    fn exp_x_minus_1(self) -> Float {
        self.exp_x_minus_1_prec_round_ref(self.significant_bits(), Nearest)
            .0
    }
}

impl ExpXMinus1Assign for Float {
    /// Computes $e^x-1$, where $x$ is a [`Float`], in place.
    ///
    /// If the output has a precision, it is the precision of the input. If the result is
    /// equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s in
    /// its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// $$
    /// x \gets e^x-1+\varepsilon.
    /// $$
    /// - If $e^x-1$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $e^x-1$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |e^x-1|\rfloor-p}$, where $p$ is the precision of the input.
    ///
    /// See the [`Float::exp_x_minus_1`] documentation for information on special cases.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::exp_x_minus_1_round_assign`] instead. If you want to specify the output precision,
    /// consider using [`Float::exp_x_minus_1_prec_assign`]. If you want both of these things,
    /// consider using [`Float::exp_x_minus_1_prec_round_assign`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.get_prec()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::ExpXMinus1Assign;
    /// use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity, One};
    /// use malachite_float::Float;
    ///
    /// let mut x = Float::NAN;
    /// x.exp_x_minus_1_assign();
    /// assert!(x.is_nan());
    ///
    /// let mut x = Float::INFINITY;
    /// x.exp_x_minus_1_assign();
    /// assert_eq!(x, Float::INFINITY);
    ///
    /// let mut x = Float::NEGATIVE_INFINITY;
    /// x.exp_x_minus_1_assign();
    /// assert_eq!(x.to_string(), "-1.0");
    ///
    /// let mut x = Float::ONE;
    /// x.exp_x_minus_1_assign();
    /// assert_eq!(x.to_string(), "2.0");
    /// ```
    #[inline]
    fn exp_x_minus_1_assign(&mut self) {
        let prec = self.significant_bits();
        self.exp_x_minus_1_prec_round_assign(prec, Nearest);
    }
}

/// Computes $e^x-1$ for a primitive float. Using this function is more accurate than using the
/// primitive float `exp_m1` function (the standard library's `exp_m1` is not correctly rounded).
///
/// $$
/// f(x) = e^x-1+\varepsilon.
/// $$
/// - If $e^x-1$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
/// - If $e^x-1$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2 |e^x-1|\rfloor-p}$,
///   where $p$ is precision of the output (typically 24 if `T` is a [`f32`] and 53 if `T` is a
///   [`f64`], but less if the output is subnormal).
///
/// Special cases:
/// - $f(\text{NaN})=\text{NaN}$
/// - $f(\infty)=\infty$
/// - $f(-\infty)=-1$
/// - $f(\pm0.0)=\pm0.0$
///
/// # Worst-case complexity
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use malachite_base::num::basic::traits::NegativeInfinity;
/// use malachite_base::num::float::NiceFloat;
/// use malachite_float::arithmetic::exp_x_minus_1::primitive_float_exp_x_minus_1;
///
/// assert!(primitive_float_exp_x_minus_1(f32::NAN).is_nan());
/// assert_eq!(
///     NiceFloat(primitive_float_exp_x_minus_1(f32::INFINITY)),
///     NiceFloat(f32::INFINITY)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_exp_x_minus_1(f32::NEGATIVE_INFINITY)),
///     NiceFloat(-1.0)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_exp_x_minus_1(1.0f32)),
///     NiceFloat(1.7182819)
/// );
/// ```
#[inline]
#[allow(clippy::type_repetition_in_bounds)]
pub fn primitive_float_exp_x_minus_1<T: PrimitiveFloat>(x: T) -> T
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    emulate_float_to_float_fn(Float::exp_x_minus_1_prec, x)
}
