// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::InnerFloat::{Finite, Infinity, NaN, Zero};
use crate::float::arithmetic::asinh::round_with_error;
use crate::{Float, emulate_float_to_float_fn, emulate_rational_to_float_fn};
use core::cmp::Ordering::{self, *};
use malachite_base::fail_on_untested_path;
use malachite_base::num::arithmetic::traits::{
    Asech, AsechAssign, CeilingLogBase2, Ln, Ln1PlusX, Reciprocal, Sign, Sqrt,
};
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::{
    Infinity as InfinityTrait, NaN as NaNTrait, One, Zero as ZeroTrait,
};
use malachite_base::num::conversion::traits::{ExactFrom, RoundingFrom};
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_nz::platform::Limb;
use malachite_q::Rational;

// Computes asech(x) = acosh(1/x) = ln((1 + sqrt(1 - x^2))/x) for a `Float` x with 0 < x < 1. MPFR
// has no asech, so this is Malachite's own algorithm. Computing 1/x first would be ill-conditioned
// near x = 1, where acosh has an infinite derivative, and would overflow for the smallest `Float`s,
// so the two halves of the domain are handled differently, each without cancellation:
//
// - For 1/2 <= x < 1, let t = 1 - x, which is exact (Sterbenz). Then 1 - x^2 = t(1 + x), and
//   asech(x) = ln(1 + u) with u = (t + sqrt(t(1 + x)))/x, all of whose terms are positive. Each of
//   the five operations forming u has a relative error of at most 2^-wp, so u's is below 4 * 2^-wp
//   (the square root halves the error of its argument). Since ln(1+u) >= u/(1+u), that moves
//   ln(1+u) by less than 4.02 * 2^-wp ln(1+u), about 4 ulps, and its own rounding adds 1/2 ulp:
//   under 2^3 ulps.
// - For 0 < x < 1/2, asech(x) = ln(1 + s) - ln(x) with s = sqrt((1 - x)(1 + x)) in (0.86, 1]. Both
//   terms are positive, ln(1 + s) < 0.7 < -ln(x), and the sum exceeds 1.3, so its ulp is at least
//   2^(1-wp). s has relative error below 2.5 * 2^-wp, which moves ln(1 + s) by less than 1.3 *
//   2^-wp; with the three roundings (of ln(1 + s), of ln(x), and of the sum), each at most half an
//   ulp of the sum, the error is below 3 ulps: under 2^3 ulps again.
//
// If t(1 + x) underflows, which needs an x with a precision above 2^30, sqrt(t) sqrt(1 + x) is used
// instead; its relative error is below 2^-wp more, which the bound absorbs.
fn asech_prec_round_normal_ref(x: &Float, prec: u64, rm: RoundingMode) -> (Float, Ordering) {
    assert_ne!(rm, Exact, "Inexact asech");
    let near_one = x.get_exponent().unwrap() == 0; // 1/2 <= x < 1
    // t = 1 - x, exact at x's precision for 1/2 <= x < 1
    let t = near_one.then(|| {
        let (t, o) = Float::ONE.sub_prec_ref_ref(x, x.get_prec().unwrap());
        assert_eq!(o, Equal);
        t
    });
    let exp_x = i64::from(x.get_exponent().unwrap());
    let mut working_prec = prec + prec.ceiling_log_base_2() + 10;
    let mut increment = Limb::WIDTH;
    loop {
        let t = if let Some(t) = &t {
            // 1 + x
            let one_plus_x = x.add_prec_ref_val(Float::ONE, working_prec).0;
            let s = if t.get_exponent().unwrap() > const { Float::MIN_EXPONENT + 1 } {
                // sqrt(t(1 + x))
                t.mul_prec_ref_val(one_plus_x, working_prec).0.sqrt()
            } else {
                fail_on_untested_path("asech_prec_round_normal_ref, t(1 + x) may underflow");
                t.sqrt_prec_ref(working_prec).0 * one_plus_x.sqrt()
            };
            // ln(1 + (t + s)/x)
            s.add_prec_val_ref(t, working_prec)
                .0
                .div_prec_val_ref(x, working_prec)
                .0
                .ln_1_plus_x()
        } else if exp_x << 1 <= 1 - i64::exact_from(working_prec) {
            // x^2 < 2^(1-wp), so ln(1 + sqrt(1 - x^2)) = ln 2 - c with 0 < c < x^2, which is at
            // most 1 ulp of the sum (whose ulp is at least 2^(1-wp)): with the three roundings the
            // error stays below 2.5 ulps, under 2^3 ulps
            Float::ln_2_prec(working_prec).0 - x.ln_prec_ref(working_prec).0
        } else {
            // s = sqrt((1 - x)(1 + x))
            let s = (Float::ONE.sub_prec_val_ref(x, working_prec).0
                * x.add_prec_ref_val(Float::ONE, working_prec).0)
                .sqrt();
            // ln(1 + s) - ln(x)
            (s + Float::ONE).ln() - x.ln_prec_ref(working_prec).0
        };
        if let Some(result) = round_with_error(t, working_prec, 3, prec, rm) {
            return result;
        }
        working_prec += increment;
        increment = working_prec >> 1;
    }
}

