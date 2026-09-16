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

use crate::Float;
use crate::InnerFloat::{Finite, Infinity, NaN, Zero};
use crate::float::arithmetic::atan::{arc_with_period_scale, scaled_unsigned};
use crate::float::arithmetic::round_near_x::small_input_shortcut;
use crate::float::arithmetic::sin::{SCALE, SCALED_INPUT_EXPONENT, scaled_underflow};
use crate::{emulate_float_to_float_fn, emulate_rational_to_float_fn};
use core::cmp::Ordering::{self, Equal, Greater, Less};
use malachite_base::num::arithmetic::traits::{CeilingLogBase2, IsPowerOf2, Square};
use malachite_base::num::basic::traits::Zero as ZeroTrait;
use malachite_base::num::comparison::traits::PartialOrdAbs;
use malachite_q::Rational;

use malachite_base::num::arithmetic::traits::{Abs, Asin, AsinAssign};
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::{NaN as NaNTrait, NegativeZero, One};
use malachite_base::num::conversion::traits::{ExactFrom, RoundingFrom};
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{self, Exact, Floor, Nearest, Up};
use malachite_nz::natural::arithmetic::float::round::float_can_round;
use malachite_nz::platform::Limb;

// Computes asin(x) for a finite nonzero `Float` x, rounded to precision `prec` with rounding mode
// `rm`.
//
// This is mpfr_asin from asin.c, MPFR 4.2.2, for a finite nonzero input.
fn asin_prec_round_normal_ref(x: &Float, prec: u64, rm: RoundingMode) -> (Float, Ordering) {
    let exp_x = i64::from(x.get_exponent().unwrap());
    // asin(x) = x + x^3/6 + ..., so the correction is below 2^(3 EXP(x) - 2) and carries the value
    // away from zero
    if let Some(result) = small_input_shortcut(x, -(exp_x << 1), 2, true, prec, rm) {
        return result;
    }
    let xp = x.abs();
    match xp.partial_cmp(&1u32).unwrap() {
        // asin(x) = NaN for |x| > 1
        Greater => (Float::NAN, Equal),
        // asin(1) = pi/2, asin(-1) = -pi/2
        Equal => {
            assert_ne!(rm, Exact, "Inexact asin");
            let negative = *x < 0u32;
            let (pi, o) = Float::pi_prec_round(prec, if negative { -rm } else { rm });
            // exact
            let half = pi >> 1u32;
            if negative {
                (-half, o.reverse())
            } else {
                (half, o)
            }
        }
        Less => {
            assert_ne!(rm, Exact, "Inexact asin");
            // The subtraction 1 - x^2 below cancels away about -EXP(1 - |x|) bits, since x^2 is
            // that close to 1. Both the working precision and the slack in the rounding test have
            // to cover that loss, so it is measured once here, from 1 - |x| rounded down.
            let p = x.get_prec().unwrap();
            let one_minus = Float::one_prec(p).sub_prec_round(xp, p, Floor).0;
            let cancel = u64::exact_from(2 - i64::from(one_minus.get_exponent().unwrap()));
            let mut w = prec + 10 + cancel;
            let mut increment = Limb::WIDTH;
            loop {
                // asin(x) = atan(x/sqrt(1 - x^2))
                let t = Float::ONE
                    .sub_prec_ref_val(x.square_prec_ref(w).0, w)
                    .0
                    .sqrt_prec(w)
                    .0;
                let t = x.div_prec_ref_val(t, w).0.atan_prec(w).0;
                if w > cancel && float_can_round(t.significand_ref().unwrap(), w - cancel, prec, rm)
                {
                    return Float::from_float_prec_round(t, prec, rm);
                }
                w += increment;
                increment = w >> 1;
            }
        }
    }
}

// Computes asin(x) u/(2 pi) for a finite nonzero `Float` x with |x| <= 1 and a nonzero u, rounded
// to precision `prec` with rounding mode `rm`. `rm` may be `Exact` only for |x| = 1, where the
// result is u/4, and for |x| = 1/2 with u a multiple of 3, where it is u/12.
//
// This is mpfr_asinu from asinu.c, MPFR 4.2.2. The quotient is formed with the numerator scaled up
// by 2^SCALE, since asin(x) u/(2 pi) can fall below the smallest positive `Float` for a tiny x and
// a small u, which MPFR's wider exponent range never sees; a result below it is then decided by the
// rounding mode alone, as in `sin_with_period`.
fn asin_with_period_prec_round_normal_ref(
    x: &Float,
    u: u64,
    prec: u64,
    rm: RoundingMode,
) -> (Float, Ordering) {
    let positive = *x > 0u32;
    let exp_x = i64::from(x.get_exponent().unwrap());
    let power_of_2 = x.significand_ref().unwrap().is_power_of_2();
    // |x| = 1: asinu(1, u) = u/4, asinu(-1, u) = -u/4, both exact
    if exp_x == 1 && power_of_2 {
        return scaled_unsigned(u, 2, positive, prec, rm);
    }
    // asin(+-1/2) = +-pi/6, so asinu(+-1/2, u) = +-u/12 is exact when u is a multiple of 3
    if exp_x == 0 && power_of_2 && u.is_multiple_of(3) {
        return scaled_unsigned(u / 3, 2, positive, prec, rm);
    }
    // Nothing else can be rounded exactly
    assert_ne!(rm, Exact, "Inexact asin_with_period");
    arc_with_period_scale(
        // scaling by a power of 2 is exact, and asin(x) u 2^SCALE stays far below the top of the
        // range, since |asin x| <= pi/2 and u < 2^64
        |w| x.asin_prec_round_ref(w, Up).0 << SCALE,
        u,
        positive,
        prec,
        rm,
    )
}

// Computes asin(x) for a nonzero `Rational` x with |x| < 1, rounded to precision `prec` with
// rounding mode `rm`. (The rest is handled by the caller.)
//
// MPFR has no arcsine of a rational. Its `Float` algorithm takes atan(x/sqrt(1 - x^2)) and pays for
// the cancellation in 1 - x^2 with extra working precision; here that subtraction is exact, so the
// identity is used in the form
//
//     asin(x) = sign(x) atan(sqrt(x^2/(1 - x^2))),
//
// whose argument is an exact `Rational`. Nothing cancels, and the input needs no rounding at all,
// which matters because the arcsine is not 1-Lipschitz: its derivative grows without bound toward
// +-1, so rounding the input first -- the approach `atan_rational` can afford -- would cost about
// half the cancelled bits.
//
// The errors that remain do not compound: the square root is correctly rounded, and the arctangent
// neither amplifies a relative error (q/((1 + q^2) atan q) <= 1 for every positive q) nor adds more
// than its own half ulp.
pub(crate) fn asin_rational_helper(x: &Rational, prec: u64, rm: RoundingMode) -> (Float, Ordering) {
    assert_ne!(rm, Exact, "Inexact asin_rational");
    let positive = *x > 0u32;
    let exp_x = x.floor_log_base_2_abs() + 1;
    // asin(x) = x(1 + x^2/6 + ...), so for an x at or below the bottom of the exponent range the
    // correction is below 2^(2 SCALED_INPUT_EXPONENT) and invisible at any working precision the
    // loop can reach: the answer is x itself, rounded. It is formed scaled up by 2^SCALE, since a
    // `Rational` can sit far below the smallest positive `Float` and the general path's own
    // rounding would collapse to zero there, leaving the loop below a value it can never certify.
    if exp_x <= SCALED_INPUT_EXPONENT {
        let scaled = x << SCALE;
        let mut w = prec + prec.ceiling_log_base_2() + 10;
        let mut increment = Limb::WIDTH;
        loop {
            // rounded away from zero, which is the side asin(x) lies on
            let t = Float::from_rational_prec_round_ref(&scaled, w, Up).0;
            if let Some(result) = scaled_underflow(&t, positive, prec, rm) {
                return result;
            }
            let t = t >> SCALE;
            if float_can_round(t.significand_ref().unwrap(), w - 2, prec, rm) {
                return Float::from_float_prec_round(t, prec, rm);
            }
            w += increment;
            increment = w >> 1;
        }
    }
    let x2 = (&x.abs()).square();
    let r = (&x2 / (Rational::ONE - &x2)).abs();
    let mut w = prec + prec.ceiling_log_base_2() + 10;
    let mut increment = Limb::WIDTH;
    loop {
        let t = Float::sqrt_rational_prec_ref(&r, w).0.atan_prec(w).0;
        if float_can_round(t.significand_ref().unwrap(), w - 3, prec, rm) {
            return Float::from_float_prec_round(if positive { t } else { -t }, prec, rm);
        }
        w += increment;
        increment = w >> 1;
    }
}

// Computes asin(x) u/(2 pi) for a nonzero `Rational` x with |x| <= 1 and a nonzero u, rounded to
// precision `prec` with rounding mode `rm`. (x = 0, u = 0, and |x| > 1 are handled by the caller.)
// `rm` may be `Exact` only for |x| = 1, where the result is u/4, and for |x| = 1/2 with u a
// multiple of 3, where it is u/12.
//
// MPFR has no arcsine of a rational. The branches match the `Float` case, with one addition: an x
// below the bottom of the exponent range is not a `Float`, but its arcsine is its own leading term,
// so the quotient is formed from x itself. That substitution neglects a relative x^2/6, which for
// such an x is below 2^(2 SCALED_INPUT_EXPONENT) and so far beneath any working precision the loop
// can reach. It is also needed rather than merely cheaper: `asin_rational_helper` reports such an x
// as an underflow, and a large u can lift the quotient back into the range, where that answer would
// be wrong.
pub(crate) fn asin_with_period_rational_helper(
    x: &Rational,
    u: u64,
    prec: u64,
    rm: RoundingMode,
) -> (Float, Ordering) {
    let positive = *x > 0u32;
    let exp_x = x.floor_log_base_2_abs() + 1; // the MPFR-style exponent of x
    // |x| = 1, since the caller has ruled out everything above it: asinu(1, u) = u/4 and asinu(-1,
    // u) = -u/4, both exact
    if exp_x == 1 {
        return scaled_unsigned(u, 2, positive, prec, rm);
    }
    // asin(+-1/2) = +-pi/6, so asinu(+-1/2, u) = +-u/12 is exact when u is a multiple of 3
    if u.is_multiple_of(3) && x.numerator_ref() == &1u32 && x.denominator_ref() == &2u32 {
        return scaled_unsigned(u / 3, 2, positive, prec, rm);
    }
    // Nothing else can be rounded exactly
    assert_ne!(rm, Exact, "Inexact asin_with_period_rational");
    if exp_x <= SCALED_INPUT_EXPONENT {
        let scaled = x << SCALE;
        return arc_with_period_scale(
            |w| Float::from_rational_prec_round_ref(&scaled, w, Up).0,
            u,
            positive,
            prec,
            rm,
        );
    }
    arc_with_period_scale(
        |w| asin_rational_helper(x, w, Up).0 << SCALE,
        u,
        positive,
        prec,
        rm,
    )
}

