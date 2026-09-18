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
use crate::float::arithmetic::asin::{asin_at_prec, asin_cancellation};
use crate::float::arithmetic::atan::{arc_with_period_scale, scaled_unsigned};
use crate::float::arithmetic::sin::{SCALE, SCALED_INPUT_EXPONENT, scaled_underflow};
use crate::{emulate_float_to_float_fn, emulate_rational_to_float_fn};
use core::cmp::Ordering::{self, Equal, Greater, Less};
use malachite_base::num::arithmetic::traits::{
    Acos, AcosAssign, CeilingLogBase2, IsPowerOf2, Square,
};
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::{NaN as NaNTrait, One, Zero as ZeroTrait};
use malachite_base::num::comparison::traits::PartialOrdAbs;
use malachite_base::num::conversion::traits::{ExactFrom, RoundingFrom};
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{self, Exact, Nearest, Up};
use malachite_nz::natural::arithmetic::float::round::float_can_round;
use malachite_nz::platform::Limb;
use malachite_q::Rational;

// An inverse cosine or secant whose radicand -- 2(1 - x) for the one, 2(x - 1) for the other -- has
// at most this exponent falls at or below the bottom of the exponent range, since the square root
// halves it.
pub(crate) const SCALED_RADICAND_EXPONENT: i64 = SCALED_INPUT_EXPONENT << 1;
// The radicand is scaled by this much, so that its square root is scaled by 2^SCALE: one shift for
// the doubling, and two SCALEs for the root.
pub(crate) const SCALED_RADICAND_SHIFT: u64 = (SCALE << 1) + 1;

// Computes acos(x) for a finite nonzero `Float` x, rounded to precision `prec` with rounding mode
// `rm`.
//
// This is mpfr_acos from acos.c, MPFR 4.2.2, for a finite nonzero input.
fn acos_prec_round_normal_ref(x: &Float, prec: u64, rm: RoundingMode) -> (Float, Ordering) {
    let positive = *x > 0u32;
    match x.partial_cmp_abs(&1u32).unwrap() {
        // acos(x) = NaN for |x| > 1
        Greater => (Float::NAN, Equal),
        // acos(1) = +0, exactly, and acos(-1) = pi
        Equal => {
            if positive {
                (Float::ZERO, Equal)
            } else {
                Float::pi_prec_round(prec, rm)
            }
        }
        Less => {
            assert_ne!(rm, Exact, "Inexact acos");
            // The quotient x/sqrt(1 - x^2) loses the bits that 1 - x^2 does, and for a positive x
            // the subtraction pi/2 - asin(x) loses about as many again, since acos(x) is small
            // there; a negative x keeps acos(x) near pi, so nothing cancels in the subtraction and
            // only the quotient's loss is charged for.
            let cancel = asin_cancellation(x, positive);
            let supplement = if positive { (cancel << 1) - 2 } else { cancel };
            let mut w = prec + prec.ceiling_log_base_2() + 10 + supplement;
            let mut increment = Limb::WIDTH;
            loop {
                // acos(x) = pi/2 - asin(x) = pi/2 - atan(x/sqrt(1 - x^2))
                let t = asin_at_prec(x, w);
                // exact
                let half_pi = Float::pi_prec(w).0 >> 1u32;
                let t = half_pi.sub_prec(t, w).0;
                if float_can_round(t.significand_ref().unwrap(), w - supplement, prec, rm) {
                    return Float::from_float_prec_round(t, prec, rm);
                }
                w += increment;
                increment = w >> 1;
            }
        }
    }
}

// Computes acos(x) u/(2 pi) for a finite nonzero `Float` x with |x| <= 1 and a nonzero u, rounded
// to precision `prec` with rounding mode `rm`. `rm` may be `Exact` only at x = 1, where the result
// is zero; at |x| = 1, where it is u/2; and at |x| = 1/2 with u a multiple of 3, where it is u/6 or
// u/3.
//
// This is mpfr_acosu from acosu.c, MPFR 4.2.2. The quotient is formed with the numerator scaled up
// by 2^SCALE, as in `atan_with_period`, since acos(x) u/(2 pi) can fall below the smallest positive
// `Float` for an x near 1 and a small u, which MPFR, computing inside a temporarily extended
// exponent range, never sees.
fn acos_with_period_prec_round_normal_ref(
    x: &Float,
    u: u64,
    prec: u64,
    rm: RoundingMode,
) -> (Float, Ordering) {
    let positive = *x > 0u32;
    let exp_x = i64::from(x.get_exponent().unwrap());
    let power_of_2 = x.significand_ref().unwrap().is_power_of_2();
    // |x| = 1: acosu(1, u) = +0, following IEEE 754-2019's acosPi, and acosu(-1, u) = u/2
    if exp_x == 1 && power_of_2 {
        return if positive {
            (Float::ZERO, Equal)
        } else {
            scaled_unsigned(u, 1, true, prec, rm)
        };
    }
    // acos(1/2) = pi/3 and acos(-1/2) = 2 pi/3, so acosu(1/2, u) = u/6 and acosu(-1/2, u) = u/3,
    // both exact when u is a multiple of 3
    if exp_x == 0 && power_of_2 && u.is_multiple_of(3) {
        return scaled_unsigned(u / 3, u32::from(positive), true, prec, rm);
    }
    // Nothing else can be rounded exactly
    assert_ne!(rm, Exact, "Inexact acos_with_period");
    // For |x| < 1/2, acos(x) = pi/2 - x r(x) with |r(x)| < 1.05, so acosu(x, u) = u/4 (1 - x s(x))
    // with 0 <= s(x) < 1. Once EXP(x) <= -prec - 3 that correction is below an eighth of an ulp of
    // u/4, so the result is the neighbour of u/4 on the side the arccosine lies: below it for a
    // positive x, whose arccosine is under pi/2, and above it for a negative one. Requiring EXP(x)
    // <= -64 as well keeps the correction below the last bit of u when u/4 is inexact.
    if exp_x <= -64 && exp_x <= -i64::exact_from(prec) - 3 {
        let w = if prec <= 63 { 65 } else { prec + 2 };
        // exact, since w >= 64
        let mut t = Float::from_unsigned_prec_round(u, w, Exact).0;
        if positive {
            t.decrement();
        } else {
            t.increment();
        }
        // the last bit of t is 1 and w exceeds the target precision, so t is not representable
        // there, which pins the ternary value below
        t >>= 2u32;
        return Float::from_float_prec_round(t, prec, rm);
    }
    arc_with_period_scale(
        // scaling by a power of 2 is exact, and acos(x) u 2^SCALE stays far below the top of the
        // range, since acos(x) <= pi and u < 2^64
        |w| x.acos_prec_round_ref(w, Up).0 << SCALE,
        u,
        true,
        prec,
        rm,
    )
}

