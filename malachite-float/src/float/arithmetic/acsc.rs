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
use crate::float::arithmetic::atan::{arc_with_period_scale, scaled_unsigned};
use crate::float::arithmetic::sin::{SCALE, SCALE_I64, SCALED_INPUT_EXPONENT, scaled_underflow};
use crate::{emulate_float_to_float_fn, emulate_rational_to_float_fn};
use core::cmp::Ordering::{self, Equal, Greater, Less};
use core::cmp::max;
use malachite_base::num::arithmetic::traits::{
    Abs, Acsc, AcscAssign, Atan, CeilingLogBase2, IsPowerOf2, PowerOf2, Reciprocal, Square,
};
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::{NaN as NaNTrait, NegativeZero, One, Zero as ZeroTrait};
use malachite_base::num::comparison::traits::PartialOrdAbs;
use malachite_base::num::conversion::traits::{ExactFrom, RoundingFrom};
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{self, Ceiling, Down, Exact, Nearest, Up};
use malachite_nz::natural::arithmetic::float::round::float_can_round;
use malachite_nz::platform::Limb;
use malachite_q::Rational;

// acsc(1) = pi/2 and acsc(-1) = -pi/2, neither of them representable. The arccosecant being odd,
// the sign is stripped and restored with the rounding mode reflected along with it.
fn signed_half_pi(negative: bool, prec: u64, rm: RoundingMode) -> (Float, Ordering) {
    let (pi, o) = Float::pi_prec_round(prec, if negative { -rm } else { rm });
    // exact
    let half = pi >> 1u32;
    if negative {
        (-half, o.reverse())
    } else {
        (half, o)
    }
}

// Turns the correctly rounded 1/|x| into the correctly rounded acsc(|x|), for an |x| so large that
// x^2 would leave the exponent range. There 1/|x| is below 2^(1 - 2^29), so acsc(x), which exceeds
// it by a factor below 1 + 2^(-2^30), is nearer to it than any representable precision can resolve:
// the reciprocal's own rounding is the answer. The two exceptions are a reciprocal that lands
// exactly on a representable value and one that lands exactly on a tie, both of which have to move
// up, acsc(x) being strictly above 1/|x|.
fn acsc_from_huge_reciprocal(
    t: Float,
    o: Ordering,
    tie: bool,
    rm: RoundingMode,
) -> (Float, Ordering) {
    if o == Equal {
        return if rm == Ceiling || rm == Up {
            let mut t = t;
            t.increment();
            (t, Greater)
        } else {
            (t, Less)
        };
    }
    if tie {
        // `Nearest` broke the tie its own way; acsc(x) is above it, so the upper neighbour wins
        let mut t = t;
        if o == Greater {
            return (t, Greater);
        }
        t.increment();
        return (t, Greater);
    }
    (t, o)
}

// Whether `v`, the exact 1/|x|, is exactly halfway between two `prec`-bit `Float`s, which `Nearest`
// would otherwise break on its own.
fn reciprocal_is_tie(wide: &Float, o_wide: Ordering, prec: u64) -> bool {
    o_wide == Equal && Float::from_float_prec_round_ref(wide, prec, Down).1 != Equal
}

// Computes acsc(|x|) for a finite `Float` x with |x| > 1, rounded to precision `prec` with rounding
// mode `rm`. The caller restores the sign, the arccosecant being odd.
//
// MPFR has no arccosecant. Rather than take asin(1/x), which would round the reciprocal first and
// pay for it -- the arcsine is not Lipschitz at 1, and 1/x lands there exactly when x is near +-1,
// where about half the bits of the reciprocal would be lost -- the identity is used in the form
//
//     acsc(x) = atan(1/sqrt(x^2 - 1))
//
// for a positive x. The subtraction x^2 - 1 is where an x near 1 loses bits, and it is done at a
// precision wide enough to be exact: the square of a p-bit `Float` needs 2p bits, and their
// difference no more. So nothing is lost, and the cost does not grow as x approaches +-1. The
// reciprocal and the square root are taken together, by one correctly rounded `reciprocal_sqrt`.
fn acsc_abs_prec_round(x: &Float, prec: u64, rm: RoundingMode) -> (Float, Ordering) {
    let exp_x = i64::from(x.get_exponent().unwrap());
    // acsc(x) = (1/x)(1 + 1/(6x^2) + ...), so once 2 EXP(x) is past the target precision the
    // correction is invisible there and the answer is decided by 1/|x| together with the fact that
    // acsc(x) lies just above it. That has to be settled without a Ziv loop, and the test has to be
    // on `prec` rather than on the working precision: acsc(x) is within 2^(-2 EXP(x)) of 1/x, so a
    // loop would balloon to billions of bits for an extreme exponent, and a reciprocal taken at
    // such a precision falls out of the exponent range and flushes to zero. Keeping the test off
    // the working precision also keeps the square below from overflowing, since it is only reached
    // when 2 EXP(x) is at most prec + 66. The square below is the whole computation, and it is only
    // out of reach when it would leave the exponent range. There acsc(x) is within 2^(-2^30) of
    // 1/|x| -- closer than any representable precision can resolve -- so the reciprocal alone
    // decides the answer, and a Ziv loop is both unnecessary and ruinous: it would balloon toward 2
    // EXP(x) bits, billions of them, and a reciprocal taken at such a precision falls out of the
    // exponent range entirely.
    if exp_x << 1 > MAX_EXPONENT_I64 {
        let a = x.abs();
        let tie = rm == Nearest && {
            let (wide, o_wide) = a.reciprocal_prec_ref(prec + 1);
            reciprocal_is_tie(&wide, o_wide, prec)
        };
        let (t, o) = a.reciprocal_prec_round(prec, rm);
        return acsc_from_huge_reciprocal(t, o, tie, rm);
    }
    // the width at which x^2 - 1 is exact
    let exact_w = (x.get_prec().unwrap() << 1) + 2;
    let mut w = prec + prec.ceiling_log_base_2() + 10;
    let mut increment = Limb::WIDTH;
    loop {
        let q = x
            .square_prec_ref(max(w, exact_w))
            .0
            .sub_prec(Float::ONE, max(w, exact_w))
            .0
            .reciprocal_sqrt_prec(w)
            .0;
        // The reciprocal square root is correctly rounded and the arctangent neither amplifies a
        // relative error nor adds more than its own half ulp, so three bits of slack suffice.
        let t = q.atan();
        if float_can_round(t.significand_ref().unwrap(), w - 3, prec, rm) {
            return Float::from_float_prec_round(t, prec, rm);
        }
        w += increment;
        increment = w >> 1;
    }
}

// Computes acsc(x) for a finite nonzero `Float` x, rounded to precision `prec` with rounding mode
// `rm`.
fn acsc_prec_round_normal_ref(x: &Float, prec: u64, rm: RoundingMode) -> (Float, Ordering) {
    let negative = *x < 0u32;
    match x.partial_cmp_abs(&1u32).unwrap() {
        // acsc(x) = NaN for |x| < 1, the cosecant never taking a value there
        Less => (Float::NAN, Equal),
        // acsc(1) = pi/2 and acsc(-1) = -pi/2
        Equal => {
            assert_ne!(rm, Exact, "Inexact acsc");
            signed_half_pi(negative, prec, rm)
        }
        Greater => {
            assert_ne!(rm, Exact, "Inexact acsc");
            // the arccosecant is odd, so the sign is stripped and restored, the rounding mode
            // reflected along with it
            let (t, o) = acsc_abs_prec_round(x, prec, if negative { -rm } else { rm });
            if negative { (-t, o.reverse()) } else { (t, o) }
        }
    }
}