impl Float {
    /// Computes $\arcsin x$, the arcsine of a [`Float`], rounding the result to the specified
    /// precision and with the specified rounding mode. The [`Float`] is taken by value. An
    /// [`Ordering`] is also returned, indicating whether the rounded arcsine is less than, equal
    /// to, or greater than the exact arcsine. Although `NaN`s are not comparable to any [`Float`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \arcsin x+\varepsilon.
    /// $$
    /// - If $x$ is NaN, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x$ is not NaN and $m$ is not `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2
    ///   |\arcsin x|\rfloor-p+1}$.
    /// - If $x$ is not NaN and $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2 |\arcsin
    ///   x|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=f(\pm\infty,p,m)=\text{NaN}$
    /// - $f(x,p,m)=\text{NaN}$ for $|x|>1$
    /// - $f(\pm0.0,p,m)=\pm0.0$
    /// - $f(\pm1,p,m)=\pm\pi/2$, rounded
    ///
    /// Neither overflow nor underflow is possible: the result lies in $[-\pi/2, \pi/2]$, and
    /// $|\arcsin x| > |x|$ for nonzero $x$, so a representable input always has a representable
    /// result.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::asin_prec`] instead. If you
    /// know that your target precision is the precision of the input, consider using
    /// [`Float::asin_round`] instead. If both of these things are true, consider using
    /// [`Float::asin`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O((n+m) (\log (n+m))^3 \log\log (n+m))$
    ///
    /// $M(n, m) = O((n+m) \log (n+m))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `self.significant_bits()`: the arcsine is taken as $\arctan(x/\sqrt{1-x^2})$ at a working
    /// precision of about $n$ plus the number of bits that cancel in $1-x^2$, which an input within
    /// $2^{-m}$ of $\pm1$ pushes to $m$; the arctangent at that width dominates. The magnitude of
    /// the input does not otherwise drive the cost.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` and `self` is nonzero and not NaN, since the arcsine of a finite
    /// nonzero [`Float`] is never exactly representable and neither is $\pm\pi/2$, or if `prec` is
    /// zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .asin_prec_round(5, Floor);
    /// assert_eq!(c.to_string(), "1.56");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .asin_prec_round(5, Ceiling);
    /// assert_eq!(c.to_string(), "1.62");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .asin_prec_round(5, Nearest);
    /// assert_eq!(c.to_string(), "1.56");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .asin_prec_round(20, Floor);
    /// assert_eq!(c.to_string(), "1.5707951");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .asin_prec_round(20, Ceiling);
    /// assert_eq!(c.to_string(), "1.5707970");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .asin_prec_round(20, Nearest);
    /// assert_eq!(c.to_string(), "1.5707970");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn asin_prec_round(self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        self.asin_prec_round_ref(prec, rm)
    }

    /// Computes $\arcsin x$, the arcsine of a [`Float`], rounding the result to the specified
    /// precision and with the specified rounding mode. The [`Float`] is taken by reference. An
    /// [`Ordering`] is also returned, indicating whether the rounded arcsine is less than, equal
    /// to, or greater than the exact arcsine. Although `NaN`s are not comparable to any [`Float`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \arcsin x+\varepsilon.
    /// $$
    /// - If $x$ is NaN, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x$ is not NaN and $m$ is not `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2
    ///   |\arcsin x|\rfloor-p+1}$.
    /// - If $x$ is not NaN and $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2 |\arcsin
    ///   x|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=f(\pm\infty,p,m)=\text{NaN}$
    /// - $f(x,p,m)=\text{NaN}$ for $|x|>1$
    /// - $f(\pm0.0,p,m)=\pm0.0$
    /// - $f(\pm1,p,m)=\pm\pi/2$, rounded
    ///
    /// Neither overflow nor underflow is possible: the result lies in $[-\pi/2, \pi/2]$, and
    /// $|\arcsin x| > |x|$ for nonzero $x$, so a representable input always has a representable
    /// result.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::asin_prec_ref`] instead. If
    /// you know that your target precision is the precision of the input, consider using
    /// [`Float::asin_round_ref`] instead. If both of these things are true, consider using
    /// `(&Float).asin()` instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O((n+m) (\log (n+m))^3 \log\log (n+m))$
    ///
    /// $M(n, m) = O((n+m) \log (n+m))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `self.significant_bits()`: the arcsine is taken as $\arctan(x/\sqrt{1-x^2})$ at a working
    /// precision of about $n$ plus the number of bits that cancel in $1-x^2$, which an input within
    /// $2^{-m}$ of $\pm1$ pushes to $m$; the arctangent at that width dominates. The magnitude of
    /// the input does not otherwise drive the cost.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` and `self` is nonzero and not NaN, since the arcsine of a finite
    /// nonzero [`Float`] is never exactly representable and neither is $\pm\pi/2$, or if `prec` is
    /// zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = (&Float::from_unsigned_prec(1u32, 100).0).asin_prec_round_ref(5, Floor);
    /// assert_eq!(c.to_string(), "1.56");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = (&Float::from_unsigned_prec(1u32, 100).0).asin_prec_round_ref(5, Ceiling);
    /// assert_eq!(c.to_string(), "1.62");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = (&Float::from_unsigned_prec(1u32, 100).0).asin_prec_round_ref(5, Nearest);
    /// assert_eq!(c.to_string(), "1.56");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = (&Float::from_unsigned_prec(1u32, 100).0).asin_prec_round_ref(20, Floor);
    /// assert_eq!(c.to_string(), "1.5707951");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = (&Float::from_unsigned_prec(1u32, 100).0).asin_prec_round_ref(20, Ceiling);
    /// assert_eq!(c.to_string(), "1.5707970");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = (&Float::from_unsigned_prec(1u32, 100).0).asin_prec_round_ref(20, Nearest);
    /// assert_eq!(c.to_string(), "1.5707970");
    /// assert_eq!(o, Greater);
    /// ```
    pub fn asin_prec_round_ref(&self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        assert_ne!(prec, 0);
        match &self.0 {
            // the arcsine is NaN outside [-1, 1], and both infinities are outside it
            NaN | Infinity { .. } => (Self::NAN, Equal),
            // asin(+0.0) = +0.0, asin(-0.0) = -0.0
            Zero { .. } => (self.clone(), Equal),
            Finite { .. } => asin_prec_round_normal_ref(self, prec, rm),
        }
    }

    /// Computes $\arcsin x$, the arcsine of a [`Float`], rounding the result to the nearest value
    /// of the specified precision. The [`Float`] is taken by value. An [`Ordering`] is also
    /// returned, indicating whether the rounded arcsine is less than, equal to, or greater than the
    /// exact arcsine. Although `NaN`s are not comparable to any [`Float`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// If the arcsine is equidistant from two [`Float`]s with the specified precision, the
    /// [`Float`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a
    /// description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = \arcsin x+\varepsilon.
    /// $$
    /// - If $x$ is NaN, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x$ is not NaN, then $|\varepsilon| < 2^{\lfloor\log_2 |\arcsin x|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=f(\pm\infty,p,m)=\text{NaN}$
    /// - $f(x,p,m)=\text{NaN}$ for $|x|>1$
    /// - $f(\pm0.0,p,m)=\pm0.0$
    /// - $f(\pm1,p,m)=\pm\pi/2$, rounded
    ///
    /// Neither overflow nor underflow is possible: the result lies in $[-\pi/2, \pi/2]$, and
    /// $|\arcsin x| > |x|$ for nonzero $x$, so a representable input always has a representable
    /// result.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::asin_prec_round`] instead. If you know that your target precision is the precision
    /// of the input, consider using [`Float::asin`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O((n+m) (\log (n+m))^3 \log\log (n+m))$
    ///
    /// $M(n, m) = O((n+m) \log (n+m))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `self.significant_bits()`: the arcsine is taken as $\arctan(x/\sqrt{1-x^2})$ at a working
    /// precision of about $n$ plus the number of bits that cancel in $1-x^2$, which an input within
    /// $2^{-m}$ of $\pm1$ pushes to $m$; the arctangent at that width dominates. The magnitude of
    /// the input does not otherwise drive the cost.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100).0.asin_prec(5);
    /// assert_eq!(c.to_string(), "1.56");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100).0.asin_prec(20);
    /// assert_eq!(c.to_string(), "1.5707970");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn asin_prec(self, prec: u64) -> (Self, Ordering) {
        self.asin_prec_round(prec, Nearest)
    }

    /// Computes $\arcsin x$, the arcsine of a [`Float`], rounding the result to the nearest value
    /// of the specified precision. The [`Float`] is taken by reference. An [`Ordering`] is also
    /// returned, indicating whether the rounded arcsine is less than, equal to, or greater than the
    /// exact arcsine. Although `NaN`s are not comparable to any [`Float`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// If the arcsine is equidistant from two [`Float`]s with the specified precision, the
    /// [`Float`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a
    /// description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = \arcsin x+\varepsilon.
    /// $$
    /// - If $x$ is NaN, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x$ is not NaN, then $|\varepsilon| < 2^{\lfloor\log_2 |\arcsin x|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=f(\pm\infty,p,m)=\text{NaN}$
    /// - $f(x,p,m)=\text{NaN}$ for $|x|>1$
    /// - $f(\pm0.0,p,m)=\pm0.0$
    /// - $f(\pm1,p,m)=\pm\pi/2$, rounded
    ///
    /// Neither overflow nor underflow is possible: the result lies in $[-\pi/2, \pi/2]$, and
    /// $|\arcsin x| > |x|$ for nonzero $x$, so a representable input always has a representable
    /// result.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::asin_prec_round_ref`] instead. If you know that your target precision is the
    /// precision of the input, consider using `(&Float).asin()` instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O((n+m) (\log (n+m))^3 \log\log (n+m))$
    ///
    /// $M(n, m) = O((n+m) \log (n+m))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `self.significant_bits()`: the arcsine is taken as $\arctan(x/\sqrt{1-x^2})$ at a working
    /// precision of about $n$ plus the number of bits that cancel in $1-x^2$, which an input within
    /// $2^{-m}$ of $\pm1$ pushes to $m$; the arctangent at that width dominates. The magnitude of
    /// the input does not otherwise drive the cost.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = (&Float::from_unsigned_prec(1u32, 100).0).asin_prec_ref(5);
    /// assert_eq!(c.to_string(), "1.56");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = (&Float::from_unsigned_prec(1u32, 100).0).asin_prec_ref(20);
    /// assert_eq!(c.to_string(), "1.5707970");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn asin_prec_ref(&self, prec: u64) -> (Self, Ordering) {
        self.asin_prec_round_ref(prec, Nearest)
    }

    /// Computes $\arcsin x$, the arcsine of a [`Float`], rounding the result with the specified
    /// rounding mode. The [`Float`] is taken by value. An [`Ordering`] is also returned, indicating
    /// whether the rounded arcsine is less than, equal to, or greater than the exact arcsine.
    /// Although `NaN`s are not comparable to any [`Float`], whenever this function returns a `NaN`
    /// it also returns `Equal`.
    ///
    /// The precision of the output is the precision of the input. See [`RoundingMode`] for a
    /// description of the possible rounding modes.
    ///
    /// $$
    /// f(x,m) = \arcsin x+\varepsilon.
    /// $$
    /// - If $x$ is NaN, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x$ is not NaN and $m$ is not `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2
    ///   |\arcsin x|\rfloor-p+1}$, where $p$ is the precision of the input.
    /// - If $x$ is not NaN and $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2 |\arcsin
    ///   x|\rfloor-p}$, where $p$ is the precision of the input.
    ///
    /// If the output has a precision, it is the precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=f(\pm\infty,p,m)=\text{NaN}$
    /// - $f(x,p,m)=\text{NaN}$ for $|x|>1$
    /// - $f(\pm0.0,p,m)=\pm0.0$
    /// - $f(\pm1,p,m)=\pm\pi/2$, rounded
    ///
    /// Neither overflow nor underflow is possible: the result lies in $[-\pi/2, \pi/2]$, and
    /// $|\arcsin x| > |x|$ for nonzero $x$, so a representable input always has a representable
    /// result.
    ///
    /// If you want to specify an output precision, consider using [`Float::asin_prec_round`]
    /// instead. If you know you'll be using the `Nearest` rounding mode, consider using
    /// [`Float::asin`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^3 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`: the
    /// arcsine is taken as $\arctan(x/\sqrt{1-x^2})$ at a working precision of about $n$ plus the
    /// number of bits that cancel in $1-x^2$, which an input within $2^{-n}$ of $\pm1$ pushes to
    /// another $n$; the arctangent at that width dominates. The magnitude of the input does not
    /// otherwise drive the cost.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` and `self` is nonzero and not NaN, since the arcsine of a finite
    /// nonzero [`Float`] is never exactly representable and neither is $\pm\pi/2$.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100).0.asin_round(Floor);
    /// assert_eq!(c.to_string(), "1.5707963267948966192313216916397");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100).0.asin_round(Ceiling);
    /// assert_eq!(c.to_string(), "1.5707963267948966192313216916412");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100).0.asin_round(Nearest);
    /// assert_eq!(c.to_string(), "1.5707963267948966192313216916397");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn asin_round(self, rm: RoundingMode) -> (Self, Ordering) {
        let prec = self.significant_bits();
        self.asin_prec_round(prec, rm)
    }

    /// Computes $\arcsin x$, the arcsine of a [`Float`], rounding the result with the specified
    /// rounding mode. The [`Float`] is taken by reference. An [`Ordering`] is also returned,
    /// indicating whether the rounded arcsine is less than, equal to, or greater than the exact
    /// arcsine. Although `NaN`s are not comparable to any [`Float`], whenever this function returns
    /// a `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the precision of the input. See [`RoundingMode`] for a
    /// description of the possible rounding modes.
    ///
    /// $$
    /// f(x,m) = \arcsin x+\varepsilon.
    /// $$
    /// - If $x$ is NaN, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x$ is not NaN and $m$ is not `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2
    ///   |\arcsin x|\rfloor-p+1}$, where $p$ is the precision of the input.
    /// - If $x$ is not NaN and $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2 |\arcsin
    ///   x|\rfloor-p}$, where $p$ is the precision of the input.
    ///
    /// If the output has a precision, it is the precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=f(\pm\infty,p,m)=\text{NaN}$
    /// - $f(x,p,m)=\text{NaN}$ for $|x|>1$
    /// - $f(\pm0.0,p,m)=\pm0.0$
    /// - $f(\pm1,p,m)=\pm\pi/2$, rounded
    ///
    /// Neither overflow nor underflow is possible: the result lies in $[-\pi/2, \pi/2]$, and
    /// $|\arcsin x| > |x|$ for nonzero $x$, so a representable input always has a representable
    /// result.
    ///
    /// If you want to specify an output precision, consider using [`Float::asin_prec_round_ref`]
    /// instead. If you know you'll be using the `Nearest` rounding mode, consider using
    /// `(&Float).asin()` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^3 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`: the
    /// arcsine is taken as $\arctan(x/\sqrt{1-x^2})$ at a working precision of about $n$ plus the
    /// number of bits that cancel in $1-x^2$, which an input within $2^{-n}$ of $\pm1$ pushes to
    /// another $n$; the arctangent at that width dominates. The magnitude of the input does not
    /// otherwise drive the cost.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` and `self` is nonzero and not NaN, since the arcsine of a finite
    /// nonzero [`Float`] is never exactly representable and neither is $\pm\pi/2$.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = (&Float::from_unsigned_prec(1u32, 100).0).asin_round_ref(Floor);
    /// assert_eq!(c.to_string(), "1.5707963267948966192313216916397");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = (&Float::from_unsigned_prec(1u32, 100).0).asin_round_ref(Ceiling);
    /// assert_eq!(c.to_string(), "1.5707963267948966192313216916412");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = (&Float::from_unsigned_prec(1u32, 100).0).asin_round_ref(Nearest);
    /// assert_eq!(c.to_string(), "1.5707963267948966192313216916397");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn asin_round_ref(&self, rm: RoundingMode) -> (Self, Ordering) {
        self.asin_prec_round_ref(self.significant_bits(), rm)
    }

    /// Computes $\arcsin x$, the arcsine of a [`Float`], rounding the result to the specified
    /// precision and with the specified rounding mode. The [`Float`] is replaced by the result, and
    /// an [`Ordering`] is returned, indicating whether the rounded arcsine is less than, equal to,
    /// or greater than the exact arcsine. Although `NaN`s are not comparable to any [`Float`],
    /// whenever this function sets a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets \arcsin x+\varepsilon.
    /// $$
    /// - If $x$ is NaN, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x$ is not NaN and $m$ is not `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2
    ///   |\arcsin x|\rfloor-p+1}$.
    /// - If $x$ is not NaN and $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2 |\arcsin
    ///   x|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// See the [`Float::asin_prec_round`] documentation for information on the special cases.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::asin_prec_assign`] instead.
    /// If you know that your target precision is the precision of the input, consider using
    /// [`Float::asin_round_assign`] instead. If both of these things are true, consider using
    /// [`Float::asin_assign`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O((n+m) (\log (n+m))^3 \log\log (n+m))$
    ///
    /// $M(n, m) = O((n+m) \log (n+m))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `self.significant_bits()`: the arcsine is taken as $\arctan(x/\sqrt{1-x^2})$ at a working
    /// precision of about $n$ plus the number of bits that cancel in $1-x^2$, which an input within
    /// $2^{-m}$ of $\pm1$ pushes to $m$; the arctangent at that width dominates. The magnitude of
    /// the input does not otherwise drive the cost.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` and `self` is nonzero and not NaN, since the arcsine of a finite
    /// nonzero [`Float`] is never exactly representable and neither is $\pm\pi/2$, or if `prec` is
    /// zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.asin_prec_round_assign(5, Floor), Less);
    /// assert_eq!(x.to_string(), "1.56");
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.asin_prec_round_assign(5, Ceiling), Greater);
    /// assert_eq!(x.to_string(), "1.62");
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.asin_prec_round_assign(5, Nearest), Less);
    /// assert_eq!(x.to_string(), "1.56");
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.asin_prec_round_assign(20, Floor), Less);
    /// assert_eq!(x.to_string(), "1.5707951");
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.asin_prec_round_assign(20, Ceiling), Greater);
    /// assert_eq!(x.to_string(), "1.5707970");
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.asin_prec_round_assign(20, Nearest), Greater);
    /// assert_eq!(x.to_string(), "1.5707970");
    /// ```
    #[inline]
    pub fn asin_prec_round_assign(&mut self, prec: u64, rm: RoundingMode) -> Ordering {
        let o;
        (*self, o) = self.asin_prec_round_ref(prec, rm);
        o
    }

    /// Computes $\arcsin x$, the arcsine of a [`Float`], rounding the result to the nearest value
    /// of the specified precision. The [`Float`] is replaced by the result, and an [`Ordering`] is
    /// returned, indicating whether the rounded arcsine is less than, equal to, or greater than the
    /// exact arcsine. Although `NaN`s are not comparable to any [`Float`], whenever this function
    /// sets a `NaN` it also returns `Equal`.
    ///
    /// If the arcsine is equidistant from two [`Float`]s with the specified precision, the
    /// [`Float`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a
    /// description of the `Nearest` rounding mode.
    ///
    /// $$
    /// x \gets \arcsin x+\varepsilon.
    /// $$
    /// - If $x$ is NaN, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x$ is not NaN, then $|\varepsilon| < 2^{\lfloor\log_2 |\arcsin x|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// See the [`Float::asin_prec`] documentation for information on the special cases.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::asin_prec_round_assign`] instead. If you know that your target precision is the
    /// precision of the input, consider using [`Float::asin_assign`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O((n+m) (\log (n+m))^3 \log\log (n+m))$
    ///
    /// $M(n, m) = O((n+m) \log (n+m))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `self.significant_bits()`: the arcsine is taken as $\arctan(x/\sqrt{1-x^2})$ at a working
    /// precision of about $n$ plus the number of bits that cancel in $1-x^2$, which an input within
    /// $2^{-m}$ of $\pm1$ pushes to $m$; the arctangent at that width dominates. The magnitude of
    /// the input does not otherwise drive the cost.
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
    /// assert_eq!(x.asin_prec_assign(5), Less);
    /// assert_eq!(x.to_string(), "1.56");
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.asin_prec_assign(20), Greater);
    /// assert_eq!(x.to_string(), "1.5707970");
    /// ```
    #[inline]
    pub fn asin_prec_assign(&mut self, prec: u64) -> Ordering {
        self.asin_prec_round_assign(prec, Nearest)
    }

    /// Computes $\arcsin x$, the arcsine of a [`Float`], rounding the result with the specified
    /// rounding mode. The [`Float`] is replaced by the result, and an [`Ordering`] is returned,
    /// indicating whether the rounded arcsine is less than, equal to, or greater than the exact
    /// arcsine. Although `NaN`s are not comparable to any [`Float`], whenever this function sets a
    /// `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the precision of the input. See [`RoundingMode`] for a
    /// description of the possible rounding modes.
    ///
    /// $$
    /// x \gets \arcsin x+\varepsilon.
    /// $$
    /// - If $x$ is NaN, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x$ is not NaN and $m$ is not `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2
    ///   |\arcsin x|\rfloor-p+1}$, where $p$ is the precision of the input.
    /// - If $x$ is not NaN and $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2 |\arcsin
    ///   x|\rfloor-p}$, where $p$ is the precision of the input.
    ///
    /// If the output has a precision, it is the precision of the input.
    ///
    /// See the [`Float::asin_round`] documentation for information on the special cases.
    ///
    /// If you want to specify an output precision, consider using [`Float::asin_prec_round_assign`]
    /// instead. If you know you'll be using the `Nearest` rounding mode, consider using
    /// [`Float::asin_assign`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^3 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`: the
    /// arcsine is taken as $\arctan(x/\sqrt{1-x^2})$ at a working precision of about $n$ plus the
    /// number of bits that cancel in $1-x^2$, which an input within $2^{-n}$ of $\pm1$ pushes to
    /// another $n$; the arctangent at that width dominates. The magnitude of the input does not
    /// otherwise drive the cost.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` and `self` is nonzero and not NaN, since the arcsine of a finite
    /// nonzero [`Float`] is never exactly representable and neither is $\pm\pi/2$.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.asin_round_assign(Floor), Less);
    /// assert_eq!(x.to_string(), "1.5707963267948966192313216916397");
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.asin_round_assign(Ceiling), Greater);
    /// assert_eq!(x.to_string(), "1.5707963267948966192313216916412");
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.asin_round_assign(Nearest), Less);
    /// assert_eq!(x.to_string(), "1.5707963267948966192313216916397");
    /// ```
    #[inline]
    pub fn asin_round_assign(&mut self, rm: RoundingMode) -> Ordering {
        let prec = self.significant_bits();
        self.asin_prec_round_assign(prec, rm)
    }

    /// Computes $\arcsin x$, the arcsine of a [`Rational`], rounding the result to the specified
    /// precision and with the specified rounding mode and returning the result as a [`Float`]. The
    /// [`Rational`] is taken by value. An [`Ordering`] is also returned, indicating whether the
    /// rounded arcsine is less than, equal to, or greater than the exact arcsine.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \arcsin x+\varepsilon.
    /// $$
    /// - If the result is NaN or zero, $\varepsilon$ may be ignored or assumed to be 0.
    /// - Otherwise, if $m$ is not `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2 |\arcsin
    ///   x|\rfloor-p+1}$.
    /// - Otherwise, if $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2 |\arcsin
    ///   x|\rfloor-p}$.
    ///
    /// The output has precision `prec`.
    ///
    /// Special cases:
    /// - $f(x,p,m)=\text{NaN}$ for $|x|>1$
    /// - $f(0,p,m)=0.0$
    /// - $f(\pm1,p,m)=\pm\pi/2$, rounded
    ///
    /// The zero and the NaNs are the only exact cases. A [`Rational`] has no signed zeros, so the
    /// zero result is positive.
    ///
    /// Overflow is not possible, since the result lies in $[-\pi/2, \pi/2]$. Underflow, which the
    /// [`Float`] arcsine cannot reach, is possible here: a [`Rational`] may lie far below the
    /// bottom of the exponent range, and there $\arcsin x$ is about $x$, so $0.0$ or
    /// $\pm2^{-2^{30}}$ is returned instead, by the rounding mode alone.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::asin_rational_prec`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `x.significant_bits()`: $x^2/(1-x^2)$ is formed exactly, and its square root and arctangent
    /// are taken at a working precision of about $n$ bits, which costs the first term; the second
    /// covers the $m$-bit input. The magnitude of the input does not drive the cost, and unlike the
    /// [`Float`] arcsine neither does its closeness to $\pm1$, since nothing cancels.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with the given precision (which is the case unless $x$ is zero or $|x|>1$).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (t, o) = Float::asin_rational_prec_round(Rational::from_unsigneds(3u8, 5), 10, Floor);
    /// assert_eq!(t.to_string(), "0.64258");
    /// assert_eq!(o, Less);
    ///
    /// let (t, o) = Float::asin_rational_prec_round(Rational::from_unsigneds(3u8, 5), 10, Ceiling);
    /// assert_eq!(t.to_string(), "0.64355");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    #[allow(clippy::needless_pass_by_value)]
    pub fn asin_rational_prec_round(x: Rational, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        Self::asin_rational_prec_round_ref(&x, prec, rm)
    }

    /// Computes $\arcsin x$, the arcsine of a [`Rational`], rounding the result to the specified
    /// precision and with the specified rounding mode and returning the result as a [`Float`]. The
    /// [`Rational`] is taken by reference. An [`Ordering`] is also returned, indicating whether the
    /// rounded arcsine is less than, equal to, or greater than the exact arcsine.
    ///
    /// See [`Float::asin_rational_prec_round`] for the error bounds, the special cases, underflow,
    /// and the complexity; this function behaves the same way.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with the given precision.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (t, o) =
    ///     Float::asin_rational_prec_round_ref(&Rational::from_unsigneds(3u8, 5), 20, Floor);
    /// assert_eq!(t.to_string(), "0.64350033");
    /// assert_eq!(o, Less);
    ///
    /// // an input of 1 is a quarter turn
    /// let (t, o) = Float::asin_rational_prec_round_ref(&Rational::ONE, 20, Floor);
    /// assert_eq!(t.to_string(), "1.5707951");
    /// assert_eq!(o, Less);
    /// ```
    pub fn asin_rational_prec_round_ref(
        x: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        assert_ne!(prec, 0);
        // asin(0) = 0, exactly (a `Rational` zero has no sign, so the result is positive)
        if *x == 0u32 {
            return (Self::ZERO, Equal);
        }
        match x.partial_cmp_abs(&1u32).unwrap() {
            // the arcsine is NaN outside [-1, 1]
            Greater => (Self::NAN, Equal),
            // asin(1) = pi/2, asin(-1) = -pi/2
            Equal => {
                assert_ne!(rm, Exact, "Inexact asin_rational");
                let negative = *x < 0u32;
                let (pi, o) = Self::pi_prec_round(prec, if negative { -rm } else { rm });
                // exact
                let half = pi >> 1u32;
                if negative {
                    (-half, o.reverse())
                } else {
                    (half, o)
                }
            }
            Less => asin_rational_helper(x, prec, rm),
        }
    }

    /// Computes $\arcsin x$, the arcsine of a [`Rational`], rounding the result to the nearest
    /// value of the specified precision and returning the result as a [`Float`]. The [`Rational`]
    /// is taken by value. An [`Ordering`] is also returned, indicating whether the rounded arcsine
    /// is less than, equal to, or greater than the exact arcsine.
    ///
    /// If the arcsine is equidistant from two [`Float`]s with the specified precision, the
    /// [`Float`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a
    /// description of the `Nearest` rounding mode.
    ///
    /// See [`Float::asin_rational_prec_round`] for the error bounds, the special cases, underflow,
    /// and the complexity; this function is that one with `Nearest`.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::asin_rational_prec_round`] instead.
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
    /// let (t, o) = Float::asin_rational_prec(Rational::from_unsigneds(3u8, 5), 10);
    /// assert_eq!(t.to_string(), "0.64355");
    /// assert_eq!(o, Greater);
    ///
    /// let (t, o) = Float::asin_rational_prec(Rational::from_unsigneds(3u8, 5), 53);
    /// assert_eq!(t.to_string(), "0.64350110879328437");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    #[allow(clippy::needless_pass_by_value)]
    pub fn asin_rational_prec(x: Rational, prec: u64) -> (Self, Ordering) {
        Self::asin_rational_prec_round_ref(&x, prec, Nearest)
    }

    /// Computes $\arcsin x$, the arcsine of a [`Rational`], rounding the result to the nearest
    /// value of the specified precision and returning the result as a [`Float`]. The [`Rational`]
    /// is taken by reference. An [`Ordering`] is also returned, indicating whether the rounded
    /// arcsine is less than, equal to, or greater than the exact arcsine.
    ///
    /// See [`Float::asin_rational_prec`] for the error bounds, the special cases, underflow, and
    /// the complexity; this function behaves the same way.
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
    /// let (t, o) = Float::asin_rational_prec_ref(&Rational::from_unsigneds(3u8, 5), 53);
    /// assert_eq!(t.to_string(), "0.64350110879328437");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn asin_rational_prec_ref(x: &Rational, prec: u64) -> (Self, Ordering) {
        Self::asin_rational_prec_round_ref(x, prec, Nearest)
    }
}