// Computes acos(x) for a `Rational` x with 0 < |x| < 1, rounded to precision `prec` with rounding
// mode `rm`. (The rest is handled by the caller.)
//
// MPFR has no arccosine of a rational. Its `Float` algorithm takes pi/2 - atan(x/sqrt(1 - x^2)) and
// pays for the cancellation in both the subtraction and the quotient; here the identity is used in
// the form
//
//     acos(x) = atan(sqrt((1 - x^2)/x^2)),
//
// whose argument is an exact `Rational`. For a positive x that is the whole answer, and nothing
// cancels anywhere: the arctangent of a small argument is small, which is exactly what acos(x) is
// when x is near 1. A negative x is pi minus that, which loses a single bit at worst, since the
// result is then at least pi/2.
pub(crate) fn acos_rational_helper(x: &Rational, prec: u64, rm: RoundingMode) -> (Float, Ordering) {
    assert_ne!(rm, Exact, "Inexact acos_rational");
    let positive = *x > 0u32;
    let mut w = prec + prec.ceiling_log_base_2() + 10;
    let mut increment = Limb::WIDTH;
    if positive {
        // With u = 1 - x, acos(x) = sqrt(2u)(1 + u/12 + ...). A `Rational` can sit close enough to
        // 1 to put that below the smallest positive `Float`, which is a regime the `Float`
        // arccosine cannot reach; there u is below 2^(2 SCALED_INPUT_EXPONENT), so the correction
        // is invisible at any working precision the loop can reach and the answer is sqrt(2u),
        // rounded. It is formed scaled up, the radicand by 2^(2 SCALE) so that its square root is
        // scaled by 2^SCALE, and the underflow is then decided by the rounding mode alone. Taking
        // the square root of 2u rather than of (1 - x^2)/x^2 also keeps this path cheap: an x this
        // close to 1 has a huge numerator and denominator, and squaring it would double their size.
        let u = Rational::ONE - x;
        if u.floor_log_base_2_abs() + 2 <= SCALED_RADICAND_EXPONENT {
            let scaled = u << SCALED_RADICAND_SHIFT;
            loop {
                // rounded away from zero, the side acos(x) is on
                let t = Float::sqrt_rational_prec_round_ref(&scaled, w, Up).0;
                if let Some(result) = scaled_underflow(&t, true, prec, rm) {
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
    }
    let x2 = x.square();
    // exact, and positive since |x| < 1
    let r = (Rational::ONE - &x2) / x2;
    loop {
        // The square root is correctly rounded and the arctangent neither amplifies a relative
        // error nor adds more than its own half ulp, so two bits of slack cover the positive case;
        // pi and the subtraction take two more.
        let t = Float::sqrt_rational_prec_ref(&r, w).0.atan_prec(w).0;
        let (t, err) = if positive {
            (t, 3)
        } else {
            (Float::pi_prec(w).0.sub_prec(t, w).0, 4)
        };
        if float_can_round(t.significand_ref().unwrap(), w - err, prec, rm) {
            return Float::from_float_prec_round(t, prec, rm);
        }
        w += increment;
        increment = w >> 1;
    }
}

// Computes acos(x) u/(2 pi) for a `Rational` x with 0 < |x| <= 1 and a nonzero u, rounded to
// precision `prec` with rounding mode `rm`. (x = 0, u = 0, and |x| > 1 are handled by the caller.)
// `rm` may be `Exact` only at |x| = 1, where the result is zero or u/2, and at |x| = 1/2 with u a
// multiple of 3, where it is u/6 or u/3.
//
// MPFR has no arccosine of a rational. The branches match the `Float` case, with one addition: an x
// close enough to 1 that acos(x) falls below the bottom of the exponent range is answered from
// sqrt(2(1 - x)) directly. That substitution is needed rather than merely cheaper, since
// `acos_rational_helper` reports such an x as an underflow, and a large u can lift the quotient
// back into the range, where that answer would be wrong.
pub(crate) fn acos_with_period_rational_helper(
    x: &Rational,
    u: u64,
    prec: u64,
    rm: RoundingMode,
) -> (Float, Ordering) {
    let positive = *x > 0u32;
    let exp_x = x.floor_log_base_2_abs() + 1; // the MPFR-style exponent of x
    // |x| = 1: acosu(1, u) = +0, following IEEE 754-2019's acosPi, and acosu(-1, u) = u/2
    if exp_x == 1 {
        return if positive {
            (Float::ZERO, Equal)
        } else {
            scaled_unsigned(u, 1, true, prec, rm)
        };
    }
    // acos(1/2) = pi/3 and acos(-1/2) = 2 pi/3, so acosu(1/2, u) = u/6 and acosu(-1/2, u) = u/3,
    // both exact when u is a multiple of 3
    if u.is_multiple_of(3) && x.numerator_ref() == &1u32 && x.denominator_ref() == &2u32 {
        return scaled_unsigned(u / 3, u32::from(positive), true, prec, rm);
    }
    // Nothing else can be rounded exactly
    assert_ne!(rm, Exact, "Inexact acos_with_period_rational");
    // as in the `Float` case, a tiny x is answered from the neighbour of u/4
    if exp_x <= -64 && exp_x <= -i64::exact_from(prec) - 3 {
        let w = if prec <= 63 { 65 } else { prec + 2 };
        // exact, since w >= 64
        let mut t = Float::from_unsigned_prec_round(u, w, Exact).0;
        if positive {
            t.decrement();
        } else {
            t.increment();
        }
        t >>= 2u32;
        return Float::from_float_prec_round(t, prec, rm);
    }
    if positive {
        // An x within 2^(2 SCALED_INPUT_EXPONENT) of 1 puts acos(x) = sqrt(2(1 - x))(1 + ...) below
        // the smallest positive `Float`, where `acos_rational_helper` would report an underflow --
        // but a large u can lift acos(x) u/(2 pi) back into the range, so the square root is taken
        // here instead, scaled up by 2^SCALE for the quotient below.
        let v = Rational::ONE - x;
        if v.floor_log_base_2_abs() + 2 <= SCALED_RADICAND_EXPONENT {
            let scaled = v << SCALED_RADICAND_SHIFT;
            return arc_with_period_scale(
                |w| Float::sqrt_rational_prec_round_ref(&scaled, w, Up).0,
                u,
                true,
                prec,
                rm,
            );
        }
    }
    arc_with_period_scale(
        |w| acos_rational_helper(x, w, Up).0 << SCALE,
        u,
        true,
        prec,
        rm,
    )
}

impl Float {
    /// Computes $\arccos x$, the arccosine of a [`Float`], rounding the result to the specified
    /// precision and with the specified rounding mode. The [`Float`] is taken by value. An
    /// [`Ordering`] is also returned, indicating whether the rounded arccosine is less than, equal
    /// to, or greater than the exact arccosine. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \arccos x+\varepsilon.
    /// $$
    /// - If $x$ is NaN, if $|x|>1$, or if $x$ is 1, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - Otherwise, if $m$ is not `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2 |\arccos
    ///   x|\rfloor-p+1}$.
    /// - Otherwise, if $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2 |\arccos
    ///   x|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=f(\pm\infty,p,m)=\text{NaN}$
    /// - $f(x,p,m)=\text{NaN}$ for $|x|>1$
    /// - $f(\pm0.0,p,m)=\pi/2$, rounded
    /// - $f(1,p,m)=0.0$
    /// - $f(-1,p,m)=\pi$, rounded
    ///
    /// The zero at $x=1$ is the only exact case; unlike the arcsine, a zero input is not one, since
    /// $\pi/2$ is never exactly representable.
    ///
    /// Overflow is not possible, since the result lies in $[0,\pi]$. The result is zero only at
    /// $x=1$: an input just below 1 gives about $\sqrt{2(1-x)}$, which stays representable unless
    /// the input's precision exceeds $2^{31}$ bits.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::acos_prec`] instead. If you
    /// know that your target precision is the precision of the input, consider using
    /// [`Float::acos_round`] instead. If both of these things are true, consider using
    /// [`Float::acos`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O((n+m) (\log (n+m))^3 \log\log (n+m))$
    ///
    /// $M(n, m) = O((n+m) \log (n+m))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `self.significant_bits()`: the arccosine is taken as $\pi/2-\arctan(x/\sqrt{1-x^2})$ at a
    /// working precision of about $n$ plus the bits that cancel there, which an input within
    /// $2^{-m}$ of 1 pushes to $2m$; a negative input loses nothing in the subtraction, but its
    /// quotient still costs $m$. The arctangent at that width dominates, and the magnitude of the
    /// input does not otherwise drive the cost.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with the given precision (which is the case unless $x$ is NaN, $|x|>1$, or $x$ is 1).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = Float::from(0.5).acos_prec_round(10, Floor);
    /// assert_eq!(c.to_string(), "1.0469");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::from(0.5).acos_prec_round(10, Ceiling);
    /// assert_eq!(c.to_string(), "1.0488");
    /// assert_eq!(o, Greater);
    ///
    /// // acos(1) is zero, exactly
    /// let (c, o) = Float::ONE.acos_prec_round(10, Exact);
    /// assert_eq!(c.to_string(), "0.0");
    /// assert_eq!(o, Equal);
    /// ```
    #[inline]
    pub fn acos_prec_round(self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        self.acos_prec_round_ref(prec, rm)
    }

    /// Computes $\arccos x$, the arccosine of a [`Float`], rounding the result to the specified
    /// precision and with the specified rounding mode. The [`Float`] is taken by reference. An
    /// [`Ordering`] is also returned, indicating whether the rounded arccosine is less than, equal
    /// to, or greater than the exact arccosine. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`Float::acos_prec_round`] for the error bounds, the special cases, and the complexity;
    /// this function behaves the same way.
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
    /// let (c, o) = (&Float::from(0.5)).acos_prec_round_ref(10, Floor);
    /// assert_eq!(c.to_string(), "1.0469");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = (&Float::from(0.5)).acos_prec_round_ref(10, Ceiling);
    /// assert_eq!(c.to_string(), "1.0488");
    /// assert_eq!(o, Greater);
    /// ```
    pub fn acos_prec_round_ref(&self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        assert_ne!(prec, 0);
        match &self.0 {
            // the arccosine is NaN outside [-1, 1], and both infinities are outside it
            NaN | Infinity { .. } => (Self::NAN, Equal),
            // acos(±0.0) = pi/2
            Zero { .. } => {
                let (pi, o) = Self::pi_prec_round(prec, rm);
                // exact
                (pi >> 1u32, o)
            }
            Finite { .. } => acos_prec_round_normal_ref(self, prec, rm),
        }
    }

    /// Computes $\arccos x$, the arccosine of a [`Float`], rounding the result to the nearest value
    /// of the specified precision. The [`Float`] is taken by value. An [`Ordering`] is also
    /// returned, indicating whether the rounded arccosine is less than, equal to, or greater than
    /// the exact arccosine. Although `NaN`s are not comparable to any [`Float`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// If the arccosine is equidistant from two [`Float`]s with the specified precision, the
    /// [`Float`] with fewer 1s in its binary expansion is chosen.
    ///
    /// See [`Float::acos_prec_round`] for the error bounds, the special cases, and the complexity;
    /// this function behaves the same way.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::acos_prec_round`] instead.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = Float::from(0.5).acos_prec(10);
    /// assert_eq!(c.to_string(), "1.0469");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::from(0.5).acos_prec(53);
    /// assert_eq!(c.to_string(), "1.0471975511965979");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn acos_prec(self, prec: u64) -> (Self, Ordering) {
        self.acos_prec_round(prec, Nearest)
    }

    /// Computes $\arccos x$, the arccosine of a [`Float`], rounding the result to the nearest value
    /// of the specified precision. The [`Float`] is taken by reference. An [`Ordering`] is also
    /// returned, indicating whether the rounded arccosine is less than, equal to, or greater than
    /// the exact arccosine. Although `NaN`s are not comparable to any [`Float`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`Float::acos_prec`] and [`Float::acos_prec_round`]; this function behaves the same way.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = (&Float::from(0.5)).acos_prec_ref(10);
    /// assert_eq!(c.to_string(), "1.0469");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = (&Float::from(0.5)).acos_prec_ref(53);
    /// assert_eq!(c.to_string(), "1.0471975511965979");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn acos_prec_ref(&self, prec: u64) -> (Self, Ordering) {
        self.acos_prec_round_ref(prec, Nearest)
    }

    /// Computes $\arccos x$, the arccosine of a [`Float`], rounding the result with the specified
    /// rounding mode. The [`Float`] is taken by value. An [`Ordering`] is also returned, indicating
    /// whether the rounded arccosine is less than, equal to, or greater than the exact arccosine.
    /// Although `NaN`s are not comparable to any [`Float`], whenever this function returns a `NaN`
    /// it also returns `Equal`.
    ///
    /// The precision of the output is the precision of the input. See [`RoundingMode`] for a
    /// description of the possible rounding modes.
    ///
    /// See [`Float::acos_prec_round`] for the error bounds and the special cases; this function
    /// behaves the same way, with $p$ the precision of the input.
    ///
    /// If you want to specify an output precision, consider using [`Float::acos_prec_round`]
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^3 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`: the
    /// arccosine is taken as $\pi/2-\arctan(x/\sqrt{1-x^2})$ at a working precision of about $n$
    /// plus the bits that cancel there, which an input within $2^{-n}$ of 1 pushes to another $2n$;
    /// the arctangent at that width dominates. The magnitude of the input does not otherwise drive
    /// the cost.
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
    /// let x = Float::from_unsigned_prec(1u32, 100).0 >> 1u32;
    /// let (c, o) = x.clone().acos_round(Floor);
    /// assert_eq!(c.to_string(), "1.0471975511965977461542144610921");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = x.acos_round(Ceiling);
    /// assert_eq!(c.to_string(), "1.0471975511965977461542144610936");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn acos_round(self, rm: RoundingMode) -> (Self, Ordering) {
        let prec = self.significant_bits();
        self.acos_prec_round(prec, rm)
    }

    /// Computes $\arccos x$, the arccosine of a [`Float`], rounding the result with the specified
    /// rounding mode. The [`Float`] is taken by reference. An [`Ordering`] is also returned,
    /// indicating whether the rounded arccosine is less than, equal to, or greater than the exact
    /// arccosine. Although `NaN`s are not comparable to any [`Float`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// See [`Float::acos_round`] and [`Float::acos_prec_round`]; this function behaves the same
    /// way.
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
    /// let x = Float::from_unsigned_prec(1u32, 100).0 >> 1u32;
    /// let (c, o) = (&x).acos_round_ref(Floor);
    /// assert_eq!(c.to_string(), "1.0471975511965977461542144610921");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn acos_round_ref(&self, rm: RoundingMode) -> (Self, Ordering) {
        self.acos_prec_round_ref(self.significant_bits(), rm)
    }

    /// Computes $\arccos x$, the arccosine of a [`Float`], in place, rounding the result to the
    /// specified precision and with the specified rounding mode. An [`Ordering`] is returned,
    /// indicating whether the rounded arccosine is less than, equal to, or greater than the exact
    /// arccosine. Although `NaN`s are not comparable to any [`Float`], whenever this function
    /// assigns a `NaN` it also returns `Equal`.
    ///
    /// See [`Float::acos_prec_round`] for the error bounds, the special cases, and the complexity;
    /// this function behaves the same way.
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
    /// let mut x = Float::from(0.5);
    /// let o = x.acos_prec_round_assign(10, Floor);
    /// assert_eq!(x.to_string(), "1.0469");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn acos_prec_round_assign(&mut self, prec: u64, rm: RoundingMode) -> Ordering {
        let (c, o) = self.acos_prec_round_ref(prec, rm);
        *self = c;
        o
    }

    /// Computes $\arccos x$, the arccosine of a [`Float`], in place, rounding the result to the
    /// nearest value of the specified precision. An [`Ordering`] is returned, indicating whether
    /// the rounded arccosine is less than, equal to, or greater than the exact arccosine. Although
    /// `NaN`s are not comparable to any [`Float`], whenever this function assigns a `NaN` it also
    /// returns `Equal`.
    ///
    /// See [`Float::acos_prec`] and [`Float::acos_prec_round`]; this function behaves the same way.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from(0.5);
    /// let o = x.acos_prec_assign(10);
    /// assert_eq!(x.to_string(), "1.0469");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn acos_prec_assign(&mut self, prec: u64) -> Ordering {
        self.acos_prec_round_assign(prec, Nearest)
    }

    /// Computes $\arccos x$, the arccosine of a [`Float`], in place, rounding the result with the
    /// specified rounding mode. The precision of the output is the precision of the input. An
    /// [`Ordering`] is returned, indicating whether the rounded arccosine is less than, equal to,
    /// or greater than the exact arccosine. Although `NaN`s are not comparable to any [`Float`],
    /// whenever this function assigns a `NaN` it also returns `Equal`.
    ///
    /// See [`Float::acos_round`] and [`Float::acos_prec_round`]; this function behaves the same
    /// way.
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
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0 >> 1u32;
    /// let o = x.acos_round_assign(Floor);
    /// assert_eq!(x.to_string(), "1.0471975511965977461542144610921");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn acos_round_assign(&mut self, rm: RoundingMode) -> Ordering {
        let prec = self.significant_bits();
        self.acos_prec_round_assign(prec, rm)
    }

    /// Computes $\arccos x$, the arccosine of a [`Rational`], rounding the result to the specified
    /// precision and with the specified rounding mode and returning the result as a [`Float`]. The
    /// [`Rational`] is taken by value. An [`Ordering`] is also returned, indicating whether the
    /// rounded arccosine is less than, equal to, or greater than the exact arccosine.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \arccos x+\varepsilon.
    /// $$
    /// - If the result is NaN, or if $x$ is 1, $\varepsilon$ may be ignored or assumed to be 0.
    /// - Otherwise, if $m$ is not `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2 |\arccos
    ///   x|\rfloor-p+1}$.
    /// - Otherwise, if $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2 |\arccos
    ///   x|\rfloor-p}$.
    ///
    /// The output has precision `prec`.
    ///
    /// Special cases:
    /// - $f(x,p,m)=\text{NaN}$ for $|x|>1$
    /// - $f(0,p,m)=\pi/2$, rounded
    /// - $f(1,p,m)=0.0$
    /// - $f(-1,p,m)=\pi$, rounded
    ///
    /// The zero at $x=1$ is the only exact case.
    ///
    /// Underflow:
    /// - If $0<f(x,p,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,p,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,p,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,p,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is returned
    ///   instead.
    ///
    /// Overflow is not possible, since the result lies in $[0,\pi]$. Underflow, which the [`Float`]
    /// arccosine cannot reach, is possible here: a [`Rational`] may lie within $2^{-2^{31}}$ of 1,
    /// and there $\arccos x$ is about $\sqrt{2(1-x)}$, which is below the smallest positive
    /// [`Float`].
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::acos_rational_prec`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `x.significant_bits()`: $(1-x^2)/x^2$ is formed exactly, and its square root and arctangent
    /// are taken at a working precision of about $n$ bits, which costs the first term; the second
    /// covers the $m$-bit input. The magnitude of the input does not drive the cost, and unlike the
    /// [`Float`] arccosine neither does its closeness to $\pm1$, since nothing cancels.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with the given precision (which is the case unless $|x|>1$ or $x$ is 1).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = Float::acos_rational_prec_round(Rational::from_unsigneds(3u8, 5), 10, Floor);
    /// assert_eq!(c.to_string(), "0.92676");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::acos_rational_prec_round(Rational::from_unsigneds(3u8, 5), 10, Ceiling);
    /// assert_eq!(c.to_string(), "0.92773");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = Float::acos_rational_prec_round(Rational::from_signeds(-3i8, 5), 10, Nearest);
    /// assert_eq!(c.to_string(), "2.2148");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    #[allow(clippy::needless_pass_by_value)]
    pub fn acos_rational_prec_round(x: Rational, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        Self::acos_rational_prec_round_ref(&x, prec, rm)
    }

    /// Computes $\arccos x$, the arccosine of a [`Rational`], rounding the result to the specified
    /// precision and with the specified rounding mode and returning the result as a [`Float`]. The
    /// [`Rational`] is taken by reference. An [`Ordering`] is also returned, indicating whether the
    /// rounded arccosine is less than, equal to, or greater than the exact arccosine.
    ///
    /// See [`Float::acos_rational_prec_round`] for the error bounds, the special cases, underflow,
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
    /// let (c, o) =
    ///     Float::acos_rational_prec_round_ref(&Rational::from_unsigneds(3u8, 5), 10, Floor);
    /// assert_eq!(c.to_string(), "0.92676");
    /// assert_eq!(o, Less);
    ///
    /// // acos(1) is zero, exactly
    /// let (c, o) = Float::acos_rational_prec_round_ref(&Rational::ONE, 10, Exact);
    /// assert_eq!(c.to_string(), "0.0");
    /// assert_eq!(o, Equal);
    /// ```
    pub fn acos_rational_prec_round_ref(
        x: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        assert_ne!(prec, 0);
        // acos(0) = pi/2
        if *x == 0u32 {
            let (pi, o) = Self::pi_prec_round(prec, rm);
            // exact
            return (pi >> 1u32, o);
        }
        match x.partial_cmp_abs(&1u32).unwrap() {
            // the arccosine is NaN outside [-1, 1]
            Greater => (Self::NAN, Equal),
            // acos(1) = +0, exactly, and acos(-1) = pi
            Equal => {
                if *x > 0u32 {
                    (Self::ZERO, Equal)
                } else {
                    Self::pi_prec_round(prec, rm)
                }
            }
            Less => acos_rational_helper(x, prec, rm),
        }
    }

    /// Computes $\arccos x$, the arccosine of a [`Rational`], rounding the result to the nearest
    /// value of the specified precision and returning the result as a [`Float`]. The [`Rational`]
    /// is taken by value. An [`Ordering`] is also returned, indicating whether the rounded
    /// arccosine is less than, equal to, or greater than the exact arccosine.
    ///
    /// If the arccosine is equidistant from two [`Float`]s with the specified precision, the
    /// [`Float`] with fewer 1s in its binary expansion is chosen.
    ///
    /// See [`Float::acos_rational_prec_round`] for the error bounds, the special cases, underflow,
    /// and the complexity; this function behaves the same way.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::acos_rational_prec_round`] instead.
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
    /// let (c, o) = Float::acos_rational_prec(Rational::from_unsigneds(3u8, 5), 10);
    /// assert_eq!(c.to_string(), "0.92773");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = Float::acos_rational_prec(Rational::from_unsigneds(3u8, 5), 53);
    /// assert_eq!(c.to_string(), "0.92729521800161219");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn acos_rational_prec(x: Rational, prec: u64) -> (Self, Ordering) {
        Self::acos_rational_prec_round(x, prec, Nearest)
    }

    /// Computes $\arccos x$, the arccosine of a [`Rational`], rounding the result to the nearest
    /// value of the specified precision and returning the result as a [`Float`]. The [`Rational`]
    /// is taken by reference. An [`Ordering`] is also returned, indicating whether the rounded
    /// arccosine is less than, equal to, or greater than the exact arccosine.
    ///
    /// See [`Float::acos_rational_prec`] and [`Float::acos_rational_prec_round`]; this function
    /// behaves the same way.
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
    /// let (c, o) = Float::acos_rational_prec_ref(&Rational::from_unsigneds(3u8, 5), 53);
    /// assert_eq!(c.to_string(), "0.92729521800161219");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn acos_rational_prec_ref(x: &Rational, prec: u64) -> (Self, Ordering) {
        Self::acos_rational_prec_round_ref(x, prec, Nearest)
    }

    /// Computes $\arccos(x)u/(2\pi)$, the arccosine of a [`Float`] measured in $u$ths of a turn,
    /// rounding the result to the specified precision and with the specified rounding mode. The
    /// [`Float`] is taken by value. An [`Ordering`] is also returned, indicating whether the
    /// rounded arccosine is less than, equal to, or greater than the exact arccosine. Although
    /// `NaN`s are not comparable to any [`Float`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,u,p,m) = \arccos(x)u/(2\pi)+\varepsilon.
    /// $$
    /// - If $x$ is NaN, if $|x|>1$, if $u = 0$, if $x$ is zero, if $|x|$ is 1, or if $|x|$ is $1/2$
    ///   and $u$ is a multiple of 3, $\varepsilon$ may be ignored or assumed to be 0.
    /// - Otherwise, if $m$ is not `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2
    ///   |\arccos(x)u/(2\pi)|\rfloor-p+1}$.
    /// - Otherwise, if $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\arccos(x)u/(2\pi)|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},u,p,m)=f(\pm\infty,u,p,m)=\text{NaN}$
    /// - $f(x,u,p,m)=\text{NaN}$ for $|x|>1$, including when $u=0$
    /// - $f(\pm0.0,u,p,m)=u/4$, a quarter turn
    /// - $f(x,0,p,m)=0.0$, since the arccosine is never negative
    /// - $f(1,u,p,m)=0.0$
    /// - $f(-1,u,p,m)=u/2$, a half turn
    /// - $f(1/2,u,p,m)=u/6$ and $f(-1/2,u,p,m)=u/3$, a sixth and a third of a turn, when $u$ is a
    ///   multiple of 3
    ///
    /// Those are the only exact cases, and the turn fractions are exact only when $p$ is large
    /// enough to hold them.
    ///
    /// Underflow:
    /// - If $0<f(x,u,p,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,u,p,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,u,p,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,u,p,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is returned
    ///   instead.
    ///
    /// Overflow is not possible, since $f(x,u,p,m) \leq u/2 < 2^{63}$. Underflow needs a small $u$
    /// together with an $x$ within $2^{-2^{31}}$ of 1, which takes a precision of more than
    /// $2^{31}$ bits; the arccosine itself cannot underflow.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::acos_with_period_prec`]
    /// instead. If you know that your target precision is the precision of the input, consider
    /// using [`Float::acos_with_period_round`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O((n+m) (\log (n+m))^3 \log\log (n+m))$
    ///
    /// $M(n, m) = O((n+m) \log (n+m))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `self.significant_bits()`: the arccosine is taken at a working precision of about $n$ plus
    /// the bits that cancel there, which an input within $2^{-m}$ of 1 pushes to $2m$, and is then
    /// scaled by $u/(2\pi)$, which needs $\pi$ to that many bits; the arccosine dominates.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with the given precision.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// // a zero input is a quarter turn, and an input of 1/2 a sixth of one
    /// let (c, o) = Float::ZERO.acos_with_period_prec_round(360, 10, Exact);
    /// assert_eq!(c.to_string(), "90.000");
    /// assert_eq!(o, Equal);
    ///
    /// let (c, o) = Float::from(0.5).acos_with_period_prec_round(360, 10, Exact);
    /// assert_eq!(c.to_string(), "60.000");
    /// assert_eq!(o, Equal);
    ///
    /// let (c, o) = Float::from(0.25).acos_with_period_prec_round(360, 10, Floor);
    /// assert_eq!(c.to_string(), "75.500");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::from(0.25).acos_with_period_prec_round(360, 10, Ceiling);
    /// assert_eq!(c.to_string(), "75.625");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn acos_with_period_prec_round(
        self,
        u: u64,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        self.acos_with_period_prec_round_ref(u, prec, rm)
    }

    /// Computes $\arccos(x)u/(2\pi)$, the arccosine of a [`Float`] measured in $u$ths of a turn,
    /// rounding the result to the specified precision and with the specified rounding mode. The
    /// [`Float`] is taken by reference. An [`Ordering`] is also returned, indicating whether the
    /// rounded arccosine is less than, equal to, or greater than the exact arccosine. Although
    /// `NaN`s are not comparable to any [`Float`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// See [`Float::acos_with_period_prec_round`] for the error bounds, the special and closed-form
    /// cases, underflow, and the complexity; this function behaves the same way.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with the given precision.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::NegativeOne;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// // an input of -1 is a half turn
    /// let (c, o) = (&Float::NEGATIVE_ONE).acos_with_period_prec_round_ref(360, 10, Exact);
    /// assert_eq!(c.to_string(), "180.00");
    /// assert_eq!(o, Equal);
    ///
    /// let (c, o) = (&Float::from(0.25)).acos_with_period_prec_round_ref(360, 10, Floor);
    /// assert_eq!(c.to_string(), "75.500");
    /// assert_eq!(o, Less);
    /// ```
    pub fn acos_with_period_prec_round_ref(
        &self,
        u: u64,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        assert_ne!(prec, 0);
        match &self.0 {
            // the arccosine is NaN outside [-1, 1], and both infinities are outside it; this holds
            // for u = 0 too, since NaN times 0 is NaN
            NaN | Infinity { .. } => (Self::NAN, Equal),
            // acos(±0.0) = pi/2, so acosu(±0.0, u) = u/4, which is zero when u is
            Zero { .. } => scaled_unsigned(u, 2, true, prec, rm),
            Finite { .. } => {
                if self.gt_abs(&1u32) {
                    (Self::NAN, Equal)
                } else if u == 0 {
                    // acosu(x, 0) = +0, since the arccosine is never negative
                    (Self::ZERO, Equal)
                } else {
                    acos_with_period_prec_round_normal_ref(self, u, prec, rm)
                }
            }
        }
    }

    /// Computes $\arccos(x)u/(2\pi)$, the arccosine of a [`Float`] measured in $u$ths of a turn,
    /// rounding the result to the nearest value of the specified precision. The [`Float`] is taken
    /// by value. An [`Ordering`] is also returned, indicating whether the rounded arccosine is less
    /// than, equal to, or greater than the exact arccosine. Although `NaN`s are not comparable to
    /// any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// If the arccosine is equidistant from two [`Float`]s with the specified precision, the
    /// [`Float`] with fewer 1s in its binary expansion is chosen.
    ///
    /// See [`Float::acos_with_period_prec_round`] for the error bounds, the special and closed-form
    /// cases, underflow, and the complexity; this function behaves the same way.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::acos_with_period_prec_round`] instead.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = Float::from(0.25).acos_with_period_prec(360, 10);
    /// assert_eq!(c.to_string(), "75.500");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::from(0.25).acos_with_period_prec(360, 53);
    /// assert_eq!(c.to_string(), "75.522487814070075");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn acos_with_period_prec(self, u: u64, prec: u64) -> (Self, Ordering) {
        self.acos_with_period_prec_round(u, prec, Nearest)
    }

    /// Computes $\arccos(x)u/(2\pi)$, the arccosine of a [`Float`] measured in $u$ths of a turn,
    /// rounding the result to the nearest value of the specified precision. The [`Float`] is taken
    /// by reference. An [`Ordering`] is also returned, indicating whether the rounded arccosine is
    /// less than, equal to, or greater than the exact arccosine. Although `NaN`s are not comparable
    /// to any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`Float::acos_with_period_prec`] and [`Float::acos_with_period_prec_round`]; this
    /// function behaves the same way.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = (&Float::from(0.25)).acos_with_period_prec_ref(360, 10);
    /// assert_eq!(c.to_string(), "75.500");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn acos_with_period_prec_ref(&self, u: u64, prec: u64) -> (Self, Ordering) {
        self.acos_with_period_prec_round_ref(u, prec, Nearest)
    }

    /// Computes $\arccos(x)u/(2\pi)$, the arccosine of a [`Float`] measured in $u$ths of a turn,
    /// rounding the result with the specified rounding mode. The precision of the output is the
    /// precision of the input. The [`Float`] is taken by value. An [`Ordering`] is also returned,
    /// indicating whether the rounded arccosine is less than, equal to, or greater than the exact
    /// arccosine. Although `NaN`s are not comparable to any [`Float`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// See [`Float::acos_with_period_prec_round`] for the error bounds, the special and closed-form
    /// cases, underflow, and the complexity; this function behaves the same way.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::acos_with_period_prec_round`] instead.
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
    /// let (c, o) = x.acos_with_period_round(360, Floor);
    /// assert_eq!(c.to_string(), "75.500");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn acos_with_period_round(self, u: u64, rm: RoundingMode) -> (Self, Ordering) {
        let prec = self.significant_bits();
        self.acos_with_period_prec_round(u, prec, rm)
    }

    /// Computes $\arccos(x)u/(2\pi)$, the arccosine of a [`Float`] measured in $u$ths of a turn,
    /// rounding the result with the specified rounding mode. The precision of the output is the
    /// precision of the input. The [`Float`] is taken by reference. An [`Ordering`] is also
    /// returned, indicating whether the rounded arccosine is less than, equal to, or greater than
    /// the exact arccosine. Although `NaN`s are not comparable to any [`Float`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`Float::acos_with_period_round`] and [`Float::acos_with_period_prec_round`]; this
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
    /// let (c, o) = (&x).acos_with_period_round_ref(360, Floor);
    /// assert_eq!(c.to_string(), "75.500");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn acos_with_period_round_ref(&self, u: u64, rm: RoundingMode) -> (Self, Ordering) {
        self.acos_with_period_prec_round_ref(u, self.significant_bits(), rm)
    }

    /// Computes $\arccos(x)u/(2\pi)$, the arccosine of a [`Float`] measured in $u$ths of a turn,
    /// rounding the result to the nearest value of the input's precision. The [`Float`] is taken by
    /// value.
    ///
    /// If the arccosine is equidistant from two [`Float`]s with the specified precision, the
    /// [`Float`] with fewer 1s in its binary expansion is chosen.
    ///
    /// See [`Float::acos_with_period_prec_round`] for the error bounds, the special and closed-form
    /// cases, underflow, and the complexity; this function behaves the same way.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::acos_with_period_round`] instead. If you want to specify an output precision,
    /// consider using [`Float::acos_with_period_prec`]. If you want both of these things, consider
    /// using [`Float::acos_with_period_prec_round`].
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    ///
    /// let x = Float::from_unsigned_prec(1u32, 10).0 >> 2u32;
    /// assert_eq!(x.acos_with_period(360).to_string(), "75.500");
    /// ```
    #[inline]
    pub fn acos_with_period(self, u: u64) -> Self {
        let prec = self.significant_bits();
        self.acos_with_period_prec(u, prec).0
    }

    /// Computes $\arccos(x)u/(2\pi)$, the arccosine of a [`Float`] measured in $u$ths of a turn,
    /// rounding the result to the nearest value of the input's precision. The [`Float`] is taken by
    /// reference.
    ///
    /// See [`Float::acos_with_period`] and [`Float::acos_with_period_prec_round`]; this function
    /// behaves the same way.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    ///
    /// let x = Float::from_unsigned_prec(1u32, 10).0 >> 2u32;
    /// assert_eq!((&x).acos_with_period_ref(360).to_string(), "75.500");
    /// ```
    #[inline]
    pub fn acos_with_period_ref(&self, u: u64) -> Self {
        self.acos_with_period_prec_ref(u, self.significant_bits()).0
    }

    /// Computes $\arccos(x)u/(2\pi)$, the arccosine of a [`Float`] measured in $u$ths of a turn, in
    /// place, rounding the result to the specified precision and with the specified rounding mode.
    /// An [`Ordering`] is returned, indicating whether the rounded arccosine is less than, equal
    /// to, or greater than the exact arccosine. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function assigns a `NaN` it also returns `Equal`.
    ///
    /// See [`Float::acos_with_period_prec_round`] for the error bounds, the special and closed-form
    /// cases, underflow, and the complexity; this function behaves the same way.
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
    /// let mut x = Float::from(0.25);
    /// let o = x.acos_with_period_prec_round_assign(360, 10, Floor);
    /// assert_eq!(x.to_string(), "75.500");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn acos_with_period_prec_round_assign(
        &mut self,
        u: u64,
        prec: u64,
        rm: RoundingMode,
    ) -> Ordering {
        let (c, o) = self.acos_with_period_prec_round_ref(u, prec, rm);
        *self = c;
        o
    }

    /// Computes $\arccos(x)u/(2\pi)$, the arccosine of a [`Float`] measured in $u$ths of a turn, in
    /// place, rounding the result to the nearest value of the specified precision. An [`Ordering`]
    /// is returned, indicating whether the rounded arccosine is less than, equal to, or greater
    /// than the exact arccosine. Although `NaN`s are not comparable to any [`Float`], whenever this
    /// function assigns a `NaN` it also returns `Equal`.
    ///
    /// See [`Float::acos_with_period_prec`] and [`Float::acos_with_period_prec_round`]; this
    /// function behaves the same way.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from(0.25);
    /// let o = x.acos_with_period_prec_assign(360, 10);
    /// assert_eq!(x.to_string(), "75.500");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn acos_with_period_prec_assign(&mut self, u: u64, prec: u64) -> Ordering {
        self.acos_with_period_prec_round_assign(u, prec, Nearest)
    }

    /// Computes $\arccos(x)u/(2\pi)$, the arccosine of a [`Float`] measured in $u$ths of a turn, in
    /// place, rounding the result with the specified rounding mode. The precision of the output is
    /// the precision of the input. An [`Ordering`] is returned, indicating whether the rounded
    /// arccosine is less than, equal to, or greater than the exact arccosine. Although `NaN`s are
    /// not comparable to any [`Float`], whenever this function assigns a `NaN` it also returns
    /// `Equal`.
    ///
    /// See [`Float::acos_with_period_round`] and [`Float::acos_with_period_prec_round`]; this
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
    /// let o = x.acos_with_period_round_assign(360, Floor);
    /// assert_eq!(x.to_string(), "75.500");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn acos_with_period_round_assign(&mut self, u: u64, rm: RoundingMode) -> Ordering {
        let prec = self.significant_bits();
        self.acos_with_period_prec_round_assign(u, prec, rm)
    }

    /// Computes $\arccos(x)u/(2\pi)$, the arccosine of a [`Float`] measured in $u$ths of a turn, in
    /// place, rounding the result to the nearest value of the input's precision.
    ///
    /// If the arccosine is equidistant from two [`Float`]s with the specified precision, the
    /// [`Float`] with fewer 1s in its binary expansion is chosen.
    ///
    /// See [`Float::acos_with_period_prec_round`] for the error bounds, the special and closed-form
    /// cases, underflow, and the complexity; this function behaves the same way.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::acos_with_period_round_assign`] instead. If you want to specify an output
    /// precision, consider using [`Float::acos_with_period_prec_assign`]. If you want both of these
    /// things, consider using [`Float::acos_with_period_prec_round_assign`].
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 10).0 >> 2u32;
    /// x.acos_with_period_assign(360);
    /// assert_eq!(x.to_string(), "75.500");
    /// ```
    #[inline]
    pub fn acos_with_period_assign(&mut self, u: u64) {
        let prec = self.significant_bits();
        self.acos_with_period_prec_assign(u, prec);
    }

    /// Computes $\arccos(x)u/(2\pi)$, the arccosine of a [`Rational`] measured in $u$ths of a turn,
    /// rounding the result to the specified precision and with the specified rounding mode and
    /// returning the result as a [`Float`]. The [`Rational`] is taken by value. An [`Ordering`] is
    /// also returned, indicating whether the rounded arccosine is less than, equal to, or greater
    /// than the exact arccosine.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,u,p,m) = \arccos(x)u/(2\pi)+\varepsilon.
    /// $$
    /// - If $|x|>1$, if $u = 0$, if $x$ is zero, if $|x|$ is 1, or if $|x|$ is $1/2$ and $u$ is a
    ///   multiple of 3, $\varepsilon$ may be ignored or assumed to be 0.
    /// - Otherwise, if $m$ is not `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2
    ///   |\arccos(x)u/(2\pi)|\rfloor-p+1}$.
    /// - Otherwise, if $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\arccos(x)u/(2\pi)|\rfloor-p}$.
    ///
    /// The output has precision `prec`.
    ///
    /// Special cases:
    /// - $f(x,u,p,m)=\text{NaN}$ for $|x|>1$, including when $u=0$
    /// - $f(0,u,p,m)=u/4$, a quarter turn
    /// - $f(x,0,p,m)=0.0$, since the arccosine is never negative
    /// - $f(1,u,p,m)=0.0$
    /// - $f(-1,u,p,m)=u/2$, a half turn
    /// - $f(1/2,u,p,m)=u/6$ and $f(-1/2,u,p,m)=u/3$, a sixth and a third of a turn, when $u$ is a
    ///   multiple of 3
    ///
    /// Those are the only exact cases, and the turn fractions are exact only when $p$ is large
    /// enough to hold them.
    ///
    /// Underflow:
    /// - If $0<f(x,u,p,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,u,p,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,u,p,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,u,p,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is returned
    ///   instead.
    ///
    /// Overflow is not possible, since $f(x,u,p,m) \leq u/2 < 2^{63}$. Underflow needs a small $u$
    /// together with an $x$ within about $2^{-2^{31}}$ of 1; unlike the [`Float`] case, a
    /// [`Rational`] can be that close.
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`Float::acos_with_period_rational_prec`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `x.significant_bits()`: $(1-x^2)/x^2$ is formed exactly, and its square root and arctangent
    /// are taken at a working precision of about $n$ bits and scaled by $u/(2\pi)$, which needs
    /// $\pi$ to that many bits; those cost the first term, and the second covers the $m$-bit input.
    /// The magnitude of the input does not drive the cost, and unlike the [`Float`] arccosine
    /// neither does its closeness to $\pm1$, since nothing cancels.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with the given precision.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// // a zero input is a quarter turn
    /// let (c, o) = Float::acos_with_period_rational_prec_round(Rational::ZERO, 360, 10, Exact);
    /// assert_eq!(c.to_string(), "90.000");
    /// assert_eq!(o, Equal);
    ///
    /// let (c, o) = Float::acos_with_period_rational_prec_round(
    ///     Rational::from_unsigneds(3u8, 5),
    ///     360,
    ///     10,
    ///     Floor,
    /// );
    /// assert_eq!(c.to_string(), "53.125");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::acos_with_period_rational_prec_round(
    ///     Rational::from_unsigneds(3u8, 5),
    ///     360,
    ///     10,
    ///     Ceiling,
    /// );
    /// assert_eq!(c.to_string(), "53.188");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    #[allow(clippy::needless_pass_by_value)]
    pub fn acos_with_period_rational_prec_round(
        x: Rational,
        u: u64,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        Self::acos_with_period_rational_prec_round_ref(&x, u, prec, rm)
    }

    /// Computes $\arccos(x)u/(2\pi)$, the arccosine of a [`Rational`] measured in $u$ths of a turn,
    /// rounding the result to the specified precision and with the specified rounding mode and
    /// returning the result as a [`Float`]. The [`Rational`] is taken by reference. An [`Ordering`]
    /// is also returned, indicating whether the rounded arccosine is less than, equal to, or
    /// greater than the exact arccosine.
    ///
    /// See [`Float::acos_with_period_rational_prec_round`] for the error bounds, the special and
    /// closed-form cases, underflow, and the complexity; this function behaves the same way.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with the given precision.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::NegativeOne;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// // an input of -1 is a half turn
    /// let (c, o) = Float::acos_with_period_rational_prec_round_ref(
    ///     &Rational::NEGATIVE_ONE,
    ///     360,
    ///     10,
    ///     Exact,
    /// );
    /// assert_eq!(c.to_string(), "180.00");
    /// assert_eq!(o, Equal);
    ///
    /// let (c, o) = Float::acos_with_period_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(3u8, 5),
    ///     360,
    ///     10,
    ///     Floor,
    /// );
    /// assert_eq!(c.to_string(), "53.125");
    /// assert_eq!(o, Less);
    /// ```
    pub fn acos_with_period_rational_prec_round_ref(
        x: &Rational,
        u: u64,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        assert_ne!(prec, 0);
        if x.gt_abs(&1u32) {
            // acosu(x, u) = NaN for |x| > 1, including for u = 0, since NaN times 0 is NaN
            return (Self::NAN, Equal);
        }
        if u == 0 {
            // acosu(x, 0) = +0, since the arccosine is never negative
            return (Self::ZERO, Equal);
        }
        if *x == 0u32 {
            // acos(0) = pi/2, so acosu(0, u) = u/4
            return scaled_unsigned(u, 2, true, prec, rm);
        }
        acos_with_period_rational_helper(x, u, prec, rm)
    }

    /// Computes $\arccos(x)u/(2\pi)$, the arccosine of a [`Rational`] measured in $u$ths of a turn,
    /// rounding the result to the nearest value of the specified precision and returning the result
    /// as a [`Float`]. The [`Rational`] is taken by value. An [`Ordering`] is also returned,
    /// indicating whether the rounded arccosine is less than, equal to, or greater than the exact
    /// arccosine.
    ///
    /// If the arccosine is equidistant from two [`Float`]s with the specified precision, the
    /// [`Float`] with fewer 1s in its binary expansion is chosen.
    ///
    /// See [`Float::acos_with_period_rational_prec_round`] for the error bounds, the special and
    /// closed-form cases, underflow, and the complexity; this function behaves the same way.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::acos_with_period_rational_prec_round`] instead.
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
    /// let (c, o) =
    ///     Float::acos_with_period_rational_prec(Rational::from_unsigneds(3u8, 5), 360, 10);
    /// assert_eq!(c.to_string(), "53.125");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) =
    ///     Float::acos_with_period_rational_prec(Rational::from_unsigneds(3u8, 5), 360, 53);
    /// assert_eq!(c.to_string(), "53.130102354155980");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn acos_with_period_rational_prec(x: Rational, u: u64, prec: u64) -> (Self, Ordering) {
        Self::acos_with_period_rational_prec_round(x, u, prec, Nearest)
    }

    /// Computes $\arccos(x)u/(2\pi)$, the arccosine of a [`Rational`] measured in $u$ths of a turn,
    /// rounding the result to the nearest value of the specified precision and returning the result
    /// as a [`Float`]. The [`Rational`] is taken by reference. An [`Ordering`] is also returned,
    /// indicating whether the rounded arccosine is less than, equal to, or greater than the exact
    /// arccosine.
    ///
    /// See [`Float::acos_with_period_rational_prec`] and
    /// [`Float::acos_with_period_rational_prec_round`]; this function behaves the same way.
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
    /// let (c, o) =
    ///     Float::acos_with_period_rational_prec_ref(&Rational::from_unsigneds(3u8, 5), 360, 53);
    /// assert_eq!(c.to_string(), "53.130102354155980");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn acos_with_period_rational_prec_ref(x: &Rational, u: u64, prec: u64) -> (Self, Ordering) {
        Self::acos_with_period_rational_prec_round_ref(x, u, prec, Nearest)
    }

    /// Computes $\arccos(x)/\pi$, the arccosine of a [`Float`] measured in half-turns, rounding the
    /// result to the specified precision and with the specified rounding mode. The [`Float`] is
    /// taken by value. An [`Ordering`] is also returned, indicating whether the rounded arccosine
    /// is less than, equal to, or greater than the exact arccosine. Although `NaN`s are not
    /// comparable to any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// This is `acos_with_period` with a period of 2: see [`Float::acos_with_period_prec_round`]
    /// for the error bounds, the special cases, underflow, and the complexity, with $u = 2$. A zero
    /// input gives $1/2$, an input of 1 gives $0.0$, and an input of $-1$ gives $1$; all three are
    /// exact at every precision, since a half and a one need only one bit, and they are the only
    /// exact cases. NaN, either infinity, and any $|x|>1$ give NaN. Overflow is not possible, since
    /// $0 \leq \arccos(x)/\pi \leq 1$.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with the given precision.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// // a zero input is half a half-turn
    /// let (c, o) = Float::ZERO.acos_pi_prec_round(10, Exact);
    /// assert_eq!(c.to_string(), "0.50000");
    /// assert_eq!(o, Equal);
    /// ```
    #[inline]
    pub fn acos_pi_prec_round(self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        self.acos_with_period_prec_round(2, prec, rm)
    }

    /// Computes $\arccos(x)/\pi$, the arccosine of a [`Float`] measured in half-turns, rounding the
    /// result to the specified precision and with the specified rounding mode. The [`Float`] is
    /// taken by reference. An [`Ordering`] is also returned, indicating whether the rounded
    /// arccosine is less than, equal to, or greater than the exact arccosine. Although `NaN`s are
    /// not comparable to any [`Float`], whenever this function returns a `NaN` it also returns
    /// `Equal`.
    ///
    /// This is `acos_with_period` with a period of 2: see
    /// [`Float::acos_with_period_prec_round_ref`] for the error bounds, the special cases,
    /// underflow, and the complexity, with $u = 2$. A zero input gives $1/2$, an input of 1 gives
    /// $0.0$, and an input of $-1$ gives $1$; all three are exact at every precision, since a half
    /// and a one need only one bit, and they are the only exact cases. NaN, either infinity, and
    /// any $|x|>1$ give NaN. Overflow is not possible, since $0 \leq \arccos(x)/\pi \leq 1$.
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
    /// let (c, o) = (&Float::from(0.25)).acos_pi_prec_round_ref(10, Floor);
    /// assert_eq!(c.to_string(), "0.41943");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn acos_pi_prec_round_ref(&self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        self.acos_with_period_prec_round_ref(2, prec, rm)
    }

    /// Computes $\arccos(x)/\pi$, the arccosine of a [`Float`] measured in half-turns, rounding the
    /// result to the nearest value of the specified precision. The [`Float`] is taken by value. An
    /// [`Ordering`] is also returned, indicating whether the rounded arccosine is less than, equal
    /// to, or greater than the exact arccosine. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// This is `acos_with_period` with a period of 2: see [`Float::acos_with_period_prec`] for the
    /// error bounds, the special cases, underflow, and the complexity, with $u = 2$. A zero input
    /// gives $1/2$, an input of 1 gives $0.0$, and an input of $-1$ gives $1$; all three are exact
    /// at every precision, since a half and a one need only one bit, and they are the only exact
    /// cases. NaN, either infinity, and any $|x|>1$ give NaN. Overflow is not possible, since $0
    /// \leq \arccos(x)/\pi \leq 1$.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = Float::from(0.25).acos_pi_prec(10);
    /// assert_eq!(c.to_string(), "0.41943");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn acos_pi_prec(self, prec: u64) -> (Self, Ordering) {
        self.acos_with_period_prec(2, prec)
    }

    /// Computes $\arccos(x)/\pi$, the arccosine of a [`Float`] measured in half-turns, rounding the
    /// result to the nearest value of the specified precision. The [`Float`] is taken by reference.
    /// An [`Ordering`] is also returned, indicating whether the rounded arccosine is less than,
    /// equal to, or greater than the exact arccosine. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// This is `acos_with_period` with a period of 2: see [`Float::acos_with_period_prec_ref`] for
    /// the error bounds, the special cases, underflow, and the complexity, with $u = 2$. A zero
    /// input gives $1/2$, an input of 1 gives $0.0$, and an input of $-1$ gives $1$; all three are
    /// exact at every precision, since a half and a one need only one bit, and they are the only
    /// exact cases. NaN, either infinity, and any $|x|>1$ give NaN. Overflow is not possible, since
    /// $0 \leq \arccos(x)/\pi \leq 1$.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = (&Float::from(0.25)).acos_pi_prec_ref(53);
    /// assert_eq!(c.to_string(), "0.41956937674483374");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn acos_pi_prec_ref(&self, prec: u64) -> (Self, Ordering) {
        self.acos_with_period_prec_ref(2, prec)
    }

    /// Computes $\arccos(x)/\pi$, the arccosine of a [`Float`] measured in half-turns, rounding the
    /// result with the specified rounding mode. The precision of the output is the precision of the
    /// input. The [`Float`] is taken by value. An [`Ordering`] is also returned, indicating whether
    /// the rounded arccosine is less than, equal to, or greater than the exact arccosine. Although
    /// `NaN`s are not comparable to any [`Float`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// This is `acos_with_period` with a period of 2: see [`Float::acos_with_period_round`] for the
    /// error bounds, the special cases, underflow, and the complexity, with $u = 2$. A zero input
    /// gives $1/2$, an input of 1 gives $0.0$, and an input of $-1$ gives $1$; all three are exact
    /// at every precision, since a half and a one need only one bit, and they are the only exact
    /// cases. NaN, either infinity, and any $|x|>1$ give NaN. Overflow is not possible, since $0
    /// \leq \arccos(x)/\pi \leq 1$.
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
    /// let (c, o) = x.acos_pi_round(Floor);
    /// assert_eq!(c.to_string(), "0.41943");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn acos_pi_round(self, rm: RoundingMode) -> (Self, Ordering) {
        self.acos_with_period_round(2, rm)
    }

    /// Computes $\arccos(x)/\pi$, the arccosine of a [`Float`] measured in half-turns, rounding the
    /// result with the specified rounding mode. The precision of the output is the precision of the
    /// input. The [`Float`] is taken by reference. An [`Ordering`] is also returned, indicating
    /// whether the rounded arccosine is less than, equal to, or greater than the exact arccosine.
    /// Although `NaN`s are not comparable to any [`Float`], whenever this function returns a `NaN`
    /// it also returns `Equal`.
    ///
    /// This is `acos_with_period` with a period of 2: see [`Float::acos_with_period_round_ref`] for
    /// the error bounds, the special cases, underflow, and the complexity, with $u = 2$. A zero
    /// input gives $1/2$, an input of 1 gives $0.0$, and an input of $-1$ gives $1$; all three are
    /// exact at every precision, since a half and a one need only one bit, and they are the only
    /// exact cases. NaN, either infinity, and any $|x|>1$ give NaN. Overflow is not possible, since
    /// $0 \leq \arccos(x)/\pi \leq 1$.
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
    /// let (c, o) = (&x).acos_pi_round_ref(Ceiling);
    /// assert_eq!(c.to_string(), "0.41992");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn acos_pi_round_ref(&self, rm: RoundingMode) -> (Self, Ordering) {
        self.acos_with_period_round_ref(2, rm)
    }

    /// Computes $\arccos(x)/\pi$, the arccosine of a [`Float`] measured in half-turns, rounding the
    /// result to the precision of the input and to the nearest [`Float`]. The [`Float`] is taken by
    /// value.
    ///
    /// If the arccosine is equidistant from two [`Float`]s with the precision of the input, the
    /// [`Float`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a
    /// description of the `Nearest` rounding mode.
    ///
    /// This is `acos_with_period` with a period of 2: see [`Float::acos_with_period`] for the error
    /// bounds, the special cases, underflow, and the complexity, with $u = 2$. A zero input gives
    /// $1/2$, an input of 1 gives $0.0$, and an input of $-1$ gives $1$; all three are exact at
    /// every precision, and they are the only exact cases. NaN, either infinity, and any $|x|>1$
    /// give NaN. Overflow is not possible, since $0 \leq \arccos(x)/\pi \leq 1$.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::acos_pi_round`] instead. If you want to specify an output precision, consider using
    /// [`Float::acos_pi_prec`]. If you want both of these things, consider using
    /// [`Float::acos_pi_prec_round`].
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    ///
    /// let x = Float::from_unsigned_prec(1u32, 10).0 >> 2u32;
    /// assert_eq!(x.acos_pi().to_string(), "0.41943");
    /// ```
    #[inline]
    pub fn acos_pi(self) -> Self {
        self.acos_with_period(2)
    }

    /// Computes $\arccos(x)/\pi$, the arccosine of a [`Float`] measured in half-turns, rounding the
    /// result to the precision of the input and to the nearest [`Float`]. The [`Float`] is taken by
    /// reference.
    ///
    /// See [`Float::acos_pi`] and [`Float::acos_with_period_prec_round`]; this function behaves the
    /// same way.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    ///
    /// let x = Float::from_unsigned_prec(1u32, 10).0 >> 2u32;
    /// assert_eq!((&x).acos_pi_ref().to_string(), "0.41943");
    /// ```
    #[inline]
    pub fn acos_pi_ref(&self) -> Self {
        self.acos_with_period_ref(2)
    }

    /// Computes $\arccos(x)/\pi$, the arccosine of a [`Float`] measured in half-turns, in place,
    /// rounding the result to the specified precision and with the specified rounding mode. An
    /// [`Ordering`] is returned, indicating whether the rounded arccosine is less than, equal to,
    /// or greater than the exact arccosine. Although `NaN`s are not comparable to any [`Float`],
    /// whenever this function assigns a `NaN` it also returns `Equal`.
    ///
    /// This is `acos_with_period` with a period of 2: see
    /// [`Float::acos_with_period_prec_round_assign`] for the error bounds, the special cases,
    /// underflow, and the complexity, with $u = 2$. A zero input gives $1/2$, an input of 1 gives
    /// $0.0$, and an input of $-1$ gives $1$; all three are exact at every precision, and they are
    /// the only exact cases. NaN, either infinity, and any $|x|>1$ give NaN. Overflow is not
    /// possible, since $0 \leq \arccos(x)/\pi \leq 1$.
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
    /// let mut x = Float::from(0.25);
    /// let o = x.acos_pi_prec_round_assign(10, Floor);
    /// assert_eq!(x.to_string(), "0.41943");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn acos_pi_prec_round_assign(&mut self, prec: u64, rm: RoundingMode) -> Ordering {
        self.acos_with_period_prec_round_assign(2, prec, rm)
    }

    /// Computes $\arccos(x)/\pi$, the arccosine of a [`Float`] measured in half-turns, in place,
    /// rounding the result to the nearest value of the specified precision. An [`Ordering`] is
    /// returned, indicating whether the rounded arccosine is less than, equal to, or greater than
    /// the exact arccosine. Although `NaN`s are not comparable to any [`Float`], whenever this
    /// function assigns a `NaN` it also returns `Equal`.
    ///
    /// See [`Float::acos_pi_prec`] and [`Float::acos_with_period_prec_round`]; this function
    /// behaves the same way.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from(0.25);
    /// let o = x.acos_pi_prec_assign(10);
    /// assert_eq!(x.to_string(), "0.41943");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn acos_pi_prec_assign(&mut self, prec: u64) -> Ordering {
        self.acos_with_period_prec_assign(2, prec)
    }

    /// Computes $\arccos(x)/\pi$, the arccosine of a [`Float`] measured in half-turns, in place,
    /// rounding the result with the specified rounding mode. The precision of the output is the
    /// precision of the input. An [`Ordering`] is returned, indicating whether the rounded
    /// arccosine is less than, equal to, or greater than the exact arccosine. Although `NaN`s are
    /// not comparable to any [`Float`], whenever this function assigns a `NaN` it also returns
    /// `Equal`.
    ///
    /// See [`Float::acos_pi_round`] and [`Float::acos_with_period_prec_round`]; this function
    /// behaves the same way.
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
    /// let o = x.acos_pi_round_assign(Floor);
    /// assert_eq!(x.to_string(), "0.41943");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn acos_pi_round_assign(&mut self, rm: RoundingMode) -> Ordering {
        self.acos_with_period_round_assign(2, rm)
    }

    /// Computes $\arccos(x)/\pi$, the arccosine of a [`Float`] measured in half-turns, in place,
    /// rounding the result to the precision of the input and to the nearest [`Float`].
    ///
    /// If the arccosine is equidistant from two [`Float`]s with the precision of the input, the
    /// [`Float`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a
    /// description of the `Nearest` rounding mode.
    ///
    /// See [`Float::acos_pi`] and [`Float::acos_with_period_prec_round`]; this function behaves the
    /// same way.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 10).0 >> 2u32;
    /// x.acos_pi_assign();
    /// assert_eq!(x.to_string(), "0.41943");
    /// ```
    #[inline]
    pub fn acos_pi_assign(&mut self) {
        self.acos_with_period_assign(2);
    }

    /// Computes $\arccos(x)/\pi$, the arccosine of a [`Rational`] measured in half-turns, rounding
    /// the result to the specified precision and with the specified rounding mode and returning the
    /// result as a [`Float`]. The [`Rational`] is taken by value. An [`Ordering`] is also returned,
    /// indicating whether the rounded arccosine is less than, equal to, or greater than the exact
    /// arccosine.
    ///
    /// This is `acos_with_period_rational` with a period of 2: see
    /// [`Float::acos_with_period_rational_prec_round`] for the error bounds, the special cases,
    /// underflow, and the complexity, with $u = 2$. A zero input gives $1/2$, an input of 1 gives
    /// $0.0$, and an input of $-1$ gives $1$; all three are exact at every precision, and they are
    /// the only exact cases. Any $|x|>1$ gives NaN. Overflow is not possible, since $0 \leq
    /// \arccos(x)/\pi \leq 1$.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with the given precision.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::NegativeOne;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// // an input of -1 is a whole half-turn
    /// let (c, o) = Float::acos_pi_rational_prec_round(Rational::NEGATIVE_ONE, 10, Exact);
    /// assert_eq!(c.to_string(), "1.0000");
    /// assert_eq!(o, Equal);
    ///
    /// let (c, o) =
    ///     Float::acos_pi_rational_prec_round(Rational::from_unsigneds(3u8, 5), 10, Floor);
    /// assert_eq!(c.to_string(), "0.29492");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn acos_pi_rational_prec_round(
        x: Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        Self::acos_with_period_rational_prec_round(x, 2, prec, rm)
    }

    /// Computes $\arccos(x)/\pi$, the arccosine of a [`Rational`] measured in half-turns, rounding
    /// the result to the specified precision and with the specified rounding mode and returning the
    /// result as a [`Float`]. The [`Rational`] is taken by reference. An [`Ordering`] is also
    /// returned, indicating whether the rounded arccosine is less than, equal to, or greater than
    /// the exact arccosine.
    ///
    /// See [`Float::acos_pi_rational_prec_round`] and
    /// [`Float::acos_with_period_rational_prec_round`]; this function behaves the same way.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with the given precision.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) =
    ///     Float::acos_pi_rational_prec_round_ref(&Rational::from_unsigneds(3u8, 5), 10, Ceiling);
    /// assert_eq!(c.to_string(), "0.29541");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn acos_pi_rational_prec_round_ref(
        x: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        Self::acos_with_period_rational_prec_round_ref(x, 2, prec, rm)
    }

    /// Computes $\arccos(x)/\pi$, the arccosine of a [`Rational`] measured in half-turns, rounding
    /// the result to the nearest value of the specified precision and returning the result as a
    /// [`Float`]. The [`Rational`] is taken by value. An [`Ordering`] is also returned, indicating
    /// whether the rounded arccosine is less than, equal to, or greater than the exact arccosine.
    ///
    /// See [`Float::acos_pi_rational_prec_round`] and [`Float::acos_with_period_rational_prec`];
    /// this function behaves the same way, rounding to nearest.
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
    /// let (c, o) = Float::acos_pi_rational_prec(Rational::from_unsigneds(3u8, 5), 53);
    /// assert_eq!(c.to_string(), "0.29516723530086653");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn acos_pi_rational_prec(x: Rational, prec: u64) -> (Self, Ordering) {
        Self::acos_with_period_rational_prec(x, 2, prec)
    }

    /// Computes $\arccos(x)/\pi$, the arccosine of a [`Rational`] measured in half-turns, rounding
    /// the result to the nearest value of the specified precision and returning the result as a
    /// [`Float`]. The [`Rational`] is taken by reference. An [`Ordering`] is also returned,
    /// indicating whether the rounded arccosine is less than, equal to, or greater than the exact
    /// arccosine.
    ///
    /// See [`Float::acos_pi_rational_prec`] and [`Float::acos_with_period_rational_prec_round`];
    /// this function behaves the same way.
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
    /// let (c, o) = Float::acos_pi_rational_prec_ref(&Rational::from_unsigneds(3u8, 5), 53);
    /// assert_eq!(c.to_string(), "0.29516723530086653");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn acos_pi_rational_prec_ref(x: &Rational, prec: u64) -> (Self, Ordering) {
        Self::acos_with_period_rational_prec_ref(x, 2, prec)
    }
}

