// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the GNU MPFR Library.
//
//      Copyright © 2005-2025 Free Software Foundation, Inc.
//
//      Contributed by the Pascaline and Caramba projects, INRIA.
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

// Port of MPFR's cosecant. `mpfr_csc` (`csc.c`) instantiates the generic reciprocal template
// (`gen_inverse.h`) with the sine: the sine is taken at the working precision, rounded toward zero,
// its reciprocal is rounded to nearest, and the result is certified with two bits of slack, inside
// a Ziv loop. The cosecant never underflows, since its magnitude is at least 1, but it overflows
// for an input within 2^(-2^30) of a multiple of pi, which MPFR's wider exponent range never sees;
// a reciprocal at the top of the range is decided from an exact bracket instead. MPFR's shortcut
// for a tiny input, where csc x is 1/x + x/6 + O(x^3), is kept: there the Ziv loop could never
// certify a reciprocal that is exactly representable.

use crate::InnerFloat::{Finite, Infinity, NaN, Zero};
use crate::float::arithmetic::cos::{phi_minus_1_prec_round, signed_constant, sin_bound};
use crate::float::arithmetic::sec::doubled;
use crate::float::arithmetic::sin::sin_rational_helper;
use crate::float::arithmetic::tan::{
    MAX_SETTLED_EXPONENT, round_bracket_signed, round_bracket_signed_by,
};
use crate::{Float, emulate_float_to_float_fn, emulate_rational_to_float_fn};
use core::cmp::Ordering::{self, Equal, Greater, Less};
use core::cmp::max;
use malachite_base::num::arithmetic::traits::{
    Abs, CeilingLogBase2, Csc, CscAssign, Mod, PowerOf2, Reciprocal,
};
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::{
    Infinity as InfinityTrait, NaN as NaNTrait, NegativeInfinity,
};
use malachite_base::num::comparison::traits::PartialOrdAbs;
use malachite_base::num::conversion::traits::{ExactFrom, RoundingFrom};
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{
    self, Ceiling, Down, Exact, Floor, Nearest, Up,
};
use malachite_nz::integer::Integer;
use malachite_nz::natural::arithmetic::float::round::float_can_round;
use malachite_nz::platform::Limb;
use malachite_q::Rational;

// csc x for a tiny x, where csc x = 1/x + x/6 + ... and |csc x - 1/x| <= 0.2 for |x| <= 1, with the
// correction sharing the sign of 1/x, so that |csc x| > |1/x|. MPFR's condition, EXP(x) <= -2
// max(PREC(x), prec), makes rounding 1/x settle the cosecant, except when 1/x is exact (x a power
// of 2), where the true value lies one step beyond it, away from zero. The general loop could not
// settle that case at any working precision, since the reciprocal is then exactly representable.
//
// This is ACTION_TINY from csc.c, MPFR 4.2.2.
fn csc_tiny(x: &Float, prec: u64, rm: RoundingMode) -> (Float, Ordering) {
    let (r, o) = x.reciprocal_prec_round_ref(prec, rm);
    if o != Equal {
        return (r, o);
    }
    assert_ne!(rm, Exact, "Inexact csc");
    let negative = x.is_sign_negative();
    // 1/x is exact, so the cosecant is one step beyond it, away from zero
    let away = match rm {
        Ceiling => !negative,
        Floor => negative,
        Up => true,
        _ => false,
    };
    let mut r = r;
    if away {
        if negative {
            r.decrement();
        } else {
            r.increment();
        }
        (r, if negative { Less } else { Greater })
    } else {
        (r, if negative { Greater } else { Less })
    }
}

// As in mpfr_overflow, with the overflow's sign: the toward-zero modes give the largest finite
// value, and the other modes an infinity.
fn csc_overflow(negative: bool, prec: u64, rm: RoundingMode) -> (Float, Ordering) {
    match (negative, rm) {
        (_, Exact) => panic!("Inexact csc"),
        (false, Floor | Down) => (Float::max_finite_value_with_prec(prec), Less),
        (false, _) => (Float::INFINITY, Greater),
        (true, Ceiling | Down) => (-Float::max_finite_value_with_prec(prec), Greater),
        (true, _) => (Float::NEGATIVE_INFINITY, Less),
    }
}

// Decides csc(x) = 1/c from the sine rounded toward zero at precision m, by a `Rational` bracket,
// for the cases the `Float` reciprocal cannot settle: it overflowed, or lies within two bits of the
// top of the exponent range, where rounding it to `prec` could still cross the end. Returns `None`
// if the bracket does not decide the rounding, so that the working precision must grow.
fn csc_bracket(c: &Float, m: u64, prec: u64, rm: RoundingMode) -> Option<(Float, Ordering)> {
    let negative = c.is_sign_negative();
    // A sine that underflowed toward zero is below the smallest positive `Float`, so the cosecant
    // is above 2^(2^30), beyond the largest finite one.
    if *c == 0u32 {
        return Some(csc_overflow(negative, prec, rm));
    }
    // Rounding toward zero puts the sine's magnitude in [|c|, |c| + ulp), so the cosecant's lies in
    // (1/(|c| + ulp), 1/|c|].
    let exp_c = i64::from(c.get_exponent().unwrap());
    let lo = Rational::exact_from(c).abs();
    let hi = &lo + Rational::power_of_2(exp_c - i64::exact_from(m));
    round_bracket_signed_by(negative, hi.reciprocal(), lo.reciprocal(), prec, rm)
}

// This is mpfr_csc from csc.c, MPFR 4.2.2, with the bracket path for results near the top of the
// exponent range.
fn csc_prec_round_normal_ref(x: &Float, prec: u64, rm: RoundingMode) -> (Float, Ordering) {
    assert_ne!(rm, Exact, "Inexact csc");
    let exp_x = i64::from(x.get_exponent().unwrap());
    // ACTION_TINY from csc.c: EXP(x) <= -2 max(PREC(x), PREC(y))
    let n = i64::exact_from(max(x.get_prec().unwrap(), prec));
    if exp_x <= -(n << 1) {
        return csc_tiny(x, prec, rm);
    }
    // Compute initial precision
    let mut m = prec + prec.ceiling_log_base_2() + 3;
    let mut increment = Limb::WIDTH;
    loop {
        // err < 1 ulp, and of a known sign: rounding toward zero puts the sine below the true one
        // in magnitude
        let c = x.sin_prec_round_ref(m, Down).0;
        // err < 1/2 + 2 < 4 ulps in all, as in algorithms.tex
        let r = (&c).reciprocal();
        // A reciprocal whose exponent is below MAX_SETTLED_EXPONENT can be rounded to any precision
        // without leaving the exponent range, so the `Float` reciprocal settles it; the rest go to
        // the bracket. The cosecant's magnitude is at least 1, so only the top of the range is in
        // play.
        match r.get_exponent().map(i64::from) {
            Some(e) if e < MAX_SETTLED_EXPONENT => {
                if float_can_round(r.significand_ref().unwrap(), m - 2, prec, rm) {
                    return Float::from_float_prec_round(r, prec, rm);
                }
            }
            _ => {
                if let Some(result) = csc_bracket(&c, m, prec, rm) {
                    return result;
                }
            }
        }
        m += increment;
        increment = m >> 1;
    }
}

// csc x for a tiny nonzero `Rational` x, bracketed by inverting a bracket on the sine: `sin_bound`
// pins sin x from both sides at a growing working precision, and the reciprocals of those bounds
// bracket the cosecant until the rounding is unambiguous (the cosecant of a nonzero rational is
// transcendental, so it eventually is). The general path cannot settle such an x: there csc x is
// 1/x + x/6 + ..., with the correction far below the resolution of any reciprocal of a rounded
// sine, so the Ziv loop would have to raise the working precision to about twice the input's
// exponent, which is unbounded below the `Float` exponent range.
fn csc_rational_tiny(
    x: &Rational,
    ax: &Rational,
    prec: u64,
    rm: RoundingMode,
) -> (Float, Ordering) {
    let mut w = prec + 64;
    loop {
        // sin x lies in [s_lo, s_hi], so its reciprocal lies in [1/s_hi, 1/s_lo]
        let s_lo = sin_bound(ax, w, false);
        let s_hi = sin_bound(ax, w, true);
        if let Some(result) =
            round_bracket_signed(x, s_hi.reciprocal(), s_lo.reciprocal(), prec, rm)
        {
            return result;
        }
        w <<= 1;
    }
}

// Computes csc(x) for a nonzero `Rational` x, rounded to precision `prec` with rounding mode `rm`.
// (csc(0) = infinity is handled by the caller.) The cosecant of a nonzero rational is
// transcendental, so the result is never exactly representable and `rm` must not be `Exact`.
//
// This is the `Float` algorithm with the sine taken from `sin_rational_helper`, which rounds the
// input once and handles both a tiny x and an x too large to be a `Float`, and with a direct
// bracket for a tiny input, standing in for MPFR's shortcut there.
pub(crate) fn csc_rational_helper(x: &Rational, prec: u64, rm: RoundingMode) -> (Float, Ordering) {
    assert_ne!(rm, Exact, "Inexact csc");
    let exp_x = x.floor_log_base_2_abs() + 1; // the MPFR-style exponent of x
    // Below this the reciprocal of a rounded sine can never be certified: the correction x/6 is
    // smaller than any ulp the loop could reach without raising the working precision to about
    // twice the input's exponent.
    if exp_x < 0 && -(exp_x << 2) > i64::exact_from(prec) + 3 {
        return csc_rational_tiny(x, &x.abs(), prec, rm);
    }
    let mut m = prec + prec.ceiling_log_base_2() + 3;
    let mut increment = Limb::WIDTH;
    loop {
        // err < 1 ulp, and of a known sign: rounding toward zero puts the sine below the true one
        // in magnitude
        let s = sin_rational_helper(x, m, Down).0;
        // err < 1/2 + 2 < 4 ulps in all, as in algorithms.tex
        let r = (&s).reciprocal();
        match r.get_exponent().map(i64::from) {
            Some(e) if e < MAX_SETTLED_EXPONENT => {
                if float_can_round(r.significand_ref().unwrap(), m - 2, prec, rm) {
                    return Float::from_float_prec_round(r, prec, rm);
                }
            }
            _ => {
                if let Some(result) = csc_bracket(&s, m, prec, rm) {
                    return result;
                }
            }
        }
        m += increment;
        increment = m >> 1;
    }
}