impl Float {
    /// Computes $\arcsin(x)u/(2\pi)$, the arcsine of a [`Float`] measured in $u$ths of a turn,
    /// rounding the result to the specified precision and with the specified rounding mode. The
    /// [`Float`] is taken by value. An [`Ordering`] is also returned, indicating whether the
    /// rounded arcsine is less than, equal to, or greater than the exact arcsine. Although `NaN`s
    /// are not comparable to any [`Float`], whenever this function returns a `NaN` it also returns
    /// `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,u,p,m) = \arcsin(x)u/(2\pi)+\varepsilon.
    /// $$
    /// - If $x$ is NaN or zero, $|x|>1$, $u = 0$, $|x|$ is 1, or $|x|$ is $1/2$ and $u$ is a
    ///   multiple of 3, $\varepsilon$ may be ignored or assumed to be 0.
    /// - Otherwise, if $m$ is not `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2
    ///   |\arcsin(x)u/(2\pi)|\rfloor-p+1}$.
    /// - Otherwise, if $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\arcsin(x)u/(2\pi)|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},u,p,m)=f(\pm\infty,u,p,m)=\text{NaN}$
    /// - $f(x,u,p,m)=\text{NaN}$ for $|x|>1$, including when $u=0$
    /// - $f(\pm0.0,u,p,m)=\pm0.0$
    /// - $f(x,0,p,m)=\pm0.0$, with the sign of $x$, so that the function stays odd
    /// - $f(\pm1,u,p,m)=\pm u/4$, a quarter turn
    /// - $f(\pm1/2,u,p,m)=\pm u/12$, a twelfth of a turn, when $u$ is a multiple of 3
    ///
    /// The last four are the only exact cases, and the quarter and twelfth turns are exact only
    /// when $p$ is large enough to hold them.
    ///
    /// Underflow:
    /// - If $0<f(x,u,p,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,u,p,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,u,p,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,u,p,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - The negative cases mirror these, since the function is odd.
    ///
    /// Overflow is not possible, since $|f(x,u,p,m)| \leq u/4 < 2^{62}$. Underflow requires a tiny
    /// $x$ together with a small $u$, since the result is about $xu/(2\pi)$ there.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::asin_with_period_prec`]
    /// instead. If you know that your target precision is the precision of the input, consider
    /// using [`Float::asin_with_period_round`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O((n+m) (\log (n+m))^3 \log\log (n+m))$
    ///
    /// $M(n, m) = O((n+m) \log (n+m))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `self.significant_bits()`: the arcsine is taken at a working precision of about $n$ plus the
    /// bits that cancel in $1-x^2$, which an input within $2^{-m}$ of $\pm1$ pushes to $m$, and is
    /// then scaled by $u/(2\pi)$, which needs $\pi$ to that many bits; the arcsine dominates.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with the given precision (which is the case unless $x$ is zero or NaN, $|x|>1$, $u$ is zero,
    /// or $p$ is large enough to hold the quarter or twelfth turn that $|x|=1$ or $|x|=1/2$ gives).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{One, OneHalf};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (t, o) = Float::ONE.asin_with_period_prec_round(360, 10, Exact);
    /// assert_eq!(t.to_string(), "90.000");
    /// assert_eq!(o, Equal);
    ///
    /// let (t, o) = Float::ONE_HALF.asin_with_period_prec_round(360, 10, Exact);
    /// assert_eq!(t.to_string(), "30.000");
    /// assert_eq!(o, Equal);
    ///
    /// let (t, o) = (Float::ONE_HALF >> 1u32).asin_with_period_prec_round(360, 10, Floor);
    /// assert_eq!(t.to_string(), "14.469");
    /// assert_eq!(o, Less);
    ///
    /// let (t, o) = (Float::ONE_HALF >> 1u32).asin_with_period_prec_round(360, 10, Ceiling);
    /// assert_eq!(t.to_string(), "14.484");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn asin_with_period_prec_round(
        self,
        u: u64,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        self.asin_with_period_prec_round_ref(u, prec, rm)
    }

    /// Computes $\arcsin(x)u/(2\pi)$, the arcsine of a [`Float`] measured in $u$ths of a turn,
    /// rounding the result to the specified precision and with the specified rounding mode. The
    /// [`Float`] is taken by reference. An [`Ordering`] is also returned, indicating whether the
    /// rounded arcsine is less than, equal to, or greater than the exact arcsine. Although `NaN`s
    /// are not comparable to any [`Float`], whenever this function returns a `NaN` it also returns
    /// `Equal`.
    ///
    /// See [`Float::asin_with_period_prec_round`] for the error bounds, the special and closed-form
    /// cases, underflow, and the complexity; this function behaves the same way.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with the given precision.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{One, OneHalf};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (t, o) = (&Float::ONE).asin_with_period_prec_round_ref(360, 10, Exact);
    /// assert_eq!(t.to_string(), "90.000");
    /// assert_eq!(o, Equal);
    ///
    /// let (t, o) = (&(Float::ONE_HALF >> 1u32)).asin_with_period_prec_round_ref(360, 10, Floor);
    /// assert_eq!(t.to_string(), "14.469");
    /// assert_eq!(o, Less);
    /// ```
    pub fn asin_with_period_prec_round_ref(
        &self,
        u: u64,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        assert_ne!(prec, 0);
        match &self.0 {
            // the arcsine is NaN outside [-1, 1], and both infinities are outside it; this holds
            // for u = 0 too, since NaN times 0 is NaN
            NaN | Infinity { .. } => (Self::NAN, Equal),
            // asinu(±0.0, u) = ±0.0, even for u = 0
            Zero { .. } => (self.clone(), Equal),
            Finite { .. } => {
                if self.gt_abs(&1u32) {
                    (Self::NAN, Equal)
                } else if u == 0 {
                    // asinu(x, 0) = 0 with the sign of x, which agrees with the x = 0 case and
                    // keeps the function odd. (MPFR returns +0 here for every x, although its own x
                    // = 0 case keeps the sign for exactly this reason.)
                    (
                        if *self < 0u32 {
                            Self::NEGATIVE_ZERO
                        } else {
                            Self::ZERO
                        },
                        Equal,
                    )
                } else {
                    asin_with_period_prec_round_normal_ref(self, u, prec, rm)
                }
            }
        }
    }

    /// Computes $\arcsin(x)u/(2\pi)$, the arcsine of a [`Float`] measured in $u$ths of a turn,
    /// rounding the result to the nearest value of the specified precision. The [`Float`] is taken
    /// by value. An [`Ordering`] is also returned, indicating whether the rounded arcsine is less
    /// than, equal to, or greater than the exact arcsine. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// If the arcsine is equidistant from two [`Float`]s with the specified precision, the
    /// [`Float`] with fewer 1s in its binary expansion is chosen.
    ///
    /// See [`Float::asin_with_period_prec_round`] for the error bounds, the special and closed-form
    /// cases, underflow, and the complexity; this function behaves the same way.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::asin_with_period_prec_round`] instead.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{One, OneHalf};
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (t, o) = Float::ONE.asin_with_period_prec(360, 10);
    /// assert_eq!(t.to_string(), "90.000");
    /// assert_eq!(o, Equal);
    ///
    /// let (t, o) = (Float::ONE_HALF >> 1u32).asin_with_period_prec(360, 10);
    /// assert_eq!(t.to_string(), "14.484");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn asin_with_period_prec(self, u: u64, prec: u64) -> (Self, Ordering) {
        self.asin_with_period_prec_round(u, prec, Nearest)
    }

    /// Computes $\arcsin(x)u/(2\pi)$, the arcsine of a [`Float`] measured in $u$ths of a turn,
    /// rounding the result to the nearest value of the specified precision. The [`Float`] is taken
    /// by reference. An [`Ordering`] is also returned, indicating whether the rounded arcsine is
    /// less than, equal to, or greater than the exact arcsine. Although `NaN`s are not comparable
    /// to any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`Float::asin_with_period_prec`] and [`Float::asin_with_period_prec_round`]; this
    /// function behaves the same way.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::OneHalf;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (t, o) = (&(Float::ONE_HALF >> 1u32)).asin_with_period_prec_ref(360, 10);
    /// assert_eq!(t.to_string(), "14.484");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn asin_with_period_prec_ref(&self, u: u64, prec: u64) -> (Self, Ordering) {
        self.asin_with_period_prec_round_ref(u, prec, Nearest)
    }

    /// Computes $\arcsin(x)u/(2\pi)$, the arcsine of a [`Float`] measured in $u$ths of a turn,
    /// rounding the result with the specified rounding mode. The [`Float`] is taken by value. An
    /// [`Ordering`] is also returned, indicating whether the rounded arcsine is less than, equal
    /// to, or greater than the exact arcsine. Although `NaN`s are not comparable to any [`Float`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the precision of the input.
    ///
    /// See [`Float::asin_with_period_prec_round`] for the error bounds, the special and closed-form
    /// cases, underflow, and the complexity; this function behaves the same way.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::asin_with_period_prec_round`] instead.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the result cannot be represented exactly with the precision of
    /// the input.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from_unsigned_prec(1u32, 10).0 >> 2u32;
    /// let (t, o) = x.asin_with_period_round(360, Floor);
    /// assert_eq!(t.to_string(), "14.469");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn asin_with_period_round(self, u: u64, rm: RoundingMode) -> (Self, Ordering) {
        let prec = self.significant_bits();
        self.asin_with_period_prec_round(u, prec, rm)
    }

    /// Computes $\arcsin(x)u/(2\pi)$, the arcsine of a [`Float`] measured in $u$ths of a turn,
    /// rounding the result with the specified rounding mode. The [`Float`] is taken by reference.
    /// An [`Ordering`] is also returned, indicating whether the rounded arcsine is less than, equal
    /// to, or greater than the exact arcsine. Although `NaN`s are not comparable to any [`Float`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`Float::asin_with_period_round`] and [`Float::asin_with_period_prec_round`]; this
    /// function behaves the same way.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the result cannot be represented exactly with the precision of
    /// the input.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from_unsigned_prec(1u32, 10).0 >> 2u32;
    /// let (t, o) = (&x).asin_with_period_round_ref(360, Floor);
    /// assert_eq!(t.to_string(), "14.469");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn asin_with_period_round_ref(&self, u: u64, rm: RoundingMode) -> (Self, Ordering) {
        self.asin_with_period_prec_round_ref(u, self.significant_bits(), rm)
    }

    /// Computes $\arcsin(x)u/(2\pi)$, the arcsine of a [`Float`] measured in $u$ths of a turn,
    /// rounding the result to the nearest value of the input's precision. The [`Float`] is taken by
    /// value.
    ///
    /// If the arcsine is equidistant from two [`Float`]s with the specified precision, the
    /// [`Float`] with fewer 1s in its binary expansion is chosen.
    ///
    /// See [`Float::asin_with_period_prec_round`] for the error bounds, the special and closed-form
    /// cases, underflow, and the complexity; this function behaves the same way.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::asin_with_period_round`] instead. If you want to specify an output precision,
    /// consider using [`Float::asin_with_period_prec`]. If you want both of these things, consider
    /// using [`Float::asin_with_period_prec_round`].
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    ///
    /// let x = Float::from_unsigned_prec(1u32, 10).0 >> 2u32;
    /// assert_eq!(x.asin_with_period(360).to_string(), "14.484");
    /// ```
    #[inline]
    pub fn asin_with_period(self, u: u64) -> Self {
        let prec = self.significant_bits();
        self.asin_with_period_prec(u, prec).0
    }

    /// Computes $\arcsin(x)u/(2\pi)$, the arcsine of a [`Float`] measured in $u$ths of a turn,
    /// rounding the result to the nearest value of the input's precision. The [`Float`] is taken by
    /// reference.
    ///
    /// See [`Float::asin_with_period`] and [`Float::asin_with_period_prec_round`]; this function
    /// behaves the same way.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    ///
    /// let x = Float::from_unsigned_prec(1u32, 10).0 >> 2u32;
    /// assert_eq!((&x).asin_with_period_ref(360).to_string(), "14.484");
    /// ```
    #[inline]
    pub fn asin_with_period_ref(&self, u: u64) -> Self {
        self.asin_with_period_prec_ref(u, self.significant_bits()).0
    }

    /// Computes $\arcsin(x)u/(2\pi)$, the arcsine of a [`Float`] measured in $u$ths of a turn, in
    /// place, rounding the result to the specified precision and with the specified rounding mode.
    /// An [`Ordering`] is returned, indicating whether the rounded arcsine is less than, equal to,
    /// or greater than the exact arcsine. Although `NaN`s are not comparable to any [`Float`],
    /// whenever this function assigns a `NaN` it also returns `Equal`.
    ///
    /// See [`Float::asin_with_period_prec_round`] for the error bounds, the special and closed-form
    /// cases, underflow, and the complexity; this function behaves the same way.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with the given precision.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::OneHalf;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::ONE_HALF >> 1u32;
    /// let o = x.asin_with_period_prec_round_assign(360, 10, Floor);
    /// assert_eq!(x.to_string(), "14.469");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn asin_with_period_prec_round_assign(
        &mut self,
        u: u64,
        prec: u64,
        rm: RoundingMode,
    ) -> Ordering {
        let (t, o) = self.asin_with_period_prec_round_ref(u, prec, rm);
        *self = t;
        o
    }

    /// Computes $\arcsin(x)u/(2\pi)$, the arcsine of a [`Float`] measured in $u$ths of a turn, in
    /// place, rounding the result to the nearest value of the specified precision. An [`Ordering`]
    /// is returned, indicating whether the rounded arcsine is less than, equal to, or greater than
    /// the exact arcsine. Although `NaN`s are not comparable to any [`Float`], whenever this
    /// function assigns a `NaN` it also returns `Equal`.
    ///
    /// See [`Float::asin_with_period_prec`] and [`Float::asin_with_period_prec_round`]; this
    /// function behaves the same way.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::OneHalf;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::ONE_HALF >> 1u32;
    /// let o = x.asin_with_period_prec_assign(360, 10);
    /// assert_eq!(x.to_string(), "14.484");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn asin_with_period_prec_assign(&mut self, u: u64, prec: u64) -> Ordering {
        self.asin_with_period_prec_round_assign(u, prec, Nearest)
    }

    /// Computes $\arcsin(x)u/(2\pi)$, the arcsine of a [`Float`] measured in $u$ths of a turn, in
    /// place, rounding the result with the specified rounding mode. An [`Ordering`] is returned,
    /// indicating whether the rounded arcsine is less than, equal to, or greater than the exact
    /// arcsine. Although `NaN`s are not comparable to any [`Float`], whenever this function assigns
    /// a `NaN` it also returns `Equal`.
    ///
    /// See [`Float::asin_with_period_round`] and [`Float::asin_with_period_prec_round`]; this
    /// function behaves the same way.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the result cannot be represented exactly with the precision of
    /// the input.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 10).0 >> 2u32;
    /// let o = x.asin_with_period_round_assign(360, Floor);
    /// assert_eq!(x.to_string(), "14.469");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn asin_with_period_round_assign(&mut self, u: u64, rm: RoundingMode) -> Ordering {
        let prec = self.significant_bits();
        self.asin_with_period_prec_round_assign(u, prec, rm)
    }

    /// Computes $\arcsin(x)u/(2\pi)$, the arcsine of a [`Float`] measured in $u$ths of a turn, in
    /// place, rounding the result to the nearest value of the input's precision.
    ///
    /// If the arcsine is equidistant from two [`Float`]s with the specified precision, the
    /// [`Float`] with fewer 1s in its binary expansion is chosen.
    ///
    /// See [`Float::asin_with_period_prec_round`] for the error bounds, the special and closed-form
    /// cases, underflow, and the complexity; this function behaves the same way.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::asin_with_period_round_assign`] instead. If you want to specify an output
    /// precision, consider using [`Float::asin_with_period_prec_assign`]. If you want both of these
    /// things, consider using [`Float::asin_with_period_prec_round_assign`].
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 10).0 >> 2u32;
    /// x.asin_with_period_assign(360);
    /// assert_eq!(x.to_string(), "14.484");
    /// ```
    #[inline]
    pub fn asin_with_period_assign(&mut self, u: u64) {
        let prec = self.significant_bits();
        self.asin_with_period_prec_assign(u, prec);
    }

    /// Computes $\arcsin(x)u/(2\pi)$, the arcsine of a [`Rational`] measured in $u$ths of a turn,
    /// rounding the result to the specified precision and with the specified rounding mode and
    /// returning the result as a [`Float`]. The [`Rational`] is taken by value. An [`Ordering`] is
    /// also returned, indicating whether the rounded arcsine is less than, equal to, or greater
    /// than the exact arcsine.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,u,p,m) = \arcsin(x)u/(2\pi)+\varepsilon.
    /// $$
    /// - If $x$ is zero, $|x|>1$, $u = 0$, $|x|$ is 1, or $|x|$ is $1/2$ and $u$ is a multiple of
    ///   3, $\varepsilon$ may be ignored or assumed to be 0.
    /// - Otherwise, if $m$ is not `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2
    ///   |\arcsin(x)u/(2\pi)|\rfloor-p+1}$.
    /// - Otherwise, if $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\arcsin(x)u/(2\pi)|\rfloor-p}$.
    ///
    /// The output has precision `prec`.
    ///
    /// Special cases:
    /// - $f(x,u,p,m)=\text{NaN}$ for $|x|>1$, including when $u=0$
    /// - $f(0,u,p,m)=0.0$
    /// - $f(x,0,p,m)=\pm0.0$, with the sign of $x$, so that the function stays odd
    /// - $f(\pm1,u,p,m)=\pm u/4$, a quarter turn
    /// - $f(\pm1/2,u,p,m)=\pm u/12$, a twelfth of a turn, when $u$ is a multiple of 3
    ///
    /// These are the only exact cases, and the quarter and twelfth turns are exact only when $p$ is
    /// large enough to hold them. A [`Rational`] has no signed zeros, so a zero $x$ gives a
    /// positive zero.
    ///
    /// Underflow:
    /// - If $0<f(x,u,p,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,u,p,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,u,p,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,u,p,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - The negative cases mirror these, since the function is odd.
    ///
    /// Overflow is not possible, since $|f(x,u,p,m)| \leq u/4 < 2^{62}$. Underflow requires a tiny
    /// $x$ together with a small $u$, since the result is about $xu/(2\pi)$ there. Unlike the
    /// [`Float`] case, $x$ itself may be far below the bottom of the exponent range.
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`Float::asin_with_period_rational_prec`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `x.significant_bits()`: $x^2/(1-x^2)$ is formed exactly, and its square root and arctangent
    /// are taken at a working precision of about $n$ bits and scaled by $u/(2\pi)$, which needs
    /// $\pi$ to that many bits; those cost the first term, and the second covers the $m$-bit input.
    /// The magnitude of the input does not drive the cost, and unlike the [`Float`] arcsine neither
    /// does its closeness to $\pm1$, since nothing cancels.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with the given precision (which is the case unless $x$ is zero, $|x|>1$, $u$ is zero, or $p$
    /// is large enough to hold the quarter or twelfth turn that $|x|=1$ or $|x|=1/2$ gives).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{One, OneHalf};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (t, o) = Float::asin_with_period_rational_prec_round(Rational::ONE, 360, 10, Exact);
    /// assert_eq!(t.to_string(), "90.000");
    /// assert_eq!(o, Equal);
    ///
    /// let (t, o) =
    ///     Float::asin_with_period_rational_prec_round(Rational::ONE_HALF, 360, 10, Exact);
    /// assert_eq!(t.to_string(), "30.000");
    /// assert_eq!(o, Equal);
    ///
    /// let (t, o) = Float::asin_with_period_rational_prec_round(
    ///     Rational::from_unsigneds(3u8, 5),
    ///     360,
    ///     10,
    ///     Floor,
    /// );
    /// assert_eq!(t.to_string(), "36.812");
    /// assert_eq!(o, Less);
    ///
    /// let (t, o) = Float::asin_with_period_rational_prec_round(
    ///     Rational::from_unsigneds(3u8, 5),
    ///     360,
    ///     10,
    ///     Ceiling,
    /// );
    /// assert_eq!(t.to_string(), "36.875");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    #[allow(clippy::needless_pass_by_value)]
    pub fn asin_with_period_rational_prec_round(
        x: Rational,
        u: u64,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        Self::asin_with_period_rational_prec_round_ref(&x, u, prec, rm)
    }

    /// Computes $\arcsin(x)u/(2\pi)$, the arcsine of a [`Rational`] measured in $u$ths of a turn,
    /// rounding the result to the specified precision and with the specified rounding mode and
    /// returning the result as a [`Float`]. The [`Rational`] is taken by reference. An [`Ordering`]
    /// is also returned, indicating whether the rounded arcsine is less than, equal to, or greater
    /// than the exact arcsine.
    ///
    /// See [`Float::asin_with_period_rational_prec_round`] for the error bounds, the special and
    /// closed-form cases, underflow, and the complexity; this function behaves the same way.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with the given precision.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (t, o) =
    ///     Float::asin_with_period_rational_prec_round_ref(&Rational::ONE, 360, 10, Exact);
    /// assert_eq!(t.to_string(), "90.000");
    /// assert_eq!(o, Equal);
    ///
    /// let (t, o) = Float::asin_with_period_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(3u8, 5),
    ///     360,
    ///     10,
    ///     Floor,
    /// );
    /// assert_eq!(t.to_string(), "36.812");
    /// assert_eq!(o, Less);
    /// ```
    pub fn asin_with_period_rational_prec_round_ref(
        x: &Rational,
        u: u64,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        assert_ne!(prec, 0);
        if x.gt_abs(&1u32) {
            // asinu(x, u) = NaN for |x| > 1, including for u = 0, since NaN times 0 is NaN
            return (Self::NAN, Equal);
        }
        if *x == 0u32 || u == 0 {
            // asinu(0, u) = 0, and asinu(x, 0) = 0 with the sign of x, so that the function stays
            // odd; a `Rational` zero has no sign, so the first case gives a positive zero
            return (
                if *x < 0u32 {
                    Self::NEGATIVE_ZERO
                } else {
                    Self::ZERO
                },
                Equal,
            );
        }
        asin_with_period_rational_helper(x, u, prec, rm)
    }

    /// Computes $\arcsin(x)u/(2\pi)$, the arcsine of a [`Rational`] measured in $u$ths of a turn,
    /// rounding the result to the nearest value of the specified precision and returning the result
    /// as a [`Float`]. The [`Rational`] is taken by value. An [`Ordering`] is also returned,
    /// indicating whether the rounded arcsine is less than, equal to, or greater than the exact
    /// arcsine.
    ///
    /// If the arcsine is equidistant from two [`Float`]s with the specified precision, the
    /// [`Float`] with fewer 1s in its binary expansion is chosen.
    ///
    /// See [`Float::asin_with_period_rational_prec_round`] for the error bounds, the special and
    /// closed-form cases, underflow, and the complexity; this function behaves the same way.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::asin_with_period_rational_prec_round`] instead.
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
    /// let (t, o) =
    ///     Float::asin_with_period_rational_prec(Rational::from_unsigneds(3u8, 5), 360, 10);
    /// assert_eq!(t.to_string(), "36.875");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn asin_with_period_rational_prec(x: Rational, u: u64, prec: u64) -> (Self, Ordering) {
        Self::asin_with_period_rational_prec_round(x, u, prec, Nearest)
    }

    /// Computes $\arcsin(x)u/(2\pi)$, the arcsine of a [`Rational`] measured in $u$ths of a turn,
    /// rounding the result to the nearest value of the specified precision and returning the result
    /// as a [`Float`]. The [`Rational`] is taken by reference. An [`Ordering`] is also returned,
    /// indicating whether the rounded arcsine is less than, equal to, or greater than the exact
    /// arcsine.
    ///
    /// See [`Float::asin_with_period_rational_prec`] and
    /// [`Float::asin_with_period_rational_prec_round`]; this function behaves the same way.
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
    /// let (t, o) =
    ///     Float::asin_with_period_rational_prec_ref(&Rational::from_unsigneds(3u8, 5), 360, 10);
    /// assert_eq!(t.to_string(), "36.875");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn asin_with_period_rational_prec_ref(x: &Rational, u: u64, prec: u64) -> (Self, Ordering) {
        Self::asin_with_period_rational_prec_round_ref(x, u, prec, Nearest)
    }
}

