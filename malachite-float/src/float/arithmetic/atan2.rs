// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the GNU MPFR Library.
//
//      Copyright 2005-2025 Free Software Foundation, Inc.
//
//      Contributed by the Pascaline and Caramba projects, INRIA.
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::{Float, emulate_float_float_to_float_fn};
use core::cmp::Ordering::{self, Equal, Greater, Less};
use core::cmp::{max, min};
use malachite_base::num::arithmetic::traits::{
    Abs, Atan2, Atan2Assign, CeilingLogBase2, IsPowerOf2,
};
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::{
    NaN as NaNTrait, NegativeZero as NegativeZeroTrait, Zero as ZeroTrait,
};
use malachite_base::num::comparison::traits::PartialOrdAbs;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{
    self, Ceiling, Down, Exact, Floor, Nearest, Up,
};
use malachite_nz::natural::arithmetic::float::round::float_can_round;
use malachite_nz::platform::Limb;

// pi/2^i, negated when `neg`. This is pi_div_2ui from atan2.c, MPFR 4.2.2; the shift is exact, so
// it does not disturb the ternary value.
fn pi_div_2ui(i: u32, neg: bool, prec: u64, rm: RoundingMode) -> (Float, Ordering) {
    assert_ne!(rm, Exact, "Inexact atan2");
    let (pi, o) = Float::pi_prec_round(prec, if neg { -rm } else { rm });
    let q = pi >> i;
    if neg { (-q, o.reverse()) } else { (q, o) }
}

// +-3 pi/4, for an infinite y over a negative infinite x. MPFR gives this its own Ziv loop, since
// unlike the other quadrant boundaries it is not a power of 2 times pi.
fn three_pi_over_4(neg: bool, prec: u64, rm: RoundingMode) -> (Float, Ordering) {
    assert_ne!(rm, Exact, "Inexact atan2");
    let mut w = prec + 10;
    let mut increment = Limb::WIDTH;
    loop {
        // error <= 2 ulps
        let mut t = Float::pi_prec(w)
            .0
            .mul_prec(const { Float::const_from_unsigned(3) }, w)
            .0;
        t >>= 2u32;
        if float_can_round(t.significand_ref().unwrap(), w - 2, prec, rm) {
            let t = if neg { -t } else { t };
            return Float::from_float_prec_round(t, prec, rm);
        }
        w += increment;
        increment = w >> 1;
    }
}

// The result of a computation that underflowed: a signed zero or the smallest positive `Float`, by
// the rounding mode alone. This is mpfr_underflow from mpfr-impl.h, MPFR 4.2.2, where `Nearest`
// rounds away from zero; the caller substitutes `Down` for the cases where it must not.
fn underflow(positive: bool, prec: u64, rm: RoundingMode) -> (Float, Ordering) {
    let away = match rm {
        Ceiling => positive,
        Floor => !positive,
        Up | Nearest => true,
        _ => false,
    };
    let min_positive = Float::min_positive_value_prec(prec);
    match (positive, away) {
        (true, true) => (min_positive, Greater),
        (true, false) => (Float::ZERO, Less),
        (false, true) => (-min_positive, Less),
        (false, false) => (Float::NEGATIVE_ZERO, Greater),
    }
}

// Whether |y/x| is below 2^(MIN_EXPONENT - 1), the smallest positive `Float`, so that the quotient
// underflows. MPFR reads this off the division's underflow flag; its exponent range is wide enough
// that the case never arises for representable inputs, while here it does.
//
// |y/x| = (my/mx) 2^d, where d is the difference of the exponents and my and mx, the significands,
// both lie in [1/2, 1). Only the middle binade needs the two significands compared, which the
// shifts below do exactly.
fn quotient_underflows(y: &Float, x: &Float, exp_y: i64, exp_x: i64) -> bool {
    match (exp_y - exp_x).cmp(&(Float::MIN_EXPONENT_I64 - 1)) {
        Less => true,
        Greater => false,
        Equal => (y >> exp_y).lt_abs(&(x >> exp_x)),
    }
}

// atan2(y, x) when |y/x| is beyond the top of the exponent range, so that the quotient is not a
// `Float`. MPFR widens its range for the whole computation and never meets this case; here the
// arctangent has to be taken from its limit instead.
//
// For z > 0, pi/2 - 1/z < atan z < pi/2. With |y/x| > 2^k the result is therefore atan|y/x| = pi/2
// - delta for x > 0, and pi - atan|y/x| = pi/2 + delta for x < 0, where 0 < delta < 2^-k: either
// way it is pi/2 perturbed by less than 2^-k, carrying the sign of y. Since k is at least
// MAX_EXPONENT - 1, that perturbation is far below the rounding error of pi itself at any usable
// precision, and the loop below is the ordinary one for pi/2.
fn atan2_huge_quotient(k: u64, negative: bool, prec: u64, rm: RoundingMode) -> (Float, Ordering) {
    let mut w = prec + 10;
    let mut increment = Limb::WIDTH;
    loop {
        // |v - pi/2| <= 2^-w, and EXP(v) = 1, so v is good to min(w, k) - 1 bits once delta is
        // counted too
        let v = Float::pi_prec(w).0 >> 1u32;
        if float_can_round(v.significand_ref().unwrap(), min(w, k) - 1, prec, rm) {
            return Float::from_float_prec_round(if negative { -v } else { v }, prec, rm);
        }
        w += increment;
        increment = w >> 1;
    }
}

