// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Float;
use crate::InnerFloat::{Finite, Infinity, NaN, Zero};
use crate::float::MAX_EXPONENT_I64;
use crate::float::arithmetic::acsc::signed_half_pi;
use crate::float::arithmetic::atan::{
    arc_with_period_scale, atan_rational_helper, scaled_unsigned,
};
use crate::float::arithmetic::round_near_x::{round_from_above, value_is_tie};
use crate::float::arithmetic::sin::{SCALE, SCALE_I64, SCALED_INPUT_EXPONENT};
use crate::{emulate_float_to_float_fn, emulate_rational_to_float_fn};
use core::cmp::Ordering::{self, Equal, Greater, Less};
use malachite_base::num::arithmetic::traits::{
    Abs, Acot, AcotAssign, CeilingLogBase2, IsPowerOf2, NegAssign, PowerOf2, Reciprocal,
};
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::{NaN as NaNTrait, NegativeZero, Zero as ZeroTrait};
use malachite_base::num::comparison::traits::PartialOrdAbs;
use malachite_base::num::conversion::traits::{ExactFrom, RoundingFrom};
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{self, Exact, Nearest, Up};
use malachite_nz::natural::arithmetic::float::round::float_can_round;
use malachite_nz::platform::Limb;
use malachite_q::Rational;

// Computes acot(|x|) for a finite `Float` x with |x| neither 0 nor 1, rounded to precision `prec`
// with rounding mode `rm`. The caller restores the sign, the arccotangent being odd.
//
// MPFR has no arccotangent. Here it is the arctangent of the reciprocal, acot(x) = atan(1/x), which
// unlike the arcsecant's and arccosecant's identities loses nothing to the reciprocal's rounding:
// the arctangent is smooth everywhere, so the reciprocal's relative error passes through
// undiminished and no subtraction has to be made exact. Below 1 the reciprocal is not taken at all,
// acot(x) = pi/2 - atan(x) there; that subtraction cannot cancel, since atan(x) < pi/4 leaves the
// result above pi/4.
fn acot_abs_prec_round(x: &Float, prec: u64, rm: RoundingMode) -> (Float, Ordering) {
    let exp_x = i64::from(x.get_exponent().unwrap());
    let xp = x.abs();
    // acot(x) = (1/x)(1 - 1/(3x^2) + ...), a relative correction below 2^(-2 EXP(x) - 1). Once that
    // is below the distance from 1/|x| to the nearest midpoint of the target precision -- at least
    // a relative 2^(-prec - p - 1) for a p-bit x -- the reciprocal alone decides the answer, bar
    // the exactly-representable and tie cases handled below. A Ziv loop cannot settle those at all,
    // `float_can_round` refusing an exactly representable result: for a power of two it would
    // balloon toward 2 EXP(x) bits, billions of them, trying to see a difference it can never
    // certify.
    if exp_x << 1 > MAX_EXPONENT_I64
        || exp_x << 1 > i64::exact_from(prec + x.get_prec().unwrap()) + 4
    {
        let tie = rm == Nearest && {
            let (wide, o_wide) = xp.reciprocal_prec_ref(prec + 1);
            value_is_tie(&wide, o_wide, prec)
        };
        let (t, o) = xp.reciprocal_prec_round(prec, rm);
        return round_from_above(t, o, tie, rm);
    }
    // |x| > 1 exactly when the exponent is positive, |x| = 1 having been handled by the caller
    let big = exp_x >= 1;
    let mut w = prec + prec.ceiling_log_base_2() + 10;
    let mut increment = Limb::WIDTH;
    loop {
        let t = if big {
            // The reciprocal is correctly rounded and the arctangent neither amplifies a relative
            // error nor adds more than its own half ulp.
            xp.reciprocal_prec_ref(w).0.atan_prec(w).0
        } else {
            // pi/2 and the arctangent each carry half an ulp, the subtraction a third, and the
            // result is at least half of pi/2, so the relative error is within a few ulps.
            (Float::pi_prec(w).0 >> 1u32)
                .sub_prec(xp.atan_prec_ref(w).0, w)
                .0
        };
        if float_can_round(t.significand_ref().unwrap(), w - 4, prec, rm) {
            return Float::from_float_prec_round(t, prec, rm);
        }
        w += increment;
        increment = w >> 1;
    }
}

// Computes acot(x) for a finite nonzero `Float` x, rounded to precision `prec` with rounding mode
// `rm`.
fn acot_prec_round_normal_ref(x: &Float, prec: u64, rm: RoundingMode) -> (Float, Ordering) {
    assert_ne!(rm, Exact, "Inexact acot");
    let negative = *x < 0u32;
    let rm_abs = if negative { -rm } else { rm };
    // acot(+-1) = +-pi/4; nothing else is exact either, pi/4 included
    let (t, o) = if x.partial_cmp_abs(&1u32).unwrap() == Equal {
        let (pi, o) = Float::pi_prec_round(prec, rm_abs);
        // exact
        (pi >> 2u32, o)
    } else {
        acot_abs_prec_round(x, prec, rm_abs)
    };
    // the arccotangent is odd, so the sign is stripped and restored, the rounding mode reflected
    // along with it
    if negative { (-t, o.reverse()) } else { (t, o) }
}

// Computes acot(x) for a nonzero `Rational` x, rounded to precision `prec` with rounding mode `rm`.
pub(crate) fn acot_rational_helper(x: &Rational, prec: u64, rm: RoundingMode) -> (Float, Ordering) {
    assert_ne!(rm, Exact, "Inexact acot_rational");
    let negative = *x < 0u32;
    let rm_abs = if negative { -rm } else { rm };
    let xp = x.abs();
    let (t, o) = match xp.partial_cmp(&1u32).unwrap() {
        // acot(+-1) = +-pi/4
        Equal => {
            let (pi, o) = Float::pi_prec_round(prec, rm_abs);
            (pi >> 2u32, o)
        }
        // acot(x) = atan(1/x), and the reciprocal of a `Rational` is exact, so this is the same
        // real number handed to the arctangent -- whose own small-input shortcut and underflow
        // handling then cover a huge |x|, where acot(x) is about 1/x.
        Greater => Float::atan_rational_prec_round((&xp).reciprocal(), prec, rm_abs),
        Less => {
            let mut w = prec + prec.ceiling_log_base_2() + 10;
            let mut increment = Limb::WIDTH;
            loop {
                // as in the `Float` case, pi/2 - atan(x) cannot cancel, the result staying above
                // pi/4
                let t = (Float::pi_prec(w).0 >> 1u32)
                    .sub_prec(atan_rational_helper(&xp, w, Nearest).0, w)
                    .0;
                if float_can_round(t.significand_ref().unwrap(), w - 4, prec, rm_abs) {
                    break Float::from_float_prec_round(t, prec, rm_abs);
                }
                w += increment;
                increment = w >> 1;
            }
        }
    };
    if negative { (-t, o.reverse()) } else { (t, o) }
}

// Computes acot(x) u/(2 pi) for a finite nonzero `Float` x, rounded to precision `prec` with
// rounding mode `rm`.
//
// The exact cases are the arctangent's, seen through the reciprocal: |x| = 1 gives an eighth of a
// turn, where the arctangent has |x| = 1 too. The arccotangent is odd, so it carries the sign of x.
fn acot_with_period_prec_round_normal_ref(
    x: &Float,
    u: u64,
    prec: u64,
    rm: RoundingMode,
) -> (Float, Ordering) {
    let positive = *x > 0u32;
    let exp_x = i64::from(x.get_exponent().unwrap());
    // |x| = 1: acotu(1, u) = u/8 and acotu(-1, u) = -u/8, both exact
    if exp_x == 1 && x.significand_ref().unwrap().is_power_of_2() {
        return scaled_unsigned(u, 3, positive, prec, rm);
    }
    // Nothing else can be rounded exactly
    assert_ne!(rm, Exact, "Inexact acot_with_period");
    // For 0 < x < 1, acot(x) = pi/2 - x r(x) with 0 < r(x) < 1, so acotu(x, u) = u/4 (1 - x s(x))
    // with 0 < s(x) < 1, and the function is odd. Once EXP(x) <= -prec - 3 that correction is below
    // an eighth of an ulp of u/4, so the result is the neighbour of u/4 on the side of zero, with
    // the sign of x. Requiring EXP(x) <= -64 as well keeps the correction below the last bit of u
    // when u/4 is inexact. Without this, a tiny x would send the Ziv loop below toward the
    // precision of x itself, the quarter turn being exactly representable.
    if exp_x <= -64 && exp_x <= -i64::exact_from(prec) - 3 {
        let w = if prec <= 63 { 65 } else { prec + 2 };
        // exact, since w >= 64
        let mut t = Float::from_unsigned_prec_round(u, w, Exact).0;
        t.decrement();
        // the last bit of t is 1 and w exceeds the target precision, so t is not representable
        // there, which pins the ternary value below
        t >>= 2u32;
        if !positive {
            t.neg_assign();
        }
        return Float::from_float_prec_round(t, prec, rm);
    }
    arc_with_period_scale(
        // scaling by a power of 2 is exact, and acot(x) u 2^SCALE stays far below the top of the
        // range, since |acot x| <= pi/2 and u < 2^64. Rounding away from zero is what the
        // arccotangent's large-x shortcut needs too, `Up` being its own reflection.
        |w| x.acot_prec_round_ref(w, Up).0 << SCALE,
        u,
        positive,
        prec,
        rm,
    )
}