impl Asin for Float {
    type Output = Self;

    /// Computes $\arcsin x$, the arcsine of a [`Float`], taking it by value.
    ///
    /// If the output has a precision, it is the precision of the input. If the arcsine is
    /// equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s in
    /// its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// $$
    /// f(x) = \arcsin x+\varepsilon.
    /// $$
    /// - If $x$ is NaN, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x$ is not NaN, then $|\varepsilon| < 2^{\lfloor\log_2 |\arcsin x|\rfloor-p}$, where
    ///   $p$ is the precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN})=f(\pm\infty)=\text{NaN}$
    /// - $f(x)=\text{NaN}$ for $|x|>1$
    /// - $f(\pm0.0)=\pm0.0$
    /// - $f(\pm1)=\pm\pi/2$, rounded
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::asin_round`] instead. If you want to specify the output precision, consider using
    /// [`Float::asin_prec`]. If you want both of these things, consider using
    /// [`Float::asin_prec_round`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^3 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`: the
    /// arcsine is taken as $\arctan(x/\sqrt{1-x^2})$ at a working precision of about $n$ plus the
    /// number of bits that cancel in $1-x^2$, which an input within $2^{-n}$ of $\pm1$ pushes to
    /// another $n$; the arctangent at that width dominates. The magnitude of the input does not
    /// otherwise drive the cost.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Asin;
    /// use malachite_base::num::basic::traits::*;
    /// use malachite_float::Float;
    ///
    /// assert!(Float::NAN.asin().is_nan());
    /// // the arcsine is NaN outside [-1, 1], and both infinities are outside it
    /// assert_eq!(Float::INFINITY.asin().to_string(), "NaN");
    /// assert_eq!(Float::NEGATIVE_INFINITY.asin().to_string(), "NaN");
    /// assert_eq!(Float::ZERO.asin().to_string(), "0.0");
    /// assert_eq!(Float::NEGATIVE_ZERO.asin().to_string(), "-0.0");
    /// assert_eq!(
    ///     Float::from_unsigned_prec(1u32, 100).0.asin().to_string(),
    ///     "1.5707963267948966192313216916397"
    /// );
    /// assert_eq!(
    ///     Float::from_unsigned_prec(100u32, 100).0.asin().to_string(),
    ///     "NaN"
    /// );
    /// ```
    #[inline]
    fn asin(self) -> Self {
        let prec = self.significant_bits();
        self.asin_prec_round(prec, Nearest).0
    }
}