impl Float {
    /// Computes $\operatorname{asech} x$, the inverse hyperbolic secant of a [`Float`], rounding
    /// the result to the specified precision and with the specified rounding mode. The [`Float`] is
    /// taken by value. An [`Ordering`] is also returned, indicating whether the rounded inverse
    /// hyperbolic secant is less than, equal to, or greater than the exact inverse hyperbolic
    /// secant. Although `NaN`s are not comparable to any [`Float`], whenever this function returns
    /// a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \operatorname{asech} x+\varepsilon.
    /// $$
    /// - If $\operatorname{asech} x$ is infinite, zero, or NaN, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $\operatorname{asech} x$ is finite and nonzero, and $m$ is not `Nearest`, then
    ///   $|\varepsilon| < 2^{\lfloor\log_2 \operatorname{asech} x\rfloor-p+1}$.
    /// - If $\operatorname{asech} x$ is finite and nonzero, and $m$ is `Nearest`, then
    ///   $|\varepsilon| \leq 2^{\lfloor\log_2 \operatorname{asech} x\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=\text{NaN}$
    /// - $f(\pm\infty,p,m)=\text{NaN}$
    /// - $f(\pm0.0,p,m)=\infty$
    /// - $f(1,p,m)=0.0$
    /// - $f(x,p,m)=\text{NaN}$ if $x<0$ or $x>1$
    ///
    /// The result never overflows, since $\operatorname{asech} x < \ln(2/x) < 2^{30}$ for every
    /// positive [`Float`] $x$. It underflows only for an $x$ within $2^{-2^{31}}$ of 1, since
    /// $\operatorname{asech}(1-t) > \sqrt t$, and such an $x$ needs a precision above $2^{31}$.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::asech_prec`] instead. If you
    /// know that your target precision is the precision of the input, consider using
    /// [`Float::asech_round`] instead. If both of these things are true, consider using
    /// [`Float::asech`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec,
    /// self.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` and `self` is finite, nonzero, and less than 1 in absolute value,
    /// since the inverse hyperbolic secant of such a [`Float`] is never exactly representable, or
    /// if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = (Float::one_prec(100) >> 1u32).asech_prec_round(5, Floor);
    /// assert_eq!(c.to_string(), "1.31");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = (Float::one_prec(100) >> 1u32).asech_prec_round(5, Ceiling);
    /// assert_eq!(c.to_string(), "1.38");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = (Float::one_prec(100) >> 1u32).asech_prec_round(5, Nearest);
    /// assert_eq!(c.to_string(), "1.31");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = (Float::one_prec(100) >> 1u32).asech_prec_round(20, Floor);
    /// assert_eq!(c.to_string(), "1.3169575");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = (Float::one_prec(100) >> 1u32).asech_prec_round(20, Ceiling);
    /// assert_eq!(c.to_string(), "1.3169594");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = (Float::one_prec(100) >> 1u32).asech_prec_round(20, Nearest);
    /// assert_eq!(c.to_string(), "1.3169575");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn asech_prec_round(self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        self.asech_prec_round_ref(prec, rm)
    }

    /// Computes $\operatorname{asech} x$, the inverse hyperbolic secant of a [`Float`], rounding
    /// the result to the specified precision and with the specified rounding mode. The [`Float`] is
    /// taken by reference. An [`Ordering`] is also returned, indicating whether the rounded inverse
    /// hyperbolic secant is less than, equal to, or greater than the exact inverse hyperbolic
    /// secant. Although `NaN`s are not comparable to any [`Float`], whenever this function returns
    /// a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \operatorname{asech} x+\varepsilon.
    /// $$
    /// - If $\operatorname{asech} x$ is infinite, zero, or NaN, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $\operatorname{asech} x$ is finite and nonzero, and $m$ is not `Nearest`, then
    ///   $|\varepsilon| < 2^{\lfloor\log_2 \operatorname{asech} x\rfloor-p+1}$.
    /// - If $\operatorname{asech} x$ is finite and nonzero, and $m$ is `Nearest`, then
    ///   $|\varepsilon| \leq 2^{\lfloor\log_2 \operatorname{asech} x\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=\text{NaN}$
    /// - $f(\pm\infty,p,m)=\text{NaN}$
    /// - $f(\pm0.0,p,m)=\infty$
    /// - $f(1,p,m)=0.0$
    /// - $f(x,p,m)=\text{NaN}$ if $x<0$ or $x>1$
    ///
    /// The result never overflows, since $\operatorname{asech} x < \ln(2/x) < 2^{30}$ for every
    /// positive [`Float`] $x$. It underflows only for an $x$ within $2^{-2^{31}}$ of 1, since
    /// $\operatorname{asech}(1-t) > \sqrt t$, and such an $x$ needs a precision above $2^{31}$.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::asech_prec_ref`] instead. If
    /// you know that your target precision is the precision of the input, consider using
    /// [`Float::asech_round_ref`] instead. If both of these things are true, consider using
    /// `(&Float).asech()` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec,
    /// self.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` and `self` is finite, nonzero, and less than 1 in absolute value,
    /// since the inverse hyperbolic secant of such a [`Float`] is never exactly representable, or
    /// if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = (Float::one_prec(100) >> 1u32).asech_prec_round_ref(5, Floor);
    /// assert_eq!(c.to_string(), "1.31");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = (Float::one_prec(100) >> 1u32).asech_prec_round_ref(5, Ceiling);
    /// assert_eq!(c.to_string(), "1.38");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = (Float::one_prec(100) >> 1u32).asech_prec_round_ref(5, Nearest);
    /// assert_eq!(c.to_string(), "1.31");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = (Float::one_prec(100) >> 1u32).asech_prec_round_ref(20, Floor);
    /// assert_eq!(c.to_string(), "1.3169575");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = (Float::one_prec(100) >> 1u32).asech_prec_round_ref(20, Ceiling);
    /// assert_eq!(c.to_string(), "1.3169594");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = (Float::one_prec(100) >> 1u32).asech_prec_round_ref(20, Nearest);
    /// assert_eq!(c.to_string(), "1.3169575");
    /// assert_eq!(o, Less);
    /// ```
    pub fn asech_prec_round_ref(&self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        assert_ne!(prec, 0);
        match &self.0 {
            // asech is NaN for NaN, both infinities, and everything below 0
            NaN | Infinity { .. } | Finite { sign: false, .. } => (Self::NAN, Equal),
            // asech(+/-0) = +Inf, the limit from above; like ln, asech treats -0 as 0
            Zero { .. } => (Self::INFINITY, Equal),
            Finite { .. } => match self.partial_cmp(&1u32).unwrap() {
                Less => asech_prec_round_normal_ref(self, prec, rm),
                // asech(1) = +0
                Equal => (Self::ZERO, Equal),
                // asech is NaN above 1
                Greater => (Self::NAN, Equal),
            },
        }
    }

    /// Computes $\operatorname{asech} x$, the inverse hyperbolic secant of a [`Float`], rounding
    /// the result to the nearest value of the specified precision. The [`Float`] is taken by value.
    /// An [`Ordering`] is also returned, indicating whether the rounded inverse hyperbolic secant
    /// is less than, equal to, or greater than the exact inverse hyperbolic secant. Although `NaN`s
    /// are not comparable to any [`Float`], whenever this function returns a `NaN` it also returns
    /// `Equal`.
    ///
    /// If the inverse hyperbolic secant is equidistant from two [`Float`]s with the specified
    /// precision, the [`Float`] with fewer 1s in its binary expansion is chosen. See
    /// [`RoundingMode`] for a description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = \operatorname{asech} x+\varepsilon.
    /// $$
    /// - If $\operatorname{asech} x$ is infinite, zero, or NaN, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $\operatorname{asech} x$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2
    ///   \operatorname{asech} x\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p)=\text{NaN}$
    /// - $f(\pm\infty,p)=\text{NaN}$
    /// - $f(\pm0.0,p)=\infty$
    /// - $f(1,p)=0.0$
    /// - $f(x,p)=\text{NaN}$ if $x<0$ or $x>1$
    ///
    /// The result never overflows, since $\operatorname{asech} x < \ln(2/x) < 2^{30}$ for every
    /// positive [`Float`] $x$. It underflows only for an $x$ within $2^{-2^{31}}$ of 1, since
    /// $\operatorname{asech}(1-t) > \sqrt t$, and such an $x$ needs a precision above $2^{31}$.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::asech_prec_round`] instead. If you know that your target precision is the precision
    /// of the input, consider using [`Float::asech`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec,
    /// self.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = (Float::one_prec(100) >> 1u32).asech_prec(5);
    /// assert_eq!(c.to_string(), "1.31");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = (Float::one_prec(100) >> 1u32).asech_prec(20);
    /// assert_eq!(c.to_string(), "1.3169575");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn asech_prec(self, prec: u64) -> (Self, Ordering) {
        self.asech_prec_round(prec, Nearest)
    }

    /// Computes $\operatorname{asech} x$, the inverse hyperbolic secant of a [`Float`], rounding
    /// the result to the nearest value of the specified precision. The [`Float`] is taken by
    /// reference. An [`Ordering`] is also returned, indicating whether the rounded inverse
    /// hyperbolic secant is less than, equal to, or greater than the exact inverse hyperbolic
    /// secant. Although `NaN`s are not comparable to any [`Float`], whenever this function returns
    /// a `NaN` it also returns `Equal`.
    ///
    /// If the inverse hyperbolic secant is equidistant from two [`Float`]s with the specified
    /// precision, the [`Float`] with fewer 1s in its binary expansion is chosen. See
    /// [`RoundingMode`] for a description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = \operatorname{asech} x+\varepsilon.
    /// $$
    /// - If $\operatorname{asech} x$ is infinite, zero, or NaN, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $\operatorname{asech} x$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2
    ///   \operatorname{asech} x\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p)=\text{NaN}$
    /// - $f(\pm\infty,p)=\text{NaN}$
    /// - $f(\pm0.0,p)=\infty$
    /// - $f(1,p)=0.0$
    /// - $f(x,p)=\text{NaN}$ if $x<0$ or $x>1$
    ///
    /// The result never overflows, since $\operatorname{asech} x < \ln(2/x) < 2^{30}$ for every
    /// positive [`Float`] $x$. It underflows only for an $x$ within $2^{-2^{31}}$ of 1, since
    /// $\operatorname{asech}(1-t) > \sqrt t$, and such an $x$ needs a precision above $2^{31}$.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::asech_prec_round_ref`] instead. If you know that your target precision is the
    /// precision of the input, consider using `(&Float).asech()` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec,
    /// self.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = (Float::one_prec(100) >> 1u32).asech_prec_ref(5);
    /// assert_eq!(c.to_string(), "1.31");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = (Float::one_prec(100) >> 1u32).asech_prec_ref(20);
    /// assert_eq!(c.to_string(), "1.3169575");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn asech_prec_ref(&self, prec: u64) -> (Self, Ordering) {
        self.asech_prec_round_ref(prec, Nearest)
    }

    /// Computes $\operatorname{asech} x$, the inverse hyperbolic secant of a [`Float`], rounding
    /// the result with the specified rounding mode. The [`Float`] is taken by value. An
    /// [`Ordering`] is also returned, indicating whether the rounded inverse hyperbolic secant is
    /// less than, equal to, or greater than the exact inverse hyperbolic secant. Although `NaN`s
    /// are not comparable to any [`Float`], whenever this function returns a `NaN` it also returns
    /// `Equal`.
    ///
    /// The precision of the output is the precision of the input. See [`RoundingMode`] for a
    /// description of the possible rounding modes.
    ///
    /// $$
    /// f(x,m) = \operatorname{asech} x+\varepsilon.
    /// $$
    /// - If $\operatorname{asech} x$ is infinite, zero, or NaN, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $\operatorname{asech} x$ is finite and nonzero, and $m$ is not `Nearest`, then
    ///   $|\varepsilon| < 2^{\lfloor\log_2 \operatorname{asech} x\rfloor-p+1}$, where $p$ is the
    ///   precision of the input.
    /// - If $\operatorname{asech} x$ is finite and nonzero, and $m$ is `Nearest`, then
    ///   $|\varepsilon| \leq 2^{\lfloor\log_2 \operatorname{asech} x\rfloor-p}$, where $p$ is the
    ///   precision of the input.
    ///
    /// If the output has a precision, it is the precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN},m)=\text{NaN}$
    /// - $f(\pm\infty,m)=\text{NaN}$
    /// - $f(\pm0.0,m)=\infty$
    /// - $f(1,m)=0.0$
    /// - $f(x,m)=\text{NaN}$ if $x<0$ or $x>1$
    ///
    /// The result never overflows, since $\operatorname{asech} x < \ln(2/x) < 2^{30}$ for every
    /// positive [`Float`] $x$. It underflows only for an $x$ within $2^{-2^{31}}$ of 1, since
    /// $\operatorname{asech}(1-t) > \sqrt t$, and such an $x$ needs a precision above $2^{31}$.
    ///
    /// If you want to specify an output precision, consider using [`Float::asech_prec_round`]
    /// instead. If you know you'll be using the `Nearest` rounding mode, consider using
    /// [`Float::asech`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` and `self` is finite, nonzero, and less than 1 in absolute value,
    /// since the inverse hyperbolic secant of such a [`Float`] is never exactly representable.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = (Float::one_prec(100) >> 1u32).asech_round(Floor);
    /// assert_eq!(c.to_string(), "1.3169578969248167086250463473073");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = (Float::one_prec(100) >> 1u32).asech_round(Ceiling);
    /// assert_eq!(c.to_string(), "1.3169578969248167086250463473089");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = (Float::one_prec(100) >> 1u32).asech_round(Nearest);
    /// assert_eq!(c.to_string(), "1.3169578969248167086250463473073");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn asech_round(self, rm: RoundingMode) -> (Self, Ordering) {
        let prec = self.significant_bits();
        self.asech_prec_round(prec, rm)
    }

    /// Computes $\operatorname{asech} x$, the inverse hyperbolic secant of a [`Float`], rounding
    /// the result with the specified rounding mode. The [`Float`] is taken by reference. An
    /// [`Ordering`] is also returned, indicating whether the rounded inverse hyperbolic secant is
    /// less than, equal to, or greater than the exact inverse hyperbolic secant. Although `NaN`s
    /// are not comparable to any [`Float`], whenever this function returns a `NaN` it also returns
    /// `Equal`.
    ///
    /// The precision of the output is the precision of the input. See [`RoundingMode`] for a
    /// description of the possible rounding modes.
    ///
    /// $$
    /// f(x,m) = \operatorname{asech} x+\varepsilon.
    /// $$
    /// - If $\operatorname{asech} x$ is infinite, zero, or NaN, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $\operatorname{asech} x$ is finite and nonzero, and $m$ is not `Nearest`, then
    ///   $|\varepsilon| < 2^{\lfloor\log_2 \operatorname{asech} x\rfloor-p+1}$, where $p$ is the
    ///   precision of the input.
    /// - If $\operatorname{asech} x$ is finite and nonzero, and $m$ is `Nearest`, then
    ///   $|\varepsilon| \leq 2^{\lfloor\log_2 \operatorname{asech} x\rfloor-p}$, where $p$ is the
    ///   precision of the input.
    ///
    /// If the output has a precision, it is the precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN},m)=\text{NaN}$
    /// - $f(\pm\infty,m)=\text{NaN}$
    /// - $f(\pm0.0,m)=\infty$
    /// - $f(1,m)=0.0$
    /// - $f(x,m)=\text{NaN}$ if $x<0$ or $x>1$
    ///
    /// The result never overflows, since $\operatorname{asech} x < \ln(2/x) < 2^{30}$ for every
    /// positive [`Float`] $x$. It underflows only for an $x$ within $2^{-2^{31}}$ of 1, since
    /// $\operatorname{asech}(1-t) > \sqrt t$, and such an $x$ needs a precision above $2^{31}$.
    ///
    /// If you want to specify an output precision, consider using [`Float::asech_prec_round_ref`]
    /// instead. If you know you'll be using the `Nearest` rounding mode, consider using
    /// `(&Float).asech()` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` and `self` is finite, nonzero, and less than 1 in absolute value,
    /// since the inverse hyperbolic secant of such a [`Float`] is never exactly representable.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = (Float::one_prec(100) >> 1u32).asech_round_ref(Floor);
    /// assert_eq!(c.to_string(), "1.3169578969248167086250463473073");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = (Float::one_prec(100) >> 1u32).asech_round_ref(Ceiling);
    /// assert_eq!(c.to_string(), "1.3169578969248167086250463473089");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = (Float::one_prec(100) >> 1u32).asech_round_ref(Nearest);
    /// assert_eq!(c.to_string(), "1.3169578969248167086250463473073");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn asech_round_ref(&self, rm: RoundingMode) -> (Self, Ordering) {
        self.asech_prec_round_ref(self.significant_bits(), rm)
    }

    /// Computes $\operatorname{asech} x$, the inverse hyperbolic secant of a [`Float`], rounding
    /// the result to the specified precision and with the specified rounding mode. The [`Float`] is
    /// replaced by the result, and an [`Ordering`] is returned, indicating whether the rounded
    /// inverse hyperbolic secant is less than, equal to, or greater than the exact inverse
    /// hyperbolic secant. Although `NaN`s are not comparable to any [`Float`], whenever this
    /// function sets a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets \operatorname{asech} x+\varepsilon.
    /// $$
    /// - If $\operatorname{asech} x$ is infinite, zero, or NaN, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $\operatorname{asech} x$ is finite and nonzero, and $m$ is not `Nearest`, then
    ///   $|\varepsilon| < 2^{\lfloor\log_2 \operatorname{asech} x\rfloor-p+1}$.
    /// - If $\operatorname{asech} x$ is finite and nonzero, and $m$ is `Nearest`, then
    ///   $|\varepsilon| \leq 2^{\lfloor\log_2 \operatorname{asech} x\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// See the [`Float::asech_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::asech_prec_assign`] instead.
    /// If you know that your target precision is the precision of the input, consider using
    /// [`Float::asech_round_assign`] instead. If both of these things are true, consider using
    /// [`Float::asech_assign`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec,
    /// self.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` and `self` is finite, nonzero, and less than 1 in absolute value,
    /// since the inverse hyperbolic secant of such a [`Float`] is never exactly representable, or
    /// if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::one_prec(100) >> 1u32;
    /// assert_eq!(x.asech_prec_round_assign(5, Floor), Less);
    /// assert_eq!(x.to_string(), "1.31");
    ///
    /// let mut x = Float::one_prec(100) >> 1u32;
    /// assert_eq!(x.asech_prec_round_assign(5, Ceiling), Greater);
    /// assert_eq!(x.to_string(), "1.38");
    ///
    /// let mut x = Float::one_prec(100) >> 1u32;
    /// assert_eq!(x.asech_prec_round_assign(5, Nearest), Less);
    /// assert_eq!(x.to_string(), "1.31");
    ///
    /// let mut x = Float::one_prec(100) >> 1u32;
    /// assert_eq!(x.asech_prec_round_assign(20, Floor), Less);
    /// assert_eq!(x.to_string(), "1.3169575");
    ///
    /// let mut x = Float::one_prec(100) >> 1u32;
    /// assert_eq!(x.asech_prec_round_assign(20, Ceiling), Greater);
    /// assert_eq!(x.to_string(), "1.3169594");
    ///
    /// let mut x = Float::one_prec(100) >> 1u32;
    /// assert_eq!(x.asech_prec_round_assign(20, Nearest), Less);
    /// assert_eq!(x.to_string(), "1.3169575");
    /// ```
    #[inline]
    pub fn asech_prec_round_assign(&mut self, prec: u64, rm: RoundingMode) -> Ordering {
        let o;
        (*self, o) = self.asech_prec_round_ref(prec, rm);
        o
    }

    /// Computes $\operatorname{asech} x$, the inverse hyperbolic secant of a [`Float`], rounding
    /// the result to the nearest value of the specified precision. The [`Float`] is replaced by the
    /// result, and an [`Ordering`] is returned, indicating whether the rounded inverse hyperbolic
    /// secant is less than, equal to, or greater than the exact inverse hyperbolic secant. Although
    /// `NaN`s are not comparable to any [`Float`], whenever this function sets a `NaN` it also
    /// returns `Equal`.
    ///
    /// If the inverse hyperbolic secant is equidistant from two [`Float`]s with the specified
    /// precision, the [`Float`] with fewer 1s in its binary expansion is chosen. See
    /// [`RoundingMode`] for a description of the `Nearest` rounding mode.
    ///
    /// $$
    /// x \gets \operatorname{asech} x+\varepsilon.
    /// $$
    /// - If $\operatorname{asech} x$ is infinite, zero, or NaN, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $\operatorname{asech} x$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2
    ///   \operatorname{asech} x\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// See the [`Float::asech_prec`] documentation for information on special cases, overflow, and
    /// underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::asech_prec_round_assign`] instead. If you know that your target precision is the
    /// precision of the input, consider using [`Float::asech_assign`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec,
    /// self.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::one_prec(100) >> 1u32;
    /// assert_eq!(x.asech_prec_assign(5), Less);
    /// assert_eq!(x.to_string(), "1.31");
    ///
    /// let mut x = Float::one_prec(100) >> 1u32;
    /// assert_eq!(x.asech_prec_assign(20), Less);
    /// assert_eq!(x.to_string(), "1.3169575");
    /// ```
    #[inline]
    pub fn asech_prec_assign(&mut self, prec: u64) -> Ordering {
        self.asech_prec_round_assign(prec, Nearest)
    }

    /// Computes $\operatorname{asech} x$, the inverse hyperbolic secant of a [`Float`], rounding
    /// the result with the specified rounding mode. The [`Float`] is replaced by the result, and an
    /// [`Ordering`] is returned, indicating whether the rounded inverse hyperbolic secant is less
    /// than, equal to, or greater than the exact inverse hyperbolic secant. Although `NaN`s are not
    /// comparable to any [`Float`], whenever this function sets a `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the precision of the input. See [`RoundingMode`] for a
    /// description of the possible rounding modes.
    ///
    /// $$
    /// x \gets \operatorname{asech} x+\varepsilon.
    /// $$
    /// - If $\operatorname{asech} x$ is infinite, zero, or NaN, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $\operatorname{asech} x$ is finite and nonzero, and $m$ is not `Nearest`, then
    ///   $|\varepsilon| < 2^{\lfloor\log_2 \operatorname{asech} x\rfloor-p+1}$, where $p$ is the
    ///   precision of the input.
    /// - If $\operatorname{asech} x$ is finite and nonzero, and $m$ is `Nearest`, then
    ///   $|\varepsilon| \leq 2^{\lfloor\log_2 \operatorname{asech} x\rfloor-p}$, where $p$ is the
    ///   precision of the input.
    ///
    /// If the output has a precision, it is the precision of the input.
    ///
    /// See the [`Float::asech_round`] documentation for information on special cases, overflow, and
    /// underflow.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::asech_prec_round_assign`] instead. If you know you'll be using the `Nearest`
    /// rounding mode, consider using [`Float::asech_assign`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` and `self` is finite, nonzero, and less than 1 in absolute value,
    /// since the inverse hyperbolic secant of such a [`Float`] is never exactly representable.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::one_prec(100) >> 1u32;
    /// assert_eq!(x.asech_round_assign(Floor), Less);
    /// assert_eq!(x.to_string(), "1.3169578969248167086250463473073");
    ///
    /// let mut x = Float::one_prec(100) >> 1u32;
    /// assert_eq!(x.asech_round_assign(Ceiling), Greater);
    /// assert_eq!(x.to_string(), "1.3169578969248167086250463473089");
    ///
    /// let mut x = Float::one_prec(100) >> 1u32;
    /// assert_eq!(x.asech_round_assign(Nearest), Less);
    /// assert_eq!(x.to_string(), "1.3169578969248167086250463473073");
    /// ```
    #[inline]
    pub fn asech_round_assign(&mut self, rm: RoundingMode) -> Ordering {
        let prec = self.significant_bits();
        self.asech_prec_round_assign(prec, rm)
    }

    /// Computes $\operatorname{asech} x$, the inverse hyperbolic secant of a [`Rational`], rounding
    /// the result to the specified precision and with the specified rounding mode and returning the
    /// result as a [`Float`]. The [`Rational`] is taken by value. An [`Ordering`] is also returned,
    /// indicating whether the rounded inverse hyperbolic secant is less than, equal to, or greater
    /// than the exact inverse hyperbolic secant. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \operatorname{asech} x+\varepsilon.
    /// $$
    /// - If $\operatorname{asech} x$ is infinite, zero, or NaN, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $\operatorname{asech} x$ is finite and nonzero, and $m$ is not `Nearest`, then
    ///   $|\varepsilon| < 2^{\lfloor\log_2 \operatorname{asech} x\rfloor-p+1}$.
    /// - If $\operatorname{asech} x$ is finite and nonzero, and $m$ is `Nearest`, then
    ///   $|\varepsilon| \leq 2^{\lfloor\log_2 \operatorname{asech} x\rfloor-p}$.
    ///
    /// These bounds do not apply when the result underflows; see below.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(0,p,m)=\infty$
    /// - $f(1,p,m)=0.0$
    /// - $f(x,p,m)=\text{NaN}$ if $x<0$ or $x>1$
    ///
    /// Overflow and underflow:
    /// - The result never overflows: for $0<x<1$ with denominator $d$, $\operatorname{asech} x <
    ///   \ln(2/x) \leq \ln 2d$.
    /// - If $0<f(x,p,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,p,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,p,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,p,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is returned
    ///   instead.
    ///
    /// Underflow requires an $x$ within $2^{-2^{31}}$ of 1, since $\operatorname{asech}(1-t) >
    /// \sqrt t$, and so a denominator of more than $2^{31}$ bits.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::asech_rational_prec`]
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^2 \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `x.significant_bits()`: the logarithm is computed at a working precision of about $n$, and
    /// the input is handled with `Rational` arithmetic.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with the given precision (which is the case for every $x$ with $0<x<1$).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = Float::asech_rational_prec_round(Rational::from_unsigneds(1u8, 3), 5, Floor);
    /// assert_eq!(c.to_string(), "1.75");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::asech_rational_prec_round(Rational::from_unsigneds(1u8, 3), 5, Ceiling);
    /// assert_eq!(c.to_string(), "1.81");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = Float::asech_rational_prec_round(Rational::from_unsigneds(1u8, 3), 20, Floor);
    /// assert_eq!(c.to_string(), "1.7627468");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) =
    ///     Float::asech_rational_prec_round(Rational::from_unsigneds(1u8, 3), 20, Ceiling);
    /// assert_eq!(c.to_string(), "1.7627487");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    #[allow(clippy::needless_pass_by_value)]
    pub fn asech_rational_prec_round(x: Rational, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        Self::asech_rational_prec_round_ref(&x, prec, rm)
    }

    /// Computes $\operatorname{asech} x$, the inverse hyperbolic secant of a [`Rational`], rounding
    /// the result to the specified precision and with the specified rounding mode and returning the
    /// result as a [`Float`]. The [`Rational`] is taken by reference. An [`Ordering`] is also
    /// returned, indicating whether the rounded inverse hyperbolic secant is less than, equal to,
    /// or greater than the exact inverse hyperbolic secant. Although `NaN`s are not comparable to
    /// any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \operatorname{asech} x+\varepsilon.
    /// $$
    /// - If $\operatorname{asech} x$ is infinite, zero, or NaN, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $\operatorname{asech} x$ is finite and nonzero, and $m$ is not `Nearest`, then
    ///   $|\varepsilon| < 2^{\lfloor\log_2 \operatorname{asech} x\rfloor-p+1}$.
    /// - If $\operatorname{asech} x$ is finite and nonzero, and $m$ is `Nearest`, then
    ///   $|\varepsilon| \leq 2^{\lfloor\log_2 \operatorname{asech} x\rfloor-p}$.
    ///
    /// These bounds do not apply when the result underflows; see below.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(0,p,m)=\infty$
    /// - $f(1,p,m)=0.0$
    /// - $f(x,p,m)=\text{NaN}$ if $x<0$ or $x>1$
    ///
    /// Overflow and underflow:
    /// - The result never overflows: for $0<x<1$ with denominator $d$, $\operatorname{asech} x <
    ///   \ln(2/x) \leq \ln 2d$.
    /// - If $0<f(x,p,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,p,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,p,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,p,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is returned
    ///   instead.
    ///
    /// Underflow requires an $x$ within $2^{-2^{31}}$ of 1, since $\operatorname{asech}(1-t) >
    /// \sqrt t$, and so a denominator of more than $2^{31}$ bits.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::asech_rational_prec_ref`]
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^2 \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `x.significant_bits()`: the logarithm is computed at a working precision of about $n$, and
    /// the input is handled with `Rational` arithmetic.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with the given precision (which is the case for every $x$ with $0<x<1$).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) =
    ///     Float::asech_rational_prec_round_ref(&Rational::from_unsigneds(1u8, 3), 5, Floor);
    /// assert_eq!(c.to_string(), "1.75");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) =
    ///     Float::asech_rational_prec_round_ref(&Rational::from_unsigneds(1u8, 3), 5, Ceiling);
    /// assert_eq!(c.to_string(), "1.81");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) =
    ///     Float::asech_rational_prec_round_ref(&Rational::from_unsigneds(1u8, 3), 20, Floor);
    /// assert_eq!(c.to_string(), "1.7627468");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) =
    ///     Float::asech_rational_prec_round_ref(&Rational::from_unsigneds(1u8, 3), 20, Ceiling);
    /// assert_eq!(c.to_string(), "1.7627487");
    /// assert_eq!(o, Greater);
    /// ```
    pub fn asech_rational_prec_round_ref(
        x: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        assert_ne!(prec, 0);
        match x.sign() {
            // asech is NaN below 0
            Less => (Self::NAN, Equal),
            // asech(0) = +Inf
            Equal => (Self::INFINITY, Equal),
            // asech(x) = acosh(1/x), and the reciprocal of a `Rational` is exact
            Greater => Self::acosh_rational_prec_round(x.reciprocal(), prec, rm),
        }
    }

    /// Computes $\operatorname{asech} x$, the inverse hyperbolic secant of a [`Rational`], rounding
    /// the result to the nearest value of the specified precision and returning the result as a
    /// [`Float`]. The [`Rational`] is taken by value. An [`Ordering`] is also returned, indicating
    /// whether the rounded inverse hyperbolic secant is less than, equal to, or greater than the
    /// exact inverse hyperbolic secant. Although `NaN`s are not comparable to any [`Float`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// If the inverse hyperbolic secant is equidistant from two [`Float`]s with the specified
    /// precision, the [`Float`] with fewer 1s in its binary expansion is chosen. See
    /// [`RoundingMode`] for a description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = \operatorname{asech} x+\varepsilon,
    /// $$
    /// where, if $\operatorname{asech} x$ is finite and nonzero, $|\varepsilon| \leq
    /// 2^{\lfloor\log_2 \operatorname{asech} x\rfloor-p}$ (unless the result underflows; see
    /// below).
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(0,p)=\infty$
    /// - $f(1,p)=0.0$
    /// - $f(x,p)=\text{NaN}$ if $x<0$ or $x>1$
    ///
    /// Overflow and underflow:
    /// - The result never overflows: for $0<x<1$ with denominator $d$, $\operatorname{asech} x <
    ///   \ln(2/x) \leq \ln 2d$.
    /// - If $0<f(x,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    ///
    /// Underflow requires an $x$ within $2^{-2^{31}}$ of 1, since $\operatorname{asech}(1-t) >
    /// \sqrt t$, and so a denominator of more than $2^{31}$ bits.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::asech_rational_prec_round`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^2 \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `x.significant_bits()`: the logarithm is computed at a working precision of about $n$, and
    /// the input is handled with `Rational` arithmetic.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{One, Two, Zero};
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = Float::asech_rational_prec(Rational::from_unsigneds(1u8, 3), 5);
    /// assert_eq!(c.to_string(), "1.75");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::asech_rational_prec(Rational::from_unsigneds(1u8, 3), 20);
    /// assert_eq!(c.to_string(), "1.7627468");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::asech_rational_prec(Rational::ONE, 10);
    /// assert_eq!(c.to_string(), "0.0");
    /// assert_eq!(o, Equal);
    ///
    /// let (c, o) = Float::asech_rational_prec(Rational::ZERO, 10);
    /// assert_eq!(c.to_string(), "Infinity");
    /// assert_eq!(o, Equal);
    ///
    /// let (c, o) = Float::asech_rational_prec(Rational::TWO, 10);
    /// assert!(c.is_nan());
    /// assert_eq!(o, Equal);
    /// ```
    #[inline]
    #[allow(clippy::needless_pass_by_value)]
    pub fn asech_rational_prec(x: Rational, prec: u64) -> (Self, Ordering) {
        Self::asech_rational_prec_round_ref(&x, prec, Nearest)
    }

    /// Computes $\operatorname{asech} x$, the inverse hyperbolic secant of a [`Rational`], rounding
    /// the result to the nearest value of the specified precision and returning the result as a
    /// [`Float`]. The [`Rational`] is taken by reference. An [`Ordering`] is also returned,
    /// indicating whether the rounded inverse hyperbolic secant is less than, equal to, or greater
    /// than the exact inverse hyperbolic secant. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// If the inverse hyperbolic secant is equidistant from two [`Float`]s with the specified
    /// precision, the [`Float`] with fewer 1s in its binary expansion is chosen. See
    /// [`RoundingMode`] for a description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = \operatorname{asech} x+\varepsilon,
    /// $$
    /// where, if $\operatorname{asech} x$ is finite and nonzero, $|\varepsilon| \leq
    /// 2^{\lfloor\log_2 \operatorname{asech} x\rfloor-p}$ (unless the result underflows; see
    /// below).
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(0,p)=\infty$
    /// - $f(1,p)=0.0$
    /// - $f(x,p)=\text{NaN}$ if $x<0$ or $x>1$
    ///
    /// Overflow and underflow:
    /// - The result never overflows: for $0<x<1$ with denominator $d$, $\operatorname{asech} x <
    ///   \ln(2/x) \leq \ln 2d$.
    /// - If $0<f(x,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    ///
    /// Underflow requires an $x$ within $2^{-2^{31}}$ of 1, since $\operatorname{asech}(1-t) >
    /// \sqrt t$, and so a denominator of more than $2^{31}$ bits.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::asech_rational_prec_round_ref`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^2 \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `x.significant_bits()`: the logarithm is computed at a working precision of about $n$, and
    /// the input is handled with `Rational` arithmetic.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{One, Two, Zero};
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = Float::asech_rational_prec_ref(&Rational::from_unsigneds(1u8, 3), 5);
    /// assert_eq!(c.to_string(), "1.75");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::asech_rational_prec_ref(&Rational::from_unsigneds(1u8, 3), 20);
    /// assert_eq!(c.to_string(), "1.7627468");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::asech_rational_prec_ref(&Rational::ONE, 10);
    /// assert_eq!(c.to_string(), "0.0");
    /// assert_eq!(o, Equal);
    ///
    /// let (c, o) = Float::asech_rational_prec_ref(&Rational::ZERO, 10);
    /// assert_eq!(c.to_string(), "Infinity");
    /// assert_eq!(o, Equal);
    ///
    /// let (c, o) = Float::asech_rational_prec_ref(&Rational::TWO, 10);
    /// assert!(c.is_nan());
    /// assert_eq!(o, Equal);
    /// ```
    #[inline]
    pub fn asech_rational_prec_ref(x: &Rational, prec: u64) -> (Self, Ordering) {
        Self::asech_rational_prec_round_ref(x, prec, Nearest)
    }
}

