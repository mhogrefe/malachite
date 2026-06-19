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
use crate::arithmetic::round_near_x::float_round_near_x;
use crate::basic::extended::ExtendedFloat;
use crate::{
    Float, emulate_float_to_float_fn, emulate_rational_to_float_fn, float_either_zero,
    float_infinity, float_nan, float_negative_infinity,
};
use core::cmp::Ordering::{self, *};
use malachite_base::num::arithmetic::traits::{
    CeilingLogBase2, CheckedLogBase2, IsPowerOf2, LogBase2, LogBase2Assign, PowerOf2, Sign,
};
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::{One, Zero as ZeroTrait};
use malachite_base::num::conversion::traits::{ExactFrom, RoundingFrom};
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_nz::natural::arithmetic::float_extras::float_can_round;
use malachite_nz::platform::Limb;
use malachite_q::Rational;

// The computation of log_base_2(x) is done by log_base_2(x) = ln(x) / ln(2).
//
// This is mpfr_log2 from log2.c, MPFR 4.3.0, where the input is finite, nonzero, and positive.
fn log_base_2_prec_round_normal(x: &Float, prec: u64, rm: RoundingMode) -> (Float, Ordering) {
    // If x is 1, the result is 0.
    if *x == 1u32 {
        return (Float::ZERO, Equal);
    }
    // If x is 2^k, log_base_2(x) is exact (though possibly subject to rounding at the target
    // precision).
    if x.is_power_of_2() {
        return Float::from_signed_prec_round(i64::from(x.get_exponent().unwrap()) - 1, prec, rm);
    }
    // The result is never exactly representable for other inputs.
    assert_ne!(rm, Exact, "Inexact log_base_2");
    // Compute the precision of the intermediary variable: the optimal number of bits, see
    // algorithms.tex.
    let mut working_prec = prec + 3 + prec.ceiling_log_base_2();
    let mut increment = Limb::WIDTH;
    loop {
        // ln(x) / ln(2)
        let t = x
            .ln_prec_ref(working_prec)
            .0
            .div_prec(Float::ln_2_prec(working_prec).0, working_prec)
            .0;
        // Estimation of the error.
        if float_can_round(t.significand_ref().unwrap(), working_prec - 3, prec, rm) {
            return Float::from_float_prec_round(t, prec, rm);
        }
        // Increase the precision.
        working_prec += increment;
        increment = working_prec >> 1;
    }
}

// Computes `log_2(1 + eps)` for a small nonzero [`Rational`] `eps` (`x - 1`, where `x` is near 1).
// The result is near zero, so unlike the near-a-larger-power case it must be computed directly
// rather than rounded near an integer; a Ziv loop over a [`Float`] approximation of `eps` does so
// without the catastrophic cancellation that `ln(x)` would suffer for `x` near 1.
fn log_base_2_rational_near_one(eps: &Rational, prec: u64, rm: RoundingMode) -> (Float, Ordering) {
    let mut working_prec = prec + 3 + prec.ceiling_log_base_2();
    let mut increment = Limb::WIDTH;
    loop {
        // log_2(1 + eps), via a Float approximation of eps. The error comes from `eps_float`
        // approximating `eps` (below an ulp) and from the rounding in `log_base_2_1_plus_x` (below
        // an ulp), so a few ulps of slack suffice.
        let eps_float = Float::from_rational_prec_ref(eps, working_prec).0;
        let off = eps_float.log_base_2_1_plus_x_prec(working_prec).0;
        if float_can_round(off.significand_ref().unwrap(), working_prec - 3, prec, rm) {
            return Float::from_float_prec_round(off, prec, rm);
        }
        working_prec += increment;
        increment = working_prec >> 1;
    }
}

// If `x` is close enough to a power of 2 that the general Ziv loop would need a precision
// proportional to the distance (potentially exhausting memory), returns the correctly-rounded
// `log_2(x)`; otherwise returns `None`. `x` must be positive and not a power of 2.
//
// `log_2(x) = k + log_2(x / 2^k)` for the nearest power of 2, `2^k`. When `x` is very close to
// `2^k` the offset `log_2(x / 2^k)` is tiny: for `k != 0` the result is `k` nudged by a fraction of
// an ulp, which `float_round_near_x` rounds directly (returning `None` when the offset is not
// sub-ulp, so the general loop — which then converges quickly — takes over); for `k == 0` (`x`
// near 1) the result is the tiny offset itself.
fn log_base_2_rational_near_power_of_2(
    x: &Rational,
    prec: u64,
    rm: RoundingMode,
) -> Option<(Float, Ordering)> {
    // 2^m <= x < 2^(m + 1)
    let m = x.floor_log_base_2_abs();
    let pow_lo = Rational::power_of_2(m);
    let pow_hi = Rational::power_of_2(m + 1);
    // eps = x / 2^k - 1 for the nearer of the two surrounding powers of 2, 2^k.
    let dist_lo = x - &pow_lo;
    let dist_hi = &pow_hi - x;
    let (k, eps) = if dist_lo <= dist_hi {
        (m, dist_lo / pow_lo)
    } else {
        (m + 1, -(dist_hi / pow_hi))
    };
    if k == 0 {
        // x is near 1, so log_2(x) = log_2(1 + eps) is near zero.
        return Some(log_base_2_rational_near_one(&eps, prec, rm));
    }
    // eps is nonzero since x is not a power of 2.
    let eps_exp = eps.floor_log_base_2_abs();
    let k_float = Float::from_signed_prec(k, k.unsigned_abs().significant_bits()).0;
    let exp_k = i64::from(k_float.get_exponent().unwrap());
    // |log_2(1 + eps)| < 3|eps| < 2^(eps_exp + 3), so passing err = exp_k - eps_exp - 3 to
    // `float_round_near_x` (which requires |offset| < 2^(exp_k - err)) is sound.
    let err = exp_k - eps_exp - 3;
    if err <= 0 {
        return None;
    }
    // The offset moves the magnitude up (away from zero) iff it has the same sign as k.
    let dir = (eps > 0) == (k > 0);
    float_round_near_x(&k_float, u64::exact_from(err), dir, prec, rm)
}

// The computation of log_base_2(x) is done by log_base_2(x) = ln(x) / ln(2). `ln_rational_prec`
// handles inputs whose magnitudes are outside the representable range of `Float`; the result of the
// division has greater magnitude than the result of `ln_rational_prec`, but only by a factor of
// 1/ln(2), so the division cannot overflow or underflow if the `ln` didn't.
fn log_base_2_rational_prec_round_helper(
    x: &Rational,
    prec: u64,
    rm: RoundingMode,
) -> (Float, Ordering) {
    // When x is extremely close to a power of 2, log_2(x) is extremely close to an integer, and the
    // Ziv loop below would need a precision proportional to the distance to round it. Handle that
    // case separately.
    if let Some(result) = log_base_2_rational_near_power_of_2(x, prec, rm) {
        return result;
    }
    let mut working_prec = prec + 3 + prec.ceiling_log_base_2();
    let mut increment = Limb::WIDTH;
    loop {
        // ln(x) / ln(2)
        let t = Float::ln_rational_prec_ref(x, working_prec)
            .0
            .div_prec(Float::ln_2_prec(working_prec).0, working_prec)
            .0;
        // Estimation of the error.
        if float_can_round(t.significand_ref().unwrap(), working_prec - 3, prec, rm) {
            return Float::from_float_prec_round(t, prec, rm);
        }
        // Increase the precision.
        working_prec += increment;
        increment = working_prec >> 1;
    }
}