impl Asin for &Float {
    type Output = Float;

    /// Computes $\arcsin x$, the arcsine of a [`Float`], taking it by reference.
    ///
    /// If the output has a precision, it is the precision of the input. If the arcsine is
    /// equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s in
    /// its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// $$
    /// f(x) = \arcsin x+\varepsilon.
    /// $$
    /// - If $x$ is NaN, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x$ is not NaN, then $|\varepsilon| < 2^{\lfloor\log_2 |\arcsin x|\rfloor-p}$, where
    ///   $p$ is the precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN})=f(\pm\infty)=\text{NaN}$
    /// - $f(x)=\text{NaN}$ for $|x|>1$
    /// - $f(\pm0.0)=\pm0.0$
    /// - $f(\pm1)=\pm\pi/2$, rounded
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::asin_round_ref`] instead. If you want to specify the output precision, consider
    /// using [`Float::asin_prec_ref`]. If you want both of these things, consider using
    /// [`Float::asin_prec_round_ref`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^3 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`: the
    /// arcsine is taken as $\arctan(x/\sqrt{1-x^2})$ at a working precision of about $n$ plus the
    /// number of bits that cancel in $1-x^2$, which an input within $2^{-n}$ of $\pm1$ pushes to
    /// another $n$; the arctangent at that width dominates. The magnitude of the input does not
    /// otherwise drive the cost.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Asin;
    /// use malachite_base::num::basic::traits::*;
    /// use malachite_float::Float;
    ///
    /// assert!(Float::NAN.asin().is_nan());
    /// // the arcsine is NaN outside [-1, 1], and both infinities are outside it
    /// assert_eq!(Float::INFINITY.asin().to_string(), "NaN");
    /// assert_eq!(Float::NEGATIVE_INFINITY.asin().to_string(), "NaN");
    /// assert_eq!(Float::ZERO.asin().to_string(), "0.0");
    /// assert_eq!(Float::NEGATIVE_ZERO.asin().to_string(), "-0.0");
    /// assert_eq!(
    ///     (&Float::from_unsigned_prec(1u32, 100).0).asin().to_string(),
    ///     "1.5707963267948966192313216916397"
    /// );
    /// assert_eq!(
    ///     (&Float::from_unsigned_prec(100u32, 100).0)
    ///         .asin()
    ///         .to_string(),
    ///     "NaN"
    /// );
    /// ```
    #[inline]
    fn asin(self) -> Float {
        self.asin_prec_round_ref(self.significant_bits(), Nearest).0
    }
}

