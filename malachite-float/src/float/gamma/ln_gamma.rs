// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::InnerFloat::{Infinity, NaN, Zero};
use crate::big_exponent_float::BigExponentFloat;
use crate::big_exponent_float::conversion::float_from_big_exponent_float::*;
use crate::big_exponent_float::gamma::ln_gamma::{ln_gamma_helper, ln_gamma_rational_helper};
use crate::{Float, emulate_float_to_float_fn, emulate_rational_to_float_fn};
use core::cmp::Ordering::{self, *};
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::conversion::traits::{ExactFrom, RoundingFrom};
use malachite_base::num::gamma::traits::{LnGamma, LnGammaAssign};
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_q::Rational;

// ln Gamma(x), or ln |Gamma(x)| if `abs` is set, rounded to `prec` bits with `rm`, and whether
// Gamma(x) is positive (`true` where the sign is not defined, except that it is `false` at -0.0,
// where Gamma is taken to be -infinity).
pub(crate) fn ln_gamma_prec_round_helper(
    abs: bool,
    x: &Float,
    prec: u64,
    rm: RoundingMode,
) -> (Float, bool, Ordering) {
    assert_ne!(prec, 0);
    match x {
        Float(NaN) => (float_nan!(), true, Equal),
        Float(Infinity { sign: false }) if !abs => (float_nan!(), true, Equal),
        Float(Infinity { .. }) => (float_infinity!(), true, Equal),
        Float(Zero { sign }) => (float_infinity!(), *sign || !abs, Equal),
        _ => {
            let (y, sign, o) = ln_gamma_helper(abs, &BigExponentFloat::from(x), prec, rm);
            let (y, o) = float_from_rounded_big_exponent_float(y, o, prec, rm);
            (y, sign, o)
        }
    }
}

// ln Gamma(q), or ln |Gamma(q)| if `abs` is set, for a `Rational` q, rounded to `prec` bits with
// `rm`, and whether Gamma(q) is positive (`true` where the sign is not defined).
pub(crate) fn ln_gamma_rational_prec_round_helper(
    abs: bool,
    q: &Rational,
    prec: u64,
    rm: RoundingMode,
) -> (Float, bool, Ordering) {
    let (y, sign, o) = ln_gamma_rational_helper(abs, q, prec, rm);
    let (y, o) = float_from_rounded_big_exponent_float(y, o, prec, rm);
    (y, sign, o)
}

