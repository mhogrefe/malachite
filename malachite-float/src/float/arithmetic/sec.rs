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

// Port of MPFR's secant. `mpfr_sec` (`sec.c`) instantiates the generic reciprocal template
// (`gen_inverse.h`) with the cosine: the cosine is taken at the working precision, rounded toward
// zero, its reciprocal is rounded to nearest, and the result is certified with two bits of slack,
// inside a Ziv loop. The secant never underflows, since its magnitude is at least 1, but it
// overflows for an input within 2^(-2^30) of an odd multiple of pi/2, which MPFR's wider exponent
// range never sees; a reciprocal at the top of the range is decided from an exact bracket instead.

use crate::InnerFloat::{Finite, Infinity, NaN, Zero};
use crate::float::arithmetic::round_near_x::float_round_near_x;
use crate::float::arithmetic::tan::{MAX_SETTLED_EXPONENT, round_bracket_signed_by};
use crate::{Float, emulate_float_to_float_fn};
use core::cmp::Ordering::{self, Equal, Greater, Less};
use core::cmp::min;
use malachite_base::num::arithmetic::traits::{
    Abs, CeilingLogBase2, PowerOf2, Reciprocal, Sec, SecAssign,
};
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::{
    Infinity as InfinityTrait, NaN as NaNTrait, NegativeInfinity, One,
};
use malachite_base::num::conversion::traits::{ExactFrom, RoundingFrom};
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{self, Ceiling, Down, Exact, Floor, Nearest};
use malachite_nz::natural::arithmetic::float::round::float_can_round;
use malachite_nz::platform::Limb;
use malachite_q::Rational;

// As in mpfr_overflow, with the overflow's sign: the toward-zero modes give the largest finite
// value, and the other modes an infinity.
fn sec_overflow(negative: bool, prec: u64, rm: RoundingMode) -> (Float, Ordering) {
    match (negative, rm) {
        (_, Exact) => panic!("Inexact sec"),
        (false, Floor | Down) => (Float::max_finite_value_with_prec(prec), Less),
        (false, _) => (Float::INFINITY, Greater),
        (true, Ceiling | Down) => (-Float::max_finite_value_with_prec(prec), Greater),
        (true, _) => (Float::NEGATIVE_INFINITY, Less),
    }
}

// Decides sec(x) = 1/c from the cosine rounded toward zero at precision m, by a `Rational` bracket,
// for the cases the `Float` reciprocal cannot settle: it overflowed, or lies within two bits of the
// top of the exponent range, where rounding it to `prec` could still cross the end. Returns `None`
// if the bracket does not decide the rounding, so that the working precision must grow.
fn sec_bracket(c: &Float, m: u64, prec: u64, rm: RoundingMode) -> Option<(Float, Ordering)> {
    let negative = c.is_sign_negative();
    // A cosine that underflowed toward zero is below the smallest positive `Float`, so the secant
    // is above 2^(2^30), beyond the largest finite one.
    if *c == 0u32 {
        return Some(sec_overflow(negative, prec, rm));
    }
    // Rounding toward zero puts the cosine's magnitude in [|c|, |c| + ulp), so the secant's lies in
    // (1/(|c| + ulp), 1/|c|].
    let exp_c = i64::from(c.get_exponent().unwrap());
    let lo = Rational::exact_from(c).abs();
    let hi = &lo + Rational::power_of_2(exp_c - i64::exact_from(m));
    round_bracket_signed_by(negative, hi.reciprocal(), lo.reciprocal(), prec, rm)
}