impl Asech for Float {
    type Output = Self;

    /// Computes $\operatorname{asech} x$, the inverse hyperbolic secant of a [`Float`], taking it
    /// by value.
    ///
    /// If the output has a precision, it is the precision of the input. If the inverse hyperbolic
    /// secant is equidistant from two [`Float`]s with the specified precision, the [`Float`] with
    /// fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of the
    /// `Nearest` rounding mode.
    ///
    /// $$
    /// f(x) = \operatorname{asech} x+\varepsilon.
    /// $$
    /// - If $\operatorname{asech} x$ is infinite, zero, or NaN, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $\operatorname{asech} x$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2
    ///   \operatorname{asech} x\rfloor-p}$, where $p$ is the precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN})=\text{NaN}$
    /// - $f(\pm\infty)=\text{NaN}$
    /// - $f(\pm0.0)=\infty$
    /// - $f(1)=0.0$
    /// - $f(x)=\text{NaN}$ if $x<0$ or $x>1$
    ///
    /// See the [`Float::asech_round`] documentation for information on overflow and underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::asech_round`] instead. If you want to specify the output precision, consider using
    /// [`Float::asech_prec`]. If you want both of these things, consider using
    /// [`Float::asech_prec_round`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Asech;
    /// use malachite_base::num::basic::traits::*;
    /// use malachite_float::Float;
    ///
    /// assert!(Float::NAN.asech().is_nan());
    /// assert!(Float::INFINITY.asech().is_nan());
    /// assert!(Float::NEGATIVE_INFINITY.asech().is_nan());
    /// assert_eq!(Float::ZERO.asech().to_string(), "Infinity");
    /// assert_eq!(Float::NEGATIVE_ZERO.asech().to_string(), "Infinity");
    /// assert_eq!(Float::ONE.asech().to_string(), "0.0");
    /// assert!(Float::TWO.asech().is_nan());
    /// assert!(Float::NEGATIVE_ONE.asech().is_nan());
    /// assert_eq!(
    ///     (Float::one_prec(100) >> 1u32).asech().to_string(),
    ///     "1.3169578969248167086250463473073"
    /// );
    /// assert_eq!(
    ///     (Float::one_prec(100) >> 2u32).asech().to_string(),
    ///     "2.0634370688955605467272811726205"
    /// );
    /// ```
    #[inline]
    fn asech(self) -> Self {
        let prec = self.significant_bits();
        self.asech_prec_round(prec, Nearest).0
    }
}