impl Float {
    /// Computes $\ln \Gamma(x)$, the logarithm of the gamma function, of a [`Float`], rounding the
    /// result to the specified precision and with the specified rounding mode. The [`Float`] is
    /// taken by value. An [`Ordering`] is also returned, indicating whether the rounded value is
    /// less than, equal to, or greater than the exact value. Although `NaN`s are not comparable to
    /// any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $\Gamma(x) = \int_0^\infty t^{x-1} e^{-t}\,dt$ for $x > 0$. Its logarithm is 0 at $x = 1$
    /// and $x = 2$, negative between them, and positive on the rest of the positive reals. This
    /// function is not defined for a negative $x$, even where $\Gamma(x)$ is positive;
    /// [`Float::ln_abs_gamma_prec_round`] gives $\ln |\Gamma(x)|$ for every $x$.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \ln \Gamma(x)+\varepsilon.
    /// $$
    /// - If $\ln \Gamma(x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $\ln \Gamma(x)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\ln \Gamma(x)|\rfloor-p+1}$.
    /// - If $\ln \Gamma(x)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\ln \Gamma(x)|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=\text{NaN}$
    /// - $f(\infty,p,m)=\infty$
    /// - $f(-\infty,p,m)=\text{NaN}$
    /// - $f(\pm0.0,p,m)=\infty$
    /// - $f(x,p,m)=\text{NaN}$ if $x<0$
    /// - $f(1,p,m)=f(2,p,m)=0.0$
    ///
    /// Overflow and underflow:
    /// - If $f(x,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$ is
    ///   returned instead.
    /// - If $0<f(x,p,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,p,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,p,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,p,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,p,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,p,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,p,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,p,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is
    ///   returned instead.
    ///
    /// Overflow happens for arguments above about $2^{2^{30}-30.5}$. Underflow only happens for an
    /// argument within about $2^{-2^{30}}$ of one of the zeros 1 and 2; such an argument has more
    /// than $2^{30}$ significant bits.
    ///
    /// The result is correctly rounded. Finding it takes $\ln \Gamma(x)$ not to be a rounding
    /// boundary, which is a number with at most `prec + 1` significant bits. This is known when $x$
    /// is a positive integer (then the value is the logarithm of an integer, which is 0 or
    /// irrational); nothing suggests that it ever fails, but it has not been proven for every $x$,
    /// and this function would not terminate if it did fail.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::ln_gamma_prec`] instead. If
    /// you know that your target precision is the precision of the input, consider using
    /// [`Float::ln_gamma_round`] instead. If both of these things are true, consider using
    /// [`Float::ln_gamma`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^3 \log n)$
    ///
    /// $M(n) = O(n^2 \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec,
    /// self.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with the given precision (it can only when it is infinite or `NaN`, or when $x$ is 1 or 2).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (g, o) = (Float::one_half_prec(100)).ln_gamma_prec_round(5, Floor);
    /// assert_eq!(g.to_string(), "0.562");
    /// assert_eq!(o, Less);
    ///
    /// let (g, o) = (Float::one_half_prec(100)).ln_gamma_prec_round(5, Ceiling);
    /// assert_eq!(g.to_string(), "0.594");
    /// assert_eq!(o, Greater);
    ///
    /// let (g, o) = (Float::one_half_prec(100)).ln_gamma_prec_round(5, Nearest);
    /// assert_eq!(g.to_string(), "0.562");
    /// assert_eq!(o, Less);
    ///
    /// let (g, o) = (Float::one_half_prec(100)).ln_gamma_prec_round(20, Floor);
    /// assert_eq!(g.to_string(), "0.57236481");
    /// assert_eq!(o, Less);
    ///
    /// let (g, o) = (Float::one_half_prec(100)).ln_gamma_prec_round(20, Ceiling);
    /// assert_eq!(g.to_string(), "0.57236576");
    /// assert_eq!(o, Greater);
    ///
    /// let (g, o) = (Float::one_half_prec(100)).ln_gamma_prec_round(20, Nearest);
    /// assert_eq!(g.to_string(), "0.57236481");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn ln_gamma_prec_round(self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        let (y, _, o) = ln_gamma_prec_round_helper(false, &self, prec, rm);
        (y, o)
    }

    /// Computes $\ln \Gamma(x)$, the logarithm of the gamma function, of a [`Float`], rounding the
    /// result to the specified precision and with the specified rounding mode. The [`Float`] is
    /// taken by reference. An [`Ordering`] is also returned, indicating whether the rounded value
    /// is less than, equal to, or greater than the exact value. Although `NaN`s are not comparable
    /// to any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $\Gamma(x) = \int_0^\infty t^{x-1} e^{-t}\,dt$ for $x > 0$. Its logarithm is 0 at $x = 1$
    /// and $x = 2$, negative between them, and positive on the rest of the positive reals. This
    /// function is not defined for a negative $x$, even where $\Gamma(x)$ is positive;
    /// [`Float::ln_abs_gamma_prec_round`] gives $\ln |\Gamma(x)|$ for every $x$.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \ln \Gamma(x)+\varepsilon.
    /// $$
    /// - If $\ln \Gamma(x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $\ln \Gamma(x)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\ln \Gamma(x)|\rfloor-p+1}$.
    /// - If $\ln \Gamma(x)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\ln \Gamma(x)|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=\text{NaN}$
    /// - $f(\infty,p,m)=\infty$
    /// - $f(-\infty,p,m)=\text{NaN}$
    /// - $f(\pm0.0,p,m)=\infty$
    /// - $f(x,p,m)=\text{NaN}$ if $x<0$
    /// - $f(1,p,m)=f(2,p,m)=0.0$
    ///
    /// Overflow and underflow:
    /// - If $f(x,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$ is
    ///   returned instead.
    /// - If $0<f(x,p,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,p,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,p,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,p,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,p,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,p,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,p,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,p,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is
    ///   returned instead.
    ///
    /// Overflow happens for arguments above about $2^{2^{30}-30.5}$. Underflow only happens for an
    /// argument within about $2^{-2^{30}}$ of one of the zeros 1 and 2; such an argument has more
    /// than $2^{30}$ significant bits.
    ///
    /// The result is correctly rounded. Finding it takes $\ln \Gamma(x)$ not to be a rounding
    /// boundary, which is a number with at most `prec + 1` significant bits. This is known when $x$
    /// is a positive integer (then the value is the logarithm of an integer, which is 0 or
    /// irrational); nothing suggests that it ever fails, but it has not been proven for every $x$,
    /// and this function would not terminate if it did fail.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::ln_gamma_prec_ref`] instead.
    /// If you know that your target precision is the precision of the input, consider using
    /// [`Float::ln_gamma_round_ref`] instead. If both of these things are true, consider using
    /// `(&Float).ln_gamma()` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^3 \log n)$
    ///
    /// $M(n) = O(n^2 \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec,
    /// self.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with the given precision (it can only when it is infinite or `NaN`, or when $x$ is 1 or 2).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (g, o) = (Float::one_half_prec(100)).ln_gamma_prec_round_ref(5, Floor);
    /// assert_eq!(g.to_string(), "0.562");
    /// assert_eq!(o, Less);
    ///
    /// let (g, o) = (Float::one_half_prec(100)).ln_gamma_prec_round_ref(5, Ceiling);
    /// assert_eq!(g.to_string(), "0.594");
    /// assert_eq!(o, Greater);
    ///
    /// let (g, o) = (Float::one_half_prec(100)).ln_gamma_prec_round_ref(5, Nearest);
    /// assert_eq!(g.to_string(), "0.562");
    /// assert_eq!(o, Less);
    ///
    /// let (g, o) = (Float::one_half_prec(100)).ln_gamma_prec_round_ref(20, Floor);
    /// assert_eq!(g.to_string(), "0.57236481");
    /// assert_eq!(o, Less);
    ///
    /// let (g, o) = (Float::one_half_prec(100)).ln_gamma_prec_round_ref(20, Ceiling);
    /// assert_eq!(g.to_string(), "0.57236576");
    /// assert_eq!(o, Greater);
    ///
    /// let (g, o) = (Float::one_half_prec(100)).ln_gamma_prec_round_ref(20, Nearest);
    /// assert_eq!(g.to_string(), "0.57236481");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn ln_gamma_prec_round_ref(&self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        let (y, _, o) = ln_gamma_prec_round_helper(false, self, prec, rm);
        (y, o)
    }

    /// Computes $\ln \Gamma(x)$, the logarithm of the gamma function, of a [`Float`], rounding the
    /// result to the nearest value of the specified precision. The [`Float`] is taken by value. An
    /// [`Ordering`] is also returned, indicating whether the rounded value is less than, equal to,
    /// or greater than the exact value. Although `NaN`s are not comparable to any [`Float`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $\Gamma(x) = \int_0^\infty t^{x-1} e^{-t}\,dt$ for $x > 0$. Its logarithm is 0 at $x = 1$
    /// and $x = 2$, negative between them, and positive on the rest of the positive reals. This
    /// function is not defined for a negative $x$, even where $\Gamma(x)$ is positive;
    /// [`Float::ln_abs_gamma_prec_round`] gives $\ln |\Gamma(x)|$ for every $x$.
    ///
    /// If the value is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = \ln \Gamma(x)+\varepsilon.
    /// $$
    /// - If $\ln \Gamma(x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $\ln \Gamma(x)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2 |\ln
    ///   \Gamma(x)|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p)=\text{NaN}$
    /// - $f(\infty,p)=\infty$
    /// - $f(-\infty,p)=\text{NaN}$
    /// - $f(\pm0.0,p)=\infty$
    /// - $f(x,p)=\text{NaN}$ if $x<0$
    /// - $f(1,p)=f(2,p)=0.0$
    ///
    /// Overflow and underflow:
    /// - If $f(x,p)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $0<f(x,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// Overflow happens for arguments above about $2^{2^{30}-30.5}$. Underflow only happens for an
    /// argument within about $2^{-2^{30}}$ of one of the zeros 1 and 2; such an argument has more
    /// than $2^{30}$ significant bits.
    ///
    /// The result is correctly rounded. Finding it takes $\ln \Gamma(x)$ not to be a rounding
    /// boundary, which is a number with at most `prec + 1` significant bits. This is known when $x$
    /// is a positive integer (then the value is the logarithm of an integer, which is 0 or
    /// irrational); nothing suggests that it ever fails, but it has not been proven for every $x$,
    /// and this function would not terminate if it did fail.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::ln_gamma_prec_round`] instead. If you know that your target precision is the
    /// precision of the input, consider using [`Float::ln_gamma`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^3 \log n)$
    ///
    /// $M(n) = O(n^2 \log n)$
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
    /// let (g, o) = (Float::one_half_prec(100)).ln_gamma_prec(5);
    /// assert_eq!(g.to_string(), "0.562");
    /// assert_eq!(o, Less);
    ///
    /// let (g, o) = (Float::one_half_prec(100)).ln_gamma_prec(20);
    /// assert_eq!(g.to_string(), "0.57236481");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn ln_gamma_prec(self, prec: u64) -> (Self, Ordering) {
        self.ln_gamma_prec_round(prec, Nearest)
    }

    /// Computes $\ln \Gamma(x)$, the logarithm of the gamma function, of a [`Float`], rounding the
    /// result to the nearest value of the specified precision. The [`Float`] is taken by reference.
    /// An [`Ordering`] is also returned, indicating whether the rounded value is less than, equal
    /// to, or greater than the exact value. Although `NaN`s are not comparable to any [`Float`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $\Gamma(x) = \int_0^\infty t^{x-1} e^{-t}\,dt$ for $x > 0$. Its logarithm is 0 at $x = 1$
    /// and $x = 2$, negative between them, and positive on the rest of the positive reals. This
    /// function is not defined for a negative $x$, even where $\Gamma(x)$ is positive;
    /// [`Float::ln_abs_gamma_prec_round`] gives $\ln |\Gamma(x)|$ for every $x$.
    ///
    /// If the value is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = \ln \Gamma(x)+\varepsilon.
    /// $$
    /// - If $\ln \Gamma(x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $\ln \Gamma(x)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2 |\ln
    ///   \Gamma(x)|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p)=\text{NaN}$
    /// - $f(\infty,p)=\infty$
    /// - $f(-\infty,p)=\text{NaN}$
    /// - $f(\pm0.0,p)=\infty$
    /// - $f(x,p)=\text{NaN}$ if $x<0$
    /// - $f(1,p)=f(2,p)=0.0$
    ///
    /// Overflow and underflow:
    /// - If $f(x,p)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $0<f(x,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// Overflow happens for arguments above about $2^{2^{30}-30.5}$. Underflow only happens for an
    /// argument within about $2^{-2^{30}}$ of one of the zeros 1 and 2; such an argument has more
    /// than $2^{30}$ significant bits.
    ///
    /// The result is correctly rounded. Finding it takes $\ln \Gamma(x)$ not to be a rounding
    /// boundary, which is a number with at most `prec + 1` significant bits. This is known when $x$
    /// is a positive integer (then the value is the logarithm of an integer, which is 0 or
    /// irrational); nothing suggests that it ever fails, but it has not been proven for every $x$,
    /// and this function would not terminate if it did fail.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::ln_gamma_prec_round_ref`] instead. If you know that your target precision is the
    /// precision of the input, consider using `(&Float).ln_gamma()` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^3 \log n)$
    ///
    /// $M(n) = O(n^2 \log n)$
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
    /// let (g, o) = (Float::one_half_prec(100)).ln_gamma_prec_ref(5);
    /// assert_eq!(g.to_string(), "0.562");
    /// assert_eq!(o, Less);
    ///
    /// let (g, o) = (Float::one_half_prec(100)).ln_gamma_prec_ref(20);
    /// assert_eq!(g.to_string(), "0.57236481");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn ln_gamma_prec_ref(&self, prec: u64) -> (Self, Ordering) {
        self.ln_gamma_prec_round_ref(prec, Nearest)
    }

    /// Computes $\ln \Gamma(x)$, the logarithm of the gamma function, of a [`Float`], rounding the
    /// result with the specified rounding mode. The [`Float`] is taken by value. An [`Ordering`] is
    /// also returned, indicating whether the rounded value is less than, equal to, or greater than
    /// the exact value. Although `NaN`s are not comparable to any [`Float`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// $\Gamma(x) = \int_0^\infty t^{x-1} e^{-t}\,dt$ for $x > 0$. Its logarithm is 0 at $x = 1$
    /// and $x = 2$, negative between them, and positive on the rest of the positive reals. This
    /// function is not defined for a negative $x$, even where $\Gamma(x)$ is positive;
    /// [`Float::ln_abs_gamma_prec_round`] gives $\ln |\Gamma(x)|$ for every $x$.
    ///
    /// The precision of the output is the precision of the input. See [`RoundingMode`] for a
    /// description of the possible rounding modes.
    ///
    /// $$
    /// f(x,m) = \ln \Gamma(x)+\varepsilon.
    /// $$
    /// - If $\ln \Gamma(x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $\ln \Gamma(x)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\ln \Gamma(x)|\rfloor-p+1}$, where $p$ is the precision of the input.
    /// - If $\ln \Gamma(x)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\ln \Gamma(x)|\rfloor-p}$, where $p$ is the precision of the input.
    ///
    /// If the output has a precision, it is the precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN},m)=\text{NaN}$
    /// - $f(\infty,m)=\infty$
    /// - $f(-\infty,m)=\text{NaN}$
    /// - $f(\pm0.0,m)=\infty$
    /// - $f(x,m)=\text{NaN}$ if $x<0$
    /// - $f(1,m)=f(2,m)=0.0$
    ///
    /// See the [`Float::ln_gamma_prec_round`] documentation for information on overflow and
    /// underflow, and on termination.
    ///
    /// If you want to specify an output precision, consider using [`Float::ln_gamma_prec_round`]
    /// instead. If you know you'll be using the `Nearest` rounding mode, consider using
    /// [`Float::ln_gamma`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^3 \log n)$
    ///
    /// $M(n) = O(n^2 \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the result cannot be represented exactly with the input
    /// precision (it can only when it is infinite or `NaN`, or when $x$ is 1 or 2).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (g, o) = (Float::one_half_prec(100)).ln_gamma_round(Floor);
    /// assert_eq!(g.to_string(), "0.57236494292470008707171367567642");
    /// assert_eq!(o, Less);
    ///
    /// let (g, o) = (Float::one_half_prec(100)).ln_gamma_round(Ceiling);
    /// assert_eq!(g.to_string(), "0.57236494292470008707171367567721");
    /// assert_eq!(o, Greater);
    ///
    /// let (g, o) = (Float::one_half_prec(100)).ln_gamma_round(Nearest);
    /// assert_eq!(g.to_string(), "0.57236494292470008707171367567642");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn ln_gamma_round(self, rm: RoundingMode) -> (Self, Ordering) {
        let prec = self.significant_bits();
        self.ln_gamma_prec_round(prec, rm)
    }

    /// Computes $\ln \Gamma(x)$, the logarithm of the gamma function, of a [`Float`], rounding the
    /// result with the specified rounding mode. The [`Float`] is taken by reference. An
    /// [`Ordering`] is also returned, indicating whether the rounded value is less than, equal to,
    /// or greater than the exact value. Although `NaN`s are not comparable to any [`Float`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $\Gamma(x) = \int_0^\infty t^{x-1} e^{-t}\,dt$ for $x > 0$. Its logarithm is 0 at $x = 1$
    /// and $x = 2$, negative between them, and positive on the rest of the positive reals. This
    /// function is not defined for a negative $x$, even where $\Gamma(x)$ is positive;
    /// [`Float::ln_abs_gamma_prec_round`] gives $\ln |\Gamma(x)|$ for every $x$.
    ///
    /// The precision of the output is the precision of the input. See [`RoundingMode`] for a
    /// description of the possible rounding modes.
    ///
    /// $$
    /// f(x,m) = \ln \Gamma(x)+\varepsilon.
    /// $$
    /// - If $\ln \Gamma(x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $\ln \Gamma(x)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\ln \Gamma(x)|\rfloor-p+1}$, where $p$ is the precision of the input.
    /// - If $\ln \Gamma(x)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\ln \Gamma(x)|\rfloor-p}$, where $p$ is the precision of the input.
    ///
    /// If the output has a precision, it is the precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN},m)=\text{NaN}$
    /// - $f(\infty,m)=\infty$
    /// - $f(-\infty,m)=\text{NaN}$
    /// - $f(\pm0.0,m)=\infty$
    /// - $f(x,m)=\text{NaN}$ if $x<0$
    /// - $f(1,m)=f(2,m)=0.0$
    ///
    /// See the [`Float::ln_gamma_prec_round`] documentation for information on overflow and
    /// underflow, and on termination.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::ln_gamma_prec_round_ref`] instead. If you know you'll be using the `Nearest`
    /// rounding mode, consider using `(&Float).ln_gamma()` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^3 \log n)$
    ///
    /// $M(n) = O(n^2 \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the result cannot be represented exactly with the input
    /// precision (it can only when it is infinite or `NaN`, or when $x$ is 1 or 2).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (g, o) = (Float::one_half_prec(100)).ln_gamma_round_ref(Floor);
    /// assert_eq!(g.to_string(), "0.57236494292470008707171367567642");
    /// assert_eq!(o, Less);
    ///
    /// let (g, o) = (Float::one_half_prec(100)).ln_gamma_round_ref(Ceiling);
    /// assert_eq!(g.to_string(), "0.57236494292470008707171367567721");
    /// assert_eq!(o, Greater);
    ///
    /// let (g, o) = (Float::one_half_prec(100)).ln_gamma_round_ref(Nearest);
    /// assert_eq!(g.to_string(), "0.57236494292470008707171367567642");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn ln_gamma_round_ref(&self, rm: RoundingMode) -> (Self, Ordering) {
        self.ln_gamma_prec_round_ref(self.significant_bits(), rm)
    }

    /// Computes $\ln \Gamma(x)$, the logarithm of the gamma function, of a [`Float`] in place,
    /// rounding the result to the specified precision and with the specified rounding mode. An
    /// [`Ordering`] is returned, indicating whether the rounded value is less than, equal to, or
    /// greater than the exact value. Although `NaN`s are not comparable to any [`Float`], whenever
    /// this function sets the [`Float`] to `NaN` it also returns `Equal`.
    ///
    /// $\Gamma(x) = \int_0^\infty t^{x-1} e^{-t}\,dt$ for $x > 0$. Its logarithm is 0 at $x = 1$
    /// and $x = 2$, negative between them, and positive on the rest of the positive reals. This
    /// function is not defined for a negative $x$, even where $\Gamma(x)$ is positive;
    /// [`Float::ln_abs_gamma_prec_round`] gives $\ln |\Gamma(x)|$ for every $x$.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets \ln \Gamma(x)+\varepsilon.
    /// $$
    /// - If $\ln \Gamma(x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $\ln \Gamma(x)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\ln \Gamma(x)|\rfloor-p+1}$.
    /// - If $\ln \Gamma(x)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\ln \Gamma(x)|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// See the [`Float::ln_gamma_prec_round`] documentation for information on special cases,
    /// overflow and underflow, and termination.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::ln_gamma_prec_assign`]
    /// instead. If you know that your target precision is the precision of the input, consider
    /// using [`Float::ln_gamma_round_assign`] instead. If both of these things are true, consider
    /// using [`Float::ln_gamma_assign`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^3 \log n)$
    ///
    /// $M(n) = O(n^2 \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec,
    /// self.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with the given precision (it can only when it is infinite or `NaN`, or when $x$ is 1 or 2).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::one_half_prec(100);
    /// assert_eq!(x.ln_gamma_prec_round_assign(5, Floor), Less);
    /// assert_eq!(x.to_string(), "0.562");
    ///
    /// let mut x = Float::one_half_prec(100);
    /// assert_eq!(x.ln_gamma_prec_round_assign(5, Ceiling), Greater);
    /// assert_eq!(x.to_string(), "0.594");
    ///
    /// let mut x = Float::one_half_prec(100);
    /// assert_eq!(x.ln_gamma_prec_round_assign(5, Nearest), Less);
    /// assert_eq!(x.to_string(), "0.562");
    ///
    /// let mut x = Float::one_half_prec(100);
    /// assert_eq!(x.ln_gamma_prec_round_assign(20, Floor), Less);
    /// assert_eq!(x.to_string(), "0.57236481");
    ///
    /// let mut x = Float::one_half_prec(100);
    /// assert_eq!(x.ln_gamma_prec_round_assign(20, Ceiling), Greater);
    /// assert_eq!(x.to_string(), "0.57236576");
    ///
    /// let mut x = Float::one_half_prec(100);
    /// assert_eq!(x.ln_gamma_prec_round_assign(20, Nearest), Less);
    /// assert_eq!(x.to_string(), "0.57236481");
    /// ```
    #[inline]
    pub fn ln_gamma_prec_round_assign(&mut self, prec: u64, rm: RoundingMode) -> Ordering {
        let (result, o) = core::mem::take(self).ln_gamma_prec_round(prec, rm);
        *self = result;
        o
    }

    /// Computes $\ln \Gamma(x)$, the logarithm of the gamma function, of a [`Float`] in place,
    /// rounding the result to the nearest value of the specified precision. An [`Ordering`] is
    /// returned, indicating whether the rounded value is less than, equal to, or greater than the
    /// exact value. Although `NaN`s are not comparable to any [`Float`], whenever this function
    /// sets the [`Float`] to `NaN` it also returns `Equal`.
    ///
    /// $\Gamma(x) = \int_0^\infty t^{x-1} e^{-t}\,dt$ for $x > 0$. Its logarithm is 0 at $x = 1$
    /// and $x = 2$, negative between them, and positive on the rest of the positive reals. This
    /// function is not defined for a negative $x$, even where $\Gamma(x)$ is positive;
    /// [`Float::ln_abs_gamma_prec_round`] gives $\ln |\Gamma(x)|$ for every $x$.
    ///
    /// If the value is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// x \gets \ln \Gamma(x)+\varepsilon.
    /// $$
    /// - If $\ln \Gamma(x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $\ln \Gamma(x)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2 |\ln
    ///   \Gamma(x)|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// See the [`Float::ln_gamma_prec_round`] documentation for information on special cases,
    /// overflow and underflow, and termination.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::ln_gamma_prec_round_assign`] instead. If you know that your target precision is the
    /// precision of the input, consider using [`Float::ln_gamma_assign`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^3 \log n)$
    ///
    /// $M(n) = O(n^2 \log n)$
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
    /// let mut x = Float::one_half_prec(100);
    /// assert_eq!(x.ln_gamma_prec_assign(5), Less);
    /// assert_eq!(x.to_string(), "0.562");
    ///
    /// let mut x = Float::one_half_prec(100);
    /// assert_eq!(x.ln_gamma_prec_assign(20), Less);
    /// assert_eq!(x.to_string(), "0.57236481");
    /// ```
    #[inline]
    pub fn ln_gamma_prec_assign(&mut self, prec: u64) -> Ordering {
        self.ln_gamma_prec_round_assign(prec, Nearest)
    }

    /// Computes $\ln \Gamma(x)$, the logarithm of the gamma function, of a [`Float`] in place,
    /// rounding the result with the specified rounding mode. An [`Ordering`] is returned,
    /// indicating whether the rounded value is less than, equal to, or greater than the exact
    /// value. Although `NaN`s are not comparable to any [`Float`], whenever this function sets the
    /// [`Float`] to `NaN` it also returns `Equal`.
    ///
    /// $\Gamma(x) = \int_0^\infty t^{x-1} e^{-t}\,dt$ for $x > 0$. Its logarithm is 0 at $x = 1$
    /// and $x = 2$, negative between them, and positive on the rest of the positive reals. This
    /// function is not defined for a negative $x$, even where $\Gamma(x)$ is positive;
    /// [`Float::ln_abs_gamma_prec_round`] gives $\ln |\Gamma(x)|$ for every $x$.
    ///
    /// The precision of the output is the precision of the input. See [`RoundingMode`] for a
    /// description of the possible rounding modes.
    ///
    /// $$
    /// x \gets \ln \Gamma(x)+\varepsilon.
    /// $$
    /// - If $\ln \Gamma(x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $\ln \Gamma(x)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\ln \Gamma(x)|\rfloor-p+1}$, where $p$ is the precision of the input.
    /// - If $\ln \Gamma(x)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\ln \Gamma(x)|\rfloor-p}$, where $p$ is the precision of the input.
    ///
    /// If the output has a precision, it is the precision of the input.
    ///
    /// See the [`Float::ln_gamma_prec_round`] documentation for information on special cases,
    /// overflow and underflow, and termination.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::ln_gamma_prec_round_assign`] instead. If you know you'll be using the `Nearest`
    /// rounding mode, consider using [`Float::ln_gamma_assign`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^3 \log n)$
    ///
    /// $M(n) = O(n^2 \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the result cannot be represented exactly with the input
    /// precision (it can only when it is infinite or `NaN`, or when $x$ is 1 or 2).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::one_half_prec(100);
    /// assert_eq!(x.ln_gamma_round_assign(Floor), Less);
    /// assert_eq!(x.to_string(), "0.57236494292470008707171367567642");
    ///
    /// let mut x = Float::one_half_prec(100);
    /// assert_eq!(x.ln_gamma_round_assign(Ceiling), Greater);
    /// assert_eq!(x.to_string(), "0.57236494292470008707171367567721");
    ///
    /// let mut x = Float::one_half_prec(100);
    /// assert_eq!(x.ln_gamma_round_assign(Nearest), Less);
    /// assert_eq!(x.to_string(), "0.57236494292470008707171367567642");
    /// ```
    #[inline]
    pub fn ln_gamma_round_assign(&mut self, rm: RoundingMode) -> Ordering {
        let prec = self.significant_bits();
        self.ln_gamma_prec_round_assign(prec, rm)
    }

    /// Computes $\ln \Gamma(x)$, the logarithm of the gamma function, of a [`Rational`], rounding
    /// the result to the specified precision and with the specified rounding mode and returning the
    /// result as a [`Float`]. The [`Rational`] is taken by value. An [`Ordering`] is also returned,
    /// indicating whether the rounded value is less than, equal to, or greater than the exact
    /// value. Although `NaN`s are not comparable to any [`Float`], whenever this function returns a
    /// `NaN` it also returns `Equal`.
    ///
    /// $\Gamma(x) = \int_0^\infty t^{x-1} e^{-t}\,dt$ for $x > 0$. Its logarithm is 0 at $x = 1$
    /// and $x = 2$, negative between them, and positive on the rest of the positive reals. This
    /// function is not defined for a negative $x$, even where $\Gamma(x)$ is positive;
    /// [`Float::ln_abs_gamma_rational_prec_round`] gives $\ln |\Gamma(x)|$ for every $x$.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \ln \Gamma(x)+\varepsilon.
    /// $$
    /// - If $\ln \Gamma(x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $\ln \Gamma(x)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\ln \Gamma(x)|\rfloor-p+1}$.
    /// - If $\ln \Gamma(x)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\ln \Gamma(x)|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(0,p,m)=\infty$
    /// - $f(x,p,m)=\text{NaN}$ if $x<0$
    /// - $f(1,p,m)=f(2,p,m)=0.0$
    ///
    /// Overflow and underflow:
    /// - If $f(x,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$ is
    ///   returned instead.
    /// - If $0<f(x,p,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,p,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,p,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,p,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,p,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,p,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,p,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,p,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is
    ///   returned instead.
    ///
    /// Overflow happens for arguments above about $2^{2^{30}-30.5}$. Underflow only happens for an
    /// argument within about $2^{-2^{30}}$ of one of the zeros 1 and 2.
    ///
    /// The result is correctly rounded. Finding it takes $\ln \Gamma(x)$ not to be a rounding
    /// boundary, which is a number with at most `prec + 1` significant bits. This is known when $x$
    /// is a positive integer (then the value is the logarithm of an integer, which is 0 or
    /// irrational); nothing suggests that it ever fails, but it has not been proven for every
    /// rational $x$, and this function would not terminate if it did fail.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::ln_gamma_rational_prec`]
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^3 \log n)$
    ///
    /// $M(n) = O(n^2 \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec, x.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with the given precision (it can only when it is infinite or `NaN`, or when $x$ is 1 or 2).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (g, o) =
    ///     Float::ln_gamma_rational_prec_round(Rational::from_unsigneds(1u8, 3), 5, Floor);
    /// assert_eq!(g.to_string(), "0.969");
    /// assert_eq!(o, Less);
    ///
    /// let (g, o) =
    ///     Float::ln_gamma_rational_prec_round(Rational::from_unsigneds(1u8, 3), 5, Ceiling);
    /// assert_eq!(g.to_string(), "1.00");
    /// assert_eq!(o, Greater);
    ///
    /// let (g, o) =
    ///     Float::ln_gamma_rational_prec_round(Rational::from_unsigneds(1u8, 3), 5, Nearest);
    /// assert_eq!(g.to_string(), "1.00");
    /// assert_eq!(o, Greater);
    ///
    /// let (g, o) =
    ///     Float::ln_gamma_rational_prec_round(Rational::from_unsigneds(1u8, 3), 20, Floor);
    /// assert_eq!(g.to_string(), "0.98542023");
    /// assert_eq!(o, Less);
    ///
    /// let (g, o) =
    ///     Float::ln_gamma_rational_prec_round(Rational::from_unsigneds(1u8, 3), 20, Ceiling);
    /// assert_eq!(g.to_string(), "0.98542118");
    /// assert_eq!(o, Greater);
    ///
    /// let (g, o) =
    ///     Float::ln_gamma_rational_prec_round(Rational::from_unsigneds(1u8, 3), 20, Nearest);
    /// assert_eq!(g.to_string(), "0.98542023");
    /// assert_eq!(o, Less);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn ln_gamma_rational_prec_round(
        x: Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        let (y, _, o) = ln_gamma_rational_prec_round_helper(false, &x, prec, rm);
        (y, o)
    }

    /// Computes $\ln \Gamma(x)$, the logarithm of the gamma function, of a [`Rational`], rounding
    /// the result to the specified precision and with the specified rounding mode and returning the
    /// result as a [`Float`]. The [`Rational`] is taken by reference. An [`Ordering`] is also
    /// returned, indicating whether the rounded value is less than, equal to, or greater than the
    /// exact value. Although `NaN`s are not comparable to any [`Float`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// $\Gamma(x) = \int_0^\infty t^{x-1} e^{-t}\,dt$ for $x > 0$. Its logarithm is 0 at $x = 1$
    /// and $x = 2$, negative between them, and positive on the rest of the positive reals. This
    /// function is not defined for a negative $x$, even where $\Gamma(x)$ is positive;
    /// [`Float::ln_abs_gamma_rational_prec_round`] gives $\ln |\Gamma(x)|$ for every $x$.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \ln \Gamma(x)+\varepsilon.
    /// $$
    /// - If $\ln \Gamma(x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $\ln \Gamma(x)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\ln \Gamma(x)|\rfloor-p+1}$.
    /// - If $\ln \Gamma(x)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\ln \Gamma(x)|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(0,p,m)=\infty$
    /// - $f(x,p,m)=\text{NaN}$ if $x<0$
    /// - $f(1,p,m)=f(2,p,m)=0.0$
    ///
    /// Overflow and underflow:
    /// - If $f(x,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$ is
    ///   returned instead.
    /// - If $0<f(x,p,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,p,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,p,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,p,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,p,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,p,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,p,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,p,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is
    ///   returned instead.
    ///
    /// Overflow happens for arguments above about $2^{2^{30}-30.5}$. Underflow only happens for an
    /// argument within about $2^{-2^{30}}$ of one of the zeros 1 and 2.
    ///
    /// The result is correctly rounded. Finding it takes $\ln \Gamma(x)$ not to be a rounding
    /// boundary, which is a number with at most `prec + 1` significant bits. This is known when $x$
    /// is a positive integer (then the value is the logarithm of an integer, which is 0 or
    /// irrational); nothing suggests that it ever fails, but it has not been proven for every
    /// rational $x$, and this function would not terminate if it did fail.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::ln_gamma_rational_prec_ref`]
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^3 \log n)$
    ///
    /// $M(n) = O(n^2 \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec, x.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with the given precision (it can only when it is infinite or `NaN`, or when $x$ is 1 or 2).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (g, o) =
    ///     Float::ln_gamma_rational_prec_round_ref(&Rational::from_unsigneds(1u8, 3), 5, Floor);
    /// assert_eq!(g.to_string(), "0.969");
    /// assert_eq!(o, Less);
    ///
    /// let (g, o) =
    ///     Float::ln_gamma_rational_prec_round_ref(&Rational::from_unsigneds(1u8, 3), 5, Ceiling);
    /// assert_eq!(g.to_string(), "1.00");
    /// assert_eq!(o, Greater);
    ///
    /// let (g, o) =
    ///     Float::ln_gamma_rational_prec_round_ref(&Rational::from_unsigneds(1u8, 3), 5, Nearest);
    /// assert_eq!(g.to_string(), "1.00");
    /// assert_eq!(o, Greater);
    ///
    /// let (g, o) =
    ///     Float::ln_gamma_rational_prec_round_ref(&Rational::from_unsigneds(1u8, 3), 20, Floor);
    /// assert_eq!(g.to_string(), "0.98542023");
    /// assert_eq!(o, Less);
    ///
    /// let (g, o) =
    ///     Float::ln_gamma_rational_prec_round_ref(&Rational::from_unsigneds(1u8, 3), 20, Ceiling);
    /// assert_eq!(g.to_string(), "0.98542118");
    /// assert_eq!(o, Greater);
    ///
    /// let (g, o) =
    ///     Float::ln_gamma_rational_prec_round_ref(&Rational::from_unsigneds(1u8, 3), 20, Nearest);
    /// assert_eq!(g.to_string(), "0.98542023");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn ln_gamma_rational_prec_round_ref(
        x: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        let (y, _, o) = ln_gamma_rational_prec_round_helper(false, x, prec, rm);
        (y, o)
    }

    /// Computes $\ln \Gamma(x)$, the logarithm of the gamma function, of a [`Rational`], rounding
    /// the result to the nearest value of the specified precision and returning the result as a
    /// [`Float`]. The [`Rational`] is taken by value. An [`Ordering`] is also returned, indicating
    /// whether the rounded value is less than, equal to, or greater than the exact value. Although
    /// `NaN`s are not comparable to any [`Float`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// $\Gamma(x) = \int_0^\infty t^{x-1} e^{-t}\,dt$ for $x > 0$. Its logarithm is 0 at $x = 1$
    /// and $x = 2$, negative between them, and positive on the rest of the positive reals. This
    /// function is not defined for a negative $x$, even where $\Gamma(x)$ is positive;
    /// [`Float::ln_abs_gamma_rational_prec_round`] gives $\ln |\Gamma(x)|$ for every $x$.
    ///
    /// If the value is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = \ln \Gamma(x)+\varepsilon.
    /// $$
    /// - If $\ln \Gamma(x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $\ln \Gamma(x)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2 |\ln
    ///   \Gamma(x)|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(0,p)=\infty$
    /// - $f(x,p)=\text{NaN}$ if $x<0$
    /// - $f(1,p)=f(2,p)=0.0$
    ///
    /// Overflow and underflow:
    /// - If $f(x,p)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $0<f(x,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// Overflow happens for arguments above about $2^{2^{30}-30.5}$. Underflow only happens for an
    /// argument within about $2^{-2^{30}}$ of one of the zeros 1 and 2.
    ///
    /// The result is correctly rounded. Finding it takes $\ln \Gamma(x)$ not to be a rounding
    /// boundary, which is a number with at most `prec + 1` significant bits. This is known when $x$
    /// is a positive integer (then the value is the logarithm of an integer, which is 0 or
    /// irrational); nothing suggests that it ever fails, but it has not been proven for every
    /// rational $x$, and this function would not terminate if it did fail.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::ln_gamma_rational_prec_round`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^3 \log n)$
    ///
    /// $M(n) = O(n^2 \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec, x.significant_bits())`.
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
    /// let (g, o) = Float::ln_gamma_rational_prec(Rational::from_unsigneds(1u8, 3), 5);
    /// assert_eq!(g.to_string(), "1.00");
    /// assert_eq!(o, Greater);
    ///
    /// let (g, o) = Float::ln_gamma_rational_prec(Rational::from_unsigneds(1u8, 3), 20);
    /// assert_eq!(g.to_string(), "0.98542023");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn ln_gamma_rational_prec(x: Rational, prec: u64) -> (Self, Ordering) {
        Self::ln_gamma_rational_prec_round(x, prec, Nearest)
    }

    /// Computes $\ln \Gamma(x)$, the logarithm of the gamma function, of a [`Rational`], rounding
    /// the result to the nearest value of the specified precision and returning the result as a
    /// [`Float`]. The [`Rational`] is taken by reference. An [`Ordering`] is also returned,
    /// indicating whether the rounded value is less than, equal to, or greater than the exact
    /// value. Although `NaN`s are not comparable to any [`Float`], whenever this function returns a
    /// `NaN` it also returns `Equal`.
    ///
    /// $\Gamma(x) = \int_0^\infty t^{x-1} e^{-t}\,dt$ for $x > 0$. Its logarithm is 0 at $x = 1$
    /// and $x = 2$, negative between them, and positive on the rest of the positive reals. This
    /// function is not defined for a negative $x$, even where $\Gamma(x)$ is positive;
    /// [`Float::ln_abs_gamma_rational_prec_round`] gives $\ln |\Gamma(x)|$ for every $x$.
    ///
    /// If the value is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = \ln \Gamma(x)+\varepsilon.
    /// $$
    /// - If $\ln \Gamma(x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $\ln \Gamma(x)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2 |\ln
    ///   \Gamma(x)|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(0,p)=\infty$
    /// - $f(x,p)=\text{NaN}$ if $x<0$
    /// - $f(1,p)=f(2,p)=0.0$
    ///
    /// Overflow and underflow:
    /// - If $f(x,p)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $0<f(x,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// Overflow happens for arguments above about $2^{2^{30}-30.5}$. Underflow only happens for an
    /// argument within about $2^{-2^{30}}$ of one of the zeros 1 and 2.
    ///
    /// The result is correctly rounded. Finding it takes $\ln \Gamma(x)$ not to be a rounding
    /// boundary, which is a number with at most `prec + 1` significant bits. This is known when $x$
    /// is a positive integer (then the value is the logarithm of an integer, which is 0 or
    /// irrational); nothing suggests that it ever fails, but it has not been proven for every
    /// rational $x$, and this function would not terminate if it did fail.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::ln_gamma_rational_prec_round_ref`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^3 \log n)$
    ///
    /// $M(n) = O(n^2 \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec, x.significant_bits())`.
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
    /// let (g, o) = Float::ln_gamma_rational_prec_ref(&Rational::from_unsigneds(1u8, 3), 5);
    /// assert_eq!(g.to_string(), "1.00");
    /// assert_eq!(o, Greater);
    ///
    /// let (g, o) = Float::ln_gamma_rational_prec_ref(&Rational::from_unsigneds(1u8, 3), 20);
    /// assert_eq!(g.to_string(), "0.98542023");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn ln_gamma_rational_prec_ref(x: &Rational, prec: u64) -> (Self, Ordering) {
        Self::ln_gamma_rational_prec_round_ref(x, prec, Nearest)
    }
}