// This is mpfr_sec from sec.c, MPFR 4.2.2, with the bracket path for results near the top of the
// exponent range.
fn sec_prec_round_normal_ref(x: &Float, prec: u64, rm: RoundingMode) -> (Float, Ordering) {
    assert_ne!(rm, Exact, "Inexact sec");
    let exp_x = i64::from(x.get_exponent().unwrap());
    // sec(x) = 1 + x^2/2 + ..., more precisely |sec(x) - 1| < x^2 for |x| <= 1, so the error is
    // below 2^(2*EXP(x)) and lies above 1.
    //
    // MPFR_FAST_COMPUTE_IF_SMALL_INPUT (y, __gmpfr_one, -2 * MPFR_GET_EXP (x), 0, 1, r, ...)
    let neg_err = -(exp_x << 1);
    if neg_err > 0 {
        let err = u64::exact_from(neg_err);
        if err > prec + 1 {
            // The reference value 1 has precision 1 < err, so float_round_near_x always succeeds.
            // The error bound only has to clear prec + 1; passing an enormous err (a tiny x has one
            // around 2^31) would make float_round_near_x do work proportional to it.
            return float_round_near_x(&Float::ONE, min(err, prec + 2), true, prec, rm).unwrap();
        }
    }
    // Compute initial precision
    let mut m = prec + prec.ceiling_log_base_2() + 3;
    let mut increment = Limb::WIDTH;
    loop {
        // err < 1 ulp, and of a known sign: rounding toward zero puts the cosine below the true one
        // in magnitude
        let c = x.cos_prec_round_ref(m, Down).0;
        // err < 1/2 + 2 < 4 ulps in all, as in algorithms.tex
        let r = (&c).reciprocal();
        // A reciprocal whose exponent is below MAX_SETTLED_EXPONENT can be rounded to any precision
        // without leaving the exponent range, so the `Float` reciprocal settles it; the rest go to
        // the bracket. The secant's magnitude is at least 1, so only the top of the range is in
        // play.
        match r.get_exponent().map(i64::from) {
            Some(e) if e < MAX_SETTLED_EXPONENT => {
                if float_can_round(r.significand_ref().unwrap(), m - 2, prec, rm) {
                    return Float::from_float_prec_round(r, prec, rm);
                }
            }
            _ => {
                if let Some(result) = sec_bracket(&c, m, prec, rm) {
                    return result;
                }
            }
        }
        m += increment;
        increment = m >> 1;
    }
}