// Computes acsc(x) for a nonzero `Rational` x with |x| > 1, rounded to precision `prec` with
// rounding mode `rm`.
pub(crate) fn acsc_rational_helper(x: &Rational, prec: u64, rm: RoundingMode) -> (Float, Ordering) {
    assert_ne!(rm, Exact, "Inexact acsc_rational");
    let negative = *x < 0u32;
    let rm = if negative { -rm } else { rm };
    let exp_x = x.floor_log_base_2_abs() + 1;
    let xp = x.abs();
    let mut w = prec + prec.ceiling_log_base_2() + 10;
    let mut increment = Limb::WIDTH;
    // acsc(x) = (1/|x|)(1 + O(x^-2)) for a large |x|. A `Rational` has no exponent bound, so it can
    // be large enough to put that below the smallest positive `Float`, which the `Float`
    // arccosecant cannot reach; there the correction is invisible at any working precision the loop
    // can reach and the answer is 1/|x|, rounded. It is formed scaled up by 2^SCALE, and the
    // underflow is then decided by the rounding mode alone.
    if 1 - exp_x <= SCALED_INPUT_EXPONENT {
        let scaled = Rational::power_of_2(SCALE_I64) / &xp;
        loop {
            // rounded away from zero, the side acsc(x) is on
            let t = Float::from_rational_prec_round_ref(&scaled, w, Up).0;
            // `rm` is already reflected, and the value here is the positive |acsc(x)|, so the
            // underflow is decided on that side and the sign restored with it
            if let Some((t, o)) = scaled_underflow(&t, true, prec, rm) {
                return if negative { (-t, o.reverse()) } else { (t, o) };
            }
            let t = t >> SCALE;
            if float_can_round(t.significand_ref().unwrap(), w - 2, prec, rm) {
                let (t, o) = Float::from_float_prec_round(t, prec, rm);
                return if negative { (-t, o.reverse()) } else { (t, o) };
            }
            w += increment;
            increment = w >> 1;
        }
    }
    // As in the `Float` case, a square that would leave the exponent range is answered from the
    // reciprocal alone; the difference between 1/|x| and acsc(x) is below 2^(-2^30) of it, beyond
    // every representable precision. This also spares a huge `Rational` from being squared.
    if exp_x << 1 > MAX_EXPONENT_I64 {
        let recip = (&xp).reciprocal();
        let tie = rm == Nearest && {
            let (wide, o_wide) = Float::from_rational_prec_ref(&recip, prec + 1);
            reciprocal_is_tie(&wide, o_wide, prec)
        };
        let (t, o) = Float::from_rational_prec_round(recip, prec, rm);
        let (t, o) = acsc_from_huge_reciprocal(t, o, tie, rm);
        return if negative { (-t, o.reverse()) } else { (t, o) };
    }
    let mut r = None;
    loop {
        // exact, and positive since |x| > 1
        let r = r.get_or_insert_with(|| (&xp).square() - Rational::ONE);
        let q = Float::reciprocal_sqrt_rational_prec_ref(r, w).0;
        // The reciprocal square root is correctly rounded and the arctangent neither amplifies a
        // relative error nor adds more than its own half ulp, so three bits of slack suffice.
        let t = q.atan();
        if float_can_round(t.significand_ref().unwrap(), w - 3, prec, rm) {
            let (t, o) = Float::from_float_prec_round(t, prec, rm);
            return if negative { (-t, o.reverse()) } else { (t, o) };
        }
        w += increment;
        increment = w >> 1;
    }
}

// Computes acsc(x) u/(2 pi) for a finite `Float` x with |x| >= 1, rounded to precision `prec` with
// rounding mode `rm`.
//
// The exact cases are the arcsine's, seen through the reciprocal: |x| = 1 gives a quarter turn and
// |x| = 2 gives a twelfth, where the arcsine has |x| = 1 and |x| = 1/2. The arccosecant is odd, so
// each carries the sign of x.
fn acsc_with_period_prec_round_normal_ref(
    x: &Float,
    u: u64,
    prec: u64,
    rm: RoundingMode,
) -> (Float, Ordering) {
    let positive = *x > 0u32;
    let exp_x = i64::from(x.get_exponent().unwrap());
    let power_of_2 = x.significand_ref().unwrap().is_power_of_2();
    // |x| = 1: acscu(1, u) = u/4 and acscu(-1, u) = -u/4, both exact
    if exp_x == 1 && power_of_2 {
        return scaled_unsigned(u, 2, positive, prec, rm);
    }
    // acsc(+-2) = +-pi/6, so acscu(+-2, u) = +-u/12 is exact when u is a multiple of 3
    if exp_x == 2 && power_of_2 && u.is_multiple_of(3) {
        return scaled_unsigned(u / 3, 2, positive, prec, rm);
    }
    // Nothing else can be rounded exactly
    assert_ne!(rm, Exact, "Inexact acsc_with_period");
    arc_with_period_scale(
        // scaling by a power of 2 is exact, and acsc(x) u 2^SCALE stays far below the top of the
        // range, since |acsc x| <= pi/2 and u < 2^64. Rounding away from zero is what the
        // arccosecant's large-x shortcut needs too, `Up` being its own reflection.
        |w| x.acsc_prec_round_ref(w, Up).0 << SCALE,
        u,
        positive,
        prec,
        rm,
    )
}

// Computes acsc(x) u/(2 pi) for a `Rational` x with |x| >= 1, rounded to precision `prec` with
// rounding mode `rm`.
pub(crate) fn acsc_with_period_rational_helper(
    x: &Rational,
    u: u64,
    prec: u64,
    rm: RoundingMode,
) -> (Float, Ordering) {
    let positive = *x > 0u32;
    let exp_x = x.floor_log_base_2_abs() + 1;
    let integer = x.denominator_ref() == &1u32;
    // |x| = 1: acscu(1, u) = u/4 and acscu(-1, u) = -u/4, both exact
    if integer && x.numerator_ref() == &1u32 {
        return scaled_unsigned(u, 2, positive, prec, rm);
    }
    // acsc(+-2) = +-pi/6, so acscu(+-2, u) = +-u/12 is exact when u is a multiple of 3
    if integer && x.numerator_ref() == &2u32 && u.is_multiple_of(3) {
        return scaled_unsigned(u / 3, 2, positive, prec, rm);
    }
    // Nothing else can be rounded exactly
    assert_ne!(rm, Exact, "Inexact acsc_with_period_rational");
    // An |x| large enough to put acsc(x) = (1/x)(1 + O(x^-2)) below the smallest positive `Float`,
    // where `acsc_rational_helper` would report an underflow -- but a large u can lift acsc(x) u/(2
    // pi) back into the range, so the reciprocal, exact as a `Rational` and with the correction
    // invisible at any reachable working precision, is taken here instead, scaled up by 2^SCALE for
    // the quotient. It keeps the sign of x, the arccosecant being odd.
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
        // scaling by a power of 2 is exact, and acsc(x) u 2^SCALE stays far below the top of the
        // range, since |acsc x| <= pi/2 and u < 2^64
        |w| acsc_rational_helper(x, w, Up).0 << SCALE,
        u,
        positive,
        prec,
        rm,
    )
}

