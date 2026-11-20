// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::float::gamma::gamma::{gamma_prec_round_helper, gamma_rational_prec_round_helper};
use crate::{Float, emulate_float_to_float_fn, emulate_rational_to_float_fn};
use core::cmp::Ordering;
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::conversion::traits::{ExactFrom, RoundingFrom};
use malachite_base::num::gamma::traits::{ReciprocalGamma, ReciprocalGammaAssign};
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_q::Rational;

impl Float {
    /// Computes $1/\Gamma(x)$, the reciprocal gamma function, of a [`Float`], rounding the result
    /// to the specified precision and with the specified rounding mode. The [`Float`] is taken by
    /// value. An [`Ordering`] is also returned, indicating whether the rounded value is less than,
    /// equal to, or greater than the exact value. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $\Gamma(x) = \int_0^\infty t^{x-1} e^{-t}\,dt$ for $x > 0$, and the recurrence $\Gamma(x) =
    /// \Gamma(x+1)/x$ extends it to every real $x$ other than $0, -1, -2, \ldots$, where it has
    /// poles. Its reciprocal is defined for every real $x$, and is 0 at those poles.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = 1/\Gamma(x)+\varepsilon.
    /// $$
    /// - If $1/\Gamma(x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $1/\Gamma(x)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |1/\Gamma(x)|\rfloor-p+1}$.
    /// - If $1/\Gamma(x)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |1/\Gamma(x)|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=\text{NaN}$
    /// - $f(\infty,p,m)=0.0$
    /// - $f(-\infty,p,m)=\text{NaN}$
    /// - $f(\pm0.0,p,m)=\pm0.0$
    /// - $f(x,p,m)=0.0$ if $x$ is a negative integer
    ///
    /// Overflow and underflow:
    /// - If $f(x,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$ is
    ///   returned instead.
    /// - If $f(x,p,m)\leq -2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is
    ///   returned instead.
    /// - If $f(x,p,m)\leq -2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`, $-(1-(1/2)^p)2^{2^{30}-1}$
    ///   is returned instead.
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
    /// Overflow happens for arguments below about $-4.5\times10^7$ that are not integers, and
    /// underflow for arguments above about $4.5\times10^7$. The negative [`Float`]s closest to
    /// zero, $-2^{-2^{30}}$ at any precision, underflow too: there $1/\Gamma(x)$ is slightly closer
    /// to zero than $x$.
    ///
    /// The result is correctly rounded. Finding it takes $1/\Gamma(x)$ not to be a rounding
    /// boundary, which is a number with at most `prec + 1` significant bits. This is known when $x$
    /// is an integer, and when it is another multiple of $1/4$ (then $\Gamma(x)$ is irrational);
    /// nothing suggests that it ever fails, but it has not been proven for every $x$, and this
    /// function would not terminate if it did fail.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::reciprocal_gamma_prec`]
    /// instead. If you know that your target precision is the precision of the input, consider
    /// using [`Float::reciprocal_gamma_round`] instead. If both of these things are true, consider
    /// using [`Float::reciprocal_gamma`] instead.
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
    /// with the given precision (it can only when it is zero or `NaN`, or when $x$ is 1, 2, or 3).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (g, o) = Float::one_half_prec(100).reciprocal_gamma_prec_round(5, Floor);
    /// assert_eq!(g.to_string(), "0.562");
    /// assert_eq!(o, Less);
    ///
    /// let (g, o) = Float::one_half_prec(100).reciprocal_gamma_prec_round(5, Ceiling);
    /// assert_eq!(g.to_string(), "0.594");
    /// assert_eq!(o, Greater);
    ///
    /// let (g, o) = Float::one_half_prec(100).reciprocal_gamma_prec_round(5, Nearest);
    /// assert_eq!(g.to_string(), "0.562");
    /// assert_eq!(o, Less);
    ///
    /// let (g, o) = Float::one_half_prec(100).reciprocal_gamma_prec_round(20, Floor);
    /// assert_eq!(g.to_string(), "0.56418896");
    /// assert_eq!(o, Less);
    ///
    /// let (g, o) = Float::one_half_prec(100).reciprocal_gamma_prec_round(20, Ceiling);
    /// assert_eq!(g.to_string(), "0.56418991");
    /// assert_eq!(o, Greater);
    ///
    /// let (g, o) = Float::one_half_prec(100).reciprocal_gamma_prec_round(20, Nearest);
    /// assert_eq!(g.to_string(), "0.56418991");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn reciprocal_gamma_prec_round(self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        gamma_prec_round_helper(true, &self, prec, rm)
    }

    /// Computes $1/\Gamma(x)$, the reciprocal gamma function, of a [`Float`], rounding the result
    /// to the specified precision and with the specified rounding mode. The [`Float`] is taken by
    /// reference. An [`Ordering`] is also returned, indicating whether the rounded value is less
    /// than, equal to, or greater than the exact value. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $\Gamma(x) = \int_0^\infty t^{x-1} e^{-t}\,dt$ for $x > 0$, and the recurrence $\Gamma(x) =
    /// \Gamma(x+1)/x$ extends it to every real $x$ other than $0, -1, -2, \ldots$, where it has
    /// poles. Its reciprocal is defined for every real $x$, and is 0 at those poles.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = 1/\Gamma(x)+\varepsilon.
    /// $$
    /// - If $1/\Gamma(x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $1/\Gamma(x)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |1/\Gamma(x)|\rfloor-p+1}$.
    /// - If $1/\Gamma(x)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |1/\Gamma(x)|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=\text{NaN}$
    /// - $f(\infty,p,m)=0.0$
    /// - $f(-\infty,p,m)=\text{NaN}$
    /// - $f(\pm0.0,p,m)=\pm0.0$
    /// - $f(x,p,m)=0.0$ if $x$ is a negative integer
    ///
    /// Overflow and underflow:
    /// - If $f(x,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$ is
    ///   returned instead.
    /// - If $f(x,p,m)\leq -2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is
    ///   returned instead.
    /// - If $f(x,p,m)\leq -2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`, $-(1-(1/2)^p)2^{2^{30}-1}$
    ///   is returned instead.
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
    /// Overflow happens for arguments below about $-4.5\times10^7$ that are not integers, and
    /// underflow for arguments above about $4.5\times10^7$. The negative [`Float`]s closest to
    /// zero, $-2^{-2^{30}}$ at any precision, underflow too: there $1/\Gamma(x)$ is slightly closer
    /// to zero than $x$.
    ///
    /// The result is correctly rounded. Finding it takes $1/\Gamma(x)$ not to be a rounding
    /// boundary, which is a number with at most `prec + 1` significant bits. This is known when $x$
    /// is an integer, and when it is another multiple of $1/4$ (then $\Gamma(x)$ is irrational);
    /// nothing suggests that it ever fails, but it has not been proven for every $x$, and this
    /// function would not terminate if it did fail.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::reciprocal_gamma_prec_ref`]
    /// instead. If you know that your target precision is the precision of the input, consider
    /// using [`Float::reciprocal_gamma_round_ref`] instead. If both of these things are true,
    /// consider using `(&Float).reciprocal_gamma()` instead.
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
    /// with the given precision (it can only when it is zero or `NaN`, or when $x$ is 1, 2, or 3).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (g, o) = Float::one_half_prec(100).reciprocal_gamma_prec_round_ref(5, Floor);
    /// assert_eq!(g.to_string(), "0.562");
    /// assert_eq!(o, Less);
    ///
    /// let (g, o) = Float::one_half_prec(100).reciprocal_gamma_prec_round_ref(5, Ceiling);
    /// assert_eq!(g.to_string(), "0.594");
    /// assert_eq!(o, Greater);
    ///
    /// let (g, o) = Float::one_half_prec(100).reciprocal_gamma_prec_round_ref(5, Nearest);
    /// assert_eq!(g.to_string(), "0.562");
    /// assert_eq!(o, Less);
    ///
    /// let (g, o) = Float::one_half_prec(100).reciprocal_gamma_prec_round_ref(20, Floor);
    /// assert_eq!(g.to_string(), "0.56418896");
    /// assert_eq!(o, Less);
    ///
    /// let (g, o) = Float::one_half_prec(100).reciprocal_gamma_prec_round_ref(20, Ceiling);
    /// assert_eq!(g.to_string(), "0.56418991");
    /// assert_eq!(o, Greater);
    ///
    /// let (g, o) = Float::one_half_prec(100).reciprocal_gamma_prec_round_ref(20, Nearest);
    /// assert_eq!(g.to_string(), "0.56418991");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn reciprocal_gamma_prec_round_ref(&self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        gamma_prec_round_helper(true, self, prec, rm)
    }

    /// Computes $1/\Gamma(x)$, the reciprocal gamma function, of a [`Float`], rounding the result
    /// to the nearest value of the specified precision. The [`Float`] is taken by value. An
    /// [`Ordering`] is also returned, indicating whether the rounded value is less than, equal to,
    /// or greater than the exact value. Although `NaN`s are not comparable to any [`Float`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $\Gamma(x) = \int_0^\infty t^{x-1} e^{-t}\,dt$ for $x > 0$, and the recurrence $\Gamma(x) =
    /// \Gamma(x+1)/x$ extends it to every real $x$ other than $0, -1, -2, \ldots$, where it has
    /// poles. Its reciprocal is defined for every real $x$, and is 0 at those poles.
    ///
    /// If the value is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = 1/\Gamma(x)+\varepsilon.
    /// $$
    /// - If $1/\Gamma(x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $1/\Gamma(x)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |1/\Gamma(x)|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p)=\text{NaN}$
    /// - $f(\infty,p)=0.0$
    /// - $f(-\infty,p)=\text{NaN}$
    /// - $f(\pm0.0,p)=\pm0.0$
    /// - $f(x,p)=0.0$ if $x$ is a negative integer
    ///
    /// Overflow and underflow:
    /// - If $f(x,p)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,p)\leq -2^{2^{30}-1}$, $-\infty$ is returned instead.
    /// - If $0<f(x,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// Overflow happens for arguments below about $-4.5\times10^7$ that are not integers, and
    /// underflow for arguments above about $4.5\times10^7$. The negative [`Float`]s closest to
    /// zero, $-2^{-2^{30}}$ at any precision, underflow too: there $1/\Gamma(x)$ is slightly closer
    /// to zero than $x$.
    ///
    /// The result is correctly rounded. Finding it takes $1/\Gamma(x)$ not to be a rounding
    /// boundary, which is a number with at most `prec + 1` significant bits. This is known when $x$
    /// is an integer, and when it is another multiple of $1/4$ (then $\Gamma(x)$ is irrational);
    /// nothing suggests that it ever fails, but it has not been proven for every $x$, and this
    /// function would not terminate if it did fail.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::reciprocal_gamma_prec_round`] instead. If you know that your target precision is
    /// the precision of the input, consider using [`Float::reciprocal_gamma`] instead.
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
    /// let (g, o) = Float::one_half_prec(100).reciprocal_gamma_prec(5);
    /// assert_eq!(g.to_string(), "0.562");
    /// assert_eq!(o, Less);
    ///
    /// let (g, o) = Float::one_half_prec(100).reciprocal_gamma_prec(20);
    /// assert_eq!(g.to_string(), "0.56418991");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn reciprocal_gamma_prec(self, prec: u64) -> (Self, Ordering) {
        self.reciprocal_gamma_prec_round(prec, Nearest)
    }

    /// Computes $1/\Gamma(x)$, the reciprocal gamma function, of a [`Float`], rounding the result
    /// to the nearest value of the specified precision. The [`Float`] is taken by reference. An
    /// [`Ordering`] is also returned, indicating whether the rounded value is less than, equal to,
    /// or greater than the exact value. Although `NaN`s are not comparable to any [`Float`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $\Gamma(x) = \int_0^\infty t^{x-1} e^{-t}\,dt$ for $x > 0$, and the recurrence $\Gamma(x) =
    /// \Gamma(x+1)/x$ extends it to every real $x$ other than $0, -1, -2, \ldots$, where it has
    /// poles. Its reciprocal is defined for every real $x$, and is 0 at those poles.
    ///
    /// If the value is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = 1/\Gamma(x)+\varepsilon.
    /// $$
    /// - If $1/\Gamma(x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $1/\Gamma(x)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |1/\Gamma(x)|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p)=\text{NaN}$
    /// - $f(\infty,p)=0.0$
    /// - $f(-\infty,p)=\text{NaN}$
    /// - $f(\pm0.0,p)=\pm0.0$
    /// - $f(x,p)=0.0$ if $x$ is a negative integer
    ///
    /// Overflow and underflow:
    /// - If $f(x,p)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,p)\leq -2^{2^{30}-1}$, $-\infty$ is returned instead.
    /// - If $0<f(x,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// Overflow happens for arguments below about $-4.5\times10^7$ that are not integers, and
    /// underflow for arguments above about $4.5\times10^7$. The negative [`Float`]s closest to
    /// zero, $-2^{-2^{30}}$ at any precision, underflow too: there $1/\Gamma(x)$ is slightly closer
    /// to zero than $x$.
    ///
    /// The result is correctly rounded. Finding it takes $1/\Gamma(x)$ not to be a rounding
    /// boundary, which is a number with at most `prec + 1` significant bits. This is known when $x$
    /// is an integer, and when it is another multiple of $1/4$ (then $\Gamma(x)$ is irrational);
    /// nothing suggests that it ever fails, but it has not been proven for every $x$, and this
    /// function would not terminate if it did fail.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::reciprocal_gamma_prec_round_ref`] instead. If you know that your target precision
    /// is the precision of the input, consider using `(&Float).reciprocal_gamma()` instead.
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
    /// let (g, o) = Float::one_half_prec(100).reciprocal_gamma_prec_ref(5);
    /// assert_eq!(g.to_string(), "0.562");
    /// assert_eq!(o, Less);
    ///
    /// let (g, o) = Float::one_half_prec(100).reciprocal_gamma_prec_ref(20);
    /// assert_eq!(g.to_string(), "0.56418991");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn reciprocal_gamma_prec_ref(&self, prec: u64) -> (Self, Ordering) {
        self.reciprocal_gamma_prec_round_ref(prec, Nearest)
    }

    /// Computes $1/\Gamma(x)$, the reciprocal gamma function, of a [`Float`], rounding the result
    /// with the specified rounding mode. The [`Float`] is taken by value. An [`Ordering`] is also
    /// returned, indicating whether the rounded value is less than, equal to, or greater than the
    /// exact value. Although `NaN`s are not comparable to any [`Float`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// $\Gamma(x) = \int_0^\infty t^{x-1} e^{-t}\,dt$ for $x > 0$, and the recurrence $\Gamma(x) =
    /// \Gamma(x+1)/x$ extends it to every real $x$ other than $0, -1, -2, \ldots$, where it has
    /// poles. Its reciprocal is defined for every real $x$, and is 0 at those poles.
    ///
    /// The precision of the output is the precision of the input. See [`RoundingMode`] for a
    /// description of the possible rounding modes.
    ///
    /// $$
    /// f(x,m) = 1/\Gamma(x)+\varepsilon.
    /// $$
    /// - If $1/\Gamma(x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $1/\Gamma(x)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |1/\Gamma(x)|\rfloor-p+1}$, where $p$ is the precision of the input.
    /// - If $1/\Gamma(x)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |1/\Gamma(x)|\rfloor-p}$, where $p$ is the precision of the input.
    ///
    /// If the output has a precision, it is the precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN},m)=\text{NaN}$
    /// - $f(\infty,m)=0.0$
    /// - $f(-\infty,m)=\text{NaN}$
    /// - $f(\pm0.0,m)=\pm0.0$
    /// - $f(x,m)=0.0$ if $x$ is a negative integer
    ///
    /// See the [`Float::reciprocal_gamma_prec_round`] documentation for information on overflow and
    /// underflow, and on termination.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::reciprocal_gamma_prec_round`] instead. If you know you'll be using the `Nearest`
    /// rounding mode, consider using [`Float::reciprocal_gamma`] instead.
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
    /// precision (it can only when it is zero or `NaN`, or when $x$ is 1, 2, or 3).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (g, o) = Float::one_half_prec(100).reciprocal_gamma_round(Floor);
    /// assert_eq!(g.to_string(), "0.56418958354775628694807945156018");
    /// assert_eq!(o, Less);
    ///
    /// let (g, o) = Float::one_half_prec(100).reciprocal_gamma_round(Ceiling);
    /// assert_eq!(g.to_string(), "0.56418958354775628694807945156097");
    /// assert_eq!(o, Greater);
    ///
    /// let (g, o) = Float::one_half_prec(100).reciprocal_gamma_round(Nearest);
    /// assert_eq!(g.to_string(), "0.56418958354775628694807945156097");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn reciprocal_gamma_round(self, rm: RoundingMode) -> (Self, Ordering) {
        let prec = self.significant_bits();
        self.reciprocal_gamma_prec_round(prec, rm)
    }

    /// Computes $1/\Gamma(x)$, the reciprocal gamma function, of a [`Float`], rounding the result
    /// with the specified rounding mode. The [`Float`] is taken by reference. An [`Ordering`] is
    /// also returned, indicating whether the rounded value is less than, equal to, or greater than
    /// the exact value. Although `NaN`s are not comparable to any [`Float`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// $\Gamma(x) = \int_0^\infty t^{x-1} e^{-t}\,dt$ for $x > 0$, and the recurrence $\Gamma(x) =
    /// \Gamma(x+1)/x$ extends it to every real $x$ other than $0, -1, -2, \ldots$, where it has
    /// poles. Its reciprocal is defined for every real $x$, and is 0 at those poles.
    ///
    /// The precision of the output is the precision of the input. See [`RoundingMode`] for a
    /// description of the possible rounding modes.
    ///
    /// $$
    /// f(x,m) = 1/\Gamma(x)+\varepsilon.
    /// $$
    /// - If $1/\Gamma(x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $1/\Gamma(x)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |1/\Gamma(x)|\rfloor-p+1}$, where $p$ is the precision of the input.
    /// - If $1/\Gamma(x)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |1/\Gamma(x)|\rfloor-p}$, where $p$ is the precision of the input.
    ///
    /// If the output has a precision, it is the precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN},m)=\text{NaN}$
    /// - $f(\infty,m)=0.0$
    /// - $f(-\infty,m)=\text{NaN}$
    /// - $f(\pm0.0,m)=\pm0.0$
    /// - $f(x,m)=0.0$ if $x$ is a negative integer
    ///
    /// See the [`Float::reciprocal_gamma_prec_round`] documentation for information on overflow and
    /// underflow, and on termination.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::reciprocal_gamma_prec_round_ref`] instead. If you know you'll be using the
    /// `Nearest` rounding mode, consider using `(&Float).reciprocal_gamma()` instead.
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
    /// precision (it can only when it is zero or `NaN`, or when $x$ is 1, 2, or 3).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (g, o) = Float::one_half_prec(100).reciprocal_gamma_round_ref(Floor);
    /// assert_eq!(g.to_string(), "0.56418958354775628694807945156018");
    /// assert_eq!(o, Less);
    ///
    /// let (g, o) = Float::one_half_prec(100).reciprocal_gamma_round_ref(Ceiling);
    /// assert_eq!(g.to_string(), "0.56418958354775628694807945156097");
    /// assert_eq!(o, Greater);
    ///
    /// let (g, o) = Float::one_half_prec(100).reciprocal_gamma_round_ref(Nearest);
    /// assert_eq!(g.to_string(), "0.56418958354775628694807945156097");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn reciprocal_gamma_round_ref(&self, rm: RoundingMode) -> (Self, Ordering) {
        self.reciprocal_gamma_prec_round_ref(self.significant_bits(), rm)
    }

    /// Computes $1/\Gamma(x)$, the reciprocal gamma function, of a [`Float`] in place, rounding the
    /// result to the specified precision and with the specified rounding mode. An [`Ordering`] is
    /// returned, indicating whether the rounded value is less than, equal to, or greater than the
    /// exact value. Although `NaN`s are not comparable to any [`Float`], whenever this function
    /// sets the [`Float`] to `NaN` it also returns `Equal`.
    ///
    /// $\Gamma(x) = \int_0^\infty t^{x-1} e^{-t}\,dt$ for $x > 0$, and the recurrence $\Gamma(x) =
    /// \Gamma(x+1)/x$ extends it to every real $x$ other than $0, -1, -2, \ldots$, where it has
    /// poles. Its reciprocal is defined for every real $x$, and is 0 at those poles.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets 1/\Gamma(x)+\varepsilon.
    /// $$
    /// - If $1/\Gamma(x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $1/\Gamma(x)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |1/\Gamma(x)|\rfloor-p+1}$.
    /// - If $1/\Gamma(x)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |1/\Gamma(x)|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// See the [`Float::reciprocal_gamma_prec_round`] documentation for information on special
    /// cases, overflow and underflow, and termination.
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`Float::reciprocal_gamma_prec_assign`] instead. If you know that your target precision is
    /// the precision of the input, consider using [`Float::reciprocal_gamma_round_assign`] instead.
    /// If both of these things are true, consider using [`Float::reciprocal_gamma_assign`] instead.
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
    /// with the given precision (it can only when it is zero or `NaN`, or when $x$ is 1, 2, or 3).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::one_half_prec(100);
    /// assert_eq!(x.reciprocal_gamma_prec_round_assign(5, Floor), Less);
    /// assert_eq!(x.to_string(), "0.562");
    ///
    /// let mut x = Float::one_half_prec(100);
    /// assert_eq!(x.reciprocal_gamma_prec_round_assign(5, Ceiling), Greater);
    /// assert_eq!(x.to_string(), "0.594");
    ///
    /// let mut x = Float::one_half_prec(100);
    /// assert_eq!(x.reciprocal_gamma_prec_round_assign(5, Nearest), Less);
    /// assert_eq!(x.to_string(), "0.562");
    ///
    /// let mut x = Float::one_half_prec(100);
    /// assert_eq!(x.reciprocal_gamma_prec_round_assign(20, Floor), Less);
    /// assert_eq!(x.to_string(), "0.56418896");
    ///
    /// let mut x = Float::one_half_prec(100);
    /// assert_eq!(x.reciprocal_gamma_prec_round_assign(20, Ceiling), Greater);
    /// assert_eq!(x.to_string(), "0.56418991");
    ///
    /// let mut x = Float::one_half_prec(100);
    /// assert_eq!(x.reciprocal_gamma_prec_round_assign(20, Nearest), Greater);
    /// assert_eq!(x.to_string(), "0.56418991");
    /// ```
    #[inline]
    pub fn reciprocal_gamma_prec_round_assign(&mut self, prec: u64, rm: RoundingMode) -> Ordering {
        let (result, o) = core::mem::take(self).reciprocal_gamma_prec_round(prec, rm);
        *self = result;
        o
    }

    /// Computes $1/\Gamma(x)$, the reciprocal gamma function, of a [`Float`] in place, rounding the
    /// result to the nearest value of the specified precision. An [`Ordering`] is returned,
    /// indicating whether the rounded value is less than, equal to, or greater than the exact
    /// value. Although `NaN`s are not comparable to any [`Float`], whenever this function sets the
    /// [`Float`] to `NaN` it also returns `Equal`.
    ///
    /// $\Gamma(x) = \int_0^\infty t^{x-1} e^{-t}\,dt$ for $x > 0$, and the recurrence $\Gamma(x) =
    /// \Gamma(x+1)/x$ extends it to every real $x$ other than $0, -1, -2, \ldots$, where it has
    /// poles. Its reciprocal is defined for every real $x$, and is 0 at those poles.
    ///
    /// If the value is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// x \gets 1/\Gamma(x)+\varepsilon.
    /// $$
    /// - If $1/\Gamma(x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $1/\Gamma(x)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |1/\Gamma(x)|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// See the [`Float::reciprocal_gamma_prec_round`] documentation for information on special
    /// cases, overflow and underflow, and termination.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::reciprocal_gamma_prec_round_assign`] instead. If you know that your target
    /// precision is the precision of the input, consider using [`Float::reciprocal_gamma_assign`]
    /// instead.
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
    /// assert_eq!(x.reciprocal_gamma_prec_assign(5), Less);
    /// assert_eq!(x.to_string(), "0.562");
    ///
    /// let mut x = Float::one_half_prec(100);
    /// assert_eq!(x.reciprocal_gamma_prec_assign(20), Greater);
    /// assert_eq!(x.to_string(), "0.56418991");
    /// ```
    #[inline]
    pub fn reciprocal_gamma_prec_assign(&mut self, prec: u64) -> Ordering {
        self.reciprocal_gamma_prec_round_assign(prec, Nearest)
    }

    /// Computes $1/\Gamma(x)$, the reciprocal gamma function, of a [`Float`] in place, rounding the
    /// result with the specified rounding mode. An [`Ordering`] is returned, indicating whether the
    /// rounded value is less than, equal to, or greater than the exact value. Although `NaN`s are
    /// not comparable to any [`Float`], whenever this function sets the [`Float`] to `NaN` it also
    /// returns `Equal`.
    ///
    /// $\Gamma(x) = \int_0^\infty t^{x-1} e^{-t}\,dt$ for $x > 0$, and the recurrence $\Gamma(x) =
    /// \Gamma(x+1)/x$ extends it to every real $x$ other than $0, -1, -2, \ldots$, where it has
    /// poles. Its reciprocal is defined for every real $x$, and is 0 at those poles.
    ///
    /// The precision of the output is the precision of the input. See [`RoundingMode`] for a
    /// description of the possible rounding modes.
    ///
    /// $$
    /// x \gets 1/\Gamma(x)+\varepsilon.
    /// $$
    /// - If $1/\Gamma(x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $1/\Gamma(x)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |1/\Gamma(x)|\rfloor-p+1}$, where $p$ is the precision of the input.
    /// - If $1/\Gamma(x)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |1/\Gamma(x)|\rfloor-p}$, where $p$ is the precision of the input.
    ///
    /// If the output has a precision, it is the precision of the input.
    ///
    /// See the [`Float::reciprocal_gamma_prec_round`] documentation for information on special
    /// cases, overflow and underflow, and termination.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::reciprocal_gamma_prec_round_assign`] instead. If you know you'll be using the
    /// `Nearest` rounding mode, consider using [`Float::reciprocal_gamma_assign`] instead.
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
    /// precision (it can only when it is zero or `NaN`, or when $x$ is 1, 2, or 3).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::one_half_prec(100);
    /// assert_eq!(x.reciprocal_gamma_round_assign(Floor), Less);
    /// assert_eq!(x.to_string(), "0.56418958354775628694807945156018");
    ///
    /// let mut x = Float::one_half_prec(100);
    /// assert_eq!(x.reciprocal_gamma_round_assign(Ceiling), Greater);
    /// assert_eq!(x.to_string(), "0.56418958354775628694807945156097");
    ///
    /// let mut x = Float::one_half_prec(100);
    /// assert_eq!(x.reciprocal_gamma_round_assign(Nearest), Greater);
    /// assert_eq!(x.to_string(), "0.56418958354775628694807945156097");
    /// ```
    #[inline]
    pub fn reciprocal_gamma_round_assign(&mut self, rm: RoundingMode) -> Ordering {
        let prec = self.significant_bits();
        self.reciprocal_gamma_prec_round_assign(prec, rm)
    }

    /// Computes $1/\Gamma(x)$, the reciprocal gamma function, of a [`Rational`], rounding the
    /// result to the specified precision and with the specified rounding mode and returning the
    /// result as a [`Float`]. The [`Rational`] is taken by value. An [`Ordering`] is also returned,
    /// indicating whether the rounded value is less than, equal to, or greater than the exact
    /// value. Although `NaN`s are not comparable to any [`Float`], whenever this function returns a
    /// `NaN` it also returns `Equal`.
    ///
    /// $\Gamma(x) = \int_0^\infty t^{x-1} e^{-t}\,dt$ for $x > 0$, and the recurrence $\Gamma(x) =
    /// \Gamma(x+1)/x$ extends it to every real $x$ other than $0, -1, -2, \ldots$, where it has
    /// poles. Its reciprocal is defined for every real $x$, and is 0 at those poles.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = 1/\Gamma(x)+\varepsilon.
    /// $$
    /// - If $1/\Gamma(x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $1/\Gamma(x)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |1/\Gamma(x)|\rfloor-p+1}$.
    /// - If $1/\Gamma(x)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |1/\Gamma(x)|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(x,p,m)=0.0$ if $x$ is 0 or a negative integer
    ///
    /// Overflow and underflow:
    /// - If $f(x,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$ is
    ///   returned instead.
    /// - If $f(x,p,m)\leq -2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is
    ///   returned instead.
    /// - If $f(x,p,m)\leq -2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`, $-(1-(1/2)^p)2^{2^{30}-1}$
    ///   is returned instead.
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
    /// Overflow happens for arguments below about $-4.5\times10^7$ that are not integers, and
    /// underflow for arguments above about $4.5\times10^7$ and for arguments of magnitude less than
    /// about $2^{-2^{30}}$.
    ///
    /// The result is correctly rounded. Finding it takes $1/\Gamma(x)$ not to be a rounding
    /// boundary, which is a number with at most `prec + 1` significant bits. This is known when $x$
    /// is an integer, and when its denominator is 2, 3, 4, or 6 (then $\Gamma(x)$ is irrational);
    /// nothing suggests that it ever fails, but it has not been proven for every rational $x$, and
    /// this function would not terminate if it did fail.
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`Float::reciprocal_gamma_rational_prec`] instead.
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
    /// with the given precision (it can only when it is zero or `NaN`, or when $x$ is 1, 2, or 3).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (g, o) =
    ///     Float::reciprocal_gamma_rational_prec_round(Rational::from_unsigneds(1u8, 3), 5, Floor);
    /// assert_eq!(g.to_string(), "0.359");
    /// assert_eq!(o, Less);
    ///
    /// let (g, o) = Float::reciprocal_gamma_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(g.to_string(), "0.375");
    /// assert_eq!(o, Greater);
    ///
    /// let (g, o) = Float::reciprocal_gamma_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(g.to_string(), "0.375");
    /// assert_eq!(o, Greater);
    ///
    /// let (g, o) = Float::reciprocal_gamma_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(g.to_string(), "0.37328196");
    /// assert_eq!(o, Less);
    ///
    /// let (g, o) = Float::reciprocal_gamma_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(g.to_string(), "0.37328243");
    /// assert_eq!(o, Greater);
    ///
    /// let (g, o) = Float::reciprocal_gamma_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(g.to_string(), "0.37328196");
    /// assert_eq!(o, Less);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn reciprocal_gamma_rational_prec_round(
        x: Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        gamma_rational_prec_round_helper(true, &x, prec, rm)
    }

    /// Computes $1/\Gamma(x)$, the reciprocal gamma function, of a [`Rational`], rounding the
    /// result to the specified precision and with the specified rounding mode and returning the
    /// result as a [`Float`]. The [`Rational`] is taken by reference. An [`Ordering`] is also
    /// returned, indicating whether the rounded value is less than, equal to, or greater than the
    /// exact value. Although `NaN`s are not comparable to any [`Float`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// $\Gamma(x) = \int_0^\infty t^{x-1} e^{-t}\,dt$ for $x > 0$, and the recurrence $\Gamma(x) =
    /// \Gamma(x+1)/x$ extends it to every real $x$ other than $0, -1, -2, \ldots$, where it has
    /// poles. Its reciprocal is defined for every real $x$, and is 0 at those poles.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = 1/\Gamma(x)+\varepsilon.
    /// $$
    /// - If $1/\Gamma(x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $1/\Gamma(x)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |1/\Gamma(x)|\rfloor-p+1}$.
    /// - If $1/\Gamma(x)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |1/\Gamma(x)|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(x,p,m)=0.0$ if $x$ is 0 or a negative integer
    ///
    /// Overflow and underflow:
    /// - If $f(x,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$ is
    ///   returned instead.
    /// - If $f(x,p,m)\leq -2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is
    ///   returned instead.
    /// - If $f(x,p,m)\leq -2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`, $-(1-(1/2)^p)2^{2^{30}-1}$
    ///   is returned instead.
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
    /// Overflow happens for arguments below about $-4.5\times10^7$ that are not integers, and
    /// underflow for arguments above about $4.5\times10^7$ and for arguments of magnitude less than
    /// about $2^{-2^{30}}$.
    ///
    /// The result is correctly rounded. Finding it takes $1/\Gamma(x)$ not to be a rounding
    /// boundary, which is a number with at most `prec + 1` significant bits. This is known when $x$
    /// is an integer, and when its denominator is 2, 3, 4, or 6 (then $\Gamma(x)$ is irrational);
    /// nothing suggests that it ever fails, but it has not been proven for every rational $x$, and
    /// this function would not terminate if it did fail.
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`Float::reciprocal_gamma_rational_prec_ref`] instead.
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
    /// with the given precision (it can only when it is zero or `NaN`, or when $x$ is 1, 2, or 3).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (g, o) = Float::reciprocal_gamma_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(g.to_string(), "0.359");
    /// assert_eq!(o, Less);
    ///
    /// let (g, o) = Float::reciprocal_gamma_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(g.to_string(), "0.375");
    /// assert_eq!(o, Greater);
    ///
    /// let (g, o) = Float::reciprocal_gamma_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(g.to_string(), "0.375");
    /// assert_eq!(o, Greater);
    ///
    /// let (g, o) = Float::reciprocal_gamma_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(g.to_string(), "0.37328196");
    /// assert_eq!(o, Less);
    ///
    /// let (g, o) = Float::reciprocal_gamma_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(g.to_string(), "0.37328243");
    /// assert_eq!(o, Greater);
    ///
    /// let (g, o) = Float::reciprocal_gamma_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(g.to_string(), "0.37328196");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn reciprocal_gamma_rational_prec_round_ref(
        x: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        gamma_rational_prec_round_helper(true, x, prec, rm)
    }

    /// Computes $1/\Gamma(x)$, the reciprocal gamma function, of a [`Rational`], rounding the
    /// result to the nearest value of the specified precision and returning the result as a
    /// [`Float`]. The [`Rational`] is taken by value. An [`Ordering`] is also returned, indicating
    /// whether the rounded value is less than, equal to, or greater than the exact value. Although
    /// `NaN`s are not comparable to any [`Float`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// $\Gamma(x) = \int_0^\infty t^{x-1} e^{-t}\,dt$ for $x > 0$, and the recurrence $\Gamma(x) =
    /// \Gamma(x+1)/x$ extends it to every real $x$ other than $0, -1, -2, \ldots$, where it has
    /// poles. Its reciprocal is defined for every real $x$, and is 0 at those poles.
    ///
    /// If the value is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = 1/\Gamma(x)+\varepsilon.
    /// $$
    /// - If $1/\Gamma(x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $1/\Gamma(x)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |1/\Gamma(x)|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(x,p)=0.0$ if $x$ is 0 or a negative integer
    ///
    /// Overflow and underflow:
    /// - If $f(x,p)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,p)\leq -2^{2^{30}-1}$, $-\infty$ is returned instead.
    /// - If $0<f(x,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// Overflow happens for arguments below about $-4.5\times10^7$ that are not integers, and
    /// underflow for arguments above about $4.5\times10^7$ and for arguments of magnitude less than
    /// about $2^{-2^{30}}$.
    ///
    /// The result is correctly rounded. Finding it takes $1/\Gamma(x)$ not to be a rounding
    /// boundary, which is a number with at most `prec + 1` significant bits. This is known when $x$
    /// is an integer, and when its denominator is 2, 3, 4, or 6 (then $\Gamma(x)$ is irrational);
    /// nothing suggests that it ever fails, but it has not been proven for every rational $x$, and
    /// this function would not terminate if it did fail.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::reciprocal_gamma_rational_prec_round`] instead.
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
    /// let (g, o) = Float::reciprocal_gamma_rational_prec(Rational::from_unsigneds(1u8, 3), 5);
    /// assert_eq!(g.to_string(), "0.375");
    /// assert_eq!(o, Greater);
    ///
    /// let (g, o) = Float::reciprocal_gamma_rational_prec(Rational::from_unsigneds(1u8, 3), 20);
    /// assert_eq!(g.to_string(), "0.37328196");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn reciprocal_gamma_rational_prec(x: Rational, prec: u64) -> (Self, Ordering) {
        Self::reciprocal_gamma_rational_prec_round(x, prec, Nearest)
    }

    /// Computes $1/\Gamma(x)$, the reciprocal gamma function, of a [`Rational`], rounding the
    /// result to the nearest value of the specified precision and returning the result as a
    /// [`Float`]. The [`Rational`] is taken by reference. An [`Ordering`] is also returned,
    /// indicating whether the rounded value is less than, equal to, or greater than the exact
    /// value. Although `NaN`s are not comparable to any [`Float`], whenever this function returns a
    /// `NaN` it also returns `Equal`.
    ///
    /// $\Gamma(x) = \int_0^\infty t^{x-1} e^{-t}\,dt$ for $x > 0$, and the recurrence $\Gamma(x) =
    /// \Gamma(x+1)/x$ extends it to every real $x$ other than $0, -1, -2, \ldots$, where it has
    /// poles. Its reciprocal is defined for every real $x$, and is 0 at those poles.
    ///
    /// If the value is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = 1/\Gamma(x)+\varepsilon.
    /// $$
    /// - If $1/\Gamma(x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $1/\Gamma(x)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |1/\Gamma(x)|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(x,p)=0.0$ if $x$ is 0 or a negative integer
    ///
    /// Overflow and underflow:
    /// - If $f(x,p)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,p)\leq -2^{2^{30}-1}$, $-\infty$ is returned instead.
    /// - If $0<f(x,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// Overflow happens for arguments below about $-4.5\times10^7$ that are not integers, and
    /// underflow for arguments above about $4.5\times10^7$ and for arguments of magnitude less than
    /// about $2^{-2^{30}}$.
    ///
    /// The result is correctly rounded. Finding it takes $1/\Gamma(x)$ not to be a rounding
    /// boundary, which is a number with at most `prec + 1` significant bits. This is known when $x$
    /// is an integer, and when its denominator is 2, 3, 4, or 6 (then $\Gamma(x)$ is irrational);
    /// nothing suggests that it ever fails, but it has not been proven for every rational $x$, and
    /// this function would not terminate if it did fail.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::reciprocal_gamma_rational_prec_round_ref`] instead.
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
    /// let (g, o) =
    ///     Float::reciprocal_gamma_rational_prec_ref(&Rational::from_unsigneds(1u8, 3), 5);
    /// assert_eq!(g.to_string(), "0.375");
    /// assert_eq!(o, Greater);
    ///
    /// let (g, o) =
    ///     Float::reciprocal_gamma_rational_prec_ref(&Rational::from_unsigneds(1u8, 3), 20);
    /// assert_eq!(g.to_string(), "0.37328196");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn reciprocal_gamma_rational_prec_ref(x: &Rational, prec: u64) -> (Self, Ordering) {
        Self::reciprocal_gamma_rational_prec_round_ref(x, prec, Nearest)
    }
}

