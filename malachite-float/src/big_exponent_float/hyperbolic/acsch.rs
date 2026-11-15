// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::big_exponent_float::BigExponentFloat;
use crate::big_exponent_float::hyperbolic::asinh::{
    InverseHyperbolicFunction, inverse_hyperbolic_helper, inverse_hyperbolic_rational_helper,
};
use core::cmp::Ordering;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_q::Rational;

impl BigExponentFloat {
    /// Computes the inverse hyperbolic cosecant of a [`BigExponentFloat`], rounding the result to
    /// the specified precision and with the specified rounding mode. The [`BigExponentFloat`] is
    /// taken by value. An [`Ordering`] is also returned, indicating whether the rounded inverse
    /// hyperbolic cosecant is less than, equal to, or greater than the exact inverse hyperbolic
    /// cosecant. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \operatorname{acsch} x+\varepsilon.
    /// $$
    /// - If $\operatorname{acsch} x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $\operatorname{acsch} x$ is finite and nonzero, and $m$ is not `Nearest`, then
    ///   $|\varepsilon| < 2^{\lfloor\log_2 |\operatorname{acsch} x|\rfloor-p+1}$.
    /// - If $\operatorname{acsch} x$ is finite and nonzero, and $m$ is `Nearest`, then
    ///   $|\varepsilon| \leq 2^{\lfloor\log_2 |\operatorname{acsch} x|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=\text{NaN}$
    /// - $f(\pm\infty,p,m)=0$
    /// - $f(0,p,m)=\text{NaN}$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::acsch_prec_round), this function accepts
    /// arguments with any exponent, and its result never overflows or underflows.
    ///
    /// A [`BigExponentFloat`] has no negative zero, so the two one-sided limits at 0 ($\infty$ and
    /// $-\infty$) cannot be told apart, and the result at 0 is `NaN`, as $1/0$ is, where the
    /// `Float` function gives an infinity.
    ///
    /// If you know you'll be using `Nearest`, consider using [`BigExponentFloat::acsch_prec`]
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^2 \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the result is finite and nonzero (when
    /// it is transcendental).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (y, o) = BigExponentFloat::ONE.acsch_prec_round(5, Floor);
    /// assert_eq!(y.to_string(), "0.88");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::ONE.acsch_prec_round(5, Ceiling);
    /// assert_eq!(y.to_string(), "0.906");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::ONE.acsch_prec_round(5, Nearest);
    /// assert_eq!(y.to_string(), "0.88");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::ONE.acsch_prec_round(20, Floor);
    /// assert_eq!(y.to_string(), "0.88137341");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::ONE.acsch_prec_round(20, Ceiling);
    /// assert_eq!(y.to_string(), "0.8813744");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::ONE.acsch_prec_round(20, Nearest);
    /// assert_eq!(y.to_string(), "0.88137341");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn acsch_prec_round(self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        inverse_hyperbolic_helper(InverseHyperbolicFunction::Acsch, &self, prec, rm, true)
    }

