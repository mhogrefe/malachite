// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::float::gamma::ln_gamma::{
    ln_gamma_prec_round_helper, ln_gamma_rational_prec_round_helper,
};
use crate::{Float, emulate_float_to_float_fn, emulate_rational_to_float_fn};
use core::cell::Cell;
use core::cmp::Ordering;
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::conversion::traits::{ExactFrom, RoundingFrom};
use malachite_base::num::gamma::traits::{LnAbsGamma, LnAbsGammaAssign};
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_q::Rational;

impl Float {
    /// Computes $\ln |\Gamma(x)|$, the logarithm of the absolute value of the gamma function, of a
    /// [`Float`], rounding the result to the specified precision and with the specified rounding
    /// mode. The [`Float`] is taken by value. The sign of $\Gamma(x)$ and an [`Ordering`] are also
    /// returned, the latter indicating whether the rounded value is less than, equal to, or greater
    /// than the exact value. Although `NaN`s are not comparable to any [`Float`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// $\Gamma(x) = \int_0^\infty t^{x-1} e^{-t}\,dt$ for $x > 0$, and the recurrence $\Gamma(x) =
    /// \Gamma(x+1)/x$ extends it to every real $x$ other than $0, -1, -2, \ldots$, where it has
    /// poles and $\ln |\Gamma(x)|$ is taken to be $\infty$.
    ///
    /// The `bool` is the sign of $\Gamma(x)$: `true` if it is positive, which it is for $x > 0$ and
    /// for a negative $x$ that is not an integer and whose integer part is odd. Where $\Gamma(x)$
    /// has no sign (at the negative integers, at $\pm\infty$, and at `NaN`), `true` is returned; at
    /// $\pm0.0$ the sign is that of the zero, as $\Gamma(\pm0.0) = \pm\infty$.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = (\ln |\Gamma(x)|+\varepsilon, \operatorname{sgn} \Gamma(x)).
    /// $$
    /// - If $\ln |\Gamma(x)|$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed
    ///   to be 0.
    /// - If $\ln |\Gamma(x)|$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon|
    ///   < 2^{\lfloor\log_2 |\ln |\Gamma(x)||\rfloor-p+1}$.
    /// - If $\ln |\Gamma(x)|$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\ln |\Gamma(x)||\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=(\text{NaN},\text{true})$
    /// - $f(\pm\infty,p,m)=(\infty,\text{true})$
    /// - $f(0.0,p,m)=(\infty,\text{true})$
    /// - $f(-0.0,p,m)=(\infty,\text{false})$
    /// - $f(x,p,m)=(\infty,\text{true})$ if $x$ is a negative integer
    /// - $f(1,p,m)=f(2,p,m)=(0.0,\text{true})$
    ///
    /// Overflow and underflow, with $y$ the first element of $f(x,p,m)$:
    /// - If $y\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is returned
    ///   instead.
    /// - If $y\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$ is
    ///   returned instead.
    /// - If $y\leq -2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is returned
    ///   instead.
    /// - If $y\leq -2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`, $-(1-(1/2)^p)2^{2^{30}-1}$ is
    ///   returned instead.
    /// - If $0<y<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<y<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned instead.
    /// - If $0<y\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<y<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}}<y<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<y<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq y<0$, and $m$ is `Nearest`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<y<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is returned
    ///   instead.
    ///
    /// Overflow happens for arguments above about $2^{2^{30}-30.5}$ and, towards $-\infty$, for
    /// negative arguments of such a magnitude that are not integers. Underflow only happens for an
    /// argument within about $2^{-2^{30}}$ of a zero of $\ln |\Gamma(x)|$; such an argument has
    /// more than $2^{30}$ significant bits.
    ///
    /// The result is correctly rounded. Finding it takes $\ln |\Gamma(x)|$ not to be a rounding
    /// boundary, which is a number with at most `prec + 1` significant bits. This is known when $x$
    /// is a positive integer (then the value is the logarithm of an integer, which is 0 or
    /// irrational); nothing suggests that it ever fails, but it has not been proven for every $x$,
    /// and this function would not terminate if it did fail.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::ln_abs_gamma_prec`] instead.
    /// If you know that your target precision is the precision of the input, consider using
    /// [`Float::ln_abs_gamma_round`] instead. If both of these things are true, consider using
    /// [`Float::ln_abs_gamma`] instead.
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
    /// let (g, sign, o) = (-Float::one_half_prec(100)).ln_abs_gamma_prec_round(5, Floor);
    /// assert_eq!(g.to_string(), "1.25");
    /// assert!(!sign);
    /// assert_eq!(o, Less);
    ///
    /// let (g, sign, o) = (-Float::one_half_prec(100)).ln_abs_gamma_prec_round(5, Ceiling);
    /// assert_eq!(g.to_string(), "1.31");
    /// assert!(!sign);
    /// assert_eq!(o, Greater);
    ///
    /// let (g, sign, o) = (-Float::one_half_prec(100)).ln_abs_gamma_prec_round(5, Nearest);
    /// assert_eq!(g.to_string(), "1.25");
    /// assert!(!sign);
    /// assert_eq!(o, Less);
    ///
    /// let (g, sign, o) = (-Float::one_half_prec(100)).ln_abs_gamma_prec_round(20, Floor);
    /// assert_eq!(g.to_string(), "1.2655106");
    /// assert!(!sign);
    /// assert_eq!(o, Less);
    ///
    /// let (g, sign, o) = (-Float::one_half_prec(100)).ln_abs_gamma_prec_round(20, Ceiling);
    /// assert_eq!(g.to_string(), "1.2655125");
    /// assert!(!sign);
    /// assert_eq!(o, Greater);
    ///
    /// let (g, sign, o) = (-Float::one_half_prec(100)).ln_abs_gamma_prec_round(20, Nearest);
    /// assert_eq!(g.to_string(), "1.2655125");
    /// assert!(!sign);
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn ln_abs_gamma_prec_round(self, prec: u64, rm: RoundingMode) -> (Self, bool, Ordering) {
        ln_gamma_prec_round_helper(true, &self, prec, rm)
    }

    /// Computes $\ln |\Gamma(x)|$, the logarithm of the absolute value of the gamma function, of a
    /// [`Float`], rounding the result to the specified precision and with the specified rounding
    /// mode. The [`Float`] is taken by reference. The sign of $\Gamma(x)$ and an [`Ordering`] are
    /// also returned, the latter indicating whether the rounded value is less than, equal to, or
    /// greater than the exact value. Although `NaN`s are not comparable to any [`Float`], whenever
    /// this function returns a `NaN` it also returns `Equal`.
    ///
    /// $\Gamma(x) = \int_0^\infty t^{x-1} e^{-t}\,dt$ for $x > 0$, and the recurrence $\Gamma(x) =
    /// \Gamma(x+1)/x$ extends it to every real $x$ other than $0, -1, -2, \ldots$, where it has
    /// poles and $\ln |\Gamma(x)|$ is taken to be $\infty$.
    ///
    /// The `bool` is the sign of $\Gamma(x)$: `true` if it is positive, which it is for $x > 0$ and
    /// for a negative $x$ that is not an integer and whose integer part is odd. Where $\Gamma(x)$
    /// has no sign (at the negative integers, at $\pm\infty$, and at `NaN`), `true` is returned; at
    /// $\pm0.0$ the sign is that of the zero, as $\Gamma(\pm0.0) = \pm\infty$.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = (\ln |\Gamma(x)|+\varepsilon, \operatorname{sgn} \Gamma(x)).
    /// $$
    /// - If $\ln |\Gamma(x)|$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed
    ///   to be 0.
    /// - If $\ln |\Gamma(x)|$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon|
    ///   < 2^{\lfloor\log_2 |\ln |\Gamma(x)||\rfloor-p+1}$.
    /// - If $\ln |\Gamma(x)|$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\ln |\Gamma(x)||\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=(\text{NaN},\text{true})$
    /// - $f(\pm\infty,p,m)=(\infty,\text{true})$
    /// - $f(0.0,p,m)=(\infty,\text{true})$
    /// - $f(-0.0,p,m)=(\infty,\text{false})$
    /// - $f(x,p,m)=(\infty,\text{true})$ if $x$ is a negative integer
    /// - $f(1,p,m)=f(2,p,m)=(0.0,\text{true})$
    ///
    /// Overflow and underflow, with $y$ the first element of $f(x,p,m)$:
    /// - If $y\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is returned
    ///   instead.
    /// - If $y\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$ is
    ///   returned instead.
    /// - If $y\leq -2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is returned
    ///   instead.
    /// - If $y\leq -2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`, $-(1-(1/2)^p)2^{2^{30}-1}$ is
    ///   returned instead.
    /// - If $0<y<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<y<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned instead.
    /// - If $0<y\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<y<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}}<y<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<y<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq y<0$, and $m$ is `Nearest`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<y<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is returned
    ///   instead.
    ///
    /// Overflow happens for arguments above about $2^{2^{30}-30.5}$ and, towards $-\infty$, for
    /// negative arguments of such a magnitude that are not integers. Underflow only happens for an
    /// argument within about $2^{-2^{30}}$ of a zero of $\ln |\Gamma(x)|$; such an argument has
    /// more than $2^{30}$ significant bits.
    ///
    /// The result is correctly rounded. Finding it takes $\ln |\Gamma(x)|$ not to be a rounding
    /// boundary, which is a number with at most `prec + 1` significant bits. This is known when $x$
    /// is a positive integer (then the value is the logarithm of an integer, which is 0 or
    /// irrational); nothing suggests that it ever fails, but it has not been proven for every $x$,
    /// and this function would not terminate if it did fail.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::ln_abs_gamma_prec_ref`]
    /// instead. If you know that your target precision is the precision of the input, consider
    /// using [`Float::ln_abs_gamma_round_ref`] instead. If both of these things are true, consider
    /// using `(&Float).ln_abs_gamma()` instead.
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
    /// let (g, sign, o) = (-Float::one_half_prec(100)).ln_abs_gamma_prec_round_ref(5, Floor);
    /// assert_eq!(g.to_string(), "1.25");
    /// assert!(!sign);
    /// assert_eq!(o, Less);
    ///
    /// let (g, sign, o) = (-Float::one_half_prec(100)).ln_abs_gamma_prec_round_ref(5, Ceiling);
    /// assert_eq!(g.to_string(), "1.31");
    /// assert!(!sign);
    /// assert_eq!(o, Greater);
    ///
    /// let (g, sign, o) = (-Float::one_half_prec(100)).ln_abs_gamma_prec_round_ref(5, Nearest);
    /// assert_eq!(g.to_string(), "1.25");
    /// assert!(!sign);
    /// assert_eq!(o, Less);
    ///
    /// let (g, sign, o) = (-Float::one_half_prec(100)).ln_abs_gamma_prec_round_ref(20, Floor);
    /// assert_eq!(g.to_string(), "1.2655106");
    /// assert!(!sign);
    /// assert_eq!(o, Less);
    ///
    /// let (g, sign, o) = (-Float::one_half_prec(100)).ln_abs_gamma_prec_round_ref(20, Ceiling);
    /// assert_eq!(g.to_string(), "1.2655125");
    /// assert!(!sign);
    /// assert_eq!(o, Greater);
    ///
    /// let (g, sign, o) = (-Float::one_half_prec(100)).ln_abs_gamma_prec_round_ref(20, Nearest);
    /// assert_eq!(g.to_string(), "1.2655125");
    /// assert!(!sign);
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn ln_abs_gamma_prec_round_ref(
        &self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, bool, Ordering) {
        ln_gamma_prec_round_helper(true, self, prec, rm)
    }