impl LnGamma for Float {
    type Output = Self;

    /// Computes $\ln \Gamma(x)$, the logarithm of the gamma function, of a [`Float`], taking it by
    /// value.
    ///
    /// $\Gamma(x) = \int_0^\infty t^{x-1} e^{-t}\,dt$ for $x > 0$. Its logarithm is 0 at $x = 1$
    /// and $x = 2$, negative between them, and positive on the rest of the positive reals. This
    /// function is not defined for a negative $x$, even where $\Gamma(x)$ is positive;
    /// [`Float::ln_abs_gamma_prec_round`] gives $\ln |\Gamma(x)|$ for every $x$.
    ///
    /// If the output has a precision, it is the precision of the input. If the value is equidistant
    /// from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s in its binary
    /// expansion is chosen. See [`RoundingMode`] for a description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x) = \ln \Gamma(x)+\varepsilon.
    /// $$
    /// - If $\ln \Gamma(x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $\ln \Gamma(x)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2 |\ln
    ///   \Gamma(x)|\rfloor-p}$, where $p$ is the precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN})=\text{NaN}$
    /// - $f(\infty)=\infty$
    /// - $f(-\infty)=\text{NaN}$
    /// - $f(\pm0.0)=\infty$
    /// - $f(x)=\text{NaN}$ if $x<0$
    /// - $f(1)=f(2)=0.0$
    ///
    /// See the [`Float::ln_gamma_prec_round`] documentation for information on overflow and
    /// underflow, and on termination.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::ln_gamma_round`] instead. If you want to specify the output precision, consider
    /// using [`Float::ln_gamma_prec`]. If you want both of these things, consider using
    /// [`Float::ln_gamma_prec_round`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^3 \log n)$
    ///
    /// $M(n) = O(n^2 \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{Infinity, NaN, NegativeOne, One, Zero};
    /// use malachite_base::num::gamma::traits::LnGamma;
    /// use malachite_float::Float;
    ///
    /// assert!(Float::NAN.ln_gamma().is_nan());
    /// assert_eq!(Float::INFINITY.ln_gamma(), Float::INFINITY);
    /// assert_eq!(Float::ZERO.ln_gamma(), Float::INFINITY);
    /// assert!(Float::NEGATIVE_ONE.ln_gamma().is_nan());
    /// assert_eq!(Float::ONE.ln_gamma(), Float::ZERO);
    /// assert_eq!(
    ///     (Float::one_half_prec(100)).ln_gamma().to_string(),
    ///     "0.57236494292470008707171367567642"
    /// );
    /// ```
    #[inline]
    fn ln_gamma(self) -> Self {
        let prec = self.significant_bits();
        self.ln_gamma_prec(prec).0
    }
}