impl AsinAssign for Float {
    /// Computes $\arcsin x$, the arcsine of a [`Float`], in place.
    ///
    /// If the output has a precision, it is the precision of the input. If the arcsine is
    /// equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s in
    /// its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// $$
    /// x \gets \arcsin x+\varepsilon.
    /// $$
    /// - If $x$ is NaN, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x$ is not NaN, then $|\varepsilon| < 2^{\lfloor\log_2 |\arcsin x|\rfloor-p}$, where
    ///   $p$ is the precision of the input.
    ///
    /// See the [`Float::asin`] documentation for information on the special cases.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::asin_round_assign`] instead. If you want to specify the output precision, consider
    /// using [`Float::asin_prec_assign`]. If you want both of these things, consider using
    /// [`Float::asin_prec_round_assign`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^3 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`: the
    /// arcsine is taken as $\arctan(x/\sqrt{1-x^2})$ at a working precision of about $n$ plus the
    /// number of bits that cancel in $1-x^2$, which an input within $2^{-n}$ of $\pm1$ pushes to
    /// another $n$; the arctangent at that width dominates. The magnitude of the input does not
    /// otherwise drive the cost.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::AsinAssign;
    /// use malachite_base::num::basic::traits::*;
    /// use malachite_float::Float;
    ///
    /// let mut x = Float::NAN;
    /// x.asin_assign();
    /// assert!(x.is_nan());
    ///
    /// let mut x = Float::INFINITY;
    /// x.asin_assign();
    /// assert_eq!(x.to_string(), "NaN");
    ///
    /// let mut x = Float::NEGATIVE_INFINITY;
    /// x.asin_assign();
    /// assert_eq!(x.to_string(), "NaN");
    ///
    /// let mut x = Float::ZERO;
    /// x.asin_assign();
    /// assert_eq!(x.to_string(), "0.0");
    ///
    /// let mut x = Float::NEGATIVE_ZERO;
    /// x.asin_assign();
    /// assert_eq!(x.to_string(), "-0.0");
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// x.asin_assign();
    /// assert_eq!(x.to_string(), "1.5707963267948966192313216916397");
    ///
    /// let mut x = Float::from_unsigned_prec(100u32, 100).0;
    /// x.asin_assign();
    /// assert_eq!(x.to_string(), "NaN");
    /// ```
    #[inline]
    fn asin_assign(&mut self) {
        let prec = self.significant_bits();
        self.asin_prec_round_assign(prec, Nearest);
    }
}
/// Computes $\arcsin x$, the arcsine of a primitive float. Using this function is more accurate
/// than using the default `asin` function or the one provided by `libm`.
///
/// $$
/// f(x) = \arcsin x+\varepsilon.
/// $$
/// - If $x$ is NaN, $\varepsilon$ may be ignored or assumed to be 0.
/// - If $x$ is not NaN, then $|\varepsilon| < 2^{\lfloor\log_2 |\arcsin x|\rfloor-p}$, where $p$ is
///   the precision of the output (24 if `T` is a [`f32`] and 53 if `T` is a [`f64`]).
///
/// Special cases:
/// - $f(\text{NaN},p,m)=f(\pm\infty,p,m)=\text{NaN}$
/// - $f(x,p,m)=\text{NaN}$ for $|x|>1$
/// - $f(\pm0.0,p,m)=\pm0.0$
/// - $f(\pm1,p,m)=\pm\pi/2$, rounded
///
/// Neither overflow nor underflow is possible: the result lies in $[-\pi/2, \pi/2]$, and $|\arcsin
/// x| > |x|$ for nonzero $x$, so the result is subnormal only when $x$ is, and then it is $x$
/// itself, since $|\arcsin x - x| < |x|^3/3$.
///
/// # Worst-case complexity
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use malachite_base::num::basic::traits::NegativeInfinity;
/// use malachite_base::num::float::NiceFloat;
/// use malachite_float::float::arithmetic::asin::primitive_float_asin;
///
/// assert!(primitive_float_asin(f32::NAN).is_nan());
/// assert_eq!(
///     NiceFloat(primitive_float_asin(f32::INFINITY)),
///     NiceFloat(f32::NAN)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_asin(f32::NEGATIVE_INFINITY)),
///     NiceFloat(f32::NAN)
/// );
/// assert_eq!(NiceFloat(primitive_float_asin(0.0f32)), NiceFloat(0.0));
/// assert_eq!(NiceFloat(primitive_float_asin(-0.0f32)), NiceFloat(-0.0));
/// assert_eq!(
///     NiceFloat(primitive_float_asin(1.0f32)),
///     NiceFloat(1.5707964)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_asin(1.0f64)),
///     NiceFloat(1.5707963267948966)
/// );
/// ```
#[inline]
#[allow(clippy::type_repetition_in_bounds)]
pub fn primitive_float_asin<T: PrimitiveFloat>(x: T) -> T
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    emulate_float_to_float_fn(Float::asin_prec, x)
}