// Computes `log_2(r)` as an `ExtendedFloat`, accurate to within 2 ulps of `prec` bits. `r` must be
// positive and not equal to 1.
//
// The result is kept in the extended exponent range so that an `r` extremely close to 1 -- where
// `log_2(r)` is tiny and would underflow an ordinary `Float` (its value-exponent reaches `-2^63`,
// far below `MIN_EXPONENT = -(2^30 - 1)`) -- is represented faithfully rather than flushed to zero.
// This lets the logarithm-with-a-rational-base functions divide two such logs and clamp only once,
// at the very end, rather than losing the operand entirely.
//
// For `r` not pathologically near 1, the ordinary `log_2(r)` is a normal `Float`, correctly rounded
// (at most 1/2 ulp), and is simply wrapped. When `r` is within about `2^(-2^30)` of 1, `log_2(r) =
// log_2(1 + y)` with `y = r - 1`, and `log_2(1 + y) = y / ln 2 + O(y^2)`; here `|y| < 2^(-2^30)` is
// far smaller than `2^(-prec)`, so the `O(y^2)` term is below an ulp and `y / ln 2` (computed in
// the extended range, where `y`'s exponent fits in the `i64`) is accurate to within 2 ulps (1/2
// from the conversion of `y`, 1/2 from the division, the rest from the dropped term).
pub(crate) fn extended_log_base_2_of_rational(r: &Rational, prec: u64) -> ExtendedFloat {
    // `log_2(r)` underflows an ordinary `Float` only when `r` is within roughly `2^(-2^30)` of 1.
    // Switch to the linear approximation a couple of exponents before that boundary; the ordinary
    // path is then guaranteed not to underflow, and the linear path is valid well beyond it.
    let y = r - Rational::ONE;
    if y.floor_log_base_2_abs() <= i64::from(Float::MIN_EXPONENT) + 1 {
        let y_ext = ExtendedFloat::from_rational_prec_round_ref(&y, prec, Nearest).0;
        let ln_2 = ExtendedFloat::from(Float::ln_2_prec(prec).0);
        y_ext.div_prec_val_ref(&ln_2, prec).0
    } else {
        ExtendedFloat::from(Float::log_base_2_rational_prec_ref(r, prec).0)
    }
}