impl Acos for Float {
    type Output = Self;

    /// Computes $\arccos x$, the arccosine of a [`Float`], taking it by value.
    ///
    /// If the output has a precision, it is the precision of the input. If the arccosine is
    /// equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s in
    /// its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// $$
    /// f(x) = \arccos x+\varepsilon.
    /// $$
    /// - If $x$ is NaN, if $|x|>1$, or if $x$ is 1, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2 |\arccos x|\rfloor-p}$, where $p$ is the
    ///   precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN})=f(\pm\infty)=\text{NaN}$
    /// - $f(x)=\text{NaN}$ for $|x|>1$
    /// - $f(\pm0.0)=\pi/2$, rounded
    /// - $f(1)=0.0$
    /// - $f(-1)=\pi$, rounded
    ///
    /// The zero at $x=1$ is the only exact case. Overflow is not possible, since the result lies in
    /// $[0,\pi]$.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::acos_round`] instead. If you want to specify the output precision, consider using
    /// [`Float::acos_prec`]. If you want both of these things, consider using
    /// [`Float::acos_prec_round`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^3 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`: the
    /// arccosine is taken as $\pi/2-\arctan(x/\sqrt{1-x^2})$ at a working precision of about $n$
    /// plus the bits that cancel there, which an input within $2^{-n}$ of 1 pushes to another $2n$;
    /// the arctangent at that width dominates. The magnitude of the input does not otherwise drive
    /// the cost.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Acos;
    /// use malachite_base::num::basic::traits::*;
    /// use malachite_float::Float;
    ///
    /// assert!(Float::NAN.acos().is_nan());
    /// // the arccosine is NaN outside [-1, 1], and both infinities are outside it
    /// assert!(Float::INFINITY.acos().is_nan());
    /// assert!(Float::NEGATIVE_INFINITY.acos().is_nan());
    /// assert_eq!(Float::ONE.acos().to_string(), "0.0");
    ///
    /// let x = Float::from_unsigned_prec(1u32, 100).0 >> 1u32;
    /// assert_eq!(x.acos().to_string(), "1.0471975511965977461542144610936");
    /// ```
    #[inline]
    fn acos(self) -> Self {
        let prec = self.significant_bits();
        self.acos_prec(prec).0
    }
}

