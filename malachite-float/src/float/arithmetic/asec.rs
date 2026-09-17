// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Float;
use crate::InnerFloat::{Finite, Infinity, NaN, Zero};
use crate::float::arithmetic::acos::{SCALED_RADICAND_EXPONENT, SCALED_RADICAND_SHIFT};
use crate::float::arithmetic::atan::{
    arc_with_period_scale, atan_rational_helper, scaled_unsigned,
};
use crate::float::arithmetic::sin::{SCALE, scaled_underflow};
use crate::{emulate_float_to_float_fn, emulate_rational_to_float_fn};
use core::cmp::Ordering::{self, Equal, Greater, Less};
use core::cmp::max;
use malachite_base::num::arithmetic::traits::{
    Abs, Asec, AsecAssign, CeilingLogBase2, IsPowerOf2, Square,
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

// Computes asec(x) for a finite nonzero `Float` x, rounded to precision `prec` with rounding mode
// `rm`.
//
// MPFR has no arcsecant. Rather than take acos(1/x), which would round the reciprocal first and pay
// for it -- the arccosine is not Lipschitz at 1, so an x near 1 would lose about half the bits of
// the reciprocal -- the identity is used in the form
//
//     asec(x) = atan(sqrt(x^2 - 1)),
//
// for a positive x, and pi minus that for a negative one. The subtraction x^2 - 1 is where an x
// near 1 loses bits, and it is done at a precision wide enough to be exact: the square of a p-bit
// `Float` needs 2p bits, and their difference no more. So nothing is lost, and unlike the arccosine
// the cost does not grow as x approaches +-1.
fn asec_prec_round_normal_ref(x: &Float, prec: u64, rm: RoundingMode) -> (Float, Ordering) {
    let positive = *x > 0u32;
    match x.partial_cmp_abs(&1u32).unwrap() {
        // asec(x) = NaN for |x| < 1, the secant never taking a value there
        Less => (Float::NAN, Equal),
        // asec(1) = +0, exactly, and asec(-1) = pi
        Equal => {
            if positive {
                (Float::ZERO, Equal)
            } else {
                Float::pi_prec_round(prec, rm)
            }
        }
        Greater => {
            assert_ne!(rm, Exact, "Inexact asec");
            let exp_x = i64::from(x.get_exponent().unwrap());
            // the width at which x^2 - 1 is exact
            let exact_w = (x.get_prec().unwrap() << 1) + 2;
            let mut w = prec + prec.ceiling_log_base_2() + 10;
            let mut increment = Limb::WIDTH;
            loop {
                let q = if exp_x << 1 > i64::exact_from(w) + 2 {
                    // x^2 - 1 = x^2(1 - x^-2), and x^-2 is below the working precision here, so the
                    // square root is |x| itself -- which also keeps a huge x from squaring out of
                    // the exponent range
                    x.abs()
                } else {
                    x.square_prec_ref(max(w, exact_w))
                        .0
                        .sub_prec(Float::ONE, max(w, exact_w))
                        .0
                        .sqrt_prec(w)
                        .0
                };
                // The square root is correctly rounded and the arctangent neither amplifies a
                // relative error nor adds more than its own half ulp, so three bits of slack cover
                // a positive x; pi and the subtraction take one more.
                let t = q.atan_prec(w).0;
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
    }
}

// Computes asec(x) u/(2 pi) for a finite `Float` x with |x| >= 1 and a nonzero u, rounded to
// precision `prec` with rounding mode `rm`. `rm` may be `Exact` only at |x| = 1, where the result
// is zero or u/2, and at |x| = 2 with u a multiple of 3, where it is u/6 or u/3.
//
// MPFR has no arcsecant, let alone one with a period; the shape follows `acos_with_period`, whose
// exact cases these are, seen through the reciprocal: asec(+-1) is acos(+-1) and asec(+-2) is
// acos(+-1/2). The quotient is formed with the numerator scaled up by 2^SCALE, as there, so that a
// result below the smallest positive `Float` is decided by the rounding mode alone.
fn asec_with_period_prec_round_normal_ref(
    x: &Float,
    u: u64,
    prec: u64,
    rm: RoundingMode,
) -> (Float, Ordering) {
    let positive = *x > 0u32;
    let exp_x = i64::from(x.get_exponent().unwrap());
    let power_of_2 = x.significand_ref().unwrap().is_power_of_2();
    // |x| = 1: asecu(1, u) = +0 and asecu(-1, u) = u/2
    if exp_x == 1 && power_of_2 {
        return if positive {
            (Float::ZERO, Equal)
        } else {
            scaled_unsigned(u, 1, true, prec, rm)
        };
    }
    // asec(2) = pi/3 and asec(-2) = 2 pi/3, so asecu(2, u) = u/6 and asecu(-2, u) = u/3, both exact
    // when u is a multiple of 3
    if exp_x == 2 && power_of_2 && u.is_multiple_of(3) {
        return scaled_unsigned(u / 3, u32::from(positive), true, prec, rm);
    }
    // Nothing else can be rounded exactly
    assert_ne!(rm, Exact, "Inexact asec_with_period");
    // asec(x) = pi/2 - 1/x + O(x^-3), so asecu(x, u) = u/4 (1 - 2/(pi x) + ...), and once EXP(x) >=
    // prec + 4 that correction is below an eighth of an ulp of u/4: the result is the neighbour of
    // u/4 on the side the arcsecant lies, below it for a positive x, whose arcsecant is under pi/2,
    // and above it for a negative one. Requiring EXP(x) >= 65 as well keeps the correction below
    // the last bit of u when u/4 is inexact. This is `acos_with_period`'s small-input branch,
    // reached through the reciprocal.
    if exp_x >= 65 && exp_x >= i64::exact_from(prec) + 4 {
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
        // scaling by a power of 2 is exact, and asec(x) u 2^SCALE stays far below the top of the
        // range, since asec(x) <= pi and u < 2^64
        |w| x.asec_prec_round_ref(w, Up).0 << SCALE,
        u,
        true,
        prec,
        rm,
    )
}

// Computes asec(x) for a `Rational` x with |x| > 1, rounded to precision `prec` with rounding mode
// `rm`. (The rest is handled by the caller.)
//
// The identity is the same one the `Float` arcsecant uses, asec(x) = atan(sqrt(x^2 - 1)) for a
// positive x and pi minus that for a negative one, and here x^2 - 1 is an exact `Rational`, so
// there is no working precision to choose for it at all.
pub(crate) fn asec_rational_helper(x: &Rational, prec: u64, rm: RoundingMode) -> (Float, Ordering) {
    assert_ne!(rm, Exact, "Inexact asec_rational");
    let positive = *x > 0u32;
    let exp_x = x.floor_log_base_2_abs() + 1;
    let xp = x.abs();
    let mut w = prec + prec.ceiling_log_base_2() + 10;
    let mut increment = Limb::WIDTH;
    // With v = |x| - 1, asec(x) = sqrt(2v)(1 + ...) for a positive x near 1. A `Rational` can sit
    // close enough to 1 to put that below the smallest positive `Float`, which the `Float`
    // arcsecant cannot reach; there v is below 2^(2 SCALED_INPUT_EXPONENT), so the correction is
    // invisible at any working precision the loop can reach and the answer is sqrt(2v), rounded. It
    // is formed scaled up, the radicand by 2^(2 SCALE) so that its square root is scaled by
    // 2^SCALE, and the underflow is then decided by the rounding mode alone. Taking the square root
    // of 2v rather than of x^2 - 1 also keeps this path cheap: an x this close to 1 has a huge
    // numerator and denominator, and squaring it would double their size.
    if positive {
        let v = &xp - Rational::ONE;
        if v.floor_log_base_2_abs() + 2 <= SCALED_RADICAND_EXPONENT {
            let scaled = v << SCALED_RADICAND_SHIFT;
            loop {
                // rounded away from zero, the side asec(x) is on
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
    let mut r = None;
    loop {
        let t = if exp_x << 1 > i64::exact_from(w) + 2 {
            // x^2 - 1 = x^2(1 - x^-2), and x^-2 is below the working precision here, so the square
            // root is |x| itself -- which also spares a huge `Rational` from being squared
            atan_rational_helper(&xp, w, Nearest).0
        } else {
            // exact, and positive since |x| > 1
            let r = r.get_or_insert_with(|| (&xp).square() - Rational::ONE);
            Float::sqrt_rational_prec_ref(r, w).0.atan_prec(w).0
        };
        // The square root is correctly rounded and the arctangent neither amplifies a relative
        // error nor adds more than its own half ulp, so three bits of slack cover a positive x; pi
        // and the subtraction take one more.
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

impl Float {
    /// Computes $\operatorname{asec} x$, the arcsecant of a [`Float`], rounding the result to the
    /// specified precision and with the specified rounding mode. The [`Float`] is taken by value.
    /// An [`Ordering`] is also returned, indicating whether the rounded arcsecant is less than,
    /// equal to, or greater than the exact arcsecant. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The arcsecant is the inverse of the secant on $[0,\pi]$, so it is the arccosine of the
    /// reciprocal, and it is defined only outside $(-1,1)$. See [`RoundingMode`] for a description
    /// of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \operatorname{asec} x+\varepsilon.
    /// $$
    /// - If $x$ is NaN, if $|x|<1$, or if $x$ is 1, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - Otherwise, if $m$ is not `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2
    ///   |\operatorname{asec} x|\rfloor-p+1}$.
    /// - Otherwise, if $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\operatorname{asec} x|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=\text{NaN}$
    /// - $f(x,p,m)=\text{NaN}$ for $|x|<1$, including $\pm0.0$
    /// - $f(\pm\infty,p,m)=\pi/2$, rounded, the value the secant grows toward
    /// - $f(1,p,m)=0.0$
    /// - $f(-1,p,m)=\pi$, rounded
    ///
    /// The zero at $x=1$ is the only exact case.
    ///
    /// Overflow is not possible, since the result lies in $[0,\pi]$. The result is zero only at
    /// $x=1$: an input just above 1 gives about $\sqrt{2(x-1)}$, which stays representable unless
    /// the input's precision exceeds $2^{31}$ bits.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::asec_prec`] instead. If you
    /// know that your target precision is the precision of the input, consider using
    /// [`Float::asec_round`] instead. If both of these things are true, consider using
    /// [`Float::asec`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `self.significant_bits()`: the arcsecant is taken as $\arctan(\sqrt{x^2-1})$, and the
    /// arctangent runs at a working precision of about $n$, which costs the first term; the second
    /// is the square, taken at twice the input's precision, where $x^2-1$ is exact. Unlike the
    /// arccosine, whose working precision grows as its input approaches $\pm1$, the arcsecant's
    /// does not: all of the cancellation is confined to that exact subtraction.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with the given precision (which is the case unless $x$ is NaN, $|x|<1$, or $x$ is 1).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{One, Two};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = Float::TWO.asec_prec_round(10, Floor);
    /// assert_eq!(c.to_string(), "1.0469");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::TWO.asec_prec_round(10, Ceiling);
    /// assert_eq!(c.to_string(), "1.0488");
    /// assert_eq!(o, Greater);
    ///
    /// // asec(1) is zero, exactly
    /// let (c, o) = Float::ONE.asec_prec_round(10, Exact);
    /// assert_eq!(c.to_string(), "0.0");
    /// assert_eq!(o, Equal);
    /// ```
    #[inline]
    pub fn asec_prec_round(self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        self.asec_prec_round_ref(prec, rm)
    }

    /// Computes $\operatorname{asec} x$, the arcsecant of a [`Float`], rounding the result to the
    /// specified precision and with the specified rounding mode. The [`Float`] is taken by
    /// reference. An [`Ordering`] is also returned, indicating whether the rounded arcsecant is
    /// less than, equal to, or greater than the exact arcsecant. Although `NaN`s are not comparable
    /// to any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`Float::asec_prec_round`] for the error bounds, the special cases, and the complexity;
    /// this function behaves the same way.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with the given precision.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{NegativeOne, Two};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = (&Float::TWO).asec_prec_round_ref(10, Floor);
    /// assert_eq!(c.to_string(), "1.0469");
    /// assert_eq!(o, Less);
    ///
    /// // an input of -1 gives pi
    /// let (c, o) = (&Float::NEGATIVE_ONE).asec_prec_round_ref(10, Nearest);
    /// assert_eq!(c.to_string(), "3.1406");
    /// assert_eq!(o, Less);
    /// ```
    pub fn asec_prec_round_ref(&self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        assert_ne!(prec, 0);
        match &self.0 {
            // the arcsecant is NaN inside (-1, 1), and both zeros are inside it
            NaN | Zero { .. } => (Self::NAN, Equal),
            // the secant grows without bound toward pi/2, so an infinite input gives pi/2
            Infinity { .. } => {
                let (pi, o) = Self::pi_prec_round(prec, rm);
                // exact
                (pi >> 1u32, o)
            }
            Finite { .. } => asec_prec_round_normal_ref(self, prec, rm),
        }
    }

    /// Computes $\operatorname{asec} x$, the arcsecant of a [`Float`], rounding the result to the
    /// nearest value of the specified precision. The [`Float`] is taken by value. An [`Ordering`]
    /// is also returned, indicating whether the rounded arcsecant is less than, equal to, or
    /// greater than the exact arcsecant. Although `NaN`s are not comparable to any [`Float`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// If the arcsecant is equidistant from two [`Float`]s with the specified precision, the
    /// [`Float`] with fewer 1s in its binary expansion is chosen.
    ///
    /// See [`Float::asec_prec_round`] for the error bounds, the special cases, and the complexity;
    /// this function behaves the same way.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::asec_prec_round`] instead.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::Two;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = Float::TWO.asec_prec(10);
    /// assert_eq!(c.to_string(), "1.0469");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::from_unsigned_prec(2u32, 100).0.asec_prec(100);
    /// assert_eq!(c.to_string(), "1.0471975511965977461542144610936");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn asec_prec(self, prec: u64) -> (Self, Ordering) {
        self.asec_prec_round(prec, Nearest)
    }

    /// Computes $\operatorname{asec} x$, the arcsecant of a [`Float`], rounding the result to the
    /// nearest value of the specified precision. The [`Float`] is taken by reference. An
    /// [`Ordering`] is also returned, indicating whether the rounded arcsecant is less than, equal
    /// to, or greater than the exact arcsecant. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`Float::asec_prec`] and [`Float::asec_prec_round`]; this function behaves the same way.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::Two;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = (&Float::TWO).asec_prec_ref(10);
    /// assert_eq!(c.to_string(), "1.0469");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = (&Float::from_unsigned_prec(2u32, 100).0).asec_prec_ref(100);
    /// assert_eq!(c.to_string(), "1.0471975511965977461542144610936");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn asec_prec_ref(&self, prec: u64) -> (Self, Ordering) {
        self.asec_prec_round_ref(prec, Nearest)
    }

    /// Computes $\operatorname{asec} x$, the arcsecant of a [`Float`], rounding the result with the
    /// specified rounding mode. The precision of the output is the precision of the input. The
    /// [`Float`] is taken by value. An [`Ordering`] is also returned, indicating whether the
    /// rounded arcsecant is less than, equal to, or greater than the exact arcsecant. Although
    /// `NaN`s are not comparable to any [`Float`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// See [`Float::asec_prec_round`] for the error bounds and the special cases; this function
    /// behaves the same way, with $p$ the precision of the input.
    ///
    /// If you want to specify an output precision, consider using [`Float::asec_prec_round`]
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^3 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`: the
    /// arcsecant is taken as $\arctan(\sqrt{x^2-1})$, with the square at twice the input's
    /// precision, where $x^2-1$ is exact, and the arctangent at about $n$ bits, which dominates.
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
    /// let x = Float::from_unsigned_prec(2u32, 100).0;
    /// let (c, o) = x.asec_round(Floor);
    /// assert_eq!(c.to_string(), "1.0471975511965977461542144610921");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn asec_round(self, rm: RoundingMode) -> (Self, Ordering) {
        let prec = self.significant_bits();
        self.asec_prec_round(prec, rm)
    }

    /// Computes $\operatorname{asec} x$, the arcsecant of a [`Float`], rounding the result with the
    /// specified rounding mode. The precision of the output is the precision of the input. The
    /// [`Float`] is taken by reference. An [`Ordering`] is also returned, indicating whether the
    /// rounded arcsecant is less than, equal to, or greater than the exact arcsecant. Although
    /// `NaN`s are not comparable to any [`Float`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// See [`Float::asec_round`] and [`Float::asec_prec_round`]; this function behaves the same
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
    /// let x = Float::from_unsigned_prec(2u32, 100).0;
    /// let (c, o) = (&x).asec_round_ref(Ceiling);
    /// assert_eq!(c.to_string(), "1.0471975511965977461542144610936");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn asec_round_ref(&self, rm: RoundingMode) -> (Self, Ordering) {
        self.asec_prec_round_ref(self.significant_bits(), rm)
    }

    /// Computes $\operatorname{asec} x$, the arcsecant of a [`Float`], in place, rounding the
    /// result to the specified precision and with the specified rounding mode. An [`Ordering`] is
    /// returned, indicating whether the rounded arcsecant is less than, equal to, or greater than
    /// the exact arcsecant. Although `NaN`s are not comparable to any [`Float`], whenever this
    /// function assigns a `NaN` it also returns `Equal`.
    ///
    /// See [`Float::asec_prec_round`] for the error bounds, the special cases, and the complexity;
    /// this function behaves the same way.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with the given precision.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::Two;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::TWO;
    /// let o = x.asec_prec_round_assign(10, Floor);
    /// assert_eq!(x.to_string(), "1.0469");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn asec_prec_round_assign(&mut self, prec: u64, rm: RoundingMode) -> Ordering {
        let (s, o) = self.asec_prec_round_ref(prec, rm);
        *self = s;
        o
    }

    /// Computes $\operatorname{asec} x$, the arcsecant of a [`Float`], in place, rounding the
    /// result to the nearest value of the specified precision. An [`Ordering`] is returned,
    /// indicating whether the rounded arcsecant is less than, equal to, or greater than the exact
    /// arcsecant. Although `NaN`s are not comparable to any [`Float`], whenever this function
    /// assigns a `NaN` it also returns `Equal`.
    ///
    /// See [`Float::asec_prec`] and [`Float::asec_prec_round`]; this function behaves the same way.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::Two;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::TWO;
    /// let o = x.asec_prec_assign(10);
    /// assert_eq!(x.to_string(), "1.0469");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn asec_prec_assign(&mut self, prec: u64) -> Ordering {
        self.asec_prec_round_assign(prec, Nearest)
    }

    /// Computes $\operatorname{asec} x$, the arcsecant of a [`Float`], in place, rounding the
    /// result with the specified rounding mode. The precision of the output is the precision of the
    /// input. An [`Ordering`] is returned, indicating whether the rounded arcsecant is less than,
    /// equal to, or greater than the exact arcsecant. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function assigns a `NaN` it also returns `Equal`.
    ///
    /// See [`Float::asec_round`] and [`Float::asec_prec_round`]; this function behaves the same
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
    /// let mut x = Float::from_unsigned_prec(2u32, 100).0;
    /// let o = x.asec_round_assign(Floor);
    /// assert_eq!(x.to_string(), "1.0471975511965977461542144610921");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn asec_round_assign(&mut self, rm: RoundingMode) -> Ordering {
        let prec = self.significant_bits();
        self.asec_prec_round_assign(prec, rm)
    }

    /// Computes $\operatorname{asec} x$, the arcsecant of a [`Rational`], rounding the result to
    /// the specified precision and with the specified rounding mode and returning the result as a
    /// [`Float`]. The [`Rational`] is taken by value. An [`Ordering`] is also returned, indicating
    /// whether the rounded arcsecant is less than, equal to, or greater than the exact arcsecant.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \operatorname{asec} x+\varepsilon.
    /// $$
    /// - If $|x|<1$ or if $x$ is 1, $\varepsilon$ may be ignored or assumed to be 0.
    /// - Otherwise, if $m$ is not `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2
    ///   |\operatorname{asec} x|\rfloor-p+1}$.
    /// - Otherwise, if $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\operatorname{asec} x|\rfloor-p}$.
    ///
    /// The output has precision `prec`.
    ///
    /// Special cases:
    /// - $f(x,p,m)=\text{NaN}$ for $|x|<1$, including zero
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
    /// arcsecant cannot reach, is possible here: a [`Rational`] may lie within $2^{-2^{31}}$ of 1,
    /// and there $\operatorname{asec} x$ is about $\sqrt{2(x-1)}$, which is below the smallest
    /// positive [`Float`].
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::asec_rational_prec`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `x.significant_bits()`: $x^2-1$ is formed exactly, and its square root and arctangent are
    /// taken at a working precision of about $n$ bits, which costs the first term; the second is
    /// the square. A large $x$ skips the square altogether, its arcsecant being the arctangent of
    /// $|x|$ to within the working precision.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with the given precision (which is the case unless $|x|<1$ or $x$ is 1).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{NegativeOne, Two};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = Float::asec_rational_prec_round(Rational::TWO, 10, Floor);
    /// assert_eq!(c.to_string(), "1.0469");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::asec_rational_prec_round(Rational::TWO, 10, Ceiling);
    /// assert_eq!(c.to_string(), "1.0488");
    /// assert_eq!(o, Greater);
    ///
    /// // an input of -1 gives pi
    /// let (c, o) = Float::asec_rational_prec_round(Rational::NEGATIVE_ONE, 10, Nearest);
    /// assert_eq!(c.to_string(), "3.1406");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    #[allow(clippy::needless_pass_by_value)]
    pub fn asec_rational_prec_round(x: Rational, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        Self::asec_rational_prec_round_ref(&x, prec, rm)
    }

    /// Computes $\operatorname{asec} x$, the arcsecant of a [`Rational`], rounding the result to
    /// the specified precision and with the specified rounding mode and returning the result as a
    /// [`Float`]. The [`Rational`] is taken by reference. An [`Ordering`] is also returned,
    /// indicating whether the rounded arcsecant is less than, equal to, or greater than the exact
    /// arcsecant.
    ///
    /// See [`Float::asec_rational_prec_round`] for the error bounds, the special cases, underflow,
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
    /// // asec(1) is zero, exactly
    /// let (c, o) = Float::asec_rational_prec_round_ref(&Rational::ONE, 10, Exact);
    /// assert_eq!(c.to_string(), "0.0");
    /// assert_eq!(o, Equal);
    ///
    /// let (c, o) =
    ///     Float::asec_rational_prec_round_ref(&Rational::from_unsigneds(5u8, 3), 10, Floor);
    /// assert_eq!(c.to_string(), "0.92676");
    /// assert_eq!(o, Less);
    /// ```
    pub fn asec_rational_prec_round_ref(
        x: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        assert_ne!(prec, 0);
        match x.partial_cmp_abs(&1u32).unwrap() {
            // the arcsecant is NaN inside (-1, 1), zero included
            Less => (Self::NAN, Equal),
            // asec(1) = +0, exactly, and asec(-1) = pi
            Equal => {
                if *x > 0u32 {
                    (Self::ZERO, Equal)
                } else {
                    Self::pi_prec_round(prec, rm)
                }
            }
            Greater => asec_rational_helper(x, prec, rm),
        }
    }

    /// Computes $\operatorname{asec} x$, the arcsecant of a [`Rational`], rounding the result to
    /// the nearest value of the specified precision and returning the result as a [`Float`]. The
    /// [`Rational`] is taken by value. An [`Ordering`] is also returned, indicating whether the
    /// rounded arcsecant is less than, equal to, or greater than the exact arcsecant.
    ///
    /// If the arcsecant is equidistant from two [`Float`]s with the specified precision, the
    /// [`Float`] with fewer 1s in its binary expansion is chosen.
    ///
    /// See [`Float::asec_rational_prec_round`] for the error bounds, the special cases, underflow,
    /// and the complexity; this function behaves the same way.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::asec_rational_prec_round`] instead.
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
    /// let (c, o) = Float::asec_rational_prec(Rational::from_unsigneds(5u8, 3), 10);
    /// assert_eq!(c.to_string(), "0.92773");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = Float::asec_rational_prec(Rational::from_unsigneds(5u8, 3), 53);
    /// assert_eq!(c.to_string(), "0.92729521800161219");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn asec_rational_prec(x: Rational, prec: u64) -> (Self, Ordering) {
        Self::asec_rational_prec_round(x, prec, Nearest)
    }

    /// Computes $\operatorname{asec} x$, the arcsecant of a [`Rational`], rounding the result to
    /// the nearest value of the specified precision and returning the result as a [`Float`]. The
    /// [`Rational`] is taken by reference. An [`Ordering`] is also returned, indicating whether the
    /// rounded arcsecant is less than, equal to, or greater than the exact arcsecant.
    ///
    /// See [`Float::asec_rational_prec`] and [`Float::asec_rational_prec_round`]; this function
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
    /// let (c, o) = Float::asec_rational_prec_ref(&Rational::from_unsigneds(5u8, 3), 53);
    /// assert_eq!(c.to_string(), "0.92729521800161219");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn asec_rational_prec_ref(x: &Rational, prec: u64) -> (Self, Ordering) {
        Self::asec_rational_prec_round_ref(x, prec, Nearest)
    }

    /// Computes $\operatorname{asec}(x)u/(2\pi)$, the arcsecant of a [`Float`] measured in $u$ths
    /// of a turn, rounding the result to the specified precision and with the specified rounding
    /// mode. The [`Float`] is taken by value. An [`Ordering`] is also returned, indicating whether
    /// the rounded arcsecant is less than, equal to, or greater than the exact arcsecant. Although
    /// `NaN`s are not comparable to any [`Float`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,u,p,m) = \operatorname{asec}(x)u/(2\pi)+\varepsilon.
    /// $$
    /// - If $x$ is NaN or infinite, if $|x|<1$, if $u = 0$, if $|x|$ is 1, or if $|x|$ is 2 and $u$
    ///   is a multiple of 3, $\varepsilon$ may be ignored or assumed to be 0.
    /// - Otherwise, if $m$ is not `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2
    ///   |\operatorname{asec}(x)u/(2\pi)|\rfloor-p+1}$.
    /// - Otherwise, if $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\operatorname{asec}(x)u/(2\pi)|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},u,p,m)=\text{NaN}$
    /// - $f(x,u,p,m)=\text{NaN}$ for $|x|<1$, including $\pm0.0$ and when $u=0$
    /// - $f(\pm\infty,u,p,m)=u/4$, a quarter turn
    /// - $f(x,0,p,m)=0.0$ for $|x|\geq1$, since the arcsecant is never negative
    /// - $f(1,u,p,m)=0.0$
    /// - $f(-1,u,p,m)=u/2$, a half turn
    /// - $f(2,u,p,m)=u/6$ and $f(-2,u,p,m)=u/3$, a sixth and a third of a turn, when $u$ is a
    ///   multiple of 3
    ///
    /// Those are the only exact cases -- the arcsecant's exact values are the arccosine's, seen
    /// through the reciprocal -- and the turn fractions are exact only when $p$ is large enough to
    /// hold them.
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
    /// $2^{31}$ bits; the arcsecant itself cannot underflow.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::asec_with_period_prec`]
    /// instead. If you know that your target precision is the precision of the input, consider
    /// using [`Float::asec_with_period_round`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `self.significant_bits()`: the arcsecant is taken at a working precision of about $n$ bits
    /// and scaled by $u/(2\pi)$, which needs $\pi$ to that many bits, and both cost the first term;
    /// the second is the exact square inside the arcsecant. A large $x$ skips all of it, the result
    /// being the neighbour of $u/4$.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with the given precision.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{Infinity, Two};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// // an infinite input is a quarter turn, and an input of 2 a sixth of one
    /// let (c, o) = Float::INFINITY.asec_with_period_prec_round(360, 10, Exact);
    /// assert_eq!(c.to_string(), "90.000");
    /// assert_eq!(o, Equal);
    ///
    /// let (c, o) = Float::TWO.asec_with_period_prec_round(360, 10, Exact);
    /// assert_eq!(c.to_string(), "60.000");
    /// assert_eq!(o, Equal);
    ///
    /// let (c, o) = Float::from(1.5).asec_with_period_prec_round(360, 10, Floor);
    /// assert_eq!(c.to_string(), "48.188");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::from(1.5).asec_with_period_prec_round(360, 10, Ceiling);
    /// assert_eq!(c.to_string(), "48.250");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn asec_with_period_prec_round(
        self,
        u: u64,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        self.asec_with_period_prec_round_ref(u, prec, rm)
    }

    /// Computes $\operatorname{asec}(x)u/(2\pi)$, the arcsecant of a [`Float`] measured in $u$ths
    /// of a turn, rounding the result to the specified precision and with the specified rounding
    /// mode. The [`Float`] is taken by reference. An [`Ordering`] is also returned, indicating
    /// whether the rounded arcsecant is less than, equal to, or greater than the exact arcsecant.
    /// Although `NaN`s are not comparable to any [`Float`], whenever this function returns a `NaN`
    /// it also returns `Equal`.
    ///
    /// See [`Float::asec_with_period_prec_round`] for the error bounds, the special and closed-form
    /// cases, underflow, and the complexity; this function behaves the same way.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with the given precision.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::Two;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// // an input of -2 is a third of a turn
    /// let (c, o) = (&-Float::TWO).asec_with_period_prec_round_ref(360, 10, Exact);
    /// assert_eq!(c.to_string(), "120.00");
    /// assert_eq!(o, Equal);
    ///
    /// let (c, o) = (&Float::from(1.5)).asec_with_period_prec_round_ref(360, 10, Floor);
    /// assert_eq!(c.to_string(), "48.188");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn asec_with_period_prec_round_ref(
        &self,
        u: u64,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        assert_ne!(prec, 0);
        match &self.0 {
            // the arcsecant is NaN inside (-1, 1), and both zeros are inside it; this holds for a
            // zero period too, since NaN times 0 is NaN
            NaN | Zero { .. } => (Self::NAN, Equal),
            // asec(±infinity) = pi/2, so asecu(±infinity, u) = u/4, which is zero when u is
            Infinity { .. } => scaled_unsigned(u, 2, true, prec, rm),
            Finite { .. } => {
                if self.lt_abs(&1u32) {
                    (Self::NAN, Equal)
                } else if u == 0 {
                    // asecu(x, 0) = +0, since the arcsecant is never negative
                    (Self::ZERO, Equal)
                } else {
                    asec_with_period_prec_round_normal_ref(self, u, prec, rm)
                }
            }
        }
    }

    /// Computes $\operatorname{asec}(x)u/(2\pi)$, the arcsecant of a [`Float`] measured in $u$ths
    /// of a turn, rounding the result to the nearest value of the specified precision. The
    /// [`Float`] is taken by value. An [`Ordering`] is also returned, indicating whether the
    /// rounded arcsecant is less than, equal to, or greater than the exact arcsecant. Although
    /// `NaN`s are not comparable to any [`Float`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// If the arcsecant is equidistant from two [`Float`]s with the specified precision, the
    /// [`Float`] with fewer 1s in its binary expansion is chosen.
    ///
    /// See [`Float::asec_with_period_prec_round`] for the error bounds, the special and closed-form
    /// cases, underflow, and the complexity; this function behaves the same way.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::asec_with_period_prec_round`] instead.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = Float::from(1.5).asec_with_period_prec(360, 10);
    /// assert_eq!(c.to_string(), "48.188");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::from(1.5).asec_with_period_prec(360, 53);
    /// assert_eq!(c.to_string(), "48.189685104221404");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn asec_with_period_prec(self, u: u64, prec: u64) -> (Self, Ordering) {
        self.asec_with_period_prec_round(u, prec, Nearest)
    }

    /// Computes $\operatorname{asec}(x)u/(2\pi)$, the arcsecant of a [`Float`] measured in $u$ths
    /// of a turn, rounding the result to the nearest value of the specified precision. The
    /// [`Float`] is taken by reference. An [`Ordering`] is also returned, indicating whether the
    /// rounded arcsecant is less than, equal to, or greater than the exact arcsecant. Although
    /// `NaN`s are not comparable to any [`Float`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// See [`Float::asec_with_period_prec`] and [`Float::asec_with_period_prec_round`]; this
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
    /// let (c, o) = (&Float::from(1.5)).asec_with_period_prec_ref(360, 10);
    /// assert_eq!(c.to_string(), "48.188");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn asec_with_period_prec_ref(&self, u: u64, prec: u64) -> (Self, Ordering) {
        self.asec_with_period_prec_round_ref(u, prec, Nearest)
    }

    /// Computes $\operatorname{asec}(x)u/(2\pi)$, the arcsecant of a [`Float`] measured in $u$ths
    /// of a turn, rounding the result with the specified rounding mode. The precision of the output
    /// is the precision of the input. The [`Float`] is taken by value. An [`Ordering`] is also
    /// returned, indicating whether the rounded arcsecant is less than, equal to, or greater than
    /// the exact arcsecant. Although `NaN`s are not comparable to any [`Float`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`Float::asec_with_period_prec_round`] for the error bounds, the special and closed-form
    /// cases, underflow, and the complexity; this function behaves the same way.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::asec_with_period_prec_round`] instead.
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
    /// let x = Float::from_unsigned_prec(3u32, 10).0 >> 1u32;
    /// let (c, o) = x.asec_with_period_round(360, Floor);
    /// assert_eq!(c.to_string(), "48.188");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn asec_with_period_round(self, u: u64, rm: RoundingMode) -> (Self, Ordering) {
        let prec = self.significant_bits();
        self.asec_with_period_prec_round(u, prec, rm)
    }

    /// Computes $\operatorname{asec}(x)u/(2\pi)$, the arcsecant of a [`Float`] measured in $u$ths
    /// of a turn, rounding the result with the specified rounding mode. The precision of the output
    /// is the precision of the input. The [`Float`] is taken by reference. An [`Ordering`] is also
    /// returned, indicating whether the rounded arcsecant is less than, equal to, or greater than
    /// the exact arcsecant. Although `NaN`s are not comparable to any [`Float`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`Float::asec_with_period_round`] and [`Float::asec_with_period_prec_round`]; this
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
    /// let x = Float::from_unsigned_prec(3u32, 10).0 >> 1u32;
    /// let (c, o) = (&x).asec_with_period_round_ref(360, Ceiling);
    /// assert_eq!(c.to_string(), "48.250");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn asec_with_period_round_ref(&self, u: u64, rm: RoundingMode) -> (Self, Ordering) {
        self.asec_with_period_prec_round_ref(u, self.significant_bits(), rm)
    }

    /// Computes $\operatorname{asec}(x)u/(2\pi)$, the arcsecant of a [`Float`] measured in $u$ths
    /// of a turn, rounding the result to the nearest value of the input's precision. The [`Float`]
    /// is taken by value.
    ///
    /// If the arcsecant is equidistant from two [`Float`]s with the specified precision, the
    /// [`Float`] with fewer 1s in its binary expansion is chosen.
    ///
    /// See [`Float::asec_with_period_prec_round`] for the error bounds, the special and closed-form
    /// cases, underflow, and the complexity; this function behaves the same way.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::asec_with_period_round`] instead. If you want to specify an output precision,
    /// consider using [`Float::asec_with_period_prec`]. If you want both of these things, consider
    /// using [`Float::asec_with_period_prec_round`].
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    ///
    /// let x = Float::from_unsigned_prec(3u32, 10).0 >> 1u32;
    /// assert_eq!(x.asec_with_period(360).to_string(), "48.188");
    /// ```
    #[inline]
    pub fn asec_with_period(self, u: u64) -> Self {
        let prec = self.significant_bits();
        self.asec_with_period_prec(u, prec).0
    }

    /// Computes $\operatorname{asec}(x)u/(2\pi)$, the arcsecant of a [`Float`] measured in $u$ths
    /// of a turn, rounding the result to the nearest value of the input's precision. The [`Float`]
    /// is taken by reference.
    ///
    /// See [`Float::asec_with_period`] and [`Float::asec_with_period_prec_round`]; this function
    /// behaves the same way.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    ///
    /// let x = Float::from_unsigned_prec(3u32, 10).0 >> 1u32;
    /// assert_eq!((&x).asec_with_period_ref(360).to_string(), "48.188");
    /// ```
    #[inline]
    pub fn asec_with_period_ref(&self, u: u64) -> Self {
        self.asec_with_period_prec_ref(u, self.significant_bits()).0
    }

    /// Computes $\operatorname{asec}(x)u/(2\pi)$, the arcsecant of a [`Float`] measured in $u$ths
    /// of a turn, in place, rounding the result to the specified precision and with the specified
    /// rounding mode. An [`Ordering`] is returned, indicating whether the rounded arcsecant is less
    /// than, equal to, or greater than the exact arcsecant. Although `NaN`s are not comparable to
    /// any [`Float`], whenever this function assigns a `NaN` it also returns `Equal`.
    ///
    /// See [`Float::asec_with_period_prec_round`] for the error bounds, the special and closed-form
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
    /// let mut x = Float::from(1.5);
    /// let o = x.asec_with_period_prec_round_assign(360, 10, Floor);
    /// assert_eq!(x.to_string(), "48.188");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn asec_with_period_prec_round_assign(
        &mut self,
        u: u64,
        prec: u64,
        rm: RoundingMode,
    ) -> Ordering {
        let (c, o) = self.asec_with_period_prec_round_ref(u, prec, rm);
        *self = c;
        o
    }

    /// Computes $\operatorname{asec}(x)u/(2\pi)$, the arcsecant of a [`Float`] measured in $u$ths
    /// of a turn, in place, rounding the result to the nearest value of the specified precision. An
    /// [`Ordering`] is returned, indicating whether the rounded arcsecant is less than, equal to,
    /// or greater than the exact arcsecant. Although `NaN`s are not comparable to any [`Float`],
    /// whenever this function assigns a `NaN` it also returns `Equal`.
    ///
    /// See [`Float::asec_with_period_prec`] and [`Float::asec_with_period_prec_round`]; this
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
    /// let mut x = Float::from(1.5);
    /// let o = x.asec_with_period_prec_assign(360, 10);
    /// assert_eq!(x.to_string(), "48.188");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn asec_with_period_prec_assign(&mut self, u: u64, prec: u64) -> Ordering {
        self.asec_with_period_prec_round_assign(u, prec, Nearest)
    }

    /// Computes $\operatorname{asec}(x)u/(2\pi)$, the arcsecant of a [`Float`] measured in $u$ths
    /// of a turn, in place, rounding the result with the specified rounding mode. The precision of
    /// the output is the precision of the input. An [`Ordering`] is returned, indicating whether
    /// the rounded arcsecant is less than, equal to, or greater than the exact arcsecant. Although
    /// `NaN`s are not comparable to any [`Float`], whenever this function assigns a `NaN` it also
    /// returns `Equal`.
    ///
    /// See [`Float::asec_with_period_round`] and [`Float::asec_with_period_prec_round`]; this
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
    /// let mut x = Float::from_unsigned_prec(3u32, 10).0 >> 1u32;
    /// let o = x.asec_with_period_round_assign(360, Floor);
    /// assert_eq!(x.to_string(), "48.188");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn asec_with_period_round_assign(&mut self, u: u64, rm: RoundingMode) -> Ordering {
        let prec = self.significant_bits();
        self.asec_with_period_prec_round_assign(u, prec, rm)
    }

    /// Computes $\operatorname{asec}(x)u/(2\pi)$, the arcsecant of a [`Float`] measured in $u$ths
    /// of a turn, in place, rounding the result to the nearest value of the input's precision.
    ///
    /// If the arcsecant is equidistant from two [`Float`]s with the specified precision, the
    /// [`Float`] with fewer 1s in its binary expansion is chosen.
    ///
    /// See [`Float::asec_with_period_prec_round`] for the error bounds, the special and closed-form
    /// cases, underflow, and the complexity; this function behaves the same way.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::asec_with_period_round_assign`] instead. If you want to specify an output
    /// precision, consider using [`Float::asec_with_period_prec_assign`]. If you want both of these
    /// things, consider using [`Float::asec_with_period_prec_round_assign`].
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    ///
    /// let mut x = Float::from_unsigned_prec(3u32, 10).0 >> 1u32;
    /// x.asec_with_period_assign(360);
    /// assert_eq!(x.to_string(), "48.188");
    /// ```
    #[inline]
    pub fn asec_with_period_assign(&mut self, u: u64) {
        let prec = self.significant_bits();
        self.asec_with_period_prec_assign(u, prec);
    }
}