impl ReciprocalGamma for Float {
    type Output = Self;

    /// Computes $1/\Gamma(x)$, the reciprocal gamma function, of a [`Float`], taking it by value.
    ///
    /// $\Gamma(x) = \int_0^\infty t^{x-1} e^{-t}\,dt$ for $x > 0$, and the recurrence $\Gamma(x) =
    /// \Gamma(x+1)/x$ extends it to every real $x$ other than $0, -1, -2, \ldots$, where it has
    /// poles. Its reciprocal is defined for every real $x$, and is 0 at those poles.
    ///
    /// If the output has a precision, it is the precision of the input. If the value is equidistant
    /// from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s in its binary
    /// expansion is chosen. See [`RoundingMode`] for a description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x) = 1/\Gamma(x)+\varepsilon.
    /// $$
    /// - If $1/\Gamma(x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $1/\Gamma(x)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |1/\Gamma(x)|\rfloor-p}$, where $p$ is the precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN})=\text{NaN}$
    /// - $f(\infty)=0.0$
    /// - $f(-\infty)=\text{NaN}$
    /// - $f(\pm0.0)=\pm0.0$
    /// - $f(x)=0.0$ if $x$ is a negative integer
    ///
    /// See the [`Float::reciprocal_gamma_prec_round`] documentation for information on overflow and
    /// underflow, and on termination.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::reciprocal_gamma_round`] instead. If you want to specify the output precision,
    /// consider using [`Float::reciprocal_gamma_prec`]. If you want both of these things, consider
    /// using [`Float::reciprocal_gamma_prec_round`].
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
    /// use malachite_base::num::basic::traits::{Infinity, NaN, NegativeOne, Zero};
    /// use malachite_base::num::gamma::traits::ReciprocalGamma;
    /// use malachite_float::Float;
    ///
    /// assert!(Float::NAN.reciprocal_gamma().is_nan());
    /// assert_eq!(Float::INFINITY.reciprocal_gamma(), Float::ZERO);
    /// assert_eq!(Float::NEGATIVE_ONE.reciprocal_gamma(), Float::ZERO);
    /// assert_eq!(
    ///     Float::one_half_prec(100).reciprocal_gamma().to_string(),
    ///     "0.56418958354775628694807945156097"
    /// );
    /// ```
    #[inline]
    fn reciprocal_gamma(self) -> Self {
        let prec = self.significant_bits();
        self.reciprocal_gamma_prec(prec).0
    }
}