impl Float {
    /// Computes $\log_2 x$, where $x$ is a [`Float`], rounding the result to the specified
    /// precision and with the specified rounding mode. The [`Float`] is taken by value. An
    /// [`Ordering`] is also returned, indicating whether the rounded value is less than, equal to,
    /// or greater than the exact value. Although `NaN`s are not comparable to any [`Float`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The base-2 logarithm of any nonzero negative number is `NaN`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \log_2 x+\varepsilon.
    /// $$
    /// - If $\log_2 x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\log_2 x$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\log_2 x|\rfloor-p+1}$.
    /// - If $\log_2 x$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\log_2 x|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=\text{NaN}$
    /// - $f(\infty,p,m)=\infty$
    /// - $f(-\infty,p,m)=\text{NaN}$
    /// - $f(\pm0.0,p,m)=-\infty$
    /// - $f(1.0,p,m)=0.0$, and the result is exact
    /// - $f(2^k,p,m)=k$, rounded to precision $p$; the result is exact if and only if $k$ is
    ///   representable with precision $p$
    /// - $f(x,p,m)=\text{NaN}$ for $x<0$
    ///
    /// Neither overflow nor underflow is possible.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::log_base_2_prec`] instead. If
    /// you know that your target precision is the precision of the input, consider using
    /// [`Float::log_base_2_round`] instead. If both of these things are true, consider using
    /// [`Float::log_base_2`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n (\log n)^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with the given precision. (The result is exactly representable if and only if the input is
    /// `NaN`, infinite, zero, equal to 1, or a power of 2 whose base-2 logarithm is representable
    /// with the given precision.)
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (log, o) = Float::from_unsigned_prec(10u32, 100)
    ///     .0
    ///     .log_base_2_prec_round(5, Floor);
    /// assert_eq!(log.to_string(), "3.2");
    /// assert_eq!(o, Less);
    ///
    /// let (log, o) = Float::from_unsigned_prec(10u32, 100)
    ///     .0
    ///     .log_base_2_prec_round(5, Ceiling);
    /// assert_eq!(log.to_string(), "3.4");
    /// assert_eq!(o, Greater);
    ///
    /// let (log, o) = Float::from_unsigned_prec(10u32, 100)
    ///     .0
    ///     .log_base_2_prec_round(5, Nearest);
    /// assert_eq!(log.to_string(), "3.4");
    /// assert_eq!(o, Greater);
    ///
    /// let (log, o) = Float::from_unsigned_prec(10u32, 100)
    ///     .0
    ///     .log_base_2_prec_round(20, Floor);
    /// assert_eq!(log.to_string(), "3.321926");
    /// assert_eq!(o, Less);
    ///
    /// let (log, o) = Float::from_unsigned_prec(10u32, 100)
    ///     .0
    ///     .log_base_2_prec_round(20, Ceiling);
    /// assert_eq!(log.to_string(), "3.32193");
    /// assert_eq!(o, Greater);
    ///
    /// let (log, o) = Float::from_unsigned_prec(10u32, 100)
    ///     .0
    ///     .log_base_2_prec_round(20, Nearest);
    /// assert_eq!(log.to_string(), "3.32193");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn log_base_2_prec_round(self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        assert_ne!(prec, 0);
        match self {
            Self(NaN | Infinity { sign: false } | Finite { sign: false, .. }) => {
                (float_nan!(), Equal)
            }
            float_either_zero!() => (float_negative_infinity!(), Equal),
            float_infinity!() => (float_infinity!(), Equal),
            _ => log_base_2_prec_round_normal(&self, prec, rm),
        }
    }

    /// Computes $\log_2 x$, where $x$ is a [`Float`], rounding the result to the specified
    /// precision and with the specified rounding mode. The [`Float`] is taken by reference. An
    /// [`Ordering`] is also returned, indicating whether the rounded value is less than, equal to,
    /// or greater than the exact value. Although `NaN`s are not comparable to any [`Float`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The base-2 logarithm of any nonzero negative number is `NaN`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \log_2 x+\varepsilon.
    /// $$
    /// - If $\log_2 x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\log_2 x$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\log_2 x|\rfloor-p+1}$.
    /// - If $\log_2 x$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\log_2 x|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=\text{NaN}$
    /// - $f(\infty,p,m)=\infty$
    /// - $f(-\infty,p,m)=\text{NaN}$
    /// - $f(\pm0.0,p,m)=-\infty$
    /// - $f(1.0,p,m)=0.0$, and the result is exact
    /// - $f(2^k,p,m)=k$, rounded to precision $p$; the result is exact if and only if $k$ is
    ///   representable with precision $p$
    /// - $f(x,p,m)=\text{NaN}$ for $x<0$
    ///
    /// Neither overflow nor underflow is possible.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::log_base_2_prec_ref`]
    /// instead. If you know that your target precision is the precision of the input, consider
    /// using [`Float::log_base_2_round_ref`] instead. If both of these things are true, consider
    /// using `(&Float).log_base_2()` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n (\log n)^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with the given precision. (The result is exactly representable if and only if the input is
    /// `NaN`, infinite, zero, equal to 1, or a power of 2 whose base-2 logarithm is representable
    /// with the given precision.)
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (log, o) = Float::from_unsigned_prec(10u32, 100)
    ///     .0
    ///     .log_base_2_prec_round_ref(5, Floor);
    /// assert_eq!(log.to_string(), "3.2");
    /// assert_eq!(o, Less);
    ///
    /// let (log, o) = Float::from_unsigned_prec(10u32, 100)
    ///     .0
    ///     .log_base_2_prec_round_ref(5, Ceiling);
    /// assert_eq!(log.to_string(), "3.4");
    /// assert_eq!(o, Greater);
    ///
    /// let (log, o) = Float::from_unsigned_prec(10u32, 100)
    ///     .0
    ///     .log_base_2_prec_round_ref(5, Nearest);
    /// assert_eq!(log.to_string(), "3.4");
    /// assert_eq!(o, Greater);
    ///
    /// let (log, o) = Float::from_unsigned_prec(10u32, 100)
    ///     .0
    ///     .log_base_2_prec_round_ref(20, Floor);
    /// assert_eq!(log.to_string(), "3.321926");
    /// assert_eq!(o, Less);
    ///
    /// let (log, o) = Float::from_unsigned_prec(10u32, 100)
    ///     .0
    ///     .log_base_2_prec_round_ref(20, Ceiling);
    /// assert_eq!(log.to_string(), "3.32193");
    /// assert_eq!(o, Greater);
    ///
    /// let (log, o) = Float::from_unsigned_prec(10u32, 100)
    ///     .0
    ///     .log_base_2_prec_round_ref(20, Nearest);
    /// assert_eq!(log.to_string(), "3.32193");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn log_base_2_prec_round_ref(&self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        assert_ne!(prec, 0);
        match self {
            Self(NaN | Infinity { sign: false } | Finite { sign: false, .. }) => {
                (float_nan!(), Equal)
            }
            float_either_zero!() => (float_negative_infinity!(), Equal),
            float_infinity!() => (float_infinity!(), Equal),
            _ => log_base_2_prec_round_normal(self, prec, rm),
        }
    }

    /// Computes $\log_2 x$, where $x$ is a [`Float`], rounding the result to the nearest value of
    /// the specified precision. The [`Float`] is taken by value. An [`Ordering`] is also returned,
    /// indicating whether the rounded value is less than, equal to, or greater than the exact
    /// value. Although `NaN`s are not comparable to any [`Float`], whenever this function returns a
    /// `NaN` it also returns `Equal`.
    ///
    /// The base-2 logarithm of any nonzero negative number is `NaN`.
    ///
    /// If the logarithm is equidistant from two [`Float`]s with the specified precision, the
    /// [`Float`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a
    /// description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = \log_2 x+\varepsilon.
    /// $$
    /// - If $\log_2 x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\log_2 x$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2 |\log_2
    ///   x|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p)=\text{NaN}$
    /// - $f(\infty,p)=\infty$
    /// - $f(-\infty,p)=\text{NaN}$
    /// - $f(\pm0.0,p)=-\infty$
    /// - $f(1.0,p)=0.0$, and the result is exact
    /// - $f(2^k,p)=k$, rounded to precision $p$; the result is exact if and only if $k$ is
    ///   representable with precision $p$
    /// - $f(x,p)=\text{NaN}$ for $x<0$
    ///
    /// Neither overflow nor underflow is possible.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::log_base_2_prec_round`] instead. If you know that your target precision is the
    /// precision of the input, consider using [`Float::log_base_2`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n (\log n)^2)$
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
    /// let (log, o) = Float::from_unsigned_prec(10u32, 100).0.log_base_2_prec(5);
    /// assert_eq!(log.to_string(), "3.4");
    /// assert_eq!(o, Greater);
    ///
    /// let (log, o) = Float::from_unsigned_prec(10u32, 100).0.log_base_2_prec(20);
    /// assert_eq!(log.to_string(), "3.32193");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn log_base_2_prec(self, prec: u64) -> (Self, Ordering) {
        self.log_base_2_prec_round(prec, Nearest)
    }

    /// Computes $\log_2 x$, where $x$ is a [`Float`], rounding the result to the nearest value of
    /// the specified precision. The [`Float`] is taken by reference. An [`Ordering`] is also
    /// returned, indicating whether the rounded value is less than, equal to, or greater than the
    /// exact value. Although `NaN`s are not comparable to any [`Float`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// The base-2 logarithm of any nonzero negative number is `NaN`.
    ///
    /// If the logarithm is equidistant from two [`Float`]s with the specified precision, the
    /// [`Float`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a
    /// description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = \log_2 x+\varepsilon.
    /// $$
    /// - If $\log_2 x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\log_2 x$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2 |\log_2
    ///   x|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p)=\text{NaN}$
    /// - $f(\infty,p)=\infty$
    /// - $f(-\infty,p)=\text{NaN}$
    /// - $f(\pm0.0,p)=-\infty$
    /// - $f(1.0,p)=0.0$, and the result is exact
    /// - $f(2^k,p)=k$, rounded to precision $p$; the result is exact if and only if $k$ is
    ///   representable with precision $p$
    /// - $f(x,p)=\text{NaN}$ for $x<0$
    ///
    /// Neither overflow nor underflow is possible.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::log_base_2_prec_round_ref`] instead. If you know that your target precision is the
    /// precision of the input, consider using `(&Float).log_base_2()` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n (\log n)^2)$
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
    /// let (log, o) = Float::from_unsigned_prec(10u32, 100)
    ///     .0
    ///     .log_base_2_prec_ref(5);
    /// assert_eq!(log.to_string(), "3.4");
    /// assert_eq!(o, Greater);
    ///
    /// let (log, o) = Float::from_unsigned_prec(10u32, 100)
    ///     .0
    ///     .log_base_2_prec_ref(20);
    /// assert_eq!(log.to_string(), "3.32193");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn log_base_2_prec_ref(&self, prec: u64) -> (Self, Ordering) {
        self.log_base_2_prec_round_ref(prec, Nearest)
    }

    /// Computes $\log_2 x$, where $x$ is a [`Float`], rounding the result with the specified
    /// rounding mode. The [`Float`] is taken by value. An [`Ordering`] is also returned, indicating
    /// whether the rounded value is less than, equal to, or greater than the exact value. Although
    /// `NaN`s are not comparable to any [`Float`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// The base-2 logarithm of any nonzero negative number is `NaN`.
    ///
    /// The precision of the output is the precision of the input. See [`RoundingMode`] for a
    /// description of the possible rounding modes.
    ///
    /// $$
    /// f(x,m) = \log_2 x+\varepsilon.
    /// $$
    /// - If $\log_2 x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\log_2 x$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\log_2 x|\rfloor-p+1}$, where $p$ is the precision of the input.
    /// - If $\log_2 x$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\log_2 x|\rfloor-p}$, where $p$ is the precision of the input.
    ///
    /// If the output has a precision, it is the precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN},m)=\text{NaN}$
    /// - $f(\infty,m)=\infty$
    /// - $f(-\infty,m)=\text{NaN}$
    /// - $f(\pm0.0,m)=-\infty$
    /// - $f(1.0,m)=0.0$, and the result is exact
    /// - $f(2^k,m)=k$, rounded to the precision of the input; the result is exact if and only if
    ///   $k$ is representable with that precision
    /// - $f(x,m)=\text{NaN}$ for $x<0$
    ///
    /// Neither overflow nor underflow is possible.
    ///
    /// If you want to specify an output precision, consider using [`Float::log_base_2_prec_round`]
    /// instead. If you know you'll be using the `Nearest` rounding mode, consider using
    /// [`Float::log_base_2`] instead.
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
    /// precision. (The result is exactly representable if and only if the input is `NaN`, infinite,
    /// zero, equal to 1, or a power of 2 whose base-2 logarithm is representable with the input
    /// precision.)
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (log, o) = Float::from_unsigned_prec(10u32, 100)
    ///     .0
    ///     .log_base_2_round(Floor);
    /// assert_eq!(log.to_string(), "3.321928094887362347870319429487");
    /// assert_eq!(o, Less);
    ///
    /// let (log, o) = Float::from_unsigned_prec(10u32, 100)
    ///     .0
    ///     .log_base_2_round(Ceiling);
    /// assert_eq!(log.to_string(), "3.32192809488736234787031942949");
    /// assert_eq!(o, Greater);
    ///
    /// let (log, o) = Float::from_unsigned_prec(10u32, 100)
    ///     .0
    ///     .log_base_2_round(Nearest);
    /// assert_eq!(log.to_string(), "3.32192809488736234787031942949");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn log_base_2_round(self, rm: RoundingMode) -> (Self, Ordering) {
        let prec = self.significant_bits();
        self.log_base_2_prec_round(prec, rm)
    }

    /// Computes $\log_2 x$, where $x$ is a [`Float`], rounding the result with the specified
    /// rounding mode. The [`Float`] is taken by reference. An [`Ordering`] is also returned,
    /// indicating whether the rounded value is less than, equal to, or greater than the exact
    /// value. Although `NaN`s are not comparable to any [`Float`], whenever this function returns a
    /// `NaN` it also returns `Equal`.
    ///
    /// The base-2 logarithm of any nonzero negative number is `NaN`.
    ///
    /// The precision of the output is the precision of the input. See [`RoundingMode`] for a
    /// description of the possible rounding modes.
    ///
    /// $$
    /// f(x,m) = \log_2 x+\varepsilon.
    /// $$
    /// - If $\log_2 x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\log_2 x$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\log_2 x|\rfloor-p+1}$, where $p$ is the precision of the input.
    /// - If $\log_2 x$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\log_2 x|\rfloor-p}$, where $p$ is the precision of the input.
    ///
    /// If the output has a precision, it is the precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN},m)=\text{NaN}$
    /// - $f(\infty,m)=\infty$
    /// - $f(-\infty,m)=\text{NaN}$
    /// - $f(\pm0.0,m)=-\infty$
    /// - $f(1.0,m)=0.0$, and the result is exact
    /// - $f(2^k,m)=k$, rounded to the precision of the input; the result is exact if and only if
    ///   $k$ is representable with that precision
    /// - $f(x,m)=\text{NaN}$ for $x<0$
    ///
    /// Neither overflow nor underflow is possible.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::log_base_2_prec_round_ref`] instead. If you know you'll be using the `Nearest`
    /// rounding mode, consider using `(&Float).log_base_2()` instead.
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
    /// precision. (The result is exactly representable if and only if the input is `NaN`, infinite,
    /// zero, equal to 1, or a power of 2 whose base-2 logarithm is representable with the input
    /// precision.)
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (log, o) = Float::from_unsigned_prec(10u32, 100)
    ///     .0
    ///     .log_base_2_round_ref(Floor);
    /// assert_eq!(log.to_string(), "3.321928094887362347870319429487");
    /// assert_eq!(o, Less);
    ///
    /// let (log, o) = Float::from_unsigned_prec(10u32, 100)
    ///     .0
    ///     .log_base_2_round_ref(Ceiling);
    /// assert_eq!(log.to_string(), "3.32192809488736234787031942949");
    /// assert_eq!(o, Greater);
    ///
    /// let (log, o) = Float::from_unsigned_prec(10u32, 100)
    ///     .0
    ///     .log_base_2_round_ref(Nearest);
    /// assert_eq!(log.to_string(), "3.32192809488736234787031942949");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn log_base_2_round_ref(&self, rm: RoundingMode) -> (Self, Ordering) {
        let prec = self.significant_bits();
        self.log_base_2_prec_round_ref(prec, rm)
    }

    /// Computes $\log_2 x$, where $x$ is a [`Float`], in place, rounding the result to the
    /// specified precision and with the specified rounding mode. An [`Ordering`] is returned,
    /// indicating whether the rounded value is less than, equal to, or greater than the exact
    /// value. Although `NaN`s are not comparable to any [`Float`], whenever this function sets the
    /// [`Float`] to `NaN` it also returns `Equal`.
    ///
    /// The base-2 logarithm of any nonzero negative number is `NaN`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets \log_2 x+\varepsilon.
    /// $$
    /// - If $\log_2 x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\log_2 x$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\log_2 x|\rfloor-p+1}$.
    /// - If $\log_2 x$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\log_2 x|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// See the [`Float::log_base_2_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::log_base_2_prec_assign`]
    /// instead. If you know that your target precision is the precision of the input, consider
    /// using [`Float::log_base_2_round_assign`] instead. If both of these things are true, consider
    /// using [`Float::log_base_2_assign`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n (\log n)^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with the given precision. (The result is exactly representable if and only if the input is
    /// `NaN`, infinite, zero, equal to 1, or a power of 2 whose base-2 logarithm is representable
    /// with the given precision.)
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from_unsigned_prec(10u32, 100).0;
    /// assert_eq!(x.log_base_2_prec_round_assign(5, Floor), Less);
    /// assert_eq!(x.to_string(), "3.2");
    ///
    /// let mut x = Float::from_unsigned_prec(10u32, 100).0;
    /// assert_eq!(x.log_base_2_prec_round_assign(5, Ceiling), Greater);
    /// assert_eq!(x.to_string(), "3.4");
    ///
    /// let mut x = Float::from_unsigned_prec(10u32, 100).0;
    /// assert_eq!(x.log_base_2_prec_round_assign(5, Nearest), Greater);
    /// assert_eq!(x.to_string(), "3.4");
    ///
    /// let mut x = Float::from_unsigned_prec(10u32, 100).0;
    /// assert_eq!(x.log_base_2_prec_round_assign(20, Floor), Less);
    /// assert_eq!(x.to_string(), "3.321926");
    ///
    /// let mut x = Float::from_unsigned_prec(10u32, 100).0;
    /// assert_eq!(x.log_base_2_prec_round_assign(20, Ceiling), Greater);
    /// assert_eq!(x.to_string(), "3.32193");
    ///
    /// let mut x = Float::from_unsigned_prec(10u32, 100).0;
    /// assert_eq!(x.log_base_2_prec_round_assign(20, Nearest), Greater);
    /// assert_eq!(x.to_string(), "3.32193");
    /// ```
    #[inline]
    pub fn log_base_2_prec_round_assign(&mut self, prec: u64, rm: RoundingMode) -> Ordering {
        let (result, o) = core::mem::take(self).log_base_2_prec_round(prec, rm);
        *self = result;
        o
    }

    /// Computes $\log_2 x$, where $x$ is a [`Float`], in place, rounding the result to the nearest
    /// value of the specified precision. An [`Ordering`] is returned, indicating whether the
    /// rounded value is less than, equal to, or greater than the exact value. Although `NaN`s are
    /// not comparable to any [`Float`], whenever this function sets the [`Float`] to `NaN` it also
    /// returns `Equal`.
    ///
    /// The base-2 logarithm of any nonzero negative number is `NaN`.
    ///
    /// If the logarithm is equidistant from two [`Float`]s with the specified precision, the
    /// [`Float`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a
    /// description of the `Nearest` rounding mode.
    ///
    /// $$
    /// x \gets \log_2 x+\varepsilon.
    /// $$
    /// - If $\log_2 x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\log_2 x$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2 |\log_2
    ///   x|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// See the [`Float::log_base_2_prec`] documentation for information on special cases, overflow,
    /// and underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::log_base_2_prec_round_assign`] instead. If you know that your target precision is
    /// the precision of the input, consider using [`Float::log_base_2_assign`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n (\log n)^2)$
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
    /// let mut x = Float::from_unsigned_prec(10u32, 100).0;
    /// assert_eq!(x.log_base_2_prec_assign(5), Greater);
    /// assert_eq!(x.to_string(), "3.4");
    ///
    /// let mut x = Float::from_unsigned_prec(10u32, 100).0;
    /// assert_eq!(x.log_base_2_prec_assign(20), Greater);
    /// assert_eq!(x.to_string(), "3.32193");
    /// ```
    #[inline]
    pub fn log_base_2_prec_assign(&mut self, prec: u64) -> Ordering {
        self.log_base_2_prec_round_assign(prec, Nearest)
    }

    /// Computes $\log_2 x$, where $x$ is a [`Float`], in place, rounding the result with the
    /// specified rounding mode. An [`Ordering`] is returned, indicating whether the rounded value
    /// is less than, equal to, or greater than the exact value. Although `NaN`s are not comparable
    /// to any [`Float`], whenever this function sets the [`Float`] to `NaN` it also returns
    /// `Equal`.
    ///
    /// The base-2 logarithm of any nonzero negative number is `NaN`.
    ///
    /// The precision of the output is the precision of the input. See [`RoundingMode`] for a
    /// description of the possible rounding modes.
    ///
    /// $$
    /// x \gets \log_2 x+\varepsilon.
    /// $$
    /// - If $\log_2 x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\log_2 x$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\log_2 x|\rfloor-p+1}$, where $p$ is the precision of the input.
    /// - If $\log_2 x$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\log_2 x|\rfloor-p}$, where $p$ is the precision of the input.
    ///
    /// If the output has a precision, it is the precision of the input.
    ///
    /// See the [`Float::log_base_2_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::log_base_2_prec_round_assign`] instead. If you know you'll be using the `Nearest`
    /// rounding mode, consider using [`Float::log_base_2_assign`] instead.
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
    /// precision. (The result is exactly representable if and only if the input is `NaN`, infinite,
    /// zero, equal to 1, or a power of 2 whose base-2 logarithm is representable with the input
    /// precision.)
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from_unsigned_prec(10u32, 100).0;
    /// assert_eq!(x.log_base_2_round_assign(Floor), Less);
    /// assert_eq!(x.to_string(), "3.321928094887362347870319429487");
    ///
    /// let mut x = Float::from_unsigned_prec(10u32, 100).0;
    /// assert_eq!(x.log_base_2_round_assign(Ceiling), Greater);
    /// assert_eq!(x.to_string(), "3.32192809488736234787031942949");
    ///
    /// let mut x = Float::from_unsigned_prec(10u32, 100).0;
    /// assert_eq!(x.log_base_2_round_assign(Nearest), Greater);
    /// assert_eq!(x.to_string(), "3.32192809488736234787031942949");
    /// ```
    #[inline]
    pub fn log_base_2_round_assign(&mut self, rm: RoundingMode) -> Ordering {
        let prec = self.significant_bits();
        self.log_base_2_prec_round_assign(prec, rm)
    }

    /// Computes $\log_2 x$, where $x$ is a [`Rational`], rounding the result to the specified
    /// precision and with the specified rounding mode and returning the result as a [`Float`]. The
    /// [`Rational`] is taken by value. An [`Ordering`] is also returned, indicating whether the
    /// rounded value is less than, equal to, or greater than the exact value. Although `NaN`s are
    /// not comparable to any [`Float`], whenever this function returns a `NaN` it also returns
    /// `Equal`.
    ///
    /// The base-2 logarithm of any negative number is `NaN`.
    ///
    /// Inputs of any magnitude are handled, including [`Rational`]s whose magnitudes are too large
    /// or too small to be representable as [`Float`]s.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \log_2 x+\varepsilon.
    /// $$
    /// - If $\log_2 x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\log_2 x$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\log_2 x|\rfloor-p+1}$.
    /// - If $\log_2 x$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\log_2 x|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(0,p,m)=-\infty$
    /// - $f(x,p,m)=\text{NaN}$ for $x<0$
    /// - $f(1,p,m)=0.0$, and the result is exact
    /// - $f(2^k,p,m)=k$, rounded to precision $p$; the result is exact if and only if $k$ is
    ///   representable with precision $p$. This includes negative powers of 2 like $1/4$, and
    ///   powers of 2 whose exponents $k$ lie far outside the exponent range of [`Float`]; the
    ///   result is just the integer $k$ as a [`Float`].
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::log_base_2_rational_prec`]
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
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with the given precision. (The result is exactly representable if and only if $x\leq 0$ or
    /// $x$ is a power of 2 whose base-2 logarithm is representable with the given precision.)
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (log, o) =
    ///     Float::log_base_2_rational_prec_round(Rational::from_unsigneds(3u8, 5), 5, Floor);
    /// assert_eq!(log.to_string(), "-0.75");
    /// assert_eq!(o, Less);
    ///
    /// let (log, o) =
    ///     Float::log_base_2_rational_prec_round(Rational::from_unsigneds(3u8, 5), 5, Ceiling);
    /// assert_eq!(log.to_string(), "-0.72");
    /// assert_eq!(o, Greater);
    ///
    /// let (log, o) =
    ///     Float::log_base_2_rational_prec_round(Rational::from_unsigneds(3u8, 5), 5, Nearest);
    /// assert_eq!(log.to_string(), "-0.75");
    /// assert_eq!(o, Less);
    ///
    /// let (log, o) =
    ///     Float::log_base_2_rational_prec_round(Rational::from_unsigneds(3u8, 5), 20, Floor);
    /// assert_eq!(log.to_string(), "-0.736966");
    /// assert_eq!(o, Less);
    ///
    /// let (log, o) =
    ///     Float::log_base_2_rational_prec_round(Rational::from_unsigneds(3u8, 5), 20, Ceiling);
    /// assert_eq!(log.to_string(), "-0.736965");
    /// assert_eq!(o, Greater);
    ///
    /// let (log, o) =
    ///     Float::log_base_2_rational_prec_round(Rational::from_unsigneds(3u8, 5), 20, Nearest);
    /// assert_eq!(log.to_string(), "-0.736965");
    /// assert_eq!(o, Greater);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn log_base_2_rational_prec_round(
        x: Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        Self::log_base_2_rational_prec_round_ref(&x, prec, rm)
    }

    /// Computes $\log_2 x$, where $x$ is a [`Rational`], rounding the result to the specified
    /// precision and with the specified rounding mode and returning the result as a [`Float`]. The
    /// [`Rational`] is taken by reference. An [`Ordering`] is also returned, indicating whether the
    /// rounded value is less than, equal to, or greater than the exact value. Although `NaN`s are
    /// not comparable to any [`Float`], whenever this function returns a `NaN` it also returns
    /// `Equal`.
    ///
    /// The base-2 logarithm of any negative number is `NaN`.
    ///
    /// Inputs of any magnitude are handled, including [`Rational`]s whose magnitudes are too large
    /// or too small to be representable as [`Float`]s.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \log_2 x+\varepsilon.
    /// $$
    /// - If $\log_2 x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\log_2 x$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\log_2 x|\rfloor-p+1}$.
    /// - If $\log_2 x$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\log_2 x|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(0,p,m)=-\infty$
    /// - $f(x,p,m)=\text{NaN}$ for $x<0$
    /// - $f(1,p,m)=0.0$, and the result is exact
    /// - $f(2^k,p,m)=k$, rounded to precision $p$; the result is exact if and only if $k$ is
    ///   representable with precision $p$. This includes negative powers of 2 like $1/4$, and
    ///   powers of 2 whose exponents $k$ lie far outside the exponent range of [`Float`]; the
    ///   result is just the integer $k$ as a [`Float`].
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`Float::log_base_2_rational_prec_ref`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n (\log n)^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with the given precision. (The result is exactly representable if and only if $x\leq 0$ or
    /// $x$ is a power of 2 whose base-2 logarithm is representable with the given precision.)
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (log, o) =
    ///     Float::log_base_2_rational_prec_round_ref(&Rational::from_unsigneds(3u8, 5), 5, Floor);
    /// assert_eq!(log.to_string(), "-0.75");
    /// assert_eq!(o, Less);
    ///
    /// let (log, o) = Float::log_base_2_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(3u8, 5),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(log.to_string(), "-0.72");
    /// assert_eq!(o, Greater);
    ///
    /// let (log, o) = Float::log_base_2_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(3u8, 5),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(log.to_string(), "-0.75");
    /// assert_eq!(o, Less);
    ///
    /// let (log, o) =
    ///     Float::log_base_2_rational_prec_round_ref(&Rational::from_unsigneds(3u8, 5), 20, Floor);
    /// assert_eq!(log.to_string(), "-0.736966");
    /// assert_eq!(o, Less);
    ///
    /// let (log, o) = Float::log_base_2_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(3u8, 5),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(log.to_string(), "-0.736965");
    /// assert_eq!(o, Greater);
    ///
    /// let (log, o) = Float::log_base_2_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(3u8, 5),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(log.to_string(), "-0.736965");
    /// assert_eq!(o, Greater);
    /// ```
    pub fn log_base_2_rational_prec_round_ref(
        x: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        assert_ne!(prec, 0);
        match x.sign() {
            Equal => return (float_negative_infinity!(), Equal),
            Less => return (float_nan!(), Equal),
            Greater => {}
        }
        // If x is 2^k, log_base_2(x) is exact (though possibly subject to rounding at the target
        // precision).
        if let Some(k) = x.checked_log_base_2() {
            return Self::from_signed_prec_round(k, prec, rm);
        }
        // The result is never exactly representable for other inputs.
        assert_ne!(rm, Exact, "Inexact log_base_2");
        log_base_2_rational_prec_round_helper(x, prec, rm)
    }

    /// Computes $\log_2 x$, where $x$ is a [`Rational`], rounding the result to the nearest value
    /// of the specified precision and returning the result as a [`Float`]. The [`Rational`] is
    /// taken by value. An [`Ordering`] is also returned, indicating whether the rounded value is
    /// less than, equal to, or greater than the exact value. Although `NaN`s are not comparable to
    /// any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The base-2 logarithm of any negative number is `NaN`.
    ///
    /// Inputs of any magnitude are handled, including [`Rational`]s whose magnitudes are too large
    /// or too small to be representable as [`Float`]s.
    ///
    /// If the logarithm is equidistant from two [`Float`]s with the specified precision, the
    /// [`Float`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a
    /// description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = \log_2 x+\varepsilon.
    /// $$
    /// - If $\log_2 x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\log_2 x$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2 |\log_2
    ///   x|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(0,p)=-\infty$
    /// - $f(x,p)=\text{NaN}$ for $x<0$
    /// - $f(1,p)=0.0$, and the result is exact
    /// - $f(2^k,p)=k$, rounded to precision $p$; the result is exact if and only if $k$ is
    ///   representable with precision $p$. This includes negative powers of 2 like $1/4$, and
    ///   powers of 2 whose exponents $k$ lie far outside the exponent range of [`Float`]; the
    ///   result is just the integer $k$ as a [`Float`].
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::log_base_2_rational_prec_round`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n (\log n)^2)$
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
    /// let (log, o) = Float::log_base_2_rational_prec(Rational::from_unsigneds(3u8, 5), 5);
    /// assert_eq!(log.to_string(), "-0.75");
    /// assert_eq!(o, Less);
    ///
    /// let (log, o) = Float::log_base_2_rational_prec(Rational::from_unsigneds(3u8, 5), 20);
    /// assert_eq!(log.to_string(), "-0.736965");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn log_base_2_rational_prec(x: Rational, prec: u64) -> (Self, Ordering) {
        Self::log_base_2_rational_prec_round(x, prec, Nearest)
    }

    /// Computes $\log_2 x$, where $x$ is a [`Rational`], rounding the result to the nearest value
    /// of the specified precision and returning the result as a [`Float`]. The [`Rational`] is
    /// taken by reference. An [`Ordering`] is also returned, indicating whether the rounded value
    /// is less than, equal to, or greater than the exact value. Although `NaN`s are not comparable
    /// to any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The base-2 logarithm of any negative number is `NaN`.
    ///
    /// Inputs of any magnitude are handled, including [`Rational`]s whose magnitudes are too large
    /// or too small to be representable as [`Float`]s.
    ///
    /// If the logarithm is equidistant from two [`Float`]s with the specified precision, the
    /// [`Float`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a
    /// description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = \log_2 x+\varepsilon.
    /// $$
    /// - If $\log_2 x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\log_2 x$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2 |\log_2
    ///   x|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(0,p)=-\infty$
    /// - $f(x,p)=\text{NaN}$ for $x<0$
    /// - $f(1,p)=0.0$, and the result is exact
    /// - $f(2^k,p)=k$, rounded to precision $p$; the result is exact if and only if $k$ is
    ///   representable with precision $p$. This includes negative powers of 2 like $1/4$, and
    ///   powers of 2 whose exponents $k$ lie far outside the exponent range of [`Float`]; the
    ///   result is just the integer $k$ as a [`Float`].
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::log_base_2_rational_prec_round_ref`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n (\log n)^2)$
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
    /// let (log, o) = Float::log_base_2_rational_prec_ref(&Rational::from_unsigneds(3u8, 5), 5);
    /// assert_eq!(log.to_string(), "-0.75");
    /// assert_eq!(o, Less);
    ///
    /// let (log, o) = Float::log_base_2_rational_prec_ref(&Rational::from_unsigneds(3u8, 5), 20);
    /// assert_eq!(log.to_string(), "-0.736965");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn log_base_2_rational_prec_ref(x: &Rational, prec: u64) -> (Self, Ordering) {
        Self::log_base_2_rational_prec_round_ref(x, prec, Nearest)
    }
}