impl Float {
    /// Computes $\operatorname{acsc} x$, the arccosecant of a [`Float`], rounding the result to the
    /// specified precision and with the specified rounding mode. The [`Float`] is taken by value.
    /// An [`Ordering`] is also returned, indicating whether the rounded arccosecant is less than,
    /// equal to, or greater than the exact arccosecant. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \operatorname{acsc}(x)+\varepsilon.
    /// $$
    /// - If $x$ is NaN, infinite, or $|x|<1$, $\varepsilon$ may be ignored or assumed to be 0.
    /// - Otherwise, if $m$ is not `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2
    ///   |\operatorname{acsc}(x)|\rfloor-p+1}$.
    /// - Otherwise, if $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\operatorname{acsc}(x)|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=\text{NaN}$
    /// - $f(x,p,m)=\text{NaN}$ for $|x|<1$, including $\pm0.0$
    /// - $f(\infty,p,m)=0.0$ and $f(-\infty,p,m)=-0.0$
    /// - $f(1,p,m)=\pi/2$ and $f(-1,p,m)=-\pi/2$
    ///
    /// The infinities, and the inputs of magnitude below 1, are the only exact cases: $\pi/2$ is
    /// never representable.
    ///
    /// The arccosecant is odd, so $f(-x,p,m)=-f(x,p,-m)$, with $-m$ the reflection of $m$ that
    /// swaps `Floor` and `Ceiling`.
    ///
    /// Overflow is not possible, since $|\operatorname{acsc}(x)| \leq \pi/2$. Underflow is not
    /// possible either: $|\operatorname{acsc}(x)|$ is about $1/|x|$ for a large $|x|$, and a
    /// [`Float`]'s exponent is bounded, so the result stays above twice the smallest positive
    /// [`Float`]. A [`Rational`] has no such bound; see [`Float::acsc_rational_prec_round`].
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::acsc_prec`] instead. If you
    /// know that your target precision is the precision of the input, consider using
    /// [`Float::acsc_round`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `self.significant_bits()`: the arccosecant is taken at a working precision of about $n$
    /// bits, which costs the first term; the second is the exact square inside it. A large $x$
    /// skips the square, its arccosecant being the arctangent of the reciprocal of $|x|$ to within
    /// the working precision.
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
    /// let (c, o) = Float::TWO.acsc_prec_round(10, Floor);
    /// assert_eq!(c.to_string(), "0.52344");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::TWO.acsc_prec_round(10, Ceiling);
    /// assert_eq!(c.to_string(), "0.52441");
    /// assert_eq!(o, Greater);
    ///
    /// // an input of -1 gives -pi/2
    /// let (c, o) = Float::NEGATIVE_ONE.acsc_prec_round(10, Nearest);
    /// assert_eq!(c.to_string(), "-1.5703");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn acsc_prec_round(self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        self.acsc_prec_round_ref(prec, rm)
    }

    /// Computes $\operatorname{acsc} x$, the arccosecant of a [`Float`], rounding the result to the
    /// specified precision and with the specified rounding mode. The [`Float`] is taken by
    /// reference. An [`Ordering`] is also returned, indicating whether the rounded arccosecant is
    /// less than, equal to, or greater than the exact arccosecant. Although `NaN`s are not
    /// comparable to any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`Float::acsc_prec_round`] for the error bounds, the special cases, and the complexity;
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
    /// let (c, o) = (&Float::TWO).acsc_prec_round_ref(10, Floor);
    /// assert_eq!(c.to_string(), "0.52344");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = (&Float::NEGATIVE_ONE).acsc_prec_round_ref(10, Nearest);
    /// assert_eq!(c.to_string(), "-1.5703");
    /// assert_eq!(o, Greater);
    /// ```
    pub fn acsc_prec_round_ref(&self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        assert_ne!(prec, 0);
        match &self.0 {
            // the arccosecant is NaN inside (-1, 1), and both zeros are inside it
            NaN | Zero { .. } => (Self::NAN, Equal),
            // the cosecant falls to zero as its argument grows, so an infinite input gives a zero
            // of the same sign -- exactly
            Infinity { sign } => (
                if *sign {
                    Self::ZERO
                } else {
                    Self::NEGATIVE_ZERO
                },
                Equal,
            ),
            Finite { .. } => acsc_prec_round_normal_ref(self, prec, rm),
        }
    }