impl Asech for &Float {
    type Output = Float;

    /// Computes $\operatorname{asech} x$, the inverse hyperbolic secant of a [`Float`], taking it
    /// by reference.
    ///
    /// If the output has a precision, it is the precision of the input. If the inverse hyperbolic
    /// secant is equidistant from two [`Float`]s with the specified precision, the [`Float`] with
    /// fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of the
    /// `Nearest` rounding mode.
    ///
    /// $$
    /// f(x) = \operatorname{asech} x+\varepsilon.
    /// $$
    /// - If $\operatorname{asech} x$ is infinite, zero, or NaN, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $\operatorname{asech} x$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2
    ///   \operatorname{asech} x\rfloor-p}$, where $p$ is the precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN})=\text{NaN}$
    /// - $f(\pm\infty)=\text{NaN}$
    /// - $f(\pm0.0)=\infty$
    /// - $f(1)=0.0$
    /// - $f(x)=\text{NaN}$ if $x<0$ or $x>1$
    ///
    /// See the [`Float::asech_round`] documentation for information on overflow and underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::asech_round_ref`] instead. If you want to specify the output precision, consider
    /// using [`Float::asech_prec_ref`]. If you want both of these things, consider using
    /// [`Float::asech_prec_round_ref`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Asech;
    /// use malachite_base::num::basic::traits::*;
    /// use malachite_float::Float;
    ///
    /// assert!((&Float::NAN).asech().is_nan());
    /// assert!((&Float::INFINITY).asech().is_nan());
    /// assert!((&Float::NEGATIVE_INFINITY).asech().is_nan());
    /// assert_eq!((&Float::ZERO).asech().to_string(), "Infinity");
    /// assert_eq!((&Float::NEGATIVE_ZERO).asech().to_string(), "Infinity");
    /// assert_eq!((&Float::ONE).asech().to_string(), "0.0");
    /// assert!((&Float::TWO).asech().is_nan());
    /// assert!((&Float::NEGATIVE_ONE).asech().is_nan());
    /// assert_eq!(
    ///     (&(Float::one_prec(100) >> 1u32)).asech().to_string(),
    ///     "1.3169578969248167086250463473073"
    /// );
    /// assert_eq!(
    ///     (&(Float::one_prec(100) >> 2u32)).asech().to_string(),
    ///     "2.0634370688955605467272811726205"
    /// );
    /// ```
    #[inline]
    fn asech(self) -> Float {
        self.asech_prec_round_ref(self.significant_bits(), Nearest)
            .0
    }
}