impl Float {
    /// Computes $\sec x$, the secant of a [`Float`], rounding the result to the specified precision
    /// and with the specified rounding mode. The [`Float`] is taken by value. An [`Ordering`] is
    /// also returned, indicating whether the rounded secant is less than, equal to, or greater than
    /// the exact secant. Although `NaN`s are not comparable to any [`Float`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \sec x+\varepsilon.
    /// $$
    /// - If $x$ is not finite, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x$ is finite and $m$ is not `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2 |\sec
    ///   x|\rfloor-p+1}$.
    /// - If $x$ is finite and $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2 |\sec
    ///   x|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=\text{NaN}$
    /// - $f(\pm\infty,p,m)=\text{NaN}$
    /// - $f(\pm0.0,p,m)=1.0$
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
    /// Underflow is not possible, since $|\sec x| \geq 1$. Overflow requires an input within
    /// $2^{-2^{30}}$ of an odd multiple of $\pi/2$, which takes more than $2^{30}$ bits of
    /// precision.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::sec_prec`] instead. If you
    /// know that your target precision is the precision of the input, consider using
    /// [`Float::sec_round`] instead. If both of these things are true, consider using
    /// [`Float::sec`] instead.
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
    /// remainder of the $m$-bit input. Unlike most functions, `sec` therefore gets slower as the
    /// magnitude of its input grows, not just as the precision does.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact`, since the secant of a finite nonzero [`Float`] is never exactly
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
    ///     .sec_prec_round(5, Floor);
    /// assert_eq!(c.to_string(), "1.81");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .sec_prec_round(5, Ceiling);
    /// assert_eq!(c.to_string(), "1.88");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .sec_prec_round(5, Nearest);
    /// assert_eq!(c.to_string(), "1.88");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .sec_prec_round(20, Floor);
    /// assert_eq!(c.to_string(), "1.8508148");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .sec_prec_round(20, Ceiling);
    /// assert_eq!(c.to_string(), "1.8508167");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .sec_prec_round(20, Nearest);
    /// assert_eq!(c.to_string(), "1.8508148");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn sec_prec_round(self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        self.sec_prec_round_ref(prec, rm)
    }

    /// Computes $\sec x$, the secant of a [`Float`], rounding the result to the specified precision
    /// and with the specified rounding mode. The [`Float`] is taken by reference. An [`Ordering`]
    /// is also returned, indicating whether the rounded secant is less than, equal to, or greater
    /// than the exact secant. Although `NaN`s are not comparable to any [`Float`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \sec x+\varepsilon.
    /// $$
    /// - If $x$ is not finite, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x$ is finite and $m$ is not `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2 |\sec
    ///   x|\rfloor-p+1}$.
    /// - If $x$ is finite and $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2 |\sec
    ///   x|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=\text{NaN}$
    /// - $f(\pm\infty,p,m)=\text{NaN}$
    /// - $f(\pm0.0,p,m)=1.0$
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
    /// Underflow is not possible, since $|\sec x| \geq 1$. Overflow requires an input within
    /// $2^{-2^{30}}$ of an odd multiple of $\pi/2$, which takes more than $2^{30}$ bits of
    /// precision.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::sec_prec_ref`] instead. If
    /// you know that your target precision is the precision of the input, consider using
    /// [`Float::sec_round_ref`] instead. If both of these things are true, consider using
    /// `(&Float).sec()` instead.
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
    /// remainder of the $m$-bit input. Unlike most functions, `sec` therefore gets slower as the
    /// magnitude of its input grows, not just as the precision does.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact`, since the secant of a finite nonzero [`Float`] is never exactly
    /// representable, or if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = (&Float::from_unsigned_prec(1u32, 100).0).sec_prec_round_ref(5, Floor);
    /// assert_eq!(c.to_string(), "1.81");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = (&Float::from_unsigned_prec(1u32, 100).0).sec_prec_round_ref(5, Ceiling);
    /// assert_eq!(c.to_string(), "1.88");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = (&Float::from_unsigned_prec(1u32, 100).0).sec_prec_round_ref(5, Nearest);
    /// assert_eq!(c.to_string(), "1.88");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = (&Float::from_unsigned_prec(1u32, 100).0).sec_prec_round_ref(20, Floor);
    /// assert_eq!(c.to_string(), "1.8508148");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = (&Float::from_unsigned_prec(1u32, 100).0).sec_prec_round_ref(20, Ceiling);
    /// assert_eq!(c.to_string(), "1.8508167");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = (&Float::from_unsigned_prec(1u32, 100).0).sec_prec_round_ref(20, Nearest);
    /// assert_eq!(c.to_string(), "1.8508148");
    /// assert_eq!(o, Less);
    /// ```
    pub fn sec_prec_round_ref(&self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        assert_ne!(prec, 0);
        match &self.0 {
            NaN | Infinity { .. } => (Self::NAN, Equal),
            // sec(+0) = sec(-0) = 1
            Zero { .. } => (Self::one_prec(prec), Equal),
            Finite { .. } => sec_prec_round_normal_ref(self, prec, rm),
        }
    }

    /// Computes $\sec x$, the secant of a [`Float`], rounding the result to the nearest value of
    /// the specified precision. The [`Float`] is taken by value. An [`Ordering`] is also returned,
    /// indicating whether the rounded secant is less than, equal to, or greater than the exact
    /// secant. Although `NaN`s are not comparable to any [`Float`], whenever this function returns
    /// a `NaN` it also returns `Equal`.
    ///
    /// If the secant is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = \sec x+\varepsilon.
    /// $$
    /// - If $x$ is not finite, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x$ is finite, then $|\varepsilon| < 2^{\lfloor\log_2 |\sec x|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p)=\text{NaN}$
    /// - $f(\pm\infty,p)=\text{NaN}$
    /// - $f(\pm0.0,p)=1.0$
    ///
    /// Overflow:
    /// - If $f(x,p)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,p)\leq -2^{2^{30}-1}$, $-\infty$ is returned instead.
    /// - If $0<f(x,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,p)<0$, $-0.0$ is returned instead.
    ///
    /// Underflow is not possible, since $|\sec x| \geq 1$. Overflow requires an input within
    /// $2^{-2^{30}}$ of an odd multiple of $\pi/2$, which takes more than $2^{30}$ bits of
    /// precision.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::sec_prec_round`] instead. If you know that your target precision is the precision
    /// of the input, consider using [`Float::sec`] instead.
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
    /// remainder of the $m$-bit input. Unlike most functions, `sec` therefore gets slower as the
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
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100).0.sec_prec(5);
    /// assert_eq!(c.to_string(), "1.88");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100).0.sec_prec(20);
    /// assert_eq!(c.to_string(), "1.8508148");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn sec_prec(self, prec: u64) -> (Self, Ordering) {
        self.sec_prec_round(prec, Nearest)
    }

    /// Computes $\sec x$, the secant of a [`Float`], rounding the result to the nearest value of
    /// the specified precision. The [`Float`] is taken by reference. An [`Ordering`] is also
    /// returned, indicating whether the rounded secant is less than, equal to, or greater than the
    /// exact secant. Although `NaN`s are not comparable to any [`Float`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// If the secant is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = \sec x+\varepsilon.
    /// $$
    /// - If $x$ is not finite, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x$ is finite, then $|\varepsilon| < 2^{\lfloor\log_2 |\sec x|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p)=\text{NaN}$
    /// - $f(\pm\infty,p)=\text{NaN}$
    /// - $f(\pm0.0,p)=1.0$
    ///
    /// Overflow:
    /// - If $f(x,p)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,p)\leq -2^{2^{30}-1}$, $-\infty$ is returned instead.
    /// - If $0<f(x,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,p)<0$, $-0.0$ is returned instead.
    ///
    /// Underflow is not possible, since $|\sec x| \geq 1$. Overflow requires an input within
    /// $2^{-2^{30}}$ of an odd multiple of $\pi/2$, which takes more than $2^{30}$ bits of
    /// precision.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::sec_prec_round_ref`] instead. If you know that your target precision is the
    /// precision of the input, consider using `(&Float).sec()` instead.
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
    /// remainder of the $m$-bit input. Unlike most functions, `sec` therefore gets slower as the
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
    /// let (c, o) = (&Float::from_unsigned_prec(1u32, 100).0).sec_prec_ref(5);
    /// assert_eq!(c.to_string(), "1.88");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = (&Float::from_unsigned_prec(1u32, 100).0).sec_prec_ref(20);
    /// assert_eq!(c.to_string(), "1.8508148");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn sec_prec_ref(&self, prec: u64) -> (Self, Ordering) {
        self.sec_prec_round_ref(prec, Nearest)
    }

    /// Computes $\sec x$, the secant of a [`Float`], rounding the result with the specified
    /// rounding mode. The [`Float`] is taken by value. An [`Ordering`] is also returned, indicating
    /// whether the rounded secant is less than, equal to, or greater than the exact secant.
    /// Although `NaN`s are not comparable to any [`Float`], whenever this function returns a `NaN`
    /// it also returns `Equal`.
    ///
    /// The precision of the output is the precision of the input. See [`RoundingMode`] for a
    /// description of the possible rounding modes.
    ///
    /// $$
    /// f(x,m) = \sec x+\varepsilon.
    /// $$
    /// - If $x$ is not finite, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x$ is finite and $m$ is not `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2 |\sec
    ///   x|\rfloor-p+1}$, where $p$ is the precision of the input.
    /// - If $x$ is finite and $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2 |\sec
    ///   x|\rfloor-p}$, where $p$ is the precision of the input.
    ///
    /// If the output has a precision, it is the precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN},m)=\text{NaN}$
    /// - $f(\pm\infty,m)=\text{NaN}$
    /// - $f(\pm0.0,m)=1.0$
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
    /// Underflow is not possible, since $|\sec x| \geq 1$. Overflow requires an input within
    /// $2^{-2^{30}}$ of an odd multiple of $\pi/2$, which takes more than $2^{30}$ bits of
    /// precision.
    ///
    /// If you want to specify an output precision, consider using [`Float::sec_prec_round`]
    /// instead. If you know you'll be using the `Nearest` rounding mode, consider using
    /// [`Float::sec`] instead.
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
    /// e$ bits. Unlike most functions, `sec` therefore gets slower as the magnitude of its input
    /// grows, not just as the precision does.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact`, since the secant of a finite nonzero [`Float`] is never exactly
    /// representable.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100).0.sec_round(Floor);
    /// assert_eq!(c.to_string(), "1.8508157176809256179117532413979");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100).0.sec_round(Ceiling);
    /// assert_eq!(c.to_string(), "1.8508157176809256179117532413995");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100).0.sec_round(Nearest);
    /// assert_eq!(c.to_string(), "1.8508157176809256179117532413979");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn sec_round(self, rm: RoundingMode) -> (Self, Ordering) {
        let prec = self.significant_bits();
        self.sec_prec_round(prec, rm)
    }

    /// Computes $\sec x$, the secant of a [`Float`], rounding the result with the specified
    /// rounding mode. The [`Float`] is taken by reference. An [`Ordering`] is also returned,
    /// indicating whether the rounded secant is less than, equal to, or greater than the exact
    /// secant. Although `NaN`s are not comparable to any [`Float`], whenever this function returns
    /// a `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the precision of the input. See [`RoundingMode`] for a
    /// description of the possible rounding modes.
    ///
    /// $$
    /// f(x,m) = \sec x+\varepsilon.
    /// $$
    /// - If $x$ is not finite, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x$ is finite and $m$ is not `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2 |\sec
    ///   x|\rfloor-p+1}$, where $p$ is the precision of the input.
    /// - If $x$ is finite and $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2 |\sec
    ///   x|\rfloor-p}$, where $p$ is the precision of the input.
    ///
    /// If the output has a precision, it is the precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN},m)=\text{NaN}$
    /// - $f(\pm\infty,m)=\text{NaN}$
    /// - $f(\pm0.0,m)=1.0$
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
    /// Underflow is not possible, since $|\sec x| \geq 1$. Overflow requires an input within
    /// $2^{-2^{30}}$ of an odd multiple of $\pi/2$, which takes more than $2^{30}$ bits of
    /// precision.
    ///
    /// If you want to specify an output precision, consider using [`Float::sec_prec_round_ref`]
    /// instead. If you know you'll be using the `Nearest` rounding mode, consider using
    /// `(&Float).sec()` instead.
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
    /// e$ bits. Unlike most functions, `sec` therefore gets slower as the magnitude of its input
    /// grows, not just as the precision does.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact`, since the secant of a finite nonzero [`Float`] is never exactly
    /// representable.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = (&Float::from_unsigned_prec(1u32, 100).0).sec_round_ref(Floor);
    /// assert_eq!(c.to_string(), "1.8508157176809256179117532413979");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = (&Float::from_unsigned_prec(1u32, 100).0).sec_round_ref(Ceiling);
    /// assert_eq!(c.to_string(), "1.8508157176809256179117532413995");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = (&Float::from_unsigned_prec(1u32, 100).0).sec_round_ref(Nearest);
    /// assert_eq!(c.to_string(), "1.8508157176809256179117532413979");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn sec_round_ref(&self, rm: RoundingMode) -> (Self, Ordering) {
        self.sec_prec_round_ref(self.significant_bits(), rm)
    }

    /// Computes $\sec x$, the secant of a [`Float`], rounding the result to the specified precision
    /// and with the specified rounding mode. The [`Float`] is replaced by the result, and an
    /// [`Ordering`] is returned, indicating whether the rounded secant is less than, equal to, or
    /// greater than the exact secant. Although `NaN`s are not comparable to any [`Float`], whenever
    /// this function sets a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets \sec x+\varepsilon.
    /// $$
    /// - If $x$ is not finite, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x$ is finite and $m$ is not `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2 |\sec
    ///   x|\rfloor-p+1}$.
    /// - If $x$ is finite and $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2 |\sec
    ///   x|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// See the [`Float::sec_prec_round`] documentation for information on special cases and
    /// overflow.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::sec_prec_assign`] instead. If
    /// you know that your target precision is the precision of the input, consider using
    /// [`Float::sec_round_assign`] instead. If both of these things are true, consider using
    /// [`Float::sec_assign`] instead.
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
    /// remainder of the $m$-bit input. Unlike most functions, `sec` therefore gets slower as the
    /// magnitude of its input grows, not just as the precision does.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact`, since the secant of a finite nonzero [`Float`] is never exactly
    /// representable, or if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.sec_prec_round_assign(5, Floor), Less);
    /// assert_eq!(x.to_string(), "1.81");
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.sec_prec_round_assign(5, Ceiling), Greater);
    /// assert_eq!(x.to_string(), "1.88");
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.sec_prec_round_assign(5, Nearest), Greater);
    /// assert_eq!(x.to_string(), "1.88");
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.sec_prec_round_assign(20, Floor), Less);
    /// assert_eq!(x.to_string(), "1.8508148");
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.sec_prec_round_assign(20, Ceiling), Greater);
    /// assert_eq!(x.to_string(), "1.8508167");
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.sec_prec_round_assign(20, Nearest), Less);
    /// assert_eq!(x.to_string(), "1.8508148");
    /// ```
    #[inline]
    pub fn sec_prec_round_assign(&mut self, prec: u64, rm: RoundingMode) -> Ordering {
        let o;
        (*self, o) = self.sec_prec_round_ref(prec, rm);
        o
    }

    /// Computes $\sec x$, the secant of a [`Float`], rounding the result to the nearest value of
    /// the specified precision. The [`Float`] is replaced by the result, and an [`Ordering`] is
    /// returned, indicating whether the rounded secant is less than, equal to, or greater than the
    /// exact secant. Although `NaN`s are not comparable to any [`Float`], whenever this function
    /// sets a `NaN` it also returns `Equal`.
    ///
    /// If the secant is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// x \gets \sec x+\varepsilon.
    /// $$
    /// - If $x$ is not finite, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x$ is finite, then $|\varepsilon| < 2^{\lfloor\log_2 |\sec x|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// See the [`Float::sec_prec`] documentation for information on special cases and overflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::sec_prec_round_assign`] instead. If you know that your target precision is the
    /// precision of the input, consider using [`Float::sec_assign`] instead.
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
    /// remainder of the $m$-bit input. Unlike most functions, `sec` therefore gets slower as the
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
    /// assert_eq!(x.sec_prec_assign(5), Greater);
    /// assert_eq!(x.to_string(), "1.88");
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.sec_prec_assign(20), Less);
    /// assert_eq!(x.to_string(), "1.8508148");
    /// ```
    #[inline]
    pub fn sec_prec_assign(&mut self, prec: u64) -> Ordering {
        self.sec_prec_round_assign(prec, Nearest)
    }

    /// Computes $\sec x$, the secant of a [`Float`], rounding the result with the specified
    /// rounding mode. The [`Float`] is replaced by the result, and an [`Ordering`] is returned,
    /// indicating whether the rounded secant is less than, equal to, or greater than the exact
    /// secant. Although `NaN`s are not comparable to any [`Float`], whenever this function sets a
    /// `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the precision of the input. See [`RoundingMode`] for a
    /// description of the possible rounding modes.
    ///
    /// $$
    /// x \gets \sec x+\varepsilon.
    /// $$
    /// - If $x$ is not finite, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x$ is finite and $m$ is not `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2 |\sec
    ///   x|\rfloor-p+1}$, where $p$ is the precision of the input.
    /// - If $x$ is finite and $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2 |\sec
    ///   x|\rfloor-p}$, where $p$ is the precision of the input.
    ///
    /// If the output has a precision, it is the precision of the input.
    ///
    /// See the [`Float::sec_round`] documentation for information on special cases and overflow.
    ///
    /// If you want to specify an output precision, consider using [`Float::sec_prec_round_assign`]
    /// instead. If you know you'll be using the `Nearest` rounding mode, consider using
    /// [`Float::sec_assign`] instead.
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
    /// e$ bits. Unlike most functions, `sec` therefore gets slower as the magnitude of its input
    /// grows, not just as the precision does.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact`, since the secant of a finite nonzero [`Float`] is never exactly
    /// representable.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.sec_round_assign(Floor), Less);
    /// assert_eq!(x.to_string(), "1.8508157176809256179117532413979");
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.sec_round_assign(Ceiling), Greater);
    /// assert_eq!(x.to_string(), "1.8508157176809256179117532413995");
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.sec_round_assign(Nearest), Less);
    /// assert_eq!(x.to_string(), "1.8508157176809256179117532413979");
    /// ```
    #[inline]
    pub fn sec_round_assign(&mut self, rm: RoundingMode) -> Ordering {
        let prec = self.significant_bits();
        self.sec_prec_round_assign(prec, rm)
    }
}