impl Asec for Float {
    type Output = Self;

    /// Computes $\operatorname{asec} x$, the arcsecant of a [`Float`], taking it by value.
    ///
    /// If the output has a precision, it is the precision of the input. If the arcsecant is
    /// equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s in
    /// its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// $$
    /// f(x) = \operatorname{asec} x+\varepsilon.
    /// $$
    /// - If $x$ is NaN, if $|x|<1$, or if $x$ is 1, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2 |\operatorname{asec} x|\rfloor-p}$, where $p$
    ///   is the precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN})=\text{NaN}$
    /// - $f(x)=\text{NaN}$ for $|x|<1$, including $\pm0.0$
    /// - $f(\pm\infty)=\pi/2$, rounded
    /// - $f(1)=0.0$
    /// - $f(-1)=\pi$, rounded
    ///
    /// The zero at $x=1$ is the only exact case. Overflow is not possible, since the result lies in
    /// $[0,\pi]$.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::asec_round`] instead. If you want to specify the output precision, consider using
    /// [`Float::asec_prec`]. If you want both of these things, consider using
    /// [`Float::asec_prec_round`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^3 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`: the
    /// arcsecant is taken as $\arctan(\sqrt{x^2-1})$, with the square at twice the input's
    /// precision, where $x^2-1$ is exact, and the arctangent at about $n$ bits, which dominates.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Asec;
    /// use malachite_base::num::basic::traits::*;
    /// use malachite_float::Float;
    ///
    /// assert!(Float::NAN.asec().is_nan());
    /// // the arcsecant is NaN inside (-1, 1)
    /// assert!(Float::ZERO.asec().is_nan());
    /// assert!(Float::ONE_HALF.asec().is_nan());
    /// assert_eq!(Float::ONE.asec().to_string(), "0.0");
    ///
    /// let x = Float::from_unsigned_prec(2u32, 100).0;
    /// assert_eq!(x.asec().to_string(), "1.0471975511965977461542144610936");
    /// ```
    #[inline]
    fn asec(self) -> Self {
        let prec = self.significant_bits();
        self.asec_prec(prec).0
    }
}