    /// Computes $\ln |\Gamma(x)|$, the logarithm of the absolute value of the gamma function, of a
    /// [`Float`], rounding the result to the nearest value of the specified precision. The
    /// [`Float`] is taken by value. The sign of $\Gamma(x)$ and an [`Ordering`] are also returned,
    /// the latter indicating whether the rounded value is less than, equal to, or greater than the
    /// exact value. Although `NaN`s are not comparable to any [`Float`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// $\Gamma(x) = \int_0^\infty t^{x-1} e^{-t}\,dt$ for $x > 0$, and the recurrence $\Gamma(x) =
    /// \Gamma(x+1)/x$ extends it to every real $x$ other than $0, -1, -2, \ldots$, where it has
    /// poles and $\ln |\Gamma(x)|$ is taken to be $\infty$.
    ///
    /// The `bool` is the sign of $\Gamma(x)$: `true` if it is positive, which it is for $x > 0$ and
    /// for a negative $x$ that is not an integer and whose integer part is odd. Where $\Gamma(x)$
    /// has no sign (at the negative integers, at $\pm\infty$, and at `NaN`), `true` is returned; at
    /// $\pm0.0$ the sign is that of the zero, as $\Gamma(\pm0.0) = \pm\infty$.
    ///
    /// If the value is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = (\ln |\Gamma(x)|+\varepsilon, \operatorname{sgn} \Gamma(x)).
    /// $$
    /// - If $\ln |\Gamma(x)|$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed
    ///   to be 0.
    /// - If $\ln |\Gamma(x)|$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2 |\ln
    ///   |\Gamma(x)||\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p)=(\text{NaN},\text{true})$
    /// - $f(\pm\infty,p)=(\infty,\text{true})$
    /// - $f(0.0,p)=(\infty,\text{true})$
    /// - $f(-0.0,p)=(\infty,\text{false})$
    /// - $f(x,p)=(\infty,\text{true})$ if $x$ is a negative integer
    /// - $f(1,p)=f(2,p)=(0.0,\text{true})$
    ///
    /// Overflow and underflow, with $y$ the first element of $f(x,p)$:
    /// - If $y\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $y\leq -2^{2^{30}-1}$, $-\infty$ is returned instead.
    /// - If $0<y\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<y<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq y<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<y<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// Overflow happens for arguments above about $2^{2^{30}-30.5}$ and, towards $-\infty$, for
    /// negative arguments of such a magnitude that are not integers. Underflow only happens for an
    /// argument within about $2^{-2^{30}}$ of a zero of $\ln |\Gamma(x)|$; such an argument has
    /// more than $2^{30}$ significant bits.
    ///
    /// The result is correctly rounded. Finding it takes $\ln |\Gamma(x)|$ not to be a rounding
    /// boundary, which is a number with at most `prec + 1` significant bits. This is known when $x$
    /// is a positive integer (then the value is the logarithm of an integer, which is 0 or
    /// irrational); nothing suggests that it ever fails, but it has not been proven for every $x$,
    /// and this function would not terminate if it did fail.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::ln_abs_gamma_prec_round`] instead. If you know that your target precision is the
    /// precision of the input, consider using [`Float::ln_abs_gamma`] instead.
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
    /// let (g, sign, o) = (-Float::one_half_prec(100)).ln_abs_gamma_prec(5);
    /// assert_eq!(g.to_string(), "1.25");
    /// assert!(!sign);
    /// assert_eq!(o, Less);
    ///
    /// let (g, sign, o) = (-Float::one_half_prec(100)).ln_abs_gamma_prec(20);
    /// assert_eq!(g.to_string(), "1.2655125");
    /// assert!(!sign);
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn ln_abs_gamma_prec(self, prec: u64) -> (Self, bool, Ordering) {
        self.ln_abs_gamma_prec_round(prec, Nearest)
    }

    /// Computes $\ln |\Gamma(x)|$, the logarithm of the absolute value of the gamma function, of a
    /// [`Float`], rounding the result to the nearest value of the specified precision. The
    /// [`Float`] is taken by reference. The sign of $\Gamma(x)$ and an [`Ordering`] are also
    /// returned, the latter indicating whether the rounded value is less than, equal to, or greater
    /// than the exact value. Although `NaN`s are not comparable to any [`Float`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// $\Gamma(x) = \int_0^\infty t^{x-1} e^{-t}\,dt$ for $x > 0$, and the recurrence $\Gamma(x) =
    /// \Gamma(x+1)/x$ extends it to every real $x$ other than $0, -1, -2, \ldots$, where it has
    /// poles and $\ln |\Gamma(x)|$ is taken to be $\infty$.
    ///
    /// The `bool` is the sign of $\Gamma(x)$: `true` if it is positive, which it is for $x > 0$ and
    /// for a negative $x$ that is not an integer and whose integer part is odd. Where $\Gamma(x)$
    /// has no sign (at the negative integers, at $\pm\infty$, and at `NaN`), `true` is returned; at
    /// $\pm0.0$ the sign is that of the zero, as $\Gamma(\pm0.0) = \pm\infty$.
    ///
    /// If the value is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = (\ln |\Gamma(x)|+\varepsilon, \operatorname{sgn} \Gamma(x)).
    /// $$
    /// - If $\ln |\Gamma(x)|$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed
    ///   to be 0.
    /// - If $\ln |\Gamma(x)|$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2 |\ln
    ///   |\Gamma(x)||\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p)=(\text{NaN},\text{true})$
    /// - $f(\pm\infty,p)=(\infty,\text{true})$
    /// - $f(0.0,p)=(\infty,\text{true})$
    /// - $f(-0.0,p)=(\infty,\text{false})$
    /// - $f(x,p)=(\infty,\text{true})$ if $x$ is a negative integer
    /// - $f(1,p)=f(2,p)=(0.0,\text{true})$
    ///
    /// Overflow and underflow, with $y$ the first element of $f(x,p)$:
    /// - If $y\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $y\leq -2^{2^{30}-1}$, $-\infty$ is returned instead.
    /// - If $0<y\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<y<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq y<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<y<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// Overflow happens for arguments above about $2^{2^{30}-30.5}$ and, towards $-\infty$, for
    /// negative arguments of such a magnitude that are not integers. Underflow only happens for an
    /// argument within about $2^{-2^{30}}$ of a zero of $\ln |\Gamma(x)|$; such an argument has
    /// more than $2^{30}$ significant bits.
    ///
    /// The result is correctly rounded. Finding it takes $\ln |\Gamma(x)|$ not to be a rounding
    /// boundary, which is a number with at most `prec + 1` significant bits. This is known when $x$
    /// is a positive integer (then the value is the logarithm of an integer, which is 0 or
    /// irrational); nothing suggests that it ever fails, but it has not been proven for every $x$,
    /// and this function would not terminate if it did fail.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::ln_abs_gamma_prec_round_ref`] instead. If you know that your target precision is
    /// the precision of the input, consider using `(&Float).ln_abs_gamma()` instead.
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
    /// let (g, sign, o) = (-Float::one_half_prec(100)).ln_abs_gamma_prec_ref(5);
    /// assert_eq!(g.to_string(), "1.25");
    /// assert!(!sign);
    /// assert_eq!(o, Less);
    ///
    /// let (g, sign, o) = (-Float::one_half_prec(100)).ln_abs_gamma_prec_ref(20);
    /// assert_eq!(g.to_string(), "1.2655125");
    /// assert!(!sign);
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn ln_abs_gamma_prec_ref(&self, prec: u64) -> (Self, bool, Ordering) {
        self.ln_abs_gamma_prec_round_ref(prec, Nearest)
    }