impl Sec for Float {
    type Output = Self;

    /// Computes $\sec x$, the secant of a [`Float`], taking it by value.
    ///
    /// If the output has a precision, it is the precision of the input. If the secant is
    /// equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s in
    /// its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// $$
    /// f(x) = \sec x+\varepsilon.
    /// $$
    /// - If $x$ is not finite, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x$ is finite, then $|\varepsilon| < 2^{\lfloor\log_2 |\sec x|\rfloor-p}$, where $p$ is
    ///   the precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN})=\text{NaN}$
    /// - $f(\pm\infty)=\text{NaN}$
    /// - $f(\pm0.0)=1.0$
    ///
    /// See the [`Float::sec_round`] documentation for information on overflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using [`Float::sec_round`]
    /// instead. If you want to specify the output precision, consider using [`Float::sec_prec`]. If
    /// you want both of these things, consider using [`Float::sec_prec_round`].
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
    /// e$ bits. Unlike most functions, `sec` therefore gets slower as the magnitude of its input
    /// grows, not just as the precision does.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Sec;
    /// use malachite_base::num::basic::traits::*;
    /// use malachite_float::Float;
    ///
    /// assert!(Float::NAN.sec().is_nan());
    /// assert!(Float::INFINITY.sec().is_nan());
    /// assert!(Float::NEGATIVE_INFINITY.sec().is_nan());
    /// assert_eq!(Float::ZERO.sec().to_string(), "1.0");
    /// assert_eq!(Float::NEGATIVE_ZERO.sec().to_string(), "1.0");
    /// assert_eq!(
    ///     Float::from_unsigned_prec(1u32, 100).0.sec().to_string(),
    ///     "1.8508157176809256179117532413979"
    /// );
    /// assert_eq!(
    ///     Float::from_unsigned_prec(100u32, 100).0.sec().to_string(),
    ///     "1.1596638229046938325514044465873"
    /// );
    /// ```
    #[inline]
    fn sec(self) -> Self {
        let prec = self.significant_bits();
        self.sec_prec_round(prec, Nearest).0
    }
}

