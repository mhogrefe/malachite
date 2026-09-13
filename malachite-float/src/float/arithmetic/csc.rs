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
use crate::float::arithmetic::tan::{MAX_SETTLED_EXPONENT, round_bracket_signed_by};
use crate::{Float, emulate_float_to_float_fn};
use core::cmp::Ordering::{self, Equal, Greater, Less};
use core::cmp::max;
use malachite_base::num::arithmetic::traits::{
    Abs, CeilingLogBase2, Csc, CscAssign, PowerOf2, Reciprocal,
};
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::{
    Infinity as InfinityTrait, NaN as NaNTrait, NegativeInfinity,
};
use malachite_base::num::conversion::traits::{ExactFrom, RoundingFrom};
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{
    self, Ceiling, Down, Exact, Floor, Nearest, Up,
};
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