// The exact and closed-form values of csc(2 pi q) at the eighths, twelfths, and twentieths of a
// turn, where the sine is 0, ±1, ±1/2, ±sqrt(2)/2, ±sqrt(3)/2, ±phi/2, or ±(phi - 1)/2.
// Returns `None` when q is none of them, or when only an inexact value is available and `rm` is
// `Exact`.
fn csc_turns_special_case(q: &Rational, prec: u64, rm: RoundingMode) -> Option<(Float, Ordering)> {
    let d = q.denominator_ref();
    if *d > 20u32 {
        return None;
    }
    let d = u64::exact_from(d);
    // the angle in units of 1/d of a turn (the numerator of a `Rational` is unsigned, so the sign
    // is restored before reducing modulo d)
    let n = u64::exact_from(
        &Integer::from_sign_and_abs_ref(*q >= 0u32, q.numerator_ref()).mod_op(Integer::from(d)),
    );
    // the cosecant, like the sine, is negative in the second half of the turn
    let negative = n > d >> 1;
    match d {
        // The poles at 0 and 180°, where the sine is a zero carrying the sign of q, so its
        // reciprocal is an infinity with that sign; that keeps the cosecant odd.
        1 | 2 => Some((
            if *q < 0u32 {
                Float::NEGATIVE_INFINITY
            } else {
                Float::INFINITY
            },
            Equal,
        )),
        // csc(90°) = 1, csc(270°) = -1
        4 => Some((
            if negative {
                -Float::one_prec(prec)
            } else {
                Float::one_prec(prec)
            },
            Equal,
        )),
        // csc(30°) = csc(150°) = 2, csc(210°) = csc(330°) = -2
        12 => Some((
            if negative {
                -(Float::one_prec(prec) << 1u32)
            } else {
                Float::one_prec(prec) << 1u32
            },
            Equal,
        )),
        _ if rm == Exact => None,
        // csc(60°) = csc(120°) = 2 sqrt(3)/3, and its negative at 240° and 300°. Doubling is
        // exact, so the correctly rounded constant stays correctly rounded.
        3 | 6 => Some(doubled(signed_constant(
            Float::sqrt_3_over_3_prec_round,
            negative,
            prec,
            rm,
        ))),
        // csc(45°) = csc(135°) = sqrt(2), and its negative at 225° and 315°
        8 => Some(signed_constant(
            Float::sqrt_2_prec_round,
            negative,
            prec,
            rm,
        )),
        // The sine is (phi - 1)/2 at 18° and phi/2 at 54°, so the cosecant is 2 phi and 2(phi -
        // 1) there, and their negatives in the second half of the turn.
        20 => Some(if n == 1 || n == 9 || n == 11 || n == 19 {
            doubled(signed_constant(Float::phi_prec_round, negative, prec, rm))
        } else {
            doubled(signed_constant(phi_minus_1_prec_round, negative, prec, rm))
        }),
        _ => None,
    }
}

// Computes csc(2 pi x/u) for a finite nonzero `Float` x and a nonzero u. This has no MPFR
// counterpart; it is `csc` with the sine taken in uths of a turn, which reduces the argument
// exactly rather than modulo an approximation of 2 pi, and so reaches the exact and closed-form
// cases that the radian version cannot see. MPFR's shortcut for a tiny input is not needed here:
// the angle 2 pi x/u is never a `Float`, so the reciprocal of the rounded sine is not stuck on an
// exactly representable value, and an angle below the exponent range makes the sine underflow,
// which the bracket reads as an overflow.
fn csc_with_period_prec_round_normal_ref(
    x: &Float,
    u: u64,
    prec: u64,
    rm: RoundingMode,
) -> (Float, Ordering) {
    // Range reduction, as in `tan_with_period`: the argument is already reduced if |x| < u.
    let xr;
    let xp = if x.lt_abs(&u) {
        x
    } else {
        // xr = x mod u, with the sign of x, exactly
        let p = i64::exact_from(x.get_prec().unwrap()) - i64::from(x.get_exponent().unwrap());
        let (r, o) =
            x.rem_unsigned_prec_round_ref(u, u64::WIDTH + u64::exact_from(max(p, 0)), Exact);
        assert_eq!(o, Equal);
        if r == 0u32 {
            // x is a multiple of u, so the sine is a zero with the sign of x and the cosecant is an
            // infinity with that sign
            return (
                if *x < 0u32 {
                    Float::NEGATIVE_INFINITY
                } else {
                    Float::INFINITY
                },
                Equal,
            );
        }
        xr = r;
        &xr
    };
    // now |xp/u| < 1
    let exp_x = i64::from(xp.get_exponent().unwrap());
    // The special cases need |x/u| >= 1/20, so the exponent test skips the `Rational` construction
    // for the small x that would make it expensive (a tiny x has a huge power-of-2 denominator).
    if exp_x >= i64::exact_from(u.significant_bits()) - 5
        && let Some(result) =
            csc_turns_special_case(&(Rational::exact_from(xp) / Rational::from(u)), prec, rm)
    {
        return result;
    }
    // Only the exact cases can be rounded exactly
    assert_ne!(rm, Exact, "Inexact csc_with_period");
    let mut m = prec + prec.ceiling_log_base_2() + 3;
    let mut increment = Limb::WIDTH;
    loop {
        // err < 1 ulp, and of a known sign: rounding toward zero puts the sine below the true one
        // in magnitude
        let s = xp.sin_with_period_prec_round_ref(u, m, Down).0;
        // err < 1/2 + 2 < 4 ulps in all, as in algorithms.tex
        let r = (&s).reciprocal();
        match r.get_exponent().map(i64::from) {
            Some(e) if e < MAX_SETTLED_EXPONENT => {
                if float_can_round(r.significand_ref().unwrap(), m - 2, prec, rm) {
                    return Float::from_float_prec_round(r, prec, rm);
                }
            }
            _ => {
                if let Some(result) = csc_bracket(&s, m, prec, rm) {
                    return result;
                }
            }
        }
        m += increment;
        increment = m >> 1;
    }
}

