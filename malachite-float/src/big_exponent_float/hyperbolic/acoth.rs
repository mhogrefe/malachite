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
    /// Computes the inverse hyperbolic cotangent of a [`BigExponentFloat`], rounding the result to
    /// the specified precision and with the specified rounding mode. The [`BigExponentFloat`] is
    /// taken by value. An [`Ordering`] is also returned, indicating whether the rounded inverse
    /// hyperbolic cotangent is less than, equal to, or greater than the exact inverse hyperbolic
    /// cotangent. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \operatorname{acoth} x+\varepsilon.
    /// $$
    /// - If $\operatorname{acoth} x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $\operatorname{acoth} x$ is finite and nonzero, and $m$ is not `Nearest`, then
    ///   $|\varepsilon| < 2^{\lfloor\log_2 |\operatorname{acoth} x|\rfloor-p+1}$.
    /// - If $\operatorname{acoth} x$ is finite and nonzero, and $m$ is `Nearest`, then
    ///   $|\varepsilon| \leq 2^{\lfloor\log_2 |\operatorname{acoth} x|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=\text{NaN}$
    /// - $f(\pm\infty,p,m)=0$
    /// - $f(\pm1,p,m)=\pm\infty$
    /// - $f(x,p,m)=\text{NaN}$ if $|x|<1$, including 0
    ///
    /// Unlike [its `Float` counterpart](crate::Float::acoth_prec_round), this function accepts
    /// arguments with any exponent, and its result never overflows or underflows.
    ///
    /// If you know you'll be using `Nearest`, consider using [`BigExponentFloat::acoth_prec`]
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
    /// use malachite_base::num::basic::traits::Two;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (y, o) = BigExponentFloat::TWO.acoth_prec_round(5, Floor);
    /// assert_eq!(y.to_string(), "0.531");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::TWO.acoth_prec_round(5, Ceiling);
    /// assert_eq!(y.to_string(), "0.562");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::TWO.acoth_prec_round(5, Nearest);
    /// assert_eq!(y.to_string(), "0.562");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::TWO.acoth_prec_round(20, Floor);
    /// assert_eq!(y.to_string(), "0.54930592");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::TWO.acoth_prec_round(20, Ceiling);
    /// assert_eq!(y.to_string(), "0.5493069");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::TWO.acoth_prec_round(20, Nearest);
    /// assert_eq!(y.to_string(), "0.54930592");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn acoth_prec_round(self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        inverse_hyperbolic_helper(InverseHyperbolicFunction::Acoth, &self, prec, rm, true)
    }

    /// Computes the inverse hyperbolic cotangent of a [`BigExponentFloat`], rounding the result to
    /// the specified precision and with the specified rounding mode. The [`BigExponentFloat`] is
    /// taken by reference. An [`Ordering`] is also returned, indicating whether the rounded inverse
    /// hyperbolic cotangent is less than, equal to, or greater than the exact inverse hyperbolic
    /// cotangent. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \operatorname{acoth} x+\varepsilon.
    /// $$
    /// - If $\operatorname{acoth} x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $\operatorname{acoth} x$ is finite and nonzero, and $m$ is not `Nearest`, then
    ///   $|\varepsilon| < 2^{\lfloor\log_2 |\operatorname{acoth} x|\rfloor-p+1}$.
    /// - If $\operatorname{acoth} x$ is finite and nonzero, and $m$ is `Nearest`, then
    ///   $|\varepsilon| \leq 2^{\lfloor\log_2 |\operatorname{acoth} x|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=\text{NaN}$
    /// - $f(\pm\infty,p,m)=0$
    /// - $f(\pm1,p,m)=\pm\infty$
    /// - $f(x,p,m)=\text{NaN}$ if $|x|<1$, including 0
    ///
    /// Unlike [its `Float` counterpart](crate::Float::acoth_prec_round), this function accepts
    /// arguments with any exponent, and its result never overflows or underflows.
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
    /// use malachite_base::num::basic::traits::Two;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (y, o) = (&BigExponentFloat::TWO).acoth_prec_round_ref(5, Floor);
    /// assert_eq!(y.to_string(), "0.531");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = (&BigExponentFloat::TWO).acoth_prec_round_ref(5, Ceiling);
    /// assert_eq!(y.to_string(), "0.562");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = (&BigExponentFloat::TWO).acoth_prec_round_ref(5, Nearest);
    /// assert_eq!(y.to_string(), "0.562");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = (&BigExponentFloat::TWO).acoth_prec_round_ref(20, Floor);
    /// assert_eq!(y.to_string(), "0.54930592");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = (&BigExponentFloat::TWO).acoth_prec_round_ref(20, Ceiling);
    /// assert_eq!(y.to_string(), "0.5493069");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = (&BigExponentFloat::TWO).acoth_prec_round_ref(20, Nearest);
    /// assert_eq!(y.to_string(), "0.54930592");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn acoth_prec_round_ref(&self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        inverse_hyperbolic_helper(InverseHyperbolicFunction::Acoth, self, prec, rm, true)
    }

    /// Replaces a [`BigExponentFloat`] with its inverse hyperbolic cotangent, rounding the result
    /// to the specified precision and with the specified rounding mode. An [`Ordering`] is
    /// returned, indicating whether the rounded inverse hyperbolic cotangent is less than, equal
    /// to, or greater than the exact inverse hyperbolic cotangent. Although `NaN`s are not
    /// comparable to any [`BigExponentFloat`], whenever this function sets the [`BigExponentFloat`]
    /// to `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \operatorname{acoth} x+\varepsilon.
    /// $$
    /// - If $\operatorname{acoth} x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $\operatorname{acoth} x$ is finite and nonzero, and $m$ is not `Nearest`, then
    ///   $|\varepsilon| < 2^{\lfloor\log_2 |\operatorname{acoth} x|\rfloor-p+1}$.
    /// - If $\operatorname{acoth} x$ is finite and nonzero, and $m$ is `Nearest`, then
    ///   $|\varepsilon| \leq 2^{\lfloor\log_2 |\operatorname{acoth} x|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=\text{NaN}$
    /// - $f(\pm\infty,p,m)=0$
    /// - $f(\pm1,p,m)=\pm\infty$
    /// - $f(x,p,m)=\text{NaN}$ if $|x|<1$, including 0
    ///
    /// Unlike [its `Float` counterpart](crate::Float::acoth_prec_round), this function accepts
    /// arguments with any exponent, and its result never overflows or underflows.
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
    /// use malachite_base::num::basic::traits::Two;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = BigExponentFloat::TWO;
    /// assert_eq!(x.acoth_prec_round_assign(5, Floor), Less);
    /// assert_eq!(x.to_string(), "0.531");
    ///
    /// let mut x = BigExponentFloat::TWO;
    /// assert_eq!(x.acoth_prec_round_assign(5, Ceiling), Greater);
    /// assert_eq!(x.to_string(), "0.562");
    ///
    /// let mut x = BigExponentFloat::TWO;
    /// assert_eq!(x.acoth_prec_round_assign(5, Nearest), Greater);
    /// assert_eq!(x.to_string(), "0.562");
    ///
    /// let mut x = BigExponentFloat::TWO;
    /// assert_eq!(x.acoth_prec_round_assign(20, Floor), Less);
    /// assert_eq!(x.to_string(), "0.54930592");
    ///
    /// let mut x = BigExponentFloat::TWO;
    /// assert_eq!(x.acoth_prec_round_assign(20, Ceiling), Greater);
    /// assert_eq!(x.to_string(), "0.5493069");
    ///
    /// let mut x = BigExponentFloat::TWO;
    /// assert_eq!(x.acoth_prec_round_assign(20, Nearest), Less);
    /// assert_eq!(x.to_string(), "0.54930592");
    /// ```
    #[inline]
    pub fn acoth_prec_round_assign(&mut self, prec: u64, rm: RoundingMode) -> Ordering {
        let (result, o) =
            inverse_hyperbolic_helper(InverseHyperbolicFunction::Acoth, self, prec, rm, true);
        *self = result;
        o
    }

    /// Computes the inverse hyperbolic cotangent of a [`BigExponentFloat`], rounding the result to
    /// the nearest value of the specified precision. The [`BigExponentFloat`] is taken by value. An
    /// [`Ordering`] is also returned, indicating whether the rounded inverse hyperbolic cotangent
    /// is less than, equal to, or greater than the exact inverse hyperbolic cotangent. Although
    /// `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function returns a
    /// `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,p) = \operatorname{acoth} x+\varepsilon.
    /// $$
    /// - If $\operatorname{acoth} x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $\operatorname{acoth} x$ is finite and nonzero, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\operatorname{acoth} x|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p)=\text{NaN}$
    /// - $f(\pm\infty,p)=0$
    /// - $f(\pm1,p)=\pm\infty$
    /// - $f(x,p)=\text{NaN}$ if $|x|<1$, including 0
    ///
    /// Unlike [its `Float` counterpart](crate::Float::acoth_prec_round), this function accepts
    /// arguments with any exponent, and its result never overflows or underflows.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`BigExponentFloat::acoth_prec_round`] instead.
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
    /// use malachite_base::num::basic::traits::Two;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (y, o) = BigExponentFloat::TWO.acoth_prec(5);
    /// assert_eq!(y.to_string(), "0.562");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::TWO.acoth_prec(20);
    /// assert_eq!(y.to_string(), "0.54930592");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn acoth_prec(self, prec: u64) -> (Self, Ordering) {
        inverse_hyperbolic_helper(InverseHyperbolicFunction::Acoth, &self, prec, Nearest, true)
    }

    /// Computes the inverse hyperbolic cotangent of a [`BigExponentFloat`], rounding the result to
    /// the nearest value of the specified precision. The [`BigExponentFloat`] is taken by
    /// reference. An [`Ordering`] is also returned, indicating whether the rounded inverse
    /// hyperbolic cotangent is less than, equal to, or greater than the exact inverse hyperbolic
    /// cotangent. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,p) = \operatorname{acoth} x+\varepsilon.
    /// $$
    /// - If $\operatorname{acoth} x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $\operatorname{acoth} x$ is finite and nonzero, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\operatorname{acoth} x|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p)=\text{NaN}$
    /// - $f(\pm\infty,p)=0$
    /// - $f(\pm1,p)=\pm\infty$
    /// - $f(x,p)=\text{NaN}$ if $|x|<1$, including 0
    ///
    /// Unlike [its `Float` counterpart](crate::Float::acoth_prec_round), this function accepts
    /// arguments with any exponent, and its result never overflows or underflows.
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
    /// use malachite_base::num::basic::traits::Two;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (y, o) = (&BigExponentFloat::TWO).acoth_prec_ref(5);
    /// assert_eq!(y.to_string(), "0.562");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = (&BigExponentFloat::TWO).acoth_prec_ref(20);
    /// assert_eq!(y.to_string(), "0.54930592");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn acoth_prec_ref(&self, prec: u64) -> (Self, Ordering) {
        inverse_hyperbolic_helper(InverseHyperbolicFunction::Acoth, self, prec, Nearest, true)
    }

    /// Replaces a [`BigExponentFloat`] with its inverse hyperbolic cotangent, rounding the result
    /// to the nearest value of the specified precision. An [`Ordering`] is returned, indicating
    /// whether the rounded inverse hyperbolic cotangent is less than, equal to, or greater than the
    /// exact inverse hyperbolic cotangent. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function sets the [`BigExponentFloat`] to `NaN` it also
    /// returns `Equal`.
    ///
    /// $$
    /// f(x,p) = \operatorname{acoth} x+\varepsilon.
    /// $$
    /// - If $\operatorname{acoth} x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $\operatorname{acoth} x$ is finite and nonzero, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\operatorname{acoth} x|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p)=\text{NaN}$
    /// - $f(\pm\infty,p)=0$
    /// - $f(\pm1,p)=\pm\infty$
    /// - $f(x,p)=\text{NaN}$ if $|x|<1$, including 0
    ///
    /// Unlike [its `Float` counterpart](crate::Float::acoth_prec_round), this function accepts
    /// arguments with any exponent, and its result never overflows or underflows.
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
    /// use malachite_base::num::basic::traits::Two;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = BigExponentFloat::TWO;
    /// assert_eq!(x.acoth_prec_assign(5), Greater);
    /// assert_eq!(x.to_string(), "0.562");
    ///
    /// let mut x = BigExponentFloat::TWO;
    /// assert_eq!(x.acoth_prec_assign(20), Less);
    /// assert_eq!(x.to_string(), "0.54930592");
    /// ```
    #[inline]
    pub fn acoth_prec_assign(&mut self, prec: u64) -> Ordering {
        let (result, o) =
            inverse_hyperbolic_helper(InverseHyperbolicFunction::Acoth, self, prec, Nearest, true);
        *self = result;
        o
    }

    /// Computes the inverse hyperbolic cotangent of a [`Rational`], returning a
    /// [`BigExponentFloat`], rounding the result to the specified precision and with the specified
    /// rounding mode. The [`Rational`] is taken by value. An [`Ordering`] is also returned,
    /// indicating whether the rounded inverse hyperbolic cotangent is less than, equal to, or
    /// greater than the exact inverse hyperbolic cotangent. Although `NaN`s are not comparable to
    /// any [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \operatorname{acoth} x+\varepsilon.
    /// $$
    /// - If $\operatorname{acoth} x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $\operatorname{acoth} x$ is finite and nonzero, and $m$ is not `Nearest`, then
    ///   $|\varepsilon| < 2^{\lfloor\log_2 |\operatorname{acoth} x|\rfloor-p+1}$.
    /// - If $\operatorname{acoth} x$ is finite and nonzero, and $m$ is `Nearest`, then
    ///   $|\varepsilon| \leq 2^{\lfloor\log_2 |\operatorname{acoth} x|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\pm1,p,m)=\pm\infty$
    /// - $f(x,p,m)=\text{NaN}$ if $|x|<1$, including 0
    ///
    /// Unlike [its `Float` counterpart](crate::Float::acoth_rational_prec_round), this function
    /// never overflows or underflows.
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`BigExponentFloat::acoth_rational_prec`] instead.
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
    /// let (y, o) = BigExponentFloat::acoth_rational_prec_round(Rational::from(3u32), 5, Floor);
    /// assert_eq!(y.to_string(), "0.344");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::acoth_rational_prec_round(Rational::from(3u32), 5, Ceiling);
    /// assert_eq!(y.to_string(), "0.359");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::acoth_rational_prec_round(Rational::from(3u32), 5, Nearest);
    /// assert_eq!(y.to_string(), "0.344");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::acoth_rational_prec_round(Rational::from(3u32), 20, Floor);
    /// assert_eq!(y.to_string(), "0.34657335");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::acoth_rational_prec_round(Rational::from(3u32), 20, Ceiling);
    /// assert_eq!(y.to_string(), "0.3465738");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::acoth_rational_prec_round(Rational::from(3u32), 20, Nearest);
    /// assert_eq!(y.to_string(), "0.34657335");
    /// assert_eq!(o, Less);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn acoth_rational_prec_round(x: Rational, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        inverse_hyperbolic_rational_helper(InverseHyperbolicFunction::Acoth, &x, prec, rm, true)
    }

    /// Computes the inverse hyperbolic cotangent of a [`Rational`], returning a
    /// [`BigExponentFloat`], rounding the result to the specified precision and with the specified
    /// rounding mode. The [`Rational`] is taken by reference. An [`Ordering`] is also returned,
    /// indicating whether the rounded inverse hyperbolic cotangent is less than, equal to, or
    /// greater than the exact inverse hyperbolic cotangent. Although `NaN`s are not comparable to
    /// any [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \operatorname{acoth} x+\varepsilon.
    /// $$
    /// - If $\operatorname{acoth} x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $\operatorname{acoth} x$ is finite and nonzero, and $m$ is not `Nearest`, then
    ///   $|\varepsilon| < 2^{\lfloor\log_2 |\operatorname{acoth} x|\rfloor-p+1}$.
    /// - If $\operatorname{acoth} x$ is finite and nonzero, and $m$ is `Nearest`, then
    ///   $|\varepsilon| \leq 2^{\lfloor\log_2 |\operatorname{acoth} x|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\pm1,p,m)=\pm\infty$
    /// - $f(x,p,m)=\text{NaN}$ if $|x|<1$, including 0
    ///
    /// Unlike [its `Float` counterpart](crate::Float::acoth_rational_prec_round), this function
    /// never overflows or underflows.
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
    ///     BigExponentFloat::acoth_rational_prec_round_ref(&Rational::from(3u32), 5, Floor);
    /// assert_eq!(y.to_string(), "0.344");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) =
    ///     BigExponentFloat::acoth_rational_prec_round_ref(&Rational::from(3u32), 5, Ceiling);
    /// assert_eq!(y.to_string(), "0.359");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) =
    ///     BigExponentFloat::acoth_rational_prec_round_ref(&Rational::from(3u32), 5, Nearest);
    /// assert_eq!(y.to_string(), "0.344");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) =
    ///     BigExponentFloat::acoth_rational_prec_round_ref(&Rational::from(3u32), 20, Floor);
    /// assert_eq!(y.to_string(), "0.34657335");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) =
    ///     BigExponentFloat::acoth_rational_prec_round_ref(&Rational::from(3u32), 20, Ceiling);
    /// assert_eq!(y.to_string(), "0.3465738");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) =
    ///     BigExponentFloat::acoth_rational_prec_round_ref(&Rational::from(3u32), 20, Nearest);
    /// assert_eq!(y.to_string(), "0.34657335");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn acoth_rational_prec_round_ref(
        x: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        inverse_hyperbolic_rational_helper(InverseHyperbolicFunction::Acoth, x, prec, rm, true)
    }

    /// Computes the inverse hyperbolic cotangent of a [`Rational`], returning a
    /// [`BigExponentFloat`], rounding the result to the nearest value of the specified precision.
    /// The [`Rational`] is taken by value. An [`Ordering`] is also returned, indicating whether the
    /// rounded inverse hyperbolic cotangent is less than, equal to, or greater than the exact
    /// inverse hyperbolic cotangent. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,p) = \operatorname{acoth} x+\varepsilon.
    /// $$
    /// - If $\operatorname{acoth} x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $\operatorname{acoth} x$ is finite and nonzero, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\operatorname{acoth} x|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\pm1,p)=\pm\infty$
    /// - $f(x,p)=\text{NaN}$ if $|x|<1$, including 0
    ///
    /// Unlike [its `Float` counterpart](crate::Float::acoth_rational_prec_round), this function
    /// never overflows or underflows.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`BigExponentFloat::acoth_rational_prec_round`] instead.
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
    /// let (y, o) = BigExponentFloat::acoth_rational_prec(Rational::from(3u32), 5);
    /// assert_eq!(y.to_string(), "0.344");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::acoth_rational_prec(Rational::from(3u32), 20);
    /// assert_eq!(y.to_string(), "0.34657335");
    /// assert_eq!(o, Less);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn acoth_rational_prec(x: Rational, prec: u64) -> (Self, Ordering) {
        inverse_hyperbolic_rational_helper(
            InverseHyperbolicFunction::Acoth,
            &x,
            prec,
            Nearest,
            true,
        )
    }

    /// Computes the inverse hyperbolic cotangent of a [`Rational`], returning a
    /// [`BigExponentFloat`], rounding the result to the nearest value of the specified precision.
    /// The [`Rational`] is taken by reference. An [`Ordering`] is also returned, indicating whether
    /// the rounded inverse hyperbolic cotangent is less than, equal to, or greater than the exact
    /// inverse hyperbolic cotangent. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,p) = \operatorname{acoth} x+\varepsilon.
    /// $$
    /// - If $\operatorname{acoth} x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $\operatorname{acoth} x$ is finite and nonzero, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\operatorname{acoth} x|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\pm1,p)=\pm\infty$
    /// - $f(x,p)=\text{NaN}$ if $|x|<1$, including 0
    ///
    /// Unlike [its `Float` counterpart](crate::Float::acoth_rational_prec_round), this function
    /// never overflows or underflows.
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
    /// let (y, o) = BigExponentFloat::acoth_rational_prec_ref(&Rational::from(3u32), 5);
    /// assert_eq!(y.to_string(), "0.344");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::acoth_rational_prec_ref(&Rational::from(3u32), 20);
    /// assert_eq!(y.to_string(), "0.34657335");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn acoth_rational_prec_ref(x: &Rational, prec: u64) -> (Self, Ordering) {
        inverse_hyperbolic_rational_helper(InverseHyperbolicFunction::Acoth, x, prec, Nearest, true)
    }
}
