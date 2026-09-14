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

// Port of MPFR's cotangent. `mpfr_cot` (`cot.c`) instantiates the generic reciprocal template
// (`gen_inverse.h`) with the tangent, and MPFR's tangent is in turn a quotient of a sine and a
// cosine; taking cot x as cos x / sin x directly saves the middle rounding and gives the two ends
// of the exponent range the same treatment, so that is what the Ziv loop below does, with the same
// working precision and the same two bits of slack. Unlike the secant and the cosecant, the
// cotangent is not bounded away from zero, so it both overflows, within 2^(-2^30) of a multiple of
// pi, and underflows, within 2^(-2^30) of an odd multiple of pi/2; each end is decided from an
// exact bracket. MPFR's shortcut for a tiny input, where cot x is 1/x - x/3 + O(x^3), is kept: the
// quotient there is exactly representable, so the Ziv loop could never certify it.

use crate::InnerFloat::{Finite, Infinity, NaN, Zero};
use crate::float::arithmetic::cos::trig_near_zero_bracket;
use crate::float::arithmetic::tan::{
    MAX_CANCEL, MAX_SETTLED_EXPONENT, MIN_SETTLED_EXPONENT, nearest_bracket,
    round_bracket_signed_by,
};
use crate::{Float, emulate_float_to_float_fn};
use core::cmp::Ordering::{self, Equal, Greater, Less};
use core::cmp::max;
use malachite_base::num::arithmetic::traits::{CeilingLogBase2, Cot, CotAssign, IsPowerOf2};
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::{
    Infinity as InfinityTrait, NaN as NaNTrait, NegativeInfinity,
};
use malachite_base::num::conversion::traits::{ExactFrom, RoundingFrom};
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{self, Ceiling, Down, Exact, Floor, Nearest};
use malachite_nz::natural::arithmetic::float::round::float_can_round;
use malachite_nz::platform::Limb;

// cot x for a tiny x, where cot x = 1/x - x/3 - ... and |cot x - 1/x| <= 0.36 for |x| <= 1, with
// the correction opposing the sign of 1/x, so that |cot x| < |1/x|. MPFR's condition, EXP(x) + 1 <=
// -2 max(PREC(x), prec), makes rounding 1/x settle the cotangent, except when 1/x is exact (x a
// power of 2), where the true value lies one step short of it, toward zero. The general loop could
// not settle that case at any working precision, since the quotient is then exactly representable.
//
// This is ACTION_TINY from cot.c, MPFR 4.2.2.
fn cot_tiny(x: &Float, prec: u64, rm: RoundingMode) -> (Float, Ordering) {
    let (r, o) = x.reciprocal_prec_round_ref(prec, rm);
    if o != Equal {
        return (r, o);
    }
    assert_ne!(rm, Exact, "Inexact cot");
    let negative = x.is_sign_negative();
    // 1/x is exact, so the cotangent is one step short of it, toward zero
    let toward = match rm {
        Floor => !negative,
        Ceiling => negative,
        Down => true,
        _ => false,
    };
    let mut r = r;
    if toward {
        if negative {
            r.increment();
        } else {
            r.decrement();
        }
        (r, if negative { Greater } else { Less })
    } else {
        (r, if negative { Less } else { Greater })
    }
}

// As in mpfr_overflow, with the overflow's sign: the toward-zero modes give the largest finite
// value, and the other modes an infinity.
fn cot_overflow(negative: bool, prec: u64, rm: RoundingMode) -> (Float, Ordering) {
    match (negative, rm) {
        (_, Exact) => panic!("Inexact cot"),
        (false, Floor | Down) => (Float::max_finite_value_with_prec(prec), Less),
        (false, _) => (Float::INFINITY, Greater),
        (true, Ceiling | Down) => (-Float::max_finite_value_with_prec(prec), Greater),
        (true, _) => (Float::NEGATIVE_INFINITY, Less),
    }
}

// Decides cot(x) = c/s from the sine and cosine rounded to nearest at precision m, by a `Rational`
// bracket, for the cases the `Float` quotient cannot settle: it overflowed, underflowed, or lies
// within two bits of either end of the exponent range, or the sine or cosine underflowed. Returns
// `None` if the bracket does not decide the rounding, so that the working precision must grow.
fn cot_bracket(
    x: &Float,
    s: &Float,
    c: &Float,
    m: u64,
    prec: u64,
    rm: RoundingMode,
) -> Option<(Float, Ordering)> {
    let negative = s.is_sign_negative() != c.is_sign_negative();
    // A sine that underflowed is at most 1.5 times the smallest positive `Float`, and the cosine is
    // then within 2^-m of 1, so the cotangent is at least 2^(2^30)/1.5 in magnitude, beyond the
    // largest finite `Float`.
    if *s == 0u32
        || (s.get_exponent() == Some(Float::MIN_EXPONENT)
            && s.significand_ref().unwrap().is_power_of_2())
    {
        return Some(cot_overflow(negative, prec, rm));
    }
    let (s_lo, s_hi) = nearest_bracket(s, m);
    let (c_lo, c_hi) = if *c == 0u32 {
        // The cosine underflowed, so its rounding says only that it is below half the smallest
        // positive `Float`, and the cotangent, barely larger than the cosine, cannot be placed
        // against that same bound: take the cosine's exact bracket from the distance to the nearest
        // odd multiple of pi/2, as the near-zero path does.
        let (lo, hi) = trig_near_zero_bracket(x, m + 64, MAX_CANCEL, true);
        if lo < 0u32 { (-hi, -lo) } else { (lo, hi) }
    } else {
        nearest_bracket(c, m)
    };
    round_bracket_signed_by(negative, c_lo / s_hi, c_hi / s_lo, prec, rm)
}