impl Acos for &Float {
    type Output = Float;

    /// Computes $\arccos x$, the arccosine of a [`Float`], taking it by reference.
    ///
    /// If the output has a precision, it is the precision of the input. If the arccosine is
    /// equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s in
    /// its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// $$
    /// f(x) = \arccos x+\varepsilon.
    /// $$
    /// - If $x$ is NaN, if $|x|>1$, or if $x$ is 1, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2 |\arccos x|\rfloor-p}$, where $p$ is the
    ///   precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN})=f(\pm\infty)=\text{NaN}$
    /// - $f(x)=\text{NaN}$ for $|x|>1$
    /// - $f(\pm0.0)=\pi/2$, rounded
    /// - $f(1)=0.0$
    /// - $f(-1)=\pi$, rounded
    ///
    /// The zero at $x=1$ is the only exact case. Overflow is not possible, since the result lies in
    /// $[0,\pi]$.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::acos_round_ref`] instead. If you want to specify the output precision, consider
    /// using [`Float::acos_prec_ref`]. If you want both of these things, consider using
    /// [`Float::acos_prec_round_ref`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^3 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`: the
    /// arccosine is taken as $\pi/2-\arctan(x/\sqrt{1-x^2})$ at a working precision of about $n$
    /// plus the bits that cancel there, which an input within $2^{-n}$ of 1 pushes to another $2n$;
    /// the arctangent at that width dominates. The magnitude of the input does not otherwise drive
    /// the cost.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Acos;
    /// use malachite_base::num::basic::traits::*;
    /// use malachite_float::Float;
    ///
    /// assert!((&Float::NAN).acos().is_nan());
    /// assert_eq!((&Float::ONE).acos().to_string(), "0.0");
    ///
    /// let x = Float::from_unsigned_prec(1u32, 100).0 >> 1u32;
    /// assert_eq!((&x).acos().to_string(), "1.0471975511965977461542144610936");
    /// ```
    #[inline]
    fn acos(self) -> Float {
        self.acos_prec_ref(self.significant_bits()).0
    }
}