impl Asec for &Float {
    type Output = Float;

    /// Computes $\operatorname{asec} x$, the arcsecant of a [`Float`], taking it by reference.
    ///
    /// If the output has a precision, it is the precision of the input. If the arcsecant is
    /// equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s in
    /// its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// $$
    /// f(x) = \operatorname{asec} x+\varepsilon.
    /// $$
    /// - If $x$ is NaN, if $|x|<1$, or if $x$ is 1, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2 |\operatorname{asec} x|\rfloor-p}$, where $p$
    ///   is the precision of the input.
    ///
    /// See the [`Float::asec`] documentation for information on the special cases.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::asec_round_ref`] instead. If you want to specify the output precision, consider
    /// using [`Float::asec_prec_ref`]. If you want both of these things, consider using
    /// [`Float::asec_prec_round_ref`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^3 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`: the
    /// arcsecant is taken as $\arctan(\sqrt{x^2-1})$, with the square at twice the input's
    /// precision, where $x^2-1$ is exact, and the arctangent at about $n$ bits, which dominates.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Asec;
    /// use malachite_base::num::basic::traits::*;
    /// use malachite_float::Float;
    ///
    /// assert!((&Float::NAN).asec().is_nan());
    /// assert_eq!((&Float::ONE).asec().to_string(), "0.0");
    ///
    /// let x = Float::from_unsigned_prec(2u32, 100).0;
    /// assert_eq!((&x).asec().to_string(), "1.0471975511965977461542144610936");
    /// ```
    #[inline]
    fn asec(self) -> Float {
        self.asec_prec_ref(self.significant_bits()).0
    }
}