impl LnGamma for &Float {
    type Output = Float;

    /// Computes $\ln \Gamma(x)$, the logarithm of the gamma function, of a [`Float`], taking it by
    /// reference.
    ///
    /// $\Gamma(x) = \int_0^\infty t^{x-1} e^{-t}\,dt$ for $x > 0$. Its logarithm is 0 at $x = 1$
    /// and $x = 2$, negative between them, and positive on the rest of the positive reals. This
    /// function is not defined for a negative $x$, even where $\Gamma(x)$ is positive;
    /// [`Float::ln_abs_gamma_prec_round`] gives $\ln |\Gamma(x)|$ for every $x$.
    ///
    /// If the output has a precision, it is the precision of the input. If the value is equidistant
    /// from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s in its binary
    /// expansion is chosen. See [`RoundingMode`] for a description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x) = \ln \Gamma(x)+\varepsilon.
    /// $$
    /// - If $\ln \Gamma(x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $\ln \Gamma(x)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2 |\ln
    ///   \Gamma(x)|\rfloor-p}$, where $p$ is the precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN})=\text{NaN}$
    /// - $f(\infty)=\infty$
    /// - $f(-\infty)=\text{NaN}$
    /// - $f(\pm0.0)=\infty$
    /// - $f(x)=\text{NaN}$ if $x<0$
    /// - $f(1)=f(2)=0.0$
    ///
    /// See the [`Float::ln_gamma_prec_round`] documentation for information on overflow and
    /// underflow, and on termination.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::ln_gamma_round_ref`] instead. If you want to specify the output precision, consider
    /// using [`Float::ln_gamma_prec_ref`]. If you want both of these things, consider using
    /// [`Float::ln_gamma_prec_round_ref`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^3 \log n)$
    ///
    /// $M(n) = O(n^2 \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{Infinity, NaN, NegativeOne, One, Zero};
    /// use malachite_base::num::gamma::traits::LnGamma;
    /// use malachite_float::Float;
    ///
    /// assert!((&Float::NAN).ln_gamma().is_nan());
    /// assert_eq!((&Float::INFINITY).ln_gamma(), Float::INFINITY);
    /// assert_eq!((&Float::ZERO).ln_gamma(), Float::INFINITY);
    /// assert!((&Float::NEGATIVE_ONE).ln_gamma().is_nan());
    /// assert_eq!((&Float::ONE).ln_gamma(), Float::ZERO);
    /// assert_eq!(
    ///     (&(Float::one_half_prec(100))).ln_gamma().to_string(),
    ///     "0.57236494292470008707171367567642"
    /// );
    /// ```
    #[inline]
    fn ln_gamma(self) -> Float {
        self.ln_gamma_prec_ref(self.significant_bits()).0
    }
}