    /// Computes $\ln |\Gamma(x)|$, the logarithm of the absolute value of the gamma function, of a
    /// [`Float`], rounding the result with the specified rounding mode. The [`Float`] is taken by
    /// value. The sign of $\Gamma(x)$ and an [`Ordering`] are also returned, the latter indicating
    /// whether the rounded value is less than, equal to, or greater than the exact value. Although
    /// `NaN`s are not comparable to any [`Float`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// $\Gamma(x) = \int_0^\infty t^{x-1} e^{-t}\,dt$ for $x > 0$, and the recurrence $\Gamma(x) =
    /// \Gamma(x+1)/x$ extends it to every real $x$ other than $0, -1, -2, \ldots$, where it has
    /// poles and $\ln |\Gamma(x)|$ is taken to be $\infty$.
    ///
    /// The `bool` is the sign of $\Gamma(x)$: `true` if it is positive, which it is for $x > 0$ and
    /// for a negative $x$ that is not an integer and whose integer part is odd. Where $\Gamma(x)$
    /// has no sign (at the negative integers, at $\pm\infty$, and at `NaN`), `true` is returned; at
    /// $\pm0.0$ the sign is that of the zero, as $\Gamma(\pm0.0) = \pm\infty$.
    ///
    /// The precision of the output is the precision of the input. See [`RoundingMode`] for a
    /// description of the possible rounding modes.
    ///
    /// $$
    /// f(x,m) = (\ln |\Gamma(x)|+\varepsilon, \operatorname{sgn} \Gamma(x)).
    /// $$
    /// - If $\ln |\Gamma(x)|$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed
    ///   to be 0.
    /// - If $\ln |\Gamma(x)|$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon|
    ///   < 2^{\lfloor\log_2 |\ln |\Gamma(x)||\rfloor-p+1}$, where $p$ is the precision of the
    ///   input.
    /// - If $\ln |\Gamma(x)|$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\ln |\Gamma(x)||\rfloor-p}$, where $p$ is the precision of the input.
    ///
    /// If the output has a precision, it is the precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN},m)=(\text{NaN},\text{true})$
    /// - $f(\pm\infty,m)=(\infty,\text{true})$
    /// - $f(0.0,m)=(\infty,\text{true})$
    /// - $f(-0.0,m)=(\infty,\text{false})$
    /// - $f(x,m)=(\infty,\text{true})$ if $x$ is a negative integer
    /// - $f(1,m)=f(2,m)=(0.0,\text{true})$
    ///
    /// See the [`Float::ln_abs_gamma_prec_round`] documentation for information on overflow and
    /// underflow, and on termination.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::ln_abs_gamma_prec_round`] instead. If you know you'll be using the `Nearest`
    /// rounding mode, consider using [`Float::ln_abs_gamma`] instead.
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
    /// let (g, sign, o) = (-Float::one_half_prec(100)).ln_abs_gamma_round(Floor);
    /// assert_eq!(g.to_string(), "1.2655121234846453964889457971344");
    /// assert!(!sign);
    /// assert_eq!(o, Less);
    ///
    /// let (g, sign, o) = (-Float::one_half_prec(100)).ln_abs_gamma_round(Ceiling);
    /// assert_eq!(g.to_string(), "1.2655121234846453964889457971360");
    /// assert!(!sign);
    /// assert_eq!(o, Greater);
    ///
    /// let (g, sign, o) = (-Float::one_half_prec(100)).ln_abs_gamma_round(Nearest);
    /// assert_eq!(g.to_string(), "1.2655121234846453964889457971344");
    /// assert!(!sign);
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn ln_abs_gamma_round(self, rm: RoundingMode) -> (Self, bool, Ordering) {
        let prec = self.significant_bits();
        self.ln_abs_gamma_prec_round(prec, rm)
    }

    /// Computes $\ln |\Gamma(x)|$, the logarithm of the absolute value of the gamma function, of a
    /// [`Float`], rounding the result with the specified rounding mode. The [`Float`] is taken by
    /// reference. The sign of $\Gamma(x)$ and an [`Ordering`] are also returned, the latter
    /// indicating whether the rounded value is less than, equal to, or greater than the exact
    /// value. Although `NaN`s are not comparable to any [`Float`], whenever this function returns a
    /// `NaN` it also returns `Equal`.
    ///
    /// $\Gamma(x) = \int_0^\infty t^{x-1} e^{-t}\,dt$ for $x > 0$, and the recurrence $\Gamma(x) =
    /// \Gamma(x+1)/x$ extends it to every real $x$ other than $0, -1, -2, \ldots$, where it has
    /// poles and $\ln |\Gamma(x)|$ is taken to be $\infty$.
    ///
    /// The `bool` is the sign of $\Gamma(x)$: `true` if it is positive, which it is for $x > 0$ and
    /// for a negative $x$ that is not an integer and whose integer part is odd. Where $\Gamma(x)$
    /// has no sign (at the negative integers, at $\pm\infty$, and at `NaN`), `true` is returned; at
    /// $\pm0.0$ the sign is that of the zero, as $\Gamma(\pm0.0) = \pm\infty$.
    ///
    /// The precision of the output is the precision of the input. See [`RoundingMode`] for a
    /// description of the possible rounding modes.
    ///
    /// $$
    /// f(x,m) = (\ln |\Gamma(x)|+\varepsilon, \operatorname{sgn} \Gamma(x)).
    /// $$
    /// - If $\ln |\Gamma(x)|$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed
    ///   to be 0.
    /// - If $\ln |\Gamma(x)|$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon|
    ///   < 2^{\lfloor\log_2 |\ln |\Gamma(x)||\rfloor-p+1}$, where $p$ is the precision of the
    ///   input.
    /// - If $\ln |\Gamma(x)|$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\ln |\Gamma(x)||\rfloor-p}$, where $p$ is the precision of the input.
    ///
    /// If the output has a precision, it is the precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN},m)=(\text{NaN},\text{true})$
    /// - $f(\pm\infty,m)=(\infty,\text{true})$
    /// - $f(0.0,m)=(\infty,\text{true})$
    /// - $f(-0.0,m)=(\infty,\text{false})$
    /// - $f(x,m)=(\infty,\text{true})$ if $x$ is a negative integer
    /// - $f(1,m)=f(2,m)=(0.0,\text{true})$
    ///
    /// See the [`Float::ln_abs_gamma_prec_round`] documentation for information on overflow and
    /// underflow, and on termination.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::ln_abs_gamma_prec_round_ref`] instead. If you know you'll be using the `Nearest`
    /// rounding mode, consider using `(&Float).ln_abs_gamma()` instead.
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
    /// let (g, sign, o) = (-Float::one_half_prec(100)).ln_abs_gamma_round_ref(Floor);
    /// assert_eq!(g.to_string(), "1.2655121234846453964889457971344");
    /// assert!(!sign);
    /// assert_eq!(o, Less);
    ///
    /// let (g, sign, o) = (-Float::one_half_prec(100)).ln_abs_gamma_round_ref(Ceiling);
    /// assert_eq!(g.to_string(), "1.2655121234846453964889457971360");
    /// assert!(!sign);
    /// assert_eq!(o, Greater);
    ///
    /// let (g, sign, o) = (-Float::one_half_prec(100)).ln_abs_gamma_round_ref(Nearest);
    /// assert_eq!(g.to_string(), "1.2655121234846453964889457971344");
    /// assert!(!sign);
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn ln_abs_gamma_round_ref(&self, rm: RoundingMode) -> (Self, bool, Ordering) {
        self.ln_abs_gamma_prec_round_ref(self.significant_bits(), rm)
    }