// Computes acot(x) u/(2 pi) for a nonzero `Rational` x and a nonzero u, rounded to precision `prec`
// with rounding mode `rm`. (x = 0 and u = 0 are handled by the caller.)
//
// The exact cases are the arctangent's, seen through the reciprocal: |x| = 1 gives an eighth of a
// turn. The arccotangent is odd, so it carries the sign of x.
pub(crate) fn acot_with_period_rational_helper(
    x: &Rational,
    u: u64,
    prec: u64,
    rm: RoundingMode,
) -> (Float, Ordering) {
    let positive = *x > 0u32;
    let exp_x = x.floor_log_base_2_abs() + 1;
    // |x| = 1: acotu(1, u) = u/8 and acotu(-1, u) = -u/8, both exact
    if x.denominator_ref() == &1u32 && x.numerator_ref() == &1u32 {
        return scaled_unsigned(u, 3, positive, prec, rm);
    }
    // Nothing else can be rounded exactly
    assert_ne!(rm, Exact, "Inexact acot_with_period_rational");
    // As in the `Float` case, a tiny x is answered from the neighbour of u/4: acot(x) = pi/2 - x
    // r(x) with 0 < r(x) < 1, so the correction is below an eighth of an ulp of u/4 once EXP(x) is
    // at most -prec - 3. A `Rational` reaches far below the exponent range, where the general path
    // would work at a precision of the order of EXP(x), the quarter turn being exactly
    // representable.
    if exp_x <= -64 && exp_x <= -i64::exact_from(prec) - 3 {
        let w = if prec <= 63 { 65 } else { prec + 2 };
        // exact, since w >= 64
        let mut t = Float::from_unsigned_prec_round(u, w, Exact).0;
        t.decrement();
        // the last bit of t is 1 and w exceeds the target precision, so t is not representable
        // there, which pins the ternary value below
        t >>= 2u32;
        if !positive {
            t.neg_assign();
        }
        return Float::from_float_prec_round(t, prec, rm);
    }
    // An |x| large enough to put acot(x) = (1/x)(1 - O(x^-2)) below the smallest positive `Float`,
    // where `acot_rational_helper` would report an underflow -- but a large u can lift acot(x) u/(2
    // pi) back into the range, so the reciprocal, exact as a `Rational` and above acot(x) by less
    // than any reachable working precision can resolve, is taken here instead, scaled up by 2^SCALE
    // for the quotient. It keeps the sign of x, the arccotangent being odd.
    if 1 - exp_x <= SCALED_INPUT_EXPONENT {
        let scaled = Rational::power_of_2(SCALE_I64) / x;
        return arc_with_period_scale(
            |w| Float::from_rational_prec_round_ref(&scaled, w, Up).0,
            u,
            positive,
            prec,
            rm,
        );
    }
    arc_with_period_scale(
        // scaling by a power of 2 is exact, and acot(x) u 2^SCALE stays far below the top of the
        // range, since |acot x| <= pi/2 and u < 2^64. Rounding away from zero is what the
        // arccotangent's own large-x handling needs too, `Up` being its own reflection.
        |w| acot_rational_helper(x, w, Up).0 << SCALE,
        u,
        positive,
        prec,
        rm,
    )
}

impl Float {
    /// Computes $\operatorname{acot} x$, the arccotangent of a [`Float`], rounding the result to
    /// the specified precision and with the specified rounding mode. The [`Float`] is taken by
    /// value. An [`Ordering`] is also returned, indicating whether the rounded arccotangent is less
    /// than, equal to, or greater than the exact arccotangent. Although `NaN`s are not comparable
    /// to any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \operatorname{acot}(x)+\varepsilon.
    /// $$
    /// - If $x$ is NaN or infinite, $\varepsilon$ may be ignored or assumed to be 0.
    /// - Otherwise, if $m$ is not `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2
    ///   |\operatorname{acot}(x)|\rfloor-p+1}$.
    /// - Otherwise, if $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\operatorname{acot}(x)|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=\text{NaN}$
    /// - $f(\infty,p,m)=0.0$ and $f(-\infty,p,m)=-0.0$
    /// - $f(\pm0.0,p,m)=\pm\pi/2$, the values the arccotangent approaches from either side
    /// - $f(\pm1,p,m)=\pm\pi/4$
    ///
    /// The infinities are the only exact cases: $\pi/2$ and $\pi/4$ are never representable. This
    /// is the odd arccotangent, the arctangent of the reciprocal, whose range is $(-\pi/2,\pi/2]$
    /// and which jumps from $-\pi/2$ to $\pi/2$ at zero; the continuous branch with range $(0,\pi)$
    /// is $\pi/2-\arctan x$, and this function is not it.
    ///
    /// The arccotangent is odd, so $f(-x,p,m)=-f(x,p,-m)$, with $-m$ the reflection of $m$ that
    /// swaps `Floor` and `Ceiling`.
    ///
    /// Overflow is not possible, since $|\operatorname{acot}(x)| \leq \pi/2$. Underflow is not
    /// possible either: $|\operatorname{acot}(x)|$ is about $1/|x|$ for a large $|x|$, and a
    /// [`Float`]'s exponent is bounded, so the result stays above the smallest positive [`Float`].
    /// A [`Rational`] has no such bound; see [`Float::acot_rational_prec_round`].
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::acot_prec`] instead. If you
    /// know that your target precision is the precision of the input, consider using
    /// [`Float::acot_round`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `self.significant_bits()`: the arctangent of the reciprocal, or of $x$ itself below 1, is
    /// taken at a working precision of about $n$ bits, which costs the first term; the second is
    /// the reciprocal of an $m$-bit input. A large $x$ skips the arctangent, its arccotangent being
    /// the reciprocal of $|x|$ to within the working precision.
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
    /// let (c, o) = Float::TWO.acot_prec_round(10, Floor);
    /// assert_eq!(c.to_string(), "0.46338");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::TWO.acot_prec_round(10, Ceiling);
    /// assert_eq!(c.to_string(), "0.46387");
    /// assert_eq!(o, Greater);
    ///
    /// // an input of -1 gives -pi/4
    /// let (c, o) = Float::NEGATIVE_ONE.acot_prec_round(10, Nearest);
    /// assert_eq!(c.to_string(), "-0.78516");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn acot_prec_round(self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        self.acot_prec_round_ref(prec, rm)
    }

    /// Computes $\operatorname{acot} x$, the arccotangent of a [`Float`], rounding the result to
    /// the specified precision and with the specified rounding mode. The [`Float`] is taken by
    /// reference. An [`Ordering`] is also returned, indicating whether the rounded arccotangent is
    /// less than, equal to, or greater than the exact arccotangent. Although `NaN`s are not
    /// comparable to any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`Float::acot_prec_round`] for the error bounds, the special cases, and the complexity;
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
    /// let (c, o) = (&Float::TWO).acot_prec_round_ref(10, Floor);
    /// assert_eq!(c.to_string(), "0.46338");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = (&Float::NEGATIVE_ONE).acot_prec_round_ref(10, Nearest);
    /// assert_eq!(c.to_string(), "-0.78516");
    /// assert_eq!(o, Greater);
    /// ```
    pub fn acot_prec_round_ref(&self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        assert_ne!(prec, 0);
        match &self.0 {
            NaN => (Self::NAN, Equal),
            // the cotangent falls to zero as its argument grows, so an infinite input gives a zero
            // of the same sign -- exactly
            Infinity { sign } => (
                if *sign {
                    Self::ZERO
                } else {
                    Self::NEGATIVE_ZERO
                },
                Equal,
            ),
            // the cotangent of a signed zero is the infinity of that sign, so a signed zero gives
            // pi/2 of that sign: the arccotangent jumps there, and the sign picks the side
            Zero { sign } => {
                assert_ne!(rm, Exact, "Inexact acot");
                signed_half_pi(!*sign, prec, rm)
            }
            Finite { .. } => acot_prec_round_normal_ref(self, prec, rm),
        }
    }