// This is mpfr_cot from cot.c, MPFR 4.2.2, with the tangent's own quotient of a sine and a cosine
// folded in and the bracket path for results near the ends of the exponent range.
fn cot_prec_round_normal_ref(x: &Float, prec: u64, rm: RoundingMode) -> (Float, Ordering) {
    assert_ne!(rm, Exact, "Inexact cot");
    let exp_x = i64::from(x.get_exponent().unwrap());
    // ACTION_TINY from cot.c: EXP(x) + 1 <= -2 max(PREC(x), PREC(y))
    let n = i64::exact_from(max(x.get_prec().unwrap(), prec));
    if exp_x < -(n << 1) {
        return cot_tiny(x, prec, rm);
    }
    // Compute initial precision
    let mut m = prec + prec.ceiling_log_base_2() + 13;
    let mut increment = Limb::WIDTH;
    loop {
        // err <= 1/2 ulp on s and c, each correctly rounded even within 2^(-2^30) of a zero of its
        // function, where it may underflow
        let (s, c, _, _) = x.sin_cos_prec_ref(m);
        // err <= 4 ulps
        let q = if s == 0u32 || c == 0u32 {
            None
        } else {
            Some(c.div_prec_ref_ref(&s, m).0)
        };
        // A quotient that overflowed, underflowed, or lies within two bits of either end of the
        // exponent range, where rounding it to `prec` could still cross the end, is decided from
        // brackets, as is a sine or cosine that underflowed.
        match q.as_ref().and_then(Float::get_exponent).map(i64::from) {
            Some(e) if e > MIN_SETTLED_EXPONENT && e < MAX_SETTLED_EXPONENT => {
                let q = q.unwrap();
                if float_can_round(q.significand_ref().unwrap(), m - 2, prec, rm) {
                    return Float::from_float_prec_round(q, prec, rm);
                }
            }
            _ => {
                if let Some(result) = cot_bracket(x, &s, &c, m, prec, rm) {
                    return result;
                }
            }
        }
        m += increment;
        increment = m >> 1;
    }
}