impl Float {
    /// Computes $\csc x$, the cosecant of a [`Float`], rounding the result to the specified
    /// precision and with the specified rounding mode. The [`Float`] is taken by value. An
    /// [`Ordering`] is also returned, indicating whether the rounded cosecant is less than, equal
    /// to, or greater than the exact cosecant. Although `NaN`s are not comparable to any [`Float`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \csc x+\varepsilon.
    /// $$
    /// - If $x$ is not finite, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x$ is finite and $m$ is not `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2 |\csc
    ///   x|\rfloor-p+1}$.
    /// - If $x$ is finite and $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2 |\csc
    ///   x|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=\text{NaN}$
    /// - $f(\pm\infty,p,m)=\text{NaN}$
    /// - $f(\pm0.0,p,m)=\pm\infty$
    ///
    /// Overflow:
    /// - If $f(x,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$ is
    ///   returned instead.
    /// - If $f(x,p,m)\leq -2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is
    ///   returned instead.
    /// - If $f(x,p,m)\leq -2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`, $-(1-(1/2)^p)2^{2^{30}-1}$
    ///   is returned instead.
    /// - If $0<f(x,p,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,p,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned instead.
    ///
    /// Underflow is not possible, since $|\csc x| \geq 1$. Overflow requires an input within
    /// $2^{-2^{30}}$ of a nonzero multiple of $\pi$, which takes more than $2^{30}$ bits of
    /// precision, or an input of magnitude about $2^{-2^{30}}$, whose reciprocal alone is beyond
    /// the largest finite [`Float`].
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::csc_prec`] instead. If you
    /// know that your target precision is the precision of the input, consider using
    /// [`Float::csc_round`] instead. If both of these things are true, consider using
    /// [`Float::csc`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m, e) = O(n (\log n)^3 \log\log n + (n+m+e) (\log (n+m+e))^2 \log\log (n+m+e))$
    ///
    /// $M(n, m, e) = O((n+m+e) \log (n+m+e))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, $m$ is
    /// `self.significant_bits()`, and $e$ is the exponent of `self` (0 if `self` has no exponent or
    /// a negative one): the cosine at working precision $n$, summed by binary splitting of the
    /// Taylor series for large $n$, and its reciprocal cost the first term, and for $|x| \geq 4$
    /// the argument is reduced modulo $2\pi$, which requires $\pi$ to about $n + e$ bits and a
    /// remainder of the $m$-bit input. Unlike most functions, `csc` therefore gets slower as the
    /// magnitude of its input grows, not just as the precision does.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact`, since the cosecant of a finite nonzero [`Float`] is never exactly
    /// representable, or if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .csc_prec_round(5, Floor);
    /// assert_eq!(c.to_string(), "1.19");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .csc_prec_round(5, Ceiling);
    /// assert_eq!(c.to_string(), "1.25");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .csc_prec_round(5, Nearest);
    /// assert_eq!(c.to_string(), "1.19");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .csc_prec_round(20, Floor);
    /// assert_eq!(c.to_string(), "1.1883945");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .csc_prec_round(20, Ceiling);
    /// assert_eq!(c.to_string(), "1.1883965");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .csc_prec_round(20, Nearest);
    /// assert_eq!(c.to_string(), "1.1883945");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn csc_prec_round(self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        self.csc_prec_round_ref(prec, rm)
    }

    /// Computes $\csc x$, the cosecant of a [`Float`], rounding the result to the specified
    /// precision and with the specified rounding mode. The [`Float`] is taken by reference. An
    /// [`Ordering`] is also returned, indicating whether the rounded cosecant is less than, equal
    /// to, or greater than the exact cosecant. Although `NaN`s are not comparable to any [`Float`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \csc x+\varepsilon.
    /// $$
    /// - If $x$ is not finite, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x$ is finite and $m$ is not `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2 |\csc
    ///   x|\rfloor-p+1}$.
    /// - If $x$ is finite and $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2 |\csc
    ///   x|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=\text{NaN}$
    /// - $f(\pm\infty,p,m)=\text{NaN}$
    /// - $f(\pm0.0,p,m)=\pm\infty$
    ///
    /// Overflow:
    /// - If $f(x,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$ is
    ///   returned instead.
    /// - If $f(x,p,m)\leq -2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is
    ///   returned instead.
    /// - If $f(x,p,m)\leq -2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`, $-(1-(1/2)^p)2^{2^{30}-1}$
    ///   is returned instead.
    /// - If $0<f(x,p,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,p,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned instead.
    ///
    /// Underflow is not possible, since $|\csc x| \geq 1$. Overflow requires an input within
    /// $2^{-2^{30}}$ of a nonzero multiple of $\pi$, which takes more than $2^{30}$ bits of
    /// precision, or an input of magnitude about $2^{-2^{30}}$, whose reciprocal alone is beyond
    /// the largest finite [`Float`].
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::csc_prec_ref`] instead. If
    /// you know that your target precision is the precision of the input, consider using
    /// [`Float::csc_round_ref`] instead. If both of these things are true, consider using
    /// `(&Float).csc()` instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m, e) = O(n (\log n)^3 \log\log n + (n+m+e) (\log (n+m+e))^2 \log\log (n+m+e))$
    ///
    /// $M(n, m, e) = O((n+m+e) \log (n+m+e))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, $m$ is
    /// `self.significant_bits()`, and $e$ is the exponent of `self` (0 if `self` has no exponent or
    /// a negative one): the cosine at working precision $n$, summed by binary splitting of the
    /// Taylor series for large $n$, and its reciprocal cost the first term, and for $|x| \geq 4$
    /// the argument is reduced modulo $2\pi$, which requires $\pi$ to about $n + e$ bits and a
    /// remainder of the $m$-bit input. Unlike most functions, `csc` therefore gets slower as the
    /// magnitude of its input grows, not just as the precision does.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact`, since the cosecant of a finite nonzero [`Float`] is never exactly
    /// representable, or if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = (&Float::from_unsigned_prec(1u32, 100).0).csc_prec_round_ref(5, Floor);
    /// assert_eq!(c.to_string(), "1.19");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = (&Float::from_unsigned_prec(1u32, 100).0).csc_prec_round_ref(5, Ceiling);
    /// assert_eq!(c.to_string(), "1.25");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = (&Float::from_unsigned_prec(1u32, 100).0).csc_prec_round_ref(5, Nearest);
    /// assert_eq!(c.to_string(), "1.19");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = (&Float::from_unsigned_prec(1u32, 100).0).csc_prec_round_ref(20, Floor);
    /// assert_eq!(c.to_string(), "1.1883945");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = (&Float::from_unsigned_prec(1u32, 100).0).csc_prec_round_ref(20, Ceiling);
    /// assert_eq!(c.to_string(), "1.1883965");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = (&Float::from_unsigned_prec(1u32, 100).0).csc_prec_round_ref(20, Nearest);
    /// assert_eq!(c.to_string(), "1.1883945");
    /// assert_eq!(o, Less);
    /// ```
    pub fn csc_prec_round_ref(&self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        assert_ne!(prec, 0);
        match &self.0 {
            NaN | Infinity { .. } => (Self::NAN, Equal),
            // csc(+0) = +infinity, csc(-0) = -infinity
            Zero { .. } => (
                if self.is_sign_negative() {
                    Self::NEGATIVE_INFINITY
                } else {
                    Self::INFINITY
                },
                Equal,
            ),
            Finite { .. } => csc_prec_round_normal_ref(self, prec, rm),
        }
    }

    /// Computes $\csc x$, the cosecant of a [`Float`], rounding the result to the nearest value of
    /// the specified precision. The [`Float`] is taken by value. An [`Ordering`] is also returned,
    /// indicating whether the rounded cosecant is less than, equal to, or greater than the exact
    /// cosecant. Although `NaN`s are not comparable to any [`Float`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// If the cosecant is equidistant from two [`Float`]s with the specified precision, the
    /// [`Float`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a
    /// description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = \csc x+\varepsilon.
    /// $$
    /// - If $x$ is not finite, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x$ is finite, then $|\varepsilon| < 2^{\lfloor\log_2 |\csc x|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p)=\text{NaN}$
    /// - $f(\pm\infty,p)=\text{NaN}$
    /// - $f(\pm0.0,p)=\pm\infty$
    ///
    /// Overflow:
    /// - If $f(x,p)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,p)\leq -2^{2^{30}-1}$, $-\infty$ is returned instead.
    /// - If $0<f(x,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,p)<0$, $-0.0$ is returned instead.
    ///
    /// Underflow is not possible, since $|\csc x| \geq 1$. Overflow requires an input within
    /// $2^{-2^{30}}$ of a nonzero multiple of $\pi$, which takes more than $2^{30}$ bits of
    /// precision, or an input of magnitude about $2^{-2^{30}}$, whose reciprocal alone is beyond
    /// the largest finite [`Float`].
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::csc_prec_round`] instead. If you know that your target precision is the precision
    /// of the input, consider using [`Float::csc`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m, e) = O(n (\log n)^3 \log\log n + (n+m+e) (\log (n+m+e))^2 \log\log (n+m+e))$
    ///
    /// $M(n, m, e) = O((n+m+e) \log (n+m+e))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, $m$ is
    /// `self.significant_bits()`, and $e$ is the exponent of `self` (0 if `self` has no exponent or
    /// a negative one): the cosine at working precision $n$, summed by binary splitting of the
    /// Taylor series for large $n$, and its reciprocal cost the first term, and for $|x| \geq 4$
    /// the argument is reduced modulo $2\pi$, which requires $\pi$ to about $n + e$ bits and a
    /// remainder of the $m$-bit input. Unlike most functions, `csc` therefore gets slower as the
    /// magnitude of its input grows, not just as the precision does.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100).0.csc_prec(5);
    /// assert_eq!(c.to_string(), "1.19");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100).0.csc_prec(20);
    /// assert_eq!(c.to_string(), "1.1883945");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn csc_prec(self, prec: u64) -> (Self, Ordering) {
        self.csc_prec_round(prec, Nearest)
    }

    /// Computes $\csc x$, the cosecant of a [`Float`], rounding the result to the nearest value of
    /// the specified precision. The [`Float`] is taken by reference. An [`Ordering`] is also
    /// returned, indicating whether the rounded cosecant is less than, equal to, or greater than
    /// the exact cosecant. Although `NaN`s are not comparable to any [`Float`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// If the cosecant is equidistant from two [`Float`]s with the specified precision, the
    /// [`Float`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a
    /// description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = \csc x+\varepsilon.
    /// $$
    /// - If $x$ is not finite, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x$ is finite, then $|\varepsilon| < 2^{\lfloor\log_2 |\csc x|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p)=\text{NaN}$
    /// - $f(\pm\infty,p)=\text{NaN}$
    /// - $f(\pm0.0,p)=\pm\infty$
    ///
    /// Overflow:
    /// - If $f(x,p)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,p)\leq -2^{2^{30}-1}$, $-\infty$ is returned instead.
    /// - If $0<f(x,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,p)<0$, $-0.0$ is returned instead.
    ///
    /// Underflow is not possible, since $|\csc x| \geq 1$. Overflow requires an input within
    /// $2^{-2^{30}}$ of a nonzero multiple of $\pi$, which takes more than $2^{30}$ bits of
    /// precision, or an input of magnitude about $2^{-2^{30}}$, whose reciprocal alone is beyond
    /// the largest finite [`Float`].
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::csc_prec_round_ref`] instead. If you know that your target precision is the
    /// precision of the input, consider using `(&Float).csc()` instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m, e) = O(n (\log n)^3 \log\log n + (n+m+e) (\log (n+m+e))^2 \log\log (n+m+e))$
    ///
    /// $M(n, m, e) = O((n+m+e) \log (n+m+e))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, $m$ is
    /// `self.significant_bits()`, and $e$ is the exponent of `self` (0 if `self` has no exponent or
    /// a negative one): the cosine at working precision $n$, summed by binary splitting of the
    /// Taylor series for large $n$, and its reciprocal cost the first term, and for $|x| \geq 4$
    /// the argument is reduced modulo $2\pi$, which requires $\pi$ to about $n + e$ bits and a
    /// remainder of the $m$-bit input. Unlike most functions, `csc` therefore gets slower as the
    /// magnitude of its input grows, not just as the precision does.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = (&Float::from_unsigned_prec(1u32, 100).0).csc_prec_ref(5);
    /// assert_eq!(c.to_string(), "1.19");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = (&Float::from_unsigned_prec(1u32, 100).0).csc_prec_ref(20);
    /// assert_eq!(c.to_string(), "1.1883945");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn csc_prec_ref(&self, prec: u64) -> (Self, Ordering) {
        self.csc_prec_round_ref(prec, Nearest)
    }

    /// Computes $\csc x$, the cosecant of a [`Float`], rounding the result with the specified
    /// rounding mode. The [`Float`] is taken by value. An [`Ordering`] is also returned, indicating
    /// whether the rounded cosecant is less than, equal to, or greater than the exact cosecant.
    /// Although `NaN`s are not comparable to any [`Float`], whenever this function returns a `NaN`
    /// it also returns `Equal`.
    ///
    /// The precision of the output is the precision of the input. See [`RoundingMode`] for a
    /// description of the possible rounding modes.
    ///
    /// $$
    /// f(x,m) = \csc x+\varepsilon.
    /// $$
    /// - If $x$ is not finite, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x$ is finite and $m$ is not `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2 |\csc
    ///   x|\rfloor-p+1}$, where $p$ is the precision of the input.
    /// - If $x$ is finite and $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2 |\csc
    ///   x|\rfloor-p}$, where $p$ is the precision of the input.
    ///
    /// If the output has a precision, it is the precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN},m)=\text{NaN}$
    /// - $f(\pm\infty,m)=\text{NaN}$
    /// - $f(\pm0.0,m)=\pm\infty$
    ///
    /// Overflow:
    /// - If $f(x,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$ is
    ///   returned instead.
    /// - If $f(x,m)\leq -2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is
    ///   returned instead.
    /// - If $f(x,m)\leq -2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`, $-(1-(1/2)^p)2^{2^{30}-1}$
    ///   is returned instead.
    /// - If $0<f(x,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned instead.
    ///
    /// Underflow is not possible, since $|\csc x| \geq 1$. Overflow requires an input within
    /// $2^{-2^{30}}$ of a nonzero multiple of $\pi$, which takes more than $2^{30}$ bits of
    /// precision, or an input of magnitude about $2^{-2^{30}}$, whose reciprocal alone is beyond
    /// the largest finite [`Float`].
    ///
    /// If you want to specify an output precision, consider using [`Float::csc_prec_round`]
    /// instead. If you know you'll be using the `Nearest` rounding mode, consider using
    /// [`Float::csc`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, e) = O(n (\log n)^3 \log\log n + (n+e) (\log (n+e))^2 \log\log (n+e))$
    ///
    /// $M(n, e) = O((n+e) \log (n+e))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits()`, and $e$ is
    /// the exponent of `self` (0 if `self` has no exponent or a negative one): the Taylor series at
    /// working precision $n$, summed by binary splitting for large $n$, costs the first term, and
    /// for $|x| \geq 4$ the argument is reduced modulo $2\pi$, which requires $\pi$ to about $n +
    /// e$ bits. Unlike most functions, `csc` therefore gets slower as the magnitude of its input
    /// grows, not just as the precision does.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact`, since the cosecant of a finite nonzero [`Float`] is never exactly
    /// representable.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100).0.csc_round(Floor);
    /// assert_eq!(c.to_string(), "1.1883951057781212162615994523744");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100).0.csc_round(Ceiling);
    /// assert_eq!(c.to_string(), "1.1883951057781212162615994523760");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100).0.csc_round(Nearest);
    /// assert_eq!(c.to_string(), "1.1883951057781212162615994523744");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn csc_round(self, rm: RoundingMode) -> (Self, Ordering) {
        let prec = self.significant_bits();
        self.csc_prec_round(prec, rm)
    }

    /// Computes $\csc x$, the cosecant of a [`Float`], rounding the result with the specified
    /// rounding mode. The [`Float`] is taken by reference. An [`Ordering`] is also returned,
    /// indicating whether the rounded cosecant is less than, equal to, or greater than the exact
    /// cosecant. Although `NaN`s are not comparable to any [`Float`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the precision of the input. See [`RoundingMode`] for a
    /// description of the possible rounding modes.
    ///
    /// $$
    /// f(x,m) = \csc x+\varepsilon.
    /// $$
    /// - If $x$ is not finite, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x$ is finite and $m$ is not `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2 |\csc
    ///   x|\rfloor-p+1}$, where $p$ is the precision of the input.
    /// - If $x$ is finite and $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2 |\csc
    ///   x|\rfloor-p}$, where $p$ is the precision of the input.
    ///
    /// If the output has a precision, it is the precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN},m)=\text{NaN}$
    /// - $f(\pm\infty,m)=\text{NaN}$
    /// - $f(\pm0.0,m)=\pm\infty$
    ///
    /// Overflow:
    /// - If $f(x,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$ is
    ///   returned instead.
    /// - If $f(x,m)\leq -2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is
    ///   returned instead.
    /// - If $f(x,m)\leq -2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`, $-(1-(1/2)^p)2^{2^{30}-1}$
    ///   is returned instead.
    /// - If $0<f(x,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned instead.
    ///
    /// Underflow is not possible, since $|\csc x| \geq 1$. Overflow requires an input within
    /// $2^{-2^{30}}$ of a nonzero multiple of $\pi$, which takes more than $2^{30}$ bits of
    /// precision, or an input of magnitude about $2^{-2^{30}}$, whose reciprocal alone is beyond
    /// the largest finite [`Float`].
    ///
    /// If you want to specify an output precision, consider using [`Float::csc_prec_round_ref`]
    /// instead. If you know you'll be using the `Nearest` rounding mode, consider using
    /// `(&Float).csc()` instead.
    ///
    /// # Worst-case complexity
    /// $T(n, e) = O(n (\log n)^3 \log\log n + (n+e) (\log (n+e))^2 \log\log (n+e))$
    ///
    /// $M(n, e) = O((n+e) \log (n+e))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits()`, and $e$ is
    /// the exponent of `self` (0 if `self` has no exponent or a negative one): the Taylor series at
    /// working precision $n$, summed by binary splitting for large $n$, costs the first term, and
    /// for $|x| \geq 4$ the argument is reduced modulo $2\pi$, which requires $\pi$ to about $n +
    /// e$ bits. Unlike most functions, `csc` therefore gets slower as the magnitude of its input
    /// grows, not just as the precision does.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact`, since the cosecant of a finite nonzero [`Float`] is never exactly
    /// representable.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = (&Float::from_unsigned_prec(1u32, 100).0).csc_round_ref(Floor);
    /// assert_eq!(c.to_string(), "1.1883951057781212162615994523744");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = (&Float::from_unsigned_prec(1u32, 100).0).csc_round_ref(Ceiling);
    /// assert_eq!(c.to_string(), "1.1883951057781212162615994523760");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = (&Float::from_unsigned_prec(1u32, 100).0).csc_round_ref(Nearest);
    /// assert_eq!(c.to_string(), "1.1883951057781212162615994523744");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn csc_round_ref(&self, rm: RoundingMode) -> (Self, Ordering) {
        self.csc_prec_round_ref(self.significant_bits(), rm)
    }

    /// Computes $\csc x$, the cosecant of a [`Float`], rounding the result to the specified
    /// precision and with the specified rounding mode. The [`Float`] is replaced by the result, and
    /// an [`Ordering`] is returned, indicating whether the rounded cosecant is less than, equal to,
    /// or greater than the exact cosecant. Although `NaN`s are not comparable to any [`Float`],
    /// whenever this function sets a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets \csc x+\varepsilon.
    /// $$
    /// - If $x$ is not finite, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x$ is finite and $m$ is not `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2 |\csc
    ///   x|\rfloor-p+1}$.
    /// - If $x$ is finite and $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2 |\csc
    ///   x|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// See the [`Float::csc_prec_round`] documentation for information on special cases and
    /// overflow.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::csc_prec_assign`] instead. If
    /// you know that your target precision is the precision of the input, consider using
    /// [`Float::csc_round_assign`] instead. If both of these things are true, consider using
    /// [`Float::csc_assign`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m, e) = O(n (\log n)^3 \log\log n + (n+m+e) (\log (n+m+e))^2 \log\log (n+m+e))$
    ///
    /// $M(n, m, e) = O((n+m+e) \log (n+m+e))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, $m$ is
    /// `self.significant_bits()`, and $e$ is the exponent of `self` (0 if `self` has no exponent or
    /// a negative one): the cosine at working precision $n$, summed by binary splitting of the
    /// Taylor series for large $n$, and its reciprocal cost the first term, and for $|x| \geq 4$
    /// the argument is reduced modulo $2\pi$, which requires $\pi$ to about $n + e$ bits and a
    /// remainder of the $m$-bit input. Unlike most functions, `csc` therefore gets slower as the
    /// magnitude of its input grows, not just as the precision does.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact`, since the cosecant of a finite nonzero [`Float`] is never exactly
    /// representable, or if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.csc_prec_round_assign(5, Floor), Less);
    /// assert_eq!(x.to_string(), "1.19");
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.csc_prec_round_assign(5, Ceiling), Greater);
    /// assert_eq!(x.to_string(), "1.25");
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.csc_prec_round_assign(5, Nearest), Less);
    /// assert_eq!(x.to_string(), "1.19");
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.csc_prec_round_assign(20, Floor), Less);
    /// assert_eq!(x.to_string(), "1.1883945");
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.csc_prec_round_assign(20, Ceiling), Greater);
    /// assert_eq!(x.to_string(), "1.1883965");
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.csc_prec_round_assign(20, Nearest), Less);
    /// assert_eq!(x.to_string(), "1.1883945");
    /// ```
    #[inline]
    pub fn csc_prec_round_assign(&mut self, prec: u64, rm: RoundingMode) -> Ordering {
        let o;
        (*self, o) = self.csc_prec_round_ref(prec, rm);
        o
    }

    /// Computes $\csc x$, the cosecant of a [`Float`], rounding the result to the nearest value of
    /// the specified precision. The [`Float`] is replaced by the result, and an [`Ordering`] is
    /// returned, indicating whether the rounded cosecant is less than, equal to, or greater than
    /// the exact cosecant. Although `NaN`s are not comparable to any [`Float`], whenever this
    /// function sets a `NaN` it also returns `Equal`.
    ///
    /// If the cosecant is equidistant from two [`Float`]s with the specified precision, the
    /// [`Float`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a
    /// description of the `Nearest` rounding mode.
    ///
    /// $$
    /// x \gets \csc x+\varepsilon.
    /// $$
    /// - If $x$ is not finite, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x$ is finite, then $|\varepsilon| < 2^{\lfloor\log_2 |\csc x|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// See the [`Float::csc_prec`] documentation for information on special cases and overflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::csc_prec_round_assign`] instead. If you know that your target precision is the
    /// precision of the input, consider using [`Float::csc_assign`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m, e) = O(n (\log n)^3 \log\log n + (n+m+e) (\log (n+m+e))^2 \log\log (n+m+e))$
    ///
    /// $M(n, m, e) = O((n+m+e) \log (n+m+e))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, $m$ is
    /// `self.significant_bits()`, and $e$ is the exponent of `self` (0 if `self` has no exponent or
    /// a negative one): the cosine at working precision $n$, summed by binary splitting of the
    /// Taylor series for large $n$, and its reciprocal cost the first term, and for $|x| \geq 4$
    /// the argument is reduced modulo $2\pi$, which requires $\pi$ to about $n + e$ bits and a
    /// remainder of the $m$-bit input. Unlike most functions, `csc` therefore gets slower as the
    /// magnitude of its input grows, not just as the precision does.
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
    /// assert_eq!(x.csc_prec_assign(5), Less);
    /// assert_eq!(x.to_string(), "1.19");
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.csc_prec_assign(20), Less);
    /// assert_eq!(x.to_string(), "1.1883945");
    /// ```
    #[inline]
    pub fn csc_prec_assign(&mut self, prec: u64) -> Ordering {
        self.csc_prec_round_assign(prec, Nearest)
    }

    /// Computes $\csc x$, the cosecant of a [`Float`], rounding the result with the specified
    /// rounding mode. The [`Float`] is replaced by the result, and an [`Ordering`] is returned,
    /// indicating whether the rounded cosecant is less than, equal to, or greater than the exact
    /// cosecant. Although `NaN`s are not comparable to any [`Float`], whenever this function sets a
    /// `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the precision of the input. See [`RoundingMode`] for a
    /// description of the possible rounding modes.
    ///
    /// $$
    /// x \gets \csc x+\varepsilon.
    /// $$
    /// - If $x$ is not finite, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x$ is finite and $m$ is not `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2 |\csc
    ///   x|\rfloor-p+1}$, where $p$ is the precision of the input.
    /// - If $x$ is finite and $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2 |\csc
    ///   x|\rfloor-p}$, where $p$ is the precision of the input.
    ///
    /// If the output has a precision, it is the precision of the input.
    ///
    /// See the [`Float::csc_round`] documentation for information on special cases and overflow.
    ///
    /// If you want to specify an output precision, consider using [`Float::csc_prec_round_assign`]
    /// instead. If you know you'll be using the `Nearest` rounding mode, consider using
    /// [`Float::csc_assign`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, e) = O(n (\log n)^3 \log\log n + (n+e) (\log (n+e))^2 \log\log (n+e))$
    ///
    /// $M(n, e) = O((n+e) \log (n+e))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits()`, and $e$ is
    /// the exponent of `self` (0 if `self` has no exponent or a negative one): the Taylor series at
    /// working precision $n$, summed by binary splitting for large $n$, costs the first term, and
    /// for $|x| \geq 4$ the argument is reduced modulo $2\pi$, which requires $\pi$ to about $n +
    /// e$ bits. Unlike most functions, `csc` therefore gets slower as the magnitude of its input
    /// grows, not just as the precision does.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact`, since the cosecant of a finite nonzero [`Float`] is never exactly
    /// representable.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.csc_round_assign(Floor), Less);
    /// assert_eq!(x.to_string(), "1.1883951057781212162615994523744");
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.csc_round_assign(Ceiling), Greater);
    /// assert_eq!(x.to_string(), "1.1883951057781212162615994523760");
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.csc_round_assign(Nearest), Less);
    /// assert_eq!(x.to_string(), "1.1883951057781212162615994523744");
    /// ```
    #[inline]
    pub fn csc_round_assign(&mut self, rm: RoundingMode) -> Ordering {
        let prec = self.significant_bits();
        self.csc_prec_round_assign(prec, rm)
    }

    /// Computes $\csc x$, the cosecant of a [`Rational`], rounding the result to the specified
    /// precision and with the specified rounding mode and returning the result as a [`Float`]. The
    /// [`Rational`] is taken by value. An [`Ordering`] is also returned, indicating whether the
    /// rounded cosecant is less than, equal to, or greater than the exact cosecant.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \csc x+\varepsilon.
    /// $$
    /// - If $m$ is not `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2 |\csc x|\rfloor-p+1}$.
    /// - If $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2 |\csc x|\rfloor-p}$.
    ///
    /// These bounds do not apply when the result overflows; see below.
    ///
    /// The output has precision `prec`.
    ///
    /// Special cases:
    /// - $f(0,p,m)=\infty$.
    ///
    /// Overflow:
    /// - If $f(x,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$ is
    ///   returned instead.
    /// - If $f(x,p,m)\leq -2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is
    ///   returned instead.
    /// - If $f(x,p,m)\leq -2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`, $-(1-(1/2)^p)2^{2^{30}-1}$
    ///   is returned instead.
    /// - If $0<f(x,p,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,p,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned instead.
    ///
    /// Underflow is not possible, since $|\csc x| \geq 1$. Overflow requires an input within
    /// $2^{-2^{30}}$ of a nonzero multiple of $\pi$, which takes a denominator of more than
    /// $2^{30}$ bits, or an input of magnitude about $2^{-2^{30}}$ or below, whose reciprocal alone
    /// is beyond the largest finite [`Float`].
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::csc_rational_prec`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m, e) = O(n (\log n)^3 \log\log n + (n+m+e) (\log (n+m+e))^2 \log\log (n+m+e))$
    ///
    /// $M(n, m, e) = O((n+m+e) \log (n+m+e))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, $m$ is `x.significant_bits()`,
    /// and $e$ is `x.floor_log_base_2_abs()` (taken as 0 when it is negative or $x = 0$): the input
    /// is rounded to a working precision and its [`Float`] cosine taken there, then reciprocated,
    /// which for $|x| \geq 2$ reduces the argument modulo $2\pi$ and so needs $\pi$ to about $n +
    /// e$ bits.
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
    /// let (c, o) = Float::csc_rational_prec_round(Rational::from_unsigneds(3u8, 5), 5, Floor);
    /// assert_eq!(c.to_string(), "1.75");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::csc_rational_prec_round(Rational::from_unsigneds(3u8, 5), 5, Ceiling);
    /// assert_eq!(c.to_string(), "1.81");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = Float::csc_rational_prec_round(Rational::from_unsigneds(3u8, 5), 20, Floor);
    /// assert_eq!(c.to_string(), "1.7710304");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::csc_rational_prec_round(Rational::from_unsigneds(3u8, 5), 20, Ceiling);
    /// assert_eq!(c.to_string(), "1.7710323");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    #[allow(clippy::needless_pass_by_value)]
    pub fn csc_rational_prec_round(x: Rational, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        Self::csc_rational_prec_round_ref(&x, prec, rm)
    }

    /// Computes $\csc x$, the cosecant of a [`Rational`], rounding the result to the specified
    /// precision and with the specified rounding mode and returning the result as a [`Float`]. The
    /// [`Rational`] is taken by reference. An [`Ordering`] is also returned, indicating whether the
    /// rounded cosecant is less than, equal to, or greater than the exact cosecant.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \csc x+\varepsilon.
    /// $$
    /// - If $m$ is not `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2 |\csc x|\rfloor-p+1}$.
    /// - If $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2 |\csc x|\rfloor-p}$.
    ///
    /// These bounds do not apply when the result overflows.
    ///
    /// The output has precision `prec`.
    ///
    /// Special cases:
    /// - $f(0,p,m)=\infty$.
    ///
    /// See the [`Float::csc_rational_prec_round`] documentation for information on overflow.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::csc_rational_prec_ref`]
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m, e) = O(n (\log n)^3 \log\log n + (n+m+e) (\log (n+m+e))^2 \log\log (n+m+e))$
    ///
    /// $M(n, m, e) = O((n+m+e) \log (n+m+e))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, $m$ is `x.significant_bits()`,
    /// and $e$ is `x.floor_log_base_2_abs()` (taken as 0 when it is negative or $x = 0$): the input
    /// is rounded to a working precision and its [`Float`] cosine taken there, then reciprocated,
    /// which for $|x| \geq 2$ reduces the argument modulo $2\pi$ and so needs $\pi$ to about $n +
    /// e$ bits.
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
    /// let (c, o) =
    ///     Float::csc_rational_prec_round_ref(&Rational::from_unsigneds(3u8, 5), 5, Floor);
    /// assert_eq!(c.to_string(), "1.75");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) =
    ///     Float::csc_rational_prec_round_ref(&Rational::from_unsigneds(3u8, 5), 5, Ceiling);
    /// assert_eq!(c.to_string(), "1.81");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) =
    ///     Float::csc_rational_prec_round_ref(&Rational::from_unsigneds(3u8, 5), 20, Floor);
    /// assert_eq!(c.to_string(), "1.7710304");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) =
    ///     Float::csc_rational_prec_round_ref(&Rational::from_unsigneds(3u8, 5), 20, Ceiling);
    /// assert_eq!(c.to_string(), "1.7710323");
    /// assert_eq!(o, Greater);
    /// ```
    pub fn csc_rational_prec_round_ref(
        x: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        assert_ne!(prec, 0);
        if *x == 0u32 {
            // csc(0) = infinity; a `Rational` zero has no sign, so the result is positive
            return (Self::INFINITY, Equal);
        }
        csc_rational_helper(x, prec, rm)
    }

    /// Computes $\csc x$, the cosecant of a [`Rational`], rounding the result to the nearest value
    /// of the specified precision and returning the result as a [`Float`]. The [`Rational`] is
    /// taken by value. An [`Ordering`] is also returned, indicating whether the rounded cosecant is
    /// less than, equal to, or greater than the exact cosecant.
    ///
    /// If the cosecant is equidistant from two [`Float`]s with the specified precision, the
    /// [`Float`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a
    /// description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = \csc x+\varepsilon,
    /// $$
    /// where $|\varepsilon| \leq 2^{\lfloor\log_2 |\csc x|\rfloor-p}$ (unless the result overflows;
    /// see below).
    ///
    /// The output has precision `prec`.
    ///
    /// Special cases:
    /// - $f(0,p)=\infty$.
    ///
    /// Overflow:
    /// - If $f(x,p)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,p)\leq -2^{2^{30}-1}$, $-\infty$ is returned instead.
    /// - If $0<f(x,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,p)<0$, $-0.0$ is returned instead.
    ///
    /// Underflow is not possible, since $|\csc x| \geq 1$. Overflow requires an input within
    /// $2^{-2^{30}}$ of a nonzero multiple of $\pi$, which takes a denominator of more than
    /// $2^{30}$ bits, or an input of magnitude about $2^{-2^{30}}$ or below, whose reciprocal alone
    /// is beyond the largest finite [`Float`].
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::csc_rational_prec_round`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m, e) = O(n (\log n)^3 \log\log n + (n+m+e) (\log (n+m+e))^2 \log\log (n+m+e))$
    ///
    /// $M(n, m, e) = O((n+m+e) \log (n+m+e))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, $m$ is `x.significant_bits()`,
    /// and $e$ is `x.floor_log_base_2_abs()` (taken as 0 when it is negative or $x = 0$): the input
    /// is rounded to a working precision and its [`Float`] cosine taken there, then reciprocated,
    /// which for $|x| \geq 2$ reduces the argument modulo $2\pi$ and so needs $\pi$ to about $n +
    /// e$ bits.
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
    /// let (c, o) = Float::csc_rational_prec(Rational::from_unsigneds(3u8, 5), 5);
    /// assert_eq!(c.to_string(), "1.75");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::csc_rational_prec(Rational::from_unsigneds(3u8, 5), 20);
    /// assert_eq!(c.to_string(), "1.7710323");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    #[allow(clippy::needless_pass_by_value)]
    pub fn csc_rational_prec(x: Rational, prec: u64) -> (Self, Ordering) {
        Self::csc_rational_prec_round_ref(&x, prec, Nearest)
    }

    /// Computes $\csc x$, the cosecant of a [`Rational`], rounding the result to the nearest value
    /// of the specified precision and returning the result as a [`Float`]. The [`Rational`] is
    /// taken by reference. An [`Ordering`] is also returned, indicating whether the rounded
    /// cosecant is less than, equal to, or greater than the exact cosecant.
    ///
    /// If the cosecant is equidistant from two [`Float`]s with the specified precision, the
    /// [`Float`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a
    /// description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = \csc x+\varepsilon,
    /// $$
    /// where $|\varepsilon| \leq 2^{\lfloor\log_2 |\csc x|\rfloor-p}$ (unless the result
    /// overflows).
    ///
    /// The output has precision `prec`.
    ///
    /// Special cases:
    /// - $f(0,p)=\infty$.
    ///
    /// See the [`Float::csc_rational_prec`] documentation for information on overflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::csc_rational_prec_round_ref`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m, e) = O(n (\log n)^3 \log\log n + (n+m+e) (\log (n+m+e))^2 \log\log (n+m+e))$
    ///
    /// $M(n, m, e) = O((n+m+e) \log (n+m+e))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, $m$ is `x.significant_bits()`,
    /// and $e$ is `x.floor_log_base_2_abs()` (taken as 0 when it is negative or $x = 0$): the input
    /// is rounded to a working precision and its [`Float`] cosine taken there, then reciprocated,
    /// which for $|x| \geq 2$ reduces the argument modulo $2\pi$ and so needs $\pi$ to about $n +
    /// e$ bits.
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
    /// let (c, o) = Float::csc_rational_prec_ref(&Rational::from_unsigneds(3u8, 5), 5);
    /// assert_eq!(c.to_string(), "1.75");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::csc_rational_prec_ref(&Rational::from_unsigneds(3u8, 5), 20);
    /// assert_eq!(c.to_string(), "1.7710323");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn csc_rational_prec_ref(x: &Rational, prec: u64) -> (Self, Ordering) {
        Self::csc_rational_prec_round_ref(x, prec, Nearest)
    }

    /// Computes $\csc(2\pi x/u)$, the cosecant of a [`Float`] measured in $u$ths of a turn,
    /// rounding the result to the specified precision and with the specified rounding mode. The
    /// [`Float`] is taken by value. An [`Ordering`] is also returned, indicating whether the
    /// rounded cosecant is less than, equal to, or greater than the exact cosecant. Although `NaN`s
    /// are not comparable to any [`Float`], whenever this function returns a `NaN` it also returns
    /// `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,u,p,m) = \csc(2\pi x/u)+\varepsilon.
    /// $$
    /// - If $x$ is not finite, $u=0$, or $x/u$ is a multiple of $1/4$ or has denominator 12 in
    ///   lowest terms, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x$ is finite, $u\neq 0$, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\csc(2\pi x/u)|\rfloor-p+1}$.
    /// - If $x$ is finite, $u\neq 0$, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\csc(2\pi x/u)|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},u,p,m)=\text{NaN}$
    /// - $f(\pm\infty,u,p,m)=\text{NaN}$
    /// - $f(x,0,p,m)=\text{NaN}$
    /// - $f(\pm0.0,u,p,m)=\pm\infty$
    /// - If $x/u$ is a multiple of $1/2$, the cosecant has a pole there, and the result is exactly
    ///   $\pm\infty$ with the sign of $x$: the sine is a zero carrying that sign, and the cosecant
    ///   is its reciprocal, which keeps the function odd.
    /// - If $x/u$ in lowest terms has denominator 4, the result is exactly $\pm1$, and if it has
    ///   denominator 12, exactly $\pm2$.
    ///
    /// When $x/u$ in lowest terms has denominator 3 or 6, the result is $\pm2\sqrt3/3$; when it has
    /// denominator 8, $\pm\sqrt2$; and when it has denominator 20, $\pm2\varphi$ or
    /// $\pm2(\varphi-1)$, where $\varphi$ is the golden ratio. Each is computed from a single
    /// correctly rounded constant rather than from $\pi$ and a sine, which is far faster.
    ///
    /// Overflow:
    /// - If $f(x,u,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,u,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$
    ///   is returned instead.
    /// - If $f(x,u,p,m)\leq -2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is
    ///   returned instead.
    /// - If $f(x,u,p,m)\leq -2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`,
    ///   $-(1-(1/2)^p)2^{2^{30}-1}$ is returned instead.
    /// - If $0<f(x,u,p,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,u,p,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned instead.
    ///
    /// Underflow is not possible, since $|\csc(2\pi x/u)| \geq 1$. Overflow requires $x/u$ within
    /// $2^{-2^{30}}$ of a multiple of $1/2$ without being one, which takes more than $2^{30}$ bits
    /// of precision, or an $x/u$ so small that $2\pi x/u$ is below $2^{-2^{30}}$.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::csc_with_period_prec`]
    /// instead. If you know that your target precision is the precision of the input, consider
    /// using [`Float::csc_with_period_round`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m, e) = O(n (\log n)^3 \log\log n + (n+m+e) (\log (n+m+e))^2 \log\log (n+m+e))$
    ///
    /// $M(n, m, e) = O((n+m+e) \log (n+m+e))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, $m$ is
    /// `self.significant_bits()`, and $e$ is the exponent of `self` (0 if `self` has no exponent or
    /// a negative one): the argument is reduced modulo $u$ exactly, and the sine of $2\pi x/u$ is
    /// then taken at a working precision of about $n + e$ bits, which needs $\pi$ to that many
    /// bits, and reciprocated.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with the given precision (which is the case unless $x/u$ is a multiple of $1/4$ or has
    /// denominator 12 in lowest terms, or $x$ is zero or not finite, or $u$ is zero).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (t, o) = Float::ONE.csc_with_period_prec_round(7, 10, Floor);
    /// assert_eq!(t.to_string(), "1.2773");
    /// assert_eq!(o, Less);
    ///
    /// let (t, o) = Float::ONE.csc_with_period_prec_round(7, 10, Ceiling);
    /// assert_eq!(t.to_string(), "1.2793");
    /// assert_eq!(o, Greater);
    ///
    /// // a quarter turn is exactly 1
    /// let (t, o) = Float::from(90u32).csc_with_period_prec_round(360, 10, Exact);
    /// assert_eq!(t.to_string(), "1.0000");
    /// assert_eq!(o, Equal);
    ///
    /// // a half turn is a pole
    /// let (t, o) = Float::from(180u32).csc_with_period_prec_round(360, 10, Exact);
    /// assert_eq!(t.to_string(), "Infinity");
    /// assert_eq!(o, Equal);
    ///
    /// // a twelfth of a turn is exactly 2
    /// let (t, o) = Float::from(30u32).csc_with_period_prec_round(360, 10, Nearest);
    /// assert_eq!(t.to_string(), "2.0000");
    /// assert_eq!(o, Equal);
    /// ```
    #[inline]
    pub fn csc_with_period_prec_round(
        self,
        u: u64,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        self.csc_with_period_prec_round_ref(u, prec, rm)
    }

    /// Computes $\csc(2\pi x/u)$, the cosecant of a [`Float`] measured in $u$ths of a turn,
    /// rounding the result to the specified precision and with the specified rounding mode. The
    /// [`Float`] is taken by reference. An [`Ordering`] is also returned, indicating whether the
    /// rounded cosecant is less than, equal to, or greater than the exact cosecant. Although `NaN`s
    /// are not comparable to any [`Float`], whenever this function returns a `NaN` it also returns
    /// `Equal`.
    ///
    /// See [`Float::csc_with_period_prec_round`] for the error bounds, the special and closed-form
    /// cases, overflow, and the complexity; this function behaves the same way.
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
    /// use std::cmp::Ordering::*;
    ///
    /// let (t, o) = Float::ONE.csc_with_period_prec_round_ref(7, 10, Floor);
    /// assert_eq!(t.to_string(), "1.2773");
    /// assert_eq!(o, Less);
    /// ```
    pub fn csc_with_period_prec_round_ref(
        &self,
        u: u64,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        assert_ne!(prec, 0);
        match &self.0 {
            // for u=0, return NaN
            _ if u == 0 => (Self::NAN, Equal),
            NaN | Infinity { .. } => (Self::NAN, Equal),
            // x is zero: csc(±0) = ±infinity
            Zero { .. } => (
                if self.is_sign_negative() {
                    Self::NEGATIVE_INFINITY
                } else {
                    Self::INFINITY
                },
                Equal,
            ),
            Finite { .. } => csc_with_period_prec_round_normal_ref(self, u, prec, rm),
        }
    }

    /// Computes $\csc(2\pi x/u)$, the cosecant of a [`Float`] measured in $u$ths of a turn,
    /// rounding the result to the nearest value of the specified precision. The [`Float`] is taken
    /// by value. An [`Ordering`] is also returned, indicating whether the rounded cosecant is less
    /// than, equal to, or greater than the exact cosecant. Although `NaN`s are not comparable to
    /// any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// If the cosecant is equidistant from two [`Float`]s with the specified precision, the
    /// [`Float`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a
    /// description of the `Nearest` rounding mode.
    ///
    /// See [`Float::csc_with_period_prec_round`] for the error bounds, the special and closed-form
    /// cases, overflow, and the complexity; this function behaves the same way with `Nearest`.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::csc_with_period_prec_round`] instead.
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
    /// let (t, o) = Float::ONE.csc_with_period_prec(7, 10);
    /// assert_eq!(t.to_string(), "1.2793");
    /// assert_eq!(o, Greater);
    ///
    /// // an eighth of a turn: sqrt(2)
    /// let (t, o) = Float::ONE.csc_with_period_prec(8, 10);
    /// assert_eq!(t.to_string(), "1.4141");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn csc_with_period_prec(self, u: u64, prec: u64) -> (Self, Ordering) {
        self.csc_with_period_prec_round(u, prec, Nearest)
    }

    /// Computes $\csc(2\pi x/u)$, the cosecant of a [`Float`] measured in $u$ths of a turn,
    /// rounding the result to the nearest value of the specified precision. The [`Float`] is taken
    /// by reference. An [`Ordering`] is also returned, indicating whether the rounded cosecant is
    /// less than, equal to, or greater than the exact cosecant. Although `NaN`s are not comparable
    /// to any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`Float::csc_with_period_prec`] and [`Float::csc_with_period_prec_round`]; this function
    /// behaves the same way.
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
    /// let (t, o) = Float::ONE.csc_with_period_prec_ref(7, 10);
    /// assert_eq!(t.to_string(), "1.2793");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn csc_with_period_prec_ref(&self, u: u64, prec: u64) -> (Self, Ordering) {
        self.csc_with_period_prec_round_ref(u, prec, Nearest)
    }

    /// Computes $\csc(2\pi x/u)$, the cosecant of a [`Float`] measured in $u$ths of a turn,
    /// rounding the result to the precision of the input and with the specified rounding mode. The
    /// [`Float`] is taken by value. An [`Ordering`] is also returned, indicating whether the
    /// rounded cosecant is less than, equal to, or greater than the exact cosecant. Although `NaN`s
    /// are not comparable to any [`Float`], whenever this function returns a `NaN` it also returns
    /// `Equal`.
    ///
    /// See [`Float::csc_with_period_prec_round`] for the error bounds, the special and closed-form
    /// cases, overflow, and the complexity; this function behaves the same way with `prec` equal to
    /// the precision of the input.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::csc_with_period_prec_round`] instead.
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
    /// let (t, o) = Float::from_unsigned_prec(1u32, 10)
    ///     .0
    ///     .csc_with_period_round(7, Floor);
    /// assert_eq!(t.to_string(), "1.2773");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn csc_with_period_round(self, u: u64, rm: RoundingMode) -> (Self, Ordering) {
        let prec = self.significant_bits();
        self.csc_with_period_prec_round(u, prec, rm)
    }

    /// Computes $\csc(2\pi x/u)$, the cosecant of a [`Float`] measured in $u$ths of a turn,
    /// rounding the result to the precision of the input and with the specified rounding mode. The
    /// [`Float`] is taken by reference. An [`Ordering`] is also returned, indicating whether the
    /// rounded cosecant is less than, equal to, or greater than the exact cosecant. Although `NaN`s
    /// are not comparable to any [`Float`], whenever this function returns a `NaN` it also returns
    /// `Equal`.
    ///
    /// See [`Float::csc_with_period_round`] and [`Float::csc_with_period_prec_round`]; this
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
    /// let (t, o) = Float::from_unsigned_prec(1u32, 10)
    ///     .0
    ///     .csc_with_period_round_ref(7, Floor);
    /// assert_eq!(t.to_string(), "1.2773");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn csc_with_period_round_ref(&self, u: u64, rm: RoundingMode) -> (Self, Ordering) {
        self.csc_with_period_prec_round_ref(u, self.significant_bits(), rm)
    }

    /// Computes $\csc(2\pi x/u)$, the cosecant of a [`Float`] measured in $u$ths of a turn (so that
    /// `u = 360` is degrees), rounding the result to the precision of the input and to the nearest
    /// [`Float`]. The [`Float`] is taken by value.
    ///
    /// If the cosecant is equidistant from two [`Float`]s with the precision of the input, the
    /// [`Float`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a
    /// description of the `Nearest` rounding mode.
    ///
    /// See [`Float::csc_with_period_prec_round`] for the error bounds, the special and closed-form
    /// cases, overflow, and the complexity; this function behaves the same way with `prec` equal to
    /// the precision of the input and `rm` equal to `Nearest`.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::csc_with_period_round`] instead. If you want to specify an output precision,
    /// consider using [`Float::csc_with_period_prec`]. If you want both of these things, consider
    /// using [`Float::csc_with_period_prec_round`].
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    ///
    /// let t = Float::from_unsigned_prec(1u32, 10).0.csc_with_period(7);
    /// assert_eq!(t.to_string(), "1.2793");
    ///
    /// // a quarter turn is exactly 1
    /// assert_eq!(Float::from(90u32).csc_with_period(360).to_string(), "1.00");
    /// ```
    #[inline]
    pub fn csc_with_period(self, u: u64) -> Self {
        let prec = self.significant_bits();
        self.csc_with_period_prec(u, prec).0
    }

    /// Computes $\csc(2\pi x/u)$, the cosecant of a [`Float`] measured in $u$ths of a turn (so that
    /// `u = 360` is degrees), rounding the result to the precision of the input and to the nearest
    /// [`Float`]. The [`Float`] is taken by reference.
    ///
    /// If the cosecant is equidistant from two [`Float`]s with the precision of the input, the
    /// [`Float`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a
    /// description of the `Nearest` rounding mode.
    ///
    /// See [`Float::csc_with_period_prec_round`] for the error bounds, the special and closed-form
    /// cases, overflow, and the complexity; this function behaves the same way with `prec` equal to
    /// the precision of the input and `rm` equal to `Nearest`.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::csc_with_period_round_ref`] instead. If you want to specify an output precision,
    /// consider using [`Float::csc_with_period_prec_ref`]. If you want both of these things,
    /// consider using [`Float::csc_with_period_prec_round_ref`].
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    ///
    /// let t = (&Float::from_unsigned_prec(1u32, 10).0).csc_with_period_ref(7);
    /// assert_eq!(t.to_string(), "1.2793");
    /// ```
    #[inline]
    pub fn csc_with_period_ref(&self, u: u64) -> Self {
        self.csc_with_period_prec_ref(u, self.significant_bits()).0
    }

    /// Replaces a [`Float`] measured in $u$ths of a turn with its cosecant, rounding the result to
    /// the specified precision and with the specified rounding mode. An [`Ordering`] is returned,
    /// indicating whether the rounded cosecant is less than, equal to, or greater than the exact
    /// cosecant. Although `NaN`s are not comparable to any [`Float`], whenever this function sets a
    /// `NaN` it also returns `Equal`.
    ///
    /// See [`Float::csc_with_period_prec_round`] for the error bounds, the special and closed-form
    /// cases, overflow, and the complexity; this function behaves the same way.
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
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::ONE;
    /// assert_eq!(x.csc_with_period_prec_round_assign(7, 10, Floor), Less);
    /// assert_eq!(x.to_string(), "1.2773");
    /// ```
    #[inline]
    pub fn csc_with_period_prec_round_assign(
        &mut self,
        u: u64,
        prec: u64,
        rm: RoundingMode,
    ) -> Ordering {
        let (t, o) = self.csc_with_period_prec_round_ref(u, prec, rm);
        *self = t;
        o
    }

    /// Replaces a [`Float`] measured in $u$ths of a turn with its cosecant, rounding the result to
    /// the nearest value of the specified precision. An [`Ordering`] is returned, indicating
    /// whether the rounded cosecant is less than, equal to, or greater than the exact cosecant.
    /// Although `NaN`s are not comparable to any [`Float`], whenever this function sets a `NaN` it
    /// also returns `Equal`.
    ///
    /// See [`Float::csc_with_period_prec`] and [`Float::csc_with_period_prec_round`]; this function
    /// behaves the same way.
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
    /// let mut x = Float::ONE;
    /// assert_eq!(x.csc_with_period_prec_assign(7, 10), Greater);
    /// assert_eq!(x.to_string(), "1.2793");
    /// ```
    #[inline]
    pub fn csc_with_period_prec_assign(&mut self, u: u64, prec: u64) -> Ordering {
        self.csc_with_period_prec_round_assign(u, prec, Nearest)
    }

    /// Replaces a [`Float`] measured in $u$ths of a turn with its cosecant, rounding the result to
    /// the precision of the input and with the specified rounding mode. An [`Ordering`] is
    /// returned, indicating whether the rounded cosecant is less than, equal to, or greater than
    /// the exact cosecant. Although `NaN`s are not comparable to any [`Float`], whenever this
    /// function sets a `NaN` it also returns `Equal`.
    ///
    /// See [`Float::csc_with_period_round`] and [`Float::csc_with_period_prec_round`]; this
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
    /// let mut x = Float::from_unsigned_prec(1u32, 10).0;
    /// assert_eq!(x.csc_with_period_round_assign(7, Floor), Less);
    /// assert_eq!(x.to_string(), "1.2773");
    /// ```
    #[inline]
    pub fn csc_with_period_round_assign(&mut self, u: u64, rm: RoundingMode) -> Ordering {
        let prec = self.significant_bits();
        self.csc_with_period_prec_round_assign(u, prec, rm)
    }

    /// Computes $\csc(2\pi x/u)$, the cosecant of a [`Float`] measured in $u$ths of a turn (so that
    /// `u = 360` is degrees), rounding the result to the precision of the input and to the nearest
    /// [`Float`]. The [`Float`] is replaced by the result.
    ///
    /// If the cosecant is equidistant from two [`Float`]s with the precision of the input, the
    /// [`Float`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a
    /// description of the `Nearest` rounding mode.
    ///
    /// See [`Float::csc_with_period_prec_round`] for the error bounds, the special and closed-form
    /// cases, overflow, and the complexity; this function behaves the same way with `prec` equal to
    /// the precision of the input and `rm` equal to `Nearest`.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::csc_with_period_round_assign`] instead. If you want to specify an output precision,
    /// consider using [`Float::csc_with_period_prec_assign`]. If you want both of these things,
    /// consider using [`Float::csc_with_period_prec_round_assign`].
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 10).0;
    /// x.csc_with_period_assign(7);
    /// assert_eq!(x.to_string(), "1.2793");
    /// ```
    #[inline]
    pub fn csc_with_period_assign(&mut self, u: u64) {
        let prec = self.significant_bits();
        self.csc_with_period_prec_assign(u, prec);
    }
}