impl LogBase2 for Float {
    type Output = Self;

    /// Computes $\log_2 x$, where $x$ is a [`Float`], taking it by value.
    ///
    /// If the output has a precision, it is the precision of the input. If the logarithm is
    /// equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s in
    /// its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// The base-2 logarithm of any nonzero negative number is `NaN`.
    ///
    /// $$
    /// f(x) = \log_2 x+\varepsilon.
    /// $$
    /// - If $\log_2 x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\log_2 x$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2 |\log_2
    ///   x|\rfloor-p}$, where $p$ is the precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN})=\text{NaN}$
    /// - $f(\infty)=\infty$
    /// - $f(-\infty)=\text{NaN}$
    /// - $f(\pm0.0)=-\infty$
    /// - $f(1.0)=0.0$, and the result is exact
    /// - $f(2^k)=k$, rounded to the precision of the input; the result is exact if and only if $k$
    ///   is representable with that precision
    /// - $f(x)=\text{NaN}$ for $x<0$
    ///
    /// Neither overflow nor underflow is possible.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::log_base_2_round`] instead. If you want to specify the output precision, consider
    /// using [`Float::log_base_2_prec`]. If you want both of these things, consider using
    /// [`Float::log_base_2_prec_round`].
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
    /// use malachite_base::num::arithmetic::traits::LogBase2;
    /// use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity};
    /// use malachite_float::Float;
    ///
    /// assert!(Float::NAN.log_base_2().is_nan());
    /// assert_eq!(Float::INFINITY.log_base_2(), Float::INFINITY);
    /// assert!(Float::NEGATIVE_INFINITY.log_base_2().is_nan());
    /// assert_eq!(
    ///     Float::from_unsigned_prec(10u32, 100)
    ///         .0
    ///         .log_base_2()
    ///         .to_string(),
    ///     "3.32192809488736234787031942949"
    /// );
    /// assert!(Float::from_signed_prec(-10, 100).0.log_base_2().is_nan());
    /// ```
    #[inline]
    fn log_base_2(self) -> Self {
        let prec = self.significant_bits();
        self.log_base_2_prec_round(prec, Nearest).0
    }
}