    /// Computes $\operatorname{acot} x$, the arccotangent of a [`Float`], rounding the result to
    /// the nearest value of the specified precision. The [`Float`] is taken by value. An
    /// [`Ordering`] is also returned, indicating whether the rounded arccotangent is less than,
    /// equal to, or greater than the exact arccotangent. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// If the arccotangent is equidistant from two [`Float`]s with the specified precision, the
    /// [`Float`] with fewer 1s in its binary expansion is chosen.
    ///
    /// See [`Float::acot_prec_round`] for the error bounds, the special cases, and the complexity;
    /// this function behaves the same way.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::acot_prec_round`] instead.
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
    /// let (c, o) = Float::TWO.acot_prec(10);
    /// assert_eq!(c.to_string(), "0.46387");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = Float::from_unsigned_prec(2u32, 100).0.acot_prec(100);
    /// assert_eq!(c.to_string(), "0.46364760900080611621425623146131");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn acot_prec(self, prec: u64) -> (Self, Ordering) {
        self.acot_prec_round(prec, Nearest)
    }

    /// Computes $\operatorname{acot} x$, the arccotangent of a [`Float`], rounding the result to
    /// the nearest value of the specified precision. The [`Float`] is taken by reference. An
    /// [`Ordering`] is also returned, indicating whether the rounded arccotangent is less than,
    /// equal to, or greater than the exact arccotangent. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`Float::acot_prec`] and [`Float::acot_prec_round`]; this function behaves the same way.
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
    /// let (c, o) = (&Float::TWO).acot_prec_ref(10);
    /// assert_eq!(c.to_string(), "0.46387");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn acot_prec_ref(&self, prec: u64) -> (Self, Ordering) {
        self.acot_prec_round_ref(prec, Nearest)
    }

    /// Computes $\operatorname{acot} x$, the arccotangent of a [`Float`], rounding the result with
    /// the specified rounding mode. The precision of the output is the precision of the input. The
    /// [`Float`] is taken by value. An [`Ordering`] is also returned, indicating whether the
    /// rounded arccotangent is less than, equal to, or greater than the exact arccotangent.
    /// Although `NaN`s are not comparable to any [`Float`], whenever this function returns a `NaN`
    /// it also returns `Equal`.
    ///
    /// See [`Float::acot_prec_round`] for the error bounds, the special cases, and the complexity;
    /// this function behaves the same way, with `prec` the precision of the input.
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
    /// let (c, o) = x.acot_round(Floor);
    /// assert_eq!(c.to_string(), "0.46364760900080611621425623146091");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn acot_round(self, rm: RoundingMode) -> (Self, Ordering) {
        let prec = self.significant_bits();
        self.acot_prec_round(prec, rm)
    }

    /// Computes $\operatorname{acot} x$, the arccotangent of a [`Float`], rounding the result with
    /// the specified rounding mode. The precision of the output is the precision of the input. The
    /// [`Float`] is taken by reference. An [`Ordering`] is also returned, indicating whether the
    /// rounded arccotangent is less than, equal to, or greater than the exact arccotangent.
    /// Although `NaN`s are not comparable to any [`Float`], whenever this function returns a `NaN`
    /// it also returns `Equal`.
    ///
    /// See [`Float::acot_round`] and [`Float::acot_prec_round`]; this function behaves the same
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
    /// let (c, o) = (&x).acot_round_ref(Ceiling);
    /// assert_eq!(c.to_string(), "0.46364760900080611621425623146131");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn acot_round_ref(&self, rm: RoundingMode) -> (Self, Ordering) {
        self.acot_prec_round_ref(self.significant_bits(), rm)
    }

    /// Computes $\operatorname{acot} x$, the arccotangent of a [`Float`], in place, rounding the
    /// result to the specified precision and with the specified rounding mode. An [`Ordering`] is
    /// returned, indicating whether the rounded arccotangent is less than, equal to, or greater
    /// than the exact arccotangent. Although `NaN`s are not comparable to any [`Float`], whenever
    /// this function assigns a `NaN` it also returns `Equal`.
    ///
    /// See [`Float::acot_prec_round`] for the error bounds, the special cases, and the complexity;
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
    /// let o = x.acot_prec_round_assign(10, Floor);
    /// assert_eq!(x.to_string(), "0.46338");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn acot_prec_round_assign(&mut self, prec: u64, rm: RoundingMode) -> Ordering {
        let (s, o) = self.acot_prec_round_ref(prec, rm);
        *self = s;
        o
    }

    /// Computes $\operatorname{acot} x$, the arccotangent of a [`Float`], in place, rounding the
    /// result to the nearest value of the specified precision. An [`Ordering`] is returned,
    /// indicating whether the rounded arccotangent is less than, equal to, or greater than the
    /// exact arccotangent. Although `NaN`s are not comparable to any [`Float`], whenever this
    /// function assigns a `NaN` it also returns `Equal`.
    ///
    /// See [`Float::acot_prec`] and [`Float::acot_prec_round`]; this function behaves the same way.
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
    /// let o = x.acot_prec_assign(10);
    /// assert_eq!(x.to_string(), "0.46387");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn acot_prec_assign(&mut self, prec: u64) -> Ordering {
        self.acot_prec_round_assign(prec, Nearest)
    }

    /// Computes $\operatorname{acot} x$, the arccotangent of a [`Float`], in place, rounding the
    /// result with the specified rounding mode. The precision of the output is the precision of the
    /// input. An [`Ordering`] is returned, indicating whether the rounded arccotangent is less
    /// than, equal to, or greater than the exact arccotangent. Although `NaN`s are not comparable
    /// to any [`Float`], whenever this function assigns a `NaN` it also returns `Equal`.
    ///
    /// See [`Float::acot_round`] and [`Float::acot_prec_round`]; this function behaves the same
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
    /// let o = x.acot_round_assign(Floor);
    /// assert_eq!(x.to_string(), "0.46364760900080611621425623146091");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn acot_round_assign(&mut self, rm: RoundingMode) -> Ordering {
        self.acot_prec_round_assign(self.significant_bits(), rm)
    }

    /// Computes $\operatorname{acot} x$, the arccotangent of a [`Rational`], rounding the result to
    /// the specified precision and with the specified rounding mode and returning the result as a
    /// [`Float`]. The [`Rational`] is taken by value. An [`Ordering`] is also returned, indicating
    /// whether the rounded arccotangent is less than, equal to, or greater than the exact
    /// arccotangent.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \operatorname{acot}(x)+\varepsilon.
    /// $$
    /// - $\varepsilon$ is never zero: no [`Rational`] has a representable arccotangent.
    /// - Otherwise, if $m$ is not `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2
    ///   |\operatorname{acot}(x)|\rfloor-p+1}$.
    /// - Otherwise, if $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\operatorname{acot}(x)|\rfloor-p}$.
    ///
    /// The output has precision `prec`.
    ///
    /// Special cases:
    /// - $f(0,p,m)=\pi/2$, the value the positive side approaches; a [`Rational`] zero has no sign
    ///   to choose the other side with
    /// - $f(\pm1,p,m)=\pm\pi/4$
    ///
    /// There are no exact cases: $\pi/2$ and $\pi/4$ are never representable, and the infinities
    /// that give a zero are out of a [`Rational`]'s reach. See [`Float::acot_prec_round`] for the
    /// branch: this is the odd arccotangent, $\arctan(1/x)$, with range $(-\pi/2,\pi/2]$.
    ///
    /// The arccotangent is odd, so $f(-x,p,m)=-f(x,p,-m)$, with $-m$ the reflection of $m$ that
    /// swaps `Floor` and `Ceiling`.
    ///
    /// Underflow:
    /// - If $0<|f(x,p,m)|<2^{-2^{30}}$, and $m$ is `Floor`, `Down`, or `Nearest` with the result at
    ///   most $2^{-2^{30}-1}$ in magnitude, a zero of the result's sign is returned instead.
    /// - Otherwise, if $0<|f(x,p,m)|<2^{-2^{30}}$, $\pm2^{-2^{30}}$ is returned instead, with the
    ///   sign of the result.
    ///
    /// Overflow is not possible, since $|\operatorname{acot}(x)| \leq \pi/2$. Underflow, which the
    /// [`Float`] arccotangent cannot reach, is possible here: $\operatorname{acot}(x)$ is about
    /// $1/x$ for a large $|x|$, and a [`Rational`] has no exponent bound, so $|x|$ can be large
    /// enough to put the result below the smallest positive [`Float`].
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::acot_rational_prec`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `x.significant_bits()`: the reciprocal is exact, and its arctangent, or $\pi/2$ minus the
    /// arctangent of $x$ itself below 1, is taken at a working precision of about $n$ bits, which
    /// costs the first term; the second is the reciprocal. A large $x$ skips the arctangent, its
    /// arccotangent being the reciprocal of $|x|$ to within the working precision.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with the given precision (which is always the case).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{NegativeOne, Two};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = Float::acot_rational_prec_round(Rational::TWO, 10, Floor);
    /// assert_eq!(c.to_string(), "0.46338");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::acot_rational_prec_round(Rational::TWO, 10, Ceiling);
    /// assert_eq!(c.to_string(), "0.46387");
    /// assert_eq!(o, Greater);
    ///
    /// // an input of -1 gives -pi/4
    /// let (c, o) = Float::acot_rational_prec_round(Rational::NEGATIVE_ONE, 10, Nearest);
    /// assert_eq!(c.to_string(), "-0.78516");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    #[allow(clippy::needless_pass_by_value)]
    pub fn acot_rational_prec_round(x: Rational, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        Self::acot_rational_prec_round_ref(&x, prec, rm)
    }

    /// Computes $\operatorname{acot} x$, the arccotangent of a [`Rational`], rounding the result to
    /// the specified precision and with the specified rounding mode and returning the result as a
    /// [`Float`]. The [`Rational`] is taken by reference. An [`Ordering`] is also returned,
    /// indicating whether the rounded arccotangent is less than, equal to, or greater than the
    /// exact arccotangent.
    ///
    /// See [`Float::acot_rational_prec_round`] for the error bounds, the special cases, underflow,
    /// and the complexity; this function behaves the same way.
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
    ///     Float::acot_rational_prec_round_ref(&Rational::from_unsigneds(5u8, 3), 10, Floor);
    /// assert_eq!(c.to_string(), "0.54004");
    /// assert_eq!(o, Less);
    /// ```
    pub fn acot_rational_prec_round_ref(
        x: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        assert_ne!(prec, 0);
        if *x == 0u32 {
            // a `Rational` zero has no sign, so it takes the side the positive inputs approach
            assert_ne!(rm, Exact, "Inexact acot_rational");
            return signed_half_pi(false, prec, rm);
        }
        acot_rational_helper(x, prec, rm)
    }

    /// Computes $\operatorname{acot} x$, the arccotangent of a [`Rational`], rounding the result to
    /// the nearest value of the specified precision and returning the result as a [`Float`]. The
    /// [`Rational`] is taken by value. An [`Ordering`] is also returned, indicating whether the
    /// rounded arccotangent is less than, equal to, or greater than the exact arccotangent.
    ///
    /// If the arccotangent is equidistant from two [`Float`]s with the specified precision, the
    /// [`Float`] with fewer 1s in its binary expansion is chosen.
    ///
    /// See [`Float::acot_rational_prec_round`] for the error bounds, the special cases, underflow,
    /// and the complexity; this function behaves the same way.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::acot_rational_prec_round`] instead.
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
    /// let (c, o) = Float::acot_rational_prec(Rational::from_unsigneds(5u8, 3), 53);
    /// assert_eq!(c.to_string(), "0.54041950027058416");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn acot_rational_prec(x: Rational, prec: u64) -> (Self, Ordering) {
        Self::acot_rational_prec_round(x, prec, Nearest)
    }

    /// Computes $\operatorname{acot} x$, the arccotangent of a [`Rational`], rounding the result to
    /// the nearest value of the specified precision and returning the result as a [`Float`]. The
    /// [`Rational`] is taken by reference. An [`Ordering`] is also returned, indicating whether the
    /// rounded arccotangent is less than, equal to, or greater than the exact arccotangent.
    ///
    /// See [`Float::acot_rational_prec`] and [`Float::acot_rational_prec_round`]; this function
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
    /// let (c, o) = Float::acot_rational_prec_ref(&Rational::from_unsigneds(5u8, 3), 53);
    /// assert_eq!(c.to_string(), "0.54041950027058416");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn acot_rational_prec_ref(x: &Rational, prec: u64) -> (Self, Ordering) {
        Self::acot_rational_prec_round_ref(x, prec, Nearest)
    }
    /// Computes $\operatorname{acot}(x)u/(2\pi)$, the arccotangent of a [`Float`] measured in
    /// $u$ths of a turn, rounding the result to the specified precision and with the specified
    /// rounding mode. The [`Float`] is taken by value. An [`Ordering`] is also returned, indicating
    /// whether the rounded arccotangent is less than, equal to, or greater than the exact
    /// arccotangent. Although `NaN`s are not comparable to any [`Float`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,u,p,m) = \operatorname{acot}(x)u/(2\pi)+\varepsilon.
    /// $$
    /// - If $x$ is NaN, infinite, or zero, if $u = 0$, or if $|x|$ is 1, $\varepsilon$ may be
    ///   ignored or assumed to be 0.
    /// - Otherwise, if $m$ is not `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2
    ///   |\operatorname{acot}(x)u/(2\pi)|\rfloor-p+1}$.
    /// - Otherwise, if $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\operatorname{acot}(x)u/(2\pi)|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},u,p,m)=\text{NaN}$
    /// - $f(\pm\infty,u,p,m)=\pm0.0$
    /// - $f(x,0,p,m)=\pm0.0$, with the sign of $x$
    /// - $f(\pm0.0,u,p,m)=\pm u/4$, a quarter turn: the two sides of the arccotangent's jump at
    ///   zero, which a period makes exact
    /// - $f(\pm1,u,p,m)=\pm u/8$, an eighth of a turn
    ///
    /// Those are the only exact cases -- the arccotangent's exact values are the arctangent's, seen
    /// through the reciprocal -- and the turn fractions are exact only when $p$ is large enough to
    /// hold them. This is the odd arccotangent, $\arctan(1/x)$; see [`Float::acot_prec_round`].
    ///
    /// The arccotangent is odd, so $f(-x,u,p,m)=-f(x,u,p,-m)$, with $-m$ the reflection of $m$ that
    /// swaps `Floor` and `Ceiling`; a zero period gives a zero with the sign of $x$ for the same
    /// reason.
    ///
    /// Underflow:
    /// - If $0<|f(x,u,p,m)|<2^{-2^{30}}$, and $m$ is `Floor`, `Down`, or `Nearest` with the result
    ///   at most $2^{-2^{30}-1}$ in magnitude, a zero of the result's sign is returned instead.
    /// - Otherwise, if $0<|f(x,u,p,m)|<2^{-2^{30}}$, $\pm2^{-2^{30}}$ is returned instead, with the
    ///   sign of the result.
    ///
    /// Overflow is not possible, since $|f(x,u,p,m)| \leq u/4 < 2^{62}$. Underflow, which the
    /// arccotangent alone cannot reach, is possible here: $|\operatorname{acot}(x)|$ is about
    /// $1/|x|$, which for the largest [`Float`]s is only twice the smallest positive one, so a
    /// small $u$ carries the quotient below it.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::acot_with_period_prec`]
    /// instead. If you know that your target precision is the precision of the input, consider
    /// using [`Float::acot_with_period_round`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `self.significant_bits()`: the arccotangent is taken at a working precision of about $n$
    /// bits and scaled by $u/(2\pi)$, which needs $\pi$ to that many bits, and both cost the first
    /// term; the second is the reciprocal of an $m$-bit input. A large $x$ skips the arctangent,
    /// its arccotangent being the reciprocal of $|x|$ to within the working precision.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with the given precision.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{NegativeOne, One};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// // an input of 1 is an eighth of a turn, and one of -1 minus an eighth
    /// let (c, o) = Float::ONE.acot_with_period_prec_round(360, 10, Exact);
    /// assert_eq!(c.to_string(), "45.000");
    /// assert_eq!(o, Equal);
    ///
    /// let (c, o) = Float::NEGATIVE_ONE.acot_with_period_prec_round(360, 10, Exact);
    /// assert_eq!(c.to_string(), "-45.000");
    /// assert_eq!(o, Equal);
    ///
    /// let (c, o) = Float::from(2.5).acot_with_period_prec_round(360, 10, Floor);
    /// assert_eq!(c.to_string(), "21.781");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn acot_with_period_prec_round(
        self,
        u: u64,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        self.acot_with_period_prec_round_ref(u, prec, rm)
    }

    /// Computes $\operatorname{acot}(x)u/(2\pi)$, the arccotangent of a [`Float`] measured in
    /// $u$ths of a turn, rounding the result to the specified precision and with the specified
    /// rounding mode. The [`Float`] is taken by reference. An [`Ordering`] is also returned,
    /// indicating whether the rounded arccotangent is less than, equal to, or greater than the
    /// exact arccotangent. Although `NaN`s are not comparable to any [`Float`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`Float::acot_with_period_prec_round`] for the error bounds, the special and closed-form
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
    /// let (c, o) = (&Float::from(2.5)).acot_with_period_prec_round_ref(360, 10, Ceiling);
    /// assert_eq!(c.to_string(), "21.812");
    /// assert_eq!(o, Greater);
    /// ```
    pub fn acot_with_period_prec_round_ref(
        &self,
        u: u64,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        assert_ne!(prec, 0);
        match &self.0 {
            NaN => (Self::NAN, Equal),
            // acot(±infinity) = ±0, so acotu(±infinity, u) = ±0 for every u, zero included
            Infinity { sign } => (
                if *sign {
                    Self::ZERO
                } else {
                    Self::NEGATIVE_ZERO
                },
                Equal,
            ),
            // acot(±0) = ±pi/2, so acotu(±0, u) = ±u/4, a quarter turn -- and ±0 when u is
            // zero, as for every other input, which keeps the function odd
            Zero { sign } => {
                if u == 0 {
                    (
                        if *sign {
                            Self::ZERO
                        } else {
                            Self::NEGATIVE_ZERO
                        },
                        Equal,
                    )
                } else {
                    scaled_unsigned(u, 2, *sign, prec, rm)
                }
            }
            Finite { sign, .. } => {
                if u == 0 {
                    // acotu(x, 0) = 0 with the sign of x, which agrees with the infinite case and
                    // keeps the function odd
                    (
                        if *sign {
                            Self::ZERO
                        } else {
                            Self::NEGATIVE_ZERO
                        },
                        Equal,
                    )
                } else {
                    acot_with_period_prec_round_normal_ref(self, u, prec, rm)
                }
            }
        }
    }

    /// Computes $\operatorname{acot}(x)u/(2\pi)$, the arccotangent of a [`Float`] measured in
    /// $u$ths of a turn, rounding the result to the nearest value of the specified precision. The
    /// [`Float`] is taken by value. An [`Ordering`] is also returned, indicating whether the
    /// rounded arccotangent is less than, equal to, or greater than the exact arccotangent.
    /// Although `NaN`s are not comparable to any [`Float`], whenever this function returns a `NaN`
    /// it also returns `Equal`.
    ///
    /// If the arccotangent is equidistant from two [`Float`]s with the specified precision, the
    /// [`Float`] with fewer 1s in its binary expansion is chosen.
    ///
    /// See [`Float::acot_with_period_prec_round`] for the error bounds, the special and closed-form
    /// cases, underflow, and the complexity; this function behaves the same way.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::acot_with_period_prec_round`] instead.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = Float::from(2.5).acot_with_period_prec(360, 10);
    /// assert_eq!(c.to_string(), "21.812");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = Float::from(2.5).acot_with_period_prec(360, 53);
    /// assert_eq!(c.to_string(), "21.801409486351812");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn acot_with_period_prec(self, u: u64, prec: u64) -> (Self, Ordering) {
        self.acot_with_period_prec_round(u, prec, Nearest)
    }

    /// Computes $\operatorname{acot}(x)u/(2\pi)$, the arccotangent of a [`Float`] measured in
    /// $u$ths of a turn, rounding the result to the nearest value of the specified precision. The
    /// [`Float`] is taken by reference. An [`Ordering`] is also returned, indicating whether the
    /// rounded arccotangent is less than, equal to, or greater than the exact arccotangent.
    /// Although `NaN`s are not comparable to any [`Float`], whenever this function returns a `NaN`
    /// it also returns `Equal`.
    ///
    /// See [`Float::acot_with_period_prec`] and [`Float::acot_with_period_prec_round`]; this
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
    /// let (c, o) = (&Float::from(2.5)).acot_with_period_prec_ref(360, 53);
    /// assert_eq!(c.to_string(), "21.801409486351812");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn acot_with_period_prec_ref(&self, u: u64, prec: u64) -> (Self, Ordering) {
        self.acot_with_period_prec_round_ref(u, prec, Nearest)
    }

    /// Computes $\operatorname{acot}(x)u/(2\pi)$, the arccotangent of a [`Float`] measured in
    /// $u$ths of a turn, rounding the result with the specified rounding mode. The precision of the
    /// output is the precision of the input. The [`Float`] is taken by value. An [`Ordering`] is
    /// also returned, indicating whether the rounded arccotangent is less than, equal to, or
    /// greater than the exact arccotangent. Although `NaN`s are not comparable to any [`Float`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`Float::acot_with_period_prec_round`] for the error bounds, the special and closed-form
    /// cases, underflow, and the complexity; this function behaves the same way, with `prec` the
    /// precision of the input.
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
    /// let x = Float::from_unsigned_prec(5u32, 100).0 >> 1u32;
    /// let (c, o) = x.acot_with_period_round(360, Floor);
    /// assert_eq!(c.to_string(), "21.801409486351811770244866086938");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn acot_with_period_round(self, u: u64, rm: RoundingMode) -> (Self, Ordering) {
        let prec = self.significant_bits();
        self.acot_with_period_prec_round(u, prec, rm)
    }

    /// Computes $\operatorname{acot}(x)u/(2\pi)$, the arccotangent of a [`Float`] measured in
    /// $u$ths of a turn, rounding the result with the specified rounding mode. The precision of the
    /// output is the precision of the input. The [`Float`] is taken by reference. An [`Ordering`]
    /// is also returned, indicating whether the rounded arccotangent is less than, equal to, or
    /// greater than the exact arccotangent. Although `NaN`s are not comparable to any [`Float`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`Float::acot_with_period_round`] and [`Float::acot_with_period_prec_round`]; this
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
    /// let x = Float::from_unsigned_prec(5u32, 100).0 >> 1u32;
    /// let (c, o) = (&x).acot_with_period_round_ref(360, Ceiling);
    /// assert_eq!(c.to_string(), "21.801409486351811770244866086963");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn acot_with_period_round_ref(&self, u: u64, rm: RoundingMode) -> (Self, Ordering) {
        self.acot_with_period_prec_round_ref(u, self.significant_bits(), rm)
    }

    /// Computes $\operatorname{acot}(x)u/(2\pi)$, the arccotangent of a [`Float`] measured in
    /// $u$ths of a turn, rounding the result to the precision of the input and to the nearest
    /// [`Float`]. The [`Float`] is taken by value.
    ///
    /// If the arccotangent is equidistant from two [`Float`]s with the precision of the input, the
    /// [`Float`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a
    /// description of the `Nearest` rounding mode.
    ///
    /// See [`Float::acot_with_period_prec_round`] for the error bounds, the special and closed-form
    /// cases, underflow, and the complexity; this function behaves the same way, with `prec` the
    /// precision of the input and `Nearest` rounding.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::acot_with_period_round`] instead. If you want to specify an output precision,
    /// consider using [`Float::acot_with_period_prec`]. If you want both of these things, consider
    /// using [`Float::acot_with_period_prec_round`].
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    ///
    /// let x = Float::from_unsigned_prec(5u32, 100).0 >> 1u32;
    /// assert_eq!(
    ///     x.acot_with_period(360).to_string(),
    ///     "21.801409486351811770244866086938"
    /// );
    /// ```
    #[inline]
    pub fn acot_with_period(self, u: u64) -> Self {
        let prec = self.significant_bits();
        self.acot_with_period_prec(u, prec).0
    }

    /// Computes $\operatorname{acot}(x)u/(2\pi)$, the arccotangent of a [`Float`] measured in
    /// $u$ths of a turn, rounding the result to the precision of the input and to the nearest
    /// [`Float`]. The [`Float`] is taken by reference.
    ///
    /// See [`Float::acot_with_period`] and [`Float::acot_with_period_prec_round`]; this function
    /// behaves the same way.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    ///
    /// let x = Float::from_unsigned_prec(5u32, 100).0 >> 1u32;
    /// assert_eq!(
    ///     (&x).acot_with_period_ref(360).to_string(),
    ///     "21.801409486351811770244866086938"
    /// );
    /// ```
    #[inline]
    pub fn acot_with_period_ref(&self, u: u64) -> Self {
        self.acot_with_period_prec_ref(u, self.significant_bits()).0
    }

    /// Computes $\operatorname{acot}(x)u/(2\pi)$, the arccotangent of a [`Float`] measured in
    /// $u$ths of a turn, in place, rounding the result to the specified precision and with the
    /// specified rounding mode. An [`Ordering`] is returned, indicating whether the rounded
    /// arccotangent is less than, equal to, or greater than the exact arccotangent. Although `NaN`s
    /// are not comparable to any [`Float`], whenever this function assigns a `NaN` it also returns
    /// `Equal`.
    ///
    /// See [`Float::acot_with_period_prec_round`] for the error bounds, the special and closed-form
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
    /// let mut x = Float::from(2.5);
    /// let o = x.acot_with_period_prec_round_assign(360, 10, Floor);
    /// assert_eq!(x.to_string(), "21.781");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn acot_with_period_prec_round_assign(
        &mut self,
        u: u64,
        prec: u64,
        rm: RoundingMode,
    ) -> Ordering {
        let (s, o) = self.acot_with_period_prec_round_ref(u, prec, rm);
        *self = s;
        o
    }

    /// Computes $\operatorname{acot}(x)u/(2\pi)$, the arccotangent of a [`Float`] measured in
    /// $u$ths of a turn, in place, rounding the result to the nearest value of the specified
    /// precision. An [`Ordering`] is returned, indicating whether the rounded arccotangent is less
    /// than, equal to, or greater than the exact arccotangent. Although `NaN`s are not comparable
    /// to any [`Float`], whenever this function assigns a `NaN` it also returns `Equal`.
    ///
    /// See [`Float::acot_with_period_prec`] and [`Float::acot_with_period_prec_round`]; this
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
    /// let mut x = Float::from(2.5);
    /// let o = x.acot_with_period_prec_assign(360, 10);
    /// assert_eq!(x.to_string(), "21.812");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn acot_with_period_prec_assign(&mut self, u: u64, prec: u64) -> Ordering {
        self.acot_with_period_prec_round_assign(u, prec, Nearest)
    }

    /// Computes $\operatorname{acot}(x)u/(2\pi)$, the arccotangent of a [`Float`] measured in
    /// $u$ths of a turn, in place, rounding the result with the specified rounding mode. The
    /// precision of the output is the precision of the input. An [`Ordering`] is returned,
    /// indicating whether the rounded arccotangent is less than, equal to, or greater than the
    /// exact arccotangent. Although `NaN`s are not comparable to any [`Float`], whenever this
    /// function assigns a `NaN` it also returns `Equal`.
    ///
    /// See [`Float::acot_with_period_round`] and [`Float::acot_with_period_prec_round`]; this
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
    /// let mut x = Float::from_unsigned_prec(5u32, 100).0 >> 1u32;
    /// let o = x.acot_with_period_round_assign(360, Floor);
    /// assert_eq!(x.to_string(), "21.801409486351811770244866086938");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn acot_with_period_round_assign(&mut self, u: u64, rm: RoundingMode) -> Ordering {
        self.acot_with_period_prec_round_assign(u, self.significant_bits(), rm)
    }

    /// Computes $\operatorname{acot}(x)u/(2\pi)$, the arccotangent of a [`Float`] measured in
    /// $u$ths of a turn, in place, rounding the result to the precision of the input and to the
    /// nearest [`Float`].
    ///
    /// If the arccotangent is equidistant from two [`Float`]s with the precision of the input, the
    /// [`Float`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a
    /// description of the `Nearest` rounding mode.
    ///
    /// See [`Float::acot_with_period`] and [`Float::acot_with_period_prec_round`]; this function
    /// behaves the same way.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    ///
    /// let mut x = Float::from_unsigned_prec(5u32, 100).0 >> 1u32;
    /// x.acot_with_period_assign(360);
    /// assert_eq!(x.to_string(), "21.801409486351811770244866086938");
    /// ```
    #[inline]
    pub fn acot_with_period_assign(&mut self, u: u64) {
        let prec = self.significant_bits();
        self.acot_with_period_prec_assign(u, prec);
    }
    /// Computes $\operatorname{acot}(x)u/(2\pi)$, the arccotangent of a [`Rational`] measured in
    /// $u$ths of a turn, rounding the result to the specified precision and with the specified
    /// rounding mode and returning the result as a [`Float`]. The [`Rational`] is taken by value.
    /// An [`Ordering`] is also returned, indicating whether the rounded arccotangent is less than,
    /// equal to, or greater than the exact arccotangent.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,u,p,m) = \operatorname{acot}(x)u/(2\pi)+\varepsilon.
    /// $$
    /// - If $x$ is zero, if $u = 0$, or if $|x|$ is 1, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - Otherwise, if $m$ is not `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2
    ///   |\operatorname{acot}(x)u/(2\pi)|\rfloor-p+1}$.
    /// - Otherwise, if $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\operatorname{acot}(x)u/(2\pi)|\rfloor-p}$.
    ///
    /// The output has precision `prec`.
    ///
    /// Special cases:
    /// - $f(x,0,p,m)=\pm0.0$, with the sign of $x$; a [`Rational`] zero has no sign, so it takes
    ///   the positive one
    /// - $f(0,u,p,m)=u/4$, a quarter turn, the value the positive side approaches
    /// - $f(\pm1,u,p,m)=\pm u/8$, an eighth of a turn
    ///
    /// Those are the only exact cases -- the arccotangent's exact values are the arctangent's, seen
    /// through the reciprocal -- and the turn fractions are exact only when $p$ is large enough to
    /// hold them. This is the odd arccotangent, $\arctan(1/x)$; see
    /// [`Float::acot_rational_prec_round`].
    ///
    /// The arccotangent is odd, so $f(-x,u,p,m)=-f(x,u,p,-m)$, with $-m$ the reflection of $m$ that
    /// swaps `Floor` and `Ceiling`.
    ///
    /// Underflow:
    /// - If $0<|f(x,u,p,m)|<2^{-2^{30}}$, and $m$ is `Floor`, `Down`, or `Nearest` with the result
    ///   at most $2^{-2^{30}-1}$ in magnitude, a zero of the result's sign is returned instead.
    /// - Otherwise, if $0<|f(x,u,p,m)|<2^{-2^{30}}$, $\pm2^{-2^{30}}$ is returned instead, with the
    ///   sign of the result.
    ///
    /// Overflow is not possible, since $|f(x,u,p,m)| \leq u/4 < 2^{62}$. Underflow needs a small
    /// $u$ together with a large $|x|$; a [`Rational`] has no exponent bound, so $|x|$ can be large
    /// enough for that at any $u$.
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`Float::acot_with_period_rational_prec`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `x.significant_bits()`: the reciprocal is exact, and its arctangent, or $\pi/2$ minus the
    /// arctangent of $x$ itself below 1, is taken at a working precision of about $n$ bits and
    /// scaled by $u/(2\pi)$, which needs $\pi$ to that many bits; those cost the first term, and
    /// the second is the reciprocal. A tiny $x$ skips the arctangent, its arccotangent being a
    /// quarter turn to within the working precision, and a large one is the reciprocal of $|x|$ to
    /// within it.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with the given precision.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{NegativeOne, One, Zero};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// // a zero is a quarter turn, an input of 1 an eighth, and one of -1 minus an eighth
    /// let (c, o) = Float::acot_with_period_rational_prec_round(Rational::ZERO, 360, 10, Exact);
    /// assert_eq!(c.to_string(), "90.000");
    /// assert_eq!(o, Equal);
    ///
    /// let (c, o) = Float::acot_with_period_rational_prec_round(Rational::ONE, 360, 10, Exact);
    /// assert_eq!(c.to_string(), "45.000");
    /// assert_eq!(o, Equal);
    ///
    /// let (c, o) =
    ///     Float::acot_with_period_rational_prec_round(Rational::NEGATIVE_ONE, 360, 10, Exact);
    /// assert_eq!(c.to_string(), "-45.000");
    /// assert_eq!(o, Equal);
    ///
    /// let (c, o) = Float::acot_with_period_rational_prec_round(
    ///     Rational::from_unsigneds(5u8, 3),
    ///     360,
    ///     10,
    ///     Floor,
    /// );
    /// assert_eq!(c.to_string(), "30.938");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    #[allow(clippy::needless_pass_by_value)]
    pub fn acot_with_period_rational_prec_round(
        x: Rational,
        u: u64,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        Self::acot_with_period_rational_prec_round_ref(&x, u, prec, rm)
    }

    /// Computes $\operatorname{acot}(x)u/(2\pi)$, the arccotangent of a [`Rational`] measured in
    /// $u$ths of a turn, rounding the result to the specified precision and with the specified
    /// rounding mode and returning the result as a [`Float`]. The [`Rational`] is taken by
    /// reference. An [`Ordering`] is also returned, indicating whether the rounded arccotangent is
    /// less than, equal to, or greater than the exact arccotangent.
    ///
    /// See [`Float::acot_with_period_rational_prec_round`] for the error bounds, the special and
    /// closed-form cases, underflow, and the complexity; this function behaves the same way.
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
    /// let (c, o) = Float::acot_with_period_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(5u8, 3),
    ///     360,
    ///     10,
    ///     Ceiling,
    /// );
    /// assert_eq!(c.to_string(), "30.969");
    /// assert_eq!(o, Greater);
    /// ```
    pub fn acot_with_period_rational_prec_round_ref(
        x: &Rational,
        u: u64,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        assert_ne!(prec, 0);
        if u == 0 {
            // acotu(x, 0) = 0 with the sign of x, which keeps the function odd; a `Rational` zero
            // has no sign, so it takes the positive one
            return (
                if *x < 0u32 {
                    Self::NEGATIVE_ZERO
                } else {
                    Self::ZERO
                },
                Equal,
            );
        }
        if *x == 0u32 {
            // acot(0) = pi/2, so acotu(0, u) = u/4, a quarter turn
            return scaled_unsigned(u, 2, true, prec, rm);
        }
        acot_with_period_rational_helper(x, u, prec, rm)
    }

    /// Computes $\operatorname{acot}(x)u/(2\pi)$, the arccotangent of a [`Rational`] measured in
    /// $u$ths of a turn, rounding the result to the nearest value of the specified precision and
    /// returning the result as a [`Float`]. The [`Rational`] is taken by value. An [`Ordering`] is
    /// also returned, indicating whether the rounded arccotangent is less than, equal to, or
    /// greater than the exact arccotangent.
    ///
    /// If the arccotangent is equidistant from two [`Float`]s with the specified precision, the
    /// [`Float`] with fewer 1s in its binary expansion is chosen.
    ///
    /// See [`Float::acot_with_period_rational_prec_round`] for the error bounds, the special and
    /// closed-form cases, underflow, and the complexity; this function behaves the same way.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::acot_with_period_rational_prec_round`] instead.
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
    ///     Float::acot_with_period_rational_prec(Rational::from_unsigneds(5u8, 3), 360, 10);
    /// assert_eq!(c.to_string(), "30.969");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) =
    ///     Float::acot_with_period_rational_prec(Rational::from_unsigneds(5u8, 3), 360, 53);
    /// assert_eq!(c.to_string(), "30.963756532073521");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn acot_with_period_rational_prec(x: Rational, u: u64, prec: u64) -> (Self, Ordering) {
        Self::acot_with_period_rational_prec_round(x, u, prec, Nearest)
    }

    /// Computes $\operatorname{acot}(x)u/(2\pi)$, the arccotangent of a [`Rational`] measured in
    /// $u$ths of a turn, rounding the result to the nearest value of the specified precision and
    /// returning the result as a [`Float`]. The [`Rational`] is taken by reference. An [`Ordering`]
    /// is also returned, indicating whether the rounded arccotangent is less than, equal to, or
    /// greater than the exact arccotangent.
    ///
    /// See [`Float::acot_with_period_rational_prec`] and
    /// [`Float::acot_with_period_rational_prec_round`]; this function behaves the same way.
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
    ///     Float::acot_with_period_rational_prec_ref(&Rational::from_unsigneds(5u8, 3), 360, 53);
    /// assert_eq!(c.to_string(), "30.963756532073521");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn acot_with_period_rational_prec_ref(x: &Rational, u: u64, prec: u64) -> (Self, Ordering) {
        Self::acot_with_period_rational_prec_round_ref(x, u, prec, Nearest)
    }
    /// Computes $\operatorname{acot}(x)/\pi$, the arccotangent of a [`Float`] measured in
    /// half-turns, rounding the result to the specified precision and with the specified rounding
    /// mode. The [`Float`] is taken by value. An [`Ordering`] is also returned, indicating whether
    /// the rounded arccotangent is less than, equal to, or greater than the exact arccotangent.
    /// Although `NaN`s are not comparable to any [`Float`], whenever this function returns a `NaN`
    /// it also returns `Equal`.
    ///
    /// This is `acot_with_period` with a period of 2: see [`Float::acot_with_period_prec_round`]
    /// for the error bounds, the special cases, underflow, and the complexity, with $u = 2$. Either
    /// infinity gives a zero of its sign, $\pm0.0$ give $\pm1/2$, and $\pm1$ give $\pm1/4$; all are
    /// exact at every precision, since a half and a quarter each need only one bit, and they are
    /// the only exact cases. Unlike the arcsecant's and arccosecant's half-turns, none of the exact
    /// cases is lost here. NaN gives NaN. Overflow is not possible, since
    /// $|\operatorname{acot}(x)/\pi| \leq 1/2$.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with the given precision.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::Infinity;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// // an infinity gives a zero, the arccotangent falling to nothing there
    /// let (c, o) = Float::INFINITY.acot_pi_prec_round(10, Exact);
    /// assert_eq!(c.to_string(), "0.0");
    /// assert_eq!(o, Equal);
    ///
    /// let (c, o) = Float::from(2.5).acot_pi_prec_round(10, Floor);
    /// assert_eq!(c.to_string(), "0.12109");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn acot_pi_prec_round(self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        self.acot_with_period_prec_round(2, prec, rm)
    }

    /// Computes $\operatorname{acot}(x)/\pi$, the arccotangent of a [`Float`] measured in
    /// half-turns, rounding the result to the specified precision and with the specified rounding
    /// mode. The [`Float`] is taken by reference. An [`Ordering`] is also returned, indicating
    /// whether the rounded arccotangent is less than, equal to, or greater than the exact
    /// arccotangent. Although `NaN`s are not comparable to any [`Float`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// This is `acot_with_period` with a period of 2: see
    /// [`Float::acot_with_period_prec_round_ref`] for the error bounds, the special cases,
    /// underflow, and the complexity, with $u = 2$. Either infinity gives a zero of its sign,
    /// $\pm0.0$ give $\pm1/2$, and $\pm1$ give $\pm1/4$; all are exact at every precision, and they
    /// are the only exact cases. NaN gives NaN. Overflow is not possible, since
    /// $|\operatorname{acot}(x)/\pi| \leq 1/2$.
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
    /// let (c, o) = (&Float::from(2.5)).acot_pi_prec_round_ref(10, Ceiling);
    /// assert_eq!(c.to_string(), "0.12122");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn acot_pi_prec_round_ref(&self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        self.acot_with_period_prec_round_ref(2, prec, rm)
    }

    /// Computes $\operatorname{acot}(x)/\pi$, the arccotangent of a [`Float`] measured in
    /// half-turns, rounding the result to the nearest value of the specified precision. The
    /// [`Float`] is taken by value. An [`Ordering`] is also returned, indicating whether the
    /// rounded arccotangent is less than, equal to, or greater than the exact arccotangent.
    /// Although `NaN`s are not comparable to any [`Float`], whenever this function returns a `NaN`
    /// it also returns `Equal`.
    ///
    /// If the arccotangent is equidistant from two [`Float`]s with the specified precision, the
    /// [`Float`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a
    /// description of the `Nearest` rounding mode.
    ///
    /// This is `acot_with_period` with a period of 2: see [`Float::acot_with_period_prec`] for the
    /// error bounds, the special cases, underflow, and the complexity, with $u = 2$. Either
    /// infinity gives a zero of its sign, $\pm0.0$ give $\pm1/2$, and $\pm1$ give $\pm1/4$; all are
    /// exact at every precision, and they are the only exact cases. NaN gives NaN. Overflow is not
    /// possible, since $|\operatorname{acot}(x)/\pi| \leq 1/2$.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::acot_pi_prec_round`] instead.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = Float::from(2.5).acot_pi_prec(10);
    /// assert_eq!(c.to_string(), "0.12109");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn acot_pi_prec(self, prec: u64) -> (Self, Ordering) {
        self.acot_with_period_prec(2, prec)
    }

    /// Computes $\operatorname{acot}(x)/\pi$, the arccotangent of a [`Float`] measured in
    /// half-turns, rounding the result to the nearest value of the specified precision. The
    /// [`Float`] is taken by reference. An [`Ordering`] is also returned, indicating whether the
    /// rounded arccotangent is less than, equal to, or greater than the exact arccotangent.
    /// Although `NaN`s are not comparable to any [`Float`], whenever this function returns a `NaN`
    /// it also returns `Equal`.
    ///
    /// See [`Float::acot_pi_prec`] and [`Float::acot_with_period_prec_round`]; this function
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
    /// let (c, o) = (&Float::from(2.5)).acot_pi_prec_ref(53);
    /// assert_eq!(c.to_string(), "0.12111894159084340");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn acot_pi_prec_ref(&self, prec: u64) -> (Self, Ordering) {
        self.acot_with_period_prec_ref(2, prec)
    }

    /// Computes $\operatorname{acot}(x)/\pi$, the arccotangent of a [`Float`] measured in
    /// half-turns, rounding the result with the specified rounding mode. The precision of the
    /// output is the precision of the input. The [`Float`] is taken by value. An [`Ordering`] is
    /// also returned, indicating whether the rounded arccotangent is less than, equal to, or
    /// greater than the exact arccotangent. Although `NaN`s are not comparable to any [`Float`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// This is `acot_with_period` with a period of 2: see [`Float::acot_with_period_round`] for the
    /// error bounds, the special cases, underflow, and the complexity, with $u = 2$. Either
    /// infinity gives a zero of its sign, $\pm0.0$ give $\pm1/2$, and $\pm1$ give $\pm1/4$; all are
    /// exact at every precision, and they are the only exact cases. NaN gives NaN. Overflow is not
    /// possible, since $|\operatorname{acot}(x)/\pi| \leq 1/2$.
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
    /// let x = Float::from_unsigned_prec(5u32, 10).0 >> 1u32;
    /// let (c, o) = x.acot_pi_round(Floor);
    /// assert_eq!(c.to_string(), "0.12109");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn acot_pi_round(self, rm: RoundingMode) -> (Self, Ordering) {
        self.acot_with_period_round(2, rm)
    }

    /// Computes $\operatorname{acot}(x)/\pi$, the arccotangent of a [`Float`] measured in
    /// half-turns, rounding the result with the specified rounding mode. The precision of the
    /// output is the precision of the input. The [`Float`] is taken by reference. An [`Ordering`]
    /// is also returned, indicating whether the rounded arccotangent is less than, equal to, or
    /// greater than the exact arccotangent. Although `NaN`s are not comparable to any [`Float`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// This is `acot_with_period` with a period of 2: see [`Float::acot_with_period_round_ref`] for
    /// the error bounds, the special cases, underflow, and the complexity, with $u = 2$. Either
    /// infinity gives a zero of its sign, $\pm0.0$ give $\pm1/2$, and $\pm1$ give $\pm1/4$; all are
    /// exact at every precision, and they are the only exact cases. NaN gives NaN. Overflow is not
    /// possible, since $|\operatorname{acot}(x)/\pi| \leq 1/2$.
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
    /// let x = Float::from_unsigned_prec(5u32, 10).0 >> 1u32;
    /// let (c, o) = (&x).acot_pi_round_ref(Ceiling);
    /// assert_eq!(c.to_string(), "0.12122");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn acot_pi_round_ref(&self, rm: RoundingMode) -> (Self, Ordering) {
        self.acot_with_period_round_ref(2, rm)
    }

    /// Computes $\operatorname{acot}(x)/\pi$, the arccotangent of a [`Float`] measured in
    /// half-turns, rounding the result to the precision of the input and to the nearest [`Float`].
    /// The [`Float`] is taken by value.
    ///
    /// If the arccotangent is equidistant from two [`Float`]s with the precision of the input, the
    /// [`Float`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a
    /// description of the `Nearest` rounding mode.
    ///
    /// This is `acot_with_period` with a period of 2: see [`Float::acot_with_period`] for the error
    /// bounds, the special cases, underflow, and the complexity, with $u = 2$. Either infinity
    /// gives a zero of its sign, $\pm0.0$ give $\pm1/2$, and $\pm1$ give $\pm1/4$; all are exact at
    /// every precision, and they are the only exact cases. NaN gives NaN. Overflow is not possible,
    /// since $|\operatorname{acot}(x)/\pi| \leq 1/2$.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::acot_pi_round`] instead. If you want to specify an output precision, consider using
    /// [`Float::acot_pi_prec`]. If you want both of these things, consider using
    /// [`Float::acot_pi_prec_round`].
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    ///
    /// let x = Float::from_unsigned_prec(5u32, 10).0 >> 1u32;
    /// assert_eq!(x.acot_pi().to_string(), "0.12109");
    /// ```
    #[inline]
    pub fn acot_pi(self) -> Self {
        self.acot_with_period(2)
    }

    /// Computes $\operatorname{acot}(x)/\pi$, the arccotangent of a [`Float`] measured in
    /// half-turns, rounding the result to the precision of the input and to the nearest [`Float`].
    /// The [`Float`] is taken by reference.
    ///
    /// See [`Float::acot_pi`] and [`Float::acot_with_period_prec_round`]; this function behaves the
    /// same way.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    ///
    /// let x = Float::from_unsigned_prec(5u32, 10).0 >> 1u32;
    /// assert_eq!((&x).acot_pi_ref().to_string(), "0.12109");
    /// ```
    #[inline]
    pub fn acot_pi_ref(&self) -> Self {
        self.acot_with_period_ref(2)
    }

    /// Computes $\operatorname{acot}(x)/\pi$, the arccotangent of a [`Float`] measured in
    /// half-turns, in place, rounding the result to the specified precision and with the specified
    /// rounding mode. An [`Ordering`] is returned, indicating whether the rounded arccotangent is
    /// less than, equal to, or greater than the exact arccotangent. Although `NaN`s are not
    /// comparable to any [`Float`], whenever this function assigns a `NaN` it also returns `Equal`.
    ///
    /// This is `acot_with_period` with a period of 2: see
    /// [`Float::acot_with_period_prec_round_assign`] for the error bounds, the special cases,
    /// underflow, and the complexity, with $u = 2$. Either infinity gives a zero of its sign,
    /// $\pm0.0$ give $\pm1/2$, and $\pm1$ give $\pm1/4$; all are exact at every precision, and they
    /// are the only exact cases. NaN gives NaN. Overflow is not possible, since
    /// $|\operatorname{acot}(x)/\pi| \leq 1/2$.
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
    /// let mut x = Float::from(2.5);
    /// let o = x.acot_pi_prec_round_assign(10, Floor);
    /// assert_eq!(x.to_string(), "0.12109");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn acot_pi_prec_round_assign(&mut self, prec: u64, rm: RoundingMode) -> Ordering {
        self.acot_with_period_prec_round_assign(2, prec, rm)
    }

    /// Computes $\operatorname{acot}(x)/\pi$, the arccotangent of a [`Float`] measured in
    /// half-turns, in place, rounding the result to the nearest value of the specified precision.
    /// An [`Ordering`] is returned, indicating whether the rounded arccotangent is less than, equal
    /// to, or greater than the exact arccotangent. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function assigns a `NaN` it also returns `Equal`.
    ///
    /// See [`Float::acot_pi_prec`] and [`Float::acot_with_period_prec_round`]; this function
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
    /// let mut x = Float::from(2.5);
    /// let o = x.acot_pi_prec_assign(10);
    /// assert_eq!(x.to_string(), "0.12109");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn acot_pi_prec_assign(&mut self, prec: u64) -> Ordering {
        self.acot_with_period_prec_assign(2, prec)
    }

    /// Computes $\operatorname{acot}(x)/\pi$, the arccotangent of a [`Float`] measured in
    /// half-turns, in place, rounding the result with the specified rounding mode. The precision of
    /// the output is the precision of the input. An [`Ordering`] is returned, indicating whether
    /// the rounded arccotangent is less than, equal to, or greater than the exact arccotangent.
    /// Although `NaN`s are not comparable to any [`Float`], whenever this function assigns a `NaN`
    /// it also returns `Equal`.
    ///
    /// See [`Float::acot_pi_round`] and [`Float::acot_with_period_prec_round`]; this function
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
    /// let mut x = Float::from_unsigned_prec(5u32, 10).0 >> 1u32;
    /// let o = x.acot_pi_round_assign(Floor);
    /// assert_eq!(x.to_string(), "0.12109");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn acot_pi_round_assign(&mut self, rm: RoundingMode) -> Ordering {
        self.acot_with_period_round_assign(2, rm)
    }

    /// Computes $\operatorname{acot}(x)/\pi$, the arccotangent of a [`Float`] measured in
    /// half-turns, in place, rounding the result to the precision of the input and to the nearest
    /// [`Float`].
    ///
    /// If the arccotangent is equidistant from two [`Float`]s with the precision of the input, the
    /// [`Float`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a
    /// description of the `Nearest` rounding mode.
    ///
    /// See [`Float::acot_pi`] and [`Float::acot_with_period_prec_round`]; this function behaves the
    /// same way.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    ///
    /// let mut x = Float::from_unsigned_prec(5u32, 10).0 >> 1u32;
    /// x.acot_pi_assign();
    /// assert_eq!(x.to_string(), "0.12109");
    /// ```
    #[inline]
    pub fn acot_pi_assign(&mut self) {
        self.acot_with_period_assign(2);
    }

    /// Computes $\operatorname{acot}(x)/\pi$, the arccotangent of a [`Rational`] measured in
    /// half-turns, rounding the result to the specified precision and with the specified rounding
    /// mode and returning the result as a [`Float`]. The [`Rational`] is taken by value. An
    /// [`Ordering`] is also returned, indicating whether the rounded arccotangent is less than,
    /// equal to, or greater than the exact arccotangent.
    ///
    /// This is `acot_with_period_rational` with a period of 2: see
    /// [`Float::acot_with_period_rational_prec_round`] for the error bounds, the special cases,
    /// underflow, and the complexity, with $u = 2$. A zero gives $1/2$ and $\pm1$ give $\pm1/4$;
    /// all are exact at every precision, and they are the only exact cases, the infinities that
    /// give a zero being out of a [`Rational`]'s reach. A [`Rational`] zero has no sign, so it
    /// takes the positive side. Overflow is not possible, since $|\operatorname{acot}(x)/\pi| \leq
    /// 1/2$.
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
    /// // an input of -1 is minus an eighth of a turn, a quarter of a half-turn
    /// let (c, o) = Float::acot_pi_rational_prec_round(Rational::NEGATIVE_ONE, 10, Exact);
    /// assert_eq!(c.to_string(), "-0.25000");
    /// assert_eq!(o, Equal);
    ///
    /// let (c, o) =
    ///     Float::acot_pi_rational_prec_round(Rational::from_unsigneds(5u8, 3), 10, Floor);
    /// assert_eq!(c.to_string(), "0.17188");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn acot_pi_rational_prec_round(
        x: Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        Self::acot_with_period_rational_prec_round(x, 2, prec, rm)
    }

    /// Computes $\operatorname{acot}(x)/\pi$, the arccotangent of a [`Rational`] measured in
    /// half-turns, rounding the result to the specified precision and with the specified rounding
    /// mode and returning the result as a [`Float`]. The [`Rational`] is taken by reference. An
    /// [`Ordering`] is also returned, indicating whether the rounded arccotangent is less than,
    /// equal to, or greater than the exact arccotangent.
    ///
    /// See [`Float::acot_pi_rational_prec_round`] and
    /// [`Float::acot_with_period_rational_prec_round_ref`]; this function behaves the same way.
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
    ///     Float::acot_pi_rational_prec_round_ref(&Rational::from_unsigneds(5u8, 3), 10, Ceiling);
    /// assert_eq!(c.to_string(), "0.17212");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn acot_pi_rational_prec_round_ref(
        x: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        Self::acot_with_period_rational_prec_round_ref(x, 2, prec, rm)
    }

    /// Computes $\operatorname{acot}(x)/\pi$, the arccotangent of a [`Rational`] measured in
    /// half-turns, rounding the result to the nearest value of the specified precision and
    /// returning the result as a [`Float`]. The [`Rational`] is taken by value. An [`Ordering`] is
    /// also returned, indicating whether the rounded arccotangent is less than, equal to, or
    /// greater than the exact arccotangent.
    ///
    /// See [`Float::acot_pi_rational_prec_round`] and [`Float::acot_with_period_rational_prec`];
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
    /// let (c, o) = Float::acot_pi_rational_prec(Rational::from_unsigneds(5u8, 3), 53);
    /// assert_eq!(c.to_string(), "0.17202086962263066");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn acot_pi_rational_prec(x: Rational, prec: u64) -> (Self, Ordering) {
        Self::acot_with_period_rational_prec(x, 2, prec)
    }

    /// Computes $\operatorname{acot}(x)/\pi$, the arccotangent of a [`Rational`] measured in
    /// half-turns, rounding the result to the nearest value of the specified precision and
    /// returning the result as a [`Float`]. The [`Rational`] is taken by reference. An [`Ordering`]
    /// is also returned, indicating whether the rounded arccotangent is less than, equal to, or
    /// greater than the exact arccotangent.
    ///
    /// See [`Float::acot_pi_rational_prec`] and [`Float::acot_with_period_rational_prec_ref`]; this
    /// function behaves the same way.
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
    /// let (c, o) = Float::acot_pi_rational_prec_ref(&Rational::from_unsigneds(5u8, 3), 53);
    /// assert_eq!(c.to_string(), "0.17202086962263066");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn acot_pi_rational_prec_ref(x: &Rational, prec: u64) -> (Self, Ordering) {
        Self::acot_with_period_rational_prec_ref(x, 2, prec)
    }
}