impl ReciprocalGamma for &Float {
    type Output = Float;

    /// Computes $1/\Gamma(x)$, the reciprocal gamma function, of a [`Float`], taking it by
    /// reference.
    ///
    /// $\Gamma(x) = \int_0^\infty t^{x-1} e^{-t}\,dt$ for $x > 0$, and the recurrence $\Gamma(x) =
    /// \Gamma(x+1)/x$ extends it to every real $x$ other than $0, -1, -2, \ldots$, where it has
    /// poles. Its reciprocal is defined for every real $x$, and is 0 at those poles.
    ///
    /// If the output has a precision, it is the precision of the input. If the value is equidistant
    /// from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s in its binary
    /// expansion is chosen. See [`RoundingMode`] for a description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x) = 1/\Gamma(x)+\varepsilon.
    /// $$
    /// - If $1/\Gamma(x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $1/\Gamma(x)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |1/\Gamma(x)|\rfloor-p}$, where $p$ is the precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN})=\text{NaN}$
    /// - $f(\infty)=0.0$
    /// - $f(-\infty)=\text{NaN}$
    /// - $f(\pm0.0)=\pm0.0$
    /// - $f(x)=0.0$ if $x$ is a negative integer
    ///
    /// See the [`Float::reciprocal_gamma_prec_round`] documentation for information on overflow and
    /// underflow, and on termination.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::reciprocal_gamma_round_ref`] instead. If you want to specify the output precision,
    /// consider using [`Float::reciprocal_gamma_prec_ref`]. If you want both of these things,
    /// consider using [`Float::reciprocal_gamma_prec_round_ref`].
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
    /// use malachite_base::num::basic::traits::{Infinity, NaN, NegativeOne, Zero};
    /// use malachite_base::num::gamma::traits::ReciprocalGamma;
    /// use malachite_float::Float;
    ///
    /// assert!((&Float::NAN).reciprocal_gamma().is_nan());
    /// assert_eq!((&Float::INFINITY).reciprocal_gamma(), Float::ZERO);
    /// assert_eq!((&Float::NEGATIVE_ONE).reciprocal_gamma(), Float::ZERO);
    /// assert_eq!(
    ///     (&Float::one_half_prec(100)).reciprocal_gamma().to_string(),
    ///     "0.56418958354775628694807945156097"
    /// );
    /// ```
    #[inline]
    fn reciprocal_gamma(self) -> Float {
        self.reciprocal_gamma_prec_ref(self.significant_bits()).0
    }
}