/// Computes $\arcsin x$, the arcsine of a [`Rational`], returning the result as a primitive float.
///
/// $$
/// f(x) = \arcsin x+\varepsilon,
/// $$
/// where $|\varepsilon| < 2^{\lfloor\log_2 |\arcsin x|\rfloor-p}$ and $p$ is the precision of the
/// output (24 if `T` is a [`f32`] and 53 if `T` is a [`f64`]); the special cases below are exact.
///
/// Special cases:
/// - $f(x)=\text{NaN}$ for $|x|>1$
/// - $f(0)=0.0$
/// - $f(\pm1)=\pm\pi/2$, rounded
///
/// Overflow is not possible, since the result lies in $[-\pi/2, \pi/2]$. The result is subnormal,
/// or zero, only for an $x$ that is itself that small.
///
/// # Worst-case complexity
/// $T(m) = O(m \log m \log\log m)$
///
/// $M(m) = O(m \log m)$
///
/// where $T$ is time, $M$ is additional memory, and $m$ is `x.significant_bits()`.
///
/// # Examples
/// ```
/// use malachite_base::num::basic::traits::{One, Zero};
/// use malachite_base::num::float::NiceFloat;
/// use malachite_float::float::arithmetic::asin::primitive_float_asin_rational;
/// use malachite_q::Rational;
///
/// assert_eq!(
///     NiceFloat(primitive_float_asin_rational::<f64>(&Rational::ZERO)),
///     NiceFloat(0.0)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_asin_rational::<f64>(&Rational::ONE)),
///     NiceFloat(1.5707963267948966)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_asin_rational::<f64>(
///         &Rational::from_unsigneds(3u8, 5)
///     )),
///     NiceFloat(0.6435011087932844)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_asin_rational::<f32>(
///         &Rational::from_unsigneds(3u8, 5)
///     )),
///     NiceFloat(0.6435011)
/// );
/// ```
#[inline]
#[allow(clippy::type_repetition_in_bounds)]
pub fn primitive_float_asin_rational<T: PrimitiveFloat>(x: &Rational) -> T
where
    Float: PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float>,
{
    emulate_rational_to_float_fn(Float::asin_rational_prec_ref, x)
}