impl Acot for Float {
    type Output = Self;

    /// Computes $\operatorname{acot} x$, the arccotangent of a [`Float`], taking it by value.
    ///
    /// If the output has a precision, it is the precision of the input. If the arccotangent is
    /// equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s in
    /// its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// $$
    /// f(x) = \operatorname{acot}(x)+\varepsilon.
    /// $$
    /// - If $x$ is NaN or infinite, $\varepsilon$ may be ignored or assumed to be 0.
    /// - Otherwise, $|\varepsilon| \leq 2^{\lfloor\log_2 |\operatorname{acot}(x)|\rfloor-p}$, where
    ///   $p$ is the precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN})=\text{NaN}$
    /// - $f(\infty)=0.0$ and $f(-\infty)=-0.0$
    /// - $f(\pm0.0)=\pm\pi/2$
    /// - $f(\pm1)=\pm\pi/4$
    ///
    /// Overflow and underflow are both impossible; see [`Float::acot_prec_round`].
    ///
    /// If you want to specify an output precision, consider using [`Float::acot_prec`] instead. If
    /// you want to specify a rounding mode as well, consider using [`Float::acot_prec_round`].
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the precision of the input, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Acot;
    /// use malachite_base::num::basic::traits::Two;
    /// use malachite_float::Float;
    ///
    /// assert_eq!(Float::TWO.acot().to_string(), "0.50");
    /// ```
    #[inline]
    fn acot(self) -> Self {
        let prec = self.significant_bits();
        self.acot_prec(prec).0
    }
}