impl Float {
    /// Computes $\cot x$, the cotangent of a [`Float`], rounding the result to the specified
    /// precision and with the specified rounding mode. The [`Float`] is taken by value. An
    /// [`Ordering`] is also returned, indicating whether the rounded cotangent is less than, equal
    /// to, or greater than the exact cotangent. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \cot x+\varepsilon.
    /// $$
    /// - If $x$ is not finite, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x$ is finite and $m$ is not `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2 |\cot
    ///   x|\rfloor-p+1}$.
    /// - If $x$ is finite and $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2 |\cot
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
    /// Overflow requires an input within $2^{-2^{30}}$ of a nonzero multiple of $\pi$, and
    /// underflow an input within $2^{-2^{30}}$ of an odd multiple of $\pi/2$, either of which takes
    /// more than $2^{30}$ bits of precision; overflow also occurs for an input of magnitude about
    /// $2^{-2^{30}}$, whose reciprocal alone is beyond the largest finite [`Float`].
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::cot_prec`] instead. If you
    /// know that your target precision is the precision of the input, consider using
    /// [`Float::cot_round`] instead. If both of these things are true, consider using
    /// [`Float::cot`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m, e) = O(n (\log n)^3 \log\log n + (n+m+e) (\log (n+m+e))^2 \log\log (n+m+e))$
    ///
    /// $M(n, m, e) = O((n+m+e) \log (n+m+e))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, $m$ is
    /// `self.significant_bits()`, and $e$ is the exponent of `self` (0 if `self` has no exponent or
    /// a negative one): the sine and cosine at working precision $n$ (for large $n$ by binary
    /// splitting of the Taylor series, otherwise the cosine, from which the sine is derived), and
    /// their quotient, cost the first term, and for $|x| \geq 4$ the argument is reduced modulo
    /// $2\pi$, which requires $\pi$ to about $n + e$ bits and a remainder of the $m$-bit input.
    /// Unlike most functions, `cot` therefore gets slower as the magnitude of its input grows, not
    /// just as the precision does.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact`, since the cotangent of a finite nonzero [`Float`] is never
    /// exactly representable, or if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .cot_prec_round(5, Floor);
    /// assert_eq!(c.to_string(), "0.625");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .cot_prec_round(5, Ceiling);
    /// assert_eq!(c.to_string(), "0.656");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .cot_prec_round(5, Nearest);
    /// assert_eq!(c.to_string(), "0.656");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .cot_prec_round(20, Floor);
    /// assert_eq!(c.to_string(), "0.64209175");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .cot_prec_round(20, Ceiling);
    /// assert_eq!(c.to_string(), "0.64209270");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .cot_prec_round(20, Nearest);
    /// assert_eq!(c.to_string(), "0.64209270");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn cot_prec_round(self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        self.cot_prec_round_ref(prec, rm)
    }

    /// Computes $\cot x$, the cotangent of a [`Float`], rounding the result to the specified
    /// precision and with the specified rounding mode. The [`Float`] is taken by reference. An
    /// [`Ordering`] is also returned, indicating whether the rounded cotangent is less than, equal
    /// to, or greater than the exact cotangent. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \cot x+\varepsilon.
    /// $$
    /// - If $x$ is not finite, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x$ is finite and $m$ is not `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2 |\cot
    ///   x|\rfloor-p+1}$.
    /// - If $x$ is finite and $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2 |\cot
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
    /// Overflow requires an input within $2^{-2^{30}}$ of a nonzero multiple of $\pi$, and
    /// underflow an input within $2^{-2^{30}}$ of an odd multiple of $\pi/2$, either of which takes
    /// more than $2^{30}$ bits of precision; overflow also occurs for an input of magnitude about
    /// $2^{-2^{30}}$, whose reciprocal alone is beyond the largest finite [`Float`].
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::cot_prec_ref`] instead. If
    /// you know that your target precision is the precision of the input, consider using
    /// [`Float::cot_round_ref`] instead. If both of these things are true, consider using
    /// `(&Float).cot()` instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m, e) = O(n (\log n)^3 \log\log n + (n+m+e) (\log (n+m+e))^2 \log\log (n+m+e))$
    ///
    /// $M(n, m, e) = O((n+m+e) \log (n+m+e))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, $m$ is
    /// `self.significant_bits()`, and $e$ is the exponent of `self` (0 if `self` has no exponent or
    /// a negative one): the sine and cosine at working precision $n$ (for large $n$ by binary
    /// splitting of the Taylor series, otherwise the cosine, from which the sine is derived), and
    /// their quotient, cost the first term, and for $|x| \geq 4$ the argument is reduced modulo
    /// $2\pi$, which requires $\pi$ to about $n + e$ bits and a remainder of the $m$-bit input.
    /// Unlike most functions, `cot` therefore gets slower as the magnitude of its input grows, not
    /// just as the precision does.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact`, since the cotangent of a finite nonzero [`Float`] is never
    /// exactly representable, or if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = (&Float::from_unsigned_prec(1u32, 100).0).cot_prec_round_ref(5, Floor);
    /// assert_eq!(c.to_string(), "0.625");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = (&Float::from_unsigned_prec(1u32, 100).0).cot_prec_round_ref(5, Ceiling);
    /// assert_eq!(c.to_string(), "0.656");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = (&Float::from_unsigned_prec(1u32, 100).0).cot_prec_round_ref(5, Nearest);
    /// assert_eq!(c.to_string(), "0.656");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = (&Float::from_unsigned_prec(1u32, 100).0).cot_prec_round_ref(20, Floor);
    /// assert_eq!(c.to_string(), "0.64209175");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = (&Float::from_unsigned_prec(1u32, 100).0).cot_prec_round_ref(20, Ceiling);
    /// assert_eq!(c.to_string(), "0.64209270");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = (&Float::from_unsigned_prec(1u32, 100).0).cot_prec_round_ref(20, Nearest);
    /// assert_eq!(c.to_string(), "0.64209270");
    /// assert_eq!(o, Greater);
    /// ```
    pub fn cot_prec_round_ref(&self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        assert_ne!(prec, 0);
        match &self.0 {
            NaN | Infinity { .. } => (Self::NAN, Equal),
            // cot(+0) = +infinity, cot(-0) = -infinity
            Zero { .. } => (
                if self.is_sign_negative() {
                    Self::NEGATIVE_INFINITY
                } else {
                    Self::INFINITY
                },
                Equal,
            ),
            Finite { .. } => cot_prec_round_normal_ref(self, prec, rm),
        }
    }

    /// Computes $\cot x$, the cotangent of a [`Float`], rounding the result to the nearest value of
    /// the specified precision. The [`Float`] is taken by value. An [`Ordering`] is also returned,
    /// indicating whether the rounded cotangent is less than, equal to, or greater than the exact
    /// cotangent. Although `NaN`s are not comparable to any [`Float`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// If the cotangent is equidistant from two [`Float`]s with the specified precision, the
    /// [`Float`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a
    /// description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = \cot x+\varepsilon.
    /// $$
    /// - If $x$ is not finite, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x$ is finite, then $|\varepsilon| < 2^{\lfloor\log_2 |\cot x|\rfloor-p}$.
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
    /// Overflow requires an input within $2^{-2^{30}}$ of a nonzero multiple of $\pi$, and
    /// underflow an input within $2^{-2^{30}}$ of an odd multiple of $\pi/2$, either of which takes
    /// more than $2^{30}$ bits of precision; overflow also occurs for an input of magnitude about
    /// $2^{-2^{30}}$, whose reciprocal alone is beyond the largest finite [`Float`].
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::cot_prec_round`] instead. If you know that your target precision is the precision
    /// of the input, consider using [`Float::cot`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m, e) = O(n (\log n)^3 \log\log n + (n+m+e) (\log (n+m+e))^2 \log\log (n+m+e))$
    ///
    /// $M(n, m, e) = O((n+m+e) \log (n+m+e))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, $m$ is
    /// `self.significant_bits()`, and $e$ is the exponent of `self` (0 if `self` has no exponent or
    /// a negative one): the sine and cosine at working precision $n$ (for large $n$ by binary
    /// splitting of the Taylor series, otherwise the cosine, from which the sine is derived), and
    /// their quotient, cost the first term, and for $|x| \geq 4$ the argument is reduced modulo
    /// $2\pi$, which requires $\pi$ to about $n + e$ bits and a remainder of the $m$-bit input.
    /// Unlike most functions, `cot` therefore gets slower as the magnitude of its input grows, not
    /// just as the precision does.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100).0.cot_prec(5);
    /// assert_eq!(c.to_string(), "0.656");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100).0.cot_prec(20);
    /// assert_eq!(c.to_string(), "0.64209270");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn cot_prec(self, prec: u64) -> (Self, Ordering) {
        self.cot_prec_round(prec, Nearest)
    }

    /// Computes $\cot x$, the cotangent of a [`Float`], rounding the result to the nearest value of
    /// the specified precision. The [`Float`] is taken by reference. An [`Ordering`] is also
    /// returned, indicating whether the rounded cotangent is less than, equal to, or greater than
    /// the exact cotangent. Although `NaN`s are not comparable to any [`Float`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// If the cotangent is equidistant from two [`Float`]s with the specified precision, the
    /// [`Float`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a
    /// description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = \cot x+\varepsilon.
    /// $$
    /// - If $x$ is not finite, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x$ is finite, then $|\varepsilon| < 2^{\lfloor\log_2 |\cot x|\rfloor-p}$.
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
    /// Overflow requires an input within $2^{-2^{30}}$ of a nonzero multiple of $\pi$, and
    /// underflow an input within $2^{-2^{30}}$ of an odd multiple of $\pi/2$, either of which takes
    /// more than $2^{30}$ bits of precision; overflow also occurs for an input of magnitude about
    /// $2^{-2^{30}}$, whose reciprocal alone is beyond the largest finite [`Float`].
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::cot_prec_round_ref`] instead. If you know that your target precision is the
    /// precision of the input, consider using `(&Float).cot()` instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m, e) = O(n (\log n)^3 \log\log n + (n+m+e) (\log (n+m+e))^2 \log\log (n+m+e))$
    ///
    /// $M(n, m, e) = O((n+m+e) \log (n+m+e))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, $m$ is
    /// `self.significant_bits()`, and $e$ is the exponent of `self` (0 if `self` has no exponent or
    /// a negative one): the sine and cosine at working precision $n$ (for large $n$ by binary
    /// splitting of the Taylor series, otherwise the cosine, from which the sine is derived), and
    /// their quotient, cost the first term, and for $|x| \geq 4$ the argument is reduced modulo
    /// $2\pi$, which requires $\pi$ to about $n + e$ bits and a remainder of the $m$-bit input.
    /// Unlike most functions, `cot` therefore gets slower as the magnitude of its input grows, not
    /// just as the precision does.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = (&Float::from_unsigned_prec(1u32, 100).0).cot_prec_ref(5);
    /// assert_eq!(c.to_string(), "0.656");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = (&Float::from_unsigned_prec(1u32, 100).0).cot_prec_ref(20);
    /// assert_eq!(c.to_string(), "0.64209270");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn cot_prec_ref(&self, prec: u64) -> (Self, Ordering) {
        self.cot_prec_round_ref(prec, Nearest)
    }

    /// Computes $\cot x$, the cotangent of a [`Float`], rounding the result with the specified
    /// rounding mode. The [`Float`] is taken by value. An [`Ordering`] is also returned, indicating
    /// whether the rounded cotangent is less than, equal to, or greater than the exact cotangent.
    /// Although `NaN`s are not comparable to any [`Float`], whenever this function returns a `NaN`
    /// it also returns `Equal`.
    ///
    /// The precision of the output is the precision of the input. See [`RoundingMode`] for a
    /// description of the possible rounding modes.
    ///
    /// $$
    /// f(x,m) = \cot x+\varepsilon.
    /// $$
    /// - If $x$ is not finite, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x$ is finite and $m$ is not `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2 |\cot
    ///   x|\rfloor-p+1}$, where $p$ is the precision of the input.
    /// - If $x$ is finite and $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2 |\cot
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
    /// Overflow requires an input within $2^{-2^{30}}$ of a nonzero multiple of $\pi$, and
    /// underflow an input within $2^{-2^{30}}$ of an odd multiple of $\pi/2$, either of which takes
    /// more than $2^{30}$ bits of precision; overflow also occurs for an input of magnitude about
    /// $2^{-2^{30}}$, whose reciprocal alone is beyond the largest finite [`Float`].
    ///
    /// If you want to specify an output precision, consider using [`Float::cot_prec_round`]
    /// instead. If you know you'll be using the `Nearest` rounding mode, consider using
    /// [`Float::cot`] instead.
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
    /// e$ bits. Unlike most functions, `cot` therefore gets slower as the magnitude of its input
    /// grows, not just as the precision does.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact`, since the cotangent of a finite nonzero [`Float`] is never
    /// exactly representable.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100).0.cot_round(Floor);
    /// assert_eq!(c.to_string(), "0.64209261593433070300641998659417");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100).0.cot_round(Ceiling);
    /// assert_eq!(c.to_string(), "0.64209261593433070300641998659496");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100).0.cot_round(Nearest);
    /// assert_eq!(c.to_string(), "0.64209261593433070300641998659417");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn cot_round(self, rm: RoundingMode) -> (Self, Ordering) {
        let prec = self.significant_bits();
        self.cot_prec_round(prec, rm)
    }

    /// Computes $\cot x$, the cotangent of a [`Float`], rounding the result with the specified
    /// rounding mode. The [`Float`] is taken by reference. An [`Ordering`] is also returned,
    /// indicating whether the rounded cotangent is less than, equal to, or greater than the exact
    /// cotangent. Although `NaN`s are not comparable to any [`Float`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the precision of the input. See [`RoundingMode`] for a
    /// description of the possible rounding modes.
    ///
    /// $$
    /// f(x,m) = \cot x+\varepsilon.
    /// $$
    /// - If $x$ is not finite, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x$ is finite and $m$ is not `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2 |\cot
    ///   x|\rfloor-p+1}$, where $p$ is the precision of the input.
    /// - If $x$ is finite and $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2 |\cot
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
    /// Overflow requires an input within $2^{-2^{30}}$ of a nonzero multiple of $\pi$, and
    /// underflow an input within $2^{-2^{30}}$ of an odd multiple of $\pi/2$, either of which takes
    /// more than $2^{30}$ bits of precision; overflow also occurs for an input of magnitude about
    /// $2^{-2^{30}}$, whose reciprocal alone is beyond the largest finite [`Float`].
    ///
    /// If you want to specify an output precision, consider using [`Float::cot_prec_round_ref`]
    /// instead. If you know you'll be using the `Nearest` rounding mode, consider using
    /// `(&Float).cot()` instead.
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
    /// e$ bits. Unlike most functions, `cot` therefore gets slower as the magnitude of its input
    /// grows, not just as the precision does.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact`, since the cotangent of a finite nonzero [`Float`] is never
    /// exactly representable.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = (&Float::from_unsigned_prec(1u32, 100).0).cot_round_ref(Floor);
    /// assert_eq!(c.to_string(), "0.64209261593433070300641998659417");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = (&Float::from_unsigned_prec(1u32, 100).0).cot_round_ref(Ceiling);
    /// assert_eq!(c.to_string(), "0.64209261593433070300641998659496");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = (&Float::from_unsigned_prec(1u32, 100).0).cot_round_ref(Nearest);
    /// assert_eq!(c.to_string(), "0.64209261593433070300641998659417");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn cot_round_ref(&self, rm: RoundingMode) -> (Self, Ordering) {
        self.cot_prec_round_ref(self.significant_bits(), rm)
    }

    /// Computes $\cot x$, the cotangent of a [`Float`], rounding the result to the specified
    /// precision and with the specified rounding mode. The [`Float`] is replaced by the result, and
    /// an [`Ordering`] is returned, indicating whether the rounded cotangent is less than, equal
    /// to, or greater than the exact cotangent. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function sets a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets \cot x+\varepsilon.
    /// $$
    /// - If $x$ is not finite, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x$ is finite and $m$ is not `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2 |\cot
    ///   x|\rfloor-p+1}$.
    /// - If $x$ is finite and $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2 |\cot
    ///   x|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// See the [`Float::cot_prec_round`] documentation for information on special cases and
    /// overflow.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::cot_prec_assign`] instead. If
    /// you know that your target precision is the precision of the input, consider using
    /// [`Float::cot_round_assign`] instead. If both of these things are true, consider using
    /// [`Float::cot_assign`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m, e) = O(n (\log n)^3 \log\log n + (n+m+e) (\log (n+m+e))^2 \log\log (n+m+e))$
    ///
    /// $M(n, m, e) = O((n+m+e) \log (n+m+e))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, $m$ is
    /// `self.significant_bits()`, and $e$ is the exponent of `self` (0 if `self` has no exponent or
    /// a negative one): the sine and cosine at working precision $n$ (for large $n$ by binary
    /// splitting of the Taylor series, otherwise the cosine, from which the sine is derived), and
    /// their quotient, cost the first term, and for $|x| \geq 4$ the argument is reduced modulo
    /// $2\pi$, which requires $\pi$ to about $n + e$ bits and a remainder of the $m$-bit input.
    /// Unlike most functions, `cot` therefore gets slower as the magnitude of its input grows, not
    /// just as the precision does.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact`, since the cotangent of a finite nonzero [`Float`] is never
    /// exactly representable, or if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.cot_prec_round_assign(5, Floor), Less);
    /// assert_eq!(x.to_string(), "0.625");
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.cot_prec_round_assign(5, Ceiling), Greater);
    /// assert_eq!(x.to_string(), "0.656");
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.cot_prec_round_assign(5, Nearest), Greater);
    /// assert_eq!(x.to_string(), "0.656");
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.cot_prec_round_assign(20, Floor), Less);
    /// assert_eq!(x.to_string(), "0.64209175");
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.cot_prec_round_assign(20, Ceiling), Greater);
    /// assert_eq!(x.to_string(), "0.64209270");
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.cot_prec_round_assign(20, Nearest), Greater);
    /// assert_eq!(x.to_string(), "0.64209270");
    /// ```
    #[inline]
    pub fn cot_prec_round_assign(&mut self, prec: u64, rm: RoundingMode) -> Ordering {
        let o;
        (*self, o) = self.cot_prec_round_ref(prec, rm);
        o
    }

    /// Computes $\cot x$, the cotangent of a [`Float`], rounding the result to the nearest value of
    /// the specified precision. The [`Float`] is replaced by the result, and an [`Ordering`] is
    /// returned, indicating whether the rounded cotangent is less than, equal to, or greater than
    /// the exact cotangent. Although `NaN`s are not comparable to any [`Float`], whenever this
    /// function sets a `NaN` it also returns `Equal`.
    ///
    /// If the cotangent is equidistant from two [`Float`]s with the specified precision, the
    /// [`Float`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a
    /// description of the `Nearest` rounding mode.
    ///
    /// $$
    /// x \gets \cot x+\varepsilon.
    /// $$
    /// - If $x$ is not finite, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x$ is finite, then $|\varepsilon| < 2^{\lfloor\log_2 |\cot x|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// See the [`Float::cot_prec`] documentation for information on special cases and overflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::cot_prec_round_assign`] instead. If you know that your target precision is the
    /// precision of the input, consider using [`Float::cot_assign`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m, e) = O(n (\log n)^3 \log\log n + (n+m+e) (\log (n+m+e))^2 \log\log (n+m+e))$
    ///
    /// $M(n, m, e) = O((n+m+e) \log (n+m+e))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, $m$ is
    /// `self.significant_bits()`, and $e$ is the exponent of `self` (0 if `self` has no exponent or
    /// a negative one): the sine and cosine at working precision $n$ (for large $n$ by binary
    /// splitting of the Taylor series, otherwise the cosine, from which the sine is derived), and
    /// their quotient, cost the first term, and for $|x| \geq 4$ the argument is reduced modulo
    /// $2\pi$, which requires $\pi$ to about $n + e$ bits and a remainder of the $m$-bit input.
    /// Unlike most functions, `cot` therefore gets slower as the magnitude of its input grows, not
    /// just as the precision does.
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
    /// assert_eq!(x.cot_prec_assign(5), Greater);
    /// assert_eq!(x.to_string(), "0.656");
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.cot_prec_assign(20), Greater);
    /// assert_eq!(x.to_string(), "0.64209270");
    /// ```
    #[inline]
    pub fn cot_prec_assign(&mut self, prec: u64) -> Ordering {
        self.cot_prec_round_assign(prec, Nearest)
    }

    /// Computes $\cot x$, the cotangent of a [`Float`], rounding the result with the specified
    /// rounding mode. The [`Float`] is replaced by the result, and an [`Ordering`] is returned,
    /// indicating whether the rounded cotangent is less than, equal to, or greater than the exact
    /// cotangent. Although `NaN`s are not comparable to any [`Float`], whenever this function sets
    /// a `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the precision of the input. See [`RoundingMode`] for a
    /// description of the possible rounding modes.
    ///
    /// $$
    /// x \gets \cot x+\varepsilon.
    /// $$
    /// - If $x$ is not finite, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x$ is finite and $m$ is not `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2 |\cot
    ///   x|\rfloor-p+1}$, where $p$ is the precision of the input.
    /// - If $x$ is finite and $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2 |\cot
    ///   x|\rfloor-p}$, where $p$ is the precision of the input.
    ///
    /// If the output has a precision, it is the precision of the input.
    ///
    /// See the [`Float::cot_round`] documentation for information on special cases and overflow.
    ///
    /// If you want to specify an output precision, consider using [`Float::cot_prec_round_assign`]
    /// instead. If you know you'll be using the `Nearest` rounding mode, consider using
    /// [`Float::cot_assign`] instead.
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
    /// e$ bits. Unlike most functions, `cot` therefore gets slower as the magnitude of its input
    /// grows, not just as the precision does.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact`, since the cotangent of a finite nonzero [`Float`] is never
    /// exactly representable.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.cot_round_assign(Floor), Less);
    /// assert_eq!(x.to_string(), "0.64209261593433070300641998659417");
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.cot_round_assign(Ceiling), Greater);
    /// assert_eq!(x.to_string(), "0.64209261593433070300641998659496");
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.cot_round_assign(Nearest), Less);
    /// assert_eq!(x.to_string(), "0.64209261593433070300641998659417");
    /// ```
    #[inline]
    pub fn cot_round_assign(&mut self, rm: RoundingMode) -> Ordering {
        let prec = self.significant_bits();
        self.cot_prec_round_assign(prec, rm)
    }
}