// Computes atan2(y, x) for finite nonzero y and x, rounded to precision `prec` with rounding mode
// `rm`.
//
// This is mpfr_atan2 from atan2.c, MPFR 4.2.2, past the special cases.
fn atan2_prec_round_normal_ref(
    y: &Float,
    x: &Float,
    prec: u64,
    rm: RoundingMode,
) -> (Float, Ordering) {
    assert_ne!(rm, Exact, "Inexact atan2");
    let exp_y = i64::from(y.get_exponent().unwrap());
    let exp_x = i64::from(x.get_exponent().unwrap());
    let x_positive = *x > 0u32;
    // When x is a power of two, y/x is exact, so atan takes it directly. The shift is exact only if
    // it stays inside the exponent range, which MPFR checks through the division's flags.
    if x_positive && x.significand_ref().unwrap().is_power_of_2() {
        let shifted = exp_y - exp_x + 1;
        if (Float::MIN_EXPONENT_I64..=Float::MAX_EXPONENT_I64).contains(&shifted) {
            return (y >> (exp_x - 1)).atan_prec_round(prec, rm);
        }
    }
    let y_negative = *y < 0u32;
    // |y/x| lies in (2^(d - 1), 2^(d + 1)), so a d this large puts it beyond the top of the range
    if exp_y - exp_x >= Float::MAX_EXPONENT_I64 {
        return atan2_huge_quotient(u64::exact_from(exp_y - exp_x - 1), y_negative, prec, rm);
    }
    let mut w = prec + 3 + prec.ceiling_log_base_2();
    let mut increment = Limb::WIDTH;
    if x_positive {
        // atan2(y, x) = atan(y/x)
        loop {
            let (t, div_o) = y.div_prec_ref_ref(x, w);
            if div_o == Equal {
                // the quotient is exact, so its arctangent is the whole answer
                return t.atan_prec_round(prec, rm);
            }
            // error <= 1 ulp, except on underflow or overflow
            if quotient_underflows(y, x, exp_y, exp_x) {
                // |atan z| < |z|, so an underflowing quotient gives an underflowing result MPFR
                // takes the sign from the quotient; in this branch x is positive, so it is the sign
                // of y. With `Nearest` a quotient that rounded to zero is below a quarter of the
                // smallest positive `Float`, and rounds toward zero rather than away.
                let rm = if rm == Nearest && t == 0u32 { Down } else { rm };
                return underflow(!y_negative, prec, rm);
            }
            // error <= 2 ulps, since |atan'| <= 1
            let mut t = t;
            t.atan_prec_assign(w);
            if float_can_round(t.significand_ref().unwrap(), w - 2, prec, rm) {
                return Float::from_float_prec_round(t, prec, rm);
            }
            w += increment;
            increment = w >> 1;
        }
    } else {
        // atan2(y, x) = sign(y) (pi - atan|y/x|)
        loop {
            // error <= 1 ulp
            let mut t = y.div_prec_ref_ref(x, w).0.abs();
            // error <= 2 ulps, since |atan'| <= 1
            t.atan_prec_assign(w);
            // error <= 1/2 ulp
            let pi = Float::pi_prec(w).0;
            // if the quotient was zero, so is its arctangent, and |y/x| was below 2^(MIN_EXPONENT -
            // 1)
            let e = if t == 0u32 {
                Float::MIN_EXPONENT_I64 - 1
            } else {
                i64::from(t.get_exponent().unwrap())
            };
            let exp_pi = i64::from(pi.get_exponent().unwrap());
            let t = pi.sub_prec(t, w).0;
            let t = if y_negative { -t } else { t };
            let exp_t = i64::from(t.get_exponent().unwrap());
            // error(t) is at most (1/2 + 2^(EXP(pi) - EXP(t) - 1) + 2^(e - EXP(t) + 1)) ulps, and
            // so at most 2^(max(max(EXP(pi) - EXP(t) - 1, e - EXP(t) + 1), -1) + 2) ulps
            let e = max(max(exp_pi - exp_t - 1, e - exp_t + 1), -1) + 2;
            if e < i64::exact_from(w)
                && float_can_round(
                    t.significand_ref().unwrap(),
                    w - u64::exact_from(e),
                    prec,
                    rm,
                )
            {
                return Float::from_float_prec_round(t, prec, rm);
            }
            w += increment;
            increment = w >> 1;
        }
    }
}

// A signed zero, exactly.
const fn signed_zero(negative: bool) -> (Float, Ordering) {
    (
        if negative {
            Float::NEGATIVE_ZERO
        } else {
            Float::ZERO
        },
        Equal,
    )
}