impl AsechAssign for Float {
    /// Computes $\operatorname{asech} x$, the inverse hyperbolic secant of a [`Float`], in place.
    ///
    /// If the output has a precision, it is the precision of the input. If the inverse hyperbolic
    /// secant is equidistant from two [`Float`]s with the specified precision, the [`Float`] with
    /// fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of the
    /// `Nearest` rounding mode.
    ///
    /// $$
    /// x \gets \operatorname{asech} x+\varepsilon.
    /// $$
    /// - If $\operatorname{asech} x$ is infinite, zero, or NaN, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $\operatorname{asech} x$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2
    ///   \operatorname{asech} x\rfloor-p}$, where $p$ is the precision of the input.
    ///
    /// See the [`Float::asech`] documentation for information on special cases, overflow, and
    /// underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::asech_round_assign`] instead. If you want to specify the output precision, consider
    /// using [`Float::asech_prec_assign`]. If you want both of these things, consider using
    /// [`Float::asech_prec_round_assign`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::AsechAssign;
    /// use malachite_base::num::basic::traits::*;
    /// use malachite_float::Float;
    ///
    /// let mut x = Float::NAN;
    /// x.asech_assign();
    /// assert!(x.is_nan());
    ///
    /// let mut x = Float::INFINITY;
    /// x.asech_assign();
    /// assert!(x.is_nan());
    ///
    /// let mut x = Float::ZERO;
    /// x.asech_assign();
    /// assert_eq!(x.to_string(), "Infinity");
    ///
    /// let mut x = Float::NEGATIVE_ZERO;
    /// x.asech_assign();
    /// assert_eq!(x.to_string(), "Infinity");
    ///
    /// let mut x = Float::ONE;
    /// x.asech_assign();
    /// assert_eq!(x.to_string(), "0.0");
    ///
    /// let mut x = Float::TWO;
    /// x.asech_assign();
    /// assert!(x.is_nan());
    ///
    /// let mut x = Float::one_prec(100) >> 1u32;
    /// x.asech_assign();
    /// assert_eq!(x.to_string(), "1.3169578969248167086250463473073");
    ///
    /// let mut x = Float::one_prec(100) >> 2u32;
    /// x.asech_assign();
    /// assert_eq!(x.to_string(), "2.0634370688955605467272811726205");
    /// ```
    #[inline]
    fn asech_assign(&mut self) {
        let prec = self.significant_bits();
        self.asech_prec_round_assign(prec, Nearest);
    }
}