    /// Computes $\operatorname{acsc} x$, the arccosecant of a [`Float`], rounding the result to the
    /// nearest value of the specified precision. The [`Float`] is taken by value. An [`Ordering`]
    /// is also returned, indicating whether the rounded arccosecant is less than, equal to, or
    /// greater than the exact arccosecant. Although `NaN`s are not comparable to any [`Float`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// If the arccosecant is equidistant from two [`Float`]s with the specified precision, the
    /// [`Float`] with fewer 1s in its binary expansion is chosen.
    ///
    /// See [`Float::acsc_prec_round`] for the error bounds, the special cases, and the complexity;
    /// this function behaves the same way.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::acsc_prec_round`] instead.
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
    /// let (c, o) = Float::TWO.acsc_prec(10);
    /// assert_eq!(c.to_string(), "0.52344");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::from_unsigned_prec(2u32, 100).0.acsc_prec(100);
    /// assert_eq!(c.to_string(), "0.52359877559829887307710723054682");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn acsc_prec(self, prec: u64) -> (Self, Ordering) {
        self.acsc_prec_round(prec, Nearest)
    }

    /// Computes $\operatorname{acsc} x$, the arccosecant of a [`Float`], rounding the result to the
    /// nearest value of the specified precision. The [`Float`] is taken by reference. An
    /// [`Ordering`] is also returned, indicating whether the rounded arccosecant is less than,
    /// equal to, or greater than the exact arccosecant. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`Float::acsc_prec`] and [`Float::acsc_prec_round`]; this function behaves the same way.
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
    /// let (c, o) = (&Float::TWO).acsc_prec_ref(10);
    /// assert_eq!(c.to_string(), "0.52344");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn acsc_prec_ref(&self, prec: u64) -> (Self, Ordering) {
        self.acsc_prec_round_ref(prec, Nearest)
    }

    /// Computes $\operatorname{acsc} x$, the arccosecant of a [`Float`], rounding the result with
    /// the specified rounding mode. The precision of the output is the precision of the input. The
    /// [`Float`] is taken by value. An [`Ordering`] is also returned, indicating whether the
    /// rounded arccosecant is less than, equal to, or greater than the exact arccosecant. Although
    /// `NaN`s are not comparable to any [`Float`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// See [`Float::acsc_prec_round`] for the error bounds, the special cases, and the complexity;
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
    /// let (c, o) = x.acsc_round(Floor);
    /// assert_eq!(c.to_string(), "0.52359877559829887307710723054603");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn acsc_round(self, rm: RoundingMode) -> (Self, Ordering) {
        let prec = self.significant_bits();
        self.acsc_prec_round(prec, rm)
    }

    /// Computes $\operatorname{acsc} x$, the arccosecant of a [`Float`], rounding the result with
    /// the specified rounding mode. The precision of the output is the precision of the input. The
    /// [`Float`] is taken by reference. An [`Ordering`] is also returned, indicating whether the
    /// rounded arccosecant is less than, equal to, or greater than the exact arccosecant. Although
    /// `NaN`s are not comparable to any [`Float`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// See [`Float::acsc_round`] and [`Float::acsc_prec_round`]; this function behaves the same
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
    /// let (c, o) = (&x).acsc_round_ref(Ceiling);
    /// assert_eq!(c.to_string(), "0.52359877559829887307710723054682");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn acsc_round_ref(&self, rm: RoundingMode) -> (Self, Ordering) {
        self.acsc_prec_round_ref(self.significant_bits(), rm)
    }

    /// Computes $\operatorname{acsc} x$, the arccosecant of a [`Float`], in place, rounding the
    /// result to the specified precision and with the specified rounding mode. An [`Ordering`] is
    /// returned, indicating whether the rounded arccosecant is less than, equal to, or greater than
    /// the exact arccosecant. Although `NaN`s are not comparable to any [`Float`], whenever this
    /// function assigns a `NaN` it also returns `Equal`.
    ///
    /// See [`Float::acsc_prec_round`] for the error bounds, the special cases, and the complexity;
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
    /// let o = x.acsc_prec_round_assign(10, Floor);
    /// assert_eq!(x.to_string(), "0.52344");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn acsc_prec_round_assign(&mut self, prec: u64, rm: RoundingMode) -> Ordering {
        let (s, o) = self.acsc_prec_round_ref(prec, rm);
        *self = s;
        o
    }

    /// Computes $\operatorname{acsc} x$, the arccosecant of a [`Float`], in place, rounding the
    /// result to the nearest value of the specified precision. An [`Ordering`] is returned,
    /// indicating whether the rounded arccosecant is less than, equal to, or greater than the exact
    /// arccosecant. Although `NaN`s are not comparable to any [`Float`], whenever this function
    /// assigns a `NaN` it also returns `Equal`.
    ///
    /// See [`Float::acsc_prec`] and [`Float::acsc_prec_round`]; this function behaves the same way.
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
    /// let o = x.acsc_prec_assign(10);
    /// assert_eq!(x.to_string(), "0.52344");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn acsc_prec_assign(&mut self, prec: u64) -> Ordering {
        self.acsc_prec_round_assign(prec, Nearest)
    }

    /// Computes $\operatorname{acsc} x$, the arccosecant of a [`Float`], in place, rounding the
    /// result with the specified rounding mode. The precision of the output is the precision of the
    /// input. An [`Ordering`] is returned, indicating whether the rounded arccosecant is less than,
    /// equal to, or greater than the exact arccosecant. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function assigns a `NaN` it also returns `Equal`.
    ///
    /// See [`Float::acsc_round`] and [`Float::acsc_prec_round`]; this function behaves the same
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
    /// let o = x.acsc_round_assign(Floor);
    /// assert_eq!(x.to_string(), "0.52359877559829887307710723054603");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn acsc_round_assign(&mut self, rm: RoundingMode) -> Ordering {
        self.acsc_prec_round_assign(self.significant_bits(), rm)
    }

    /// Computes $\operatorname{acsc} x$, the arccosecant of a [`Rational`], rounding the result to
    /// the specified precision and with the specified rounding mode and returning the result as a
    /// [`Float`]. The [`Rational`] is taken by value. An [`Ordering`] is also returned, indicating
    /// whether the rounded arccosecant is less than, equal to, or greater than the exact
    /// arccosecant.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \operatorname{acsc}(x)+\varepsilon.
    /// $$
    /// - If $|x|<1$, $\varepsilon$ may be ignored or assumed to be 0.
    /// - Otherwise, if $m$ is not `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2
    ///   |\operatorname{acsc}(x)|\rfloor-p+1}$.
    /// - Otherwise, if $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\operatorname{acsc}(x)|\rfloor-p}$.
    ///
    /// The output has precision `prec`.
    ///
    /// Special cases:
    /// - $f(x,p,m)=\text{NaN}$ for $|x|<1$, including zero
    /// - $f(1,p,m)=\pi/2$ and $f(-1,p,m)=-\pi/2$
    ///
    /// The inputs of magnitude below 1 are the only exact cases, $\pi/2$ never being representable
    /// and the infinities that give a zero being out of a [`Rational`]'s reach.
    ///
    /// The arccosecant is odd, so $f(-x,p,m)=-f(x,p,-m)$, with $-m$ the reflection of $m$ that
    /// swaps `Floor` and `Ceiling`.
    ///
    /// Underflow:
    /// - If $0<|f(x,p,m)|<2^{-2^{30}}$, and $m$ is `Floor`, `Down`, or `Nearest` with the result at
    ///   most $2^{-2^{30}-1}$ in magnitude, a zero of the result's sign is returned instead.
    /// - Otherwise, if $0<|f(x,p,m)|<2^{-2^{30}}$, $\pm2^{-2^{30}}$ is returned instead, with the
    ///   sign of the result.
    ///
    /// Overflow is not possible, since $|\operatorname{acsc}(x)| \leq \pi/2$. Underflow, which the
    /// [`Float`] arccosecant cannot reach, is possible here: $\operatorname{acsc}(x)$ is about
    /// $1/x$ for a large $|x|$, and a [`Rational`] has no exponent bound, so $|x|$ can be large
    /// enough to put the result below the smallest positive [`Float`].
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::acsc_rational_prec`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `x.significant_bits()`: $x^2-1$ is formed exactly, and the arctangent of the reciprocal of
    /// its square root is taken at a working precision of about $n$ bits, which costs the first
    /// term; the second is the square. A large $x$ skips the square altogether, its arccosecant
    /// being the arctangent of the reciprocal of $|x|$ to within the working precision.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with the given precision (which is the case unless $|x|<1$).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{NegativeOne, Two};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = Float::acsc_rational_prec_round(Rational::TWO, 10, Floor);
    /// assert_eq!(c.to_string(), "0.52344");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::acsc_rational_prec_round(Rational::TWO, 10, Ceiling);
    /// assert_eq!(c.to_string(), "0.52441");
    /// assert_eq!(o, Greater);
    ///
    /// // an input of -1 gives -pi/2
    /// let (c, o) = Float::acsc_rational_prec_round(Rational::NEGATIVE_ONE, 10, Nearest);
    /// assert_eq!(c.to_string(), "-1.5703");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    #[allow(clippy::needless_pass_by_value)]
    pub fn acsc_rational_prec_round(x: Rational, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        Self::acsc_rational_prec_round_ref(&x, prec, rm)
    }

    /// Computes $\operatorname{acsc} x$, the arccosecant of a [`Rational`], rounding the result to
    /// the specified precision and with the specified rounding mode and returning the result as a
    /// [`Float`]. The [`Rational`] is taken by reference. An [`Ordering`] is also returned,
    /// indicating whether the rounded arccosecant is less than, equal to, or greater than the exact
    /// arccosecant.
    ///
    /// See [`Float::acsc_rational_prec_round`] for the error bounds, the special cases, underflow,
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
    ///     Float::acsc_rational_prec_round_ref(&Rational::from_unsigneds(5u8, 3), 10, Floor);
    /// assert_eq!(c.to_string(), "0.64258");
    /// assert_eq!(o, Less);
    /// ```
    pub fn acsc_rational_prec_round_ref(
        x: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        assert_ne!(prec, 0);
        match x.partial_cmp_abs(&1u32).unwrap() {
            // the arccosecant is NaN inside (-1, 1), zero included
            Less => (Self::NAN, Equal),
            // acsc(1) = pi/2 and acsc(-1) = -pi/2
            Equal => {
                assert_ne!(rm, Exact, "Inexact acsc_rational");
                signed_half_pi(*x < 0u32, prec, rm)
            }
            Greater => acsc_rational_helper(x, prec, rm),
        }
    }

    /// Computes $\operatorname{acsc} x$, the arccosecant of a [`Rational`], rounding the result to
    /// the nearest value of the specified precision and returning the result as a [`Float`]. The
    /// [`Rational`] is taken by value. An [`Ordering`] is also returned, indicating whether the
    /// rounded arccosecant is less than, equal to, or greater than the exact arccosecant.
    ///
    /// If the arccosecant is equidistant from two [`Float`]s with the specified precision, the
    /// [`Float`] with fewer 1s in its binary expansion is chosen.
    ///
    /// See [`Float::acsc_rational_prec_round`] for the error bounds, the special cases, underflow,
    /// and the complexity; this function behaves the same way.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::acsc_rational_prec_round`] instead.
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
    /// let (c, o) = Float::acsc_rational_prec(Rational::from_unsigneds(5u8, 3), 53);
    /// assert_eq!(c.to_string(), "0.64350110879328437");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn acsc_rational_prec(x: Rational, prec: u64) -> (Self, Ordering) {
        Self::acsc_rational_prec_round(x, prec, Nearest)
    }

    /// Computes $\operatorname{acsc} x$, the arccosecant of a [`Rational`], rounding the result to
    /// the nearest value of the specified precision and returning the result as a [`Float`]. The
    /// [`Rational`] is taken by reference. An [`Ordering`] is also returned, indicating whether the
    /// rounded arccosecant is less than, equal to, or greater than the exact arccosecant.
    ///
    /// See [`Float::acsc_rational_prec`] and [`Float::acsc_rational_prec_round`]; this function
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
    /// let (c, o) = Float::acsc_rational_prec_ref(&Rational::from_unsigneds(5u8, 3), 53);
    /// assert_eq!(c.to_string(), "0.64350110879328437");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn acsc_rational_prec_ref(x: &Rational, prec: u64) -> (Self, Ordering) {
        Self::acsc_rational_prec_round_ref(x, prec, Nearest)
    }

    /// Computes $\operatorname{acsc}(x)u/(2\pi)$, the arccosecant of a [`Float`] measured in $u$ths
    /// of a turn, rounding the result to the specified precision and with the specified rounding
    /// mode. The [`Float`] is taken by value. An [`Ordering`] is also returned, indicating whether
    /// the rounded arccosecant is less than, equal to, or greater than the exact arccosecant.
    /// Although `NaN`s are not comparable to any [`Float`], whenever this function returns a `NaN`
    /// it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,u,p,m) = \operatorname{acsc}(x)u/(2\pi)+\varepsilon.
    /// $$
    /// - If $x$ is NaN or infinite, if $|x|<1$, if $u = 0$, if $|x|$ is 1, or if $|x|$ is 2 and $u$
    ///   is a multiple of 3, $\varepsilon$ may be ignored or assumed to be 0.
    /// - Otherwise, if $m$ is not `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2
    ///   |\operatorname{acsc}(x)u/(2\pi)|\rfloor-p+1}$.
    /// - Otherwise, if $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\operatorname{acsc}(x)u/(2\pi)|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},u,p,m)=\text{NaN}$
    /// - $f(x,u,p,m)=\text{NaN}$ for $|x|<1$, including $\pm0.0$ and when $u=0$
    /// - $f(\pm\infty,u,p,m)=\pm0.0$
    /// - $f(x,0,p,m)=\pm0.0$ for $|x|\geq1$, with the sign of $x$
    /// - $f(\pm1,u,p,m)=\pm u/4$, a quarter turn
    /// - $f(\pm2,u,p,m)=\pm u/12$, a twelfth of a turn, when $u$ is a multiple of 3
    ///
    /// Those are the only exact cases -- the arccosecant's exact values are the arcsine's, seen
    /// through the reciprocal -- and the turn fractions are exact only when $p$ is large enough to
    /// hold them.
    ///
    /// The arccosecant is odd, so $f(-x,u,p,m)=-f(x,u,p,-m)$, with $-m$ the reflection of $m$ that
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
    /// arccosecant alone cannot reach, is possible here: $|\operatorname{acsc}(x)|$ is about
    /// $1/|x|$, which for the largest [`Float`]s is only twice the smallest positive one, so a
    /// small $u$ carries the quotient below it.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::acsc_with_period_prec`]
    /// instead. If you know that your target precision is the precision of the input, consider
    /// using [`Float::acsc_with_period_round`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `self.significant_bits()`: the arccosecant is taken at a working precision of about $n$ bits
    /// and scaled by $u/(2\pi)$, which needs $\pi$ to that many bits, and both cost the first term;
    /// the second is the exact square inside the arccosecant. A large $x$ skips the square, its
    /// arccosecant being the reciprocal of $|x|$ to within the working precision.
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
    /// // an input of 2 is a twelfth of a turn, and one of -1 minus a quarter
    /// let (c, o) = Float::TWO.acsc_with_period_prec_round(360, 10, Exact);
    /// assert_eq!(c.to_string(), "30.000");
    /// assert_eq!(o, Equal);
    ///
    /// let (c, o) = Float::NEGATIVE_ONE.acsc_with_period_prec_round(360, 10, Exact);
    /// assert_eq!(c.to_string(), "-90.000");
    /// assert_eq!(o, Equal);
    ///
    /// let (c, o) = Float::from(2.5).acsc_with_period_prec_round(360, 10, Floor);
    /// assert_eq!(c.to_string(), "23.562");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn acsc_with_period_prec_round(
        self,
        u: u64,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        self.acsc_with_period_prec_round_ref(u, prec, rm)
    }

    /// Computes $\operatorname{acsc}(x)u/(2\pi)$, the arccosecant of a [`Float`] measured in $u$ths
    /// of a turn, rounding the result to the specified precision and with the specified rounding
    /// mode. The [`Float`] is taken by reference. An [`Ordering`] is also returned, indicating
    /// whether the rounded arccosecant is less than, equal to, or greater than the exact
    /// arccosecant. Although `NaN`s are not comparable to any [`Float`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// See [`Float::acsc_with_period_prec_round`] for the error bounds, the special and closed-form
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
    /// let (c, o) = (&Float::from(2.5)).acsc_with_period_prec_round_ref(360, 10, Ceiling);
    /// assert_eq!(c.to_string(), "23.594");
    /// assert_eq!(o, Greater);
    /// ```
    pub fn acsc_with_period_prec_round_ref(
        &self,
        u: u64,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        assert_ne!(prec, 0);
        match &self.0 {
            // the arccosecant is NaN inside (-1, 1), and both zeros are inside it; this holds for a
            // zero period too, since NaN times 0 is NaN
            NaN | Zero { .. } => (Self::NAN, Equal),
            // acsc(±infinity) = ±0, so acscu(±infinity, u) = ±0 for every u, zero included
            Infinity { sign } => (
                if *sign {
                    Self::ZERO
                } else {
                    Self::NEGATIVE_ZERO
                },
                Equal,
            ),
            Finite { sign, .. } => {
                if self.lt_abs(&1u32) {
                    (Self::NAN, Equal)
                } else if u == 0 {
                    // acscu(x, 0) = 0 with the sign of x, which agrees with the infinite case and
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
                    acsc_with_period_prec_round_normal_ref(self, u, prec, rm)
                }
            }
        }
    }

    /// Computes $\operatorname{acsc}(x)u/(2\pi)$, the arccosecant of a [`Float`] measured in $u$ths
    /// of a turn, rounding the result to the nearest value of the specified precision. The
    /// [`Float`] is taken by value. An [`Ordering`] is also returned, indicating whether the
    /// rounded arccosecant is less than, equal to, or greater than the exact arccosecant. Although
    /// `NaN`s are not comparable to any [`Float`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// If the arccosecant is equidistant from two [`Float`]s with the specified precision, the
    /// [`Float`] with fewer 1s in its binary expansion is chosen.
    ///
    /// See [`Float::acsc_with_period_prec_round`] for the error bounds, the special and closed-form
    /// cases, underflow, and the complexity; this function behaves the same way.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::acsc_with_period_prec_round`] instead.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = Float::from(2.5).acsc_with_period_prec(360, 10);
    /// assert_eq!(c.to_string(), "23.594");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = Float::from(2.5).acsc_with_period_prec(360, 53);
    /// assert_eq!(c.to_string(), "23.578178478201831");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn acsc_with_period_prec(self, u: u64, prec: u64) -> (Self, Ordering) {
        self.acsc_with_period_prec_round(u, prec, Nearest)
    }

    /// Computes $\operatorname{acsc}(x)u/(2\pi)$, the arccosecant of a [`Float`] measured in $u$ths
    /// of a turn, rounding the result to the nearest value of the specified precision. The
    /// [`Float`] is taken by reference. An [`Ordering`] is also returned, indicating whether the
    /// rounded arccosecant is less than, equal to, or greater than the exact arccosecant. Although
    /// `NaN`s are not comparable to any [`Float`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// See [`Float::acsc_with_period_prec`] and [`Float::acsc_with_period_prec_round`]; this
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
    /// let (c, o) = (&Float::from(2.5)).acsc_with_period_prec_ref(360, 53);
    /// assert_eq!(c.to_string(), "23.578178478201831");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn acsc_with_period_prec_ref(&self, u: u64, prec: u64) -> (Self, Ordering) {
        self.acsc_with_period_prec_round_ref(u, prec, Nearest)
    }

    /// Computes $\operatorname{acsc}(x)u/(2\pi)$, the arccosecant of a [`Float`] measured in $u$ths
    /// of a turn, rounding the result with the specified rounding mode. The precision of the output
    /// is the precision of the input. The [`Float`] is taken by value. An [`Ordering`] is also
    /// returned, indicating whether the rounded arccosecant is less than, equal to, or greater than
    /// the exact arccosecant. Although `NaN`s are not comparable to any [`Float`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`Float::acsc_with_period_prec_round`] for the error bounds, the special and closed-form
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
    /// let (c, o) = x.acsc_with_period_round(360, Floor);
    /// assert_eq!(c.to_string(), "23.578178478201831104022499419824");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn acsc_with_period_round(self, u: u64, rm: RoundingMode) -> (Self, Ordering) {
        let prec = self.significant_bits();
        self.acsc_with_period_prec_round(u, prec, rm)
    }

    /// Computes $\operatorname{acsc}(x)u/(2\pi)$, the arccosecant of a [`Float`] measured in $u$ths
    /// of a turn, rounding the result with the specified rounding mode. The precision of the output
    /// is the precision of the input. The [`Float`] is taken by reference. An [`Ordering`] is also
    /// returned, indicating whether the rounded arccosecant is less than, equal to, or greater than
    /// the exact arccosecant. Although `NaN`s are not comparable to any [`Float`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`Float::acsc_with_period_round`] and [`Float::acsc_with_period_prec_round`]; this
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
    /// let (c, o) = (&x).acsc_with_period_round_ref(360, Ceiling);
    /// assert_eq!(c.to_string(), "23.578178478201831104022499419849");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn acsc_with_period_round_ref(&self, u: u64, rm: RoundingMode) -> (Self, Ordering) {
        self.acsc_with_period_prec_round_ref(u, self.significant_bits(), rm)
    }

    /// Computes $\operatorname{acsc}(x)u/(2\pi)$, the arccosecant of a [`Float`] measured in $u$ths
    /// of a turn, rounding the result to the precision of the input and to the nearest [`Float`].
    /// The [`Float`] is taken by value.
    ///
    /// If the arccosecant is equidistant from two [`Float`]s with the precision of the input, the
    /// [`Float`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a
    /// description of the `Nearest` rounding mode.
    ///
    /// See [`Float::acsc_with_period_prec_round`] for the error bounds, the special and closed-form
    /// cases, underflow, and the complexity; this function behaves the same way, with `prec` the
    /// precision of the input and `Nearest` rounding.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::acsc_with_period_round`] instead. If you want to specify an output precision,
    /// consider using [`Float::acsc_with_period_prec`]. If you want both of these things, consider
    /// using [`Float::acsc_with_period_prec_round`].
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    ///
    /// let x = Float::from_unsigned_prec(5u32, 100).0 >> 1u32;
    /// assert_eq!(
    ///     x.acsc_with_period(360).to_string(),
    ///     "23.578178478201831104022499419824"
    /// );
    /// ```
    #[inline]
    pub fn acsc_with_period(self, u: u64) -> Self {
        let prec = self.significant_bits();
        self.acsc_with_period_prec(u, prec).0
    }

    /// Computes $\operatorname{acsc}(x)u/(2\pi)$, the arccosecant of a [`Float`] measured in $u$ths
    /// of a turn, rounding the result to the precision of the input and to the nearest [`Float`].
    /// The [`Float`] is taken by reference.
    ///
    /// See [`Float::acsc_with_period`] and [`Float::acsc_with_period_prec_round`]; this function
    /// behaves the same way.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    ///
    /// let x = Float::from_unsigned_prec(5u32, 100).0 >> 1u32;
    /// assert_eq!(
    ///     (&x).acsc_with_period_ref(360).to_string(),
    ///     "23.578178478201831104022499419824"
    /// );
    /// ```
    #[inline]
    pub fn acsc_with_period_ref(&self, u: u64) -> Self {
        self.acsc_with_period_prec_ref(u, self.significant_bits()).0
    }

    /// Computes $\operatorname{acsc}(x)u/(2\pi)$, the arccosecant of a [`Float`] measured in $u$ths
    /// of a turn, in place, rounding the result to the specified precision and with the specified
    /// rounding mode. An [`Ordering`] is returned, indicating whether the rounded arccosecant is
    /// less than, equal to, or greater than the exact arccosecant. Although `NaN`s are not
    /// comparable to any [`Float`], whenever this function assigns a `NaN` it also returns `Equal`.
    ///
    /// See [`Float::acsc_with_period_prec_round`] for the error bounds, the special and closed-form
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
    /// let o = x.acsc_with_period_prec_round_assign(360, 10, Floor);
    /// assert_eq!(x.to_string(), "23.562");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn acsc_with_period_prec_round_assign(
        &mut self,
        u: u64,
        prec: u64,
        rm: RoundingMode,
    ) -> Ordering {
        let (s, o) = self.acsc_with_period_prec_round_ref(u, prec, rm);
        *self = s;
        o
    }

    /// Computes $\operatorname{acsc}(x)u/(2\pi)$, the arccosecant of a [`Float`] measured in $u$ths
    /// of a turn, in place, rounding the result to the nearest value of the specified precision. An
    /// [`Ordering`] is returned, indicating whether the rounded arccosecant is less than, equal to,
    /// or greater than the exact arccosecant. Although `NaN`s are not comparable to any [`Float`],
    /// whenever this function assigns a `NaN` it also returns `Equal`.
    ///
    /// See [`Float::acsc_with_period_prec`] and [`Float::acsc_with_period_prec_round`]; this
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
    /// let o = x.acsc_with_period_prec_assign(360, 10);
    /// assert_eq!(x.to_string(), "23.594");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn acsc_with_period_prec_assign(&mut self, u: u64, prec: u64) -> Ordering {
        self.acsc_with_period_prec_round_assign(u, prec, Nearest)
    }

    /// Computes $\operatorname{acsc}(x)u/(2\pi)$, the arccosecant of a [`Float`] measured in $u$ths
    /// of a turn, in place, rounding the result with the specified rounding mode. The precision of
    /// the output is the precision of the input. An [`Ordering`] is returned, indicating whether
    /// the rounded arccosecant is less than, equal to, or greater than the exact arccosecant.
    /// Although `NaN`s are not comparable to any [`Float`], whenever this function assigns a `NaN`
    /// it also returns `Equal`.
    ///
    /// See [`Float::acsc_with_period_round`] and [`Float::acsc_with_period_prec_round`]; this
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
    /// let o = x.acsc_with_period_round_assign(360, Floor);
    /// assert_eq!(x.to_string(), "23.578178478201831104022499419824");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn acsc_with_period_round_assign(&mut self, u: u64, rm: RoundingMode) -> Ordering {
        self.acsc_with_period_prec_round_assign(u, self.significant_bits(), rm)
    }

    /// Computes $\operatorname{acsc}(x)u/(2\pi)$, the arccosecant of a [`Float`] measured in $u$ths
    /// of a turn, in place, rounding the result to the precision of the input and to the nearest
    /// [`Float`].
    ///
    /// If the arccosecant is equidistant from two [`Float`]s with the precision of the input, the
    /// [`Float`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a
    /// description of the `Nearest` rounding mode.
    ///
    /// See [`Float::acsc_with_period`] and [`Float::acsc_with_period_prec_round`]; this function
    /// behaves the same way.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    ///
    /// let mut x = Float::from_unsigned_prec(5u32, 100).0 >> 1u32;
    /// x.acsc_with_period_assign(360);
    /// assert_eq!(x.to_string(), "23.578178478201831104022499419824");
    /// ```
    #[inline]
    pub fn acsc_with_period_assign(&mut self, u: u64) {
        let prec = self.significant_bits();
        self.acsc_with_period_prec_assign(u, prec);
    }

    /// Computes $\operatorname{acsc}(x)u/(2\pi)$, the arccosecant of a [`Rational`] measured in
    /// $u$ths of a turn, rounding the result to the specified precision and with the specified
    /// rounding mode and returning the result as a [`Float`]. The [`Rational`] is taken by value.
    /// An [`Ordering`] is also returned, indicating whether the rounded arccosecant is less than,
    /// equal to, or greater than the exact arccosecant.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,u,p,m) = \operatorname{acsc}(x)u/(2\pi)+\varepsilon.
    /// $$
    /// - If $|x|<1$, if $u = 0$, if $|x|$ is 1, or if $|x|$ is 2 and $u$ is a multiple of 3,
    ///   $\varepsilon$ may be ignored or assumed to be 0.
    /// - Otherwise, if $m$ is not `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2
    ///   |\operatorname{acsc}(x)u/(2\pi)|\rfloor-p+1}$.
    /// - Otherwise, if $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\operatorname{acsc}(x)u/(2\pi)|\rfloor-p}$.
    ///
    /// The output has precision `prec`.
    ///
    /// Special cases:
    /// - $f(x,u,p,m)=\text{NaN}$ for $|x|<1$, including zero and when $u=0$
    /// - $f(x,0,p,m)=\pm0.0$ for $|x|\geq1$, with the sign of $x$
    /// - $f(\pm1,u,p,m)=\pm u/4$, a quarter turn
    /// - $f(\pm2,u,p,m)=\pm u/12$, a twelfth of a turn, when $u$ is a multiple of 3
    ///
    /// Those are the only exact cases, and the turn fractions are exact only when $p$ is large
    /// enough to hold them.
    ///
    /// The arccosecant is odd, so $f(-x,u,p,m)=-f(x,u,p,-m)$, with $-m$ the reflection of $m$ that
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
    /// [`Float::acsc_with_period_rational_prec`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `x.significant_bits()`: $x^2-1$ is formed exactly, and the arctangent of the reciprocal of
    /// its square root is taken at a working precision of about $n$ bits and scaled by $u/(2\pi)$,
    /// which needs $\pi$ to that many bits; those cost the first term, and the second is the
    /// square. A large $x$ skips the square, its arccosecant being the reciprocal of $|x|$ to
    /// within the working precision.
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
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// // an input of 2 is a twelfth of a turn, and one of -1 minus a quarter
    /// let (c, o) = Float::acsc_with_period_rational_prec_round(Rational::TWO, 360, 10, Exact);
    /// assert_eq!(c.to_string(), "30.000");
    /// assert_eq!(o, Equal);
    ///
    /// let (c, o) =
    ///     Float::acsc_with_period_rational_prec_round(Rational::NEGATIVE_ONE, 360, 10, Exact);
    /// assert_eq!(c.to_string(), "-90.000");
    /// assert_eq!(o, Equal);
    ///
    /// let (c, o) = Float::acsc_with_period_rational_prec_round(
    ///     Rational::from_unsigneds(5u8, 3),
    ///     360,
    ///     10,
    ///     Floor,
    /// );
    /// assert_eq!(c.to_string(), "36.812");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    #[allow(clippy::needless_pass_by_value)]
    pub fn acsc_with_period_rational_prec_round(
        x: Rational,
        u: u64,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        Self::acsc_with_period_rational_prec_round_ref(&x, u, prec, rm)
    }

    /// Computes $\operatorname{acsc}(x)u/(2\pi)$, the arccosecant of a [`Rational`] measured in
    /// $u$ths of a turn, rounding the result to the specified precision and with the specified
    /// rounding mode and returning the result as a [`Float`]. The [`Rational`] is taken by
    /// reference. An [`Ordering`] is also returned, indicating whether the rounded arccosecant is
    /// less than, equal to, or greater than the exact arccosecant.
    ///
    /// See [`Float::acsc_with_period_rational_prec_round`] for the error bounds, the special and
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
    /// let (c, o) = Float::acsc_with_period_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(5u8, 3),
    ///     360,
    ///     10,
    ///     Ceiling,
    /// );
    /// assert_eq!(c.to_string(), "36.875");
    /// assert_eq!(o, Greater);
    /// ```
    pub fn acsc_with_period_rational_prec_round_ref(
        x: &Rational,
        u: u64,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        assert_ne!(prec, 0);
        if x.lt_abs(&1u32) {
            // the arccosecant is NaN inside (-1, 1), zero included; this holds for a zero period
            // too, since NaN times 0 is NaN
            return (Self::NAN, Equal);
        }
        if u == 0 {
            // acscu(x, 0) = 0 with the sign of x, which keeps the function odd
            return (
                if *x > 0u32 {
                    Self::ZERO
                } else {
                    Self::NEGATIVE_ZERO
                },
                Equal,
            );
        }
        acsc_with_period_rational_helper(x, u, prec, rm)
    }

    /// Computes $\operatorname{acsc}(x)u/(2\pi)$, the arccosecant of a [`Rational`] measured in
    /// $u$ths of a turn, rounding the result to the nearest value of the specified precision and
    /// returning the result as a [`Float`]. The [`Rational`] is taken by value. An [`Ordering`] is
    /// also returned, indicating whether the rounded arccosecant is less than, equal to, or greater
    /// than the exact arccosecant.
    ///
    /// If the arccosecant is equidistant from two [`Float`]s with the specified precision, the
    /// [`Float`] with fewer 1s in its binary expansion is chosen.
    ///
    /// See [`Float::acsc_with_period_rational_prec_round`] for the error bounds, the special and
    /// closed-form cases, underflow, and the complexity; this function behaves the same way.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::acsc_with_period_rational_prec_round`] instead.
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
    ///     Float::acsc_with_period_rational_prec(Rational::from_unsigneds(5u8, 3), 360, 10);
    /// assert_eq!(c.to_string(), "36.875");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) =
    ///     Float::acsc_with_period_rational_prec(Rational::from_unsigneds(5u8, 3), 360, 53);
    /// assert_eq!(c.to_string(), "36.869897645844020");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn acsc_with_period_rational_prec(x: Rational, u: u64, prec: u64) -> (Self, Ordering) {
        Self::acsc_with_period_rational_prec_round(x, u, prec, Nearest)
    }

    /// Computes $\operatorname{acsc}(x)u/(2\pi)$, the arccosecant of a [`Rational`] measured in
    /// $u$ths of a turn, rounding the result to the nearest value of the specified precision and
    /// returning the result as a [`Float`]. The [`Rational`] is taken by reference. An [`Ordering`]
    /// is also returned, indicating whether the rounded arccosecant is less than, equal to, or
    /// greater than the exact arccosecant.
    ///
    /// See [`Float::acsc_with_period_rational_prec`] and
    /// [`Float::acsc_with_period_rational_prec_round`]; this function behaves the same way.
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
    ///     Float::acsc_with_period_rational_prec_ref(&Rational::from_unsigneds(5u8, 3), 360, 53);
    /// assert_eq!(c.to_string(), "36.869897645844020");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn acsc_with_period_rational_prec_ref(x: &Rational, u: u64, prec: u64) -> (Self, Ordering) {
        Self::acsc_with_period_rational_prec_round_ref(x, u, prec, Nearest)
    }
}