    /// Computes $\ln |\Gamma(x)|$, the logarithm of the absolute value of the gamma function, of a
    /// [`Float`] in place, rounding the result to the specified precision and with the specified
    /// rounding mode. The sign of $\Gamma(x)$ and an [`Ordering`] are returned, the latter
    /// indicating whether the rounded value is less than, equal to, or greater than the exact
    /// value. Although `NaN`s are not comparable to any [`Float`], whenever this function sets the
    /// [`Float`] to `NaN` it also returns `Equal`.
    ///
    /// $\Gamma(x) = \int_0^\infty t^{x-1} e^{-t}\,dt$ for $x > 0$, and the recurrence $\Gamma(x) =
    /// \Gamma(x+1)/x$ extends it to every real $x$ other than $0, -1, -2, \ldots$, where it has
    /// poles and $\ln |\Gamma(x)|$ is taken to be $\infty$.
    ///
    /// The `bool` is the sign of $\Gamma(x)$: `true` if it is positive, which it is for $x > 0$ and
    /// for a negative $x$ that is not an integer and whose integer part is odd. Where $\Gamma(x)$
    /// has no sign (at the negative integers, at $\pm\infty$, and at `NaN`), `true` is returned; at
    /// $\pm0.0$ the sign is that of the zero, as $\Gamma(\pm0.0) = \pm\infty$.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets \ln |\Gamma(x)|+\varepsilon.
    /// $$
    /// - If $\ln |\Gamma(x)|$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed
    ///   to be 0.
    /// - If $\ln |\Gamma(x)|$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon|
    ///   < 2^{\lfloor\log_2 |\ln |\Gamma(x)||\rfloor-p+1}$.
    /// - If $\ln |\Gamma(x)|$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\ln |\Gamma(x)||\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// See the [`Float::ln_abs_gamma_prec_round`] documentation for information on special cases,
    /// overflow and underflow, and termination.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::ln_abs_gamma_prec_assign`]
    /// instead. If you know that your target precision is the precision of the input, consider
    /// using [`Float::ln_abs_gamma_round_assign`] instead. If both of these things are true,
    /// consider using [`Float::ln_abs_gamma_assign`] instead.
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
    /// let mut x = -Float::one_half_prec(100);
    /// assert_eq!(x.ln_abs_gamma_prec_round_assign(5, Floor), (false, Less));
    /// assert_eq!(x.to_string(), "1.25");
    ///
    /// let mut x = -Float::one_half_prec(100);
    /// assert_eq!(
    ///     x.ln_abs_gamma_prec_round_assign(5, Ceiling),
    ///     (false, Greater)
    /// );
    /// assert_eq!(x.to_string(), "1.31");
    ///
    /// let mut x = -Float::one_half_prec(100);
    /// assert_eq!(x.ln_abs_gamma_prec_round_assign(5, Nearest), (false, Less));
    /// assert_eq!(x.to_string(), "1.25");
    ///
    /// let mut x = -Float::one_half_prec(100);
    /// assert_eq!(x.ln_abs_gamma_prec_round_assign(20, Floor), (false, Less));
    /// assert_eq!(x.to_string(), "1.2655106");
    ///
    /// let mut x = -Float::one_half_prec(100);
    /// assert_eq!(
    ///     x.ln_abs_gamma_prec_round_assign(20, Ceiling),
    ///     (false, Greater)
    /// );
    /// assert_eq!(x.to_string(), "1.2655125");
    ///
    /// let mut x = -Float::one_half_prec(100);
    /// assert_eq!(
    ///     x.ln_abs_gamma_prec_round_assign(20, Nearest),
    ///     (false, Greater)
    /// );
    /// assert_eq!(x.to_string(), "1.2655125");
    /// ```
    #[inline]
    pub fn ln_abs_gamma_prec_round_assign(
        &mut self,
        prec: u64,
        rm: RoundingMode,
    ) -> (bool, Ordering) {
        let (result, sign, o) = core::mem::take(self).ln_abs_gamma_prec_round(prec, rm);
        *self = result;
        (sign, o)
    }

    /// Computes $\ln |\Gamma(x)|$, the logarithm of the absolute value of the gamma function, of a
    /// [`Float`] in place, rounding the result to the nearest value of the specified precision. The
    /// sign of $\Gamma(x)$ and an [`Ordering`] are returned, the latter indicating whether the
    /// rounded value is less than, equal to, or greater than the exact value. Although `NaN`s are
    /// not comparable to any [`Float`], whenever this function sets the [`Float`] to `NaN` it also
    /// returns `Equal`.
    ///
    /// $\Gamma(x) = \int_0^\infty t^{x-1} e^{-t}\,dt$ for $x > 0$, and the recurrence $\Gamma(x) =
    /// \Gamma(x+1)/x$ extends it to every real $x$ other than $0, -1, -2, \ldots$, where it has
    /// poles and $\ln |\Gamma(x)|$ is taken to be $\infty$.
    ///
    /// The `bool` is the sign of $\Gamma(x)$: `true` if it is positive, which it is for $x > 0$ and
    /// for a negative $x$ that is not an integer and whose integer part is odd. Where $\Gamma(x)$
    /// has no sign (at the negative integers, at $\pm\infty$, and at `NaN`), `true` is returned; at
    /// $\pm0.0$ the sign is that of the zero, as $\Gamma(\pm0.0) = \pm\infty$.
    ///
    /// If the value is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// x \gets \ln |\Gamma(x)|+\varepsilon.
    /// $$
    /// - If $\ln |\Gamma(x)|$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed
    ///   to be 0.
    /// - If $\ln |\Gamma(x)|$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2 |\ln
    ///   |\Gamma(x)||\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// See the [`Float::ln_abs_gamma_prec_round`] documentation for information on special cases,
    /// overflow and underflow, and termination.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::ln_abs_gamma_prec_round_assign`] instead. If you know that your target precision is
    /// the precision of the input, consider using [`Float::ln_abs_gamma_assign`] instead.
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
    /// let mut x = -Float::one_half_prec(100);
    /// assert_eq!(x.ln_abs_gamma_prec_assign(5), (false, Less));
    /// assert_eq!(x.to_string(), "1.25");
    ///
    /// let mut x = -Float::one_half_prec(100);
    /// assert_eq!(x.ln_abs_gamma_prec_assign(20), (false, Greater));
    /// assert_eq!(x.to_string(), "1.2655125");
    /// ```
    #[inline]
    pub fn ln_abs_gamma_prec_assign(&mut self, prec: u64) -> (bool, Ordering) {
        self.ln_abs_gamma_prec_round_assign(prec, Nearest)
    }

    /// Computes $\ln |\Gamma(x)|$, the logarithm of the absolute value of the gamma function, of a
    /// [`Float`] in place, rounding the result with the specified rounding mode. The sign of
    /// $\Gamma(x)$ and an [`Ordering`] are returned, the latter indicating whether the rounded
    /// value is less than, equal to, or greater than the exact value. Although `NaN`s are not
    /// comparable to any [`Float`], whenever this function sets the [`Float`] to `NaN` it also
    /// returns `Equal`.
    ///
    /// $\Gamma(x) = \int_0^\infty t^{x-1} e^{-t}\,dt$ for $x > 0$, and the recurrence $\Gamma(x) =
    /// \Gamma(x+1)/x$ extends it to every real $x$ other than $0, -1, -2, \ldots$, where it has
    /// poles and $\ln |\Gamma(x)|$ is taken to be $\infty$.
    ///
    /// The `bool` is the sign of $\Gamma(x)$: `true` if it is positive, which it is for $x > 0$ and
    /// for a negative $x$ that is not an integer and whose integer part is odd. Where $\Gamma(x)$
    /// has no sign (at the negative integers, at $\pm\infty$, and at `NaN`), `true` is returned; at
    /// $\pm0.0$ the sign is that of the zero, as $\Gamma(\pm0.0) = \pm\infty$.
    ///
    /// The precision of the output is the precision of the input. See [`RoundingMode`] for a
    /// description of the possible rounding modes.
    ///
    /// $$
    /// x \gets \ln |\Gamma(x)|+\varepsilon.
    /// $$
    /// - If $\ln |\Gamma(x)|$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed
    ///   to be 0.
    /// - If $\ln |\Gamma(x)|$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon|
    ///   < 2^{\lfloor\log_2 |\ln |\Gamma(x)||\rfloor-p+1}$, where $p$ is the precision of the
    ///   input.
    /// - If $\ln |\Gamma(x)|$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\ln |\Gamma(x)||\rfloor-p}$, where $p$ is the precision of the input.
    ///
    /// If the output has a precision, it is the precision of the input.
    ///
    /// See the [`Float::ln_abs_gamma_prec_round`] documentation for information on special cases,
    /// overflow and underflow, and termination.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::ln_abs_gamma_prec_round_assign`] instead. If you know you'll be using the `Nearest`
    /// rounding mode, consider using [`Float::ln_abs_gamma_assign`] instead.
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
    /// let mut x = -Float::one_half_prec(100);
    /// assert_eq!(x.ln_abs_gamma_round_assign(Floor), (false, Less));
    /// assert_eq!(x.to_string(), "1.2655121234846453964889457971344");
    ///
    /// let mut x = -Float::one_half_prec(100);
    /// assert_eq!(x.ln_abs_gamma_round_assign(Ceiling), (false, Greater));
    /// assert_eq!(x.to_string(), "1.2655121234846453964889457971360");
    ///
    /// let mut x = -Float::one_half_prec(100);
    /// assert_eq!(x.ln_abs_gamma_round_assign(Nearest), (false, Less));
    /// assert_eq!(x.to_string(), "1.2655121234846453964889457971344");
    /// ```
    #[inline]
    pub fn ln_abs_gamma_round_assign(&mut self, rm: RoundingMode) -> (bool, Ordering) {
        let prec = self.significant_bits();
        self.ln_abs_gamma_prec_round_assign(prec, rm)
    }