impl Sec for &Float {
    type Output = Float;

    /// Computes $\sec x$, the secant of a [`Float`], taking it by reference.
    ///
    /// If the output has a precision, it is the precision of the input. If the secant is
    /// equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s in
    /// its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// $$
    /// f(x) = \sec x+\varepsilon.
    /// $$
    /// - If $x$ is not finite, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x$ is finite, then $|\varepsilon| < 2^{\lfloor\log_2 |\sec x|\rfloor-p}$, where $p$ is
    ///   the precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN})=\text{NaN}$
    /// - $f(\pm\infty)=\text{NaN}$
    /// - $f(\pm0.0)=1.0$
    ///
    /// See the [`Float::sec_round`] documentation for information on overflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::sec_round_ref`] instead. If you want to specify the output precision, consider
    /// using [`Float::sec_prec_ref`]. If you want both of these things, consider using
    /// [`Float::sec_prec_round_ref`].
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
    /// e$ bits. Unlike most functions, `sec` therefore gets slower as the magnitude of its input
    /// grows, not just as the precision does.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Sec;
    /// use malachite_base::num::basic::traits::*;
    /// use malachite_float::Float;
    ///
    /// assert!(Float::NAN.sec().is_nan());
    /// assert!(Float::INFINITY.sec().is_nan());
    /// assert!(Float::NEGATIVE_INFINITY.sec().is_nan());
    /// assert_eq!(Float::ZERO.sec().to_string(), "1.0");
    /// assert_eq!(Float::NEGATIVE_ZERO.sec().to_string(), "1.0");
    /// assert_eq!(
    ///     (&Float::from_unsigned_prec(1u32, 100).0).sec().to_string(),
    ///     "1.8508157176809256179117532413979"
    /// );
    /// assert_eq!(
    ///     (&Float::from_unsigned_prec(100u32, 100).0)
    ///         .sec()
    ///         .to_string(),
    ///     "1.1596638229046938325514044465873"
    /// );
    /// ```
    #[inline]
    fn sec(self) -> Float {
        self.sec_prec_round_ref(self.significant_bits(), Nearest).0
    }
}