impl AcosAssign for Float {
    /// Computes $\arccos x$, the arccosine of a [`Float`], in place.
    ///
    /// If the output has a precision, it is the precision of the input. If the arccosine is
    /// equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s in
    /// its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// $$
    /// x \gets \arccos x+\varepsilon.
    /// $$
    /// - If $x$ is NaN, if $|x|>1$, or if $x$ is 1, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2 |\arccos x|\rfloor-p}$, where $p$ is the
    ///   precision of the input.
    ///
    /// See the [`Float::acos`] documentation for information on the special cases.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::acos_round_assign`] instead. If you want to specify the output precision, consider
    /// using [`Float::acos_prec_assign`]. If you want both of these things, consider using
    /// [`Float::acos_prec_round_assign`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^3 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`: the
    /// arccosine is taken as $\pi/2-\arctan(x/\sqrt{1-x^2})$ at a working precision of about $n$
    /// plus the bits that cancel there, which an input within $2^{-n}$ of 1 pushes to another $2n$;
    /// the arctangent at that width dominates. The magnitude of the input does not otherwise drive
    /// the cost.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::AcosAssign;
    /// use malachite_base::num::basic::traits::*;
    /// use malachite_float::Float;
    ///
    /// let mut x = Float::NAN;
    /// x.acos_assign();
    /// assert!(x.is_nan());
    ///
    /// let mut x = Float::ONE;
    /// x.acos_assign();
    /// assert_eq!(x.to_string(), "0.0");
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0 >> 1u32;
    /// x.acos_assign();
    /// assert_eq!(x.to_string(), "1.0471975511965977461542144610936");
    /// ```
    #[inline]
    fn acos_assign(&mut self) {
        let prec = self.significant_bits();
        self.acos_prec_assign(prec);
    }
}