    /// Computes $\ln |\Gamma(x)|$, the logarithm of the absolute value of the gamma function, of a
    /// [`Rational`], rounding the result to the specified precision and with the specified rounding
    /// mode and returning the result as a [`Float`]. The [`Rational`] is taken by value. The sign
    /// of $\Gamma(x)$ and an [`Ordering`] are also returned, the latter indicating whether the
    /// rounded value is less than, equal to, or greater than the exact value. Although `NaN`s are
    /// not comparable to any [`Float`], whenever this function returns a `NaN` it also returns
    /// `Equal`.
    ///
    /// $\Gamma(x) = \int_0^\infty t^{x-1} e^{-t}\,dt$ for $x > 0$, and the recurrence $\Gamma(x) =
    /// \Gamma(x+1)/x$ extends it to every real $x$ other than $0, -1, -2, \ldots$, where it has
    /// poles and $\ln |\Gamma(x)|$ is taken to be $\infty$.
    ///
    /// The `bool` is the sign of $\Gamma(x)$: `true` if it is positive, which it is for $x > 0$ and
    /// for a negative $x$ that is not an integer and whose integer part is odd. Where $\Gamma(x)$
    /// has no sign (at the negative integers and at 0), `true` is returned.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = (\ln |\Gamma(x)|+\varepsilon, \operatorname{sgn} \Gamma(x)).
    /// $$
    /// - If $\ln |\Gamma(x)|$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed
    ///   to be 0.
    /// - If $\ln |\Gamma(x)|$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon|
    ///   < 2^{\lfloor\log_2 |\ln |\Gamma(x)||\rfloor-p+1}$.
    /// - If $\ln |\Gamma(x)|$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\ln |\Gamma(x)||\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(x,p,m)=(\infty,\text{true})$ if $x$ is 0 or a negative integer
    /// - $f(1,p,m)=f(2,p,m)=(0.0,\text{true})$
    ///
    /// Overflow and underflow, with $y$ the first element of $f(x,p,m)$:
    /// - If $y\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is returned
    ///   instead.
    /// - If $y\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$ is
    ///   returned instead.
    /// - If $y\leq -2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is returned
    ///   instead.
    /// - If $y\leq -2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`, $-(1-(1/2)^p)2^{2^{30}-1}$ is
    ///   returned instead.
    /// - If $0<y<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<y<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned instead.
    /// - If $0<y\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<y<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}}<y<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<y<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq y<0$, and $m$ is `Nearest`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<y<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is returned
    ///   instead.
    ///
    /// Overflow happens for arguments above about $2^{2^{30}-30.5}$ and, towards $-\infty$, for
    /// negative arguments of such a magnitude that are not integers. Underflow only happens for an
    /// argument within about $2^{-2^{30}}$ of a zero of $\ln |\Gamma(x)|$.
    ///
    /// The result is correctly rounded. Finding it takes $\ln |\Gamma(x)|$ not to be a rounding
    /// boundary, which is a number with at most `prec + 1` significant bits. This is known when $x$
    /// is a positive integer (then the value is the logarithm of an integer, which is 0 or
    /// irrational); nothing suggests that it ever fails, but it has not been proven for every
    /// rational $x$, and this function would not terminate if it did fail.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::ln_abs_gamma_rational_prec`]
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
    /// let (g, sign, o) =
    ///     Float::ln_abs_gamma_rational_prec_round(Rational::from_signeds(-1i8, 3), 5, Floor);
    /// assert_eq!(g.to_string(), "1.38");
    /// assert!(!sign);
    /// assert_eq!(o, Less);
    ///
    /// let (g, sign, o) =
    ///     Float::ln_abs_gamma_rational_prec_round(Rational::from_signeds(-1i8, 3), 5, Ceiling);
    /// assert_eq!(g.to_string(), "1.44");
    /// assert!(!sign);
    /// assert_eq!(o, Greater);
    ///
    /// let (g, sign, o) =
    ///     Float::ln_abs_gamma_rational_prec_round(Rational::from_signeds(-1i8, 3), 5, Nearest);
    /// assert_eq!(g.to_string(), "1.38");
    /// assert!(!sign);
    /// assert_eq!(o, Less);
    ///
    /// let (g, sign, o) =
    ///     Float::ln_abs_gamma_rational_prec_round(Rational::from_signeds(-1i8, 3), 20, Floor);
    /// assert_eq!(g.to_string(), "1.4017620");
    /// assert!(!sign);
    /// assert_eq!(o, Less);
    ///
    /// let (g, sign, o) =
    ///     Float::ln_abs_gamma_rational_prec_round(Rational::from_signeds(-1i8, 3), 20, Ceiling);
    /// assert_eq!(g.to_string(), "1.4017639");
    /// assert!(!sign);
    /// assert_eq!(o, Greater);
    ///
    /// let (g, sign, o) =
    ///     Float::ln_abs_gamma_rational_prec_round(Rational::from_signeds(-1i8, 3), 20, Nearest);
    /// assert_eq!(g.to_string(), "1.4017620");
    /// assert!(!sign);
    /// assert_eq!(o, Less);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn ln_abs_gamma_rational_prec_round(
        x: Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, bool, Ordering) {
        ln_gamma_rational_prec_round_helper(true, &x, prec, rm)
    }

    /// Computes $\ln |\Gamma(x)|$, the logarithm of the absolute value of the gamma function, of a
    /// [`Rational`], rounding the result to the specified precision and with the specified rounding
    /// mode and returning the result as a [`Float`]. The [`Rational`] is taken by reference. The
    /// sign of $\Gamma(x)$ and an [`Ordering`] are also returned, the latter indicating whether the
    /// rounded value is less than, equal to, or greater than the exact value. Although `NaN`s are
    /// not comparable to any [`Float`], whenever this function returns a `NaN` it also returns
    /// `Equal`.
    ///
    /// $\Gamma(x) = \int_0^\infty t^{x-1} e^{-t}\,dt$ for $x > 0$, and the recurrence $\Gamma(x) =
    /// \Gamma(x+1)/x$ extends it to every real $x$ other than $0, -1, -2, \ldots$, where it has
    /// poles and $\ln |\Gamma(x)|$ is taken to be $\infty$.
    ///
    /// The `bool` is the sign of $\Gamma(x)$: `true` if it is positive, which it is for $x > 0$ and
    /// for a negative $x$ that is not an integer and whose integer part is odd. Where $\Gamma(x)$
    /// has no sign (at the negative integers and at 0), `true` is returned.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = (\ln |\Gamma(x)|+\varepsilon, \operatorname{sgn} \Gamma(x)).
    /// $$
    /// - If $\ln |\Gamma(x)|$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed
    ///   to be 0.
    /// - If $\ln |\Gamma(x)|$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon|
    ///   < 2^{\lfloor\log_2 |\ln |\Gamma(x)||\rfloor-p+1}$.
    /// - If $\ln |\Gamma(x)|$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\ln |\Gamma(x)||\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(x,p,m)=(\infty,\text{true})$ if $x$ is 0 or a negative integer
    /// - $f(1,p,m)=f(2,p,m)=(0.0,\text{true})$
    ///
    /// Overflow and underflow, with $y$ the first element of $f(x,p,m)$:
    /// - If $y\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is returned
    ///   instead.
    /// - If $y\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$ is
    ///   returned instead.
    /// - If $y\leq -2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is returned
    ///   instead.
    /// - If $y\leq -2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`, $-(1-(1/2)^p)2^{2^{30}-1}$ is
    ///   returned instead.
    /// - If $0<y<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<y<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned instead.
    /// - If $0<y\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<y<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}}<y<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<y<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq y<0$, and $m$ is `Nearest`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<y<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is returned
    ///   instead.
    ///
    /// Overflow happens for arguments above about $2^{2^{30}-30.5}$ and, towards $-\infty$, for
    /// negative arguments of such a magnitude that are not integers. Underflow only happens for an
    /// argument within about $2^{-2^{30}}$ of a zero of $\ln |\Gamma(x)|$.
    ///
    /// The result is correctly rounded. Finding it takes $\ln |\Gamma(x)|$ not to be a rounding
    /// boundary, which is a number with at most `prec + 1` significant bits. This is known when $x$
    /// is a positive integer (then the value is the logarithm of an integer, which is 0 or
    /// irrational); nothing suggests that it ever fails, but it has not been proven for every
    /// rational $x$, and this function would not terminate if it did fail.
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`Float::ln_abs_gamma_rational_prec_ref`] instead.
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
    /// let (g, sign, o) =
    ///     Float::ln_abs_gamma_rational_prec_round_ref(&Rational::from_signeds(-1i8, 3), 5, Floor);
    /// assert_eq!(g.to_string(), "1.38");
    /// assert!(!sign);
    /// assert_eq!(o, Less);
    ///
    /// let (g, sign, o) = Float::ln_abs_gamma_rational_prec_round_ref(
    ///     &Rational::from_signeds(-1i8, 3),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(g.to_string(), "1.44");
    /// assert!(!sign);
    /// assert_eq!(o, Greater);
    ///
    /// let (g, sign, o) = Float::ln_abs_gamma_rational_prec_round_ref(
    ///     &Rational::from_signeds(-1i8, 3),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(g.to_string(), "1.38");
    /// assert!(!sign);
    /// assert_eq!(o, Less);
    ///
    /// let (g, sign, o) = Float::ln_abs_gamma_rational_prec_round_ref(
    ///     &Rational::from_signeds(-1i8, 3),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(g.to_string(), "1.4017620");
    /// assert!(!sign);
    /// assert_eq!(o, Less);
    ///
    /// let (g, sign, o) = Float::ln_abs_gamma_rational_prec_round_ref(
    ///     &Rational::from_signeds(-1i8, 3),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(g.to_string(), "1.4017639");
    /// assert!(!sign);
    /// assert_eq!(o, Greater);
    ///
    /// let (g, sign, o) = Float::ln_abs_gamma_rational_prec_round_ref(
    ///     &Rational::from_signeds(-1i8, 3),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(g.to_string(), "1.4017620");
    /// assert!(!sign);
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn ln_abs_gamma_rational_prec_round_ref(
        x: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, bool, Ordering) {
        ln_gamma_rational_prec_round_helper(true, x, prec, rm)
    }

    /// Computes $\ln |\Gamma(x)|$, the logarithm of the absolute value of the gamma function, of a
    /// [`Rational`], rounding the result to the nearest value of the specified precision and
    /// returning the result as a [`Float`]. The [`Rational`] is taken by value. The sign of
    /// $\Gamma(x)$ and an [`Ordering`] are also returned, the latter indicating whether the rounded
    /// value is less than, equal to, or greater than the exact value. Although `NaN`s are not
    /// comparable to any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $\Gamma(x) = \int_0^\infty t^{x-1} e^{-t}\,dt$ for $x > 0$, and the recurrence $\Gamma(x) =
    /// \Gamma(x+1)/x$ extends it to every real $x$ other than $0, -1, -2, \ldots$, where it has
    /// poles and $\ln |\Gamma(x)|$ is taken to be $\infty$.
    ///
    /// The `bool` is the sign of $\Gamma(x)$: `true` if it is positive, which it is for $x > 0$ and
    /// for a negative $x$ that is not an integer and whose integer part is odd. Where $\Gamma(x)$
    /// has no sign (at the negative integers and at 0), `true` is returned.
    ///
    /// If the value is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = (\ln |\Gamma(x)|+\varepsilon, \operatorname{sgn} \Gamma(x)).
    /// $$
    /// - If $\ln |\Gamma(x)|$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed
    ///   to be 0.
    /// - If $\ln |\Gamma(x)|$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2 |\ln
    ///   |\Gamma(x)||\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(x,p)=(\infty,\text{true})$ if $x$ is 0 or a negative integer
    /// - $f(1,p)=f(2,p)=(0.0,\text{true})$
    ///
    /// Overflow and underflow, with $y$ the first element of $f(x,p)$:
    /// - If $y\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $y\leq -2^{2^{30}-1}$, $-\infty$ is returned instead.
    /// - If $0<y\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<y<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq y<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<y<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// Overflow happens for arguments above about $2^{2^{30}-30.5}$ and, towards $-\infty$, for
    /// negative arguments of such a magnitude that are not integers. Underflow only happens for an
    /// argument within about $2^{-2^{30}}$ of a zero of $\ln |\Gamma(x)|$.
    ///
    /// The result is correctly rounded. Finding it takes $\ln |\Gamma(x)|$ not to be a rounding
    /// boundary, which is a number with at most `prec + 1` significant bits. This is known when $x$
    /// is a positive integer (then the value is the logarithm of an integer, which is 0 or
    /// irrational); nothing suggests that it ever fails, but it has not been proven for every
    /// rational $x$, and this function would not terminate if it did fail.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::ln_abs_gamma_rational_prec_round`] instead.
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
    /// let (g, sign, o) = Float::ln_abs_gamma_rational_prec(Rational::from_signeds(-1i8, 3), 5);
    /// assert_eq!(g.to_string(), "1.38");
    /// assert!(!sign);
    /// assert_eq!(o, Less);
    ///
    /// let (g, sign, o) = Float::ln_abs_gamma_rational_prec(Rational::from_signeds(-1i8, 3), 20);
    /// assert_eq!(g.to_string(), "1.4017620");
    /// assert!(!sign);
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn ln_abs_gamma_rational_prec(x: Rational, prec: u64) -> (Self, bool, Ordering) {
        Self::ln_abs_gamma_rational_prec_round(x, prec, Nearest)
    }

    /// Computes $\ln |\Gamma(x)|$, the logarithm of the absolute value of the gamma function, of a
    /// [`Rational`], rounding the result to the nearest value of the specified precision and
    /// returning the result as a [`Float`]. The [`Rational`] is taken by reference. The sign of
    /// $\Gamma(x)$ and an [`Ordering`] are also returned, the latter indicating whether the rounded
    /// value is less than, equal to, or greater than the exact value. Although `NaN`s are not
    /// comparable to any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $\Gamma(x) = \int_0^\infty t^{x-1} e^{-t}\,dt$ for $x > 0$, and the recurrence $\Gamma(x) =
    /// \Gamma(x+1)/x$ extends it to every real $x$ other than $0, -1, -2, \ldots$, where it has
    /// poles and $\ln |\Gamma(x)|$ is taken to be $\infty$.
    ///
    /// The `bool` is the sign of $\Gamma(x)$: `true` if it is positive, which it is for $x > 0$ and
    /// for a negative $x$ that is not an integer and whose integer part is odd. Where $\Gamma(x)$
    /// has no sign (at the negative integers and at 0), `true` is returned.
    ///
    /// If the value is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = (\ln |\Gamma(x)|+\varepsilon, \operatorname{sgn} \Gamma(x)).
    /// $$
    /// - If $\ln |\Gamma(x)|$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed
    ///   to be 0.
    /// - If $\ln |\Gamma(x)|$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2 |\ln
    ///   |\Gamma(x)||\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(x,p)=(\infty,\text{true})$ if $x$ is 0 or a negative integer
    /// - $f(1,p)=f(2,p)=(0.0,\text{true})$
    ///
    /// Overflow and underflow, with $y$ the first element of $f(x,p)$:
    /// - If $y\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $y\leq -2^{2^{30}-1}$, $-\infty$ is returned instead.
    /// - If $0<y\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<y<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq y<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<y<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// Overflow happens for arguments above about $2^{2^{30}-30.5}$ and, towards $-\infty$, for
    /// negative arguments of such a magnitude that are not integers. Underflow only happens for an
    /// argument within about $2^{-2^{30}}$ of a zero of $\ln |\Gamma(x)|$.
    ///
    /// The result is correctly rounded. Finding it takes $\ln |\Gamma(x)|$ not to be a rounding
    /// boundary, which is a number with at most `prec + 1` significant bits. This is known when $x$
    /// is a positive integer (then the value is the logarithm of an integer, which is 0 or
    /// irrational); nothing suggests that it ever fails, but it has not been proven for every
    /// rational $x$, and this function would not terminate if it did fail.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::ln_abs_gamma_rational_prec_round_ref`] instead.
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
    /// let (g, sign, o) =
    ///     Float::ln_abs_gamma_rational_prec_ref(&Rational::from_signeds(-1i8, 3), 5);
    /// assert_eq!(g.to_string(), "1.38");
    /// assert!(!sign);
    /// assert_eq!(o, Less);
    ///
    /// let (g, sign, o) =
    ///     Float::ln_abs_gamma_rational_prec_ref(&Rational::from_signeds(-1i8, 3), 20);
    /// assert_eq!(g.to_string(), "1.4017620");
    /// assert!(!sign);
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn ln_abs_gamma_rational_prec_ref(x: &Rational, prec: u64) -> (Self, bool, Ordering) {
        Self::ln_abs_gamma_rational_prec_round_ref(x, prec, Nearest)
    }
}