impl Acot for &Float {
    type Output = Float;

    /// Computes $\operatorname{acot} x$, the arccotangent of a [`Float`], taking it by reference.
    ///
    /// See [`Acot::acot`] and [`Float::acot_prec_round`]; this function behaves the same way.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the precision of the input, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Acot;
    /// use malachite_base::num::basic::traits::Two;
    /// use malachite_float::Float;
    ///
    /// assert_eq!((&Float::TWO).acot().to_string(), "0.50");
    /// ```
    #[inline]
    fn acot(self) -> Float {
        self.acot_prec_ref(self.significant_bits()).0
    }
}

impl AcotAssign for Float {
    /// Replaces a [`Float`] with its arccotangent, $\operatorname{acot}(x)$.
    ///
    /// See [`Acot::acot`] and [`Float::acot_prec_round`]; this function behaves the same way.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the precision of the input, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::AcotAssign;
    /// use malachite_base::num::basic::traits::Two;
    /// use malachite_float::Float;
    ///
    /// let mut x = Float::TWO;
    /// x.acot_assign();
    /// assert_eq!(x.to_string(), "0.50");
    /// ```
    #[inline]
    fn acot_assign(&mut self) {
        let prec = self.significant_bits();
        self.acot_prec_assign(prec);
    }
}