/// Computes $\arcsin(x)u/(2\pi)$, the arcsine of a primitive float measured in $u$ths of a turn (so
/// that `u = 360` gives degrees), returning the result as a primitive float.
///
/// $$
/// f(x,u) = \arcsin(x)u/(2\pi)+\varepsilon.
/// $$
/// - If $x$ is zero, $|x|>1$, $u = 0$, $|x|$ is 1, or $|x|$ is $1/2$ and $u$ is a multiple of 3,
///   $\varepsilon$ may be ignored or assumed to be 0.
/// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2 |\arcsin(x)u/(2\pi)|\rfloor-p}$, where $p$ is the
///   precision of the output (24 if `T` is a [`f32`] and 53 if `T` is a [`f64`]).
///
/// Special cases:
/// - $f(x,u)=\text{NaN}$ for $|x|>1$, including when $u=0$
/// - $f(\pm0.0,u)=\pm0.0$
/// - $f(x,0)=\pm0.0$, with the sign of $x$, so that the function stays odd
/// - $f(\pm1,u)=\pm u/4$, a quarter turn
/// - $f(\pm1/2,u)=\pm u/12$, a twelfth of a turn, when $u$ is a multiple of 3
///
/// Overflow is not possible, since $|f(x,u)| \leq u/4 < 2^{62}$. The result is subnormal, or zero,
/// only when $x$ is tiny and $u$ is small, since the result is about $xu/(2\pi)$ there.
///
/// # Worst-case complexity
/// $T(m) = O(m \log m \log\log m)$
///
/// $M(m) = O(m \log m)$
///
/// where $T$ is time, $M$ is additional memory, and $m$ is `x.significant_bits()`.
///
/// # Examples
/// ```
/// use malachite_base::num::float::NiceFloat;
/// use malachite_float::float::arithmetic::asin::primitive_float_asin_with_period;
///
/// assert!(primitive_float_asin_with_period(f32::NAN, 360).is_nan());
/// // an input outside [-1, 1] is NaN
/// assert!(primitive_float_asin_with_period(2.0f32, 360).is_nan());
/// // an input of 1 is a quarter turn
/// assert_eq!(
///     NiceFloat(primitive_float_asin_with_period(1.0f32, 360)),
///     NiceFloat(90.0)
/// );
/// // an input of 1/2 is a twelfth of a turn
/// assert_eq!(
///     NiceFloat(primitive_float_asin_with_period(0.5f32, 360)),
///     NiceFloat(30.0)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_asin_with_period(0.25f32, 360)),
///     NiceFloat(14.477512)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_asin_with_period(0.25f64, 360)),
///     NiceFloat(14.477512185929925)
/// );
/// ```
#[inline]
#[allow(clippy::type_repetition_in_bounds)]
pub fn primitive_float_asin_with_period<T: PrimitiveFloat>(x: T, u: u64) -> T
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    emulate_float_to_float_fn(|x, prec| Float::asin_with_period_prec(x, u, prec), x)
}

/// Computes $\arcsin(x)u/(2\pi)$, the arcsine of a [`Rational`] measured in $u$ths of a turn (so
/// that `u = 360` gives degrees), returning the result as a primitive float.
///
/// $$
/// f(x,u) = \arcsin(x)u/(2\pi)+\varepsilon.
/// $$
/// - If $x$ is zero, $|x|>1$, $u = 0$, $|x|$ is 1, or $|x|$ is $1/2$ and $u$ is a multiple of 3,
///   $\varepsilon$ may be ignored or assumed to be 0.
/// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2 |\arcsin(x)u/(2\pi)|\rfloor-p}$, where $p$ is the
///   precision of the output (24 if `T` is a [`f32`] and 53 if `T` is a [`f64`]).
///
/// Special cases:
/// - $f(x,u)=\text{NaN}$ for $|x|>1$, including when $u=0$
/// - $f(0,u)=0.0$
/// - $f(x,0)=\pm0.0$, with the sign of $x$, so that the function stays odd
/// - $f(\pm1,u)=\pm u/4$, a quarter turn
/// - $f(\pm1/2,u)=\pm u/12$, a twelfth of a turn, when $u$ is a multiple of 3
///
/// Overflow is not possible, since $|f(x,u)| \leq u/4 < 2^{62}$. The result is subnormal, or zero,
/// only when $x$ is tiny and $u$ is small, since the result is about $xu/(2\pi)$ there.
///
/// # Worst-case complexity
/// $T(m) = O(m \log m \log\log m)$
///
/// $M(m) = O(m \log m)$
///
/// where $T$ is time, $M$ is additional memory, and $m$ is `x.significant_bits()`.
///
/// # Examples
/// ```
/// use malachite_base::num::basic::traits::{One, OneHalf, Zero};
/// use malachite_base::num::float::NiceFloat;
/// use malachite_float::float::arithmetic::asin::primitive_float_asin_with_period_rational;
/// use malachite_q::Rational;
///
/// assert_eq!(
///     NiceFloat(primitive_float_asin_with_period_rational::<f64>(
///         &Rational::ZERO,
///         360
///     )),
///     NiceFloat(0.0)
/// );
/// // an input of 1 is a quarter turn
/// assert_eq!(
///     NiceFloat(primitive_float_asin_with_period_rational::<f64>(
///         &Rational::ONE,
///         360
///     )),
///     NiceFloat(90.0)
/// );
/// // an input of 1/2 is a twelfth of a turn
/// assert_eq!(
///     NiceFloat(primitive_float_asin_with_period_rational::<f64>(
///         &Rational::ONE_HALF,
///         360
///     )),
///     NiceFloat(30.0)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_asin_with_period_rational::<f64>(
///         &Rational::from_unsigneds(3u8, 5),
///         360
///     )),
///     NiceFloat(36.86989764584402)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_asin_with_period_rational::<f32>(
///         &Rational::from_unsigneds(3u8, 5),
///         360
///     )),
///     NiceFloat(36.869896)
/// );
/// ```
#[inline]
#[allow(clippy::type_repetition_in_bounds)]
pub fn primitive_float_asin_with_period_rational<T: PrimitiveFloat>(x: &Rational, u: u64) -> T
where
    Float: PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float>,
{
    emulate_rational_to_float_fn(
        |x, prec| Float::asin_with_period_rational_prec_ref(x, u, prec),
        x,
    )
}