impl LnGammaAssign for Float {
    /// Computes $\ln \Gamma(x)$, the logarithm of the gamma function, of a [`Float`] in place.
    ///
    /// $\Gamma(x) = \int_0^\infty t^{x-1} e^{-t}\,dt$ for $x > 0$. Its logarithm is 0 at $x = 1$
    /// and $x = 2$, negative between them, and positive on the rest of the positive reals. This
    /// function is not defined for a negative $x$, even where $\Gamma(x)$ is positive;
    /// [`Float::ln_abs_gamma_prec_round`] gives $\ln |\Gamma(x)|$ for every $x$.
    ///
    /// If the output has a precision, it is the precision of the input. If the value is equidistant
    /// from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s in its binary
    /// expansion is chosen. See [`RoundingMode`] for a description of the `Nearest` rounding mode.
    ///
    /// $$
    /// x \gets \ln \Gamma(x)+\varepsilon.
    /// $$
    /// - If $\ln \Gamma(x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $\ln \Gamma(x)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2 |\ln
    ///   \Gamma(x)|\rfloor-p}$, where $p$ is the precision of the input.
    ///
    /// See the [`Float::ln_gamma_prec_round`] documentation for information on special cases,
    /// overflow and underflow, and termination.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::ln_gamma_round_assign`] instead. If you want to specify the output precision,
    /// consider using [`Float::ln_gamma_prec_assign`]. If you want both of these things, consider
    /// using [`Float::ln_gamma_prec_round_assign`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^3 \log n)$
    ///
    /// $M(n) = O(n^2 \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{Infinity, NaN, NegativeOne, One, Zero};
    /// use malachite_base::num::gamma::traits::LnGammaAssign;
    /// use malachite_float::Float;
    ///
    /// let mut x = Float::NAN;
    /// x.ln_gamma_assign();
    /// assert!(x.is_nan());
    ///
    /// let mut x = Float::INFINITY;
    /// x.ln_gamma_assign();
    /// assert_eq!(x, Float::INFINITY);
    ///
    /// let mut x = Float::ZERO;
    /// x.ln_gamma_assign();
    /// assert_eq!(x, Float::INFINITY);
    ///
    /// let mut x = Float::NEGATIVE_ONE;
    /// x.ln_gamma_assign();
    /// assert!(x.is_nan());
    ///
    /// let mut x = Float::ONE;
    /// x.ln_gamma_assign();
    /// assert_eq!(x, Float::ZERO);
    ///
    /// let mut x = Float::one_half_prec(100);
    /// x.ln_gamma_assign();
    /// assert_eq!(x.to_string(), "0.57236494292470008707171367567642");
    /// ```
    #[inline]
    fn ln_gamma_assign(&mut self) {
        let prec = self.significant_bits();
        self.ln_gamma_prec_assign(prec);
    }
}