impl AsecAssign for Float {
    /// Computes $\operatorname{asec} x$, the arcsecant of a [`Float`], in place.
    ///
    /// If the output has a precision, it is the precision of the input. If the arcsecant is
    /// equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s in
    /// its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// $$
    /// x \gets \operatorname{asec} x+\varepsilon.
    /// $$
    /// - If $x$ is NaN, if $|x|<1$, or if $x$ is 1, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2 |\operatorname{asec} x|\rfloor-p}$, where $p$
    ///   is the precision of the input.
    ///
    /// See the [`Float::asec`] documentation for information on the special cases.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::asec_round_assign`] instead. If you want to specify the output precision, consider
    /// using [`Float::asec_prec_assign`]. If you want both of these things, consider using
    /// [`Float::asec_prec_round_assign`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^3 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`: the
    /// arcsecant is taken as $\arctan(\sqrt{x^2-1})$, with the square at twice the input's
    /// precision, where $x^2-1$ is exact, and the arctangent at about $n$ bits, which dominates.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::AsecAssign;
    /// use malachite_base::num::basic::traits::*;
    /// use malachite_float::Float;
    ///
    /// let mut x = Float::NAN;
    /// x.asec_assign();
    /// assert!(x.is_nan());
    ///
    /// let mut x = Float::ONE;
    /// x.asec_assign();
    /// assert_eq!(x.to_string(), "0.0");
    ///
    /// let mut x = Float::from_unsigned_prec(2u32, 100).0;
    /// x.asec_assign();
    /// assert_eq!(x.to_string(), "1.0471975511965977461542144610936");
    /// ```
    #[inline]
    fn asec_assign(&mut self) {
        let prec = self.significant_bits();
        self.asec_prec_assign(prec);
    }
}