/// Computes $\operatorname{acot} x$, the arccotangent of a primitive float, returning the result as
/// a primitive float.
///
/// This is the correctly rounded arccotangent: the exact $\operatorname{acot}(x)$ is rounded once,
/// to the nearest value of the input's type.
///
/// Special cases:
/// - $f(\text{NaN})=\text{NaN}$
/// - $f(\infty)=0.0$ and $f(-\infty)=-0.0$
/// - $f(\pm0.0)=\pm\pi/2$
/// - $f(\pm1)=\pm\pi/4$
///
/// This is the odd arccotangent, the arctangent of the reciprocal, with range $(-\pi/2,\pi/2]$.
/// Overflow is not possible, since $|\operatorname{acot}(x)| \leq \pi/2$, and neither is underflow:
/// a primitive float's exponent is bounded, so $1/|x|$ stays well inside the normal range.
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
/// use malachite_float::float::arithmetic::acot::primitive_float_acot;
///
/// assert!(primitive_float_acot(f32::NAN).is_nan());
/// // an input of zero gives pi/2, and one of 1 gives pi/4
/// assert_eq!(
///     NiceFloat(primitive_float_acot(0.0f32)),
///     NiceFloat(core::f32::consts::FRAC_PI_2)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_acot(1.0f32)),
///     NiceFloat(core::f32::consts::FRAC_PI_4)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_acot(f32::INFINITY)),
///     NiceFloat(0.0)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_acot(2.0f32)),
///     NiceFloat(0.4636476)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_acot(-2.0f32)),
///     NiceFloat(-0.4636476)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_acot(2.0f64)),
///     NiceFloat(0.4636476090008061)
/// );
/// ```
#[inline]
#[allow(clippy::type_repetition_in_bounds)]
pub fn primitive_float_acot<T: PrimitiveFloat>(x: T) -> T
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    emulate_float_to_float_fn(Float::acot_prec, x)
}