impl Cot for Float {
    type Output = Self;

    /// Computes $\cot x$, the cotangent of a [`Float`], taking it by value.
    ///
    /// If the output has a precision, it is the precision of the input. If the cotangent is
    /// equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s in
    /// its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// $$
    /// f(x) = \cot x+\varepsilon.
    /// $$
    /// - If $x$ is not finite, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x$ is finite, then $|\varepsilon| < 2^{\lfloor\log_2 |\cot x|\rfloor-p}$, where $p$ is
    ///   the precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN})=\text{NaN}$
    /// - $f(\pm\infty)=\text{NaN}$
    /// - $f(\pm0.0)=\pm\infty$
    ///
    /// See the [`Float::cot_round`] documentation for information on overflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using [`Float::cot_round`]
    /// instead. If you want to specify the output precision, consider using [`Float::cot_prec`]. If
    /// you want both of these things, consider using [`Float::cot_prec_round`].
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
    /// e$ bits. Unlike most functions, `cot` therefore gets slower as the magnitude of its input
    /// grows, not just as the precision does.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Cot;
    /// use malachite_base::num::basic::traits::*;
    /// use malachite_float::Float;
    ///
    /// assert!(Float::NAN.cot().is_nan());
    /// assert!(Float::INFINITY.cot().is_nan());
    /// assert!(Float::NEGATIVE_INFINITY.cot().is_nan());
    /// assert_eq!(Float::ZERO.cot().to_string(), "Infinity");
    /// assert_eq!(Float::NEGATIVE_ZERO.cot().to_string(), "-Infinity");
    /// assert_eq!(
    ///     Float::from_unsigned_prec(1u32, 100).0.cot().to_string(),
    ///     "0.64209261593433070300641998659417"
    /// );
    /// assert_eq!(
    ///     Float::from_unsigned_prec(100u32, 100).0.cot().to_string(),
    ///     "-1.7029569194264692160987314595571"
    /// );
    /// ```
    #[inline]
    fn cot(self) -> Self {
        let prec = self.significant_bits();
        self.cot_prec_round(prec, Nearest).0
    }
}