/// Computes $\arccos x$, the arccosine of a primitive float, returning the result as a primitive
/// float.
///
/// $$
/// f(x) = \arccos x+\varepsilon,
/// $$
/// where $|\varepsilon| < 2^{\lfloor\log_2 |\arccos x|\rfloor-p}$ and $p$ is the precision of the
/// output (24 if `T` is a [`f32`] and 53 if `T` is a [`f64`]); the special cases below are exact.
///
/// Special cases:
/// - $f(\text{NaN})=f(\pm\infty)=\text{NaN}$
/// - $f(x)=\text{NaN}$ for $|x|>1$
/// - $f(\pm0.0)=\pi/2$, rounded
/// - $f(1)=0.0$
/// - $f(-1)=\pi$, rounded
///
/// Overflow is not possible, since the result lies in $[0,\pi]$, and neither is underflow: the only
/// input whose arccosine is zero is 1, where the result is exact.
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
/// use malachite_float::float::arithmetic::acos::primitive_float_acos;
///
/// assert!(primitive_float_acos(f32::NAN).is_nan());
/// // the arccosine is NaN outside [-1, 1]
/// assert!(primitive_float_acos(2.0f32).is_nan());
/// assert_eq!(NiceFloat(primitive_float_acos(1.0f32)), NiceFloat(0.0));
/// assert_eq!(
///     NiceFloat(primitive_float_acos(0.5f32)),
///     NiceFloat(1.0471976)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_acos(0.5f64)),
///     NiceFloat(1.0471975511965979)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_acos(-1.0f64)),
///     NiceFloat(3.141592653589793)
/// );
/// ```
#[inline]
#[allow(clippy::type_repetition_in_bounds)]
pub fn primitive_float_acos<T: PrimitiveFloat>(x: T) -> T
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    emulate_float_to_float_fn(Float::acos_prec, x)
}