impl Float {
    /// Computes $\operatorname{atan2}(y,x)$, the angle of the point $(x,y)$ measured from the
    /// positive $x$-axis, rounding the result to the specified precision and with the specified
    /// rounding mode. The [`Float`]s are both taken by reference. An [`Ordering`] is also returned,
    /// indicating whether the rounded angle is less than, equal to, or greater than the exact
    /// angle. Although `NaN`s are not comparable to any [`Float`], whenever this function returns a
    /// `NaN` it also returns `Equal`.
    ///
    /// See [`Float::atan2_prec_round`] for the error bounds, the special cases, underflow, and the
    /// complexity; this function behaves the same way.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with the given precision (which is the case unless the result is a zero).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{NegativeOne, One, Zero};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (t, o) = (&Float::ONE).atan2_prec_round_ref_ref(&Float::ONE, 10, Floor);
    /// assert_eq!(t.to_string(), "0.78516");
    /// assert_eq!(o, Less);
    ///
    /// // a negative x with a zero y is half a turn
    /// let (t, o) = (&Float::ZERO).atan2_prec_round_ref_ref(&Float::NEGATIVE_ONE, 10, Floor);
    /// assert_eq!(t.to_string(), "3.1406");
    /// assert_eq!(o, Less);
    /// ```
    pub fn atan2_prec_round_ref_ref(
        &self,
        other: &Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        assert_ne!(prec, 0);
        let (y, x) = (self, other);
        // atan2 is NaN if either argument is
        if y.is_nan() || x.is_nan() {
            return (Self::NAN, Equal);
        }
        // the quadrant is chosen by the sign bits, so a signed zero behaves like a signed number
        let y_negative = y.is_sign_negative();
        let x_negative = x.is_sign_negative();
        // atan2(+-0, x) = +-pi for x < 0 (or -0.0), and +-0 for x > 0 (or +0.0)
        if *y == 0u32 {
            return if x_negative {
                pi_div_2ui(0, y_negative, prec, rm)
            } else {
                signed_zero(y_negative)
            };
        }
        // atan2(y, +-0) = +-pi/2, with the sign of y
        if *x == 0u32 {
            return pi_div_2ui(1, y_negative, prec, rm);
        }
        if !y.is_finite() {
            // atan2(+-infinity, x) = +-pi/2 for finite x, +-pi/4 for +infinity, +-3pi/4 for
            // -infinity
            return if x.is_finite() {
                pi_div_2ui(1, y_negative, prec, rm)
            } else if x_negative {
                three_pi_over_4(y_negative, prec, rm)
            } else {
                pi_div_2ui(2, y_negative, prec, rm)
            };
        }
        // atan2(+-y, -infinity) = +-pi, atan2(+-y, +infinity) = +-0, for finite nonzero y
        if !x.is_finite() {
            return if x_negative {
                pi_div_2ui(0, y_negative, prec, rm)
            } else {
                signed_zero(y_negative)
            };
        }
        atan2_prec_round_normal_ref(y, x, prec, rm)
    }

    /// Computes $\operatorname{atan2}(y,x)$, the angle of the point $(x,y)$ measured from the
    /// positive $x$-axis, rounding the result to the specified precision and with the specified
    /// rounding mode. The [`Float`]s are both taken by value. An [`Ordering`] is also returned,
    /// indicating whether the rounded angle is less than, equal to, or greater than the exact
    /// angle. Although `NaN`s are not comparable to any [`Float`], whenever this function returns a
    /// `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(y,x,p,m) = \operatorname{atan2}(y,x)+\varepsilon.
    /// $$
    /// - If $y$ or $x$ is NaN, or the result is a zero, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - Otherwise, if $m$ is not `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2
    ///   |\operatorname{atan2}(y,x)|\rfloor-p+1}$.
    /// - Otherwise, if $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\operatorname{atan2}(y,x)|\rfloor-p}$.
    ///
    /// Special cases, in which the sign of a zero argument selects the quadrant:
    /// - $f(\text{NaN},x,p,m)=f(y,\text{NaN},p,m)=\text{NaN}$
    /// - $f(\pm0.0,x,p,m)=\pm0.0$ if $x$ is positive or $+0.0$
    /// - $f(\pm0.0,x,p,m)=\pm\pi$ if $x$ is negative or $-0.0$
    /// - $f(y,\pm0.0,p,m)=\pm\pi/2$, with the sign of $y$, for nonzero $y$
    /// - $f(\pm\infty,x,p,m)=\pm\pi/2$ for finite $x$
    /// - $f(\pm\infty,+\infty,p,m)=\pm\pi/4$
    /// - $f(\pm\infty,-\infty,p,m)=\pm3\pi/4$
    /// - $f(y,+\infty,p,m)=\pm0.0$, with the sign of $y$, for finite nonzero $y$
    /// - $f(y,-\infty,p,m)=\pm\pi$, with the sign of $y$, for finite nonzero $y$
    ///
    /// The zeros are the only exact cases; every other result is a nonzero multiple of $\pi$ or an
    /// arctangent, and so is irrational.
    ///
    /// Overflow is not possible, since $|\operatorname{atan2}(y,x)| \leq \pi$. The result
    /// underflows only for a positive $x$ with $|y/x|$ below $2^{-2^{30}}$, where it is about
    /// $y/x$; there $0.0$ or $\pm2^{-2^{30}}$ is returned instead, by the rounding mode alone.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::atan2_prec`] instead. If you
    /// know that your target precision is the precision of the inputs, consider using
    /// [`Float::atan2_round`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `max(self.significant_bits(), other.significant_bits())`: the quotient is formed at a
    /// working precision of about $n$ bits and its arctangent taken there, which costs the first
    /// term; the second covers the inputs. The magnitudes of the inputs do not drive the cost.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with the given precision (which is the case unless the result is a zero).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{NegativeOne, One, Zero};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (t, o) = Float::ONE.atan2_prec_round(Float::ONE, 10, Floor);
    /// assert_eq!(t.to_string(), "0.78516");
    /// assert_eq!(o, Less);
    ///
    /// // a negative x with a zero y is half a turn
    /// let (t, o) = Float::ZERO.atan2_prec_round(Float::NEGATIVE_ONE, 10, Floor);
    /// assert_eq!(t.to_string(), "3.1406");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    #[allow(clippy::needless_pass_by_value)]
    pub fn atan2_prec_round(self, other: Self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        self.atan2_prec_round_ref_ref(&other, prec, rm)
    }

