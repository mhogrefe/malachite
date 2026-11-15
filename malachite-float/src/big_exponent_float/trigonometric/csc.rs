// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::big_exponent_float::BigExponentFloat;
use crate::big_exponent_float::trigonometric::sin_cos::{
    TrigFunction, trig_helper, trig_rational_helper, trig_with_period_helper,
    trig_with_period_rational_helper,
};
use core::cmp::Ordering;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_q::Rational;

impl BigExponentFloat {
    /// Computes the cosecant of a [`BigExponentFloat`], rounding the result to the specified
    /// precision and with the specified rounding mode. The [`BigExponentFloat`] is taken by value.
    /// An [`Ordering`] is also returned, indicating whether the rounded cosecant is less than,
    /// equal to, or greater than the exact cosecant. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \csc x+\varepsilon.
    /// $$
    /// - If $\csc x$ is zero, or the result is `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\csc x$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\csc x|\rfloor-p+1}$.
    /// - If $\csc x$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\csc x|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=f(\pm\infty,p,m)=\text{NaN}$
    /// - $f(0,p,m)=\text{NaN}$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::csc_prec_round), this function accepts
    /// arguments with any exponent, and its result never overflows or underflows.
    ///
    /// Warning: an argument with exponent $e > 0$ is reduced modulo $\pi/2$, which takes about $e$
    /// bits of $\pi$. The time and memory needed therefore grow with the exponent, without any
    /// limit: an argument near $2^{2^{40}}$ needs about $2^{40}$ bits, or 128 GiB, of $\pi$.
    ///
    /// A [`BigExponentFloat`] has no negative zero, so the two one-sided limits at 0 ($\infty$ and
    /// $-\infty$) cannot be told apart, and the result at 0 is `NaN`, as $1/0$ is, where the
    /// `Float` function gives an infinity.
    ///
    /// If you know you'll be using `Nearest`, consider using [`BigExponentFloat::csc_prec`]
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m, e) = O(n (\log n)^3 \log\log n + (n+m+e) (\log (n+m+e))^2 \log\log (n+m+e))$
    ///
    /// $M(n, m, e) = O((n+m+e) \log (n+m+e))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, $m$ is
    /// `self.significant_bits()`, and $e$ is the exponent of the argument (0 if it is negative).
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the argument is finite and nonzero (the
    /// cosecant of a nonzero rational number is transcendental).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (y, o) = BigExponentFloat::ONE.csc_prec_round(5, Floor);
    /// assert_eq!(y.to_string(), "1.19");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::ONE.csc_prec_round(5, Ceiling);
    /// assert_eq!(y.to_string(), "1.2");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::ONE.csc_prec_round(5, Nearest);
    /// assert_eq!(y.to_string(), "1.19");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::ONE.csc_prec_round(20, Floor);
    /// assert_eq!(y.to_string(), "1.1883945");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::ONE.csc_prec_round(20, Ceiling);
    /// assert_eq!(y.to_string(), "1.188396");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::ONE.csc_prec_round(20, Nearest);
    /// assert_eq!(y.to_string(), "1.1883945");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn csc_prec_round(self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        trig_helper(TrigFunction::Csc, &self, prec, rm, true)
    }

    /// Computes the cosecant of a [`BigExponentFloat`], rounding the result to the specified
    /// precision and with the specified rounding mode. The [`BigExponentFloat`] is taken by
    /// reference. An [`Ordering`] is also returned, indicating whether the rounded cosecant is less
    /// than, equal to, or greater than the exact cosecant. Although `NaN`s are not comparable to
    /// any [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \csc x+\varepsilon.
    /// $$
    /// - If $\csc x$ is zero, or the result is `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\csc x$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\csc x|\rfloor-p+1}$.
    /// - If $\csc x$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\csc x|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=f(\pm\infty,p,m)=\text{NaN}$
    /// - $f(0,p,m)=\text{NaN}$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::csc_prec_round), this function accepts
    /// arguments with any exponent, and its result never overflows or underflows.
    ///
    /// Warning: an argument with exponent $e > 0$ is reduced modulo $\pi/2$, which takes about $e$
    /// bits of $\pi$. The time and memory needed therefore grow with the exponent, without any
    /// limit: an argument near $2^{2^{40}}$ needs about $2^{40}$ bits, or 128 GiB, of $\pi$.
    ///
    /// A [`BigExponentFloat`] has no negative zero, so the two one-sided limits at 0 ($\infty$ and
    /// $-\infty$) cannot be told apart, and the result at 0 is `NaN`, as $1/0$ is, where the
    /// `Float` function gives an infinity.
    ///
    /// # Worst-case complexity
    /// $T(n, m, e) = O(n (\log n)^3 \log\log n + (n+m+e) (\log (n+m+e))^2 \log\log (n+m+e))$
    ///
    /// $M(n, m, e) = O((n+m+e) \log (n+m+e))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, $m$ is
    /// `self.significant_bits()`, and $e$ is the exponent of the argument (0 if it is negative).
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the argument is finite and nonzero (the
    /// cosecant of a nonzero rational number is transcendental).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (y, o) = (&BigExponentFloat::ONE).csc_prec_round_ref(5, Floor);
    /// assert_eq!(y.to_string(), "1.19");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = (&BigExponentFloat::ONE).csc_prec_round_ref(5, Ceiling);
    /// assert_eq!(y.to_string(), "1.2");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = (&BigExponentFloat::ONE).csc_prec_round_ref(5, Nearest);
    /// assert_eq!(y.to_string(), "1.19");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = (&BigExponentFloat::ONE).csc_prec_round_ref(20, Floor);
    /// assert_eq!(y.to_string(), "1.1883945");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = (&BigExponentFloat::ONE).csc_prec_round_ref(20, Ceiling);
    /// assert_eq!(y.to_string(), "1.188396");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = (&BigExponentFloat::ONE).csc_prec_round_ref(20, Nearest);
    /// assert_eq!(y.to_string(), "1.1883945");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn csc_prec_round_ref(&self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        trig_helper(TrigFunction::Csc, self, prec, rm, true)
    }

    /// Replaces a [`BigExponentFloat`] with its cosecant, rounding the result to the specified
    /// precision and with the specified rounding mode. An [`Ordering`] is returned, indicating
    /// whether the rounded cosecant is less than, equal to, or greater than the exact cosecant.
    /// Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function sets
    /// the [`BigExponentFloat`] to `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \csc x+\varepsilon.
    /// $$
    /// - If $\csc x$ is zero, or the result is `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\csc x$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\csc x|\rfloor-p+1}$.
    /// - If $\csc x$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\csc x|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=f(\pm\infty,p,m)=\text{NaN}$
    /// - $f(0,p,m)=\text{NaN}$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::csc_prec_round), this function accepts
    /// arguments with any exponent, and its result never overflows or underflows.
    ///
    /// Warning: an argument with exponent $e > 0$ is reduced modulo $\pi/2$, which takes about $e$
    /// bits of $\pi$. The time and memory needed therefore grow with the exponent, without any
    /// limit: an argument near $2^{2^{40}}$ needs about $2^{40}$ bits, or 128 GiB, of $\pi$.
    ///
    /// A [`BigExponentFloat`] has no negative zero, so the two one-sided limits at 0 ($\infty$ and
    /// $-\infty$) cannot be told apart, and the result at 0 is `NaN`, as $1/0$ is, where the
    /// `Float` function gives an infinity.
    ///
    /// # Worst-case complexity
    /// $T(n, m, e) = O(n (\log n)^3 \log\log n + (n+m+e) (\log (n+m+e))^2 \log\log (n+m+e))$
    ///
    /// $M(n, m, e) = O((n+m+e) \log (n+m+e))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, $m$ is
    /// `self.significant_bits()`, and $e$ is the exponent of the argument (0 if it is negative).
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the argument is finite and nonzero (the
    /// cosecant of a nonzero rational number is transcendental).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = BigExponentFloat::ONE;
    /// assert_eq!(x.csc_prec_round_assign(5, Floor), Less);
    /// assert_eq!(x.to_string(), "1.19");
    ///
    /// let mut x = BigExponentFloat::ONE;
    /// assert_eq!(x.csc_prec_round_assign(5, Ceiling), Greater);
    /// assert_eq!(x.to_string(), "1.2");
    ///
    /// let mut x = BigExponentFloat::ONE;
    /// assert_eq!(x.csc_prec_round_assign(5, Nearest), Less);
    /// assert_eq!(x.to_string(), "1.19");
    ///
    /// let mut x = BigExponentFloat::ONE;
    /// assert_eq!(x.csc_prec_round_assign(20, Floor), Less);
    /// assert_eq!(x.to_string(), "1.1883945");
    ///
    /// let mut x = BigExponentFloat::ONE;
    /// assert_eq!(x.csc_prec_round_assign(20, Ceiling), Greater);
    /// assert_eq!(x.to_string(), "1.188396");
    ///
    /// let mut x = BigExponentFloat::ONE;
    /// assert_eq!(x.csc_prec_round_assign(20, Nearest), Less);
    /// assert_eq!(x.to_string(), "1.1883945");
    /// ```
    #[inline]
    pub fn csc_prec_round_assign(&mut self, prec: u64, rm: RoundingMode) -> Ordering {
        let (result, o) = trig_helper(TrigFunction::Csc, self, prec, rm, true);
        *self = result;
        o
    }

    /// Computes the cosecant of a [`BigExponentFloat`], rounding the result to the nearest value of
    /// the specified precision. The [`BigExponentFloat`] is taken by value. An [`Ordering`] is also
    /// returned, indicating whether the rounded cosecant is less than, equal to, or greater than
    /// the exact cosecant. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever
    /// this function returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,p) = \csc x+\varepsilon.
    /// $$
    /// - If $\csc x$ is zero, or the result is `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\csc x$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2 |\csc
    ///   x|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p)=f(\pm\infty,p)=\text{NaN}$
    /// - $f(0,p)=\text{NaN}$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::csc_prec_round), this function accepts
    /// arguments with any exponent, and its result never overflows or underflows.
    ///
    /// Warning: an argument with exponent $e > 0$ is reduced modulo $\pi/2$, which takes about $e$
    /// bits of $\pi$. The time and memory needed therefore grow with the exponent, without any
    /// limit: an argument near $2^{2^{40}}$ needs about $2^{40}$ bits, or 128 GiB, of $\pi$.
    ///
    /// A [`BigExponentFloat`] has no negative zero, so the two one-sided limits at 0 ($\infty$ and
    /// $-\infty$) cannot be told apart, and the result at 0 is `NaN`, as $1/0$ is, where the
    /// `Float` function gives an infinity.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`BigExponentFloat::csc_prec_round`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m, e) = O(n (\log n)^3 \log\log n + (n+m+e) (\log (n+m+e))^2 \log\log (n+m+e))$
    ///
    /// $M(n, m, e) = O((n+m+e) \log (n+m+e))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, $m$ is
    /// `self.significant_bits()`, and $e$ is the exponent of the argument (0 if it is negative).
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
    /// let (y, o) = BigExponentFloat::ONE.csc_prec(5);
    /// assert_eq!(y.to_string(), "1.19");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::ONE.csc_prec(20);
    /// assert_eq!(y.to_string(), "1.1883945");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn csc_prec(self, prec: u64) -> (Self, Ordering) {
        trig_helper(TrigFunction::Csc, &self, prec, Nearest, true)
    }

    /// Computes the cosecant of a [`BigExponentFloat`], rounding the result to the nearest value of
    /// the specified precision. The [`BigExponentFloat`] is taken by reference. An [`Ordering`] is
    /// also returned, indicating whether the rounded cosecant is less than, equal to, or greater
    /// than the exact cosecant. Although `NaN`s are not comparable to any [`BigExponentFloat`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,p) = \csc x+\varepsilon.
    /// $$
    /// - If $\csc x$ is zero, or the result is `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\csc x$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2 |\csc
    ///   x|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p)=f(\pm\infty,p)=\text{NaN}$
    /// - $f(0,p)=\text{NaN}$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::csc_prec_round), this function accepts
    /// arguments with any exponent, and its result never overflows or underflows.
    ///
    /// Warning: an argument with exponent $e > 0$ is reduced modulo $\pi/2$, which takes about $e$
    /// bits of $\pi$. The time and memory needed therefore grow with the exponent, without any
    /// limit: an argument near $2^{2^{40}}$ needs about $2^{40}$ bits, or 128 GiB, of $\pi$.
    ///
    /// A [`BigExponentFloat`] has no negative zero, so the two one-sided limits at 0 ($\infty$ and
    /// $-\infty$) cannot be told apart, and the result at 0 is `NaN`, as $1/0$ is, where the
    /// `Float` function gives an infinity.
    ///
    /// # Worst-case complexity
    /// $T(n, m, e) = O(n (\log n)^3 \log\log n + (n+m+e) (\log (n+m+e))^2 \log\log (n+m+e))$
    ///
    /// $M(n, m, e) = O((n+m+e) \log (n+m+e))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, $m$ is
    /// `self.significant_bits()`, and $e$ is the exponent of the argument (0 if it is negative).
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
    /// let (y, o) = (&BigExponentFloat::ONE).csc_prec_ref(5);
    /// assert_eq!(y.to_string(), "1.19");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = (&BigExponentFloat::ONE).csc_prec_ref(20);
    /// assert_eq!(y.to_string(), "1.1883945");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn csc_prec_ref(&self, prec: u64) -> (Self, Ordering) {
        trig_helper(TrigFunction::Csc, self, prec, Nearest, true)
    }

    /// Replaces a [`BigExponentFloat`] with its cosecant, rounding the result to the nearest value
    /// of the specified precision. An [`Ordering`] is returned, indicating whether the rounded
    /// cosecant is less than, equal to, or greater than the exact cosecant. Although `NaN`s are not
    /// comparable to any [`BigExponentFloat`], whenever this function sets the [`BigExponentFloat`]
    /// to `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,p) = \csc x+\varepsilon.
    /// $$
    /// - If $\csc x$ is zero, or the result is `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\csc x$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2 |\csc
    ///   x|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p)=f(\pm\infty,p)=\text{NaN}$
    /// - $f(0,p)=\text{NaN}$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::csc_prec_round), this function accepts
    /// arguments with any exponent, and its result never overflows or underflows.
    ///
    /// Warning: an argument with exponent $e > 0$ is reduced modulo $\pi/2$, which takes about $e$
    /// bits of $\pi$. The time and memory needed therefore grow with the exponent, without any
    /// limit: an argument near $2^{2^{40}}$ needs about $2^{40}$ bits, or 128 GiB, of $\pi$.
    ///
    /// A [`BigExponentFloat`] has no negative zero, so the two one-sided limits at 0 ($\infty$ and
    /// $-\infty$) cannot be told apart, and the result at 0 is `NaN`, as $1/0$ is, where the
    /// `Float` function gives an infinity.
    ///
    /// # Worst-case complexity
    /// $T(n, m, e) = O(n (\log n)^3 \log\log n + (n+m+e) (\log (n+m+e))^2 \log\log (n+m+e))$
    ///
    /// $M(n, m, e) = O((n+m+e) \log (n+m+e))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, $m$ is
    /// `self.significant_bits()`, and $e$ is the exponent of the argument (0 if it is negative).
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
    /// assert_eq!(x.csc_prec_assign(5), Less);
    /// assert_eq!(x.to_string(), "1.19");
    ///
    /// let mut x = BigExponentFloat::ONE;
    /// assert_eq!(x.csc_prec_assign(20), Less);
    /// assert_eq!(x.to_string(), "1.1883945");
    /// ```
    #[inline]
    pub fn csc_prec_assign(&mut self, prec: u64) -> Ordering {
        let (result, o) = trig_helper(TrigFunction::Csc, self, prec, Nearest, true);
        *self = result;
        o
    }

    /// Computes the cosecant of a [`Rational`], returning a [`BigExponentFloat`], rounding the
    /// result to the specified precision and with the specified rounding mode. The [`Rational`] is
    /// taken by value. An [`Ordering`] is also returned, indicating whether the rounded cosecant is
    /// less than, equal to, or greater than the exact cosecant. Although `NaN`s are not comparable
    /// to any [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \csc x+\varepsilon.
    /// $$
    /// - If $\csc x$ is zero, or the result is `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\csc x$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\csc x|\rfloor-p+1}$.
    /// - If $\csc x$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\csc x|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(0,p,m)=\text{NaN}$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::csc_rational_prec_round), this function never
    /// overflows or underflows.
    ///
    /// A [`BigExponentFloat`] has no negative zero, so the two one-sided limits at 0 ($\infty$ and
    /// $-\infty$) cannot be told apart, and the result at 0 is `NaN`, as $1/0$ is, where the
    /// `Float` function gives an infinity.
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`BigExponentFloat::csc_rational_prec`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m, e) = O(n (\log n)^3 \log\log n + (n+m+e) (\log (n+m+e))^2 \log\log (n+m+e))$
    ///
    /// $M(n, m, e) = O((n+m+e) \log (n+m+e))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, $m$ is `x.significant_bits()`,
    /// and $e$ is the exponent of the argument (0 if it is negative).
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the argument is finite and nonzero (the
    /// cosecant of a nonzero rational number is transcendental).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (y, o) =
    ///     BigExponentFloat::csc_rational_prec_round(Rational::from_unsigneds(1u8, 3), 5, Floor);
    /// assert_eq!(y.to_string(), "3.0");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) =
    ///     BigExponentFloat::csc_rational_prec_round(Rational::from_unsigneds(1u8, 3), 5, Ceiling);
    /// assert_eq!(y.to_string(), "3.12");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) =
    ///     BigExponentFloat::csc_rational_prec_round(Rational::from_unsigneds(1u8, 3), 5, Nearest);
    /// assert_eq!(y.to_string(), "3.0");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) =
    ///     BigExponentFloat::csc_rational_prec_round(Rational::from_unsigneds(1u8, 3), 20, Floor);
    /// assert_eq!(y.to_string(), "3.056282");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::csc_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(y.to_string(), "3.0562859");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::csc_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(y.to_string(), "3.0562859");
    /// assert_eq!(o, Greater);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn csc_rational_prec_round(x: Rational, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        trig_rational_helper(TrigFunction::Csc, &x, prec, rm, true)
    }

    /// Computes the cosecant of a [`Rational`], returning a [`BigExponentFloat`], rounding the
    /// result to the specified precision and with the specified rounding mode. The [`Rational`] is
    /// taken by reference. An [`Ordering`] is also returned, indicating whether the rounded
    /// cosecant is less than, equal to, or greater than the exact cosecant. Although `NaN`s are not
    /// comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \csc x+\varepsilon.
    /// $$
    /// - If $\csc x$ is zero, or the result is `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\csc x$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\csc x|\rfloor-p+1}$.
    /// - If $\csc x$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\csc x|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(0,p,m)=\text{NaN}$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::csc_rational_prec_round), this function never
    /// overflows or underflows.
    ///
    /// A [`BigExponentFloat`] has no negative zero, so the two one-sided limits at 0 ($\infty$ and
    /// $-\infty$) cannot be told apart, and the result at 0 is `NaN`, as $1/0$ is, where the
    /// `Float` function gives an infinity.
    ///
    /// # Worst-case complexity
    /// $T(n, m, e) = O(n (\log n)^3 \log\log n + (n+m+e) (\log (n+m+e))^2 \log\log (n+m+e))$
    ///
    /// $M(n, m, e) = O((n+m+e) \log (n+m+e))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, $m$ is `x.significant_bits()`,
    /// and $e$ is the exponent of the argument (0 if it is negative).
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the argument is finite and nonzero (the
    /// cosecant of a nonzero rational number is transcendental).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (y, o) = BigExponentFloat::csc_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(y.to_string(), "3.0");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::csc_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(y.to_string(), "3.12");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::csc_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(y.to_string(), "3.0");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::csc_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(y.to_string(), "3.056282");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::csc_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(y.to_string(), "3.0562859");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::csc_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(y.to_string(), "3.0562859");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn csc_rational_prec_round_ref(
        x: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        trig_rational_helper(TrigFunction::Csc, x, prec, rm, true)
    }

    /// Computes the cosecant of a [`Rational`], returning a [`BigExponentFloat`], rounding the
    /// result to the nearest value of the specified precision. The [`Rational`] is taken by value.
    /// An [`Ordering`] is also returned, indicating whether the rounded cosecant is less than,
    /// equal to, or greater than the exact cosecant. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,p) = \csc x+\varepsilon.
    /// $$
    /// - If $\csc x$ is zero, or the result is `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\csc x$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2 |\csc
    ///   x|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(0,p)=\text{NaN}$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::csc_rational_prec_round), this function never
    /// overflows or underflows.
    ///
    /// A [`BigExponentFloat`] has no negative zero, so the two one-sided limits at 0 ($\infty$ and
    /// $-\infty$) cannot be told apart, and the result at 0 is `NaN`, as $1/0$ is, where the
    /// `Float` function gives an infinity.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`BigExponentFloat::csc_rational_prec_round`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m, e) = O(n (\log n)^3 \log\log n + (n+m+e) (\log (n+m+e))^2 \log\log (n+m+e))$
    ///
    /// $M(n, m, e) = O((n+m+e) \log (n+m+e))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, $m$ is `x.significant_bits()`,
    /// and $e$ is the exponent of the argument (0 if it is negative).
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
    /// let (y, o) = BigExponentFloat::csc_rational_prec(Rational::from_unsigneds(1u8, 3), 5);
    /// assert_eq!(y.to_string(), "3.0");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::csc_rational_prec(Rational::from_unsigneds(1u8, 3), 20);
    /// assert_eq!(y.to_string(), "3.0562859");
    /// assert_eq!(o, Greater);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn csc_rational_prec(x: Rational, prec: u64) -> (Self, Ordering) {
        trig_rational_helper(TrigFunction::Csc, &x, prec, Nearest, true)
    }

    /// Computes the cosecant of a [`Rational`], returning a [`BigExponentFloat`], rounding the
    /// result to the nearest value of the specified precision. The [`Rational`] is taken by
    /// reference. An [`Ordering`] is also returned, indicating whether the rounded cosecant is less
    /// than, equal to, or greater than the exact cosecant. Although `NaN`s are not comparable to
    /// any [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,p) = \csc x+\varepsilon.
    /// $$
    /// - If $\csc x$ is zero, or the result is `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\csc x$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2 |\csc
    ///   x|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(0,p)=\text{NaN}$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::csc_rational_prec_round), this function never
    /// overflows or underflows.
    ///
    /// A [`BigExponentFloat`] has no negative zero, so the two one-sided limits at 0 ($\infty$ and
    /// $-\infty$) cannot be told apart, and the result at 0 is `NaN`, as $1/0$ is, where the
    /// `Float` function gives an infinity.
    ///
    /// # Worst-case complexity
    /// $T(n, m, e) = O(n (\log n)^3 \log\log n + (n+m+e) (\log (n+m+e))^2 \log\log (n+m+e))$
    ///
    /// $M(n, m, e) = O((n+m+e) \log (n+m+e))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, $m$ is `x.significant_bits()`,
    /// and $e$ is the exponent of the argument (0 if it is negative).
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
    /// let (y, o) = BigExponentFloat::csc_rational_prec_ref(&Rational::from_unsigneds(1u8, 3), 5);
    /// assert_eq!(y.to_string(), "3.0");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::csc_rational_prec_ref(&Rational::from_unsigneds(1u8, 3), 20);
    /// assert_eq!(y.to_string(), "3.0562859");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn csc_rational_prec_ref(x: &Rational, prec: u64) -> (Self, Ordering) {
        trig_rational_helper(TrigFunction::Csc, x, prec, Nearest, true)
    }

    /// Computes $\csc(2\pi x/u)$, the cosecant of a [`BigExponentFloat`] measured in $u$ths of a
    /// turn, rounding the result to the specified precision and with the specified rounding mode.
    /// The [`BigExponentFloat`] is taken by value. An [`Ordering`] is also returned, indicating
    /// whether the rounded cosecant is less than, equal to, or greater than the exact cosecant.
    /// Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,u,p,m) = \csc(2\pi x/u)+\varepsilon.
    /// $$
    /// - If $\csc(2\pi x/u)$ is zero, or the result is `NaN`, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $\csc(2\pi x/u)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\csc(2\pi x/u)|\rfloor-p+1}$.
    /// - If $\csc(2\pi x/u)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\csc(2\pi x/u)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},u,p,m)=f(\pm\infty,u,p,m)=\text{NaN}$
    /// - $f(x,0,p,m)=\text{NaN}$
    /// - $f(0,u,p,m)=\text{NaN}$
    /// - If $x/u$ is an odd multiple of $1/4$, the result is exactly $1$ or $-1$, and if it is
    ///   $\pm1/12$ or $\pm5/12$ modulo $1$, exactly $2$ or $-2$.
    /// - If $x/u$ is a multiple of $1/2$, the cosecant has a pole there, and the result is `NaN`.
    ///
    /// Unlike [its `Float` counterpart](crate::Float::csc_with_period_prec_round), this function
    /// accepts arguments with any exponent, and its result never overflows or underflows. The
    /// argument is reduced exactly, modulo $u$, which for a large exponent takes time proportional
    /// to the number of bits of the exponent.
    ///
    /// At a pole, the two one-sided limits are $\infty$ and $-\infty$, and, as a
    /// [`BigExponentFloat`] has no negative zero to tell them apart, the result is `NaN`, as $1/0$
    /// is, where the `Float` function gives an infinity.
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`BigExponentFloat::csc_with_period_prec`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `self.significant_bits()` plus the number of bits of the exponent of the argument.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (y, o) = BigExponentFloat::ONE.csc_with_period_prec_round(7, 5, Floor);
    /// assert_eq!(y.to_string(), "1.2");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::ONE.csc_with_period_prec_round(7, 5, Ceiling);
    /// assert_eq!(y.to_string(), "1.31");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::ONE.csc_with_period_prec_round(7, 5, Nearest);
    /// assert_eq!(y.to_string(), "1.2");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::ONE.csc_with_period_prec_round(7, 20, Floor);
    /// assert_eq!(y.to_string(), "1.2790470");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::ONE.csc_with_period_prec_round(7, 20, Ceiling);
    /// assert_eq!(y.to_string(), "1.279049");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::ONE.csc_with_period_prec_round(7, 20, Nearest);
    /// assert_eq!(y.to_string(), "1.279049");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn csc_with_period_prec_round(
        self,
        u: u64,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        trig_with_period_helper(TrigFunction::Csc, &self, u, prec, rm, true)
    }

    /// Computes $\csc(2\pi x/u)$, the cosecant of a [`BigExponentFloat`] measured in $u$ths of a
    /// turn, rounding the result to the specified precision and with the specified rounding mode.
    /// The [`BigExponentFloat`] is taken by reference. An [`Ordering`] is also returned, indicating
    /// whether the rounded cosecant is less than, equal to, or greater than the exact cosecant.
    /// Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,u,p,m) = \csc(2\pi x/u)+\varepsilon.
    /// $$
    /// - If $\csc(2\pi x/u)$ is zero, or the result is `NaN`, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $\csc(2\pi x/u)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\csc(2\pi x/u)|\rfloor-p+1}$.
    /// - If $\csc(2\pi x/u)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\csc(2\pi x/u)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},u,p,m)=f(\pm\infty,u,p,m)=\text{NaN}$
    /// - $f(x,0,p,m)=\text{NaN}$
    /// - $f(0,u,p,m)=\text{NaN}$
    /// - If $x/u$ is an odd multiple of $1/4$, the result is exactly $1$ or $-1$, and if it is
    ///   $\pm1/12$ or $\pm5/12$ modulo $1$, exactly $2$ or $-2$.
    /// - If $x/u$ is a multiple of $1/2$, the cosecant has a pole there, and the result is `NaN`.
    ///
    /// Unlike [its `Float` counterpart](crate::Float::csc_with_period_prec_round), this function
    /// accepts arguments with any exponent, and its result never overflows or underflows. The
    /// argument is reduced exactly, modulo $u$, which for a large exponent takes time proportional
    /// to the number of bits of the exponent.
    ///
    /// At a pole, the two one-sided limits are $\infty$ and $-\infty$, and, as a
    /// [`BigExponentFloat`] has no negative zero to tell them apart, the result is `NaN`, as $1/0$
    /// is, where the `Float` function gives an infinity.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `self.significant_bits()` plus the number of bits of the exponent of the argument.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (y, o) = (&BigExponentFloat::ONE).csc_with_period_prec_round_ref(7, 5, Floor);
    /// assert_eq!(y.to_string(), "1.2");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = (&BigExponentFloat::ONE).csc_with_period_prec_round_ref(7, 5, Ceiling);
    /// assert_eq!(y.to_string(), "1.31");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = (&BigExponentFloat::ONE).csc_with_period_prec_round_ref(7, 5, Nearest);
    /// assert_eq!(y.to_string(), "1.2");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = (&BigExponentFloat::ONE).csc_with_period_prec_round_ref(7, 20, Floor);
    /// assert_eq!(y.to_string(), "1.2790470");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = (&BigExponentFloat::ONE).csc_with_period_prec_round_ref(7, 20, Ceiling);
    /// assert_eq!(y.to_string(), "1.279049");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = (&BigExponentFloat::ONE).csc_with_period_prec_round_ref(7, 20, Nearest);
    /// assert_eq!(y.to_string(), "1.279049");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn csc_with_period_prec_round_ref(
        &self,
        u: u64,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        trig_with_period_helper(TrigFunction::Csc, self, u, prec, rm, true)
    }

    /// Replaces a [`BigExponentFloat`] with its cosecant measured in $u$ths of a turn, rounding the
    /// result to the specified precision and with the specified rounding mode. An [`Ordering`] is
    /// returned, indicating whether the rounded cosecant is less than, equal to, or greater than
    /// the exact cosecant. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever
    /// this function sets the [`BigExponentFloat`] to `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,u,p,m) = \csc(2\pi x/u)+\varepsilon.
    /// $$
    /// - If $\csc(2\pi x/u)$ is zero, or the result is `NaN`, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $\csc(2\pi x/u)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\csc(2\pi x/u)|\rfloor-p+1}$.
    /// - If $\csc(2\pi x/u)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\csc(2\pi x/u)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},u,p,m)=f(\pm\infty,u,p,m)=\text{NaN}$
    /// - $f(x,0,p,m)=\text{NaN}$
    /// - $f(0,u,p,m)=\text{NaN}$
    /// - If $x/u$ is an odd multiple of $1/4$, the result is exactly $1$ or $-1$, and if it is
    ///   $\pm1/12$ or $\pm5/12$ modulo $1$, exactly $2$ or $-2$.
    /// - If $x/u$ is a multiple of $1/2$, the cosecant has a pole there, and the result is `NaN`.
    ///
    /// Unlike [its `Float` counterpart](crate::Float::csc_with_period_prec_round), this function
    /// accepts arguments with any exponent, and its result never overflows or underflows. The
    /// argument is reduced exactly, modulo $u$, which for a large exponent takes time proportional
    /// to the number of bits of the exponent.
    ///
    /// At a pole, the two one-sided limits are $\infty$ and $-\infty$, and, as a
    /// [`BigExponentFloat`] has no negative zero to tell them apart, the result is `NaN`, as $1/0$
    /// is, where the `Float` function gives an infinity.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `self.significant_bits()` plus the number of bits of the exponent of the argument.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = BigExponentFloat::ONE;
    /// assert_eq!(x.csc_with_period_prec_round_assign(7, 5, Floor), Less);
    /// assert_eq!(x.to_string(), "1.2");
    ///
    /// let mut x = BigExponentFloat::ONE;
    /// assert_eq!(x.csc_with_period_prec_round_assign(7, 5, Ceiling), Greater);
    /// assert_eq!(x.to_string(), "1.31");
    ///
    /// let mut x = BigExponentFloat::ONE;
    /// assert_eq!(x.csc_with_period_prec_round_assign(7, 5, Nearest), Less);
    /// assert_eq!(x.to_string(), "1.2");
    ///
    /// let mut x = BigExponentFloat::ONE;
    /// assert_eq!(x.csc_with_period_prec_round_assign(7, 20, Floor), Less);
    /// assert_eq!(x.to_string(), "1.2790470");
    ///
    /// let mut x = BigExponentFloat::ONE;
    /// assert_eq!(x.csc_with_period_prec_round_assign(7, 20, Ceiling), Greater);
    /// assert_eq!(x.to_string(), "1.279049");
    ///
    /// let mut x = BigExponentFloat::ONE;
    /// assert_eq!(x.csc_with_period_prec_round_assign(7, 20, Nearest), Greater);
    /// assert_eq!(x.to_string(), "1.279049");
    /// ```
    #[inline]
    pub fn csc_with_period_prec_round_assign(
        &mut self,
        u: u64,
        prec: u64,
        rm: RoundingMode,
    ) -> Ordering {
        let (result, o) = trig_with_period_helper(TrigFunction::Csc, self, u, prec, rm, true);
        *self = result;
        o
    }

    /// Computes $\csc(2\pi x/u)$, the cosecant of a [`BigExponentFloat`] measured in $u$ths of a
    /// turn, rounding the result to the nearest value of the specified precision. The
    /// [`BigExponentFloat`] is taken by value. An [`Ordering`] is also returned, indicating whether
    /// the rounded cosecant is less than, equal to, or greater than the exact cosecant. Although
    /// `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function returns a
    /// `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,u,p) = \csc(2\pi x/u)+\varepsilon.
    /// $$
    /// - If $\csc(2\pi x/u)$ is zero, or the result is `NaN`, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $\csc(2\pi x/u)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\csc(2\pi x/u)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},u,p)=f(\pm\infty,u,p)=\text{NaN}$
    /// - $f(x,0,p)=\text{NaN}$
    /// - $f(0,u,p)=\text{NaN}$
    /// - If $x/u$ is an odd multiple of $1/4$, the result is exactly $1$ or $-1$, and if it is
    ///   $\pm1/12$ or $\pm5/12$ modulo $1$, exactly $2$ or $-2$.
    /// - If $x/u$ is a multiple of $1/2$, the cosecant has a pole there, and the result is `NaN`.
    ///
    /// Unlike [its `Float` counterpart](crate::Float::csc_with_period_prec_round), this function
    /// accepts arguments with any exponent, and its result never overflows or underflows. The
    /// argument is reduced exactly, modulo $u$, which for a large exponent takes time proportional
    /// to the number of bits of the exponent.
    ///
    /// At a pole, the two one-sided limits are $\infty$ and $-\infty$, and, as a
    /// [`BigExponentFloat`] has no negative zero to tell them apart, the result is `NaN`, as $1/0$
    /// is, where the `Float` function gives an infinity.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`BigExponentFloat::csc_with_period_prec_round`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `self.significant_bits()` plus the number of bits of the exponent of the argument.
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
    /// let (y, o) = BigExponentFloat::ONE.csc_with_period_prec(7, 5);
    /// assert_eq!(y.to_string(), "1.2");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::ONE.csc_with_period_prec(7, 20);
    /// assert_eq!(y.to_string(), "1.279049");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn csc_with_period_prec(self, u: u64, prec: u64) -> (Self, Ordering) {
        trig_with_period_helper(TrigFunction::Csc, &self, u, prec, Nearest, true)
    }

    /// Computes $\csc(2\pi x/u)$, the cosecant of a [`BigExponentFloat`] measured in $u$ths of a
    /// turn, rounding the result to the nearest value of the specified precision. The
    /// [`BigExponentFloat`] is taken by reference. An [`Ordering`] is also returned, indicating
    /// whether the rounded cosecant is less than, equal to, or greater than the exact cosecant.
    /// Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,u,p) = \csc(2\pi x/u)+\varepsilon.
    /// $$
    /// - If $\csc(2\pi x/u)$ is zero, or the result is `NaN`, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $\csc(2\pi x/u)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\csc(2\pi x/u)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},u,p)=f(\pm\infty,u,p)=\text{NaN}$
    /// - $f(x,0,p)=\text{NaN}$
    /// - $f(0,u,p)=\text{NaN}$
    /// - If $x/u$ is an odd multiple of $1/4$, the result is exactly $1$ or $-1$, and if it is
    ///   $\pm1/12$ or $\pm5/12$ modulo $1$, exactly $2$ or $-2$.
    /// - If $x/u$ is a multiple of $1/2$, the cosecant has a pole there, and the result is `NaN`.
    ///
    /// Unlike [its `Float` counterpart](crate::Float::csc_with_period_prec_round), this function
    /// accepts arguments with any exponent, and its result never overflows or underflows. The
    /// argument is reduced exactly, modulo $u$, which for a large exponent takes time proportional
    /// to the number of bits of the exponent.
    ///
    /// At a pole, the two one-sided limits are $\infty$ and $-\infty$, and, as a
    /// [`BigExponentFloat`] has no negative zero to tell them apart, the result is `NaN`, as $1/0$
    /// is, where the `Float` function gives an infinity.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `self.significant_bits()` plus the number of bits of the exponent of the argument.
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
    /// let (y, o) = (&BigExponentFloat::ONE).csc_with_period_prec_ref(7, 5);
    /// assert_eq!(y.to_string(), "1.2");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = (&BigExponentFloat::ONE).csc_with_period_prec_ref(7, 20);
    /// assert_eq!(y.to_string(), "1.279049");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn csc_with_period_prec_ref(&self, u: u64, prec: u64) -> (Self, Ordering) {
        trig_with_period_helper(TrigFunction::Csc, self, u, prec, Nearest, true)
    }

    /// Replaces a [`BigExponentFloat`] with its cosecant measured in $u$ths of a turn, rounding the
    /// result to the nearest value of the specified precision. An [`Ordering`] is returned,
    /// indicating whether the rounded cosecant is less than, equal to, or greater than the exact
    /// cosecant. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this
    /// function sets the [`BigExponentFloat`] to `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,u,p) = \csc(2\pi x/u)+\varepsilon.
    /// $$
    /// - If $\csc(2\pi x/u)$ is zero, or the result is `NaN`, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $\csc(2\pi x/u)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\csc(2\pi x/u)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},u,p)=f(\pm\infty,u,p)=\text{NaN}$
    /// - $f(x,0,p)=\text{NaN}$
    /// - $f(0,u,p)=\text{NaN}$
    /// - If $x/u$ is an odd multiple of $1/4$, the result is exactly $1$ or $-1$, and if it is
    ///   $\pm1/12$ or $\pm5/12$ modulo $1$, exactly $2$ or $-2$.
    /// - If $x/u$ is a multiple of $1/2$, the cosecant has a pole there, and the result is `NaN`.
    ///
    /// Unlike [its `Float` counterpart](crate::Float::csc_with_period_prec_round), this function
    /// accepts arguments with any exponent, and its result never overflows or underflows. The
    /// argument is reduced exactly, modulo $u$, which for a large exponent takes time proportional
    /// to the number of bits of the exponent.
    ///
    /// At a pole, the two one-sided limits are $\infty$ and $-\infty$, and, as a
    /// [`BigExponentFloat`] has no negative zero to tell them apart, the result is `NaN`, as $1/0$
    /// is, where the `Float` function gives an infinity.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `self.significant_bits()` plus the number of bits of the exponent of the argument.
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
    /// assert_eq!(x.csc_with_period_prec_assign(7, 5), Less);
    /// assert_eq!(x.to_string(), "1.2");
    ///
    /// let mut x = BigExponentFloat::ONE;
    /// assert_eq!(x.csc_with_period_prec_assign(7, 20), Greater);
    /// assert_eq!(x.to_string(), "1.279049");
    /// ```
    #[inline]
    pub fn csc_with_period_prec_assign(&mut self, u: u64, prec: u64) -> Ordering {
        let (result, o) = trig_with_period_helper(TrigFunction::Csc, self, u, prec, Nearest, true);
        *self = result;
        o
    }

    /// Computes $\csc(2\pi x/u)$, the cosecant of a [`Rational`] measured in $u$ths of a turn,
    /// returning a [`BigExponentFloat`], rounding the result to the specified precision and with
    /// the specified rounding mode. The [`Rational`] is taken by value. An [`Ordering`] is also
    /// returned, indicating whether the rounded cosecant is less than, equal to, or greater than
    /// the exact cosecant. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever
    /// this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,u,p,m) = \csc(2\pi x/u)+\varepsilon.
    /// $$
    /// - If $\csc(2\pi x/u)$ is zero, or the result is `NaN`, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $\csc(2\pi x/u)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\csc(2\pi x/u)|\rfloor-p+1}$.
    /// - If $\csc(2\pi x/u)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\csc(2\pi x/u)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,0,p,m)=\text{NaN}$
    /// - $f(0,u,p,m)=\text{NaN}$
    /// - If $x/u$ is an odd multiple of $1/4$, the result is exactly $1$ or $-1$, and if it is
    ///   $\pm1/12$ or $\pm5/12$ modulo $1$, exactly $2$ or $-2$.
    /// - If $x/u$ is a multiple of $1/2$, the cosecant has a pole there, and the result is `NaN`.
    ///
    /// Unlike [its `Float` counterpart](crate::Float::csc_with_period_rational_prec_round), this
    /// function never overflows or underflows.
    ///
    /// At a pole, the two one-sided limits are $\infty$ and $-\infty$, and, as a
    /// [`BigExponentFloat`] has no negative zero to tell them apart, the result is `NaN`, as $1/0$
    /// is, where the `Float` function gives an infinity.
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`BigExponentFloat::csc_with_period_rational_prec`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `x.significant_bits()` plus the number of bits of the exponent of the argument.
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
    /// let (y, o) = BigExponentFloat::csc_with_period_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     7,
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(y.to_string(), "3.38");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::csc_with_period_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     7,
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(y.to_string(), "3.5");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::csc_with_period_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     7,
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(y.to_string(), "3.38");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::csc_with_period_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     7,
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(y.to_string(), "3.3926430");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::csc_with_period_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     7,
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(y.to_string(), "3.392647");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::csc_with_period_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     7,
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(y.to_string(), "3.392647");
    /// assert_eq!(o, Greater);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn csc_with_period_rational_prec_round(
        x: Rational,
        u: u64,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        trig_with_period_rational_helper(TrigFunction::Csc, &x, u, prec, rm, true)
    }

    /// Computes $\csc(2\pi x/u)$, the cosecant of a [`Rational`] measured in $u$ths of a turn,
    /// returning a [`BigExponentFloat`], rounding the result to the specified precision and with
    /// the specified rounding mode. The [`Rational`] is taken by reference. An [`Ordering`] is also
    /// returned, indicating whether the rounded cosecant is less than, equal to, or greater than
    /// the exact cosecant. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever
    /// this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,u,p,m) = \csc(2\pi x/u)+\varepsilon.
    /// $$
    /// - If $\csc(2\pi x/u)$ is zero, or the result is `NaN`, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $\csc(2\pi x/u)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\csc(2\pi x/u)|\rfloor-p+1}$.
    /// - If $\csc(2\pi x/u)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\csc(2\pi x/u)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,0,p,m)=\text{NaN}$
    /// - $f(0,u,p,m)=\text{NaN}$
    /// - If $x/u$ is an odd multiple of $1/4$, the result is exactly $1$ or $-1$, and if it is
    ///   $\pm1/12$ or $\pm5/12$ modulo $1$, exactly $2$ or $-2$.
    /// - If $x/u$ is a multiple of $1/2$, the cosecant has a pole there, and the result is `NaN`.
    ///
    /// Unlike [its `Float` counterpart](crate::Float::csc_with_period_rational_prec_round), this
    /// function never overflows or underflows.
    ///
    /// At a pole, the two one-sided limits are $\infty$ and $-\infty$, and, as a
    /// [`BigExponentFloat`] has no negative zero to tell them apart, the result is `NaN`, as $1/0$
    /// is, where the `Float` function gives an infinity.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `x.significant_bits()` plus the number of bits of the exponent of the argument.
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
    /// let (y, o) = BigExponentFloat::csc_with_period_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     7,
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(y.to_string(), "3.38");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::csc_with_period_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     7,
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(y.to_string(), "3.5");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::csc_with_period_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     7,
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(y.to_string(), "3.38");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::csc_with_period_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     7,
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(y.to_string(), "3.3926430");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::csc_with_period_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     7,
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(y.to_string(), "3.392647");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::csc_with_period_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     7,
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(y.to_string(), "3.392647");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn csc_with_period_rational_prec_round_ref(
        x: &Rational,
        u: u64,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        trig_with_period_rational_helper(TrigFunction::Csc, x, u, prec, rm, true)
    }

    /// Computes $\csc(2\pi x/u)$, the cosecant of a [`Rational`] measured in $u$ths of a turn,
    /// returning a [`BigExponentFloat`], rounding the result to the nearest value of the specified
    /// precision. The [`Rational`] is taken by value. An [`Ordering`] is also returned, indicating
    /// whether the rounded cosecant is less than, equal to, or greater than the exact cosecant.
    /// Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,u,p) = \csc(2\pi x/u)+\varepsilon.
    /// $$
    /// - If $\csc(2\pi x/u)$ is zero, or the result is `NaN`, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $\csc(2\pi x/u)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\csc(2\pi x/u)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,0,p)=\text{NaN}$
    /// - $f(0,u,p)=\text{NaN}$
    /// - If $x/u$ is an odd multiple of $1/4$, the result is exactly $1$ or $-1$, and if it is
    ///   $\pm1/12$ or $\pm5/12$ modulo $1$, exactly $2$ or $-2$.
    /// - If $x/u$ is a multiple of $1/2$, the cosecant has a pole there, and the result is `NaN`.
    ///
    /// Unlike [its `Float` counterpart](crate::Float::csc_with_period_rational_prec_round), this
    /// function never overflows or underflows.
    ///
    /// At a pole, the two one-sided limits are $\infty$ and $-\infty$, and, as a
    /// [`BigExponentFloat`] has no negative zero to tell them apart, the result is `NaN`, as $1/0$
    /// is, where the `Float` function gives an infinity.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`BigExponentFloat::csc_with_period_rational_prec_round`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `x.significant_bits()` plus the number of bits of the exponent of the argument.
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
    ///     BigExponentFloat::csc_with_period_rational_prec(Rational::from_unsigneds(1u8, 3), 7, 5);
    /// assert_eq!(y.to_string(), "3.38");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::csc_with_period_rational_prec(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     7,
    ///     20,
    /// );
    /// assert_eq!(y.to_string(), "3.392647");
    /// assert_eq!(o, Greater);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn csc_with_period_rational_prec(x: Rational, u: u64, prec: u64) -> (Self, Ordering) {
        trig_with_period_rational_helper(TrigFunction::Csc, &x, u, prec, Nearest, true)
    }

    /// Computes $\csc(2\pi x/u)$, the cosecant of a [`Rational`] measured in $u$ths of a turn,
    /// returning a [`BigExponentFloat`], rounding the result to the nearest value of the specified
    /// precision. The [`Rational`] is taken by reference. An [`Ordering`] is also returned,
    /// indicating whether the rounded cosecant is less than, equal to, or greater than the exact
    /// cosecant. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,u,p) = \csc(2\pi x/u)+\varepsilon.
    /// $$
    /// - If $\csc(2\pi x/u)$ is zero, or the result is `NaN`, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $\csc(2\pi x/u)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\csc(2\pi x/u)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,0,p)=\text{NaN}$
    /// - $f(0,u,p)=\text{NaN}$
    /// - If $x/u$ is an odd multiple of $1/4$, the result is exactly $1$ or $-1$, and if it is
    ///   $\pm1/12$ or $\pm5/12$ modulo $1$, exactly $2$ or $-2$.
    /// - If $x/u$ is a multiple of $1/2$, the cosecant has a pole there, and the result is `NaN`.
    ///
    /// Unlike [its `Float` counterpart](crate::Float::csc_with_period_rational_prec_round), this
    /// function never overflows or underflows.
    ///
    /// At a pole, the two one-sided limits are $\infty$ and $-\infty$, and, as a
    /// [`BigExponentFloat`] has no negative zero to tell them apart, the result is `NaN`, as $1/0$
    /// is, where the `Float` function gives an infinity.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `x.significant_bits()` plus the number of bits of the exponent of the argument.
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
    /// let (y, o) = BigExponentFloat::csc_with_period_rational_prec_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     7,
    ///     5,
    /// );
    /// assert_eq!(y.to_string(), "3.38");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::csc_with_period_rational_prec_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     7,
    ///     20,
    /// );
    /// assert_eq!(y.to_string(), "3.392647");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn csc_with_period_rational_prec_ref(x: &Rational, u: u64, prec: u64) -> (Self, Ordering) {
        trig_with_period_rational_helper(TrigFunction::Csc, x, u, prec, Nearest, true)
    }

    /// Computes $\csc(\pi x)$, the cosecant of a [`BigExponentFloat`] measured in half-turns,
    /// rounding the result to the specified precision and with the specified rounding mode. The
    /// [`BigExponentFloat`] is taken by value. An [`Ordering`] is also returned, indicating whether
    /// the rounded cosecant is less than, equal to, or greater than the exact cosecant. Although
    /// `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function returns a
    /// `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \csc(\pi x)+\varepsilon.
    /// $$
    /// - If $\csc(\pi x)$ is zero, or the result is `NaN`, $\varepsilon$ may be ignored or assumed
    ///   to be 0.
    /// - If $\csc(\pi x)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\csc(\pi x)|\rfloor-p+1}$.
    /// - If $\csc(\pi x)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\csc(\pi x)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=f(\pm\infty,p,m)=\text{NaN}$
    /// - $f(0,p,m)=\text{NaN}$
    /// - If $x$ is an odd multiple of $1/2$, the result is exactly $1$ or $-1$, and if it is
    ///   $\pm1/6$ or $\pm5/6$ modulo $2$, exactly $2$ or $-2$.
    /// - If $x$ is an integer, the cosecant has a pole there, and the result is `NaN`.
    ///
    /// Unlike [its `Float` counterpart](crate::Float::csc_pi_prec_round), this function accepts
    /// arguments with any exponent, and its result never overflows or underflows. The argument is
    /// reduced exactly, modulo 2, which for a large exponent takes time proportional to the number
    /// of bits of the exponent.
    ///
    /// At a pole, the two one-sided limits are $\infty$ and $-\infty$, and, as a
    /// [`BigExponentFloat`] has no negative zero to tell them apart, the result is `NaN`, as $1/0$
    /// is, where the `Float` function gives an infinity.
    ///
    /// If you know you'll be using `Nearest`, consider using [`BigExponentFloat::csc_pi_prec`]
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `self.significant_bits()` plus the number of bits of the exponent of the argument.
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
    /// let (y, o) = BigExponentFloat::from(0.375).csc_pi_prec_round(5, Floor);
    /// assert_eq!(y.to_string(), "1.06");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::from(0.375).csc_pi_prec_round(5, Ceiling);
    /// assert_eq!(y.to_string(), "1.12");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::from(0.375).csc_pi_prec_round(5, Nearest);
    /// assert_eq!(y.to_string(), "1.06");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::from(0.375).csc_pi_prec_round(20, Floor);
    /// assert_eq!(y.to_string(), "1.0823917");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::from(0.375).csc_pi_prec_round(20, Ceiling);
    /// assert_eq!(y.to_string(), "1.082394");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::from(0.375).csc_pi_prec_round(20, Nearest);
    /// assert_eq!(y.to_string(), "1.0823917");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn csc_pi_prec_round(self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        trig_with_period_helper(TrigFunction::Csc, &self, 2, prec, rm, true)
    }

    /// Computes $\csc(\pi x)$, the cosecant of a [`BigExponentFloat`] measured in half-turns,
    /// rounding the result to the specified precision and with the specified rounding mode. The
    /// [`BigExponentFloat`] is taken by reference. An [`Ordering`] is also returned, indicating
    /// whether the rounded cosecant is less than, equal to, or greater than the exact cosecant.
    /// Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \csc(\pi x)+\varepsilon.
    /// $$
    /// - If $\csc(\pi x)$ is zero, or the result is `NaN`, $\varepsilon$ may be ignored or assumed
    ///   to be 0.
    /// - If $\csc(\pi x)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\csc(\pi x)|\rfloor-p+1}$.
    /// - If $\csc(\pi x)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\csc(\pi x)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=f(\pm\infty,p,m)=\text{NaN}$
    /// - $f(0,p,m)=\text{NaN}$
    /// - If $x$ is an odd multiple of $1/2$, the result is exactly $1$ or $-1$, and if it is
    ///   $\pm1/6$ or $\pm5/6$ modulo $2$, exactly $2$ or $-2$.
    /// - If $x$ is an integer, the cosecant has a pole there, and the result is `NaN`.
    ///
    /// Unlike [its `Float` counterpart](crate::Float::csc_pi_prec_round), this function accepts
    /// arguments with any exponent, and its result never overflows or underflows. The argument is
    /// reduced exactly, modulo 2, which for a large exponent takes time proportional to the number
    /// of bits of the exponent.
    ///
    /// At a pole, the two one-sided limits are $\infty$ and $-\infty$, and, as a
    /// [`BigExponentFloat`] has no negative zero to tell them apart, the result is `NaN`, as $1/0$
    /// is, where the `Float` function gives an infinity.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `self.significant_bits()` plus the number of bits of the exponent of the argument.
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
    /// let (y, o) = (&BigExponentFloat::from(0.375)).csc_pi_prec_round_ref(5, Floor);
    /// assert_eq!(y.to_string(), "1.06");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = (&BigExponentFloat::from(0.375)).csc_pi_prec_round_ref(5, Ceiling);
    /// assert_eq!(y.to_string(), "1.12");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = (&BigExponentFloat::from(0.375)).csc_pi_prec_round_ref(5, Nearest);
    /// assert_eq!(y.to_string(), "1.06");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = (&BigExponentFloat::from(0.375)).csc_pi_prec_round_ref(20, Floor);
    /// assert_eq!(y.to_string(), "1.0823917");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = (&BigExponentFloat::from(0.375)).csc_pi_prec_round_ref(20, Ceiling);
    /// assert_eq!(y.to_string(), "1.082394");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = (&BigExponentFloat::from(0.375)).csc_pi_prec_round_ref(20, Nearest);
    /// assert_eq!(y.to_string(), "1.0823917");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn csc_pi_prec_round_ref(&self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        trig_with_period_helper(TrigFunction::Csc, self, 2, prec, rm, true)
    }

    /// Replaces a [`BigExponentFloat`] with its cosecant measured in half-turns, rounding the
    /// result to the specified precision and with the specified rounding mode. An [`Ordering`] is
    /// returned, indicating whether the rounded cosecant is less than, equal to, or greater than
    /// the exact cosecant. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever
    /// this function sets the [`BigExponentFloat`] to `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \csc(\pi x)+\varepsilon.
    /// $$
    /// - If $\csc(\pi x)$ is zero, or the result is `NaN`, $\varepsilon$ may be ignored or assumed
    ///   to be 0.
    /// - If $\csc(\pi x)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\csc(\pi x)|\rfloor-p+1}$.
    /// - If $\csc(\pi x)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\csc(\pi x)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=f(\pm\infty,p,m)=\text{NaN}$
    /// - $f(0,p,m)=\text{NaN}$
    /// - If $x$ is an odd multiple of $1/2$, the result is exactly $1$ or $-1$, and if it is
    ///   $\pm1/6$ or $\pm5/6$ modulo $2$, exactly $2$ or $-2$.
    /// - If $x$ is an integer, the cosecant has a pole there, and the result is `NaN`.
    ///
    /// Unlike [its `Float` counterpart](crate::Float::csc_pi_prec_round), this function accepts
    /// arguments with any exponent, and its result never overflows or underflows. The argument is
    /// reduced exactly, modulo 2, which for a large exponent takes time proportional to the number
    /// of bits of the exponent.
    ///
    /// At a pole, the two one-sided limits are $\infty$ and $-\infty$, and, as a
    /// [`BigExponentFloat`] has no negative zero to tell them apart, the result is `NaN`, as $1/0$
    /// is, where the `Float` function gives an infinity.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `self.significant_bits()` plus the number of bits of the exponent of the argument.
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
    /// assert_eq!(x.csc_pi_prec_round_assign(5, Floor), Less);
    /// assert_eq!(x.to_string(), "1.06");
    ///
    /// let mut x = BigExponentFloat::from(0.375);
    /// assert_eq!(x.csc_pi_prec_round_assign(5, Ceiling), Greater);
    /// assert_eq!(x.to_string(), "1.12");
    ///
    /// let mut x = BigExponentFloat::from(0.375);
    /// assert_eq!(x.csc_pi_prec_round_assign(5, Nearest), Less);
    /// assert_eq!(x.to_string(), "1.06");
    ///
    /// let mut x = BigExponentFloat::from(0.375);
    /// assert_eq!(x.csc_pi_prec_round_assign(20, Floor), Less);
    /// assert_eq!(x.to_string(), "1.0823917");
    ///
    /// let mut x = BigExponentFloat::from(0.375);
    /// assert_eq!(x.csc_pi_prec_round_assign(20, Ceiling), Greater);
    /// assert_eq!(x.to_string(), "1.082394");
    ///
    /// let mut x = BigExponentFloat::from(0.375);
    /// assert_eq!(x.csc_pi_prec_round_assign(20, Nearest), Less);
    /// assert_eq!(x.to_string(), "1.0823917");
    /// ```
    #[inline]
    pub fn csc_pi_prec_round_assign(&mut self, prec: u64, rm: RoundingMode) -> Ordering {
        let (result, o) = trig_with_period_helper(TrigFunction::Csc, self, 2, prec, rm, true);
        *self = result;
        o
    }

    /// Computes $\csc(\pi x)$, the cosecant of a [`BigExponentFloat`] measured in half-turns,
    /// rounding the result to the nearest value of the specified precision. The
    /// [`BigExponentFloat`] is taken by value. An [`Ordering`] is also returned, indicating whether
    /// the rounded cosecant is less than, equal to, or greater than the exact cosecant. Although
    /// `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function returns a
    /// `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,p) = \csc(\pi x)+\varepsilon.
    /// $$
    /// - If $\csc(\pi x)$ is zero, or the result is `NaN`, $\varepsilon$ may be ignored or assumed
    ///   to be 0.
    /// - If $\csc(\pi x)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\csc(\pi x)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p)=f(\pm\infty,p)=\text{NaN}$
    /// - $f(0,p)=\text{NaN}$
    /// - If $x$ is an odd multiple of $1/2$, the result is exactly $1$ or $-1$, and if it is
    ///   $\pm1/6$ or $\pm5/6$ modulo $2$, exactly $2$ or $-2$.
    /// - If $x$ is an integer, the cosecant has a pole there, and the result is `NaN`.
    ///
    /// Unlike [its `Float` counterpart](crate::Float::csc_pi_prec_round), this function accepts
    /// arguments with any exponent, and its result never overflows or underflows. The argument is
    /// reduced exactly, modulo 2, which for a large exponent takes time proportional to the number
    /// of bits of the exponent.
    ///
    /// At a pole, the two one-sided limits are $\infty$ and $-\infty$, and, as a
    /// [`BigExponentFloat`] has no negative zero to tell them apart, the result is `NaN`, as $1/0$
    /// is, where the `Float` function gives an infinity.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`BigExponentFloat::csc_pi_prec_round`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `self.significant_bits()` plus the number of bits of the exponent of the argument.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (y, o) = BigExponentFloat::from(0.375).csc_pi_prec(5);
    /// assert_eq!(y.to_string(), "1.06");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::from(0.375).csc_pi_prec(20);
    /// assert_eq!(y.to_string(), "1.0823917");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn csc_pi_prec(self, prec: u64) -> (Self, Ordering) {
        trig_with_period_helper(TrigFunction::Csc, &self, 2, prec, Nearest, true)
    }

    /// Computes $\csc(\pi x)$, the cosecant of a [`BigExponentFloat`] measured in half-turns,
    /// rounding the result to the nearest value of the specified precision. The
    /// [`BigExponentFloat`] is taken by reference. An [`Ordering`] is also returned, indicating
    /// whether the rounded cosecant is less than, equal to, or greater than the exact cosecant.
    /// Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,p) = \csc(\pi x)+\varepsilon.
    /// $$
    /// - If $\csc(\pi x)$ is zero, or the result is `NaN`, $\varepsilon$ may be ignored or assumed
    ///   to be 0.
    /// - If $\csc(\pi x)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\csc(\pi x)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p)=f(\pm\infty,p)=\text{NaN}$
    /// - $f(0,p)=\text{NaN}$
    /// - If $x$ is an odd multiple of $1/2$, the result is exactly $1$ or $-1$, and if it is
    ///   $\pm1/6$ or $\pm5/6$ modulo $2$, exactly $2$ or $-2$.
    /// - If $x$ is an integer, the cosecant has a pole there, and the result is `NaN`.
    ///
    /// Unlike [its `Float` counterpart](crate::Float::csc_pi_prec_round), this function accepts
    /// arguments with any exponent, and its result never overflows or underflows. The argument is
    /// reduced exactly, modulo 2, which for a large exponent takes time proportional to the number
    /// of bits of the exponent.
    ///
    /// At a pole, the two one-sided limits are $\infty$ and $-\infty$, and, as a
    /// [`BigExponentFloat`] has no negative zero to tell them apart, the result is `NaN`, as $1/0$
    /// is, where the `Float` function gives an infinity.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `self.significant_bits()` plus the number of bits of the exponent of the argument.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (y, o) = (&BigExponentFloat::from(0.375)).csc_pi_prec_ref(5);
    /// assert_eq!(y.to_string(), "1.06");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = (&BigExponentFloat::from(0.375)).csc_pi_prec_ref(20);
    /// assert_eq!(y.to_string(), "1.0823917");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn csc_pi_prec_ref(&self, prec: u64) -> (Self, Ordering) {
        trig_with_period_helper(TrigFunction::Csc, self, 2, prec, Nearest, true)
    }

    /// Replaces a [`BigExponentFloat`] with its cosecant measured in half-turns, rounding the
    /// result to the nearest value of the specified precision. An [`Ordering`] is returned,
    /// indicating whether the rounded cosecant is less than, equal to, or greater than the exact
    /// cosecant. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this
    /// function sets the [`BigExponentFloat`] to `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,p) = \csc(\pi x)+\varepsilon.
    /// $$
    /// - If $\csc(\pi x)$ is zero, or the result is `NaN`, $\varepsilon$ may be ignored or assumed
    ///   to be 0.
    /// - If $\csc(\pi x)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\csc(\pi x)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p)=f(\pm\infty,p)=\text{NaN}$
    /// - $f(0,p)=\text{NaN}$
    /// - If $x$ is an odd multiple of $1/2$, the result is exactly $1$ or $-1$, and if it is
    ///   $\pm1/6$ or $\pm5/6$ modulo $2$, exactly $2$ or $-2$.
    /// - If $x$ is an integer, the cosecant has a pole there, and the result is `NaN`.
    ///
    /// Unlike [its `Float` counterpart](crate::Float::csc_pi_prec_round), this function accepts
    /// arguments with any exponent, and its result never overflows or underflows. The argument is
    /// reduced exactly, modulo 2, which for a large exponent takes time proportional to the number
    /// of bits of the exponent.
    ///
    /// At a pole, the two one-sided limits are $\infty$ and $-\infty$, and, as a
    /// [`BigExponentFloat`] has no negative zero to tell them apart, the result is `NaN`, as $1/0$
    /// is, where the `Float` function gives an infinity.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `self.significant_bits()` plus the number of bits of the exponent of the argument.
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
    /// assert_eq!(x.csc_pi_prec_assign(5), Less);
    /// assert_eq!(x.to_string(), "1.06");
    ///
    /// let mut x = BigExponentFloat::from(0.375);
    /// assert_eq!(x.csc_pi_prec_assign(20), Less);
    /// assert_eq!(x.to_string(), "1.0823917");
    /// ```
    #[inline]
    pub fn csc_pi_prec_assign(&mut self, prec: u64) -> Ordering {
        let (result, o) = trig_with_period_helper(TrigFunction::Csc, self, 2, prec, Nearest, true);
        *self = result;
        o
    }

    /// Computes $\csc(\pi x)$, the cosecant of a [`Rational`] measured in half-turns, returning a
    /// [`BigExponentFloat`], rounding the result to the specified precision and with the specified
    /// rounding mode. The [`Rational`] is taken by value. An [`Ordering`] is also returned,
    /// indicating whether the rounded cosecant is less than, equal to, or greater than the exact
    /// cosecant. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \csc(\pi x)+\varepsilon.
    /// $$
    /// - If $\csc(\pi x)$ is zero, or the result is `NaN`, $\varepsilon$ may be ignored or assumed
    ///   to be 0.
    /// - If $\csc(\pi x)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\csc(\pi x)|\rfloor-p+1}$.
    /// - If $\csc(\pi x)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\csc(\pi x)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(0,p,m)=\text{NaN}$
    /// - If $x$ is an odd multiple of $1/2$, the result is exactly $1$ or $-1$, and if it is
    ///   $\pm1/6$ or $\pm5/6$ modulo $2$, exactly $2$ or $-2$.
    /// - If $x$ is an integer, the cosecant has a pole there, and the result is `NaN`.
    ///
    /// Unlike [its `Float` counterpart](crate::Float::csc_pi_rational_prec_round), this function
    /// never overflows or underflows.
    ///
    /// At a pole, the two one-sided limits are $\infty$ and $-\infty$, and, as a
    /// [`BigExponentFloat`] has no negative zero to tell them apart, the result is `NaN`, as $1/0$
    /// is, where the `Float` function gives an infinity.
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`BigExponentFloat::csc_pi_rational_prec`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `x.significant_bits()` plus the number of bits of the exponent of the argument.
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
    /// let (y, o) = BigExponentFloat::csc_pi_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 5),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(y.to_string(), "1.69");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::csc_pi_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 5),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(y.to_string(), "1.8");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::csc_pi_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 5),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(y.to_string(), "1.69");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::csc_pi_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 5),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(y.to_string(), "1.701302");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::csc_pi_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 5),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(y.to_string(), "1.7013035");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::csc_pi_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 5),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(y.to_string(), "1.701302");
    /// assert_eq!(o, Less);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn csc_pi_rational_prec_round(
        x: Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        trig_with_period_rational_helper(TrigFunction::Csc, &x, 2, prec, rm, true)
    }

    /// Computes $\csc(\pi x)$, the cosecant of a [`Rational`] measured in half-turns, returning a
    /// [`BigExponentFloat`], rounding the result to the specified precision and with the specified
    /// rounding mode. The [`Rational`] is taken by reference. An [`Ordering`] is also returned,
    /// indicating whether the rounded cosecant is less than, equal to, or greater than the exact
    /// cosecant. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \csc(\pi x)+\varepsilon.
    /// $$
    /// - If $\csc(\pi x)$ is zero, or the result is `NaN`, $\varepsilon$ may be ignored or assumed
    ///   to be 0.
    /// - If $\csc(\pi x)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\csc(\pi x)|\rfloor-p+1}$.
    /// - If $\csc(\pi x)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\csc(\pi x)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(0,p,m)=\text{NaN}$
    /// - If $x$ is an odd multiple of $1/2$, the result is exactly $1$ or $-1$, and if it is
    ///   $\pm1/6$ or $\pm5/6$ modulo $2$, exactly $2$ or $-2$.
    /// - If $x$ is an integer, the cosecant has a pole there, and the result is `NaN`.
    ///
    /// Unlike [its `Float` counterpart](crate::Float::csc_pi_rational_prec_round), this function
    /// never overflows or underflows.
    ///
    /// At a pole, the two one-sided limits are $\infty$ and $-\infty$, and, as a
    /// [`BigExponentFloat`] has no negative zero to tell them apart, the result is `NaN`, as $1/0$
    /// is, where the `Float` function gives an infinity.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `x.significant_bits()` plus the number of bits of the exponent of the argument.
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
    /// let (y, o) = BigExponentFloat::csc_pi_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(1u8, 5),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(y.to_string(), "1.69");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::csc_pi_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(1u8, 5),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(y.to_string(), "1.8");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::csc_pi_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(1u8, 5),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(y.to_string(), "1.69");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::csc_pi_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(1u8, 5),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(y.to_string(), "1.701302");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::csc_pi_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(1u8, 5),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(y.to_string(), "1.7013035");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::csc_pi_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(1u8, 5),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(y.to_string(), "1.701302");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn csc_pi_rational_prec_round_ref(
        x: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        trig_with_period_rational_helper(TrigFunction::Csc, x, 2, prec, rm, true)
    }

    /// Computes $\csc(\pi x)$, the cosecant of a [`Rational`] measured in half-turns, returning a
    /// [`BigExponentFloat`], rounding the result to the nearest value of the specified precision.
    /// The [`Rational`] is taken by value. An [`Ordering`] is also returned, indicating whether the
    /// rounded cosecant is less than, equal to, or greater than the exact cosecant. Although `NaN`s
    /// are not comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it
    /// also returns `Equal`.
    ///
    /// $$
    /// f(x,p) = \csc(\pi x)+\varepsilon.
    /// $$
    /// - If $\csc(\pi x)$ is zero, or the result is `NaN`, $\varepsilon$ may be ignored or assumed
    ///   to be 0.
    /// - If $\csc(\pi x)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\csc(\pi x)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(0,p)=\text{NaN}$
    /// - If $x$ is an odd multiple of $1/2$, the result is exactly $1$ or $-1$, and if it is
    ///   $\pm1/6$ or $\pm5/6$ modulo $2$, exactly $2$ or $-2$.
    /// - If $x$ is an integer, the cosecant has a pole there, and the result is `NaN`.
    ///
    /// Unlike [its `Float` counterpart](crate::Float::csc_pi_rational_prec_round), this function
    /// never overflows or underflows.
    ///
    /// At a pole, the two one-sided limits are $\infty$ and $-\infty$, and, as a
    /// [`BigExponentFloat`] has no negative zero to tell them apart, the result is `NaN`, as $1/0$
    /// is, where the `Float` function gives an infinity.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`BigExponentFloat::csc_pi_rational_prec_round`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `x.significant_bits()` plus the number of bits of the exponent of the argument.
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
    /// let (y, o) = BigExponentFloat::csc_pi_rational_prec(Rational::from_unsigneds(1u8, 5), 5);
    /// assert_eq!(y.to_string(), "1.69");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::csc_pi_rational_prec(Rational::from_unsigneds(1u8, 5), 20);
    /// assert_eq!(y.to_string(), "1.701302");
    /// assert_eq!(o, Less);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn csc_pi_rational_prec(x: Rational, prec: u64) -> (Self, Ordering) {
        trig_with_period_rational_helper(TrigFunction::Csc, &x, 2, prec, Nearest, true)
    }

    /// Computes $\csc(\pi x)$, the cosecant of a [`Rational`] measured in half-turns, returning a
    /// [`BigExponentFloat`], rounding the result to the nearest value of the specified precision.
    /// The [`Rational`] is taken by reference. An [`Ordering`] is also returned, indicating whether
    /// the rounded cosecant is less than, equal to, or greater than the exact cosecant. Although
    /// `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function returns a
    /// `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,p) = \csc(\pi x)+\varepsilon.
    /// $$
    /// - If $\csc(\pi x)$ is zero, or the result is `NaN`, $\varepsilon$ may be ignored or assumed
    ///   to be 0.
    /// - If $\csc(\pi x)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\csc(\pi x)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(0,p)=\text{NaN}$
    /// - If $x$ is an odd multiple of $1/2$, the result is exactly $1$ or $-1$, and if it is
    ///   $\pm1/6$ or $\pm5/6$ modulo $2$, exactly $2$ or $-2$.
    /// - If $x$ is an integer, the cosecant has a pole there, and the result is `NaN`.
    ///
    /// Unlike [its `Float` counterpart](crate::Float::csc_pi_rational_prec_round), this function
    /// never overflows or underflows.
    ///
    /// At a pole, the two one-sided limits are $\infty$ and $-\infty$, and, as a
    /// [`BigExponentFloat`] has no negative zero to tell them apart, the result is `NaN`, as $1/0$
    /// is, where the `Float` function gives an infinity.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `x.significant_bits()` plus the number of bits of the exponent of the argument.
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
    ///     BigExponentFloat::csc_pi_rational_prec_ref(&Rational::from_unsigneds(1u8, 5), 5);
    /// assert_eq!(y.to_string(), "1.69");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) =
    ///     BigExponentFloat::csc_pi_rational_prec_ref(&Rational::from_unsigneds(1u8, 5), 20);
    /// assert_eq!(y.to_string(), "1.701302");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn csc_pi_rational_prec_ref(x: &Rational, prec: u64) -> (Self, Ordering) {
        trig_with_period_rational_helper(TrigFunction::Csc, x, 2, prec, Nearest, true)
    }
}