/// Computes $\operatorname{asec} x$, the arcsecant of a primitive float, returning the result as a
/// primitive float.
///
/// $$
/// f(x) = \operatorname{asec} x+\varepsilon,
/// $$
/// where $|\varepsilon| < 2^{\lfloor\log_2 |\operatorname{asec} x|\rfloor-p}$ and $p$ is the
/// precision of the output (24 if `T` is a [`f32`] and 53 if `T` is a [`f64`]); the special cases
/// below are exact.
///
/// Special cases:
/// - $f(\text{NaN})=\text{NaN}$
/// - $f(x)=\text{NaN}$ for $|x|<1$, including $\pm0.0$
/// - $f(\pm\infty)=\pi/2$, rounded
/// - $f(1)=0.0$
/// - $f(-1)=\pi$, rounded
///
/// Overflow is not possible, since the result lies in $[0,\pi]$, and neither is underflow: the only
/// input whose arcsecant is zero is 1, where the result is exact.
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
/// use malachite_float::float::arithmetic::asec::primitive_float_asec;
///
/// assert!(primitive_float_asec(f32::NAN).is_nan());
/// // the arcsecant is NaN inside (-1, 1)
/// assert!(primitive_float_asec(0.5f32).is_nan());
/// assert_eq!(NiceFloat(primitive_float_asec(1.0f32)), NiceFloat(0.0));
/// assert_eq!(
///     NiceFloat(primitive_float_asec(2.0f32)),
///     NiceFloat(1.0471976)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_asec(2.0f64)),
///     NiceFloat(1.0471975511965979)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_asec(-1.0f64)),
///     NiceFloat(3.141592653589793)
/// );
/// // a huge input is a quarter turn
/// assert_eq!(
///     NiceFloat(primitive_float_asec(1.0e300f64)),
///     NiceFloat(1.5707963267948966)
/// );
/// ```
#[inline]
#[allow(clippy::type_repetition_in_bounds)]
pub fn primitive_float_asec<T: PrimitiveFloat>(x: T) -> T
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    emulate_float_to_float_fn(Float::asec_prec, x)
}