/// Computes $\arccos x$, the arccosine of a [`Rational`], returning the result as a primitive
/// float.
///
/// $$
/// f(x) = \arccos x+\varepsilon,
/// $$
/// where $|\varepsilon| < 2^{\lfloor\log_2 |\arccos x|\rfloor-p}$ and $p$ is the precision of the
/// output (24 if `T` is a [`f32`] and 53 if `T` is a [`f64`]); the special cases below are exact.
///
/// Special cases:
/// - $f(x)=\text{NaN}$ for $|x|>1$
/// - $f(0)=\pi/2$, rounded
/// - $f(1)=0.0$
/// - $f(-1)=\pi$, rounded
///
/// Overflow is not possible, since the result lies in $[0,\pi]$. The result is subnormal, or zero,
/// only for an $x$ within $2^{-2^{31}}$ of 1.
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
/// use malachite_base::num::basic::traits::{One, Two};
/// use malachite_base::num::float::NiceFloat;
/// use malachite_float::float::arithmetic::acos::primitive_float_acos_rational;
/// use malachite_q::Rational;
///
/// // the arccosine is NaN outside [-1, 1]
/// assert!(primitive_float_acos_rational::<f64>(&Rational::TWO).is_nan());
/// assert_eq!(
///     NiceFloat(primitive_float_acos_rational::<f64>(&Rational::ONE)),
///     NiceFloat(0.0)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_acos_rational::<f64>(
///         &Rational::from_unsigneds(3u8, 5)
///     )),
///     NiceFloat(0.9272952180016122)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_acos_rational::<f32>(
///         &Rational::from_unsigneds(3u8, 5)
///     )),
///     NiceFloat(0.9272952)
/// );
/// ```
#[inline]
#[allow(clippy::type_repetition_in_bounds)]
pub fn primitive_float_acos_rational<T: PrimitiveFloat>(x: &Rational) -> T
where
    Float: PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float>,
{
    emulate_rational_to_float_fn(Float::acos_rational_prec_ref, x)
}