    /// Computes $\operatorname{atan2}(y,x)$, the angle of the point $(x,y)$ measured from the
    /// positive $x$-axis, rounding the result to the specified precision and with the specified
    /// rounding mode. The first [`Float`] is taken by value and the second by reference. An
    /// [`Ordering`] is also returned, indicating whether the rounded angle is less than, equal to,
    /// or greater than the exact angle. Although `NaN`s are not comparable to any [`Float`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`Float::atan2_prec_round`] for the error bounds, the special cases, underflow, and the
    /// complexity; this function behaves the same way.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with the given precision (which is the case unless the result is a zero).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{NegativeOne, One, Zero};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (t, o) = Float::ONE.atan2_prec_round_val_ref(&Float::ONE, 10, Floor);
    /// assert_eq!(t.to_string(), "0.78516");
    /// assert_eq!(o, Less);
    ///
    /// // a negative x with a zero y is half a turn
    /// let (t, o) = Float::ZERO.atan2_prec_round_val_ref(&Float::NEGATIVE_ONE, 10, Floor);
    /// assert_eq!(t.to_string(), "3.1406");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    #[allow(clippy::needless_pass_by_value)]
    pub fn atan2_prec_round_val_ref(
        self,
        other: &Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        self.atan2_prec_round_ref_ref(other, prec, rm)
    }

    /// Computes $\operatorname{atan2}(y,x)$, the angle of the point $(x,y)$ measured from the
    /// positive $x$-axis, rounding the result to the specified precision and with the specified
    /// rounding mode. The first [`Float`] is taken by reference and the second by value. An
    /// [`Ordering`] is also returned, indicating whether the rounded angle is less than, equal to,
    /// or greater than the exact angle. Although `NaN`s are not comparable to any [`Float`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`Float::atan2_prec_round`] for the error bounds, the special cases, underflow, and the
    /// complexity; this function behaves the same way.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with the given precision (which is the case unless the result is a zero).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{NegativeOne, One, Zero};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (t, o) = (&Float::ONE).atan2_prec_round_ref_val(Float::ONE, 10, Floor);
    /// assert_eq!(t.to_string(), "0.78516");
    /// assert_eq!(o, Less);
    ///
    /// // a negative x with a zero y is half a turn
    /// let (t, o) = (&Float::ZERO).atan2_prec_round_ref_val(Float::NEGATIVE_ONE, 10, Floor);
    /// assert_eq!(t.to_string(), "3.1406");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    #[allow(clippy::needless_pass_by_value)]
    pub fn atan2_prec_round_ref_val(
        &self,
        other: Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        self.atan2_prec_round_ref_ref(&other, prec, rm)
    }

    /// Computes $\operatorname{atan2}(y,x)$, the angle of the point $(x,y)$ measured from the
    /// positive $x$-axis, rounding the result to the nearest value of the specified precision. The
    /// [`Float`]s are both taken by value. An [`Ordering`] is also returned, indicating whether the
    /// rounded angle is less than, equal to, or greater than the exact angle. Although `NaN`s are
    /// not comparable to any [`Float`], whenever this function returns a `NaN` it also returns
    /// `Equal`.
    ///
    /// If the angle is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// See [`Float::atan2_prec_round`] for the error bounds, the special cases, underflow, and the
    /// complexity; this function is that one with `Nearest`.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::atan2_prec_round`] instead.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (t, o) = Float::ONE.atan2_prec(Float::ONE, 10);
    /// assert_eq!(t.to_string(), "0.78516");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    #[allow(clippy::needless_pass_by_value)]
    pub fn atan2_prec(self, other: Self, prec: u64) -> (Self, Ordering) {
        self.atan2_prec_round_ref_ref(&other, prec, Nearest)
    }

    /// Computes $\operatorname{atan2}(y,x)$, the angle of the point $(x,y)$ measured from the
    /// positive $x$-axis, rounding the result to the nearest value of the specified precision. The
    /// first [`Float`] is taken by value and the second by reference. An [`Ordering`] is also
    /// returned, indicating whether the rounded angle is less than, equal to, or greater than the
    /// exact angle. Although `NaN`s are not comparable to any [`Float`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// If the angle is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// See [`Float::atan2_prec_round`] for the error bounds, the special cases, underflow, and the
    /// complexity; this function is that one with `Nearest`.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::atan2_prec_round`] instead.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (t, o) = Float::ONE.atan2_prec_val_ref(&Float::ONE, 10);
    /// assert_eq!(t.to_string(), "0.78516");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    #[allow(clippy::needless_pass_by_value)]
    pub fn atan2_prec_val_ref(self, other: &Self, prec: u64) -> (Self, Ordering) {
        self.atan2_prec_round_ref_ref(other, prec, Nearest)
    }

