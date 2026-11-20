// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::big_exponent_float::BigExponentFloat;
use crate::big_exponent_float::gamma::gamma::{gamma_helper, gamma_rational_helper};
use core::cmp::Ordering;
use core::mem::swap;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_q::Rational;

impl BigExponentFloat {
    /// Computes $1/\Gamma(x)$, the reciprocal gamma function, of a [`BigExponentFloat`], rounding
    /// the result to the specified precision and with the specified rounding mode. The
    /// [`BigExponentFloat`] is taken by value. An [`Ordering`] is also returned, indicating whether
    /// the rounded value is less than, equal to, or greater than the exact value. Although `NaN`s
    /// are not comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it
    /// also returns `Equal`.
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
    /// Special cases:
    /// - $f(\text{NaN},p,m)=\text{NaN}$
    /// - $f(\infty,p,m)=0$
    /// - $f(-\infty,p,m)=\text{NaN}$
    /// - $f(x,p,m)=0$ if $x$ is 0 or a negative integer
    ///
    /// Unlike [its `Float` counterpart](crate::Float::reciprocal_gamma_prec_round), this function
    /// never overflows or underflows.
    ///
    /// Warning: the absolute value of $\ln \Gamma(x)$ is about $|x| \ln |x|$ for a large $|x|$, so
    /// an argument with exponent $e > 0$ takes about $e$ bits of working precision beyond `prec`,
    /// and gives a result whose exponent has about $e$ bits. The time and memory needed therefore
    /// grow with the exponent, without any limit: an argument near $2^{2^{40}}$ needs more than
    /// $2^{40}$ bits, or 128 GiB.
    ///
    /// The result is correctly rounded. Finding it takes $1/\Gamma(x)$ not to be a rounding
    /// boundary, which is a number with at most `prec + 1` significant bits. This is known when $x$
    /// is an integer, and when it is another multiple of $1/4$ (then $\Gamma(x)$ is irrational);
    /// nothing suggests that it ever fails, but it has not been proven for every $x$, and this
    /// function would not terminate if it did fail.
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`BigExponentFloat::reciprocal_gamma_prec`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O((n + m)^3 \log (n + m))$
    ///
    /// $M(n, m) = O((n + m)^2 \log (n + m))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(prec, self.significant_bits())`,
    /// and $m$ is the exponent of $x$ if that is positive, and 0 otherwise.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with the given precision (it can only when it is zero or `NaN`, or when $x$ is 1, 2, or 3).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::OneHalf;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (g, o) = BigExponentFloat::ONE_HALF.reciprocal_gamma_prec_round(5, Floor);
    /// assert_eq!(g.to_string(), "0.562");
    /// assert_eq!(o, Less);
    ///
    /// let (g, o) = BigExponentFloat::ONE_HALF.reciprocal_gamma_prec_round(5, Ceiling);
    /// assert_eq!(g.to_string(), "0.594");
    /// assert_eq!(o, Greater);
    ///
    /// let (g, o) = BigExponentFloat::ONE_HALF.reciprocal_gamma_prec_round(5, Nearest);
    /// assert_eq!(g.to_string(), "0.562");
    /// assert_eq!(o, Less);
    ///
    /// let (g, o) = BigExponentFloat::ONE_HALF.reciprocal_gamma_prec_round(20, Floor);
    /// assert_eq!(g.to_string(), "0.56418896");
    /// assert_eq!(o, Less);
    ///
    /// let (g, o) = BigExponentFloat::ONE_HALF.reciprocal_gamma_prec_round(20, Ceiling);
    /// assert_eq!(g.to_string(), "0.5641899");
    /// assert_eq!(o, Greater);
    ///
    /// let (g, o) = BigExponentFloat::ONE_HALF.reciprocal_gamma_prec_round(20, Nearest);
    /// assert_eq!(g.to_string(), "0.5641899");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn reciprocal_gamma_prec_round(self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        gamma_helper(true, &self, prec, rm)
    }

    /// Computes $1/\Gamma(x)$, the reciprocal gamma function, of a [`BigExponentFloat`], rounding
    /// the result to the specified precision and with the specified rounding mode. The
    /// [`BigExponentFloat`] is taken by reference. An [`Ordering`] is also returned, indicating
    /// whether the rounded value is less than, equal to, or greater than the exact value. Although
    /// `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function returns a
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
    /// Special cases:
    /// - $f(\text{NaN},p,m)=\text{NaN}$
    /// - $f(\infty,p,m)=0$
    /// - $f(-\infty,p,m)=\text{NaN}$
    /// - $f(x,p,m)=0$ if $x$ is 0 or a negative integer
    ///
    /// Unlike [its `Float` counterpart](crate::Float::reciprocal_gamma_prec_round), this function
    /// never overflows or underflows.
    ///
    /// Warning: the absolute value of $\ln \Gamma(x)$ is about $|x| \ln |x|$ for a large $|x|$, so
    /// an argument with exponent $e > 0$ takes about $e$ bits of working precision beyond `prec`,
    /// and gives a result whose exponent has about $e$ bits. The time and memory needed therefore
    /// grow with the exponent, without any limit: an argument near $2^{2^{40}}$ needs more than
    /// $2^{40}$ bits, or 128 GiB.
    ///
    /// The result is correctly rounded. Finding it takes $1/\Gamma(x)$ not to be a rounding
    /// boundary, which is a number with at most `prec + 1` significant bits. This is known when $x$
    /// is an integer, and when it is another multiple of $1/4$ (then $\Gamma(x)$ is irrational);
    /// nothing suggests that it ever fails, but it has not been proven for every $x$, and this
    /// function would not terminate if it did fail.
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`BigExponentFloat::reciprocal_gamma_prec_ref`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O((n + m)^3 \log (n + m))$
    ///
    /// $M(n, m) = O((n + m)^2 \log (n + m))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(prec, self.significant_bits())`,
    /// and $m$ is the exponent of $x$ if that is positive, and 0 otherwise.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with the given precision (it can only when it is zero or `NaN`, or when $x$ is 1, 2, or 3).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::OneHalf;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (g, o) = BigExponentFloat::ONE_HALF.reciprocal_gamma_prec_round_ref(5, Floor);
    /// assert_eq!(g.to_string(), "0.562");
    /// assert_eq!(o, Less);
    ///
    /// let (g, o) = BigExponentFloat::ONE_HALF.reciprocal_gamma_prec_round_ref(5, Ceiling);
    /// assert_eq!(g.to_string(), "0.594");
    /// assert_eq!(o, Greater);
    ///
    /// let (g, o) = BigExponentFloat::ONE_HALF.reciprocal_gamma_prec_round_ref(5, Nearest);
    /// assert_eq!(g.to_string(), "0.562");
    /// assert_eq!(o, Less);
    ///
    /// let (g, o) = BigExponentFloat::ONE_HALF.reciprocal_gamma_prec_round_ref(20, Floor);
    /// assert_eq!(g.to_string(), "0.56418896");
    /// assert_eq!(o, Less);
    ///
    /// let (g, o) = BigExponentFloat::ONE_HALF.reciprocal_gamma_prec_round_ref(20, Ceiling);
    /// assert_eq!(g.to_string(), "0.5641899");
    /// assert_eq!(o, Greater);
    ///
    /// let (g, o) = BigExponentFloat::ONE_HALF.reciprocal_gamma_prec_round_ref(20, Nearest);
    /// assert_eq!(g.to_string(), "0.5641899");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn reciprocal_gamma_prec_round_ref(&self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        gamma_helper(true, self, prec, rm)
    }

    /// Computes $1/\Gamma(x)$, the reciprocal gamma function, of a [`BigExponentFloat`] in place,
    /// rounding the result to the specified precision and with the specified rounding mode. An
    /// [`Ordering`] is returned, indicating whether the rounded value is less than, equal to, or
    /// greater than the exact value. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function sets the [`BigExponentFloat`] to `NaN` it also
    /// returns `Equal`.
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
    /// See the [`BigExponentFloat::reciprocal_gamma_prec_round`] documentation for information on
    /// special cases, running time, and termination.
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`BigExponentFloat::reciprocal_gamma_prec_assign`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O((n + m)^3 \log (n + m))$
    ///
    /// $M(n, m) = O((n + m)^2 \log (n + m))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(prec, self.significant_bits())`,
    /// and $m$ is the exponent of $x$ if that is positive, and 0 otherwise.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with the given precision (it can only when it is zero or `NaN`, or when $x$ is 1, 2, or 3).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::OneHalf;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = BigExponentFloat::ONE_HALF;
    /// assert_eq!(x.reciprocal_gamma_prec_round_assign(5, Floor), Less);
    /// assert_eq!(x.to_string(), "0.562");
    ///
    /// let mut x = BigExponentFloat::ONE_HALF;
    /// assert_eq!(x.reciprocal_gamma_prec_round_assign(5, Ceiling), Greater);
    /// assert_eq!(x.to_string(), "0.594");
    ///
    /// let mut x = BigExponentFloat::ONE_HALF;
    /// assert_eq!(x.reciprocal_gamma_prec_round_assign(5, Nearest), Less);
    /// assert_eq!(x.to_string(), "0.562");
    ///
    /// let mut x = BigExponentFloat::ONE_HALF;
    /// assert_eq!(x.reciprocal_gamma_prec_round_assign(20, Floor), Less);
    /// assert_eq!(x.to_string(), "0.56418896");
    ///
    /// let mut x = BigExponentFloat::ONE_HALF;
    /// assert_eq!(x.reciprocal_gamma_prec_round_assign(20, Ceiling), Greater);
    /// assert_eq!(x.to_string(), "0.5641899");
    ///
    /// let mut x = BigExponentFloat::ONE_HALF;
    /// assert_eq!(x.reciprocal_gamma_prec_round_assign(20, Nearest), Greater);
    /// assert_eq!(x.to_string(), "0.5641899");
    /// ```
    #[inline]
    pub fn reciprocal_gamma_prec_round_assign(&mut self, prec: u64, rm: RoundingMode) -> Ordering {
        let (mut g, o) = gamma_helper(true, self, prec, rm);
        swap(self, &mut g);
        o
    }

    /// Computes $1/\Gamma(x)$, the reciprocal gamma function, of a [`BigExponentFloat`], rounding
    /// the result to the nearest value of the specified precision. The [`BigExponentFloat`] is
    /// taken by value. An [`Ordering`] is also returned, indicating whether the rounded value is
    /// less than, equal to, or greater than the exact value. Although `NaN`s are not comparable to
    /// any [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $\Gamma(x) = \int_0^\infty t^{x-1} e^{-t}\,dt$ for $x > 0$, and the recurrence $\Gamma(x) =
    /// \Gamma(x+1)/x$ extends it to every real $x$ other than $0, -1, -2, \ldots$, where it has
    /// poles. Its reciprocal is defined for every real $x$, and is 0 at those poles.
    ///
    /// If the value is equidistant from two [`BigExponentFloat`]s with the specified precision, the
    /// [`BigExponentFloat`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`]
    /// for a description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = 1/\Gamma(x)+\varepsilon.
    /// $$
    /// - If $1/\Gamma(x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $1/\Gamma(x)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |1/\Gamma(x)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p)=\text{NaN}$
    /// - $f(\infty,p)=0$
    /// - $f(-\infty,p)=\text{NaN}$
    /// - $f(x,p)=0$ if $x$ is 0 or a negative integer
    ///
    /// Unlike [its `Float` counterpart](crate::Float::reciprocal_gamma_prec_round), this function
    /// never overflows or underflows.
    ///
    /// Warning: the absolute value of $\ln \Gamma(x)$ is about $|x| \ln |x|$ for a large $|x|$, so
    /// an argument with exponent $e > 0$ takes about $e$ bits of working precision beyond `prec`,
    /// and gives a result whose exponent has about $e$ bits. The time and memory needed therefore
    /// grow with the exponent, without any limit: an argument near $2^{2^{40}}$ needs more than
    /// $2^{40}$ bits, or 128 GiB.
    ///
    /// The result is correctly rounded. Finding it takes $1/\Gamma(x)$ not to be a rounding
    /// boundary, which is a number with at most `prec + 1` significant bits. This is known when $x$
    /// is an integer, and when it is another multiple of $1/4$ (then $\Gamma(x)$ is irrational);
    /// nothing suggests that it ever fails, but it has not been proven for every $x$, and this
    /// function would not terminate if it did fail.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`BigExponentFloat::reciprocal_gamma_prec_round`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O((n + m)^3 \log (n + m))$
    ///
    /// $M(n, m) = O((n + m)^2 \log (n + m))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(prec, self.significant_bits())`,
    /// and $m$ is the exponent of $x$ if that is positive, and 0 otherwise.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::OneHalf;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (g, o) = BigExponentFloat::ONE_HALF.reciprocal_gamma_prec(5);
    /// assert_eq!(g.to_string(), "0.562");
    /// assert_eq!(o, Less);
    ///
    /// let (g, o) = BigExponentFloat::ONE_HALF.reciprocal_gamma_prec(20);
    /// assert_eq!(g.to_string(), "0.5641899");
    /// assert_eq!(o, Greater);
    ///
    /// let (g, o) = BigExponentFloat::from(1.0e9).reciprocal_gamma_prec(20);
    /// assert_eq!(g.to_string(), "1.0096286e-8565705514");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn reciprocal_gamma_prec(self, prec: u64) -> (Self, Ordering) {
        self.reciprocal_gamma_prec_round(prec, Nearest)
    }

    /// Computes $1/\Gamma(x)$, the reciprocal gamma function, of a [`BigExponentFloat`], rounding
    /// the result to the nearest value of the specified precision. The [`BigExponentFloat`] is
    /// taken by reference. An [`Ordering`] is also returned, indicating whether the rounded value
    /// is less than, equal to, or greater than the exact value. Although `NaN`s are not comparable
    /// to any [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $\Gamma(x) = \int_0^\infty t^{x-1} e^{-t}\,dt$ for $x > 0$, and the recurrence $\Gamma(x) =
    /// \Gamma(x+1)/x$ extends it to every real $x$ other than $0, -1, -2, \ldots$, where it has
    /// poles. Its reciprocal is defined for every real $x$, and is 0 at those poles.
    ///
    /// If the value is equidistant from two [`BigExponentFloat`]s with the specified precision, the
    /// [`BigExponentFloat`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`]
    /// for a description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = 1/\Gamma(x)+\varepsilon.
    /// $$
    /// - If $1/\Gamma(x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $1/\Gamma(x)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |1/\Gamma(x)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p)=\text{NaN}$
    /// - $f(\infty,p)=0$
    /// - $f(-\infty,p)=\text{NaN}$
    /// - $f(x,p)=0$ if $x$ is 0 or a negative integer
    ///
    /// Unlike [its `Float` counterpart](crate::Float::reciprocal_gamma_prec_round), this function
    /// never overflows or underflows.
    ///
    /// Warning: the absolute value of $\ln \Gamma(x)$ is about $|x| \ln |x|$ for a large $|x|$, so
    /// an argument with exponent $e > 0$ takes about $e$ bits of working precision beyond `prec`,
    /// and gives a result whose exponent has about $e$ bits. The time and memory needed therefore
    /// grow with the exponent, without any limit: an argument near $2^{2^{40}}$ needs more than
    /// $2^{40}$ bits, or 128 GiB.
    ///
    /// The result is correctly rounded. Finding it takes $1/\Gamma(x)$ not to be a rounding
    /// boundary, which is a number with at most `prec + 1` significant bits. This is known when $x$
    /// is an integer, and when it is another multiple of $1/4$ (then $\Gamma(x)$ is irrational);
    /// nothing suggests that it ever fails, but it has not been proven for every $x$, and this
    /// function would not terminate if it did fail.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`BigExponentFloat::reciprocal_gamma_prec_round_ref`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O((n + m)^3 \log (n + m))$
    ///
    /// $M(n, m) = O((n + m)^2 \log (n + m))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(prec, self.significant_bits())`,
    /// and $m$ is the exponent of $x$ if that is positive, and 0 otherwise.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::OneHalf;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (g, o) = BigExponentFloat::ONE_HALF.reciprocal_gamma_prec_ref(5);
    /// assert_eq!(g.to_string(), "0.562");
    /// assert_eq!(o, Less);
    ///
    /// let (g, o) = BigExponentFloat::ONE_HALF.reciprocal_gamma_prec_ref(20);
    /// assert_eq!(g.to_string(), "0.5641899");
    /// assert_eq!(o, Greater);
    ///
    /// let (g, o) = BigExponentFloat::from(1.0e9).reciprocal_gamma_prec_ref(20);
    /// assert_eq!(g.to_string(), "1.0096286e-8565705514");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn reciprocal_gamma_prec_ref(&self, prec: u64) -> (Self, Ordering) {
        self.reciprocal_gamma_prec_round_ref(prec, Nearest)
    }

    /// Computes $1/\Gamma(x)$, the reciprocal gamma function, of a [`BigExponentFloat`] in place,
    /// rounding the result to the nearest value of the specified precision. An [`Ordering`] is
    /// returned, indicating whether the rounded value is less than, equal to, or greater than the
    /// exact value. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this
    /// function sets the [`BigExponentFloat`] to `NaN` it also returns `Equal`.
    ///
    /// $\Gamma(x) = \int_0^\infty t^{x-1} e^{-t}\,dt$ for $x > 0$, and the recurrence $\Gamma(x) =
    /// \Gamma(x+1)/x$ extends it to every real $x$ other than $0, -1, -2, \ldots$, where it has
    /// poles. Its reciprocal is defined for every real $x$, and is 0 at those poles.
    ///
    /// If the value is equidistant from two [`BigExponentFloat`]s with the specified precision, the
    /// [`BigExponentFloat`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`]
    /// for a description of the `Nearest` rounding mode.
    ///
    /// $$
    /// x \gets 1/\Gamma(x)+\varepsilon.
    /// $$
    /// - If $1/\Gamma(x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $1/\Gamma(x)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |1/\Gamma(x)|\rfloor-p}$.
    ///
    /// See the [`BigExponentFloat::reciprocal_gamma_prec_round`] documentation for information on
    /// special cases, running time, and termination.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`BigExponentFloat::reciprocal_gamma_prec_round_assign`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O((n + m)^3 \log (n + m))$
    ///
    /// $M(n, m) = O((n + m)^2 \log (n + m))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(prec, self.significant_bits())`,
    /// and $m$ is the exponent of $x$ if that is positive, and 0 otherwise.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::OneHalf;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = BigExponentFloat::ONE_HALF;
    /// assert_eq!(x.reciprocal_gamma_prec_assign(5), Less);
    /// assert_eq!(x.to_string(), "0.562");
    ///
    /// let mut x = BigExponentFloat::ONE_HALF;
    /// assert_eq!(x.reciprocal_gamma_prec_assign(20), Greater);
    /// assert_eq!(x.to_string(), "0.5641899");
    ///
    /// let mut x = BigExponentFloat::from(1.0e9);
    /// assert_eq!(x.reciprocal_gamma_prec_assign(20), Less);
    /// assert_eq!(x.to_string(), "1.0096286e-8565705514");
    /// ```
    #[inline]
    pub fn reciprocal_gamma_prec_assign(&mut self, prec: u64) -> Ordering {
        self.reciprocal_gamma_prec_round_assign(prec, Nearest)
    }

    /// Computes $1/\Gamma(x)$, the reciprocal gamma function, of a [`Rational`], rounding the
    /// result to the specified precision and with the specified rounding mode and returning the
    /// result as a [`BigExponentFloat`]. The [`Rational`] is taken by value. An [`Ordering`] is
    /// also returned, indicating whether the rounded value is less than, equal to, or greater than
    /// the exact value. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever
    /// this function returns a `NaN` it also returns `Equal`.
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
    /// Special cases:
    /// - $f(x,p,m)=0$ if $x$ is 0 or a negative integer
    ///
    /// Unlike [its `Float` counterpart](crate::Float::reciprocal_gamma_rational_prec_round), this
    /// function never overflows or underflows.
    ///
    /// Warning: the absolute value of $\ln \Gamma(x)$ is about $|x| \ln |x|$ for a large $|x|$, so
    /// an argument with exponent $e > 0$ takes about $e$ bits of working precision beyond `prec`,
    /// and gives a result whose exponent has about $e$ bits. The time and memory needed therefore
    /// grow with the exponent, without any limit: an argument near $2^{2^{40}}$ needs more than
    /// $2^{40}$ bits, or 128 GiB.
    ///
    /// The result is correctly rounded. Finding it takes $1/\Gamma(x)$ not to be a rounding
    /// boundary, which is a number with at most `prec + 1` significant bits. This is known when $x$
    /// is an integer, and when its denominator is 2, 3, 4, or 6 (then $\Gamma(x)$ is irrational);
    /// nothing suggests that it ever fails, but it has not been proven for every rational $x$, and
    /// this function would not terminate if it did fail.
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`BigExponentFloat::reciprocal_gamma_rational_prec`] instead.
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
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (g, o) = BigExponentFloat::reciprocal_gamma_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(g.to_string(), "0.359");
    /// assert_eq!(o, Less);
    ///
    /// let (g, o) = BigExponentFloat::reciprocal_gamma_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(g.to_string(), "0.38");
    /// assert_eq!(o, Greater);
    ///
    /// let (g, o) = BigExponentFloat::reciprocal_gamma_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(g.to_string(), "0.38");
    /// assert_eq!(o, Greater);
    ///
    /// let (g, o) = BigExponentFloat::reciprocal_gamma_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(g.to_string(), "0.37328196");
    /// assert_eq!(o, Less);
    ///
    /// let (g, o) = BigExponentFloat::reciprocal_gamma_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(g.to_string(), "0.3732824");
    /// assert_eq!(o, Greater);
    ///
    /// let (g, o) = BigExponentFloat::reciprocal_gamma_rational_prec_round(
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
        gamma_rational_helper(true, &x, prec, rm)
    }

    /// Computes $1/\Gamma(x)$, the reciprocal gamma function, of a [`Rational`], rounding the
    /// result to the specified precision and with the specified rounding mode and returning the
    /// result as a [`BigExponentFloat`]. The [`Rational`] is taken by reference. An [`Ordering`] is
    /// also returned, indicating whether the rounded value is less than, equal to, or greater than
    /// the exact value. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever
    /// this function returns a `NaN` it also returns `Equal`.
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
    /// Special cases:
    /// - $f(x,p,m)=0$ if $x$ is 0 or a negative integer
    ///
    /// Unlike [its `Float` counterpart](crate::Float::reciprocal_gamma_rational_prec_round), this
    /// function never overflows or underflows.
    ///
    /// Warning: the absolute value of $\ln \Gamma(x)$ is about $|x| \ln |x|$ for a large $|x|$, so
    /// an argument with exponent $e > 0$ takes about $e$ bits of working precision beyond `prec`,
    /// and gives a result whose exponent has about $e$ bits. The time and memory needed therefore
    /// grow with the exponent, without any limit: an argument near $2^{2^{40}}$ needs more than
    /// $2^{40}$ bits, or 128 GiB.
    ///
    /// The result is correctly rounded. Finding it takes $1/\Gamma(x)$ not to be a rounding
    /// boundary, which is a number with at most `prec + 1` significant bits. This is known when $x$
    /// is an integer, and when its denominator is 2, 3, 4, or 6 (then $\Gamma(x)$ is irrational);
    /// nothing suggests that it ever fails, but it has not been proven for every rational $x$, and
    /// this function would not terminate if it did fail.
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`BigExponentFloat::reciprocal_gamma_rational_prec_ref`] instead.
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
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (g, o) = BigExponentFloat::reciprocal_gamma_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(g.to_string(), "0.359");
    /// assert_eq!(o, Less);
    ///
    /// let (g, o) = BigExponentFloat::reciprocal_gamma_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(g.to_string(), "0.38");
    /// assert_eq!(o, Greater);
    ///
    /// let (g, o) = BigExponentFloat::reciprocal_gamma_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(g.to_string(), "0.38");
    /// assert_eq!(o, Greater);
    ///
    /// let (g, o) = BigExponentFloat::reciprocal_gamma_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(g.to_string(), "0.37328196");
    /// assert_eq!(o, Less);
    ///
    /// let (g, o) = BigExponentFloat::reciprocal_gamma_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(g.to_string(), "0.3732824");
    /// assert_eq!(o, Greater);
    ///
    /// let (g, o) = BigExponentFloat::reciprocal_gamma_rational_prec_round_ref(
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
        gamma_rational_helper(true, x, prec, rm)
    }

    /// Computes $1/\Gamma(x)$, the reciprocal gamma function, of a [`Rational`], rounding the
    /// result to the nearest value of the specified precision and returning the result as a
    /// [`BigExponentFloat`]. The [`Rational`] is taken by value. An [`Ordering`] is also returned,
    /// indicating whether the rounded value is less than, equal to, or greater than the exact
    /// value. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// $\Gamma(x) = \int_0^\infty t^{x-1} e^{-t}\,dt$ for $x > 0$, and the recurrence $\Gamma(x) =
    /// \Gamma(x+1)/x$ extends it to every real $x$ other than $0, -1, -2, \ldots$, where it has
    /// poles. Its reciprocal is defined for every real $x$, and is 0 at those poles.
    ///
    /// If the value is equidistant from two [`BigExponentFloat`]s with the specified precision, the
    /// [`BigExponentFloat`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`]
    /// for a description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = 1/\Gamma(x)+\varepsilon.
    /// $$
    /// - If $1/\Gamma(x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $1/\Gamma(x)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |1/\Gamma(x)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,p)=0$ if $x$ is 0 or a negative integer
    ///
    /// Unlike [its `Float` counterpart](crate::Float::reciprocal_gamma_rational_prec_round), this
    /// function never overflows or underflows.
    ///
    /// Warning: the absolute value of $\ln \Gamma(x)$ is about $|x| \ln |x|$ for a large $|x|$, so
    /// an argument with exponent $e > 0$ takes about $e$ bits of working precision beyond `prec`,
    /// and gives a result whose exponent has about $e$ bits. The time and memory needed therefore
    /// grow with the exponent, without any limit: an argument near $2^{2^{40}}$ needs more than
    /// $2^{40}$ bits, or 128 GiB.
    ///
    /// The result is correctly rounded. Finding it takes $1/\Gamma(x)$ not to be a rounding
    /// boundary, which is a number with at most `prec + 1` significant bits. This is known when $x$
    /// is an integer, and when its denominator is 2, 3, 4, or 6 (then $\Gamma(x)$ is irrational);
    /// nothing suggests that it ever fails, but it has not been proven for every rational $x$, and
    /// this function would not terminate if it did fail.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`BigExponentFloat::reciprocal_gamma_rational_prec_round`] instead.
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
    /// let (g, o) =
    ///     BigExponentFloat::reciprocal_gamma_rational_prec(Rational::from_unsigneds(1u8, 3), 5);
    /// assert_eq!(g.to_string(), "0.38");
    /// assert_eq!(o, Greater);
    ///
    /// let (g, o) =
    ///     BigExponentFloat::reciprocal_gamma_rational_prec(Rational::from_unsigneds(1u8, 3), 20);
    /// assert_eq!(g.to_string(), "0.37328196");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn reciprocal_gamma_rational_prec(x: Rational, prec: u64) -> (Self, Ordering) {
        Self::reciprocal_gamma_rational_prec_round(x, prec, Nearest)
    }

    /// Computes $1/\Gamma(x)$, the reciprocal gamma function, of a [`Rational`], rounding the
    /// result to the nearest value of the specified precision and returning the result as a
    /// [`BigExponentFloat`]. The [`Rational`] is taken by reference. An [`Ordering`] is also
    /// returned, indicating whether the rounded value is less than, equal to, or greater than the
    /// exact value. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// $\Gamma(x) = \int_0^\infty t^{x-1} e^{-t}\,dt$ for $x > 0$, and the recurrence $\Gamma(x) =
    /// \Gamma(x+1)/x$ extends it to every real $x$ other than $0, -1, -2, \ldots$, where it has
    /// poles. Its reciprocal is defined for every real $x$, and is 0 at those poles.
    ///
    /// If the value is equidistant from two [`BigExponentFloat`]s with the specified precision, the
    /// [`BigExponentFloat`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`]
    /// for a description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = 1/\Gamma(x)+\varepsilon.
    /// $$
    /// - If $1/\Gamma(x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $1/\Gamma(x)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |1/\Gamma(x)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,p)=0$ if $x$ is 0 or a negative integer
    ///
    /// Unlike [its `Float` counterpart](crate::Float::reciprocal_gamma_rational_prec_round), this
    /// function never overflows or underflows.
    ///
    /// Warning: the absolute value of $\ln \Gamma(x)$ is about $|x| \ln |x|$ for a large $|x|$, so
    /// an argument with exponent $e > 0$ takes about $e$ bits of working precision beyond `prec`,
    /// and gives a result whose exponent has about $e$ bits. The time and memory needed therefore
    /// grow with the exponent, without any limit: an argument near $2^{2^{40}}$ needs more than
    /// $2^{40}$ bits, or 128 GiB.
    ///
    /// The result is correctly rounded. Finding it takes $1/\Gamma(x)$ not to be a rounding
    /// boundary, which is a number with at most `prec + 1` significant bits. This is known when $x$
    /// is an integer, and when its denominator is 2, 3, 4, or 6 (then $\Gamma(x)$ is irrational);
    /// nothing suggests that it ever fails, but it has not been proven for every rational $x$, and
    /// this function would not terminate if it did fail.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`BigExponentFloat::reciprocal_gamma_rational_prec_round_ref`] instead.
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
    /// let (g, o) = BigExponentFloat::reciprocal_gamma_rational_prec_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     5,
    /// );
    /// assert_eq!(g.to_string(), "0.38");
    /// assert_eq!(o, Greater);
    ///
    /// let (g, o) = BigExponentFloat::reciprocal_gamma_rational_prec_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     20,
    /// );
    /// assert_eq!(g.to_string(), "0.37328196");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn reciprocal_gamma_rational_prec_ref(x: &Rational, prec: u64) -> (Self, Ordering) {
        Self::reciprocal_gamma_rational_prec_round_ref(x, prec, Nearest)
    }
}