/// Computes $\ln \Gamma(x)$, the logarithm of the gamma function, of a primitive float.
///
/// $\Gamma(x) = \int_0^\infty t^{x-1} e^{-t}\,dt$ for $x > 0$. Its logarithm is 0 at $x = 1$ and $x
/// = 2$, negative between them, and positive on the rest of the positive reals. This function is
/// not defined for a negative $x$, even where $\Gamma(x)$ is positive;
/// [`Float::ln_abs_gamma_prec_round`] gives $\ln |\Gamma(x)|$ for every $x$.
///
/// $$
/// f(x) = \ln \Gamma(x)+\varepsilon.
/// $$
/// - If $\ln \Gamma(x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
///   0.
/// - If $\ln \Gamma(x)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2 |\ln
///   \Gamma(x)|\rfloor-p}$, where $p$ is the precision of the output (typically 24 if `T` is a
///   [`f32`] and 53 if `T` is a [`f64`], but less if the output is subnormal).
///
/// Special cases:
/// - $f(\text{NaN})=\text{NaN}$
/// - $f(\infty)=\infty$
/// - $f(-\infty)=\text{NaN}$
/// - $f(\pm0.0)=\infty$
/// - $f(x)=\text{NaN}$ if $x<0$
/// - $f(1)=f(2)=0.0$
///
/// The result is $\infty$ when $\ln \Gamma(x)$ is too large for `T`: for $x$ above about
/// $4\times10^{36}$ (`f32`) or $2.5\times10^{305}$ (`f64`).
///
/// See the [`Float::ln_gamma_prec_round`] documentation for information on termination.
///
/// # Worst-case complexity
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use malachite_base::num::float::NiceFloat;
/// use malachite_float::float::gamma::ln_gamma::primitive_float_ln_gamma;
///
/// assert!(primitive_float_ln_gamma(f32::NAN).is_nan());
/// assert_eq!(
///     NiceFloat(primitive_float_ln_gamma(f32::INFINITY)),
///     NiceFloat(f32::INFINITY)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_ln_gamma(0.0f32)),
///     NiceFloat(f32::INFINITY)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_ln_gamma(-0.0f32)),
///     NiceFloat(f32::INFINITY)
/// );
/// assert_eq!(NiceFloat(primitive_float_ln_gamma(1.0f32)), NiceFloat(0.0));
/// assert_eq!(
///     NiceFloat(primitive_float_ln_gamma(0.5f32)),
///     NiceFloat(0.5723649)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_ln_gamma(5.0f32)),
///     NiceFloat(3.1780539)
/// );
/// assert!(primitive_float_ln_gamma(-1.0f32).is_nan());
/// assert!(primitive_float_ln_gamma(-0.5f32).is_nan());
/// ```
#[inline]
#[allow(clippy::type_repetition_in_bounds)]
pub fn primitive_float_ln_gamma<T: PrimitiveFloat>(x: T) -> T
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    emulate_float_to_float_fn(Float::ln_gamma_prec, x)
}