/// Computes $\operatorname{asec} x$, the arcsecant of a [`Rational`], returning the result as a
/// primitive float.
///
/// $$
/// f(x) = \operatorname{asec} x+\varepsilon,
/// $$
/// where $|\varepsilon| < 2^{\lfloor\log_2 |\operatorname{asec} x|\rfloor-p}$ and $p$ is the
/// precision of the output (24 if `T` is a [`f32`] and 53 if `T` is a [`f64`]); the special cases
/// below are exact.
///
/// Special cases:
/// - $f(x)=\text{NaN}$ for $|x|<1$, including zero
/// - $f(1)=0.0$
/// - $f(-1)=\pi$, rounded
///
/// Overflow is not possible, since the result lies in $[0,\pi]$. The result is subnormal, or zero,
/// only for an $x$ within about $2^{-2^{31}}$ of 1.
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
/// use malachite_base::num::basic::traits::{NegativeOne, One, OneHalf, Two};
/// use malachite_base::num::float::NiceFloat;
/// use malachite_float::float::arithmetic::asec::primitive_float_asec_rational;
/// use malachite_q::Rational;
///
/// // the arcsecant is NaN inside (-1, 1)
/// assert!(primitive_float_asec_rational::<f64>(&Rational::ONE_HALF).is_nan());
/// assert_eq!(
///     NiceFloat(primitive_float_asec_rational::<f64>(&Rational::ONE)),
///     NiceFloat(0.0)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_asec_rational::<f64>(
///         &Rational::NEGATIVE_ONE
///     )),
///     NiceFloat(3.141592653589793)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_asec_rational::<f64>(&Rational::TWO)),
///     NiceFloat(1.0471975511965979)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_asec_rational::<f32>(
///         &Rational::from_unsigneds(5u8, 3)
///     )),
///     NiceFloat(0.9272952)
/// );
/// ```
#[inline]
#[allow(clippy::type_repetition_in_bounds)]
pub fn primitive_float_asec_rational<T: PrimitiveFloat>(x: &Rational) -> T
where
    Float: PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float>,
{
    emulate_rational_to_float_fn(Float::asec_rational_prec_ref, x)
}