impl LogBase2 for &Float {
    type Output = Float;

    /// Computes $\log_2 x$, where $x$ is a [`Float`], taking it by reference.
    ///
    /// If the output has a precision, it is the precision of the input. If the logarithm is
    /// equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s in
    /// its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// The base-2 logarithm of any nonzero negative number is `NaN`.
    ///
    /// $$
    /// f(x) = \log_2 x+\varepsilon.
    /// $$
    /// - If $\log_2 x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\log_2 x$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2 |\log_2
    ///   x|\rfloor-p}$, where $p$ is the precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN})=\text{NaN}$
    /// - $f(\infty)=\infty$
    /// - $f(-\infty)=\text{NaN}$
    /// - $f(\pm0.0)=-\infty$
    /// - $f(1.0)=0.0$, and the result is exact
    /// - $f(2^k)=k$, rounded to the precision of the input; the result is exact if and only if $k$
    ///   is representable with that precision
    /// - $f(x)=\text{NaN}$ for $x<0$
    ///
    /// Neither overflow nor underflow is possible.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::log_base_2_round_ref`] instead. If you want to specify the output precision,
    /// consider using [`Float::log_base_2_prec_ref`]. If you want both of these things, consider
    /// using [`Float::log_base_2_prec_round_ref`].
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
    /// use malachite_base::num::arithmetic::traits::LogBase2;
    /// use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity};
    /// use malachite_float::Float;
    ///
    /// assert!((&Float::NAN).log_base_2().is_nan());
    /// assert_eq!((&Float::INFINITY).log_base_2(), Float::INFINITY);
    /// assert!((&Float::NEGATIVE_INFINITY).log_base_2().is_nan());
    /// assert_eq!(
    ///     (&Float::from_unsigned_prec(10u32, 100).0)
    ///         .log_base_2()
    ///         .to_string(),
    ///     "3.32192809488736234787031942949"
    /// );
    /// assert!((&Float::from_signed_prec(-10, 100).0).log_base_2().is_nan());
    /// ```
    #[inline]
    fn log_base_2(self) -> Float {
        let prec = self.significant_bits();
        self.log_base_2_prec_round_ref(prec, Nearest).0
    }
}

