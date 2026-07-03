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
use crate::arithmetic::exp::{exp_overflow, one_neighbor};
use crate::arithmetic::round_near_x::float_round_near_x;
use crate::{
    Float, emulate_float_to_float_fn, emulate_rational_to_float_fn, float_infinity, float_nan,
    float_zero, floor_and_ceiling,
};
use core::cmp::Ordering::{self, *};
use core::cmp::max;
use malachite_base::num::arithmetic::traits::{
    CeilingLogBase2, ExpXMinus1, ExpXMinus1Assign, Sign,
};
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::{NegativeOne, One, Zero as ZeroTrait};
use malachite_base::num::conversion::traits::{ExactFrom, RoundingFrom, SaturatingFrom};
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_nz::integer::Integer;
use malachite_nz::natural::arithmetic::float_extras::float_can_round;
use malachite_nz::platform::{Limb, SignedLimb};
use malachite_q::Rational;

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
        let clamped = neg_ceil >= MAX_EXP;
        let err = if clamped {
            u64::exact_from(Float::MAX_EXPONENT)
        } else {
            u64::exact_from(&neg_ceil)
        };
        if let Some(result) = float_round_near_x(&Float::NEGATIVE_ONE, err, false, prec, rm) {
            return result;
        }
        // `float_round_near_x` could not resolve the rounding, so prec + 1 >= err. If the clamp was
        // active, |x| / ln(2) can exceed MAX_EXPONENT: exp(x) may lie below the smallest positive
        // Float (so the loop below could not compute it), while prec is so large that the bits of
        // e^x may still land within the output's prec-bit window. Delegate to the deep-negative
        // helper. (Without the clamp, err = neg_ceil <= prec + 1, and neg_ceil < MAX_EXPONENT puts
        // |x| / ln(2) < neg_ceil + 2 <= 2^30 = |MIN_EXPONENT - 1|, so exp(x) does not underflow and
        // the loop below handles it.)
        if clamped {
            return exp_x_minus_1_deep_negative(x, prec, rm, u64::saturating_from(&neg_ceil));
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

// Computes e^x - 1 for a Float x so negative that e^x lies at or below the smallest positive Float
// (or nearly so: |x| / ln(2) >= MAX_EXPONENT), while prec + 1 >= MAX_EXPONENT, so rounding from -1
// cannot be certified. e^x is not representable, but since prec is enormous the bits of e^x may
// still land within the output's prec-bit window and must be computed for real. Since e^x - 1 =
// 2^(x / ln(2)) - 1, bracket y = x / ln(2) between dyadic Floats (y has magnitude about |x| / 0.7
// -- an ordinary Float, even though 2^y is not) and apply the monotonically increasing
// `power_of_2_x_minus_1_prec_round` to both ends, tightening the bracket Ziv-style until both round
// identically. That function's own deep-negative machinery computes 2^(y_end) - 1 exactly where
// needed, and its huge-negative shortcut keeps the ends cheap when |x| / ln(2) > prec + 1 (where
// the result is just -1 or its neighbor). `s_est` is a lower bound on |x| / ln(2), used to size the
// initial working precision: the result's leading ~|y| bits are a run of ones, so only about prec -
// s_est bits of 2^y are needed.
fn exp_x_minus_1_deep_negative(
    x: &Float,
    prec: u64,
    rm: RoundingMode,
    s_est: u64,
) -> (Float, Ordering) {
    let xr = Rational::exact_from(x);
    let mut working_prec = prec.saturating_add(2).saturating_sub(s_est) + (Limb::WIDTH << 1);
    let mut increment = Limb::WIDTH;
    loop {
        // ln_2_lo <= ln(2) <= ln_2_hi, as exact Rationals, from a single ln(2) computation.
        let (ln_2_lo, ln_2_hi) = floor_and_ceiling(Float::ln_2_prec_round(working_prec, Floor));
        // x < 0: dividing x by the smaller (larger) positive bound gives the more (less) negative
        // quotient, so these exact Rationals bracket y.
        let y_lo = &xr / Rational::exact_from(&ln_2_lo);
        let y_hi = &xr / Rational::exact_from(&ln_2_hi);
        // Widen to dyadic Floats, rounding outward.
        let y_lo = Float::from_rational_prec_round(y_lo, working_prec, Floor).0;
        let y_hi = Float::from_rational_prec_round(y_hi, working_prec, Ceiling).0;
        let (e_lo, mut o_lo) = y_lo.power_of_2_x_minus_1_prec_round(prec, rm);
        let (e_hi, mut o_hi) = y_hi.power_of_2_x_minus_1_prec_round(prec, rm);
        // A bracket end that lands on an integer y makes 2^y - 1 exactly representable, rounding
        // with `Equal`; the true value lies strictly between the ends, so the other end's ordering
        // is the true one. (Both cannot be `Equal`: the ends are distinct and the function is
        // strictly increasing.)
        if o_lo == Equal {
            o_lo = o_hi;
        }
        if o_hi == Equal {
            o_hi = o_lo;
        }
        if o_lo == o_hi && e_lo == e_hi {
            return (e_lo, o_lo);
        }
        working_prec += increment;
        increment = working_prec >> 1;
    }
}

// Computes `exp(x) - 1` for a nonzero `Rational` `x` with `|x| < 2^MIN_EXPONENT`, by summing its
// Taylor series `sum_{k>=1} x^k / k!`. Used when `x` is so small that `expm1(x) ~ x` underflows:
// the squeeze in `exp_x_minus_1_rational_helper` cannot bracket such an `x` (its Float bounds
// collapse to 0). The series is bracketed between two rationals which are rounded with
// `from_rational_prec_round` (which performs the underflow rounding) until both ends agree.
pub(crate) fn exp_x_minus_1_rational_near_zero(
    x: &Rational,
    prec: u64,
    rm: RoundingMode,
) -> (Float, Ordering) {
    let negative = *x < 0u32;
    let mut s = Rational::ZERO; // partial sum S_{k-1}, starting at S_0 = 0
    let mut term = Rational::ONE; // x^(k-1) / (k-1)!
    let mut k = 1u64;
    loop {
        term *= x;
        term /= Rational::from(k); // term = x^k / k!
        let s_next = &s + &term; // S_k
        let (lo, hi) = if negative {
            // The terms alternate in sign with strictly decreasing magnitude (|x| / (k + 1) < 1),
            // so expm1(x) lies between consecutive partial sums.
            if s < s_next {
                (s.clone(), s_next.clone())
            } else {
                (s_next.clone(), s.clone())
            }
        } else {
            // Every term is positive, so S_k < expm1(x), and the remainder is bounded by t_{k+1} /
            // (1
            // - x).
            let next = (&term * x) / Rational::from(k + 1); // t_{k+1}
            (s_next.clone(), &s_next + next / (Rational::ONE - x))
        };
        s = s_next;
        k += 1;
        let (f_lo, mut o_lo) = Float::from_rational_prec_round_ref(&lo, prec, rm);
        let (f_hi, mut o_hi) = Float::from_rational_prec_round_ref(&hi, prec, rm);
        // A bound that is exactly representable at `prec` rounds with `Equal`; treat it as agreeing
        // with the other bound. (`hi == 0` triggers this for small negative x, since 0 is exact.)
        if o_lo == Equal {
            o_lo = o_hi;
        }
        if o_hi == Equal {
            o_hi = o_lo;
        }
        if o_lo == o_hi && f_lo == f_hi {
            return (f_lo, o_lo);
        }
    }
}

// Computes `exp(x) - 1` for a nonzero `Rational` `x`, rounded to precision `prec` with rounding
// mode `rm`. (`expm1(0) = 0` is handled by the caller.) Because the value at a nonzero rational is
// transcendental, the result is never exactly representable, so `rm` must not be `Exact`.
fn exp_x_minus_1_rational_helper(x: &Rational, prec: u64, rm: RoundingMode) -> (Float, Ordering) {
    assert_ne!(rm, Exact, "Inexact exp_x_minus_1");
    let positive = x.sign() == Greater;
    let exp_x = x.floor_log_base_2_abs() + 1; // the MPFR-style exponent of x
    // x is too small to be represented as a normal Float (|x| < 2^MIN_EXPONENT). The squeeze below
    // cannot bracket it (its Float bounds would be 0), so sum the Taylor series instead. expm1(x) ~
    // x underflows, which `from_rational_prec_round` handles in the helper.
    if exp_x <= const { Float::MIN_EXPONENT as i64 } {
        return exp_x_minus_1_rational_near_zero(x, prec, rm);
    }
    // |x| is too large to be a finite Float. For x > 0, expm1(x) overflows to +inf; for x < 0,
    // expm1(x) = -1 + exp(x) tends to -1. Smaller x that still overflow are caught in the loop
    // below.
    if exp_x >= const { Float::MAX_EXPONENT as i64 } {
        if positive {
            return exp_overflow(prec, rm);
        }
        // exp(x) is far below ulp(-1) at any precision, so expm1(x) rounds to -1 or its toward-zero
        // neighbor.
        let err = u64::exact_from(Float::MAX_EXPONENT);
        if let Some(result) = float_round_near_x(&Float::NEGATIVE_ONE, err, false, prec, rm) {
            return result;
        }
        // `prec` is enormous (>= MAX_EXPONENT), so `float_round_near_x` cannot resolve the
        // rounding; but exp(x) is still far below ulp(-1), so -1 rounds the same way.
        return match rm {
            Ceiling | Down => (-one_neighbor(prec, false), Greater), // -1 + ulp (toward zero)
            _ => (-Float::one_prec(prec), Less),                     // -1
        };
    }
    // General case: bracket x between the Floats x_lo <= x <= x_hi, apply expm1 to both, and
    // increase the working precision until the two bounds round to the same result. expm1 is
    // monotonic, so once the bounds agree the exact expm1(x) (which lies between them) rounds the
    // same way.
    let mut working_prec = prec + 10;
    let mut increment = Limb::WIDTH;
    loop {
        let (x_lo, x_o) = Float::from_rational_prec_round_ref(x, working_prec, Floor);
        if x_o == Equal {
            // x is exactly representable at `working_prec`, so expm1(x) is simply expm1(x_lo).
            return x_lo.exp_x_minus_1_prec_round(prec, rm);
        }
        let (x_lo, x_hi) = floor_and_ceiling((x_lo, x_o));
        // expm1 of a finite nonzero Float is transcendental, so it is never exact: both orderings
        // are `Less` or `Greater`, never `Equal`.
        let (e_lo, o_lo) = x_lo.exp_x_minus_1_prec_round_ref(prec, rm);
        let (e_hi, o_hi) = x_hi.exp_x_minus_1_prec_round_ref(prec, rm);
        if o_lo == o_hi && e_lo == e_hi {
            return (e_lo, o_lo);
        }
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
    /// let (e, o) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .exp_x_minus_1_prec(20);
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

    /// Computes $e^x-1$, where $x$ is a [`Rational`], rounding the result to the specified
    /// precision and with the specified rounding mode and returning the result as a [`Float`]. The
    /// [`Rational`] is taken by value. An [`Ordering`] is also returned, indicating whether the
    /// rounded value is less than, equal to, or greater than the exact value.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = e^x-1+\varepsilon.
    /// $$
    /// - If $m$ is not `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2 |e^x-1|\rfloor-p+1}$.
    /// - If $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2 |e^x-1|\rfloor-p}$.
    ///
    /// These bounds do not apply when the result overflows or underflows; see below.
    ///
    /// The output has precision `prec`.
    ///
    /// Special cases:
    /// - $f(0,p,m)=0$.
    ///
    /// Overflow and underflow:
    /// - If $f(x,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$ is
    ///   returned instead.
    /// - If $0<f(x,p,m)<2^{-2^{30}}$ and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,p,m)<2^{-2^{30}}$ and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,p,m)\leq2^{-2^{30}-1}$ and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,p,m)<2^{-2^{30}}$ and $m$ is `Nearest`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,p,m)<0$ and $m$ is `Ceiling` or `Down`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,p,m)<0$ and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,p,m)<0$ and $m$ is `Nearest`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,p,m)<-2^{-2^{30}-1}$ and $m$ is `Nearest`, $-2^{-2^{30}}$ is returned
    ///   instead.
    ///
    /// Unlike $e^x$, $e^x-1$ never underflows to zero for large negative $x$: it instead tends to
    /// $-1$.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::exp_x_minus_1_rational_prec`]
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
    /// let (e, o) =
    ///     Float::exp_x_minus_1_rational_prec_round(Rational::from_unsigneds(3u8, 5), 5, Floor);
    /// assert_eq!(e.to_string(), "0.81");
    /// assert_eq!(o, Less);
    ///
    /// let (e, o) =
    ///     Float::exp_x_minus_1_rational_prec_round(Rational::from_unsigneds(3u8, 5), 5, Ceiling);
    /// assert_eq!(e.to_string(), "0.84");
    /// assert_eq!(o, Greater);
    ///
    /// let (e, o) =
    ///     Float::exp_x_minus_1_rational_prec_round(Rational::from_unsigneds(3u8, 5), 20, Floor);
    /// assert_eq!(e.to_string(), "0.822119");
    /// assert_eq!(o, Less);
    ///
    /// let (e, o) =
    ///     Float::exp_x_minus_1_rational_prec_round(Rational::from_unsigneds(3u8, 5), 20, Ceiling);
    /// assert_eq!(e.to_string(), "0.82212");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn exp_x_minus_1_rational_prec_round(
        x: Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        Self::exp_x_minus_1_rational_prec_round_ref(&x, prec, rm)
    }

    /// Computes $e^x-1$, where $x$ is a [`Rational`], rounding the result to the specified
    /// precision and with the specified rounding mode and returning the result as a [`Float`]. The
    /// [`Rational`] is taken by reference. An [`Ordering`] is also returned, indicating whether the
    /// rounded value is less than, equal to, or greater than the exact value.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = e^x-1+\varepsilon.
    /// $$
    /// - If $m$ is not `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2 |e^x-1|\rfloor-p+1}$.
    /// - If $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2 |e^x-1|\rfloor-p}$.
    ///
    /// These bounds do not apply when the result overflows or underflows; see below.
    ///
    /// The output has precision `prec`.
    ///
    /// Special cases:
    /// - $f(0,p,m)=0$.
    ///
    /// Overflow and underflow:
    /// - If $f(x,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$ is
    ///   returned instead.
    /// - If $0<f(x,p,m)<2^{-2^{30}}$ and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,p,m)<2^{-2^{30}}$ and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,p,m)\leq2^{-2^{30}-1}$ and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,p,m)<2^{-2^{30}}$ and $m$ is `Nearest`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,p,m)<0$ and $m$ is `Ceiling` or `Down`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,p,m)<0$ and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,p,m)<0$ and $m$ is `Nearest`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,p,m)<-2^{-2^{30}-1}$ and $m$ is `Nearest`, $-2^{-2^{30}}$ is returned
    ///   instead.
    ///
    /// Unlike $e^x$, $e^x-1$ never underflows to zero for large negative $x$: it instead tends to
    /// $-1$.
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`Float::exp_x_minus_1_rational_prec_ref`] instead.
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
    /// with the given precision (which is the case for every nonzero input).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (e, o) = Float::exp_x_minus_1_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(3u8, 5),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(e.to_string(), "0.81");
    /// assert_eq!(o, Less);
    ///
    /// let (e, o) = Float::exp_x_minus_1_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(3u8, 5),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(e.to_string(), "0.84");
    /// assert_eq!(o, Greater);
    ///
    /// let (e, o) = Float::exp_x_minus_1_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(3u8, 5),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(e.to_string(), "0.822119");
    /// assert_eq!(o, Less);
    ///
    /// let (e, o) = Float::exp_x_minus_1_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(3u8, 5),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(e.to_string(), "0.82212");
    /// assert_eq!(o, Greater);
    /// ```
    pub fn exp_x_minus_1_rational_prec_round_ref(
        x: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        assert_ne!(prec, 0);
        if *x == 0u32 {
            // expm1(0) = 0, exactly.
            return (float_zero!(), Equal);
        }
        exp_x_minus_1_rational_helper(x, prec, rm)
    }

    /// Computes $e^x-1$, where $x$ is a [`Rational`], rounding the result to the nearest value of
    /// the specified precision and returning the result as a [`Float`]. The [`Rational`] is taken
    /// by value. An [`Ordering`] is also returned, indicating whether the rounded value is less
    /// than, equal to, or greater than the exact value.
    ///
    /// If the value is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = e^x-1+\varepsilon,
    /// $$
    /// where $|\varepsilon| \leq 2^{\lfloor\log_2 |e^x-1|\rfloor-p}$ (unless the result overflows
    /// or underflows; see below).
    ///
    /// The output has precision `prec`.
    ///
    /// Special cases:
    /// - $f(0,p)=0$.
    ///
    /// Overflow and underflow:
    /// - If $f(x,p)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $0<f(x,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// Unlike $e^x$, $e^x-1$ never underflows to zero for large negative $x$: it instead tends to
    /// $-1$.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::exp_x_minus_1_rational_prec_round`] instead.
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
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (e, o) = Float::exp_x_minus_1_rational_prec(Rational::from_unsigneds(3u8, 5), 5);
    /// assert_eq!(e.to_string(), "0.81");
    /// assert_eq!(o, Less);
    ///
    /// let (e, o) = Float::exp_x_minus_1_rational_prec(Rational::from_unsigneds(3u8, 5), 20);
    /// assert_eq!(e.to_string(), "0.822119");
    /// assert_eq!(o, Less);
    ///
    /// let (e, o) = Float::exp_x_minus_1_rational_prec(Rational::from_signeds(-3i8, 5), 10);
    /// assert_eq!(e.to_string(), "-0.4512");
    /// assert_eq!(o, Greater);
    ///
    /// let (e, o) = Float::exp_x_minus_1_rational_prec(Rational::from(0), 10);
    /// assert_eq!(e.to_string(), "0.0");
    /// assert_eq!(o, Equal);
    /// ```
    #[inline]
    pub fn exp_x_minus_1_rational_prec(x: Rational, prec: u64) -> (Self, Ordering) {
        Self::exp_x_minus_1_rational_prec_round_ref(&x, prec, Nearest)
    }

    /// Computes $e^x-1$, where $x$ is a [`Rational`], rounding the result to the nearest value of
    /// the specified precision and returning the result as a [`Float`]. The [`Rational`] is taken
    /// by reference. An [`Ordering`] is also returned, indicating whether the rounded value is less
    /// than, equal to, or greater than the exact value.
    ///
    /// If the value is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = e^x-1+\varepsilon,
    /// $$
    /// where $|\varepsilon| \leq 2^{\lfloor\log_2 |e^x-1|\rfloor-p}$ (unless the result overflows
    /// or underflows; see below).
    ///
    /// The output has precision `prec`.
    ///
    /// Special cases:
    /// - $f(0,p)=0$.
    ///
    /// Overflow and underflow:
    /// - If $f(x,p)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $0<f(x,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// Unlike $e^x$, $e^x-1$ never underflows to zero for large negative $x$: it instead tends to
    /// $-1$.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::exp_x_minus_1_rational_prec_round_ref`] instead.
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
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (e, o) = Float::exp_x_minus_1_rational_prec_ref(&Rational::from_unsigneds(3u8, 5), 5);
    /// assert_eq!(e.to_string(), "0.81");
    /// assert_eq!(o, Less);
    ///
    /// let (e, o) = Float::exp_x_minus_1_rational_prec_ref(&Rational::from_unsigneds(3u8, 5), 20);
    /// assert_eq!(e.to_string(), "0.822119");
    /// assert_eq!(o, Less);
    ///
    /// let (e, o) = Float::exp_x_minus_1_rational_prec_ref(&Rational::from_signeds(-3i8, 5), 10);
    /// assert_eq!(e.to_string(), "-0.4512");
    /// assert_eq!(o, Greater);
    ///
    /// let (e, o) = Float::exp_x_minus_1_rational_prec_ref(&Rational::from(0), 10);
    /// assert_eq!(e.to_string(), "0.0");
    /// assert_eq!(o, Equal);
    /// ```
    #[inline]
    pub fn exp_x_minus_1_rational_prec_ref(x: &Rational, prec: u64) -> (Self, Ordering) {
        Self::exp_x_minus_1_rational_prec_round_ref(x, prec, Nearest)
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

/// Computes $e^x-1$, where $x$ is a [`Rational`], returning the result as a primitive float.
///
/// $$
/// f(x) = e^x-1+\varepsilon.
/// $$
/// - If $e^x-1$ is infinite or zero, $\varepsilon$ may be ignored or assumed to be 0.
/// - If $e^x-1$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2 |e^x-1|\rfloor-p}$,
///   where $p$ is the precision of the output (typically 24 if `T` is a [`f32`] and 53 if `T` is a
///   [`f64`], but less if the output is subnormal).
///
/// Special cases:
/// - $f(0)=0$
///
/// Overflow and underflow are possible: a large positive `x` gives $\infty$, and a small nonzero
/// `x` may give $\pm0.0$. Unlike $e^x$, a large negative `x` does not underflow to zero; it gives
/// `-1.0`.
///
/// # Worst-case complexity
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use malachite_base::num::basic::traits::Zero;
/// use malachite_base::num::float::NiceFloat;
/// use malachite_float::arithmetic::exp_x_minus_1::primitive_float_exp_x_minus_1_rational;
/// use malachite_q::Rational;
///
/// assert_eq!(
///     NiceFloat(primitive_float_exp_x_minus_1_rational::<f64>(
///         &Rational::ZERO
///     )),
///     NiceFloat(0.0)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_exp_x_minus_1_rational::<f64>(
///         &Rational::from_unsigneds(1u8, 3)
///     )),
///     NiceFloat(0.3956124250860895)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_exp_x_minus_1_rational::<f64>(
///         &Rational::from(10000)
///     )),
///     NiceFloat(f64::INFINITY)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_exp_x_minus_1_rational::<f64>(
///         &Rational::from(-10000)
///     )),
///     NiceFloat(-1.0)
/// );
/// ```
#[inline]
#[allow(clippy::type_repetition_in_bounds)]
pub fn primitive_float_exp_x_minus_1_rational<T: PrimitiveFloat>(x: &Rational) -> T
where
    Float: PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    emulate_rational_to_float_fn(Float::exp_x_minus_1_rational_prec_ref, x)
}