    /// Computes the inverse hyperbolic cosecant of a [`BigExponentFloat`], rounding the result to
    /// the specified precision and with the specified rounding mode. The [`BigExponentFloat`] is
    /// taken by reference. An [`Ordering`] is also returned, indicating whether the rounded inverse
    /// hyperbolic cosecant is less than, equal to, or greater than the exact inverse hyperbolic
    /// cosecant. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \operatorname{acsch} x+\varepsilon.
    /// $$
    /// - If $\operatorname{acsch} x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $\operatorname{acsch} x$ is finite and nonzero, and $m$ is not `Nearest`, then
    ///   $|\varepsilon| < 2^{\lfloor\log_2 |\operatorname{acsch} x|\rfloor-p+1}$.
    /// - If $\operatorname{acsch} x$ is finite and nonzero, and $m$ is `Nearest`, then
    ///   $|\varepsilon| \leq 2^{\lfloor\log_2 |\operatorname{acsch} x|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=\text{NaN}$
    /// - $f(\pm\infty,p,m)=0$
    /// - $f(0,p,m)=\text{NaN}$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::acsch_prec_round), this function accepts
    /// arguments with any exponent, and its result never overflows or underflows.
    ///
    /// A [`BigExponentFloat`] has no negative zero, so the two one-sided limits at 0 ($\infty$ and
    /// $-\infty$) cannot be told apart, and the result at 0 is `NaN`, as $1/0$ is, where the
    /// `Float` function gives an infinity.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^2 \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the result is finite and nonzero (when
    /// it is transcendental).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (y, o) = (&BigExponentFloat::ONE).acsch_prec_round_ref(5, Floor);
    /// assert_eq!(y.to_string(), "0.88");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = (&BigExponentFloat::ONE).acsch_prec_round_ref(5, Ceiling);
    /// assert_eq!(y.to_string(), "0.906");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = (&BigExponentFloat::ONE).acsch_prec_round_ref(5, Nearest);
    /// assert_eq!(y.to_string(), "0.88");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = (&BigExponentFloat::ONE).acsch_prec_round_ref(20, Floor);
    /// assert_eq!(y.to_string(), "0.88137341");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = (&BigExponentFloat::ONE).acsch_prec_round_ref(20, Ceiling);
    /// assert_eq!(y.to_string(), "0.8813744");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = (&BigExponentFloat::ONE).acsch_prec_round_ref(20, Nearest);
    /// assert_eq!(y.to_string(), "0.88137341");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn acsch_prec_round_ref(&self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        inverse_hyperbolic_helper(InverseHyperbolicFunction::Acsch, self, prec, rm, true)
    }

    /// Replaces a [`BigExponentFloat`] with its inverse hyperbolic cosecant, rounding the result to
    /// the specified precision and with the specified rounding mode. An [`Ordering`] is returned,
    /// indicating whether the rounded inverse hyperbolic cosecant is less than, equal to, or
    /// greater than the exact inverse hyperbolic cosecant. Although `NaN`s are not comparable to
    /// any [`BigExponentFloat`], whenever this function sets the [`BigExponentFloat`] to `NaN` it
    /// also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \operatorname{acsch} x+\varepsilon.
    /// $$
    /// - If $\operatorname{acsch} x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $\operatorname{acsch} x$ is finite and nonzero, and $m$ is not `Nearest`, then
    ///   $|\varepsilon| < 2^{\lfloor\log_2 |\operatorname{acsch} x|\rfloor-p+1}$.
    /// - If $\operatorname{acsch} x$ is finite and nonzero, and $m$ is `Nearest`, then
    ///   $|\varepsilon| \leq 2^{\lfloor\log_2 |\operatorname{acsch} x|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=\text{NaN}$
    /// - $f(\pm\infty,p,m)=0$
    /// - $f(0,p,m)=\text{NaN}$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::acsch_prec_round), this function accepts
    /// arguments with any exponent, and its result never overflows or underflows.
    ///
    /// A [`BigExponentFloat`] has no negative zero, so the two one-sided limits at 0 ($\infty$ and
    /// $-\infty$) cannot be told apart, and the result at 0 is `NaN`, as $1/0$ is, where the
    /// `Float` function gives an infinity.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^2 \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the result is finite and nonzero (when
    /// it is transcendental).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = BigExponentFloat::ONE;
    /// assert_eq!(x.acsch_prec_round_assign(5, Floor), Less);
    /// assert_eq!(x.to_string(), "0.88");
    ///
    /// let mut x = BigExponentFloat::ONE;
    /// assert_eq!(x.acsch_prec_round_assign(5, Ceiling), Greater);
    /// assert_eq!(x.to_string(), "0.906");
    ///
    /// let mut x = BigExponentFloat::ONE;
    /// assert_eq!(x.acsch_prec_round_assign(5, Nearest), Less);
    /// assert_eq!(x.to_string(), "0.88");
    ///
    /// let mut x = BigExponentFloat::ONE;
    /// assert_eq!(x.acsch_prec_round_assign(20, Floor), Less);
    /// assert_eq!(x.to_string(), "0.88137341");
    ///
    /// let mut x = BigExponentFloat::ONE;
    /// assert_eq!(x.acsch_prec_round_assign(20, Ceiling), Greater);
    /// assert_eq!(x.to_string(), "0.8813744");
    ///
    /// let mut x = BigExponentFloat::ONE;
    /// assert_eq!(x.acsch_prec_round_assign(20, Nearest), Less);
    /// assert_eq!(x.to_string(), "0.88137341");
    /// ```
    #[inline]
    pub fn acsch_prec_round_assign(&mut self, prec: u64, rm: RoundingMode) -> Ordering {
        let (result, o) =
            inverse_hyperbolic_helper(InverseHyperbolicFunction::Acsch, self, prec, rm, true);
        *self = result;
        o
    }

    /// Computes the inverse hyperbolic cosecant of a [`BigExponentFloat`], rounding the result to
    /// the nearest value of the specified precision. The [`BigExponentFloat`] is taken by value. An
    /// [`Ordering`] is also returned, indicating whether the rounded inverse hyperbolic cosecant is
    /// less than, equal to, or greater than the exact inverse hyperbolic cosecant. Although `NaN`s
    /// are not comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it
    /// also returns `Equal`.
    ///
    /// $$
    /// f(x,p) = \operatorname{acsch} x+\varepsilon.
    /// $$
    /// - If $\operatorname{acsch} x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $\operatorname{acsch} x$ is finite and nonzero, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\operatorname{acsch} x|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p)=\text{NaN}$
    /// - $f(\pm\infty,p)=0$
    /// - $f(0,p)=\text{NaN}$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::acsch_prec_round), this function accepts
    /// arguments with any exponent, and its result never overflows or underflows.
    ///
    /// A [`BigExponentFloat`] has no negative zero, so the two one-sided limits at 0 ($\infty$ and
    /// $-\infty$) cannot be told apart, and the result at 0 is `NaN`, as $1/0$ is, where the
    /// `Float` function gives an infinity.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`BigExponentFloat::acsch_prec_round`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^2 \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (y, o) = BigExponentFloat::ONE.acsch_prec(5);
    /// assert_eq!(y.to_string(), "0.88");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::ONE.acsch_prec(20);
    /// assert_eq!(y.to_string(), "0.88137341");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn acsch_prec(self, prec: u64) -> (Self, Ordering) {
        inverse_hyperbolic_helper(InverseHyperbolicFunction::Acsch, &self, prec, Nearest, true)
    }

    /// Computes the inverse hyperbolic cosecant of a [`BigExponentFloat`], rounding the result to
    /// the nearest value of the specified precision. The [`BigExponentFloat`] is taken by
    /// reference. An [`Ordering`] is also returned, indicating whether the rounded inverse
    /// hyperbolic cosecant is less than, equal to, or greater than the exact inverse hyperbolic
    /// cosecant. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,p) = \operatorname{acsch} x+\varepsilon.
    /// $$
    /// - If $\operatorname{acsch} x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $\operatorname{acsch} x$ is finite and nonzero, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\operatorname{acsch} x|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p)=\text{NaN}$
    /// - $f(\pm\infty,p)=0$
    /// - $f(0,p)=\text{NaN}$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::acsch_prec_round), this function accepts
    /// arguments with any exponent, and its result never overflows or underflows.
    ///
    /// A [`BigExponentFloat`] has no negative zero, so the two one-sided limits at 0 ($\infty$ and
    /// $-\infty$) cannot be told apart, and the result at 0 is `NaN`, as $1/0$ is, where the
    /// `Float` function gives an infinity.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^2 \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (y, o) = (&BigExponentFloat::ONE).acsch_prec_ref(5);
    /// assert_eq!(y.to_string(), "0.88");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = (&BigExponentFloat::ONE).acsch_prec_ref(20);
    /// assert_eq!(y.to_string(), "0.88137341");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn acsch_prec_ref(&self, prec: u64) -> (Self, Ordering) {
        inverse_hyperbolic_helper(InverseHyperbolicFunction::Acsch, self, prec, Nearest, true)
    }

    /// Replaces a [`BigExponentFloat`] with its inverse hyperbolic cosecant, rounding the result to
    /// the nearest value of the specified precision. An [`Ordering`] is returned, indicating
    /// whether the rounded inverse hyperbolic cosecant is less than, equal to, or greater than the
    /// exact inverse hyperbolic cosecant. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function sets the [`BigExponentFloat`] to `NaN` it also
    /// returns `Equal`.
    ///
    /// $$
    /// f(x,p) = \operatorname{acsch} x+\varepsilon.
    /// $$
    /// - If $\operatorname{acsch} x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $\operatorname{acsch} x$ is finite and nonzero, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\operatorname{acsch} x|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p)=\text{NaN}$
    /// - $f(\pm\infty,p)=0$
    /// - $f(0,p)=\text{NaN}$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::acsch_prec_round), this function accepts
    /// arguments with any exponent, and its result never overflows or underflows.
    ///
    /// A [`BigExponentFloat`] has no negative zero, so the two one-sided limits at 0 ($\infty$ and
    /// $-\infty$) cannot be told apart, and the result at 0 is `NaN`, as $1/0$ is, where the
    /// `Float` function gives an infinity.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^2 \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = BigExponentFloat::ONE;
    /// assert_eq!(x.acsch_prec_assign(5), Less);
    /// assert_eq!(x.to_string(), "0.88");
    ///
    /// let mut x = BigExponentFloat::ONE;
    /// assert_eq!(x.acsch_prec_assign(20), Less);
    /// assert_eq!(x.to_string(), "0.88137341");
    /// ```
    #[inline]
    pub fn acsch_prec_assign(&mut self, prec: u64) -> Ordering {
        let (result, o) =
            inverse_hyperbolic_helper(InverseHyperbolicFunction::Acsch, self, prec, Nearest, true);
        *self = result;
        o
    }

    /// Computes the inverse hyperbolic cosecant of a [`Rational`], returning a
    /// [`BigExponentFloat`], rounding the result to the specified precision and with the specified
    /// rounding mode. The [`Rational`] is taken by value. An [`Ordering`] is also returned,
    /// indicating whether the rounded inverse hyperbolic cosecant is less than, equal to, or
    /// greater than the exact inverse hyperbolic cosecant. Although `NaN`s are not comparable to
    /// any [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \operatorname{acsch} x+\varepsilon.
    /// $$
    /// - If $\operatorname{acsch} x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $\operatorname{acsch} x$ is finite and nonzero, and $m$ is not `Nearest`, then
    ///   $|\varepsilon| < 2^{\lfloor\log_2 |\operatorname{acsch} x|\rfloor-p+1}$.
    /// - If $\operatorname{acsch} x$ is finite and nonzero, and $m$ is `Nearest`, then
    ///   $|\varepsilon| \leq 2^{\lfloor\log_2 |\operatorname{acsch} x|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(0,p,m)=\text{NaN}$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::acsch_rational_prec_round), this function
    /// never overflows or underflows.
    ///
    /// A [`BigExponentFloat`] has no negative zero, so the two one-sided limits at 0 ($\infty$ and
    /// $-\infty$) cannot be told apart, and the result at 0 is `NaN`, as $1/0$ is, where the
    /// `Float` function gives an infinity.
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`BigExponentFloat::acsch_rational_prec`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^2 \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `x.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the result is finite and nonzero (when
    /// it is transcendental).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (y, o) =
    ///     BigExponentFloat::acsch_rational_prec_round(Rational::from_unsigneds(1u8, 3), 5, Floor);
    /// assert_eq!(y.to_string(), "1.81");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::acsch_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(y.to_string(), "1.88");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::acsch_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(y.to_string(), "1.81");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::acsch_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(y.to_string(), "1.8184452");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::acsch_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(y.to_string(), "1.818447");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::acsch_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(y.to_string(), "1.818447");
    /// assert_eq!(o, Greater);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn acsch_rational_prec_round(x: Rational, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        inverse_hyperbolic_rational_helper(InverseHyperbolicFunction::Acsch, &x, prec, rm, true)
    }

    /// Computes the inverse hyperbolic cosecant of a [`Rational`], returning a
    /// [`BigExponentFloat`], rounding the result to the specified precision and with the specified
    /// rounding mode. The [`Rational`] is taken by reference. An [`Ordering`] is also returned,
    /// indicating whether the rounded inverse hyperbolic cosecant is less than, equal to, or
    /// greater than the exact inverse hyperbolic cosecant. Although `NaN`s are not comparable to
    /// any [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \operatorname{acsch} x+\varepsilon.
    /// $$
    /// - If $\operatorname{acsch} x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $\operatorname{acsch} x$ is finite and nonzero, and $m$ is not `Nearest`, then
    ///   $|\varepsilon| < 2^{\lfloor\log_2 |\operatorname{acsch} x|\rfloor-p+1}$.
    /// - If $\operatorname{acsch} x$ is finite and nonzero, and $m$ is `Nearest`, then
    ///   $|\varepsilon| \leq 2^{\lfloor\log_2 |\operatorname{acsch} x|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(0,p,m)=\text{NaN}$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::acsch_rational_prec_round), this function
    /// never overflows or underflows.
    ///
    /// A [`BigExponentFloat`] has no negative zero, so the two one-sided limits at 0 ($\infty$ and
    /// $-\infty$) cannot be told apart, and the result at 0 is `NaN`, as $1/0$ is, where the
    /// `Float` function gives an infinity.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^2 \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `x.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the result is finite and nonzero (when
    /// it is transcendental).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (y, o) = BigExponentFloat::acsch_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(y.to_string(), "1.81");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::acsch_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(y.to_string(), "1.88");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::acsch_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(y.to_string(), "1.81");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::acsch_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(y.to_string(), "1.8184452");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::acsch_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(y.to_string(), "1.818447");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::acsch_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(y.to_string(), "1.818447");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn acsch_rational_prec_round_ref(
        x: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        inverse_hyperbolic_rational_helper(InverseHyperbolicFunction::Acsch, x, prec, rm, true)
    }

    /// Computes the inverse hyperbolic cosecant of a [`Rational`], returning a
    /// [`BigExponentFloat`], rounding the result to the nearest value of the specified precision.
    /// The [`Rational`] is taken by value. An [`Ordering`] is also returned, indicating whether the
    /// rounded inverse hyperbolic cosecant is less than, equal to, or greater than the exact
    /// inverse hyperbolic cosecant. Although `NaN`s are not comparable to any [`BigExponentFloat`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,p) = \operatorname{acsch} x+\varepsilon.
    /// $$
    /// - If $\operatorname{acsch} x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $\operatorname{acsch} x$ is finite and nonzero, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\operatorname{acsch} x|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(0,p)=\text{NaN}$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::acsch_rational_prec_round), this function
    /// never overflows or underflows.
    ///
    /// A [`BigExponentFloat`] has no negative zero, so the two one-sided limits at 0 ($\infty$ and
    /// $-\infty$) cannot be told apart, and the result at 0 is `NaN`, as $1/0$ is, where the
    /// `Float` function gives an infinity.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`BigExponentFloat::acsch_rational_prec_round`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^2 \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `x.significant_bits()`.
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
    /// let (y, o) = BigExponentFloat::acsch_rational_prec(Rational::from_unsigneds(1u8, 3), 5);
    /// assert_eq!(y.to_string(), "1.81");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::acsch_rational_prec(Rational::from_unsigneds(1u8, 3), 20);
    /// assert_eq!(y.to_string(), "1.818447");
    /// assert_eq!(o, Greater);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn acsch_rational_prec(x: Rational, prec: u64) -> (Self, Ordering) {
        inverse_hyperbolic_rational_helper(
            InverseHyperbolicFunction::Acsch,
            &x,
            prec,
            Nearest,
            true,
        )
    }

    /// Computes the inverse hyperbolic cosecant of a [`Rational`], returning a
    /// [`BigExponentFloat`], rounding the result to the nearest value of the specified precision.
    /// The [`Rational`] is taken by reference. An [`Ordering`] is also returned, indicating whether
    /// the rounded inverse hyperbolic cosecant is less than, equal to, or greater than the exact
    /// inverse hyperbolic cosecant. Although `NaN`s are not comparable to any [`BigExponentFloat`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,p) = \operatorname{acsch} x+\varepsilon.
    /// $$
    /// - If $\operatorname{acsch} x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $\operatorname{acsch} x$ is finite and nonzero, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\operatorname{acsch} x|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(0,p)=\text{NaN}$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::acsch_rational_prec_round), this function
    /// never overflows or underflows.
    ///
    /// A [`BigExponentFloat`] has no negative zero, so the two one-sided limits at 0 ($\infty$ and
    /// $-\infty$) cannot be told apart, and the result at 0 is `NaN`, as $1/0$ is, where the
    /// `Float` function gives an infinity.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^2 \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `x.significant_bits()`.
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
    /// let (y, o) =
    ///     BigExponentFloat::acsch_rational_prec_ref(&Rational::from_unsigneds(1u8, 3), 5);
    /// assert_eq!(y.to_string(), "1.81");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) =
    ///     BigExponentFloat::acsch_rational_prec_ref(&Rational::from_unsigneds(1u8, 3), 20);
    /// assert_eq!(y.to_string(), "1.818447");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn acsch_rational_prec_ref(x: &Rational, prec: u64) -> (Self, Ordering) {
        inverse_hyperbolic_rational_helper(InverseHyperbolicFunction::Acsch, x, prec, Nearest, true)
    }
}