impl Csc for Float {
    type Output = Self;

    /// Computes $\csc x$, the cosecant of a [`Float`], taking it by value.
    ///
    /// If the output has a precision, it is the precision of the input. If the cosecant is
    /// equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s in
    /// its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// $$
    /// f(x) = \csc x+\varepsilon.
    /// $$
    /// - If $x$ is not finite, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x$ is finite, then $|\varepsilon| < 2^{\lfloor\log_2 |\csc x|\rfloor-p}$, where $p$ is
    ///   the precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN})=\text{NaN}$
    /// - $f(\pm\infty)=\text{NaN}$
    /// - $f(\pm0.0)=\pm\infty$
    ///
    /// See the [`Float::csc_round`] documentation for information on overflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using [`Float::csc_round`]
    /// instead. If you want to specify the output precision, consider using [`Float::csc_prec`]. If
    /// you want both of these things, consider using [`Float::csc_prec_round`].
    ///
    /// # Worst-case complexity
    /// $T(n, e) = O(n (\log n)^3 \log\log n + (n+e) (\log (n+e))^2 \log\log (n+e))$
    ///
    /// $M(n, e) = O((n+e) \log (n+e))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits()`, and $e$ is
    /// the exponent of `self` (0 if `self` has no exponent or a negative one): the Taylor series at
    /// working precision $n$, summed by binary splitting for large $n$, costs the first term, and
    /// for $|x| \geq 4$ the argument is reduced modulo $2\pi$, which requires $\pi$ to about $n +
    /// e$ bits. Unlike most functions, `csc` therefore gets slower as the magnitude of its input
    /// grows, not just as the precision does.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Csc;
    /// use malachite_base::num::basic::traits::*;
    /// use malachite_float::Float;
    ///
    /// assert!(Float::NAN.csc().is_nan());
    /// assert!(Float::INFINITY.csc().is_nan());
    /// assert!(Float::NEGATIVE_INFINITY.csc().is_nan());
    /// assert_eq!(Float::ZERO.csc().to_string(), "Infinity");
    /// assert_eq!(Float::NEGATIVE_ZERO.csc().to_string(), "-Infinity");
    /// assert_eq!(
    ///     Float::from_unsigned_prec(1u32, 100).0.csc().to_string(),
    ///     "1.1883951057781212162615994523744"
    /// );
    /// assert_eq!(
    ///     Float::from_unsigned_prec(100u32, 100).0.csc().to_string(),
    ///     "-1.9748575314240999612122645488016"
    /// );
    /// ```
    #[inline]
    fn csc(self) -> Self {
        let prec = self.significant_bits();
        self.csc_prec_round(prec, Nearest).0
    }
}