impl SecAssign for Float {
    /// Computes $\sec x$, the secant of a [`Float`], in place.
    ///
    /// If the output has a precision, it is the precision of the input. If the secant is
    /// equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s in
    /// its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// $$
    /// x \gets \sec x+\varepsilon.
    /// $$
    /// - If $x$ is not finite, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x$ is finite, then $|\varepsilon| < 2^{\lfloor\log_2 |\sec x|\rfloor-p}$, where $p$ is
    ///   the precision of the input.
    ///
    /// See the [`Float::sec`] documentation for information on special cases and overflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::sec_round_assign`] instead. If you want to specify the output precision, consider
    /// using [`Float::sec_prec_assign`]. If you want both of these things, consider using
    /// [`Float::sec_prec_round_assign`].
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
    /// e$ bits. Unlike most functions, `sec` therefore gets slower as the magnitude of its input
    /// grows, not just as the precision does.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::SecAssign;
    /// use malachite_base::num::basic::traits::*;
    /// use malachite_float::Float;
    ///
    /// let mut x = Float::NAN;
    /// x.sec_assign();
    /// assert!(x.is_nan());
    ///
    /// let mut x = Float::INFINITY;
    /// x.sec_assign();
    /// assert!(x.is_nan());
    ///
    /// let mut x = Float::NEGATIVE_INFINITY;
    /// x.sec_assign();
    /// assert!(x.is_nan());
    ///
    /// let mut x = Float::ZERO;
    /// x.sec_assign();
    /// assert_eq!(x.to_string(), "1.0");
    ///
    /// let mut x = Float::NEGATIVE_ZERO;
    /// x.sec_assign();
    /// assert_eq!(x.to_string(), "1.0");
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// x.sec_assign();
    /// assert_eq!(x.to_string(), "1.8508157176809256179117532413979");
    ///
    /// let mut x = Float::from_unsigned_prec(100u32, 100).0;
    /// x.sec_assign();
    /// assert_eq!(x.to_string(), "1.1596638229046938325514044465873");
    /// ```
    #[inline]
    fn sec_assign(&mut self) {
        let prec = self.significant_bits();
        self.sec_prec_round_assign(prec, Nearest);
    }
}

