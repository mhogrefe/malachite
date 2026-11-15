// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::big_exponent_float::BigExponentFloat;
use crate::big_exponent_float::trigonometric::atan::{
    InverseTrigFunction, inverse_trig_helper, inverse_trig_rational_helper,
};
use core::cmp::Ordering;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_q::Rational;

impl BigExponentFloat {
    /// Computes $\operatorname{arccot} x$, the arccotangent of a [`BigExponentFloat`], rounding the
    /// result to the specified precision and with the specified rounding mode. The
    /// [`BigExponentFloat`] is taken by value. An [`Ordering`] is also returned, indicating whether
    /// the rounded arccotangent is less than, equal to, or greater than the exact arccotangent.
    /// Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \operatorname{arccot} x+\varepsilon.
    /// $$
    /// - If $\operatorname{arccot} x$ is zero, or the result is `NaN`, $\varepsilon$ may be ignored
    ///   or assumed to be 0.
    /// - If $\operatorname{arccot} x$ is nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\operatorname{arccot} x|\rfloor-p+1}$.
    /// - If $\operatorname{arccot} x$ is nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\operatorname{arccot} x|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=\text{NaN}$
    /// - $f(\pm\infty,p,m)=0$
    /// - $f(0,p,m)=\pi/2$, rounded
    /// - Since a [`BigExponentFloat`] has no negative zero, its zero is treated as $+0$ (as FLINT's
    ///   `arb_atan2` treats it), where the `Float` function gives $\pm\pi/2$ by the sign of the
    ///   zero
    ///
    /// Unlike [its `Float` counterpart](crate::Float::acot_prec_round), this function accepts
    /// arguments with any exponent, and its result never underflows.
    ///
    /// If you know you'll be using `Nearest`, consider using [`BigExponentFloat::acot_prec`]
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the result is nonzero (every nonzero
    /// result is irrational).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (y, o) = BigExponentFloat::from(0.375).acot_prec_round(5, Floor);
    /// assert_eq!(y.to_string(), "1.19");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::from(0.375).acot_prec_round(5, Ceiling);
    /// assert_eq!(y.to_string(), "1.2");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::from(0.375).acot_prec_round(5, Nearest);
    /// assert_eq!(y.to_string(), "1.19");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::from(0.375).acot_prec_round(20, Floor);
    /// assert_eq!(y.to_string(), "1.212025");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::from(0.375).acot_prec_round(20, Ceiling);
    /// assert_eq!(y.to_string(), "1.2120266");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::from(0.375).acot_prec_round(20, Nearest);
    /// assert_eq!(y.to_string(), "1.2120266");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn acot_prec_round(self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        inverse_trig_helper(InverseTrigFunction::Acot, &self, None, prec, rm, true)
    }