impl Acsc for Float {
    type Output = Self;

    /// Computes $\operatorname{acsc} x$, the arccosecant of a [`Float`], taking it by value.
    ///
    /// If the output has a precision, it is the precision of the input. If the arccosecant is
    /// equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s in
    /// its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// $$
    /// f(x) = \operatorname{acsc}(x)+\varepsilon.
    /// $$
    /// - If $x$ is NaN, infinite, or $|x|<1$, $\varepsilon$ may be ignored or assumed to be 0.
    /// - Otherwise, $|\varepsilon| \leq 2^{\lfloor\log_2 |\operatorname{acsc}(x)|\rfloor-p}$, where
    ///   $p$ is the precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN})=\text{NaN}$
    /// - $f(x)=\text{NaN}$ for $|x|<1$, including $\pm0.0$
    /// - $f(\infty)=0.0$ and $f(-\infty)=-0.0$
    /// - $f(1)=\pi/2$ and $f(-1)=-\pi/2$
    ///
    /// Overflow and underflow are both impossible; see [`Float::acsc_prec_round`].
    ///
    /// If you want to specify an output precision, consider using [`Float::acsc_prec`] instead. If
    /// you want to specify a rounding mode as well, consider using [`Float::acsc_prec_round`].
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
    /// use malachite_base::num::arithmetic::traits::Acsc;
    /// use malachite_base::num::basic::traits::Two;
    /// use malachite_float::Float;
    ///
    /// assert_eq!(Float::TWO.acsc().to_string(), "0.50");
    /// ```
    #[inline]
    fn acsc(self) -> Self {
        let prec = self.significant_bits();
        self.acsc_prec(prec).0
    }
}