impl Cot for &Float {
    type Output = Float;

    /// Computes $\cot x$, the cotangent of a [`Float`], taking it by reference.
    ///
    /// If the output has a precision, it is the precision of the input. If the cotangent is
    /// equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s in
    /// its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// $$
    /// f(x) = \cot x+\varepsilon.
    /// $$
    /// - If $x$ is not finite, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x$ is finite, then $|\varepsilon| < 2^{\lfloor\log_2 |\cot x|\rfloor-p}$, where $p$ is
    ///   the precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN})=\text{NaN}$
    /// - $f(\pm\infty)=\text{NaN}$
    /// - $f(\pm0.0)=\pm\infty$
    ///
    /// See the [`Float::cot_round`] documentation for information on overflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::cot_round_ref`] instead. If you want to specify the output precision, consider
    /// using [`Float::cot_prec_ref`]. If you want both of these things, consider using
    /// [`Float::cot_prec_round_ref`].
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
    /// e$ bits. Unlike most functions, `cot` therefore gets slower as the magnitude of its input
    /// grows, not just as the precision does.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Cot;
    /// use malachite_base::num::basic::traits::*;
    /// use malachite_float::Float;
    ///
    /// assert!(Float::NAN.cot().is_nan());
    /// assert!(Float::INFINITY.cot().is_nan());
    /// assert!(Float::NEGATIVE_INFINITY.cot().is_nan());
    /// assert_eq!(Float::ZERO.cot().to_string(), "Infinity");
    /// assert_eq!(Float::NEGATIVE_ZERO.cot().to_string(), "-Infinity");
    /// assert_eq!(
    ///     (&Float::from_unsigned_prec(1u32, 100).0).cot().to_string(),
    ///     "0.64209261593433070300641998659417"
    /// );
    /// assert_eq!(
    ///     (&Float::from_unsigned_prec(100u32, 100).0)
    ///         .cot()
    ///         .to_string(),
    ///     "-1.7029569194264692160987314595571"
    /// );
    /// ```
    #[inline]
    fn cot(self) -> Float {
        self.cot_prec_round_ref(self.significant_bits(), Nearest).0
    }
}