    /// Computes $\operatorname{arccot} x$, the arccotangent of a [`BigExponentFloat`], rounding the
    /// result to the specified precision and with the specified rounding mode. The
    /// [`BigExponentFloat`] is taken by reference. An [`Ordering`] is also returned, indicating
    /// whether the rounded arccotangent is less than, equal to, or greater than the exact
    /// arccotangent. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \operatorname{arccot} x+\varepsilon.
    /// $$
    /// - If $\operatorname{arccot} x$ is zero, or the result is `NaN`, $\varepsilon$ may be ignored
    ///   or assumed to be 0.
    /// - If $\operatorname{arccot} x$ is nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\operatorname{arccot} x|\rfloor-p+1}$.
    /// - If $\operatorname{arccot} x$ is nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\operatorname{arccot} x|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=\text{NaN}$
    /// - $f(\pm\infty,p,m)=0$
    /// - $f(0,p,m)=\pi/2$, rounded
    /// - Since a [`BigExponentFloat`] has no negative zero, its zero is treated as $+0$ (as FLINT's
    ///   `arb_atan2` treats it), where the `Float` function gives $\pm\pi/2$ by the sign of the
    ///   zero
    ///
    /// Unlike [its `Float` counterpart](crate::Float::acot_prec_round), this function accepts
    /// arguments with any exponent, and its result never underflows.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the result is nonzero (every nonzero
    /// result is irrational).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (y, o) = (&BigExponentFloat::from(0.375)).acot_prec_round_ref(5, Floor);
    /// assert_eq!(y.to_string(), "1.19");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = (&BigExponentFloat::from(0.375)).acot_prec_round_ref(5, Ceiling);
    /// assert_eq!(y.to_string(), "1.2");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = (&BigExponentFloat::from(0.375)).acot_prec_round_ref(5, Nearest);
    /// assert_eq!(y.to_string(), "1.19");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = (&BigExponentFloat::from(0.375)).acot_prec_round_ref(20, Floor);
    /// assert_eq!(y.to_string(), "1.212025");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = (&BigExponentFloat::from(0.375)).acot_prec_round_ref(20, Ceiling);
    /// assert_eq!(y.to_string(), "1.2120266");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = (&BigExponentFloat::from(0.375)).acot_prec_round_ref(20, Nearest);
    /// assert_eq!(y.to_string(), "1.2120266");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn acot_prec_round_ref(&self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        inverse_trig_helper(InverseTrigFunction::Acot, self, None, prec, rm, true)
    }

    /// Replaces a [`BigExponentFloat`] with $\operatorname{arccot} x$, its arccotangent, rounding
    /// the result to the specified precision and with the specified rounding mode. An [`Ordering`]
    /// is returned, indicating whether the rounded arccotangent is less than, equal to, or greater
    /// than the exact arccotangent. Although `NaN`s are not comparable to any [`BigExponentFloat`],
    /// whenever this function sets the [`BigExponentFloat`] to `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \operatorname{arccot} x+\varepsilon.
    /// $$
    /// - If $\operatorname{arccot} x$ is zero, or the result is `NaN`, $\varepsilon$ may be ignored
    ///   or assumed to be 0.
    /// - If $\operatorname{arccot} x$ is nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\operatorname{arccot} x|\rfloor-p+1}$.
    /// - If $\operatorname{arccot} x$ is nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\operatorname{arccot} x|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=\text{NaN}$
    /// - $f(\pm\infty,p,m)=0$
    /// - $f(0,p,m)=\pi/2$, rounded
    /// - Since a [`BigExponentFloat`] has no negative zero, its zero is treated as $+0$ (as FLINT's
    ///   `arb_atan2` treats it), where the `Float` function gives $\pm\pi/2$ by the sign of the
    ///   zero
    ///
    /// Unlike [its `Float` counterpart](crate::Float::acot_prec_round), this function accepts
    /// arguments with any exponent, and its result never underflows.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the result is nonzero (every nonzero
    /// result is irrational).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = BigExponentFloat::from(0.375);
    /// assert_eq!(x.acot_prec_round_assign(5, Floor), Less);
    /// assert_eq!(x.to_string(), "1.19");
    ///
    /// let mut x = BigExponentFloat::from(0.375);
    /// assert_eq!(x.acot_prec_round_assign(5, Ceiling), Greater);
    /// assert_eq!(x.to_string(), "1.2");
    ///
    /// let mut x = BigExponentFloat::from(0.375);
    /// assert_eq!(x.acot_prec_round_assign(5, Nearest), Less);
    /// assert_eq!(x.to_string(), "1.19");
    ///
    /// let mut x = BigExponentFloat::from(0.375);
    /// assert_eq!(x.acot_prec_round_assign(20, Floor), Less);
    /// assert_eq!(x.to_string(), "1.212025");
    ///
    /// let mut x = BigExponentFloat::from(0.375);
    /// assert_eq!(x.acot_prec_round_assign(20, Ceiling), Greater);
    /// assert_eq!(x.to_string(), "1.2120266");
    ///
    /// let mut x = BigExponentFloat::from(0.375);
    /// assert_eq!(x.acot_prec_round_assign(20, Nearest), Greater);
    /// assert_eq!(x.to_string(), "1.2120266");
    /// ```
    #[inline]
    pub fn acot_prec_round_assign(&mut self, prec: u64, rm: RoundingMode) -> Ordering {
        let (result, o) =
            inverse_trig_helper(InverseTrigFunction::Acot, self, None, prec, rm, true);
        *self = result;
        o
    }

    /// Computes $\operatorname{arccot} x$, the arccotangent of a [`BigExponentFloat`], rounding the
    /// result to the nearest value of the specified precision. The [`BigExponentFloat`] is taken by
    /// value. An [`Ordering`] is also returned, indicating whether the rounded arccotangent is less
    /// than, equal to, or greater than the exact arccotangent. Although `NaN`s are not comparable
    /// to any [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,p) = \operatorname{arccot} x+\varepsilon.
    /// $$
    /// - If $\operatorname{arccot} x$ is zero, or the result is `NaN`, $\varepsilon$ may be ignored
    ///   or assumed to be 0.
    /// - If $\operatorname{arccot} x$ is nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\operatorname{arccot} x|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p)=\text{NaN}$
    /// - $f(\pm\infty,p)=0$
    /// - $f(0,p)=\pi/2$, rounded
    /// - Since a [`BigExponentFloat`] has no negative zero, its zero is treated as $+0$ (as FLINT's
    ///   `arb_atan2` treats it), where the `Float` function gives $\pm\pi/2$ by the sign of the
    ///   zero
    ///
    /// Unlike [its `Float` counterpart](crate::Float::acot_prec_round), this function accepts
    /// arguments with any exponent, and its result never underflows.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`BigExponentFloat::acot_prec_round`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (y, o) = BigExponentFloat::from(0.375).acot_prec(5);
    /// assert_eq!(y.to_string(), "1.19");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::from(0.375).acot_prec(20);
    /// assert_eq!(y.to_string(), "1.2120266");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn acot_prec(self, prec: u64) -> (Self, Ordering) {
        inverse_trig_helper(InverseTrigFunction::Acot, &self, None, prec, Nearest, true)
    }

    /// Computes $\operatorname{arccot} x$, the arccotangent of a [`BigExponentFloat`], rounding the
    /// result to the nearest value of the specified precision. The [`BigExponentFloat`] is taken by
    /// reference. An [`Ordering`] is also returned, indicating whether the rounded arccotangent is
    /// less than, equal to, or greater than the exact arccotangent. Although `NaN`s are not
    /// comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// $$
    /// f(x,p) = \operatorname{arccot} x+\varepsilon.
    /// $$
    /// - If $\operatorname{arccot} x$ is zero, or the result is `NaN`, $\varepsilon$ may be ignored
    ///   or assumed to be 0.
    /// - If $\operatorname{arccot} x$ is nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\operatorname{arccot} x|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p)=\text{NaN}$
    /// - $f(\pm\infty,p)=0$
    /// - $f(0,p)=\pi/2$, rounded
    /// - Since a [`BigExponentFloat`] has no negative zero, its zero is treated as $+0$ (as FLINT's
    ///   `arb_atan2` treats it), where the `Float` function gives $\pm\pi/2$ by the sign of the
    ///   zero
    ///
    /// Unlike [its `Float` counterpart](crate::Float::acot_prec_round), this function accepts
    /// arguments with any exponent, and its result never underflows.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (y, o) = (&BigExponentFloat::from(0.375)).acot_prec_ref(5);
    /// assert_eq!(y.to_string(), "1.19");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = (&BigExponentFloat::from(0.375)).acot_prec_ref(20);
    /// assert_eq!(y.to_string(), "1.2120266");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn acot_prec_ref(&self, prec: u64) -> (Self, Ordering) {
        inverse_trig_helper(InverseTrigFunction::Acot, self, None, prec, Nearest, true)
    }

    /// Replaces a [`BigExponentFloat`] with $\operatorname{arccot} x$, its arccotangent, rounding
    /// the result to the nearest value of the specified precision. An [`Ordering`] is returned,
    /// indicating whether the rounded arccotangent is less than, equal to, or greater than the
    /// exact arccotangent. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever
    /// this function sets the [`BigExponentFloat`] to `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,p) = \operatorname{arccot} x+\varepsilon.
    /// $$
    /// - If $\operatorname{arccot} x$ is zero, or the result is `NaN`, $\varepsilon$ may be ignored
    ///   or assumed to be 0.
    /// - If $\operatorname{arccot} x$ is nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\operatorname{arccot} x|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p)=\text{NaN}$
    /// - $f(\pm\infty,p)=0$
    /// - $f(0,p)=\pi/2$, rounded
    /// - Since a [`BigExponentFloat`] has no negative zero, its zero is treated as $+0$ (as FLINT's
    ///   `arb_atan2` treats it), where the `Float` function gives $\pm\pi/2$ by the sign of the
    ///   zero
    ///
    /// Unlike [its `Float` counterpart](crate::Float::acot_prec_round), this function accepts
    /// arguments with any exponent, and its result never underflows.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = BigExponentFloat::from(0.375);
    /// assert_eq!(x.acot_prec_assign(5), Less);
    /// assert_eq!(x.to_string(), "1.19");
    ///
    /// let mut x = BigExponentFloat::from(0.375);
    /// assert_eq!(x.acot_prec_assign(20), Greater);
    /// assert_eq!(x.to_string(), "1.2120266");
    /// ```
    #[inline]
    pub fn acot_prec_assign(&mut self, prec: u64) -> Ordering {
        let (result, o) =
            inverse_trig_helper(InverseTrigFunction::Acot, self, None, prec, Nearest, true);
        *self = result;
        o
    }

    /// Computes $\operatorname{arccot} x$, the arccotangent of a [`Rational`], returning a
    /// [`BigExponentFloat`], rounding the result to the specified precision and with the specified
    /// rounding mode. The [`Rational`] is taken by value. An [`Ordering`] is also returned,
    /// indicating whether the rounded arccotangent is less than, equal to, or greater than the
    /// exact arccotangent. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever
    /// this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \operatorname{arccot} x+\varepsilon.
    /// $$
    /// - If $\operatorname{arccot} x$ is zero, or the result is `NaN`, $\varepsilon$ may be ignored
    ///   or assumed to be 0.
    /// - If $\operatorname{arccot} x$ is nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\operatorname{arccot} x|\rfloor-p+1}$.
    /// - If $\operatorname{arccot} x$ is nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\operatorname{arccot} x|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(0,p,m)=\pi/2$, rounded
    /// - Since a [`BigExponentFloat`] has no negative zero, its zero is treated as $+0$ (as FLINT's
    ///   `arb_atan2` treats it), where the `Float` function gives $\pm\pi/2$ by the sign of the
    ///   zero
    ///
    /// Unlike [its `Float` counterpart](crate::Float::acot_rational_prec_round), this function
    /// never underflows.
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`BigExponentFloat::acot_rational_prec`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `x.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the result is nonzero (every nonzero
    /// result is irrational).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (y, o) =
    ///     BigExponentFloat::acot_rational_prec_round(Rational::from_unsigneds(1u8, 3), 5, Floor);
    /// assert_eq!(y.to_string(), "1.19");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::acot_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(y.to_string(), "1.2");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::acot_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(y.to_string(), "1.2");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) =
    ///     BigExponentFloat::acot_rational_prec_round(Rational::from_unsigneds(1u8, 3), 20, Floor);
    /// assert_eq!(y.to_string(), "1.2490444");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::acot_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(y.to_string(), "1.249046");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::acot_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(y.to_string(), "1.249046");
    /// assert_eq!(o, Greater);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn acot_rational_prec_round(x: Rational, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        inverse_trig_rational_helper(InverseTrigFunction::Acot, &x, None, prec, rm)
    }

    /// Computes $\operatorname{arccot} x$, the arccotangent of a [`Rational`], returning a
    /// [`BigExponentFloat`], rounding the result to the specified precision and with the specified
    /// rounding mode. The [`Rational`] is taken by reference. An [`Ordering`] is also returned,
    /// indicating whether the rounded arccotangent is less than, equal to, or greater than the
    /// exact arccotangent. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever
    /// this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \operatorname{arccot} x+\varepsilon.
    /// $$
    /// - If $\operatorname{arccot} x$ is zero, or the result is `NaN`, $\varepsilon$ may be ignored
    ///   or assumed to be 0.
    /// - If $\operatorname{arccot} x$ is nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\operatorname{arccot} x|\rfloor-p+1}$.
    /// - If $\operatorname{arccot} x$ is nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\operatorname{arccot} x|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(0,p,m)=\pi/2$, rounded
    /// - Since a [`BigExponentFloat`] has no negative zero, its zero is treated as $+0$ (as FLINT's
    ///   `arb_atan2` treats it), where the `Float` function gives $\pm\pi/2$ by the sign of the
    ///   zero
    ///
    /// Unlike [its `Float` counterpart](crate::Float::acot_rational_prec_round), this function
    /// never underflows.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `x.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the result is nonzero (every nonzero
    /// result is irrational).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (y, o) = BigExponentFloat::acot_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(y.to_string(), "1.19");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::acot_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(y.to_string(), "1.2");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::acot_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(y.to_string(), "1.2");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::acot_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(y.to_string(), "1.2490444");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::acot_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(y.to_string(), "1.249046");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::acot_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(y.to_string(), "1.249046");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn acot_rational_prec_round_ref(
        x: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        inverse_trig_rational_helper(InverseTrigFunction::Acot, x, None, prec, rm)
    }

    /// Computes $\operatorname{arccot} x$, the arccotangent of a [`Rational`], returning a
    /// [`BigExponentFloat`], rounding the result to the nearest value of the specified precision.
    /// The [`Rational`] is taken by value. An [`Ordering`] is also returned, indicating whether the
    /// rounded arccotangent is less than, equal to, or greater than the exact arccotangent.
    /// Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,p) = \operatorname{arccot} x+\varepsilon.
    /// $$
    /// - If $\operatorname{arccot} x$ is zero, or the result is `NaN`, $\varepsilon$ may be ignored
    ///   or assumed to be 0.
    /// - If $\operatorname{arccot} x$ is nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\operatorname{arccot} x|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(0,p)=\pi/2$, rounded
    /// - Since a [`BigExponentFloat`] has no negative zero, its zero is treated as $+0$ (as FLINT's
    ///   `arb_atan2` treats it), where the `Float` function gives $\pm\pi/2$ by the sign of the
    ///   zero
    ///
    /// Unlike [its `Float` counterpart](crate::Float::acot_rational_prec_round), this function
    /// never underflows.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`BigExponentFloat::acot_rational_prec_round`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
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
    /// let (y, o) = BigExponentFloat::acot_rational_prec(Rational::from_unsigneds(1u8, 3), 5);
    /// assert_eq!(y.to_string(), "1.2");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::acot_rational_prec(Rational::from_unsigneds(1u8, 3), 20);
    /// assert_eq!(y.to_string(), "1.249046");
    /// assert_eq!(o, Greater);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn acot_rational_prec(x: Rational, prec: u64) -> (Self, Ordering) {
        inverse_trig_rational_helper(InverseTrigFunction::Acot, &x, None, prec, Nearest)
    }

    /// Computes $\operatorname{arccot} x$, the arccotangent of a [`Rational`], returning a
    /// [`BigExponentFloat`], rounding the result to the nearest value of the specified precision.
    /// The [`Rational`] is taken by reference. An [`Ordering`] is also returned, indicating whether
    /// the rounded arccotangent is less than, equal to, or greater than the exact arccotangent.
    /// Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,p) = \operatorname{arccot} x+\varepsilon.
    /// $$
    /// - If $\operatorname{arccot} x$ is zero, or the result is `NaN`, $\varepsilon$ may be ignored
    ///   or assumed to be 0.
    /// - If $\operatorname{arccot} x$ is nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\operatorname{arccot} x|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(0,p)=\pi/2$, rounded
    /// - Since a [`BigExponentFloat`] has no negative zero, its zero is treated as $+0$ (as FLINT's
    ///   `arb_atan2` treats it), where the `Float` function gives $\pm\pi/2$ by the sign of the
    ///   zero
    ///
    /// Unlike [its `Float` counterpart](crate::Float::acot_rational_prec_round), this function
    /// never underflows.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
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
    /// let (y, o) = BigExponentFloat::acot_rational_prec_ref(&Rational::from_unsigneds(1u8, 3), 5);
    /// assert_eq!(y.to_string(), "1.2");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) =
    ///     BigExponentFloat::acot_rational_prec_ref(&Rational::from_unsigneds(1u8, 3), 20);
    /// assert_eq!(y.to_string(), "1.249046");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn acot_rational_prec_ref(x: &Rational, prec: u64) -> (Self, Ordering) {
        inverse_trig_rational_helper(InverseTrigFunction::Acot, x, None, prec, Nearest)
    }

    /// Computes $u\operatorname{arccot} x/(2\pi)$, the arccotangent of a [`BigExponentFloat`]
    /// measured in $u$ths of a turn, rounding the result to the specified precision and with the
    /// specified rounding mode. The [`BigExponentFloat`] is taken by value. An [`Ordering`] is also
    /// returned, indicating whether the rounded arccotangent is less than, equal to, or greater
    /// than the exact arccotangent. Although `NaN`s are not comparable to any [`BigExponentFloat`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,u,p,m) = u\operatorname{arccot} x/(2\pi)+\varepsilon.
    /// $$
    /// - If $u\operatorname{arccot} x/(2\pi)$ is zero, or the result is `NaN`, $\varepsilon$ may be
    ///   ignored or assumed to be 0.
    /// - If $u\operatorname{arccot} x/(2\pi)$ is nonzero, and $m$ is not `Nearest`, then
    ///   $|\varepsilon| < 2^{\lfloor\log_2 |u\operatorname{arccot} x/(2\pi)|\rfloor-p+1}$.
    /// - If $u\operatorname{arccot} x/(2\pi)$ is nonzero, and $m$ is `Nearest`, then $|\varepsilon|
    ///   \leq 2^{\lfloor\log_2 |u\operatorname{arccot} x/(2\pi)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},u,p,m)=\text{NaN}$
    /// - $f(\pm\infty,u,p,m)=0$
    /// - $f(0,u,p,m)=u/4$
    /// - $f(x,0,p,m)=0$ for $x$ in the domain
    /// - the results $f(\pm1,\ldots)=\pmu/8$ are exact when representable with `prec` bits, and
    ///   rounded otherwise
    /// - Since a [`BigExponentFloat`] has no negative zero, its zero is treated as $+0$ (as FLINT's
    ///   `arb_atan2` treats it), where the `Float` function gives $\pmu/4$ by the sign of the zero
    ///
    /// Unlike [its `Float` counterpart](crate::Float::acot_with_period_prec_round), this function
    /// accepts arguments with any exponent, and its result never underflows.
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`BigExponentFloat::acot_with_period_prec`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (y, o) = BigExponentFloat::from(0.375).acot_with_period_prec_round(7, 5, Floor);
    /// assert_eq!(y.to_string(), "1.31");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::from(0.375).acot_with_period_prec_round(7, 5, Ceiling);
    /// assert_eq!(y.to_string(), "1.38");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::from(0.375).acot_with_period_prec_round(7, 5, Nearest);
    /// assert_eq!(y.to_string(), "1.38");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::from(0.375).acot_with_period_prec_round(7, 20, Floor);
    /// assert_eq!(y.to_string(), "1.3502979");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::from(0.375).acot_with_period_prec_round(7, 20, Ceiling);
    /// assert_eq!(y.to_string(), "1.350300");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::from(0.375).acot_with_period_prec_round(7, 20, Nearest);
    /// assert_eq!(y.to_string(), "1.350300");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn acot_with_period_prec_round(
        self,
        u: u64,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        inverse_trig_helper(InverseTrigFunction::Acot, &self, Some(u), prec, rm, true)
    }

    /// Computes $u\operatorname{arccot} x/(2\pi)$, the arccotangent of a [`BigExponentFloat`]
    /// measured in $u$ths of a turn, rounding the result to the specified precision and with the
    /// specified rounding mode. The [`BigExponentFloat`] is taken by reference. An [`Ordering`] is
    /// also returned, indicating whether the rounded arccotangent is less than, equal to, or
    /// greater than the exact arccotangent. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,u,p,m) = u\operatorname{arccot} x/(2\pi)+\varepsilon.
    /// $$
    /// - If $u\operatorname{arccot} x/(2\pi)$ is zero, or the result is `NaN`, $\varepsilon$ may be
    ///   ignored or assumed to be 0.
    /// - If $u\operatorname{arccot} x/(2\pi)$ is nonzero, and $m$ is not `Nearest`, then
    ///   $|\varepsilon| < 2^{\lfloor\log_2 |u\operatorname{arccot} x/(2\pi)|\rfloor-p+1}$.
    /// - If $u\operatorname{arccot} x/(2\pi)$ is nonzero, and $m$ is `Nearest`, then $|\varepsilon|
    ///   \leq 2^{\lfloor\log_2 |u\operatorname{arccot} x/(2\pi)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},u,p,m)=\text{NaN}$
    /// - $f(\pm\infty,u,p,m)=0$
    /// - $f(0,u,p,m)=u/4$
    /// - $f(x,0,p,m)=0$ for $x$ in the domain
    /// - the results $f(\pm1,\ldots)=\pmu/8$ are exact when representable with `prec` bits, and
    ///   rounded otherwise
    /// - Since a [`BigExponentFloat`] has no negative zero, its zero is treated as $+0$ (as FLINT's
    ///   `arb_atan2` treats it), where the `Float` function gives $\pmu/4$ by the sign of the zero
    ///
    /// Unlike [its `Float` counterpart](crate::Float::acot_with_period_prec_round), this function
    /// accepts arguments with any exponent, and its result never underflows.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (y, o) = (&BigExponentFloat::from(0.375)).acot_with_period_prec_round_ref(7, 5, Floor);
    /// assert_eq!(y.to_string(), "1.31");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) =
    ///     (&BigExponentFloat::from(0.375)).acot_with_period_prec_round_ref(7, 5, Ceiling);
    /// assert_eq!(y.to_string(), "1.38");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) =
    ///     (&BigExponentFloat::from(0.375)).acot_with_period_prec_round_ref(7, 5, Nearest);
    /// assert_eq!(y.to_string(), "1.38");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = (&BigExponentFloat::from(0.375)).acot_with_period_prec_round_ref(7, 20, Floor);
    /// assert_eq!(y.to_string(), "1.3502979");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) =
    ///     (&BigExponentFloat::from(0.375)).acot_with_period_prec_round_ref(7, 20, Ceiling);
    /// assert_eq!(y.to_string(), "1.350300");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) =
    ///     (&BigExponentFloat::from(0.375)).acot_with_period_prec_round_ref(7, 20, Nearest);
    /// assert_eq!(y.to_string(), "1.350300");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn acot_with_period_prec_round_ref(
        &self,
        u: u64,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        inverse_trig_helper(InverseTrigFunction::Acot, self, Some(u), prec, rm, true)
    }

    /// Replaces a [`BigExponentFloat`] with $u\operatorname{arccot} x/(2\pi)$, its arccotangent
    /// measured in $u$ths of a turn, rounding the result to the specified precision and with the
    /// specified rounding mode. An [`Ordering`] is returned, indicating whether the rounded
    /// arccotangent is less than, equal to, or greater than the exact arccotangent. Although `NaN`s
    /// are not comparable to any [`BigExponentFloat`], whenever this function sets the
    /// [`BigExponentFloat`] to `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,u,p,m) = u\operatorname{arccot} x/(2\pi)+\varepsilon.
    /// $$
    /// - If $u\operatorname{arccot} x/(2\pi)$ is zero, or the result is `NaN`, $\varepsilon$ may be
    ///   ignored or assumed to be 0.
    /// - If $u\operatorname{arccot} x/(2\pi)$ is nonzero, and $m$ is not `Nearest`, then
    ///   $|\varepsilon| < 2^{\lfloor\log_2 |u\operatorname{arccot} x/(2\pi)|\rfloor-p+1}$.
    /// - If $u\operatorname{arccot} x/(2\pi)$ is nonzero, and $m$ is `Nearest`, then $|\varepsilon|
    ///   \leq 2^{\lfloor\log_2 |u\operatorname{arccot} x/(2\pi)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},u,p,m)=\text{NaN}$
    /// - $f(\pm\infty,u,p,m)=0$
    /// - $f(0,u,p,m)=u/4$
    /// - $f(x,0,p,m)=0$ for $x$ in the domain
    /// - the results $f(\pm1,\ldots)=\pmu/8$ are exact when representable with `prec` bits, and
    ///   rounded otherwise
    /// - Since a [`BigExponentFloat`] has no negative zero, its zero is treated as $+0$ (as FLINT's
    ///   `arb_atan2` treats it), where the `Float` function gives $\pmu/4$ by the sign of the zero
    ///
    /// Unlike [its `Float` counterpart](crate::Float::acot_with_period_prec_round), this function
    /// accepts arguments with any exponent, and its result never underflows.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = BigExponentFloat::from(0.375);
    /// assert_eq!(x.acot_with_period_prec_round_assign(7, 5, Floor), Less);
    /// assert_eq!(x.to_string(), "1.31");
    ///
    /// let mut x = BigExponentFloat::from(0.375);
    /// assert_eq!(x.acot_with_period_prec_round_assign(7, 5, Ceiling), Greater);
    /// assert_eq!(x.to_string(), "1.38");
    ///
    /// let mut x = BigExponentFloat::from(0.375);
    /// assert_eq!(x.acot_with_period_prec_round_assign(7, 5, Nearest), Greater);
    /// assert_eq!(x.to_string(), "1.38");
    ///
    /// let mut x = BigExponentFloat::from(0.375);
    /// assert_eq!(x.acot_with_period_prec_round_assign(7, 20, Floor), Less);
    /// assert_eq!(x.to_string(), "1.3502979");
    ///
    /// let mut x = BigExponentFloat::from(0.375);
    /// assert_eq!(
    ///     x.acot_with_period_prec_round_assign(7, 20, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "1.350300");
    ///
    /// let mut x = BigExponentFloat::from(0.375);
    /// assert_eq!(
    ///     x.acot_with_period_prec_round_assign(7, 20, Nearest),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "1.350300");
    /// ```
    #[inline]
    pub fn acot_with_period_prec_round_assign(
        &mut self,
        u: u64,
        prec: u64,
        rm: RoundingMode,
    ) -> Ordering {
        let (result, o) =
            inverse_trig_helper(InverseTrigFunction::Acot, self, Some(u), prec, rm, true);
        *self = result;
        o
    }

    /// Computes $u\operatorname{arccot} x/(2\pi)$, the arccotangent of a [`BigExponentFloat`]
    /// measured in $u$ths of a turn, rounding the result to the nearest value of the specified
    /// precision. The [`BigExponentFloat`] is taken by value. An [`Ordering`] is also returned,
    /// indicating whether the rounded arccotangent is less than, equal to, or greater than the
    /// exact arccotangent. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever
    /// this function returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,u,p) = u\operatorname{arccot} x/(2\pi)+\varepsilon.
    /// $$
    /// - If $u\operatorname{arccot} x/(2\pi)$ is zero, or the result is `NaN`, $\varepsilon$ may be
    ///   ignored or assumed to be 0.
    /// - If $u\operatorname{arccot} x/(2\pi)$ is nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |u\operatorname{arccot} x/(2\pi)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},u,p)=\text{NaN}$
    /// - $f(\pm\infty,u,p)=0$
    /// - $f(0,u,p)=u/4$
    /// - $f(x,0,p)=0$ for $x$ in the domain
    /// - the results $f(\pm1,\ldots)=\pmu/8$ are exact when representable with `prec` bits, and
    ///   rounded otherwise
    /// - Since a [`BigExponentFloat`] has no negative zero, its zero is treated as $+0$ (as FLINT's
    ///   `arb_atan2` treats it), where the `Float` function gives $\pmu/4$ by the sign of the zero
    ///
    /// Unlike [its `Float` counterpart](crate::Float::acot_with_period_prec_round), this function
    /// accepts arguments with any exponent, and its result never underflows.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`BigExponentFloat::acot_with_period_prec_round`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (y, o) = BigExponentFloat::from(0.375).acot_with_period_prec(7, 5);
    /// assert_eq!(y.to_string(), "1.38");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::from(0.375).acot_with_period_prec(7, 20);
    /// assert_eq!(y.to_string(), "1.350300");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn acot_with_period_prec(self, u: u64, prec: u64) -> (Self, Ordering) {
        inverse_trig_helper(
            InverseTrigFunction::Acot,
            &self,
            Some(u),
            prec,
            Nearest,
            true,
        )
    }

    /// Computes $u\operatorname{arccot} x/(2\pi)$, the arccotangent of a [`BigExponentFloat`]
    /// measured in $u$ths of a turn, rounding the result to the nearest value of the specified
    /// precision. The [`BigExponentFloat`] is taken by reference. An [`Ordering`] is also returned,
    /// indicating whether the rounded arccotangent is less than, equal to, or greater than the
    /// exact arccotangent. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever
    /// this function returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,u,p) = u\operatorname{arccot} x/(2\pi)+\varepsilon.
    /// $$
    /// - If $u\operatorname{arccot} x/(2\pi)$ is zero, or the result is `NaN`, $\varepsilon$ may be
    ///   ignored or assumed to be 0.
    /// - If $u\operatorname{arccot} x/(2\pi)$ is nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |u\operatorname{arccot} x/(2\pi)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},u,p)=\text{NaN}$
    /// - $f(\pm\infty,u,p)=0$
    /// - $f(0,u,p)=u/4$
    /// - $f(x,0,p)=0$ for $x$ in the domain
    /// - the results $f(\pm1,\ldots)=\pmu/8$ are exact when representable with `prec` bits, and
    ///   rounded otherwise
    /// - Since a [`BigExponentFloat`] has no negative zero, its zero is treated as $+0$ (as FLINT's
    ///   `arb_atan2` treats it), where the `Float` function gives $\pmu/4$ by the sign of the zero
    ///
    /// Unlike [its `Float` counterpart](crate::Float::acot_with_period_prec_round), this function
    /// accepts arguments with any exponent, and its result never underflows.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (y, o) = (&BigExponentFloat::from(0.375)).acot_with_period_prec_ref(7, 5);
    /// assert_eq!(y.to_string(), "1.38");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = (&BigExponentFloat::from(0.375)).acot_with_period_prec_ref(7, 20);
    /// assert_eq!(y.to_string(), "1.350300");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn acot_with_period_prec_ref(&self, u: u64, prec: u64) -> (Self, Ordering) {
        inverse_trig_helper(
            InverseTrigFunction::Acot,
            self,
            Some(u),
            prec,
            Nearest,
            true,
        )
    }

    /// Replaces a [`BigExponentFloat`] with $u\operatorname{arccot} x/(2\pi)$, its arccotangent
    /// measured in $u$ths of a turn, rounding the result to the nearest value of the specified
    /// precision. An [`Ordering`] is returned, indicating whether the rounded arccotangent is less
    /// than, equal to, or greater than the exact arccotangent. Although `NaN`s are not comparable
    /// to any [`BigExponentFloat`], whenever this function sets the [`BigExponentFloat`] to `NaN`
    /// it also returns `Equal`.
    ///
    /// $$
    /// f(x,u,p) = u\operatorname{arccot} x/(2\pi)+\varepsilon.
    /// $$
    /// - If $u\operatorname{arccot} x/(2\pi)$ is zero, or the result is `NaN`, $\varepsilon$ may be
    ///   ignored or assumed to be 0.
    /// - If $u\operatorname{arccot} x/(2\pi)$ is nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |u\operatorname{arccot} x/(2\pi)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},u,p)=\text{NaN}$
    /// - $f(\pm\infty,u,p)=0$
    /// - $f(0,u,p)=u/4$
    /// - $f(x,0,p)=0$ for $x$ in the domain
    /// - the results $f(\pm1,\ldots)=\pmu/8$ are exact when representable with `prec` bits, and
    ///   rounded otherwise
    /// - Since a [`BigExponentFloat`] has no negative zero, its zero is treated as $+0$ (as FLINT's
    ///   `arb_atan2` treats it), where the `Float` function gives $\pmu/4$ by the sign of the zero
    ///
    /// Unlike [its `Float` counterpart](crate::Float::acot_with_period_prec_round), this function
    /// accepts arguments with any exponent, and its result never underflows.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = BigExponentFloat::from(0.375);
    /// assert_eq!(x.acot_with_period_prec_assign(7, 5), Greater);
    /// assert_eq!(x.to_string(), "1.38");
    ///
    /// let mut x = BigExponentFloat::from(0.375);
    /// assert_eq!(x.acot_with_period_prec_assign(7, 20), Greater);
    /// assert_eq!(x.to_string(), "1.350300");
    /// ```
    #[inline]
    pub fn acot_with_period_prec_assign(&mut self, u: u64, prec: u64) -> Ordering {
        let (result, o) = inverse_trig_helper(
            InverseTrigFunction::Acot,
            self,
            Some(u),
            prec,
            Nearest,
            true,
        );
        *self = result;
        o
    }

    /// Computes $u\operatorname{arccot} x/(2\pi)$, the arccotangent of a [`Rational`] measured in
    /// $u$ths of a turn, returning a [`BigExponentFloat`], rounding the result to the specified
    /// precision and with the specified rounding mode. The [`Rational`] is taken by value. An
    /// [`Ordering`] is also returned, indicating whether the rounded arccotangent is less than,
    /// equal to, or greater than the exact arccotangent. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,u,p,m) = u\operatorname{arccot} x/(2\pi)+\varepsilon.
    /// $$
    /// - If $u\operatorname{arccot} x/(2\pi)$ is zero, or the result is `NaN`, $\varepsilon$ may be
    ///   ignored or assumed to be 0.
    /// - If $u\operatorname{arccot} x/(2\pi)$ is nonzero, and $m$ is not `Nearest`, then
    ///   $|\varepsilon| < 2^{\lfloor\log_2 |u\operatorname{arccot} x/(2\pi)|\rfloor-p+1}$.
    /// - If $u\operatorname{arccot} x/(2\pi)$ is nonzero, and $m$ is `Nearest`, then $|\varepsilon|
    ///   \leq 2^{\lfloor\log_2 |u\operatorname{arccot} x/(2\pi)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(0,u,p,m)=u/4$
    /// - $f(x,0,p,m)=0$ for $x$ in the domain
    /// - the results $f(\pm1,\ldots)=\pmu/8$ are exact when representable with `prec` bits, and
    ///   rounded otherwise
    /// - Since a [`BigExponentFloat`] has no negative zero, its zero is treated as $+0$ (as FLINT's
    ///   `arb_atan2` treats it), where the `Float` function gives $\pmu/4$ by the sign of the zero
    ///
    /// Unlike [its `Float` counterpart](crate::Float::acot_with_period_rational_prec_round), this
    /// function never underflows.
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`BigExponentFloat::acot_with_period_rational_prec`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `x.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (y, o) = BigExponentFloat::acot_with_period_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     7,
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(y.to_string(), "1.38");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::acot_with_period_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     7,
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(y.to_string(), "1.44");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::acot_with_period_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     7,
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(y.to_string(), "1.38");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::acot_with_period_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     7,
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(y.to_string(), "1.3915424");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::acot_with_period_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     7,
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(y.to_string(), "1.391544");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::acot_with_period_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     7,
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(y.to_string(), "1.3915424");
    /// assert_eq!(o, Less);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn acot_with_period_rational_prec_round(
        x: Rational,
        u: u64,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        inverse_trig_rational_helper(InverseTrigFunction::Acot, &x, Some(u), prec, rm)
    }

    /// Computes $u\operatorname{arccot} x/(2\pi)$, the arccotangent of a [`Rational`] measured in
    /// $u$ths of a turn, returning a [`BigExponentFloat`], rounding the result to the specified
    /// precision and with the specified rounding mode. The [`Rational`] is taken by reference. An
    /// [`Ordering`] is also returned, indicating whether the rounded arccotangent is less than,
    /// equal to, or greater than the exact arccotangent. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,u,p,m) = u\operatorname{arccot} x/(2\pi)+\varepsilon.
    /// $$
    /// - If $u\operatorname{arccot} x/(2\pi)$ is zero, or the result is `NaN`, $\varepsilon$ may be
    ///   ignored or assumed to be 0.
    /// - If $u\operatorname{arccot} x/(2\pi)$ is nonzero, and $m$ is not `Nearest`, then
    ///   $|\varepsilon| < 2^{\lfloor\log_2 |u\operatorname{arccot} x/(2\pi)|\rfloor-p+1}$.
    /// - If $u\operatorname{arccot} x/(2\pi)$ is nonzero, and $m$ is `Nearest`, then $|\varepsilon|
    ///   \leq 2^{\lfloor\log_2 |u\operatorname{arccot} x/(2\pi)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(0,u,p,m)=u/4$
    /// - $f(x,0,p,m)=0$ for $x$ in the domain
    /// - the results $f(\pm1,\ldots)=\pmu/8$ are exact when representable with `prec` bits, and
    ///   rounded otherwise
    /// - Since a [`BigExponentFloat`] has no negative zero, its zero is treated as $+0$ (as FLINT's
    ///   `arb_atan2` treats it), where the `Float` function gives $\pmu/4$ by the sign of the zero
    ///
    /// Unlike [its `Float` counterpart](crate::Float::acot_with_period_rational_prec_round), this
    /// function never underflows.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `x.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (y, o) = BigExponentFloat::acot_with_period_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     7,
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(y.to_string(), "1.38");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::acot_with_period_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     7,
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(y.to_string(), "1.44");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::acot_with_period_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     7,
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(y.to_string(), "1.38");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::acot_with_period_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     7,
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(y.to_string(), "1.3915424");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::acot_with_period_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     7,
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(y.to_string(), "1.391544");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::acot_with_period_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     7,
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(y.to_string(), "1.3915424");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn acot_with_period_rational_prec_round_ref(
        x: &Rational,
        u: u64,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        inverse_trig_rational_helper(InverseTrigFunction::Acot, x, Some(u), prec, rm)
    }

    /// Computes $u\operatorname{arccot} x/(2\pi)$, the arccotangent of a [`Rational`] measured in
    /// $u$ths of a turn, returning a [`BigExponentFloat`], rounding the result to the nearest value
    /// of the specified precision. The [`Rational`] is taken by value. An [`Ordering`] is also
    /// returned, indicating whether the rounded arccotangent is less than, equal to, or greater
    /// than the exact arccotangent. Although `NaN`s are not comparable to any [`BigExponentFloat`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,u,p) = u\operatorname{arccot} x/(2\pi)+\varepsilon.
    /// $$
    /// - If $u\operatorname{arccot} x/(2\pi)$ is zero, or the result is `NaN`, $\varepsilon$ may be
    ///   ignored or assumed to be 0.
    /// - If $u\operatorname{arccot} x/(2\pi)$ is nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |u\operatorname{arccot} x/(2\pi)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(0,u,p)=u/4$
    /// - $f(x,0,p)=0$ for $x$ in the domain
    /// - the results $f(\pm1,\ldots)=\pmu/8$ are exact when representable with `prec` bits, and
    ///   rounded otherwise
    /// - Since a [`BigExponentFloat`] has no negative zero, its zero is treated as $+0$ (as FLINT's
    ///   `arb_atan2` treats it), where the `Float` function gives $\pmu/4$ by the sign of the zero
    ///
    /// Unlike [its `Float` counterpart](crate::Float::acot_with_period_rational_prec_round), this
    /// function never underflows.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`BigExponentFloat::acot_with_period_rational_prec_round`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
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
    /// let (y, o) = BigExponentFloat::acot_with_period_rational_prec(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     7,
    ///     5,
    /// );
    /// assert_eq!(y.to_string(), "1.38");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::acot_with_period_rational_prec(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     7,
    ///     20,
    /// );
    /// assert_eq!(y.to_string(), "1.3915424");
    /// assert_eq!(o, Less);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn acot_with_period_rational_prec(x: Rational, u: u64, prec: u64) -> (Self, Ordering) {
        inverse_trig_rational_helper(InverseTrigFunction::Acot, &x, Some(u), prec, Nearest)
    }

    /// Computes $u\operatorname{arccot} x/(2\pi)$, the arccotangent of a [`Rational`] measured in
    /// $u$ths of a turn, returning a [`BigExponentFloat`], rounding the result to the nearest value
    /// of the specified precision. The [`Rational`] is taken by reference. An [`Ordering`] is also
    /// returned, indicating whether the rounded arccotangent is less than, equal to, or greater
    /// than the exact arccotangent. Although `NaN`s are not comparable to any [`BigExponentFloat`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,u,p) = u\operatorname{arccot} x/(2\pi)+\varepsilon.
    /// $$
    /// - If $u\operatorname{arccot} x/(2\pi)$ is zero, or the result is `NaN`, $\varepsilon$ may be
    ///   ignored or assumed to be 0.
    /// - If $u\operatorname{arccot} x/(2\pi)$ is nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |u\operatorname{arccot} x/(2\pi)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(0,u,p)=u/4$
    /// - $f(x,0,p)=0$ for $x$ in the domain
    /// - the results $f(\pm1,\ldots)=\pmu/8$ are exact when representable with `prec` bits, and
    ///   rounded otherwise
    /// - Since a [`BigExponentFloat`] has no negative zero, its zero is treated as $+0$ (as FLINT's
    ///   `arb_atan2` treats it), where the `Float` function gives $\pmu/4$ by the sign of the zero
    ///
    /// Unlike [its `Float` counterpart](crate::Float::acot_with_period_rational_prec_round), this
    /// function never underflows.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
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
    /// let (y, o) = BigExponentFloat::acot_with_period_rational_prec_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     7,
    ///     5,
    /// );
    /// assert_eq!(y.to_string(), "1.38");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::acot_with_period_rational_prec_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     7,
    ///     20,
    /// );
    /// assert_eq!(y.to_string(), "1.3915424");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn acot_with_period_rational_prec_ref(x: &Rational, u: u64, prec: u64) -> (Self, Ordering) {
        inverse_trig_rational_helper(InverseTrigFunction::Acot, x, Some(u), prec, Nearest)
    }

    /// Computes $\operatorname{arccot}(x)/\pi$, the arccotangent of a [`BigExponentFloat`] measured
    /// in half-turns, rounding the result to the specified precision and with the specified
    /// rounding mode. The [`BigExponentFloat`] is taken by value. An [`Ordering`] is also returned,
    /// indicating whether the rounded arccotangent is less than, equal to, or greater than the
    /// exact arccotangent. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever
    /// this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \operatorname{arccot}(x)/\pi+\varepsilon.
    /// $$
    /// - If $\operatorname{arccot}(x)/\pi$ is zero, or the result is `NaN`, $\varepsilon$ may be
    ///   ignored or assumed to be 0.
    /// - If $\operatorname{arccot}(x)/\pi$ is nonzero, and $m$ is not `Nearest`, then
    ///   $|\varepsilon| < 2^{\lfloor\log_2 |\operatorname{arccot}(x)/\pi|\rfloor-p+1}$.
    /// - If $\operatorname{arccot}(x)/\pi$ is nonzero, and $m$ is `Nearest`, then $|\varepsilon|
    ///   \leq 2^{\lfloor\log_2 |\operatorname{arccot}(x)/\pi|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=\text{NaN}$
    /// - $f(\pm\infty,p,m)=0$
    /// - $f(0,p,m)=1/2$
    /// - $f(x,0,p,m)=0$ for $x$ in the domain
    /// - the results $f(\pm1,\ldots)=\pm1/4$ are exact when representable with `prec` bits, and
    ///   rounded otherwise
    /// - Since a [`BigExponentFloat`] has no negative zero, its zero is treated as $+0$ (as FLINT's
    ///   `arb_atan2` treats it), where the `Float` function gives $\pm1/2$ by the sign of the zero
    ///
    /// Unlike [its `Float` counterpart](crate::Float::acot_pi_prec_round), this function accepts
    /// arguments with any exponent, and its result never underflows.
    ///
    /// If you know you'll be using `Nearest`, consider using [`BigExponentFloat::acot_pi_prec`]
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (y, o) = BigExponentFloat::from(0.375).acot_pi_prec_round(5, Floor);
    /// assert_eq!(y.to_string(), "0.38");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::from(0.375).acot_pi_prec_round(5, Ceiling);
    /// assert_eq!(y.to_string(), "0.391");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::from(0.375).acot_pi_prec_round(5, Nearest);
    /// assert_eq!(y.to_string(), "0.391");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::from(0.375).acot_pi_prec_round(20, Floor);
    /// assert_eq!(y.to_string(), "0.3857994");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::from(0.375).acot_pi_prec_round(20, Ceiling);
    /// assert_eq!(y.to_string(), "0.38579988");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::from(0.375).acot_pi_prec_round(20, Nearest);
    /// assert_eq!(y.to_string(), "0.38579988");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn acot_pi_prec_round(self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        inverse_trig_helper(InverseTrigFunction::Acot, &self, Some(2), prec, rm, true)
    }

    /// Computes $\operatorname{arccot}(x)/\pi$, the arccotangent of a [`BigExponentFloat`] measured
    /// in half-turns, rounding the result to the specified precision and with the specified
    /// rounding mode. The [`BigExponentFloat`] is taken by reference. An [`Ordering`] is also
    /// returned, indicating whether the rounded arccotangent is less than, equal to, or greater
    /// than the exact arccotangent. Although `NaN`s are not comparable to any [`BigExponentFloat`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \operatorname{arccot}(x)/\pi+\varepsilon.
    /// $$
    /// - If $\operatorname{arccot}(x)/\pi$ is zero, or the result is `NaN`, $\varepsilon$ may be
    ///   ignored or assumed to be 0.
    /// - If $\operatorname{arccot}(x)/\pi$ is nonzero, and $m$ is not `Nearest`, then
    ///   $|\varepsilon| < 2^{\lfloor\log_2 |\operatorname{arccot}(x)/\pi|\rfloor-p+1}$.
    /// - If $\operatorname{arccot}(x)/\pi$ is nonzero, and $m$ is `Nearest`, then $|\varepsilon|
    ///   \leq 2^{\lfloor\log_2 |\operatorname{arccot}(x)/\pi|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=\text{NaN}$
    /// - $f(\pm\infty,p,m)=0$
    /// - $f(0,p,m)=1/2$
    /// - $f(x,0,p,m)=0$ for $x$ in the domain
    /// - the results $f(\pm1,\ldots)=\pm1/4$ are exact when representable with `prec` bits, and
    ///   rounded otherwise
    /// - Since a [`BigExponentFloat`] has no negative zero, its zero is treated as $+0$ (as FLINT's
    ///   `arb_atan2` treats it), where the `Float` function gives $\pm1/2$ by the sign of the zero
    ///
    /// Unlike [its `Float` counterpart](crate::Float::acot_pi_prec_round), this function accepts
    /// arguments with any exponent, and its result never underflows.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (y, o) = (&BigExponentFloat::from(0.375)).acot_pi_prec_round_ref(5, Floor);
    /// assert_eq!(y.to_string(), "0.38");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = (&BigExponentFloat::from(0.375)).acot_pi_prec_round_ref(5, Ceiling);
    /// assert_eq!(y.to_string(), "0.391");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = (&BigExponentFloat::from(0.375)).acot_pi_prec_round_ref(5, Nearest);
    /// assert_eq!(y.to_string(), "0.391");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = (&BigExponentFloat::from(0.375)).acot_pi_prec_round_ref(20, Floor);
    /// assert_eq!(y.to_string(), "0.3857994");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = (&BigExponentFloat::from(0.375)).acot_pi_prec_round_ref(20, Ceiling);
    /// assert_eq!(y.to_string(), "0.38579988");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = (&BigExponentFloat::from(0.375)).acot_pi_prec_round_ref(20, Nearest);
    /// assert_eq!(y.to_string(), "0.38579988");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn acot_pi_prec_round_ref(&self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        inverse_trig_helper(InverseTrigFunction::Acot, self, Some(2), prec, rm, true)
    }

    /// Replaces a [`BigExponentFloat`] with $\operatorname{arccot}(x)/\pi$, its arccotangent
    /// measured in half-turns, rounding the result to the specified precision and with the
    /// specified rounding mode. An [`Ordering`] is returned, indicating whether the rounded
    /// arccotangent is less than, equal to, or greater than the exact arccotangent. Although `NaN`s
    /// are not comparable to any [`BigExponentFloat`], whenever this function sets the
    /// [`BigExponentFloat`] to `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \operatorname{arccot}(x)/\pi+\varepsilon.
    /// $$
    /// - If $\operatorname{arccot}(x)/\pi$ is zero, or the result is `NaN`, $\varepsilon$ may be
    ///   ignored or assumed to be 0.
    /// - If $\operatorname{arccot}(x)/\pi$ is nonzero, and $m$ is not `Nearest`, then
    ///   $|\varepsilon| < 2^{\lfloor\log_2 |\operatorname{arccot}(x)/\pi|\rfloor-p+1}$.
    /// - If $\operatorname{arccot}(x)/\pi$ is nonzero, and $m$ is `Nearest`, then $|\varepsilon|
    ///   \leq 2^{\lfloor\log_2 |\operatorname{arccot}(x)/\pi|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=\text{NaN}$
    /// - $f(\pm\infty,p,m)=0$
    /// - $f(0,p,m)=1/2$
    /// - $f(x,0,p,m)=0$ for $x$ in the domain
    /// - the results $f(\pm1,\ldots)=\pm1/4$ are exact when representable with `prec` bits, and
    ///   rounded otherwise
    /// - Since a [`BigExponentFloat`] has no negative zero, its zero is treated as $+0$ (as FLINT's
    ///   `arb_atan2` treats it), where the `Float` function gives $\pm1/2$ by the sign of the zero
    ///
    /// Unlike [its `Float` counterpart](crate::Float::acot_pi_prec_round), this function accepts
    /// arguments with any exponent, and its result never underflows.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = BigExponentFloat::from(0.375);
    /// assert_eq!(x.acot_pi_prec_round_assign(5, Floor), Less);
    /// assert_eq!(x.to_string(), "0.38");
    ///
    /// let mut x = BigExponentFloat::from(0.375);
    /// assert_eq!(x.acot_pi_prec_round_assign(5, Ceiling), Greater);
    /// assert_eq!(x.to_string(), "0.391");
    ///
    /// let mut x = BigExponentFloat::from(0.375);
    /// assert_eq!(x.acot_pi_prec_round_assign(5, Nearest), Greater);
    /// assert_eq!(x.to_string(), "0.391");
    ///
    /// let mut x = BigExponentFloat::from(0.375);
    /// assert_eq!(x.acot_pi_prec_round_assign(20, Floor), Less);
    /// assert_eq!(x.to_string(), "0.3857994");
    ///
    /// let mut x = BigExponentFloat::from(0.375);
    /// assert_eq!(x.acot_pi_prec_round_assign(20, Ceiling), Greater);
    /// assert_eq!(x.to_string(), "0.38579988");
    ///
    /// let mut x = BigExponentFloat::from(0.375);
    /// assert_eq!(x.acot_pi_prec_round_assign(20, Nearest), Greater);
    /// assert_eq!(x.to_string(), "0.38579988");
    /// ```
    #[inline]
    pub fn acot_pi_prec_round_assign(&mut self, prec: u64, rm: RoundingMode) -> Ordering {
        let (result, o) =
            inverse_trig_helper(InverseTrigFunction::Acot, self, Some(2), prec, rm, true);
        *self = result;
        o
    }

    /// Computes $\operatorname{arccot}(x)/\pi$, the arccotangent of a [`BigExponentFloat`] measured
    /// in half-turns, rounding the result to the nearest value of the specified precision. The
    /// [`BigExponentFloat`] is taken by value. An [`Ordering`] is also returned, indicating whether
    /// the rounded arccotangent is less than, equal to, or greater than the exact arccotangent.
    /// Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,p) = \operatorname{arccot}(x)/\pi+\varepsilon.
    /// $$
    /// - If $\operatorname{arccot}(x)/\pi$ is zero, or the result is `NaN`, $\varepsilon$ may be
    ///   ignored or assumed to be 0.
    /// - If $\operatorname{arccot}(x)/\pi$ is nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\operatorname{arccot}(x)/\pi|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p)=\text{NaN}$
    /// - $f(\pm\infty,p)=0$
    /// - $f(0,p)=1/2$
    /// - $f(x,0,p)=0$ for $x$ in the domain
    /// - the results $f(\pm1,\ldots)=\pm1/4$ are exact when representable with `prec` bits, and
    ///   rounded otherwise
    /// - Since a [`BigExponentFloat`] has no negative zero, its zero is treated as $+0$ (as FLINT's
    ///   `arb_atan2` treats it), where the `Float` function gives $\pm1/2$ by the sign of the zero
    ///
    /// Unlike [its `Float` counterpart](crate::Float::acot_pi_prec_round), this function accepts
    /// arguments with any exponent, and its result never underflows.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`BigExponentFloat::acot_pi_prec_round`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (y, o) = BigExponentFloat::from(0.375).acot_pi_prec(5);
    /// assert_eq!(y.to_string(), "0.391");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::from(0.375).acot_pi_prec(20);
    /// assert_eq!(y.to_string(), "0.38579988");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn acot_pi_prec(self, prec: u64) -> (Self, Ordering) {
        inverse_trig_helper(
            InverseTrigFunction::Acot,
            &self,
            Some(2),
            prec,
            Nearest,
            true,
        )
    }

    /// Computes $\operatorname{arccot}(x)/\pi$, the arccotangent of a [`BigExponentFloat`] measured
    /// in half-turns, rounding the result to the nearest value of the specified precision. The
    /// [`BigExponentFloat`] is taken by reference. An [`Ordering`] is also returned, indicating
    /// whether the rounded arccotangent is less than, equal to, or greater than the exact
    /// arccotangent. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,p) = \operatorname{arccot}(x)/\pi+\varepsilon.
    /// $$
    /// - If $\operatorname{arccot}(x)/\pi$ is zero, or the result is `NaN`, $\varepsilon$ may be
    ///   ignored or assumed to be 0.
    /// - If $\operatorname{arccot}(x)/\pi$ is nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\operatorname{arccot}(x)/\pi|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p)=\text{NaN}$
    /// - $f(\pm\infty,p)=0$
    /// - $f(0,p)=1/2$
    /// - $f(x,0,p)=0$ for $x$ in the domain
    /// - the results $f(\pm1,\ldots)=\pm1/4$ are exact when representable with `prec` bits, and
    ///   rounded otherwise
    /// - Since a [`BigExponentFloat`] has no negative zero, its zero is treated as $+0$ (as FLINT's
    ///   `arb_atan2` treats it), where the `Float` function gives $\pm1/2$ by the sign of the zero
    ///
    /// Unlike [its `Float` counterpart](crate::Float::acot_pi_prec_round), this function accepts
    /// arguments with any exponent, and its result never underflows.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (y, o) = (&BigExponentFloat::from(0.375)).acot_pi_prec_ref(5);
    /// assert_eq!(y.to_string(), "0.391");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = (&BigExponentFloat::from(0.375)).acot_pi_prec_ref(20);
    /// assert_eq!(y.to_string(), "0.38579988");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn acot_pi_prec_ref(&self, prec: u64) -> (Self, Ordering) {
        inverse_trig_helper(
            InverseTrigFunction::Acot,
            self,
            Some(2),
            prec,
            Nearest,
            true,
        )
    }

    /// Replaces a [`BigExponentFloat`] with $\operatorname{arccot}(x)/\pi$, its arccotangent
    /// measured in half-turns, rounding the result to the nearest value of the specified precision.
    /// An [`Ordering`] is returned, indicating whether the rounded arccotangent is less than, equal
    /// to, or greater than the exact arccotangent. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function sets the [`BigExponentFloat`] to `NaN` it also
    /// returns `Equal`.
    ///
    /// $$
    /// f(x,p) = \operatorname{arccot}(x)/\pi+\varepsilon.
    /// $$
    /// - If $\operatorname{arccot}(x)/\pi$ is zero, or the result is `NaN`, $\varepsilon$ may be
    ///   ignored or assumed to be 0.
    /// - If $\operatorname{arccot}(x)/\pi$ is nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\operatorname{arccot}(x)/\pi|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p)=\text{NaN}$
    /// - $f(\pm\infty,p)=0$
    /// - $f(0,p)=1/2$
    /// - $f(x,0,p)=0$ for $x$ in the domain
    /// - the results $f(\pm1,\ldots)=\pm1/4$ are exact when representable with `prec` bits, and
    ///   rounded otherwise
    /// - Since a [`BigExponentFloat`] has no negative zero, its zero is treated as $+0$ (as FLINT's
    ///   `arb_atan2` treats it), where the `Float` function gives $\pm1/2$ by the sign of the zero
    ///
    /// Unlike [its `Float` counterpart](crate::Float::acot_pi_prec_round), this function accepts
    /// arguments with any exponent, and its result never underflows.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = BigExponentFloat::from(0.375);
    /// assert_eq!(x.acot_pi_prec_assign(5), Greater);
    /// assert_eq!(x.to_string(), "0.391");
    ///
    /// let mut x = BigExponentFloat::from(0.375);
    /// assert_eq!(x.acot_pi_prec_assign(20), Greater);
    /// assert_eq!(x.to_string(), "0.38579988");
    /// ```
    #[inline]
    pub fn acot_pi_prec_assign(&mut self, prec: u64) -> Ordering {
        let (result, o) = inverse_trig_helper(
            InverseTrigFunction::Acot,
            self,
            Some(2),
            prec,
            Nearest,
            true,
        );
        *self = result;
        o
    }

    /// Computes $\operatorname{arccot}(x)/\pi$, the arccotangent of a [`Rational`] measured in
    /// half-turns, returning a [`BigExponentFloat`], rounding the result to the specified precision
    /// and with the specified rounding mode. The [`Rational`] is taken by value. An [`Ordering`] is
    /// also returned, indicating whether the rounded arccotangent is less than, equal to, or
    /// greater than the exact arccotangent. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \operatorname{arccot}(x)/\pi+\varepsilon.
    /// $$
    /// - If $\operatorname{arccot}(x)/\pi$ is zero, or the result is `NaN`, $\varepsilon$ may be
    ///   ignored or assumed to be 0.
    /// - If $\operatorname{arccot}(x)/\pi$ is nonzero, and $m$ is not `Nearest`, then
    ///   $|\varepsilon| < 2^{\lfloor\log_2 |\operatorname{arccot}(x)/\pi|\rfloor-p+1}$.
    /// - If $\operatorname{arccot}(x)/\pi$ is nonzero, and $m$ is `Nearest`, then $|\varepsilon|
    ///   \leq 2^{\lfloor\log_2 |\operatorname{arccot}(x)/\pi|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(0,p,m)=1/2$
    /// - $f(x,0,p,m)=0$ for $x$ in the domain
    /// - the results $f(\pm1,\ldots)=\pm1/4$ are exact when representable with `prec` bits, and
    ///   rounded otherwise
    /// - Since a [`BigExponentFloat`] has no negative zero, its zero is treated as $+0$ (as FLINT's
    ///   `arb_atan2` treats it), where the `Float` function gives $\pm1/2$ by the sign of the zero
    ///
    /// Unlike [its `Float` counterpart](crate::Float::acot_pi_rational_prec_round), this function
    /// never underflows.
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`BigExponentFloat::acot_pi_rational_prec`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `x.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (y, o) = BigExponentFloat::acot_pi_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(y.to_string(), "0.391");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::acot_pi_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(y.to_string(), "0.406");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::acot_pi_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(y.to_string(), "0.391");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::acot_pi_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(y.to_string(), "0.39758348");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::acot_pi_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(y.to_string(), "0.3975840");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::acot_pi_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(y.to_string(), "0.39758348");
    /// assert_eq!(o, Less);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn acot_pi_rational_prec_round(
        x: Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        inverse_trig_rational_helper(InverseTrigFunction::Acot, &x, Some(2), prec, rm)
    }

    /// Computes $\operatorname{arccot}(x)/\pi$, the arccotangent of a [`Rational`] measured in
    /// half-turns, returning a [`BigExponentFloat`], rounding the result to the specified precision
    /// and with the specified rounding mode. The [`Rational`] is taken by reference. An
    /// [`Ordering`] is also returned, indicating whether the rounded arccotangent is less than,
    /// equal to, or greater than the exact arccotangent. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \operatorname{arccot}(x)/\pi+\varepsilon.
    /// $$
    /// - If $\operatorname{arccot}(x)/\pi$ is zero, or the result is `NaN`, $\varepsilon$ may be
    ///   ignored or assumed to be 0.
    /// - If $\operatorname{arccot}(x)/\pi$ is nonzero, and $m$ is not `Nearest`, then
    ///   $|\varepsilon| < 2^{\lfloor\log_2 |\operatorname{arccot}(x)/\pi|\rfloor-p+1}$.
    /// - If $\operatorname{arccot}(x)/\pi$ is nonzero, and $m$ is `Nearest`, then $|\varepsilon|
    ///   \leq 2^{\lfloor\log_2 |\operatorname{arccot}(x)/\pi|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(0,p,m)=1/2$
    /// - $f(x,0,p,m)=0$ for $x$ in the domain
    /// - the results $f(\pm1,\ldots)=\pm1/4$ are exact when representable with `prec` bits, and
    ///   rounded otherwise
    /// - Since a [`BigExponentFloat`] has no negative zero, its zero is treated as $+0$ (as FLINT's
    ///   `arb_atan2` treats it), where the `Float` function gives $\pm1/2$ by the sign of the zero
    ///
    /// Unlike [its `Float` counterpart](crate::Float::acot_pi_rational_prec_round), this function
    /// never underflows.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `x.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (y, o) = BigExponentFloat::acot_pi_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(y.to_string(), "0.391");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::acot_pi_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(y.to_string(), "0.406");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::acot_pi_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(y.to_string(), "0.391");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::acot_pi_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(y.to_string(), "0.39758348");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::acot_pi_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(y.to_string(), "0.3975840");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::acot_pi_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(y.to_string(), "0.39758348");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn acot_pi_rational_prec_round_ref(
        x: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        inverse_trig_rational_helper(InverseTrigFunction::Acot, x, Some(2), prec, rm)
    }

    /// Computes $\operatorname{arccot}(x)/\pi$, the arccotangent of a [`Rational`] measured in
    /// half-turns, returning a [`BigExponentFloat`], rounding the result to the nearest value of
    /// the specified precision. The [`Rational`] is taken by value. An [`Ordering`] is also
    /// returned, indicating whether the rounded arccotangent is less than, equal to, or greater
    /// than the exact arccotangent. Although `NaN`s are not comparable to any [`BigExponentFloat`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,p) = \operatorname{arccot}(x)/\pi+\varepsilon.
    /// $$
    /// - If $\operatorname{arccot}(x)/\pi$ is zero, or the result is `NaN`, $\varepsilon$ may be
    ///   ignored or assumed to be 0.
    /// - If $\operatorname{arccot}(x)/\pi$ is nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\operatorname{arccot}(x)/\pi|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(0,p)=1/2$
    /// - $f(x,0,p)=0$ for $x$ in the domain
    /// - the results $f(\pm1,\ldots)=\pm1/4$ are exact when representable with `prec` bits, and
    ///   rounded otherwise
    /// - Since a [`BigExponentFloat`] has no negative zero, its zero is treated as $+0$ (as FLINT's
    ///   `arb_atan2` treats it), where the `Float` function gives $\pm1/2$ by the sign of the zero
    ///
    /// Unlike [its `Float` counterpart](crate::Float::acot_pi_rational_prec_round), this function
    /// never underflows.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`BigExponentFloat::acot_pi_rational_prec_round`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
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
    /// let (y, o) = BigExponentFloat::acot_pi_rational_prec(Rational::from_unsigneds(1u8, 3), 5);
    /// assert_eq!(y.to_string(), "0.391");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::acot_pi_rational_prec(Rational::from_unsigneds(1u8, 3), 20);
    /// assert_eq!(y.to_string(), "0.39758348");
    /// assert_eq!(o, Less);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn acot_pi_rational_prec(x: Rational, prec: u64) -> (Self, Ordering) {
        inverse_trig_rational_helper(InverseTrigFunction::Acot, &x, Some(2), prec, Nearest)
    }

    /// Computes $\operatorname{arccot}(x)/\pi$, the arccotangent of a [`Rational`] measured in
    /// half-turns, returning a [`BigExponentFloat`], rounding the result to the nearest value of
    /// the specified precision. The [`Rational`] is taken by reference. An [`Ordering`] is also
    /// returned, indicating whether the rounded arccotangent is less than, equal to, or greater
    /// than the exact arccotangent. Although `NaN`s are not comparable to any [`BigExponentFloat`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,p) = \operatorname{arccot}(x)/\pi+\varepsilon.
    /// $$
    /// - If $\operatorname{arccot}(x)/\pi$ is zero, or the result is `NaN`, $\varepsilon$ may be
    ///   ignored or assumed to be 0.
    /// - If $\operatorname{arccot}(x)/\pi$ is nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\operatorname{arccot}(x)/\pi|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(0,p)=1/2$
    /// - $f(x,0,p)=0$ for $x$ in the domain
    /// - the results $f(\pm1,\ldots)=\pm1/4$ are exact when representable with `prec` bits, and
    ///   rounded otherwise
    /// - Since a [`BigExponentFloat`] has no negative zero, its zero is treated as $+0$ (as FLINT's
    ///   `arb_atan2` treats it), where the `Float` function gives $\pm1/2$ by the sign of the zero
    ///
    /// Unlike [its `Float` counterpart](crate::Float::acot_pi_rational_prec_round), this function
    /// never underflows.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
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
    ///     BigExponentFloat::acot_pi_rational_prec_ref(&Rational::from_unsigneds(1u8, 3), 5);
    /// assert_eq!(y.to_string(), "0.391");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) =
    ///     BigExponentFloat::acot_pi_rational_prec_ref(&Rational::from_unsigneds(1u8, 3), 20);
    /// assert_eq!(y.to_string(), "0.39758348");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn acot_pi_rational_prec_ref(x: &Rational, prec: u64) -> (Self, Ordering) {
        inverse_trig_rational_helper(InverseTrigFunction::Acot, x, Some(2), prec, Nearest)
    }
}
