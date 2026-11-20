// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::big_exponent_float::BigExponentFloat;
use crate::big_exponent_float::gamma::ln_gamma::{ln_gamma_helper, ln_gamma_rational_helper};
use core::cmp::Ordering;
use core::mem::swap;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_q::Rational;

impl BigExponentFloat {
    /// Computes $\ln |\Gamma(x)|$, the logarithm of the absolute value of the gamma function, of a
    /// [`BigExponentFloat`], rounding the result to the specified precision and with the specified
    /// rounding mode. The [`BigExponentFloat`] is taken by value. The sign of $\Gamma(x)$ and an
    /// [`Ordering`] are also returned, the latter indicating whether the rounded value is less
    /// than, equal to, or greater than the exact value. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $\Gamma(x) = \int_0^\infty t^{x-1} e^{-t}\,dt$ for $x > 0$, and the recurrence $\Gamma(x) =
    /// \Gamma(x+1)/x$ extends it to every real $x$ other than $0, -1, -2, \ldots$, where it has
    /// poles and $\ln |\Gamma(x)|$ is taken to be $\infty$.
    ///
    /// The `bool` is the sign of $\Gamma(x)$: `true` if it is positive, which it is for $x > 0$ and
    /// for a negative $x$ that is not an integer and whose integer part is odd. Where $\Gamma(x)$
    /// has no sign (at the negative integers, at 0, at $\pm\infty$, and at `NaN`), `true` is
    /// returned.
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
    /// Special cases:
    /// - $f(\text{NaN},p,m)=(\text{NaN},\text{true})$
    /// - $f(\pm\infty,p,m)=(\infty,\text{true})$
    /// - $f(x,p,m)=(\infty,\text{true})$ if $x$ is 0 or a negative integer
    /// - $f(1,p,m)=f(2,p,m)=(0,\text{true})$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::ln_abs_gamma_prec_round), this function never
    /// overflows or underflows.
    ///
    /// The result is correctly rounded. Finding it takes $\ln |\Gamma(x)|$ not to be a rounding
    /// boundary, which is a number with at most `prec + 1` significant bits. This is known when $x$
    /// is a positive integer (then the value is the logarithm of an integer, which is 0 or
    /// irrational); nothing suggests that it ever fails, but it has not been proven for every $x$,
    /// and this function would not terminate if it did fail.
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`BigExponentFloat::ln_abs_gamma_prec`] instead.
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
    /// use malachite_base::num::basic::traits::OneHalf;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (g, sign, o) = (-BigExponentFloat::ONE_HALF).ln_abs_gamma_prec_round(5, Floor);
    /// assert_eq!(g.to_string(), "1.2");
    /// assert!(!sign);
    /// assert_eq!(o, Less);
    ///
    /// let (g, sign, o) = (-BigExponentFloat::ONE_HALF).ln_abs_gamma_prec_round(5, Ceiling);
    /// assert_eq!(g.to_string(), "1.31");
    /// assert!(!sign);
    /// assert_eq!(o, Greater);
    ///
    /// let (g, sign, o) = (-BigExponentFloat::ONE_HALF).ln_abs_gamma_prec_round(5, Nearest);
    /// assert_eq!(g.to_string(), "1.2");
    /// assert!(!sign);
    /// assert_eq!(o, Less);
    ///
    /// let (g, sign, o) = (-BigExponentFloat::ONE_HALF).ln_abs_gamma_prec_round(20, Floor);
    /// assert_eq!(g.to_string(), "1.265511");
    /// assert!(!sign);
    /// assert_eq!(o, Less);
    ///
    /// let (g, sign, o) = (-BigExponentFloat::ONE_HALF).ln_abs_gamma_prec_round(20, Ceiling);
    /// assert_eq!(g.to_string(), "1.2655125");
    /// assert!(!sign);
    /// assert_eq!(o, Greater);
    ///
    /// let (g, sign, o) = (-BigExponentFloat::ONE_HALF).ln_abs_gamma_prec_round(20, Nearest);
    /// assert_eq!(g.to_string(), "1.2655125");
    /// assert!(!sign);
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn ln_abs_gamma_prec_round(self, prec: u64, rm: RoundingMode) -> (Self, bool, Ordering) {
        ln_gamma_helper(true, &self, prec, rm)
    }

    /// Computes $\ln |\Gamma(x)|$, the logarithm of the absolute value of the gamma function, of a
    /// [`BigExponentFloat`], rounding the result to the specified precision and with the specified
    /// rounding mode. The [`BigExponentFloat`] is taken by reference. The sign of $\Gamma(x)$ and
    /// an [`Ordering`] are also returned, the latter indicating whether the rounded value is less
    /// than, equal to, or greater than the exact value. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $\Gamma(x) = \int_0^\infty t^{x-1} e^{-t}\,dt$ for $x > 0$, and the recurrence $\Gamma(x) =
    /// \Gamma(x+1)/x$ extends it to every real $x$ other than $0, -1, -2, \ldots$, where it has
    /// poles and $\ln |\Gamma(x)|$ is taken to be $\infty$.
    ///
    /// The `bool` is the sign of $\Gamma(x)$: `true` if it is positive, which it is for $x > 0$ and
    /// for a negative $x$ that is not an integer and whose integer part is odd. Where $\Gamma(x)$
    /// has no sign (at the negative integers, at 0, at $\pm\infty$, and at `NaN`), `true` is
    /// returned.
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
    /// Special cases:
    /// - $f(\text{NaN},p,m)=(\text{NaN},\text{true})$
    /// - $f(\pm\infty,p,m)=(\infty,\text{true})$
    /// - $f(x,p,m)=(\infty,\text{true})$ if $x$ is 0 or a negative integer
    /// - $f(1,p,m)=f(2,p,m)=(0,\text{true})$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::ln_abs_gamma_prec_round), this function never
    /// overflows or underflows.
    ///
    /// The result is correctly rounded. Finding it takes $\ln |\Gamma(x)|$ not to be a rounding
    /// boundary, which is a number with at most `prec + 1` significant bits. This is known when $x$
    /// is a positive integer (then the value is the logarithm of an integer, which is 0 or
    /// irrational); nothing suggests that it ever fails, but it has not been proven for every $x$,
    /// and this function would not terminate if it did fail.
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`BigExponentFloat::ln_abs_gamma_prec_ref`] instead.
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
    /// use malachite_base::num::basic::traits::OneHalf;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (g, sign, o) = (-BigExponentFloat::ONE_HALF).ln_abs_gamma_prec_round_ref(5, Floor);
    /// assert_eq!(g.to_string(), "1.2");
    /// assert!(!sign);
    /// assert_eq!(o, Less);
    ///
    /// let (g, sign, o) = (-BigExponentFloat::ONE_HALF).ln_abs_gamma_prec_round_ref(5, Ceiling);
    /// assert_eq!(g.to_string(), "1.31");
    /// assert!(!sign);
    /// assert_eq!(o, Greater);
    ///
    /// let (g, sign, o) = (-BigExponentFloat::ONE_HALF).ln_abs_gamma_prec_round_ref(5, Nearest);
    /// assert_eq!(g.to_string(), "1.2");
    /// assert!(!sign);
    /// assert_eq!(o, Less);
    ///
    /// let (g, sign, o) = (-BigExponentFloat::ONE_HALF).ln_abs_gamma_prec_round_ref(20, Floor);
    /// assert_eq!(g.to_string(), "1.265511");
    /// assert!(!sign);
    /// assert_eq!(o, Less);
    ///
    /// let (g, sign, o) = (-BigExponentFloat::ONE_HALF).ln_abs_gamma_prec_round_ref(20, Ceiling);
    /// assert_eq!(g.to_string(), "1.2655125");
    /// assert!(!sign);
    /// assert_eq!(o, Greater);
    ///
    /// let (g, sign, o) = (-BigExponentFloat::ONE_HALF).ln_abs_gamma_prec_round_ref(20, Nearest);
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
        ln_gamma_helper(true, self, prec, rm)
    }

    /// Computes $\ln |\Gamma(x)|$, the logarithm of the absolute value of the gamma function, of a
    /// [`BigExponentFloat`] in place, rounding the result to the specified precision and with the
    /// specified rounding mode. The sign of $\Gamma(x)$ and an [`Ordering`] are returned, the
    /// latter indicating whether the rounded value is less than, equal to, or greater than the
    /// exact value. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this
    /// function sets the [`BigExponentFloat`] to `NaN` it also returns `Equal`.
    ///
    /// $\Gamma(x) = \int_0^\infty t^{x-1} e^{-t}\,dt$ for $x > 0$, and the recurrence $\Gamma(x) =
    /// \Gamma(x+1)/x$ extends it to every real $x$ other than $0, -1, -2, \ldots$, where it has
    /// poles and $\ln |\Gamma(x)|$ is taken to be $\infty$.
    ///
    /// The `bool` is the sign of $\Gamma(x)$: `true` if it is positive, which it is for $x > 0$ and
    /// for a negative $x$ that is not an integer and whose integer part is odd. Where $\Gamma(x)$
    /// has no sign (at the negative integers, at 0, at $\pm\infty$, and at `NaN`), `true` is
    /// returned.
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
    /// See the [`BigExponentFloat::ln_abs_gamma_prec_round`] documentation for information on
    /// special cases and termination.
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`BigExponentFloat::ln_abs_gamma_prec_assign`] instead.
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
    /// use malachite_base::num::basic::traits::OneHalf;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = -BigExponentFloat::ONE_HALF;
    /// assert_eq!(x.ln_abs_gamma_prec_round_assign(5, Floor), (false, Less));
    /// assert_eq!(x.to_string(), "1.2");
    ///
    /// let mut x = -BigExponentFloat::ONE_HALF;
    /// assert_eq!(
    ///     x.ln_abs_gamma_prec_round_assign(5, Ceiling),
    ///     (false, Greater)
    /// );
    /// assert_eq!(x.to_string(), "1.31");
    ///
    /// let mut x = -BigExponentFloat::ONE_HALF;
    /// assert_eq!(x.ln_abs_gamma_prec_round_assign(5, Nearest), (false, Less));
    /// assert_eq!(x.to_string(), "1.2");
    ///
    /// let mut x = -BigExponentFloat::ONE_HALF;
    /// assert_eq!(x.ln_abs_gamma_prec_round_assign(20, Floor), (false, Less));
    /// assert_eq!(x.to_string(), "1.265511");
    ///
    /// let mut x = -BigExponentFloat::ONE_HALF;
    /// assert_eq!(
    ///     x.ln_abs_gamma_prec_round_assign(20, Ceiling),
    ///     (false, Greater)
    /// );
    /// assert_eq!(x.to_string(), "1.2655125");
    ///
    /// let mut x = -BigExponentFloat::ONE_HALF;
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
        let (mut y, sign, o) = ln_gamma_helper(true, self, prec, rm);
        swap(self, &mut y);
        (sign, o)
    }

    /// Computes $\ln |\Gamma(x)|$, the logarithm of the absolute value of the gamma function, of a
    /// [`BigExponentFloat`], rounding the result to the nearest value of the specified precision.
    /// The [`BigExponentFloat`] is taken by value. The sign of $\Gamma(x)$ and an [`Ordering`] are
    /// also returned, the latter indicating whether the rounded value is less than, equal to, or
    /// greater than the exact value. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $\Gamma(x) = \int_0^\infty t^{x-1} e^{-t}\,dt$ for $x > 0$, and the recurrence $\Gamma(x) =
    /// \Gamma(x+1)/x$ extends it to every real $x$ other than $0, -1, -2, \ldots$, where it has
    /// poles and $\ln |\Gamma(x)|$ is taken to be $\infty$.
    ///
    /// The `bool` is the sign of $\Gamma(x)$: `true` if it is positive, which it is for $x > 0$ and
    /// for a negative $x$ that is not an integer and whose integer part is odd. Where $\Gamma(x)$
    /// has no sign (at the negative integers, at 0, at $\pm\infty$, and at `NaN`), `true` is
    /// returned.
    ///
    /// If the value is equidistant from two [`BigExponentFloat`]s with the specified precision, the
    /// [`BigExponentFloat`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`]
    /// for a description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = (\ln |\Gamma(x)|+\varepsilon, \operatorname{sgn} \Gamma(x)).
    /// $$
    /// - If $\ln |\Gamma(x)|$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed
    ///   to be 0.
    /// - If $\ln |\Gamma(x)|$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2 |\ln
    ///   |\Gamma(x)||\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p)=(\text{NaN},\text{true})$
    /// - $f(\pm\infty,p)=(\infty,\text{true})$
    /// - $f(x,p)=(\infty,\text{true})$ if $x$ is 0 or a negative integer
    /// - $f(1,p)=f(2,p)=(0,\text{true})$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::ln_abs_gamma_prec_round), this function never
    /// overflows or underflows.
    ///
    /// The result is correctly rounded. Finding it takes $\ln |\Gamma(x)|$ not to be a rounding
    /// boundary, which is a number with at most `prec + 1` significant bits. This is known when $x$
    /// is a positive integer (then the value is the logarithm of an integer, which is 0 or
    /// irrational); nothing suggests that it ever fails, but it has not been proven for every $x$,
    /// and this function would not terminate if it did fail.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`BigExponentFloat::ln_abs_gamma_prec_round`] instead.
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
    /// use malachite_base::num::arithmetic::traits::PowerOf2;
    /// use malachite_base::num::basic::traits::OneHalf;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (g, sign, o) = (-BigExponentFloat::ONE_HALF).ln_abs_gamma_prec(5);
    /// assert_eq!(g.to_string(), "1.2");
    /// assert!(!sign);
    /// assert_eq!(o, Less);
    ///
    /// let (g, sign, o) = (-BigExponentFloat::ONE_HALF).ln_abs_gamma_prec(20);
    /// assert_eq!(g.to_string(), "1.2655125");
    /// assert!(!sign);
    /// assert_eq!(o, Greater);
    ///
    /// let (g, sign, o) = BigExponentFloat::power_of_2(1u64 << 40).ln_abs_gamma_prec(20);
    /// assert_eq!(g.to_string(), "6.1406009e330985980553");
    /// assert!(sign);
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn ln_abs_gamma_prec(self, prec: u64) -> (Self, bool, Ordering) {
        self.ln_abs_gamma_prec_round(prec, Nearest)
    }

    /// Computes $\ln |\Gamma(x)|$, the logarithm of the absolute value of the gamma function, of a
    /// [`BigExponentFloat`], rounding the result to the nearest value of the specified precision.
    /// The [`BigExponentFloat`] is taken by reference. The sign of $\Gamma(x)$ and an [`Ordering`]
    /// are also returned, the latter indicating whether the rounded value is less than, equal to,
    /// or greater than the exact value. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $\Gamma(x) = \int_0^\infty t^{x-1} e^{-t}\,dt$ for $x > 0$, and the recurrence $\Gamma(x) =
    /// \Gamma(x+1)/x$ extends it to every real $x$ other than $0, -1, -2, \ldots$, where it has
    /// poles and $\ln |\Gamma(x)|$ is taken to be $\infty$.
    ///
    /// The `bool` is the sign of $\Gamma(x)$: `true` if it is positive, which it is for $x > 0$ and
    /// for a negative $x$ that is not an integer and whose integer part is odd. Where $\Gamma(x)$
    /// has no sign (at the negative integers, at 0, at $\pm\infty$, and at `NaN`), `true` is
    /// returned.
    ///
    /// If the value is equidistant from two [`BigExponentFloat`]s with the specified precision, the
    /// [`BigExponentFloat`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`]
    /// for a description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = (\ln |\Gamma(x)|+\varepsilon, \operatorname{sgn} \Gamma(x)).
    /// $$
    /// - If $\ln |\Gamma(x)|$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed
    ///   to be 0.
    /// - If $\ln |\Gamma(x)|$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2 |\ln
    ///   |\Gamma(x)||\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p)=(\text{NaN},\text{true})$
    /// - $f(\pm\infty,p)=(\infty,\text{true})$
    /// - $f(x,p)=(\infty,\text{true})$ if $x$ is 0 or a negative integer
    /// - $f(1,p)=f(2,p)=(0,\text{true})$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::ln_abs_gamma_prec_round), this function never
    /// overflows or underflows.
    ///
    /// The result is correctly rounded. Finding it takes $\ln |\Gamma(x)|$ not to be a rounding
    /// boundary, which is a number with at most `prec + 1` significant bits. This is known when $x$
    /// is a positive integer (then the value is the logarithm of an integer, which is 0 or
    /// irrational); nothing suggests that it ever fails, but it has not been proven for every $x$,
    /// and this function would not terminate if it did fail.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`BigExponentFloat::ln_abs_gamma_prec_round_ref`] instead.
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
    /// use malachite_base::num::arithmetic::traits::PowerOf2;
    /// use malachite_base::num::basic::traits::OneHalf;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (g, sign, o) = (-BigExponentFloat::ONE_HALF).ln_abs_gamma_prec_ref(5);
    /// assert_eq!(g.to_string(), "1.2");
    /// assert!(!sign);
    /// assert_eq!(o, Less);
    ///
    /// let (g, sign, o) = (-BigExponentFloat::ONE_HALF).ln_abs_gamma_prec_ref(20);
    /// assert_eq!(g.to_string(), "1.2655125");
    /// assert!(!sign);
    /// assert_eq!(o, Greater);
    ///
    /// let (g, sign, o) = BigExponentFloat::power_of_2(1u64 << 40).ln_abs_gamma_prec_ref(20);
    /// assert_eq!(g.to_string(), "6.1406009e330985980553");
    /// assert!(sign);
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn ln_abs_gamma_prec_ref(&self, prec: u64) -> (Self, bool, Ordering) {
        self.ln_abs_gamma_prec_round_ref(prec, Nearest)
    }

    /// Computes $\ln |\Gamma(x)|$, the logarithm of the absolute value of the gamma function, of a
    /// [`BigExponentFloat`] in place, rounding the result to the nearest value of the specified
    /// precision. The sign of $\Gamma(x)$ and an [`Ordering`] are returned, the latter indicating
    /// whether the rounded value is less than, equal to, or greater than the exact value. Although
    /// `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function sets the
    /// [`BigExponentFloat`] to `NaN` it also returns `Equal`.
    ///
    /// $\Gamma(x) = \int_0^\infty t^{x-1} e^{-t}\,dt$ for $x > 0$, and the recurrence $\Gamma(x) =
    /// \Gamma(x+1)/x$ extends it to every real $x$ other than $0, -1, -2, \ldots$, where it has
    /// poles and $\ln |\Gamma(x)|$ is taken to be $\infty$.
    ///
    /// The `bool` is the sign of $\Gamma(x)$: `true` if it is positive, which it is for $x > 0$ and
    /// for a negative $x$ that is not an integer and whose integer part is odd. Where $\Gamma(x)$
    /// has no sign (at the negative integers, at 0, at $\pm\infty$, and at `NaN`), `true` is
    /// returned.
    ///
    /// If the value is equidistant from two [`BigExponentFloat`]s with the specified precision, the
    /// [`BigExponentFloat`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`]
    /// for a description of the `Nearest` rounding mode.
    ///
    /// $$
    /// x \gets \ln |\Gamma(x)|+\varepsilon.
    /// $$
    /// - If $\ln |\Gamma(x)|$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed
    ///   to be 0.
    /// - If $\ln |\Gamma(x)|$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2 |\ln
    ///   |\Gamma(x)||\rfloor-p}$.
    ///
    /// See the [`BigExponentFloat::ln_abs_gamma_prec_round`] documentation for information on
    /// special cases and termination.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`BigExponentFloat::ln_abs_gamma_prec_round_assign`] instead.
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
    /// use malachite_base::num::arithmetic::traits::PowerOf2;
    /// use malachite_base::num::basic::traits::OneHalf;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = -BigExponentFloat::ONE_HALF;
    /// assert_eq!(x.ln_abs_gamma_prec_assign(5), (false, Less));
    /// assert_eq!(x.to_string(), "1.2");
    ///
    /// let mut x = -BigExponentFloat::ONE_HALF;
    /// assert_eq!(x.ln_abs_gamma_prec_assign(20), (false, Greater));
    /// assert_eq!(x.to_string(), "1.2655125");
    ///
    /// let mut x = BigExponentFloat::power_of_2(1u64 << 40);
    /// assert_eq!(x.ln_abs_gamma_prec_assign(20), (true, Less));
    /// assert_eq!(x.to_string(), "6.1406009e330985980553");
    /// ```
    #[inline]
    pub fn ln_abs_gamma_prec_assign(&mut self, prec: u64) -> (bool, Ordering) {
        self.ln_abs_gamma_prec_round_assign(prec, Nearest)
    }

    /// Computes $\ln |\Gamma(x)|$, the logarithm of the absolute value of the gamma function, of a
    /// [`Rational`], rounding the result to the specified precision and with the specified rounding
    /// mode and returning the result as a [`BigExponentFloat`]. The [`Rational`] is taken by value.
    /// The sign of $\Gamma(x)$ and an [`Ordering`] are also returned, the latter indicating whether
    /// the rounded value is less than, equal to, or greater than the exact value. Although `NaN`s
    /// are not comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it
    /// also returns `Equal`.
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
    /// Special cases:
    /// - $f(x,p,m)=(\infty,\text{true})$ if $x$ is 0 or a negative integer
    /// - $f(1,p,m)=f(2,p,m)=(0,\text{true})$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::ln_abs_gamma_rational_prec_round), this
    /// function never overflows or underflows.
    ///
    /// The result is correctly rounded. Finding it takes $\ln |\Gamma(x)|$ not to be a rounding
    /// boundary, which is a number with at most `prec + 1` significant bits. This is known when $x$
    /// is a positive integer (then the value is the logarithm of an integer, which is 0 or
    /// irrational); nothing suggests that it ever fails, but it has not been proven for every
    /// rational $x$, and this function would not terminate if it did fail.
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`BigExponentFloat::ln_abs_gamma_rational_prec`] instead.
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
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (g, sign, o) = BigExponentFloat::ln_abs_gamma_rational_prec_round(
    ///     Rational::from_signeds(-1i8, 3),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(g.to_string(), "1.38");
    /// assert!(!sign);
    /// assert_eq!(o, Less);
    ///
    /// let (g, sign, o) = BigExponentFloat::ln_abs_gamma_rational_prec_round(
    ///     Rational::from_signeds(-1i8, 3),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(g.to_string(), "1.44");
    /// assert!(!sign);
    /// assert_eq!(o, Greater);
    ///
    /// let (g, sign, o) = BigExponentFloat::ln_abs_gamma_rational_prec_round(
    ///     Rational::from_signeds(-1i8, 3),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(g.to_string(), "1.38");
    /// assert!(!sign);
    /// assert_eq!(o, Less);
    ///
    /// let (g, sign, o) = BigExponentFloat::ln_abs_gamma_rational_prec_round(
    ///     Rational::from_signeds(-1i8, 3),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(g.to_string(), "1.4017620");
    /// assert!(!sign);
    /// assert_eq!(o, Less);
    ///
    /// let (g, sign, o) = BigExponentFloat::ln_abs_gamma_rational_prec_round(
    ///     Rational::from_signeds(-1i8, 3),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(g.to_string(), "1.40176");
    /// assert!(!sign);
    /// assert_eq!(o, Greater);
    ///
    /// let (g, sign, o) = BigExponentFloat::ln_abs_gamma_rational_prec_round(
    ///     Rational::from_signeds(-1i8, 3),
    ///     20,
    ///     Nearest,
    /// );
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
        ln_gamma_rational_helper(true, &x, prec, rm)
    }

    /// Computes $\ln |\Gamma(x)|$, the logarithm of the absolute value of the gamma function, of a
    /// [`Rational`], rounding the result to the specified precision and with the specified rounding
    /// mode and returning the result as a [`BigExponentFloat`]. The [`Rational`] is taken by
    /// reference. The sign of $\Gamma(x)$ and an [`Ordering`] are also returned, the latter
    /// indicating whether the rounded value is less than, equal to, or greater than the exact
    /// value. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
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
    /// Special cases:
    /// - $f(x,p,m)=(\infty,\text{true})$ if $x$ is 0 or a negative integer
    /// - $f(1,p,m)=f(2,p,m)=(0,\text{true})$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::ln_abs_gamma_rational_prec_round), this
    /// function never overflows or underflows.
    ///
    /// The result is correctly rounded. Finding it takes $\ln |\Gamma(x)|$ not to be a rounding
    /// boundary, which is a number with at most `prec + 1` significant bits. This is known when $x$
    /// is a positive integer (then the value is the logarithm of an integer, which is 0 or
    /// irrational); nothing suggests that it ever fails, but it has not been proven for every
    /// rational $x$, and this function would not terminate if it did fail.
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`BigExponentFloat::ln_abs_gamma_rational_prec_ref`] instead.
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
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (g, sign, o) = BigExponentFloat::ln_abs_gamma_rational_prec_round_ref(
    ///     &Rational::from_signeds(-1i8, 3),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(g.to_string(), "1.38");
    /// assert!(!sign);
    /// assert_eq!(o, Less);
    ///
    /// let (g, sign, o) = BigExponentFloat::ln_abs_gamma_rational_prec_round_ref(
    ///     &Rational::from_signeds(-1i8, 3),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(g.to_string(), "1.44");
    /// assert!(!sign);
    /// assert_eq!(o, Greater);
    ///
    /// let (g, sign, o) = BigExponentFloat::ln_abs_gamma_rational_prec_round_ref(
    ///     &Rational::from_signeds(-1i8, 3),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(g.to_string(), "1.38");
    /// assert!(!sign);
    /// assert_eq!(o, Less);
    ///
    /// let (g, sign, o) = BigExponentFloat::ln_abs_gamma_rational_prec_round_ref(
    ///     &Rational::from_signeds(-1i8, 3),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(g.to_string(), "1.4017620");
    /// assert!(!sign);
    /// assert_eq!(o, Less);
    ///
    /// let (g, sign, o) = BigExponentFloat::ln_abs_gamma_rational_prec_round_ref(
    ///     &Rational::from_signeds(-1i8, 3),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(g.to_string(), "1.40176");
    /// assert!(!sign);
    /// assert_eq!(o, Greater);
    ///
    /// let (g, sign, o) = BigExponentFloat::ln_abs_gamma_rational_prec_round_ref(
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
        ln_gamma_rational_helper(true, x, prec, rm)
    }

    /// Computes $\ln |\Gamma(x)|$, the logarithm of the absolute value of the gamma function, of a
    /// [`Rational`], rounding the result to the nearest value of the specified precision and
    /// returning the result as a [`BigExponentFloat`]. The [`Rational`] is taken by value. The sign
    /// of $\Gamma(x)$ and an [`Ordering`] are also returned, the latter indicating whether the
    /// rounded value is less than, equal to, or greater than the exact value. Although `NaN`s are
    /// not comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// $\Gamma(x) = \int_0^\infty t^{x-1} e^{-t}\,dt$ for $x > 0$, and the recurrence $\Gamma(x) =
    /// \Gamma(x+1)/x$ extends it to every real $x$ other than $0, -1, -2, \ldots$, where it has
    /// poles and $\ln |\Gamma(x)|$ is taken to be $\infty$.
    ///
    /// The `bool` is the sign of $\Gamma(x)$: `true` if it is positive, which it is for $x > 0$ and
    /// for a negative $x$ that is not an integer and whose integer part is odd. Where $\Gamma(x)$
    /// has no sign (at the negative integers and at 0), `true` is returned.
    ///
    /// If the value is equidistant from two [`BigExponentFloat`]s with the specified precision, the
    /// [`BigExponentFloat`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`]
    /// for a description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = (\ln |\Gamma(x)|+\varepsilon, \operatorname{sgn} \Gamma(x)).
    /// $$
    /// - If $\ln |\Gamma(x)|$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed
    ///   to be 0.
    /// - If $\ln |\Gamma(x)|$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2 |\ln
    ///   |\Gamma(x)||\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,p)=(\infty,\text{true})$ if $x$ is 0 or a negative integer
    /// - $f(1,p)=f(2,p)=(0,\text{true})$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::ln_abs_gamma_rational_prec_round), this
    /// function never overflows or underflows.
    ///
    /// The result is correctly rounded. Finding it takes $\ln |\Gamma(x)|$ not to be a rounding
    /// boundary, which is a number with at most `prec + 1` significant bits. This is known when $x$
    /// is a positive integer (then the value is the logarithm of an integer, which is 0 or
    /// irrational); nothing suggests that it ever fails, but it has not been proven for every
    /// rational $x$, and this function would not terminate if it did fail.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`BigExponentFloat::ln_abs_gamma_rational_prec_round`] instead.
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
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (g, sign, o) =
    ///     BigExponentFloat::ln_abs_gamma_rational_prec(Rational::from_signeds(-1i8, 3), 5);
    /// assert_eq!(g.to_string(), "1.38");
    /// assert!(!sign);
    /// assert_eq!(o, Less);
    ///
    /// let (g, sign, o) =
    ///     BigExponentFloat::ln_abs_gamma_rational_prec(Rational::from_signeds(-1i8, 3), 20);
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
    /// returning the result as a [`BigExponentFloat`]. The [`Rational`] is taken by reference. The
    /// sign of $\Gamma(x)$ and an [`Ordering`] are also returned, the latter indicating whether the
    /// rounded value is less than, equal to, or greater than the exact value. Although `NaN`s are
    /// not comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// $\Gamma(x) = \int_0^\infty t^{x-1} e^{-t}\,dt$ for $x > 0$, and the recurrence $\Gamma(x) =
    /// \Gamma(x+1)/x$ extends it to every real $x$ other than $0, -1, -2, \ldots$, where it has
    /// poles and $\ln |\Gamma(x)|$ is taken to be $\infty$.
    ///
    /// The `bool` is the sign of $\Gamma(x)$: `true` if it is positive, which it is for $x > 0$ and
    /// for a negative $x$ that is not an integer and whose integer part is odd. Where $\Gamma(x)$
    /// has no sign (at the negative integers and at 0), `true` is returned.
    ///
    /// If the value is equidistant from two [`BigExponentFloat`]s with the specified precision, the
    /// [`BigExponentFloat`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`]
    /// for a description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = (\ln |\Gamma(x)|+\varepsilon, \operatorname{sgn} \Gamma(x)).
    /// $$
    /// - If $\ln |\Gamma(x)|$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed
    ///   to be 0.
    /// - If $\ln |\Gamma(x)|$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2 |\ln
    ///   |\Gamma(x)||\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,p)=(\infty,\text{true})$ if $x$ is 0 or a negative integer
    /// - $f(1,p)=f(2,p)=(0,\text{true})$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::ln_abs_gamma_rational_prec_round), this
    /// function never overflows or underflows.
    ///
    /// The result is correctly rounded. Finding it takes $\ln |\Gamma(x)|$ not to be a rounding
    /// boundary, which is a number with at most `prec + 1` significant bits. This is known when $x$
    /// is a positive integer (then the value is the logarithm of an integer, which is 0 or
    /// irrational); nothing suggests that it ever fails, but it has not been proven for every
    /// rational $x$, and this function would not terminate if it did fail.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`BigExponentFloat::ln_abs_gamma_rational_prec_round_ref`] instead.
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
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (g, sign, o) =
    ///     BigExponentFloat::ln_abs_gamma_rational_prec_ref(&Rational::from_signeds(-1i8, 3), 5);
    /// assert_eq!(g.to_string(), "1.38");
    /// assert!(!sign);
    /// assert_eq!(o, Less);
    ///
    /// let (g, sign, o) =
    ///     BigExponentFloat::ln_abs_gamma_rational_prec_ref(&Rational::from_signeds(-1i8, 3), 20);
    /// assert_eq!(g.to_string(), "1.4017620");
    /// assert!(!sign);
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn ln_abs_gamma_rational_prec_ref(x: &Rational, prec: u64) -> (Self, bool, Ordering) {
        Self::ln_abs_gamma_rational_prec_round_ref(x, prec, Nearest)
    }
}