impl Acsc for &Float {
    type Output = Float;

    /// Computes $\operatorname{acsc} x$, the arccosecant of a [`Float`], taking it by reference.
    ///
    /// See [`Acsc::acsc`] and [`Float::acsc_prec_round`]; this function behaves the same way.
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
    /// use malachite_base::num::arithmetic::traits::Acsc;
    /// use malachite_base::num::basic::traits::Two;
    /// use malachite_float::Float;
    ///
    /// assert_eq!((&Float::TWO).acsc().to_string(), "0.50");
    /// ```
    #[inline]
    fn acsc(self) -> Float {
        self.acsc_prec_ref(self.significant_bits()).0
    }
}

impl AcscAssign for Float {
    /// Replaces a [`Float`] with its arccosecant, $\operatorname{acsc}(x)$.
    ///
    /// See [`Acsc::acsc`] and [`Float::acsc_prec_round`]; this function behaves the same way.
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
    /// use malachite_base::num::arithmetic::traits::AcscAssign;
    /// use malachite_base::num::basic::traits::Two;
    /// use malachite_float::Float;
    ///
    /// let mut x = Float::TWO;
    /// x.acsc_assign();
    /// assert_eq!(x.to_string(), "0.50");
    /// ```
    #[inline]
    fn acsc_assign(&mut self) {
        let prec = self.significant_bits();
        self.acsc_prec_assign(prec);
    }
}