/// Computes $\arccos(x)u/(2\pi)$, the arccosine of a primitive float measured in $u$ths of a turn
/// (so that `u = 360` gives degrees), returning the result as a primitive float.
///
/// $$
/// f(x,u) = \arccos(x)u/(2\pi)+\varepsilon.
/// $$
/// - If $x$ is NaN, if $|x|>1$, if $u = 0$, if $x$ is zero, if $|x|$ is 1, or if $|x|$ is $1/2$ and
///   $u$ is a multiple of 3, $\varepsilon$ may be ignored or assumed to be 0.
/// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2 |\arccos(x)u/(2\pi)|\rfloor-p}$, where $p$ is the
///   precision of the output (24 if `T` is a [`f32`] and 53 if `T` is a [`f64`]).
///
/// Special cases:
/// - $f(\text{NaN},u)=f(\pm\infty,u)=\text{NaN}$
/// - $f(x,u)=\text{NaN}$ for $|x|>1$, including when $u=0$
/// - $f(\pm0.0,u)=u/4$, a quarter turn
/// - $f(x,0)=0.0$, since the arccosine is never negative
/// - $f(1,u)=0.0$
/// - $f(-1,u)=u/2$, a half turn
/// - $f(1/2,u)=u/6$ and $f(-1/2,u)=u/3$, a sixth and a third of a turn, when $u$ is a multiple of 3
///
/// Overflow is not possible, since $f(x,u) \leq u/2 < 2^{63}$, and neither is underflow: an $f32$
/// or $f64$ is never close enough to 1 for that.
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
/// use malachite_float::float::arithmetic::acos::primitive_float_acos_with_period;
///
/// assert!(primitive_float_acos_with_period(f32::NAN, 360).is_nan());
/// // an input outside [-1, 1] is NaN
/// assert!(primitive_float_acos_with_period(2.0f32, 360).is_nan());
/// // a zero input is a quarter turn, an input of 1/2 a sixth of one, and one of -1 a half turn
/// assert_eq!(
///     NiceFloat(primitive_float_acos_with_period(0.0f32, 360)),
///     NiceFloat(90.0)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_acos_with_period(0.5f32, 360)),
///     NiceFloat(60.0)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_acos_with_period(-1.0f32, 360)),
///     NiceFloat(180.0)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_acos_with_period(0.25f32, 360)),
///     NiceFloat(75.52249)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_acos_with_period(0.25f64, 360)),
///     NiceFloat(75.52248781407008)
/// );
/// ```
#[inline]
#[allow(clippy::type_repetition_in_bounds)]
pub fn primitive_float_acos_with_period<T: PrimitiveFloat>(x: T, u: u64) -> T
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    emulate_float_to_float_fn(|x, prec| Float::acos_with_period_prec(x, u, prec), x)
}

/// Computes $\arccos(x)u/(2\pi)$, the arccosine of a [`Rational`] measured in $u$ths of a turn (so
/// that `u = 360` gives degrees), returning the result as a primitive float.
///
/// $$
/// f(x,u) = \arccos(x)u/(2\pi)+\varepsilon.
/// $$
/// - If $|x|>1$, if $u = 0$, if $x$ is zero, if $|x|$ is 1, or if $|x|$ is $1/2$ and $u$ is a
///   multiple of 3, $\varepsilon$ may be ignored or assumed to be 0.
/// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2 |\arccos(x)u/(2\pi)|\rfloor-p}$, where $p$ is the
///   precision of the output (24 if `T` is a [`f32`] and 53 if `T` is a [`f64`]).
///
/// Special cases:
/// - $f(x,u)=\text{NaN}$ for $|x|>1$, including when $u=0$
/// - $f(0,u)=u/4$, a quarter turn
/// - $f(x,0)=0.0$, since the arccosine is never negative
/// - $f(1,u)=0.0$
/// - $f(-1,u)=u/2$, a half turn
/// - $f(1/2,u)=u/6$ and $f(-1/2,u)=u/3$, a sixth and a third of a turn, when $u$ is a multiple of 3
///
/// Overflow is not possible, since $f(x,u) \leq u/2 < 2^{63}$. The result is subnormal, or zero,
/// only when $u$ is small and $x$ is within about $2^{-2^{31}}$ of 1.
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
/// use malachite_base::num::basic::traits::{NegativeOne, One, Zero};
/// use malachite_base::num::float::NiceFloat;
/// use malachite_float::float::arithmetic::acos::primitive_float_acos_with_period_rational;
/// use malachite_q::Rational;
///
/// // a zero input is a quarter turn, an input of 1 zero, and one of -1 a half turn
/// assert_eq!(
///     NiceFloat(primitive_float_acos_with_period_rational::<f64>(
///         &Rational::ZERO,
///         360
///     )),
///     NiceFloat(90.0)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_acos_with_period_rational::<f64>(
///         &Rational::ONE,
///         360
///     )),
///     NiceFloat(0.0)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_acos_with_period_rational::<f64>(
///         &Rational::NEGATIVE_ONE,
///         360
///     )),
///     NiceFloat(180.0)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_acos_with_period_rational::<f64>(
///         &Rational::from_unsigneds(3u8, 5),
///         360
///     )),
///     NiceFloat(53.13010235415598)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_acos_with_period_rational::<f32>(
///         &Rational::from_unsigneds(3u8, 5),
///         360
///     )),
///     NiceFloat(53.130104)
/// );
/// ```
#[inline]
#[allow(clippy::type_repetition_in_bounds)]
pub fn primitive_float_acos_with_period_rational<T: PrimitiveFloat>(x: &Rational, u: u64) -> T
where
    Float: PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float>,
{
    emulate_rational_to_float_fn(
        |x, prec| Float::acos_with_period_rational_prec_ref(x, u, prec),
        x,
    )
}

/// Computes $\arccos(x)/\pi$, the arccosine of a primitive float measured in half-turns, returning
/// the result as a primitive float.
///
/// This is `primitive_float_acos_with_period` with a period of 2: see
/// [`primitive_float_acos_with_period`] for the error bounds, the special cases, and the
/// complexity, with $u = 2$. A zero input gives $1/2$, an input of 1 gives $0.0$, and an input of
/// $-1$ gives $1$; NaN, either infinity, and any $|x|>1$ give NaN. Overflow is not possible, since
/// $0 \leq \arccos(x)/\pi \leq 1$.
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
/// use malachite_float::float::arithmetic::acos::primitive_float_acos_pi;
///
/// assert!(primitive_float_acos_pi(f32::NAN).is_nan());
/// // the arccosine is NaN outside [-1, 1]
/// assert!(primitive_float_acos_pi(2.0f32).is_nan());
/// assert_eq!(NiceFloat(primitive_float_acos_pi(0.0f32)), NiceFloat(0.5));
/// assert_eq!(NiceFloat(primitive_float_acos_pi(1.0f32)), NiceFloat(0.0));
/// assert_eq!(NiceFloat(primitive_float_acos_pi(-1.0f32)), NiceFloat(1.0));
/// assert_eq!(
///     NiceFloat(primitive_float_acos_pi(0.25f32)),
///     NiceFloat(0.41956937)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_acos_pi(0.25f64)),
///     NiceFloat(0.41956937674483374)
/// );
/// ```
#[inline]
#[allow(clippy::type_repetition_in_bounds)]
pub fn primitive_float_acos_pi<T: PrimitiveFloat>(x: T) -> T
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    primitive_float_acos_with_period(x, 2)
}

/// Computes $\arccos(x)/\pi$, the arccosine of a [`Rational`] measured in half-turns, returning the
/// result as a primitive float.
///
/// This is `primitive_float_acos_with_period_rational` with a period of 2: see
/// [`primitive_float_acos_with_period_rational`] for the error bounds, the special cases, and the
/// complexity, with $u = 2$. A zero input gives $1/2$, an input of 1 gives $0.0$, and an input of
/// $-1$ gives $1$; any $|x|>1$ gives NaN. Overflow is not possible, since $0 \leq \arccos(x)/\pi
/// \leq 1$.
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
/// use malachite_base::num::basic::traits::{NegativeOne, One, Zero};
/// use malachite_base::num::float::NiceFloat;
/// use malachite_float::float::arithmetic::acos::primitive_float_acos_pi_rational;
/// use malachite_q::Rational;
///
/// assert_eq!(
///     NiceFloat(primitive_float_acos_pi_rational::<f64>(&Rational::ZERO)),
///     NiceFloat(0.5)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_acos_pi_rational::<f64>(&Rational::ONE)),
///     NiceFloat(0.0)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_acos_pi_rational::<f64>(
///         &Rational::NEGATIVE_ONE
///     )),
///     NiceFloat(1.0)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_acos_pi_rational::<f64>(
///         &Rational::from_unsigneds(3u8, 5)
///     )),
///     NiceFloat(0.2951672353008665)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_acos_pi_rational::<f32>(
///         &Rational::from_unsigneds(3u8, 5)
///     )),
///     NiceFloat(0.29516724)
/// );
/// ```
#[inline]
#[allow(clippy::type_repetition_in_bounds)]
pub fn primitive_float_acos_pi_rational<T: PrimitiveFloat>(x: &Rational) -> T
where
    Float: PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float>,
{
    primitive_float_acos_with_period_rational(x, 2)
}