/// Computes $\sec x$, the secant of a primitive float, correctly rounded. Neither the standard
/// library nor `libm` provides a secant.
///
/// $$
/// f(x) = \sec x+\varepsilon.
/// $$
/// - If $x$ is not finite, $\varepsilon$ may be ignored or assumed to be 0.
/// - If $x$ is finite, then $|\varepsilon| < 2^{\lfloor\log_2 |\sec x|\rfloor-p}$, where $p$ is the
///   precision of the output (24 if `T` is a [`f32`] and 53 if `T` is a [`f64`]).
///
/// Special cases:
/// - $f(\text{NaN})=\text{NaN}$
/// - $f(\pm\infty)=\text{NaN}$
/// - $f(\pm0.0)=1.0$
///
/// Overflow is not possible: no [`f32`] or [`f64`] is close enough to an odd multiple of $\pi/2$
/// for its secant to exceed the largest finite value (the largest secant of an [`f64`], like the
/// largest tangent, is below $2^{55}$). The result is never subnormal, since $|\sec x| \geq 1$.
///
/// # Worst-case complexity
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use malachite_base::num::basic::traits::NegativeInfinity;
/// use malachite_base::num::float::NiceFloat;
/// use malachite_float::float::arithmetic::sec::primitive_float_sec;
///
/// assert!(primitive_float_sec(f32::NAN).is_nan());
/// assert!(primitive_float_sec(f32::INFINITY).is_nan());
/// assert!(primitive_float_sec(f32::NEGATIVE_INFINITY).is_nan());
/// assert_eq!(NiceFloat(primitive_float_sec(0.0f32)), NiceFloat(1.0));
/// assert_eq!(NiceFloat(primitive_float_sec(-0.0f32)), NiceFloat(1.0));
/// assert_eq!(NiceFloat(primitive_float_sec(1.0f32)), NiceFloat(1.8508158));
/// assert_eq!(
///     NiceFloat(primitive_float_sec(1.0f64)),
///     NiceFloat(1.8508157176809257)
/// );
/// ```
#[inline]
#[allow(clippy::type_repetition_in_bounds)]
pub fn primitive_float_sec<T: PrimitiveFloat>(x: T) -> T
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    emulate_float_to_float_fn(Float::sec_prec, x)
}