impl ReciprocalGammaAssign for Float {
    /// Computes $1/\Gamma(x)$, the reciprocal gamma function, of a [`Float`] in place.
    ///
    /// $\Gamma(x) = \int_0^\infty t^{x-1} e^{-t}\,dt$ for $x > 0$, and the recurrence $\Gamma(x) =
    /// \Gamma(x+1)/x$ extends it to every real $x$ other than $0, -1, -2, \ldots$, where it has
    /// poles. Its reciprocal is defined for every real $x$, and is 0 at those poles.
    ///
    /// If the output has a precision, it is the precision of the input. If the value is equidistant
    /// from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s in its binary
    /// expansion is chosen. See [`RoundingMode`] for a description of the `Nearest` rounding mode.
    ///
    /// $$
    /// x \gets 1/\Gamma(x)+\varepsilon.
    /// $$
    /// - If $1/\Gamma(x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $1/\Gamma(x)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |1/\Gamma(x)|\rfloor-p}$, where $p$ is the precision of the input.
    ///
    /// See the [`Float::reciprocal_gamma_prec_round`] documentation for information on special
    /// cases, overflow and underflow, and termination.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::reciprocal_gamma_round_assign`] instead. If you want to specify the output
    /// precision, consider using [`Float::reciprocal_gamma_prec_assign`]. If you want both of these
    /// things, consider using [`Float::reciprocal_gamma_prec_round_assign`].
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
    /// use malachite_base::num::basic::traits::{Infinity, NaN, NegativeOne, Zero};
    /// use malachite_base::num::gamma::traits::ReciprocalGammaAssign;
    /// use malachite_float::Float;
    ///
    /// let mut x = Float::NAN;
    /// x.reciprocal_gamma_assign();
    /// assert!(x.is_nan());
    ///
    /// let mut x = Float::INFINITY;
    /// x.reciprocal_gamma_assign();
    /// assert_eq!(x, Float::ZERO);
    ///
    /// let mut x = Float::NEGATIVE_ONE;
    /// x.reciprocal_gamma_assign();
    /// assert_eq!(x, Float::ZERO);
    ///
    /// let mut x = Float::one_half_prec(100);
    /// x.reciprocal_gamma_assign();
    /// assert_eq!(x.to_string(), "0.56418958354775628694807945156097");
    /// ```
    #[inline]
    fn reciprocal_gamma_assign(&mut self) {
        let prec = self.significant_bits();
        self.reciprocal_gamma_prec_assign(prec);
    }
}