    /// Computes $\operatorname{atan2}(y,x)$, the angle of the point $(x,y)$ measured from the
    /// positive $x$-axis, rounding the result to the nearest value of the specified precision. The
    /// first [`Float`] is taken by reference and the second by value. An [`Ordering`] is also
    /// returned, indicating whether the rounded angle is less than, equal to, or greater than the
    /// exact angle. Although `NaN`s are not comparable to any [`Float`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// If the angle is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// See [`Float::atan2_prec_round`] for the error bounds, the special cases, underflow, and the
    /// complexity; this function is that one with `Nearest`.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::atan2_prec_round`] instead.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (t, o) = (&Float::ONE).atan2_prec_ref_val(Float::ONE, 10);
    /// assert_eq!(t.to_string(), "0.78516");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    #[allow(clippy::needless_pass_by_value)]
    pub fn atan2_prec_ref_val(&self, other: Self, prec: u64) -> (Self, Ordering) {
        self.atan2_prec_round_ref_ref(&other, prec, Nearest)
    }

    /// Computes $\operatorname{atan2}(y,x)$, the angle of the point $(x,y)$ measured from the
    /// positive $x$-axis, rounding the result to the nearest value of the specified precision. The
    /// [`Float`]s are both taken by reference. An [`Ordering`] is also returned, indicating whether
    /// the rounded angle is less than, equal to, or greater than the exact angle. Although `NaN`s
    /// are not comparable to any [`Float`], whenever this function returns a `NaN` it also returns
    /// `Equal`.
    ///
    /// If the angle is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// See [`Float::atan2_prec_round`] for the error bounds, the special cases, underflow, and the
    /// complexity; this function is that one with `Nearest`.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::atan2_prec_round`] instead.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (t, o) = (&Float::ONE).atan2_prec_ref_ref(&Float::ONE, 10);
    /// assert_eq!(t.to_string(), "0.78516");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn atan2_prec_ref_ref(&self, other: &Self, prec: u64) -> (Self, Ordering) {
        self.atan2_prec_round_ref_ref(other, prec, Nearest)
    }

    /// Computes $\operatorname{atan2}(y,x)$, the angle of the point $(x,y)$ measured from the
    /// positive $x$-axis, rounding the result with the specified rounding mode. The [`Float`]s are
    /// both taken by value. An [`Ordering`] is also returned, indicating whether the rounded angle
    /// is less than, equal to, or greater than the exact angle. Although `NaN`s are not comparable
    /// to any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the maximum of the precisions of the inputs. See
    /// [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// See [`Float::atan2_prec_round`] for the error bounds, the special cases, underflow, and the
    /// complexity; this function is that one with `prec` the maximum input precision.
    ///
    /// If you want to specify the output precision, consider using [`Float::atan2_prec_round`]
    /// instead.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the result cannot be represented exactly with the precision of
    /// the inputs.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (t, o) = Float::from(0.3f64).atan2_round(Float::from(0.4f64), Floor);
    /// assert_eq!(t.to_string(), "0.64350110879328426");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    #[allow(clippy::needless_pass_by_value)]
    pub fn atan2_round(self, other: Self, rm: RoundingMode) -> (Self, Ordering) {
        let prec = max(self.significant_bits(), other.significant_bits());
        self.atan2_prec_round_ref_ref(&other, prec, rm)
    }

    /// Computes $\operatorname{atan2}(y,x)$, the angle of the point $(x,y)$ measured from the
    /// positive $x$-axis, rounding the result with the specified rounding mode. The first [`Float`]
    /// is taken by value and the second by reference. An [`Ordering`] is also returned, indicating
    /// whether the rounded angle is less than, equal to, or greater than the exact angle. Although
    /// `NaN`s are not comparable to any [`Float`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// The precision of the output is the maximum of the precisions of the inputs. See
    /// [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// See [`Float::atan2_prec_round`] for the error bounds, the special cases, underflow, and the
    /// complexity; this function is that one with `prec` the maximum input precision.
    ///
    /// If you want to specify the output precision, consider using [`Float::atan2_prec_round`]
    /// instead.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the result cannot be represented exactly with the precision of
    /// the inputs.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (t, o) = Float::from(0.3f64).atan2_round_val_ref(&Float::from(0.4f64), Floor);
    /// assert_eq!(t.to_string(), "0.64350110879328426");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    #[allow(clippy::needless_pass_by_value)]
    pub fn atan2_round_val_ref(self, other: &Self, rm: RoundingMode) -> (Self, Ordering) {
        let prec = max(self.significant_bits(), other.significant_bits());
        self.atan2_prec_round_ref_ref(other, prec, rm)
    }

    /// Computes $\operatorname{atan2}(y,x)$, the angle of the point $(x,y)$ measured from the
    /// positive $x$-axis, rounding the result with the specified rounding mode. The first [`Float`]
    /// is taken by reference and the second by value. An [`Ordering`] is also returned, indicating
    /// whether the rounded angle is less than, equal to, or greater than the exact angle. Although
    /// `NaN`s are not comparable to any [`Float`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// The precision of the output is the maximum of the precisions of the inputs. See
    /// [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// See [`Float::atan2_prec_round`] for the error bounds, the special cases, underflow, and the
    /// complexity; this function is that one with `prec` the maximum input precision.
    ///
    /// If you want to specify the output precision, consider using [`Float::atan2_prec_round`]
    /// instead.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the result cannot be represented exactly with the precision of
    /// the inputs.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (t, o) = (&Float::from(0.3f64)).atan2_round_ref_val(Float::from(0.4f64), Floor);
    /// assert_eq!(t.to_string(), "0.64350110879328426");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    #[allow(clippy::needless_pass_by_value)]
    pub fn atan2_round_ref_val(&self, other: Self, rm: RoundingMode) -> (Self, Ordering) {
        let prec = max(self.significant_bits(), other.significant_bits());
        self.atan2_prec_round_ref_ref(&other, prec, rm)
    }

    /// Computes $\operatorname{atan2}(y,x)$, the angle of the point $(x,y)$ measured from the
    /// positive $x$-axis, rounding the result with the specified rounding mode. The [`Float`]s are
    /// both taken by reference. An [`Ordering`] is also returned, indicating whether the rounded
    /// angle is less than, equal to, or greater than the exact angle. Although `NaN`s are not
    /// comparable to any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the maximum of the precisions of the inputs. See
    /// [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// See [`Float::atan2_prec_round`] for the error bounds, the special cases, underflow, and the
    /// complexity; this function is that one with `prec` the maximum input precision.
    ///
    /// If you want to specify the output precision, consider using [`Float::atan2_prec_round`]
    /// instead.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the result cannot be represented exactly with the precision of
    /// the inputs.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (t, o) = (&Float::from(0.3f64)).atan2_round_ref_ref(&Float::from(0.4f64), Floor);
    /// assert_eq!(t.to_string(), "0.64350110879328426");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn atan2_round_ref_ref(&self, other: &Self, rm: RoundingMode) -> (Self, Ordering) {
        let prec = max(self.significant_bits(), other.significant_bits());
        self.atan2_prec_round_ref_ref(other, prec, rm)
    }

    /// Computes $\operatorname{atan2}(y,x)$, the angle of the point $(x,y)$ measured from the
    /// positive $x$-axis, rounding the result to the specified precision and with the specified
    /// rounding mode. The first [`Float`] is replaced by the result, and the second is taken by
    /// value. An [`Ordering`] is returned, indicating whether the rounded angle is less than, equal
    /// to, or greater than the exact angle.
    ///
    /// See [`Float::atan2_prec_round`] for the error bounds, the special cases, underflow, and the
    /// complexity; this function behaves the same way.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with the given precision (which is the case unless the result is a zero).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut y = Float::ONE;
    /// assert_eq!(y.atan2_prec_round_assign(Float::ONE, 10, Floor), Less);
    /// assert_eq!(y.to_string(), "0.78516");
    /// ```
    #[inline]
    #[allow(clippy::needless_pass_by_value)]
    pub fn atan2_prec_round_assign(
        &mut self,
        other: Self,
        prec: u64,
        rm: RoundingMode,
    ) -> Ordering {
        let (t, o) = self.atan2_prec_round_ref_ref(&other, prec, rm);
        *self = t;
        o
    }

    /// Computes $\operatorname{atan2}(y,x)$, the angle of the point $(x,y)$ measured from the
    /// positive $x$-axis, rounding the result to the specified precision and with the specified
    /// rounding mode. The first [`Float`] is replaced by the result, and the second is taken by
    /// reference. An [`Ordering`] is returned, indicating whether the rounded angle is less than,
    /// equal to, or greater than the exact angle.
    ///
    /// See [`Float::atan2_prec_round`] for the error bounds, the special cases, underflow, and the
    /// complexity; this function behaves the same way.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with the given precision (which is the case unless the result is a zero).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut y = Float::ONE;
    /// assert_eq!(y.atan2_prec_round_assign_ref(&Float::ONE, 10, Floor), Less);
    /// assert_eq!(y.to_string(), "0.78516");
    /// ```
    #[inline]
    pub fn atan2_prec_round_assign_ref(
        &mut self,
        other: &Self,
        prec: u64,
        rm: RoundingMode,
    ) -> Ordering {
        let (t, o) = self.atan2_prec_round_ref_ref(other, prec, rm);
        *self = t;
        o
    }

    /// Computes $\operatorname{atan2}(y,x)$, the angle of the point $(x,y)$ measured from the
    /// positive $x$-axis, rounding the result to the nearest value of the specified precision. The
    /// first [`Float`] is replaced by the result, and the second is taken by value. An [`Ordering`]
    /// is returned, indicating whether the rounded angle is less than, equal to, or greater than
    /// the exact angle.
    ///
    /// See [`Float::atan2_prec_round`] for the error bounds, the special cases, underflow, and the
    /// complexity; this function behaves the same way.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut y = Float::ONE;
    /// assert_eq!(y.atan2_prec_assign(Float::ONE, 10), Less);
    /// assert_eq!(y.to_string(), "0.78516");
    /// ```
    #[inline]
    #[allow(clippy::needless_pass_by_value)]
    pub fn atan2_prec_assign(&mut self, other: Self, prec: u64) -> Ordering {
        let (t, o) = self.atan2_prec_ref_ref(&other, prec);
        *self = t;
        o
    }

    /// Computes $\operatorname{atan2}(y,x)$, the angle of the point $(x,y)$ measured from the
    /// positive $x$-axis, rounding the result to the nearest value of the specified precision. The
    /// first [`Float`] is replaced by the result, and the second is taken by reference. An
    /// [`Ordering`] is returned, indicating whether the rounded angle is less than, equal to, or
    /// greater than the exact angle.
    ///
    /// See [`Float::atan2_prec_round`] for the error bounds, the special cases, underflow, and the
    /// complexity; this function behaves the same way.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut y = Float::ONE;
    /// assert_eq!(y.atan2_prec_assign_ref(&Float::ONE, 10), Less);
    /// assert_eq!(y.to_string(), "0.78516");
    /// ```
    #[inline]
    pub fn atan2_prec_assign_ref(&mut self, other: &Self, prec: u64) -> Ordering {
        let (t, o) = self.atan2_prec_ref_ref(other, prec);
        *self = t;
        o
    }

    /// Computes $\operatorname{atan2}(y,x)$, the angle of the point $(x,y)$ measured from the
    /// positive $x$-axis, rounding the result to the specified rounding mode. The first [`Float`]
    /// is replaced by the result, and the second is taken by value. An [`Ordering`] is returned,
    /// indicating whether the rounded angle is less than, equal to, or greater than the exact
    /// angle.
    ///
    /// See [`Float::atan2_prec_round`] for the error bounds, the special cases, underflow, and the
    /// complexity; this function behaves the same way.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the result cannot be represented exactly with the precision of
    /// the inputs.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut y = Float::from(0.3f64);
    /// assert_eq!(y.atan2_round_assign(Float::from(0.4f64), Floor), Less);
    /// assert_eq!(y.to_string(), "0.64350110879328426");
    /// ```
    #[inline]
    #[allow(clippy::needless_pass_by_value)]
    pub fn atan2_round_assign(&mut self, other: Self, rm: RoundingMode) -> Ordering {
        let prec = max(self.significant_bits(), other.significant_bits());
        let (t, o) = self.atan2_prec_round_ref_ref(&other, prec, rm);
        *self = t;
        o
    }

    /// Computes $\operatorname{atan2}(y,x)$, the angle of the point $(x,y)$ measured from the
    /// positive $x$-axis, rounding the result to the specified rounding mode. The first [`Float`]
    /// is replaced by the result, and the second is taken by reference. An [`Ordering`] is
    /// returned, indicating whether the rounded angle is less than, equal to, or greater than the
    /// exact angle.
    ///
    /// See [`Float::atan2_prec_round`] for the error bounds, the special cases, underflow, and the
    /// complexity; this function behaves the same way.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the result cannot be represented exactly with the precision of
    /// the inputs.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut y = Float::from(0.3f64);
    /// assert_eq!(y.atan2_round_assign_ref(&Float::from(0.4f64), Floor), Less);
    /// assert_eq!(y.to_string(), "0.64350110879328426");
    /// ```
    #[inline]
    pub fn atan2_round_assign_ref(&mut self, other: &Self, rm: RoundingMode) -> Ordering {
        let prec = max(self.significant_bits(), other.significant_bits());
        let (t, o) = self.atan2_prec_round_ref_ref(other, prec, rm);
        *self = t;
        o
    }
}