/// Computes $\operatorname{asech} x$, the inverse hyperbolic secant of a primitive float. Using
/// this function is more accurate than using the default `asech` function or the one provided by
/// `libm`.
///
/// $$
/// f(x) = \operatorname{asech} x+\varepsilon.
/// $$
/// - If $\operatorname{asech} x$ is infinite, zero, or NaN, $\varepsilon$ may be ignored or assumed
///   to be 0.
/// - If $\operatorname{asech} x$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2
///   \operatorname{asech} x\rfloor-p}$, where $p$ is the precision of the output (24 if `T` is a
///   [`f32`] and 53 if `T` is a [`f64`]).
///
/// Special cases:
/// - $f(\text{NaN})=\text{NaN}$
/// - $f(\pm\infty)=\text{NaN}$
/// - $f(\pm0.0)=\infty$
/// - $f(1)=0.0$
/// - $f(x)=\text{NaN}$ if $x<0$ or $x>1$
///
/// Overflow is not possible. The result is subnormal only when $x$ is, and then it is $x$ itself,
/// since $|\operatorname{asech} x - x| < |x|^3/2$.
///
/// # Worst-case complexity
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use malachite_base::num::basic::traits::NegativeInfinity;
/// use malachite_base::num::float::NiceFloat;
/// use malachite_float::float::arithmetic::asech::primitive_float_asech;
///
/// assert!(primitive_float_asech(f32::NAN).is_nan());
/// assert!(primitive_float_asech(f32::INFINITY).is_nan());
/// assert!(primitive_float_asech(f32::NEGATIVE_INFINITY).is_nan());
/// assert_eq!(
///     NiceFloat(primitive_float_asech(0.0f32)),
///     NiceFloat(f32::INFINITY)
/// );
/// assert_eq!(NiceFloat(primitive_float_asech(1.0f32)), NiceFloat(0.0));
/// assert!(primitive_float_asech(2.0f32).is_nan());
/// assert_eq!(
///     NiceFloat(primitive_float_asech(0.5f32)),
///     NiceFloat(1.316958)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_asech(0.5f64)),
///     NiceFloat(1.3169578969248168)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_asech(0.1f64)),
///     NiceFloat(2.993222846126381)
/// );
/// ```
#[inline]
#[allow(clippy::type_repetition_in_bounds)]
pub fn primitive_float_asech<T: PrimitiveFloat>(x: T) -> T
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    emulate_float_to_float_fn(Float::asech_prec, x)
}