impl CotAssign for Float {
    /// Computes $\cot x$, the cotangent of a [`Float`], in place.
    ///
    /// If the output has a precision, it is the precision of the input. If the cotangent is
    /// equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s in
    /// its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// $$
    /// x \gets \cot x+\varepsilon.
    /// $$
    /// - If $x$ is not finite, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x$ is finite, then $|\varepsilon| < 2^{\lfloor\log_2 |\cot x|\rfloor-p}$, where $p$ is
    ///   the precision of the input.
    ///
    /// See the [`Float::cot`] documentation for information on special cases and overflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::cot_round_assign`] instead. If you want to specify the output precision, consider
    /// using [`Float::cot_prec_assign`]. If you want both of these things, consider using
    /// [`Float::cot_prec_round_assign`].
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
    /// e$ bits. Unlike most functions, `cot` therefore gets slower as the magnitude of its input
    /// grows, not just as the precision does.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::CotAssign;
    /// use malachite_base::num::basic::traits::*;
    /// use malachite_float::Float;
    ///
    /// let mut x = Float::NAN;
    /// x.cot_assign();
    /// assert!(x.is_nan());
    ///
    /// let mut x = Float::INFINITY;
    /// x.cot_assign();
    /// assert!(x.is_nan());
    ///
    /// let mut x = Float::NEGATIVE_INFINITY;
    /// x.cot_assign();
    /// assert!(x.is_nan());
    ///
    /// let mut x = Float::ZERO;
    /// x.cot_assign();
    /// assert_eq!(x.to_string(), "Infinity");
    ///
    /// let mut x = Float::NEGATIVE_ZERO;
    /// x.cot_assign();
    /// assert_eq!(x.to_string(), "-Infinity");
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// x.cot_assign();
    /// assert_eq!(x.to_string(), "0.64209261593433070300641998659417");
    ///
    /// let mut x = Float::from_unsigned_prec(100u32, 100).0;
    /// x.cot_assign();
    /// assert_eq!(x.to_string(), "-1.7029569194264692160987314595571");
    /// ```
    #[inline]
    fn cot_assign(&mut self) {
        let prec = self.significant_bits();
        self.cot_prec_round_assign(prec, Nearest);
    }
}