/// Computes $\operatorname{acsc} x$, the arccosecant of a primitive float, returning the result as
/// a primitive float.
///
/// This is the correctly rounded arccosecant: the exact $\operatorname{acsc}(x)$ is rounded once,
/// to the nearest value of the input's type.
///
/// Special cases:
/// - $f(\text{NaN})=\text{NaN}$
/// - $f(x)=\text{NaN}$ for $|x|<1$, including $\pm0.0$
/// - $f(\infty)=0.0$ and $f(-\infty)=-0.0$
///
/// Overflow is not possible, since $|\operatorname{acsc}(x)| \leq \pi/2$, and neither is underflow:
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
/// use malachite_float::float::arithmetic::acsc::primitive_float_acsc;
///
/// assert!(primitive_float_acsc(f32::NAN).is_nan());
/// // the arccosecant is NaN inside (-1, 1)
/// assert!(primitive_float_acsc(0.5f32).is_nan());
/// assert_eq!(
///     NiceFloat(primitive_float_acsc(f32::INFINITY)),
///     NiceFloat(0.0)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_acsc(2.0f32)),
///     NiceFloat(0.5235988)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_acsc(-2.0f32)),
///     NiceFloat(-0.5235988)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_acsc(2.0f64)),
///     NiceFloat(0.5235987755982989)
/// );
/// ```
#[inline]
#[allow(clippy::type_repetition_in_bounds)]
pub fn primitive_float_acsc<T: PrimitiveFloat>(x: T) -> T
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    emulate_float_to_float_fn(Float::acsc_prec, x)
}