/// Computes $\operatorname{acot} x$, the arccotangent of a [`Rational`], returning the result as a
/// primitive float.
///
/// This is the correctly rounded arccotangent: the exact $\operatorname{acot}(x)$ is rounded once,
/// to the nearest value of the output type.
///
/// Special cases:
/// - $f(0)=\pi/2$
/// - $f(\pm1)=\pm\pi/4$
///
/// This is the odd arccotangent, the arctangent of the reciprocal, with range $(-\pi/2,\pi/2]$.
/// Overflow is not possible, since $|\operatorname{acot}(x)| \leq \pi/2$. The result is subnormal,
/// or zero, only when $|x|$ is large enough to put $1/|x|$ below the bottom of the output type's
/// normal range.
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
/// use malachite_base::num::basic::traits::{NegativeOne, One, Two, Zero};
/// use malachite_base::num::float::NiceFloat;
/// use malachite_float::float::arithmetic::acot::primitive_float_acot_rational;
/// use malachite_q::Rational;
///
/// // an input of zero gives pi/2
/// assert_eq!(
///     NiceFloat(primitive_float_acot_rational::<f64>(&Rational::ZERO)),
///     NiceFloat(core::f64::consts::FRAC_PI_2)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_acot_rational::<f64>(&Rational::ONE)),
///     NiceFloat(0.7853981633974483)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_acot_rational::<f64>(
///         &Rational::NEGATIVE_ONE
///     )),
///     NiceFloat(-0.7853981633974483)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_acot_rational::<f64>(&Rational::TWO)),
///     NiceFloat(0.4636476090008061)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_acot_rational::<f32>(
///         &Rational::from_unsigneds(5u8, 3)
///     )),
///     NiceFloat(0.5404195)
/// );
/// ```
#[inline]
#[allow(clippy::type_repetition_in_bounds)]
pub fn primitive_float_acot_rational<T: PrimitiveFloat>(x: &Rational) -> T
where
    Float: PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float>,
{
    emulate_rational_to_float_fn(Float::acot_rational_prec_ref, x)
}