impl LogBase2Assign for Float {
    /// Computes $\log_2 x$, where $x$ is a [`Float`], in place.
    ///
    /// If the output has a precision, it is the precision of the input. If the logarithm is
    /// equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s in
    /// its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// The base-2 logarithm of any nonzero negative number is `NaN`.
    ///
    /// $$
    /// x \gets \log_2 x+\varepsilon.
    /// $$
    /// - If $\log_2 x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\log_2 x$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2 |\log_2
    ///   x|\rfloor-p}$, where $p$ is the precision of the input.
    ///
    /// See the [`Float::log_base_2`] documentation for information on special cases, overflow, and
    /// underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::log_base_2_round_assign`] instead. If you want to specify the output precision,
    /// consider using [`Float::log_base_2_prec_assign`]. If you want both of these things, consider
    /// using [`Float::log_base_2_prec_round_assign`].
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
    /// use malachite_base::num::arithmetic::traits::LogBase2Assign;
    /// use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity};
    /// use malachite_float::Float;
    ///
    /// let mut x = Float::NAN;
    /// x.log_base_2_assign();
    /// assert!(x.is_nan());
    ///
    /// let mut x = Float::INFINITY;
    /// x.log_base_2_assign();
    /// assert_eq!(x, Float::INFINITY);
    ///
    /// let mut x = Float::NEGATIVE_INFINITY;
    /// x.log_base_2_assign();
    /// assert!(x.is_nan());
    ///
    /// let mut x = Float::from_unsigned_prec(10u32, 100).0;
    /// x.log_base_2_assign();
    /// assert_eq!(x.to_string(), "3.32192809488736234787031942949");
    ///
    /// let mut x = Float::from_signed_prec(-10, 100).0;
    /// x.log_base_2_assign();
    /// assert!(x.is_nan());
    /// ```
    #[inline]
    fn log_base_2_assign(&mut self) {
        let prec = self.significant_bits();
        self.log_base_2_prec_round_assign(prec, Nearest);
    }
}