impl Csc for &Float {
    type Output = Float;

    /// Computes $\csc x$, the cosecant of a [`Float`], taking it by reference.
    ///
    /// If the output has a precision, it is the precision of the input. If the cosecant is
    /// equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s in
    /// its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// $$
    /// f(x) = \csc x+\varepsilon.
    /// $$
    /// - If $x$ is not finite, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x$ is finite, then $|\varepsilon| < 2^{\lfloor\log_2 |\csc x|\rfloor-p}$, where $p$ is
    ///   the precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN})=\text{NaN}$
    /// - $f(\pm\infty)=\text{NaN}$
    /// - $f(\pm0.0)=\pm\infty$
    ///
    /// See the [`Float::csc_round`] documentation for information on overflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::csc_round_ref`] instead. If you want to specify the output precision, consider
    /// using [`Float::csc_prec_ref`]. If you want both of these things, consider using
    /// [`Float::csc_prec_round_ref`].
    ///
    /// # Worst-case complexity
    /// $T(n, e) = O(n (\log n)^3 \log\log n + (n+e) (\log (n+e))^2 \log\log (n+e))$
    ///
    /// $M(n, e) = O((n+e) \log (n+e))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits()`, and $e$ is
    /// the exponent of `self` (0 if `self` has no exponent or a negative one): the Taylor series at
    /// working precision $n$, summed by binary splitting for large $n$, costs the first term, and
    /// for $|x| \geq 4$ the argument is reduced modulo $2\pi$, which requires $\pi$ to about $n +
    /// e$ bits. Unlike most functions, `csc` therefore gets slower as the magnitude of its input
    /// grows, not just as the precision does.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Csc;
    /// use malachite_base::num::basic::traits::*;
    /// use malachite_float::Float;
    ///
    /// assert!(Float::NAN.csc().is_nan());
    /// assert!(Float::INFINITY.csc().is_nan());
    /// assert!(Float::NEGATIVE_INFINITY.csc().is_nan());
    /// assert_eq!(Float::ZERO.csc().to_string(), "Infinity");
    /// assert_eq!(Float::NEGATIVE_ZERO.csc().to_string(), "-Infinity");
    /// assert_eq!(
    ///     (&Float::from_unsigned_prec(1u32, 100).0).csc().to_string(),
    ///     "1.1883951057781212162615994523744"
    /// );
    /// assert_eq!(
    ///     (&Float::from_unsigned_prec(100u32, 100).0)
    ///         .csc()
    ///         .to_string(),
    ///     "-1.9748575314240999612122645488016"
    /// );
    /// ```
    #[inline]
    fn csc(self) -> Float {
        self.csc_prec_round_ref(self.significant_bits(), Nearest).0
    }
}