/// Computes $\ln \Gamma(x)$, the logarithm of the gamma function, of a [`Rational`], returning the
/// result as a primitive float.
///
/// $\Gamma(x) = \int_0^\infty t^{x-1} e^{-t}\,dt$ for $x > 0$. Its logarithm is 0 at $x = 1$ and $x
/// = 2$, negative between them, and positive on the rest of the positive reals. This function is
/// not defined for a negative $x$, even where $\Gamma(x)$ is positive;
/// [`Float::ln_abs_gamma_rational_prec_round`] gives $\ln |\Gamma(x)|$ for every $x$.
///
/// $$
/// f(x) = \ln \Gamma(x)+\varepsilon.
/// $$
/// - If $\ln \Gamma(x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
///   0.
/// - If $\ln \Gamma(x)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2 |\ln
///   \Gamma(x)|\rfloor-p}$, where $p$ is the precision of the output (typically 24 if `T` is a
///   [`f32`] and 53 if `T` is a [`f64`], but less if the output is subnormal).
///
/// Special cases:
/// - $f(0)=\infty$
/// - $f(x)=\text{NaN}$ if $x<0$
/// - $f(1)=f(2)=0.0$
///
/// The result is $\infty$ when $\ln \Gamma(x)$ is too large for `T`: for $x$ above about
/// $4\times10^{36}$ (`f32`) or $2.5\times10^{305}$ (`f64`).
///
/// See the [`Float::ln_gamma_rational_prec_round`] documentation for information on termination.
///
/// # Worst-case complexity
/// $T(n) = O(n^3 \log n)$
///
/// $M(n) = O(n^2 \log n)$
///
/// where $T$ is time, $M$ is additional memory, and $n$ is `x.significant_bits()`.
///
/// # Examples
/// ```
/// use malachite_base::num::basic::traits::{NegativeOne, One, Zero};
/// use malachite_base::num::float::NiceFloat;
/// use malachite_float::float::gamma::ln_gamma::primitive_float_ln_gamma_rational;
/// use malachite_q::Rational;
///
/// assert_eq!(
///     NiceFloat(primitive_float_ln_gamma_rational::<f64>(&Rational::ZERO)),
///     NiceFloat(f64::INFINITY)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_ln_gamma_rational::<f64>(&Rational::ONE)),
///     NiceFloat(0.0)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_ln_gamma_rational::<f64>(
///         &Rational::from_unsigneds(1u8, 3)
///     )),
///     NiceFloat(0.9854206469277671)
/// );
/// assert!(primitive_float_ln_gamma_rational::<f64>(&Rational::from_signeds(-1i8, 3)).is_nan());
/// assert!(primitive_float_ln_gamma_rational::<f64>(&Rational::NEGATIVE_ONE).is_nan());
/// assert_eq!(
///     NiceFloat(primitive_float_ln_gamma_rational::<f64>(&Rational::from(5))),
///     NiceFloat(3.1780538303479458)
/// );
/// ```
#[inline]
#[allow(clippy::type_repetition_in_bounds)]
pub fn primitive_float_ln_gamma_rational<T: PrimitiveFloat>(x: &Rational) -> T
where
    Float: PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    emulate_rational_to_float_fn(Float::ln_gamma_rational_prec_ref, x)
}