/// Computes $\cot x$, the cotangent of a primitive float, correctly rounded. Neither the standard
/// library nor `libm` provides a cotangent.
///
/// $$
/// f(x) = \cot x+\varepsilon.
/// $$
/// - If $x$ is not finite, $\varepsilon$ may be ignored or assumed to be 0.
/// - If $x$ is finite, then $|\varepsilon| < 2^{\lfloor\log_2 |\cot x|\rfloor-p}$, where $p$ is the
///   precision of the output (24 if `T` is a [`f32`] and 53 if `T` is a [`f64`]).
///
/// Special cases:
/// - $f(\text{NaN})=\text{NaN}$
/// - $f(\pm\infty)=\text{NaN}$
/// - $f(\pm0.0)=\pm\infty$
///
/// Overflow is possible: the cotangent of a tiny $x$ is close to $1/x$, so an $x$ with $|x|$ below
/// about $2^{-128}$ has a cotangent beyond the largest [`f32`], and one below about $2^{-1024}$
/// beyond the largest [`f64`]; the result is then $\pm\infty$. No [`f32`] or [`f64`] is close
/// enough to a nonzero multiple of $\pi$ for its cotangent to overflow that way, nor close enough
/// to an odd multiple of $\pi/2$ for it to underflow: the floats are spaced far more widely there
/// than either would take.
///
/// # Worst-case complexity
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use malachite_base::num::basic::traits::NegativeInfinity;
/// use malachite_base::num::float::NiceFloat;
/// use malachite_float::float::arithmetic::cot::primitive_float_cot;
///
/// assert!(primitive_float_cot(f32::NAN).is_nan());
/// assert!(primitive_float_cot(f32::INFINITY).is_nan());
/// assert!(primitive_float_cot(f32::NEGATIVE_INFINITY).is_nan());
/// assert_eq!(
///     NiceFloat(primitive_float_cot(0.0f32)),
///     NiceFloat(f32::INFINITY)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_cot(-0.0f32)),
///     NiceFloat(f32::NEGATIVE_INFINITY)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_cot(1.0f32)),
///     NiceFloat(0.64209265)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_cot(1.0f64)),
///     NiceFloat(0.6420926159343308)
/// );
/// ```
#[inline]
#[allow(clippy::type_repetition_in_bounds)]
pub fn primitive_float_cot<T: PrimitiveFloat>(x: T) -> T
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    emulate_float_to_float_fn(Float::cot_prec, x)
}