impl CscAssign for Float {
    /// Computes $\csc x$, the cosecant of a [`Float`], in place.
    ///
    /// If the output has a precision, it is the precision of the input. If the cosecant is
    /// equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s in
    /// its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// $$
    /// x \gets \csc x+\varepsilon.
    /// $$
    /// - If $x$ is not finite, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x$ is finite, then $|\varepsilon| < 2^{\lfloor\log_2 |\csc x|\rfloor-p}$, where $p$ is
    ///   the precision of the input.
    ///
    /// See the [`Float::csc`] documentation for information on special cases and overflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::csc_round_assign`] instead. If you want to specify the output precision, consider
    /// using [`Float::csc_prec_assign`]. If you want both of these things, consider using
    /// [`Float::csc_prec_round_assign`].
    ///
    /// # Worst-case complexity
    /// $T(n, e) = O(n (\log n)^3 \log\log n + (n+e) (\log (n+e))^2 \log\log (n+e))$
    ///
    /// $M(n, e) = O((n+e) \log (n+e))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits()`, and $e$ is
    /// the exponent of `self` (0 if `self` has no exponent or a negative one): the Taylor series at
    /// working precision $n$, summed by binary splitting for large $n$, costs the first term, and
    /// for $|x| \geq 4$ the argument is reduced modulo $2\pi$, which requires $\pi$ to about $n +
    /// e$ bits. Unlike most functions, `csc` therefore gets slower as the magnitude of its input
    /// grows, not just as the precision does.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::CscAssign;
    /// use malachite_base::num::basic::traits::*;
    /// use malachite_float::Float;
    ///
    /// let mut x = Float::NAN;
    /// x.csc_assign();
    /// assert!(x.is_nan());
    ///
    /// let mut x = Float::INFINITY;
    /// x.csc_assign();
    /// assert!(x.is_nan());
    ///
    /// let mut x = Float::NEGATIVE_INFINITY;
    /// x.csc_assign();
    /// assert!(x.is_nan());
    ///
    /// let mut x = Float::ZERO;
    /// x.csc_assign();
    /// assert_eq!(x.to_string(), "Infinity");
    ///
    /// let mut x = Float::NEGATIVE_ZERO;
    /// x.csc_assign();
    /// assert_eq!(x.to_string(), "-Infinity");
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// x.csc_assign();
    /// assert_eq!(x.to_string(), "1.1883951057781212162615994523744");
    ///
    /// let mut x = Float::from_unsigned_prec(100u32, 100).0;
    /// x.csc_assign();
    /// assert_eq!(x.to_string(), "-1.9748575314240999612122645488016");
    /// ```
    #[inline]
    fn csc_assign(&mut self) {
        let prec = self.significant_bits();
        self.csc_prec_round_assign(prec, Nearest);
    }
}