/// Computes the base-2 logarithm of a primitive float, $\log_2 x$.
///
/// This function is correctly rounded. The standard library's `log2` is correctly rounded for
/// [`f32`] but not always for [`f64`], so for some [`f64`] inputs this function is more accurate.
///
/// The base-2 logarithm of any nonzero negative number is `NaN`.
///
/// $$
/// f(x) = \log_2 x+\varepsilon.
/// $$
/// - If $\log_2 x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
/// - If $\log_2 x$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2 |\log_2
///   x|\rfloor-p}$, where $p$ is precision of the output (typically 24 if `T` is a [`f32`] and 53
///   if `T` is a [`f64`], but less if the output is subnormal).
///
/// Special cases:
/// - $f(\text{NaN})=\text{NaN}$
/// - $f(\infty)=\infty$
/// - $f(-\infty)=\text{NaN}$
/// - $f(\pm0.0)=-\infty$
/// - $f(x)=\text{NaN}$ for $x<0$
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
/// use malachite_float::arithmetic::log_base_2::primitive_float_log_base_2;
///
/// assert!(primitive_float_log_base_2(f32::NAN).is_nan());
/// assert_eq!(
///     NiceFloat(primitive_float_log_base_2(f32::INFINITY)),
///     NiceFloat(f32::INFINITY)
/// );
/// assert!(primitive_float_log_base_2(f32::NEGATIVE_INFINITY).is_nan());
/// assert_eq!(
///     NiceFloat(primitive_float_log_base_2(0.0f32)),
///     NiceFloat(f32::NEGATIVE_INFINITY)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_log_base_2(8.0f32)),
///     NiceFloat(3.0)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_log_base_2(10.0f32)),
///     NiceFloat(3.321928)
/// );
/// assert!(primitive_float_log_base_2(-10.0f32).is_nan());
/// ```
#[inline]
#[allow(clippy::type_repetition_in_bounds)]
pub fn primitive_float_log_base_2<T: PrimitiveFloat>(x: T) -> T
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    emulate_float_to_float_fn(Float::log_base_2_prec, x)
}