/// Computes $1/\Gamma(x)$, the reciprocal gamma function, of a primitive float.
///
/// $\Gamma(x) = \int_0^\infty t^{x-1} e^{-t}\,dt$ for $x > 0$, and the recurrence $\Gamma(x) =
/// \Gamma(x+1)/x$ extends it to every real $x$ other than $0, -1, -2, \ldots$, where it has poles.
/// Its reciprocal is defined for every real $x$, and is 0 at those poles.
///
/// $$
/// f(x) = 1/\Gamma(x)+\varepsilon.
/// $$
/// - If $1/\Gamma(x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
/// - If $1/\Gamma(x)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
///   |1/\Gamma(x)|\rfloor-p}$, where $p$ is the precision of the output (typically 24 if `T` is a
///   [`f32`] and 53 if `T` is a [`f64`], but less if the output is subnormal).
///
/// Special cases:
/// - $f(\text{NaN})=\text{NaN}$
/// - $f(\infty)=0.0$
/// - $f(-\infty)=\text{NaN}$
/// - $f(\pm0.0)=\pm0.0$
/// - $f(x)=0.0$ if $x$ is a negative integer
///
/// The result is $\pm\infty$ when $1/\Gamma(x)$ is too large for `T`, which happens for a negative
/// $x$ of large enough magnitude that is not an integer. It is $0.0$ when $1/\Gamma(x)$ is too
/// small: for $x$ above about 39.3 (`f32`) or 178.5 (`f64`).
///
/// See the [`Float::reciprocal_gamma_prec_round`] documentation for information on termination.
///
/// # Worst-case complexity
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use malachite_base::num::float::NiceFloat;
/// use malachite_float::float::gamma::reciprocal_gamma::primitive_float_reciprocal_gamma;
///
/// assert!(primitive_float_reciprocal_gamma(f32::NAN).is_nan());
/// assert_eq!(
///     NiceFloat(primitive_float_reciprocal_gamma(f32::INFINITY)),
///     NiceFloat(0.0)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_reciprocal_gamma(0.0f32)),
///     NiceFloat(0.0)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_reciprocal_gamma(-0.0f32)),
///     NiceFloat(-0.0)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_reciprocal_gamma(0.5f32)),
///     NiceFloat(0.5641896)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_reciprocal_gamma(5.0f32)),
///     NiceFloat(0.041666668)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_reciprocal_gamma(-1.0f32)),
///     NiceFloat(0.0)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_reciprocal_gamma(-0.5f32)),
///     NiceFloat(-0.2820948)
/// );
/// ```
#[inline]
#[allow(clippy::type_repetition_in_bounds)]
pub fn primitive_float_reciprocal_gamma<T: PrimitiveFloat>(x: T) -> T
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    emulate_float_to_float_fn(Float::reciprocal_gamma_prec, x)
}