/// Computes $\csc x$, the cosecant of a primitive float, correctly rounded. Neither the standard
/// library nor `libm` provides a cosecant.
///
/// $$
/// f(x) = \csc x+\varepsilon.
/// $$
/// - If $x$ is not finite, $\varepsilon$ may be ignored or assumed to be 0.
/// - If $x$ is finite, then $|\varepsilon| < 2^{\lfloor\log_2 |\csc x|\rfloor-p}$, where $p$ is the
///   precision of the output (24 if `T` is a [`f32`] and 53 if `T` is a [`f64`]).
///
/// Special cases:
/// - $f(\text{NaN})=\text{NaN}$
/// - $f(\pm\infty)=\text{NaN}$
/// - $f(\pm0.0)=\pm\infty$
///
/// Overflow is possible: the cosecant of a tiny $x$ is close to $1/x$, so an $x$ with $|x|$ below
/// about $2^{-128}$ has a cosecant beyond the largest [`f32`], and one below about $2^{-1024}$
/// beyond the largest [`f64`]; the result is then $\pm\infty$. No [`f32`] or [`f64`] is close
/// enough to a nonzero multiple of $\pi$ for its cosecant to overflow, and the result is never
/// subnormal, since $|\csc x| \geq 1$.
///
/// # Worst-case complexity
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use malachite_base::num::basic::traits::NegativeInfinity;
/// use malachite_base::num::float::NiceFloat;
/// use malachite_float::float::arithmetic::csc::primitive_float_csc;
///
/// assert!(primitive_float_csc(f32::NAN).is_nan());
/// assert!(primitive_float_csc(f32::INFINITY).is_nan());
/// assert!(primitive_float_csc(f32::NEGATIVE_INFINITY).is_nan());
/// assert_eq!(
///     NiceFloat(primitive_float_csc(0.0f32)),
///     NiceFloat(f32::INFINITY)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_csc(-0.0f32)),
///     NiceFloat(f32::NEGATIVE_INFINITY)
/// );
/// assert_eq!(NiceFloat(primitive_float_csc(1.0f32)), NiceFloat(1.1883951));
/// assert_eq!(
///     NiceFloat(primitive_float_csc(1.0f64)),
///     NiceFloat(1.1883951057781212)
/// );
/// ```
#[inline]
#[allow(clippy::type_repetition_in_bounds)]
pub fn primitive_float_csc<T: PrimitiveFloat>(x: T) -> T
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    emulate_float_to_float_fn(Float::csc_prec, x)
}

