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
use crate::float::arithmetic::acsc::{reciprocal_is_tie, signed_half_pi};
use crate::float::arithmetic::atan::atan_rational_helper;
use crate::{emulate_float_to_float_fn, emulate_rational_to_float_fn};
use core::cmp::Ordering::{self, Equal, Greater, Less};
use malachite_base::num::arithmetic::traits::{Abs, Acot, AcotAssign, CeilingLogBase2, Reciprocal};
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::{NaN as NaNTrait, NegativeZero, Zero as ZeroTrait};
use malachite_base::num::comparison::traits::PartialOrdAbs;
use malachite_base::num::conversion::traits::{ExactFrom, RoundingFrom};
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{self, Down, Exact, Floor, Nearest};
use malachite_nz::natural::arithmetic::float::round::float_can_round;
use malachite_nz::platform::Limb;
use malachite_q::Rational;

// Turns the correctly rounded 1/|x| into the correctly rounded acot(|x|), for an |x| so large that
// 1/|x| is below 2^(1 - 2^29). There acot(x), which falls short of 1/|x| by a factor below 1 -
// 2^(-2^30), is nearer to it than any representable precision can resolve: the reciprocal's own
// rounding is the answer. The two exceptions are a reciprocal that lands exactly on a representable
// value and one that lands exactly on a tie, both of which have to move down, acot(x) being
// strictly below 1/|x|.
fn acot_from_huge_reciprocal(
    t: Float,
    o: Ordering,
    tie: bool,
    rm: RoundingMode,
) -> (Float, Ordering) {
    if o == Equal {
        return if rm == Floor || rm == Down {
            let mut t = t;
            t.decrement();
            (t, Less)
        } else {
            (t, Greater)
        };
    }
    if tie {
        // `Nearest` broke the tie its own way; acot(x) is below it, so the lower neighbour wins
        let mut t = t;
        if o == Less {
            return (t, Less);
        }
        t.decrement();
        return (t, Less);
    }
    (t, o)
}

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
    // An |x| so large that acot(x) = (1/x)(1 - 1/(3x^2) + ...) sits within 2^(-2^30) of 1/|x| --
    // closer than any representable precision can resolve. The reciprocal alone decides the answer,
    // and a Ziv loop would be ruinous: for a power of two, it would balloon toward 2 EXP(x) bits,
    // billions of them, trying to see the difference.
    if exp_x << 1 > MAX_EXPONENT_I64 {
        let tie = rm == Nearest && {
            let (wide, o_wide) = xp.reciprocal_prec_ref(prec + 1);
            reciprocal_is_tie(&wide, o_wide, prec)
        };
        let (t, o) = xp.reciprocal_prec_round(prec, rm);
        return acot_from_huge_reciprocal(t, o, tie, rm);
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