impl Atan2<Self> for Float {
    type Output = Self;

    /// Computes $\operatorname{atan2}(y,x)$, the angle of the point $(x,y)$ measured from the
    /// positive $x$-axis, taking both [`Float`]s by value.
    ///
    /// The precision of the output is the maximum of the precisions of the inputs, and the result
    /// is rounded to nearest. See [`Float::atan2_prec_round`] for the error bounds, the special
    /// cases, underflow, and the complexity.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Atan2;
    /// use malachite_float::Float;
    ///
    /// assert_eq!(
    ///     Float::from(0.3f64).atan2(Float::from(0.4f64)).to_string(),
    ///     "0.64350110879328437"
    /// );
    /// ```
    #[inline]
    fn atan2(self, other: Self) -> Self {
        self.atan2_round_ref_ref(&other, Nearest).0
    }
}

impl Atan2<&Self> for Float {
    type Output = Self;

    /// Computes $\operatorname{atan2}(y,x)$, the angle of the point $(x,y)$ measured from the
    /// positive $x$-axis, taking the first [`Float`] by value and the second by reference.
    ///
    /// See [`Float::atan2_prec_round`] for the error bounds, the special cases, underflow, and the
    /// complexity.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Atan2;
    /// use malachite_float::Float;
    ///
    /// assert_eq!(
    ///     Float::from(0.3f64).atan2(&Float::from(0.4f64)).to_string(),
    ///     "0.64350110879328437"
    /// );
    /// ```
    #[inline]
    fn atan2(self, other: &Self) -> Self {
        self.atan2_round_ref_ref(other, Nearest).0
    }
}