/// Computes $1/\Gamma(x)$, the reciprocal gamma function, of a [`Rational`], returning the result
/// as a primitive float.
///
/// $\Gamma(x) = \int_0^\infty t^{x-1} e^{-t}\,dt$ for $x > 0$, and the recurrence $\Gamma(x) =
/// \Gamma(x+1)/x$ extends it to every real $x$ other than $0, -1, -2, \ldots$, where it has poles.
/// Its reciprocal is defined for every real $x$, and is 0 at those poles.
///
/// $$
/// f(x) = 1/\Gamma(x)+\varepsilon.
/// $$
/// - If $1/\Gamma(x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
/// - If $1/\Gamma(x)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
///   |1/\Gamma(x)|\rfloor-p}$, where $p$ is the precision of the output (typically 24 if `T` is a
///   [`f32`] and 53 if `T` is a [`f64`], but less if the output is subnormal).
///
/// Special cases:
/// - $f(x)=0.0$ if $x$ is 0 or a negative integer
///
/// The result is $\pm\infty$ when $1/\Gamma(x)$ is too large for `T`, which happens for a negative
/// $x$ of large enough magnitude that is not an integer. It is $\pm0.0$ when $1/\Gamma(x)$ is too
/// small: for $x$ above about 39.3 (`f32`) or 178.5 (`f64`), and for an $x$ close enough to zero.
///
/// See the [`Float::reciprocal_gamma_rational_prec_round`] documentation for information on
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
/// use malachite_base::num::basic::traits::{NegativeOne, Zero};
/// use malachite_base::num::float::NiceFloat;
/// use malachite_float::float::gamma::reciprocal_gamma::primitive_float_reciprocal_gamma_rational;
/// use malachite_q::Rational;
///
/// assert_eq!(
///     NiceFloat(primitive_float_reciprocal_gamma_rational::<f64>(
///         &Rational::ZERO
///     )),
///     NiceFloat(0.0)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_reciprocal_gamma_rational::<f64>(
///         &Rational::from_unsigneds(1u8, 3)
///     )),
///     NiceFloat(0.3732821739073952)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_reciprocal_gamma_rational::<f64>(
///         &Rational::NEGATIVE_ONE
///     )),
///     NiceFloat(0.0)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_reciprocal_gamma_rational::<f64>(
///         &Rational::from(5)
///     )),
///     NiceFloat(0.041666666666666664)
/// );
/// ```
#[inline]
#[allow(clippy::type_repetition_in_bounds)]
pub fn primitive_float_reciprocal_gamma_rational<T: PrimitiveFloat>(x: &Rational) -> T
where
    Float: PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    emulate_rational_to_float_fn(Float::reciprocal_gamma_rational_prec_ref, x)
}