/// Computes $\operatorname{asech} x$, the inverse hyperbolic secant of a [`Rational`], returning
/// the result as a primitive float. The result is correctly rounded.
///
/// $$
/// f(x) = \operatorname{asech} x+\varepsilon.
/// $$
/// - If $\operatorname{asech} x$ is infinite, zero, or NaN, $\varepsilon$ may be ignored or assumed
///   to be 0.
/// - If $\operatorname{asech} x$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2
///   \operatorname{asech} x\rfloor-p}$, where $p$ is the precision of the output (typically 24 if
///   `T` is a [`f32`] and 53 if `T` is a [`f64`], but less if the output is subnormal).
///
/// Special cases:
/// - $f(0)=\infty$
/// - $f(1)=0.0$
/// - $f(x)=\text{NaN}$ if $x<0$ or $x>1$
///
/// Overflow is not possible. Underflow is: an `x` close enough to 1 gives `0.0`.
///
/// # Worst-case complexity
/// $T(m) = O(m (\log m)^2 \log\log m)$
///
/// $M(m) = O(m \log m)$
///
/// where $T$ is time, $M$ is additional memory, and $m$ is `x.significant_bits()`.
///
/// # Examples
/// ```
/// use malachite_base::num::basic::traits::{One, Two, Zero};
/// use malachite_base::num::float::NiceFloat;
/// use malachite_float::float::arithmetic::asech::primitive_float_asech_rational;
/// use malachite_q::Rational;
///
/// assert_eq!(
///     NiceFloat(primitive_float_asech_rational::<f64>(&Rational::ZERO)),
///     NiceFloat(f64::INFINITY)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_asech_rational::<f64>(&Rational::ONE)),
///     NiceFloat(0.0)
/// );
/// assert!(primitive_float_asech_rational::<f64>(&Rational::TWO).is_nan());
/// assert_eq!(
///     NiceFloat(primitive_float_asech_rational::<f64>(
///         &Rational::from_unsigneds(1u8, 3)
///     )),
///     NiceFloat(1.762747174039086)
/// );
/// ```
#[inline]
#[allow(clippy::type_repetition_in_bounds)]
pub fn primitive_float_asech_rational<T: PrimitiveFloat>(x: &Rational) -> T
where
    Float: PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    emulate_rational_to_float_fn(Float::asech_rational_prec_ref, x)
}