/// Computes the base-2 logarithm of a [`Rational`], returning a primitive float result.
///
/// If the logarithm is equidistant from two primitive floats, the primitive float with fewer 1s in
/// its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest` rounding
/// mode.
///
/// The logarithm of any negative number is `NaN`.
///
/// $$
/// f(x) = \log_2{x}+\varepsilon.
/// $$
/// - If $\log_2{x}$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
/// - If $\log_2{x}$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2
///   |\log_2{x}|\rfloor-p}$, where $p$ is precision of the output (typically 24 if `T` is a [`f32`]
///   and 53 if `T` is a [`f64`], but less if the output is subnormal).
///
/// Special cases:
/// - $f(0)=-\infty$
///
/// Neither overflow nor underflow is possible.
///
/// # Worst-case complexity
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use malachite_base::num::basic::traits::{NegativeInfinity, Zero};
/// use malachite_base::num::float::NiceFloat;
/// use malachite_float::arithmetic::log_base_2::primitive_float_log_base_2_rational;
/// use malachite_q::Rational;
///
/// assert_eq!(
///     NiceFloat(primitive_float_log_base_2_rational::<f64>(&Rational::ZERO)),
///     NiceFloat(f64::NEGATIVE_INFINITY)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_log_base_2_rational::<f64>(
///         &Rational::from_unsigneds(1u8, 3)
///     )),
///     NiceFloat(-1.584962500721156)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_log_base_2_rational::<f64>(&Rational::from(
///         10000
///     ))),
///     NiceFloat(13.287712379549449)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_log_base_2_rational::<f64>(&Rational::from(
///         -10000
///     ))),
///     NiceFloat(f64::NAN)
/// );
/// ```
#[inline]
#[allow(clippy::type_repetition_in_bounds)]
pub fn primitive_float_log_base_2_rational<T: PrimitiveFloat>(x: &Rational) -> T
where
    Float: PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    emulate_rational_to_float_fn(Float::log_base_2_rational_prec_ref, x)
}