/// Computes $\operatorname{acot}(x)u/(2\pi)$, the arccotangent of a primitive float measured in
/// $u$ths of a turn (so that `u = 360` gives degrees), returning the result as a primitive float.
///
/// This is `primitive_float_acot` scaled by $u/(2\pi)$: see [`Float::acot_with_period_prec_round`]
/// for the error bounds and the special cases. NaN gives NaN; $\pm\infty$ give $\pm0.0$; a zero
/// period gives a zero with the sign of $x$; $\pm0.0$ give $\pm u/4$, a quarter turn; and $\pm1$
/// give $\pm u/8$, an eighth. This is the odd arccotangent, $\arctan(1/x)$.
///
/// Overflow is not possible, since $|f(x,u)| \leq u/4 < 2^{62}$. The result is subnormal, or zero,
/// only when $u$ is small and $|x|$ is large enough to put $u/(2\pi|x|)$ below the bottom of the
/// type's normal range.
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
/// use malachite_float::float::arithmetic::acot::primitive_float_acot_with_period;
///
/// assert!(primitive_float_acot_with_period(f32::NAN, 360).is_nan());
/// assert_eq!(
///     NiceFloat(primitive_float_acot_with_period(f32::INFINITY, 360)),
///     NiceFloat(0.0)
/// );
/// // a zero is a quarter turn, an input of 1 an eighth, and one of -1 minus an eighth
/// assert_eq!(
///     NiceFloat(primitive_float_acot_with_period(0.0f32, 360)),
///     NiceFloat(90.0)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_acot_with_period(1.0f32, 360)),
///     NiceFloat(45.0)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_acot_with_period(-1.0f32, 360)),
///     NiceFloat(-45.0)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_acot_with_period(2.5f64, 360)),
///     NiceFloat(21.80140948635181)
/// );
/// ```
#[inline]
#[allow(clippy::type_repetition_in_bounds)]
pub fn primitive_float_acot_with_period<T: PrimitiveFloat>(x: T, u: u64) -> T
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    emulate_float_to_float_fn(|x, prec| Float::acot_with_period_prec(x, u, prec), x)
}

/// Computes $\operatorname{acot}(x)u/(2\pi)$, the arccotangent of a [`Rational`] measured in $u$ths
/// of a turn (so that `u = 360` gives degrees), returning the result as a primitive float.
///
/// This is `primitive_float_acot_rational` scaled by $u/(2\pi)$: see
/// [`Float::acot_with_period_rational_prec_round`] for the error bounds and the special cases. A
/// zero period gives a zero with the sign of $x$, a [`Rational`] zero having no sign and so taking
/// the positive one; a zero input gives $u/4$, a quarter turn; and $\pm1$ give $\pm u/8$, an
/// eighth. This is the odd arccotangent, $\arctan(1/x)$.
///
/// Overflow is not possible, since $|f(x,u)| \leq u/4 < 2^{62}$. The result is subnormal, or zero,
/// only when $u$ is small and $|x|$ is large enough to put $u/(2\pi|x|)$ below the bottom of the
/// type's normal range.
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
/// use malachite_float::float::arithmetic::acot::primitive_float_acot_with_period_rational;
/// use malachite_q::Rational;
///
/// // a zero is a quarter turn, an input of 1 an eighth, and one of -1 minus an eighth
/// assert_eq!(
///     NiceFloat(primitive_float_acot_with_period_rational::<f64>(
///         &Rational::ZERO,
///         360
///     )),
///     NiceFloat(90.0)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_acot_with_period_rational::<f64>(
///         &Rational::ONE,
///         360
///     )),
///     NiceFloat(45.0)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_acot_with_period_rational::<f64>(
///         &Rational::NEGATIVE_ONE,
///         360
///     )),
///     NiceFloat(-45.0)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_acot_with_period_rational::<f64>(
///         &Rational::from_unsigneds(5u8, 3),
///         360
///     )),
///     NiceFloat(30.96375653207352)
/// );
/// ```
#[inline]
#[allow(clippy::type_repetition_in_bounds)]
pub fn primitive_float_acot_with_period_rational<T: PrimitiveFloat>(x: &Rational, u: u64) -> T
where
    Float: PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float>,
{
    emulate_rational_to_float_fn(
        |x, prec| Float::acot_with_period_rational_prec_ref(x, u, prec),
        x,
    )
}

/// Computes $\operatorname{acot}(x)/\pi$, the arccotangent of a primitive float measured in
/// half-turns, returning the result as a primitive float.
///
/// This is `primitive_float_acot_with_period` with a period of 2: see
/// [`primitive_float_acot_with_period`] for the error bounds, the special cases, and the
/// complexity, with $u = 2$. Either infinity gives a zero of its sign, $\pm0.0$ give $\pm1/2$, and
/// $\pm1$ give $\pm1/4$; NaN gives NaN. Overflow is not possible, since
/// $|\operatorname{acot}(x)/\pi| \leq 1/2$.
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
/// use malachite_float::float::arithmetic::acot::primitive_float_acot_pi;
///
/// assert!(primitive_float_acot_pi(f32::NAN).is_nan());
/// // a zero is half a half-turn, and the arccotangent is defined inside (-1, 1) too
/// assert_eq!(NiceFloat(primitive_float_acot_pi(0.0f32)), NiceFloat(0.5));
/// assert_eq!(
///     NiceFloat(primitive_float_acot_pi(0.5f32)),
///     NiceFloat(0.3524164)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_acot_pi(f32::INFINITY)),
///     NiceFloat(0.0)
/// );
/// assert_eq!(NiceFloat(primitive_float_acot_pi(1.0f32)), NiceFloat(0.25));
/// assert_eq!(
///     NiceFloat(primitive_float_acot_pi(-1.0f32)),
///     NiceFloat(-0.25)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_acot_pi(2.5f32)),
///     NiceFloat(0.12111894)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_acot_pi(2.5f64)),
///     NiceFloat(0.1211189415908434)
/// );
/// ```
#[inline]
#[allow(clippy::type_repetition_in_bounds)]
pub fn primitive_float_acot_pi<T: PrimitiveFloat>(x: T) -> T
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    primitive_float_acot_with_period(x, 2)
}

/// Computes $\operatorname{acot}(x)/\pi$, the arccotangent of a [`Rational`] measured in
/// half-turns, returning the result as a primitive float.
///
/// This is `primitive_float_acot_with_period_rational` with a period of 2: see
/// [`primitive_float_acot_with_period_rational`] for the error bounds, the special cases, and the
/// complexity, with $u = 2$. A zero gives $1/2$ and $\pm1$ give $\pm1/4$. Overflow is not possible,
/// since $|\operatorname{acot}(x)/\pi| \leq 1/2$.
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
/// use malachite_base::num::basic::traits::{NegativeOne, One, OneHalf, Zero};
/// use malachite_base::num::float::NiceFloat;
/// use malachite_float::float::arithmetic::acot::primitive_float_acot_pi_rational;
/// use malachite_q::Rational;
///
/// // a zero is half a half-turn, and the arccotangent is defined inside (-1, 1) too
/// assert_eq!(
///     NiceFloat(primitive_float_acot_pi_rational::<f64>(&Rational::ZERO)),
///     NiceFloat(0.5)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_acot_pi_rational::<f64>(&Rational::ONE_HALF)),
///     NiceFloat(0.35241638234956674)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_acot_pi_rational::<f64>(&Rational::ONE)),
///     NiceFloat(0.25)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_acot_pi_rational::<f64>(
///         &Rational::NEGATIVE_ONE
///     )),
///     NiceFloat(-0.25)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_acot_pi_rational::<f64>(
///         &Rational::from_unsigneds(5u8, 3)
///     )),
///     NiceFloat(0.17202086962263066)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_acot_pi_rational::<f32>(
///         &Rational::from_unsigneds(5u8, 3)
///     )),
///     NiceFloat(0.17202087)
/// );
/// ```
#[inline]
#[allow(clippy::type_repetition_in_bounds)]
pub fn primitive_float_acot_pi_rational<T: PrimitiveFloat>(x: &Rational) -> T
where
    Float: PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float>,
{
    primitive_float_acot_with_period_rational(x, 2)
}