/// Computes $\operatorname{asec}(x)u/(2\pi)$, the arcsecant of a primitive float measured in $u$ths
/// of a turn (so that `u = 360` gives degrees), returning the result as a primitive float.
///
/// This is `primitive_float_asec` scaled by $u/(2\pi)$: see [`primitive_float_asec`] and
/// [`Float::asec_with_period_prec_round`] for the error bounds and the special cases. NaN and every
/// $|x|<1$ give NaN, even when $u=0$; $\pm\infty$ gives $u/4$; a zero period gives $0.0$; $1$ gives
/// $0.0$; $-1$ gives $u/2$; and $\pm2$ give $u/6$ and $u/3$ when $u$ is a multiple of 3.
///
/// Overflow is not possible, since $f(x,u) \leq u/2 < 2^{63}$, and neither is underflow: an `f32`
/// or `f64` is never close enough to 1 for that.
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
/// use malachite_float::float::arithmetic::asec::primitive_float_asec_with_period;
///
/// assert!(primitive_float_asec_with_period(f32::NAN, 360).is_nan());
/// // the arcsecant is NaN inside (-1, 1)
/// assert!(primitive_float_asec_with_period(0.5f32, 360).is_nan());
/// // an infinite input is a quarter turn, an input of 2 a sixth of one, and -1 a half turn
/// assert_eq!(
///     NiceFloat(primitive_float_asec_with_period(f32::INFINITY, 360)),
///     NiceFloat(90.0)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_asec_with_period(2.0f32, 360)),
///     NiceFloat(60.0)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_asec_with_period(-1.0f32, 360)),
///     NiceFloat(180.0)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_asec_with_period(1.5f32, 360)),
///     NiceFloat(48.189686)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_asec_with_period(1.5f64, 360)),
///     NiceFloat(48.189685104221404)
/// );
/// ```
#[inline]
#[allow(clippy::type_repetition_in_bounds)]
pub fn primitive_float_asec_with_period<T: PrimitiveFloat>(x: T, u: u64) -> T
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    emulate_float_to_float_fn(|x, prec| Float::asec_with_period_prec(x, u, prec), x)
}