/// Computes $\csc x$, the cosecant of a [`Rational`], returning the result as a primitive float.
///
/// $$
/// f(x) = \csc x+\varepsilon,
/// $$
/// where $|\varepsilon| < 2^{\lfloor\log_2 |\csc x|\rfloor-p}$, and $p$ is the precision of the
/// output (24 if `T` is a [`f32`] and 53 if `T` is a [`f64`]).
///
/// Special cases:
/// - $f(0)=\infty$
///
/// Overflow is possible: a [`Rational`] within about $2^{-129}$ of a nonzero multiple of $\pi$ has
/// a cosecant beyond the largest [`f32`], and one within about $2^{-1025}$ of one beyond the
/// largest [`f64`]; so does any [`Rational`] small enough that its reciprocal alone leaves the
/// range, and $0$ itself, whose cosecant is $\infty$. Underflow is not possible, since $|\csc x|
/// \geq 1$.
///
/// # Worst-case complexity
/// $T(m, e) = O((m+e) (\log (m+e))^2 \log\log (m+e))$
///
/// $M(m, e) = O((m+e) \log (m+e))$
///
/// where $T$ is time, $M$ is additional memory, $m$ is `x.significant_bits()`, and $e$ is
/// `x.floor_log_base_2_abs()` (taken as 0 when it is negative or $x = 0$): for $|x| \geq 3$ the
/// argument is reduced modulo $2\pi$, which needs $\pi$ to about $e$ bits.
///
/// # Examples
/// ```
/// use malachite_base::num::basic::traits::Zero;
/// use malachite_base::num::float::NiceFloat;
/// use malachite_float::float::arithmetic::csc::primitive_float_csc_rational;
/// use malachite_q::Rational;
///
/// assert_eq!(
///     NiceFloat(primitive_float_csc_rational::<f64>(&Rational::ZERO)),
///     NiceFloat(f64::INFINITY)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_csc_rational::<f64>(
///         &Rational::from_unsigneds(1u8, 3)
///     )),
///     NiceFloat(3.0562842545795195)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_csc_rational::<f32>(
///         &Rational::from_unsigneds(1u8, 3)
///     )),
///     NiceFloat(3.0562842)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_csc_rational::<f64>(&Rational::from(10000))),
///     NiceFloat(-3.2720972452826818)
/// );
/// ```
#[inline]
#[allow(clippy::type_repetition_in_bounds)]
pub fn primitive_float_csc_rational<T: PrimitiveFloat>(x: &Rational) -> T
where
    Float: PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    emulate_rational_to_float_fn(Float::csc_rational_prec_ref, x)
}

/// Computes $\csc(2\pi x/u)$, the cosecant of a primitive float measured in $u$ths of a turn (so
/// that `u = 360` is degrees).
///
/// $$
/// f(x,u) = \csc(2\pi x/u)+\varepsilon.
/// $$
/// - If $x$ is not finite, $u=0$, or $x/u$ is a multiple of $1/4$ or has denominator 12 in lowest
///   terms, $\varepsilon$ may be ignored or assumed to be 0.
/// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2 |\csc(2\pi x/u)|\rfloor-p}$, where $p$ is the
///   precision of the output (24 if `T` is a [`f32`] and 53 if `T` is a [`f64`]).
///
/// Special cases:
/// - $f(\text{NaN},u)=\text{NaN}$
/// - $f(\pm\infty,u)=\text{NaN}$
/// - $f(x,0)=\text{NaN}$
/// - $f(\pm0.0,u)=\pm\infty$
/// - If $x/u$ is a multiple of $1/2$, the cosecant has a pole there, and the result is exactly
///   $\pm\infty$ with the sign of $x$: the sine is a zero carrying that sign, and the cosecant is
///   its reciprocal, which keeps the function odd.
/// - If $x/u$ in lowest terms has denominator 4, the result is exactly $\pm1$, and if it has
///   denominator 12, exactly $\pm2$.
/// - If $x/u$ in lowest terms has denominator 3 or 6, the result is $\pm2\sqrt3/3$; if 8,
///   $\pm\sqrt2$; and if 20, $\pm2\varphi$ or $\pm2(\varphi-1)$, where $\varphi$ is the golden
///   ratio.
///
/// Overflow happens at a pole, where the result is exactly $\pm\infty$, and for a tiny $x/u$, whose
/// cosecant is close to $u/(2\pi x)$: an [`f32`] or [`f64`] whose fraction of a turn is not a
/// multiple of $1/2$ is more than $2^{-66}$ of a turn away from one, so a cosecant that is not a
/// pole stays below $2^{64}$ unless the angle itself is tiny. Underflow is not possible, since
/// $|\csc(2\pi x/u)| \geq 1$.
///
/// # Worst-case complexity
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use malachite_base::num::basic::traits::NegativeInfinity;
/// use malachite_base::num::float::NiceFloat;
/// use malachite_float::float::arithmetic::csc::primitive_float_csc_with_period;
///
/// assert!(primitive_float_csc_with_period(f32::NAN, 360).is_nan());
/// assert!(primitive_float_csc_with_period(f32::INFINITY, 360).is_nan());
/// assert!(primitive_float_csc_with_period(f32::NEGATIVE_INFINITY, 360).is_nan());
/// assert!(primitive_float_csc_with_period(1.0f32, 0).is_nan());
/// assert_eq!(
///     NiceFloat(primitive_float_csc_with_period(-0.0f32, 360)),
///     NiceFloat(f32::NEGATIVE_INFINITY)
/// );
/// // a quarter turn is exactly 1
/// assert_eq!(
///     NiceFloat(primitive_float_csc_with_period(90.0f32, 360)),
///     NiceFloat(1.0)
/// );
/// // a half turn is a pole
/// assert_eq!(
///     NiceFloat(primitive_float_csc_with_period(180.0f32, 360)),
///     NiceFloat(f32::INFINITY)
/// );
/// // a sixth of a turn: 2 sqrt(3)/3
/// assert_eq!(
///     NiceFloat(primitive_float_csc_with_period(60.0f32, 360)),
///     NiceFloat(1.1547005)
/// );
/// // a twelfth of a turn is exactly 2
/// assert_eq!(
///     NiceFloat(primitive_float_csc_with_period(30.0f64, 360)),
///     NiceFloat(2.0)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_csc_with_period(1.0f32, 7)),
///     NiceFloat(1.279048)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_csc_with_period(1.0f64, 7)),
///     NiceFloat(1.2790480076899327)
/// );
/// ```
#[inline]
#[allow(clippy::type_repetition_in_bounds)]
pub fn primitive_float_csc_with_period<T: PrimitiveFloat>(x: T, u: u64) -> T
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    emulate_float_to_float_fn(|x, prec| Float::csc_with_period_prec(x, u, prec), x)
}