impl LnAbsGamma for Float {
    type Output = Self;

    /// Computes $\ln |\Gamma(x)|$, the logarithm of the absolute value of the gamma function, of a
    /// [`Float`], taking it by value. The sign of $\Gamma(x)$ is also returned.
    ///
    /// $\Gamma(x) = \int_0^\infty t^{x-1} e^{-t}\,dt$ for $x > 0$, and the recurrence $\Gamma(x) =
    /// \Gamma(x+1)/x$ extends it to every real $x$ other than $0, -1, -2, \ldots$, where it has
    /// poles and $\ln |\Gamma(x)|$ is taken to be $\infty$.
    ///
    /// The `bool` is the sign of $\Gamma(x)$: `true` if it is positive, which it is for $x > 0$ and
    /// for a negative $x$ that is not an integer and whose integer part is odd. Where $\Gamma(x)$
    /// has no sign (at the negative integers, at $\pm\infty$, and at `NaN`), `true` is returned; at
    /// $\pm0.0$ the sign is that of the zero, as $\Gamma(\pm0.0) = \pm\infty$.
    ///
    /// If the output has a precision, it is the precision of the input. If the value is equidistant
    /// from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s in its binary
    /// expansion is chosen. See [`RoundingMode`] for a description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x) = (\ln |\Gamma(x)|+\varepsilon, \operatorname{sgn} \Gamma(x)).
    /// $$
    /// - If $\ln |\Gamma(x)|$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed
    ///   to be 0.
    /// - If $\ln |\Gamma(x)|$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2 |\ln
    ///   |\Gamma(x)||\rfloor-p}$, where $p$ is the precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN})=(\text{NaN},\text{true})$
    /// - $f(\pm\infty)=(\infty,\text{true})$
    /// - $f(0.0)=(\infty,\text{true})$
    /// - $f(-0.0)=(\infty,\text{false})$
    /// - $f(x)=(\infty,\text{true})$ if $x$ is a negative integer
    /// - $f(1)=f(2)=(0.0,\text{true})$
    ///
    /// See the [`Float::ln_abs_gamma_prec_round`] documentation for information on overflow and
    /// underflow, and on termination.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::ln_abs_gamma_round`] instead. If you want to specify the output precision, consider
    /// using [`Float::ln_abs_gamma_prec`]. If you want both of these things, consider using
    /// [`Float::ln_abs_gamma_prec_round`].
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
    /// use malachite_base::num::gamma::traits::LnAbsGamma;
    /// use malachite_float::Float;
    ///
    /// let (y, sign) = Float::NAN.ln_abs_gamma();
    /// assert!(y.is_nan());
    /// assert!(sign);
    ///
    /// let (y, sign) = Float::INFINITY.ln_abs_gamma();
    /// assert_eq!(y, Float::INFINITY);
    /// assert!(sign);
    ///
    /// let (y, sign) = Float::ZERO.ln_abs_gamma();
    /// assert_eq!(y, Float::INFINITY);
    /// assert!(sign);
    ///
    /// let (y, sign) = Float::NEGATIVE_ONE.ln_abs_gamma();
    /// assert_eq!(y, Float::INFINITY);
    /// assert!(sign);
    ///
    /// let (y, sign) = Float::ONE.ln_abs_gamma();
    /// assert_eq!(y, Float::ZERO);
    /// assert!(sign);
    ///
    /// let (y, sign) = (-Float::one_half_prec(100)).ln_abs_gamma();
    /// assert_eq!(y.to_string(), "1.2655121234846453964889457971344");
    /// assert!(!sign);
    /// ```
    #[inline]
    fn ln_abs_gamma(self) -> (Self, bool) {
        let prec = self.significant_bits();
        let (y, sign, _) = self.ln_abs_gamma_prec(prec);
        (y, sign)
    }
}