/// Computes $\operatorname{acsc} x$, the arccosecant of a [`Rational`], returning the result as a
/// primitive float.
///
/// This is the correctly rounded arccosecant: the exact $\operatorname{acsc}(x)$ is rounded once,
/// to the nearest value of the output type.
///
/// Special cases:
/// - $f(x)=\text{NaN}$ for $|x|<1$, including zero
///
/// Overflow is not possible, since $|\operatorname{acsc}(x)| \leq \pi/2$. The result is subnormal,
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
/// use malachite_base::num::basic::traits::{NegativeOne, One, OneHalf, Two};
/// use malachite_base::num::float::NiceFloat;
/// use malachite_float::float::arithmetic::acsc::primitive_float_acsc_rational;
/// use malachite_q::Rational;
///
/// // the arccosecant is NaN inside (-1, 1)
/// assert!(primitive_float_acsc_rational::<f64>(&Rational::ONE_HALF).is_nan());
/// assert_eq!(
///     NiceFloat(primitive_float_acsc_rational::<f64>(&Rational::ONE)),
///     NiceFloat(1.5707963267948966)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_acsc_rational::<f64>(
///         &Rational::NEGATIVE_ONE
///     )),
///     NiceFloat(-1.5707963267948966)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_acsc_rational::<f64>(&Rational::TWO)),
///     NiceFloat(0.5235987755982989)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_acsc_rational::<f32>(
///         &Rational::from_unsigneds(5u8, 3)
///     )),
///     NiceFloat(0.6435011)
/// );
/// ```
#[inline]
#[allow(clippy::type_repetition_in_bounds)]
pub fn primitive_float_acsc_rational<T: PrimitiveFloat>(x: &Rational) -> T
where
    Float: PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float>,
{
    emulate_rational_to_float_fn(Float::acsc_rational_prec_ref, x)
}

/// Computes $\operatorname{acsc}(x)u/(2\pi)$, the arccosecant of a primitive float measured in
/// $u$ths of a turn (so that `u = 360` gives degrees), returning the result as a primitive float.
///
/// This is `primitive_float_acsc` scaled by $u/(2\pi)$: see [`Float::acsc_with_period_prec_round`]
/// for the error bounds and the special cases. NaN and every $|x|<1$, including the zeros, give
/// NaN, even when $u=0$; $\pm\infty$ give $\pm0.0$; a zero period gives a zero with the sign of
/// $x$; $\pm1$ give $\pm u/4$; and $\pm2$ give $\pm u/12$ when $u$ is a multiple of 3.
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
/// use malachite_float::float::arithmetic::acsc::primitive_float_acsc_with_period;
///
/// assert!(primitive_float_acsc_with_period(f32::NAN, 360).is_nan());
/// // the arccosecant is NaN inside (-1, 1)
/// assert!(primitive_float_acsc_with_period(0.5f32, 360).is_nan());
/// assert_eq!(
///     NiceFloat(primitive_float_acsc_with_period(f32::INFINITY, 360)),
///     NiceFloat(0.0)
/// );
/// // an input of 2 is a twelfth of a turn, and one of -1 minus a quarter
/// assert_eq!(
///     NiceFloat(primitive_float_acsc_with_period(2.0f32, 360)),
///     NiceFloat(30.0)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_acsc_with_period(-1.0f32, 360)),
///     NiceFloat(-90.0)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_acsc_with_period(2.5f64, 360)),
///     NiceFloat(23.57817847820183)
/// );
/// ```
#[inline]
#[allow(clippy::type_repetition_in_bounds)]
pub fn primitive_float_acsc_with_period<T: PrimitiveFloat>(x: T, u: u64) -> T
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    emulate_float_to_float_fn(|x, prec| Float::acsc_with_period_prec(x, u, prec), x)
}

/// Computes $\operatorname{acsc}(x)u/(2\pi)$, the arccosecant of a [`Rational`] measured in $u$ths
/// of a turn (so that `u = 360` gives degrees), returning the result as a primitive float.
///
/// This is `primitive_float_acsc_rational` scaled by $u/(2\pi)$: see
/// [`Float::acsc_with_period_rational_prec_round`] for the error bounds and the special cases.
/// Every $|x|<1$ gives NaN, even when $u=0$; a zero period gives a zero with the sign of $x$;
/// $\pm1$ give $\pm u/4$; and $\pm2$ give $\pm u/12$ when $u$ is a multiple of 3.
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
/// use malachite_base::num::basic::traits::{NegativeOne, OneHalf, Two};
/// use malachite_base::num::float::NiceFloat;
/// use malachite_float::float::arithmetic::acsc::primitive_float_acsc_with_period_rational;
/// use malachite_q::Rational;
///
/// // the arccosecant is NaN inside (-1, 1)
/// assert!(primitive_float_acsc_with_period_rational::<f64>(&Rational::ONE_HALF, 360).is_nan());
/// // an input of 2 is a twelfth of a turn, and one of -1 minus a quarter
/// assert_eq!(
///     NiceFloat(primitive_float_acsc_with_period_rational::<f64>(
///         &Rational::TWO,
///         360
///     )),
///     NiceFloat(30.0)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_acsc_with_period_rational::<f64>(
///         &Rational::NEGATIVE_ONE,
///         360
///     )),
///     NiceFloat(-90.0)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_acsc_with_period_rational::<f64>(
///         &Rational::from_unsigneds(5u8, 3),
///         360
///     )),
///     NiceFloat(36.86989764584402)
/// );
/// ```
#[inline]
#[allow(clippy::type_repetition_in_bounds)]
pub fn primitive_float_acsc_with_period_rational<T: PrimitiveFloat>(x: &Rational, u: u64) -> T
where
    Float: PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float>,
{
    emulate_rational_to_float_fn(
        |x, prec| Float::acsc_with_period_rational_prec_ref(x, u, prec),
        x,
    )
}