impl Atan2<Float> for &Float {
    type Output = Float;

    /// Computes $\operatorname{atan2}(y,x)$, the angle of the point $(x,y)$ measured from the
    /// positive $x$-axis, taking the first [`Float`] by reference and the second by value.
    ///
    /// See [`Float::atan2_prec_round`] for the error bounds, the special cases, underflow, and the
    /// complexity.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Atan2;
    /// use malachite_float::Float;
    ///
    /// assert_eq!(
    ///     (&Float::from(0.3f64))
    ///         .atan2(Float::from(0.4f64))
    ///         .to_string(),
    ///     "0.64350110879328437"
    /// );
    /// ```
    #[inline]
    fn atan2(self, other: Float) -> Float {
        self.atan2_round_ref_ref(&other, Nearest).0
    }
}

impl Atan2<&Float> for &Float {
    type Output = Float;

    /// Computes $\operatorname{atan2}(y,x)$, the angle of the point $(x,y)$ measured from the
    /// positive $x$-axis, taking both [`Float`]s by reference.
    ///
    /// See [`Float::atan2_prec_round`] for the error bounds, the special cases, underflow, and the
    /// complexity.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Atan2;
    /// use malachite_float::Float;
    ///
    /// assert_eq!(
    ///     (&Float::from(0.3f64))
    ///         .atan2(&Float::from(0.4f64))
    ///         .to_string(),
    ///     "0.64350110879328437"
    /// );
    /// ```
    #[inline]
    fn atan2(self, other: &Float) -> Float {
        self.atan2_round_ref_ref(other, Nearest).0
    }
}