impl LnAbsGamma for &Float {
    type Output = Float;

    /// Computes $\ln |\Gamma(x)|$, the logarithm of the absolute value of the gamma function, of a
    /// [`Float`], taking it by reference. The sign of $\Gamma(x)$ is also returned.
    ///
    /// $\Gamma(x) = \int_0^\infty t^{x-1} e^{-t}\,dt$ for $x > 0$, and the recurrence $\Gamma(x) =
    /// \Gamma(x+1)/x$ extends it to every real $x$ other than $0, -1, -2, \ldots$, where it has
    /// poles and $\ln |\Gamma(x)|$ is taken to be $\infty$.
    ///
    /// The `bool` is the sign of $\Gamma(x)$: `true` if it is positive, which it is for $x > 0$ and
    /// for a negative $x$ that is not an integer and whose integer part is odd. Where $\Gamma(x)$
    /// has no sign (at the negative integers, at $\pm\infty$, and at `NaN`), `true` is returned; at
    /// $\pm0.0$ the sign is that of the zero, as $\Gamma(\pm0.0) = \pm\infty$.
    ///
    /// If the output has a precision, it is the precision of the input. If the value is equidistant
    /// from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s in its binary
    /// expansion is chosen. See [`RoundingMode`] for a description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x) = (\ln |\Gamma(x)|+\varepsilon, \operatorname{sgn} \Gamma(x)).
    /// $$
    /// - If $\ln |\Gamma(x)|$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed
    ///   to be 0.
    /// - If $\ln |\Gamma(x)|$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2 |\ln
    ///   |\Gamma(x)||\rfloor-p}$, where $p$ is the precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN})=(\text{NaN},\text{true})$
    /// - $f(\pm\infty)=(\infty,\text{true})$
    /// - $f(0.0)=(\infty,\text{true})$
    /// - $f(-0.0)=(\infty,\text{false})$
    /// - $f(x)=(\infty,\text{true})$ if $x$ is a negative integer
    /// - $f(1)=f(2)=(0.0,\text{true})$
    ///
    /// See the [`Float::ln_abs_gamma_prec_round`] documentation for information on overflow and
    /// underflow, and on termination.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::ln_abs_gamma_round_ref`] instead. If you want to specify the output precision,
    /// consider using [`Float::ln_abs_gamma_prec_ref`]. If you want both of these things, consider
    /// using [`Float::ln_abs_gamma_prec_round_ref`].
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
    /// use malachite_base::num::gamma::traits::LnAbsGamma;
    /// use malachite_float::Float;
    ///
    /// let (y, sign) = (&Float::NAN).ln_abs_gamma();
    /// assert!(y.is_nan());
    /// assert!(sign);
    ///
    /// let (y, sign) = (&Float::INFINITY).ln_abs_gamma();
    /// assert_eq!(y, Float::INFINITY);
    /// assert!(sign);
    ///
    /// let (y, sign) = (&Float::ZERO).ln_abs_gamma();
    /// assert_eq!(y, Float::INFINITY);
    /// assert!(sign);
    ///
    /// let (y, sign) = (&Float::NEGATIVE_ONE).ln_abs_gamma();
    /// assert_eq!(y, Float::INFINITY);
    /// assert!(sign);
    ///
    /// let (y, sign) = (&Float::ONE).ln_abs_gamma();
    /// assert_eq!(y, Float::ZERO);
    /// assert!(sign);
    ///
    /// let (y, sign) = (&(-Float::one_half_prec(100))).ln_abs_gamma();
    /// assert_eq!(y.to_string(), "1.2655121234846453964889457971344");
    /// assert!(!sign);
    /// ```
    #[inline]
    fn ln_abs_gamma(self) -> (Float, bool) {
        let (y, sign, _) = self.ln_abs_gamma_prec_ref(self.significant_bits());
        (y, sign)
    }
}

impl LnAbsGammaAssign for Float {
    /// Computes $\ln |\Gamma(x)|$, the logarithm of the absolute value of the gamma function, of a
    /// [`Float`] in place. The sign of $\Gamma(x)$ is returned.
    ///
    /// $\Gamma(x) = \int_0^\infty t^{x-1} e^{-t}\,dt$ for $x > 0$, and the recurrence $\Gamma(x) =
    /// \Gamma(x+1)/x$ extends it to every real $x$ other than $0, -1, -2, \ldots$, where it has
    /// poles and $\ln |\Gamma(x)|$ is taken to be $\infty$.
    ///
    /// The `bool` is the sign of $\Gamma(x)$: `true` if it is positive, which it is for $x > 0$ and
    /// for a negative $x$ that is not an integer and whose integer part is odd. Where $\Gamma(x)$
    /// has no sign (at the negative integers, at $\pm\infty$, and at `NaN`), `true` is returned; at
    /// $\pm0.0$ the sign is that of the zero, as $\Gamma(\pm0.0) = \pm\infty$.
    ///
    /// If the output has a precision, it is the precision of the input. If the value is equidistant
    /// from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s in its binary
    /// expansion is chosen. See [`RoundingMode`] for a description of the `Nearest` rounding mode.
    ///
    /// $$
    /// x \gets \ln |\Gamma(x)|+\varepsilon.
    /// $$
    /// - If $\ln |\Gamma(x)|$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed
    ///   to be 0.
    /// - If $\ln |\Gamma(x)|$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2 |\ln
    ///   |\Gamma(x)||\rfloor-p}$, where $p$ is the precision of the input.
    ///
    /// See the [`Float::ln_abs_gamma_prec_round`] documentation for information on special cases,
    /// overflow and underflow, and termination.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::ln_abs_gamma_round_assign`] instead. If you want to specify the output precision,
    /// consider using [`Float::ln_abs_gamma_prec_assign`]. If you want both of these things,
    /// consider using [`Float::ln_abs_gamma_prec_round_assign`].
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
    /// use malachite_base::num::gamma::traits::LnAbsGammaAssign;
    /// use malachite_float::Float;
    ///
    /// let mut x = Float::NAN;
    /// assert!(x.ln_abs_gamma_assign());
    /// assert!(x.is_nan());
    ///
    /// let mut x = Float::INFINITY;
    /// assert!(x.ln_abs_gamma_assign());
    /// assert_eq!(x, Float::INFINITY);
    ///
    /// let mut x = Float::ZERO;
    /// assert!(x.ln_abs_gamma_assign());
    /// assert_eq!(x, Float::INFINITY);
    ///
    /// let mut x = Float::NEGATIVE_ONE;
    /// assert!(x.ln_abs_gamma_assign());
    /// assert_eq!(x, Float::INFINITY);
    ///
    /// let mut x = Float::ONE;
    /// assert!(x.ln_abs_gamma_assign());
    /// assert_eq!(x, Float::ZERO);
    ///
    /// let mut x = -Float::one_half_prec(100);
    /// assert!(!x.ln_abs_gamma_assign());
    /// assert_eq!(x.to_string(), "1.2655121234846453964889457971344");
    /// ```
    #[inline]
    fn ln_abs_gamma_assign(&mut self) -> bool {
        let prec = self.significant_bits();
        self.ln_abs_gamma_prec_assign(prec).0
    }
}