impl Atan2Assign<Self> for Float {
    /// Replaces a [`Float`] $y$ with $\operatorname{atan2}(y,x)$, taking $x$ by value.
    ///
    /// See [`Float::atan2_prec_round`] for the error bounds, the special cases, underflow, and the
    /// complexity.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Atan2Assign;
    /// use malachite_float::Float;
    ///
    /// let mut y = Float::from(0.3f64);
    /// y.atan2_assign(Float::from(0.4f64));
    /// assert_eq!(y.to_string(), "0.64350110879328437");
    /// ```
    #[inline]
    fn atan2_assign(&mut self, other: Self) {
        self.atan2_round_assign_ref(&other, Nearest);
    }
}

impl Atan2Assign<&Self> for Float {
    /// Replaces a [`Float`] $y$ with $\operatorname{atan2}(y,x)$, taking $x$ by reference.
    ///
    /// See [`Float::atan2_prec_round`] for the error bounds, the special cases, underflow, and the
    /// complexity.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Atan2Assign;
    /// use malachite_float::Float;
    ///
    /// let mut y = Float::from(0.3f64);
    /// y.atan2_assign(&Float::from(0.4f64));
    /// assert_eq!(y.to_string(), "0.64350110879328437");
    /// ```
    #[inline]
    fn atan2_assign(&mut self, other: &Self) {
        self.atan2_round_assign_ref(other, Nearest);
    }
}

/// Computes $\operatorname{atan2}(y,x)$, the angle of the point $(x,y)$ measured from the positive
/// $x$-axis, for primitive floats.
///
/// $$
/// f(y,x) = \operatorname{atan2}(y,x)+\varepsilon,
/// $$
/// where $|\varepsilon| < 2^{\lfloor\log_2 |\operatorname{atan2}(y,x)|\rfloor-p}$ and $p$ is the
/// precision of the output (24 if `T` is a [`f32`] and 53 if `T` is a [`f64`]); the special cases
/// below are exact.
///
/// Special cases, in which the sign of a zero argument selects the quadrant:
/// - $f(\text{NaN},x)=f(y,\text{NaN})=\text{NaN}$
/// - $f(\pm0.0,x)=\pm0.0$ if $x$ is positive or $+0.0$, and $\pm\pi$ if $x$ is negative or $-0.0$
/// - $f(y,\pm0.0)=\pm\pi/2$, with the sign of $y$, for nonzero $y$
/// - $f(\pm\infty,x)=\pm\pi/2$ for finite $x$, $\pm\pi/4$ for $+\infty$, and $\pm3\pi/4$ for
///   $-\infty$
/// - $f(y,+\infty)=\pm0.0$ and $f(y,-\infty)=\pm\pi$, with the sign of $y$, for finite nonzero $y$
///
/// Overflow is not possible, since $|\operatorname{atan2}(y,x)| \leq \pi$. The result is subnormal,
/// or zero, only for a positive $x$ with $|y/x|$ subnormal or smaller.
///
/// # Worst-case complexity
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use malachite_base::num::float::NiceFloat;
/// use malachite_float::float::arithmetic::atan2::primitive_float_atan2;
///
/// assert!(primitive_float_atan2(f32::NAN, 1.0).is_nan());
/// assert_eq!(
///     NiceFloat(primitive_float_atan2(1.0f32, 1.0)),
///     NiceFloat(0.7853982)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_atan2(1.0f64, 1.0)),
///     NiceFloat(0.7853981633974483)
/// );
/// // a negative x with a zero y is half a turn
/// assert_eq!(
///     NiceFloat(primitive_float_atan2(0.0f64, -1.0)),
///     NiceFloat(3.141592653589793)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_atan2(-0.0f64, -1.0)),
///     NiceFloat(-3.141592653589793)
/// );
/// ```
#[inline]
#[allow(clippy::type_repetition_in_bounds)]
pub fn primitive_float_atan2<T: PrimitiveFloat>(y: T, x: T) -> T
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float>,
{
    emulate_float_float_to_float_fn(|y, x, prec| y.atan2_prec_ref_ref(&x, prec), y, x)
}