/// Computes $\ln |\Gamma(x)|$, the logarithm of the absolute value of the gamma function, of a
/// primitive float. The sign of $\Gamma(x)$ is also returned.
///
/// $\Gamma(x) = \int_0^\infty t^{x-1} e^{-t}\,dt$ for $x > 0$, and the recurrence $\Gamma(x) =
/// \Gamma(x+1)/x$ extends it to every real $x$ other than $0, -1, -2, \ldots$, where it has poles
/// and $\ln |\Gamma(x)|$ is taken to be $\infty$.
///
/// The `bool` is the sign of $\Gamma(x)$: `true` if it is positive, which it is for $x > 0$ and for
/// a negative $x$ that is not an integer and whose integer part is odd. Where $\Gamma(x)$ has no
/// sign (at the negative integers, at $\pm\infty$, and at `NaN`), `true` is returned; at $\pm0.0$
/// the sign is that of the zero, as $\Gamma(\pm0.0) = \pm\infty$.
///
/// $$
/// f(x) = (\ln |\Gamma(x)|+\varepsilon, \operatorname{sgn} \Gamma(x)).
/// $$
/// - If $\ln |\Gamma(x)|$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to
///   be 0.
/// - If $\ln |\Gamma(x)|$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2 |\ln
///   |\Gamma(x)||\rfloor-p}$, where $p$ is the precision of the output (typically 24 if `T` is a
///   [`f32`] and 53 if `T` is a [`f64`], but less if the output is subnormal).
///
/// Special cases:
/// - $f(\text{NaN})=(\text{NaN},\text{true})$
/// - $f(\pm\infty)=(\infty,\text{true})$
/// - $f(0.0)=(\infty,\text{true})$
/// - $f(-0.0)=(\infty,\text{false})$
/// - $f(x)=(\infty,\text{true})$ if $x$ is a negative integer
/// - $f(1)=f(2)=(0.0,\text{true})$
///
/// The result is $\infty$ when $\ln |\Gamma(x)|$ is too large for `T`: for $x$ above about
/// $4\times10^{36}$ (`f32`) or $2.5\times10^{305}$ (`f64`).
///
/// See the [`Float::ln_abs_gamma_prec_round`] documentation for information on termination.
///
/// # Worst-case complexity
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use malachite_base::num::float::NiceFloat;
/// use malachite_float::float::gamma::ln_abs_gamma::primitive_float_ln_abs_gamma;
///
/// assert!(primitive_float_ln_abs_gamma(f32::NAN).0.is_nan());
/// let (y, sign) = primitive_float_ln_abs_gamma(f32::INFINITY);
/// assert_eq!(NiceFloat(y), NiceFloat(f32::INFINITY));
/// assert!(sign);
///
/// let (y, sign) = primitive_float_ln_abs_gamma(0.0f32);
/// assert_eq!(NiceFloat(y), NiceFloat(f32::INFINITY));
/// assert!(sign);
///
/// let (y, sign) = primitive_float_ln_abs_gamma(-0.0f32);
/// assert_eq!(NiceFloat(y), NiceFloat(f32::INFINITY));
/// assert!(!sign);
///
/// let (y, sign) = primitive_float_ln_abs_gamma(1.0f32);
/// assert_eq!(NiceFloat(y), NiceFloat(0.0));
/// assert!(sign);
///
/// let (y, sign) = primitive_float_ln_abs_gamma(0.5f32);
/// assert_eq!(NiceFloat(y), NiceFloat(0.5723649));
/// assert!(sign);
///
/// let (y, sign) = primitive_float_ln_abs_gamma(5.0f32);
/// assert_eq!(NiceFloat(y), NiceFloat(3.1780539));
/// assert!(sign);
///
/// let (y, sign) = primitive_float_ln_abs_gamma(-1.0f32);
/// assert_eq!(NiceFloat(y), NiceFloat(f32::INFINITY));
/// assert!(sign);
///
/// let (y, sign) = primitive_float_ln_abs_gamma(-0.5f32);
/// assert_eq!(NiceFloat(y), NiceFloat(1.2655121));
/// assert!(!sign);
/// ```
#[inline]
#[allow(clippy::type_repetition_in_bounds)]
pub fn primitive_float_ln_abs_gamma<T: PrimitiveFloat>(x: T) -> (T, bool)
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    let sign = Cell::new(true);
    let y = emulate_float_to_float_fn(
        |x, prec| {
            let (y, s, o) = x.ln_abs_gamma_prec(prec);
            sign.set(s);
            (y, o)
        },
        x,
    );
    (y, sign.get())
}

/// Computes $\ln |\Gamma(x)|$, the logarithm of the absolute value of the gamma function, of a
/// [`Rational`], returning the result as a primitive float. The sign of $\Gamma(x)$ is also
/// returned.
///
/// $\Gamma(x) = \int_0^\infty t^{x-1} e^{-t}\,dt$ for $x > 0$, and the recurrence $\Gamma(x) =
/// \Gamma(x+1)/x$ extends it to every real $x$ other than $0, -1, -2, \ldots$, where it has poles
/// and $\ln |\Gamma(x)|$ is taken to be $\infty$.
///
/// The `bool` is the sign of $\Gamma(x)$: `true` if it is positive, which it is for $x > 0$ and for
/// a negative $x$ that is not an integer and whose integer part is odd. Where $\Gamma(x)$ has no
/// sign (at the negative integers and at 0), `true` is returned.
///
/// $$
/// f(x) = (\ln |\Gamma(x)|+\varepsilon, \operatorname{sgn} \Gamma(x)).
/// $$
/// - If $\ln |\Gamma(x)|$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to
///   be 0.
/// - If $\ln |\Gamma(x)|$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2 |\ln
///   |\Gamma(x)||\rfloor-p}$, where $p$ is the precision of the output (typically 24 if `T` is a
///   [`f32`] and 53 if `T` is a [`f64`], but less if the output is subnormal).
///
/// Special cases:
/// - $f(x)=(\infty,\text{true})$ if $x$ is 0 or a negative integer
/// - $f(1)=f(2)=(0.0,\text{true})$
///
/// The result is $\infty$ when $\ln |\Gamma(x)|$ is too large for `T`: for $x$ above about
/// $4\times10^{36}$ (`f32`) or $2.5\times10^{305}$ (`f64`). It is $-\infty$ for a negative $x$ of
/// such a magnitude that is not an integer.
///
/// See the [`Float::ln_abs_gamma_rational_prec_round`] documentation for information on
/// termination.
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
/// use malachite_float::float::gamma::ln_abs_gamma::primitive_float_ln_abs_gamma_rational;
/// use malachite_q::Rational;
///
/// let (y, sign) = primitive_float_ln_abs_gamma_rational::<f64>(&Rational::ZERO);
/// assert_eq!(NiceFloat(y), NiceFloat(f64::INFINITY));
/// assert!(sign);
///
/// let (y, sign) = primitive_float_ln_abs_gamma_rational::<f64>(&Rational::ONE);
/// assert_eq!(NiceFloat(y), NiceFloat(0.0));
/// assert!(sign);
///
/// let (y, sign) = primitive_float_ln_abs_gamma_rational::<f64>(&Rational::from_unsigneds(1u8, 3));
/// assert_eq!(NiceFloat(y), NiceFloat(0.9854206469277671));
/// assert!(sign);
///
/// let (y, sign) = primitive_float_ln_abs_gamma_rational::<f64>(&Rational::from_signeds(-1i8, 3));
/// assert_eq!(NiceFloat(y), NiceFloat(1.4017625638156332));
/// assert!(!sign);
///
/// let (y, sign) = primitive_float_ln_abs_gamma_rational::<f64>(&Rational::NEGATIVE_ONE);
/// assert_eq!(NiceFloat(y), NiceFloat(f64::INFINITY));
/// assert!(sign);
///
/// let (y, sign) = primitive_float_ln_abs_gamma_rational::<f64>(&Rational::from(5));
/// assert_eq!(NiceFloat(y), NiceFloat(3.1780538303479458));
/// assert!(sign);
/// ```
#[inline]
#[allow(clippy::type_repetition_in_bounds)]
pub fn primitive_float_ln_abs_gamma_rational<T: PrimitiveFloat>(x: &Rational) -> (T, bool)
where
    Float: PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    let sign = Cell::new(true);
    let y = emulate_rational_to_float_fn(
        |x, prec| {
            let (y, s, o) = Float::ln_abs_gamma_rational_prec_ref(x, prec);
            sign.set(s);
            (y, o)
        },
        x,
    );
    (y, sign.get())
}
