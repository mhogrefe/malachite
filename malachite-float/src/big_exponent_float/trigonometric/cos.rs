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
    /// Computes the cosine of a [`BigExponentFloat`], rounding the result to the specified
    /// precision and with the specified rounding mode. The [`BigExponentFloat`] is taken by value.
    /// An [`Ordering`] is also returned, indicating whether the rounded cosine is less than, equal
    /// to, or greater than the exact cosine. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \cos x+\varepsilon.
    /// $$
    /// - If $\cos x$ is zero, or the result is `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\cos x$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\cos x|\rfloor-p+1}$.
    /// - If $\cos x$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\cos x|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=f(\pm\infty,p,m)=\text{NaN}$
    /// - $f(0,p,m)=1$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::cos_prec_round), this function accepts
    /// arguments with any exponent, and its result never overflows or underflows.
    ///
    /// Warning: an argument with exponent $e > 0$ is reduced modulo $\pi/2$, which takes about $e$
    /// bits of $\pi$. The time and memory needed therefore grow with the exponent, without any
    /// limit: an argument near $2^{2^{40}}$ needs about $2^{40}$ bits, or 128 GiB, of $\pi$.
    ///
    /// If you know you'll be using `Nearest`, consider using [`BigExponentFloat::cos_prec`]
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
    /// cosine of a nonzero rational number is transcendental).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (y, o) = BigExponentFloat::ONE.cos_prec_round(5, Floor);
    /// assert_eq!(y.to_string(), "0.531");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::ONE.cos_prec_round(5, Ceiling);
    /// assert_eq!(y.to_string(), "0.562");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::ONE.cos_prec_round(5, Nearest);
    /// assert_eq!(y.to_string(), "0.531");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::ONE.cos_prec_round(20, Floor);
    /// assert_eq!(y.to_string(), "0.5403023");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::ONE.cos_prec_round(20, Ceiling);
    /// assert_eq!(y.to_string(), "0.54030323");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::ONE.cos_prec_round(20, Nearest);
    /// assert_eq!(y.to_string(), "0.5403023");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn cos_prec_round(self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        trig_helper(TrigFunction::Cos, &self, prec, rm, true)
    }

    /// Computes the cosine of a [`BigExponentFloat`], rounding the result to the specified
    /// precision and with the specified rounding mode. The [`BigExponentFloat`] is taken by
    /// reference. An [`Ordering`] is also returned, indicating whether the rounded cosine is less
    /// than, equal to, or greater than the exact cosine. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \cos x+\varepsilon.
    /// $$
    /// - If $\cos x$ is zero, or the result is `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\cos x$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\cos x|\rfloor-p+1}$.
    /// - If $\cos x$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\cos x|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=f(\pm\infty,p,m)=\text{NaN}$
    /// - $f(0,p,m)=1$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::cos_prec_round), this function accepts
    /// arguments with any exponent, and its result never overflows or underflows.
    ///
    /// Warning: an argument with exponent $e > 0$ is reduced modulo $\pi/2$, which takes about $e$
    /// bits of $\pi$. The time and memory needed therefore grow with the exponent, without any
    /// limit: an argument near $2^{2^{40}}$ needs about $2^{40}$ bits, or 128 GiB, of $\pi$.
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
    /// cosine of a nonzero rational number is transcendental).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (y, o) = (&BigExponentFloat::ONE).cos_prec_round_ref(5, Floor);
    /// assert_eq!(y.to_string(), "0.531");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = (&BigExponentFloat::ONE).cos_prec_round_ref(5, Ceiling);
    /// assert_eq!(y.to_string(), "0.562");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = (&BigExponentFloat::ONE).cos_prec_round_ref(5, Nearest);
    /// assert_eq!(y.to_string(), "0.531");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = (&BigExponentFloat::ONE).cos_prec_round_ref(20, Floor);
    /// assert_eq!(y.to_string(), "0.5403023");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = (&BigExponentFloat::ONE).cos_prec_round_ref(20, Ceiling);
    /// assert_eq!(y.to_string(), "0.54030323");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = (&BigExponentFloat::ONE).cos_prec_round_ref(20, Nearest);
    /// assert_eq!(y.to_string(), "0.5403023");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn cos_prec_round_ref(&self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        trig_helper(TrigFunction::Cos, self, prec, rm, true)
    }

    /// Replaces a [`BigExponentFloat`] with its cosine, rounding the result to the specified
    /// precision and with the specified rounding mode. An [`Ordering`] is returned, indicating
    /// whether the rounded cosine is less than, equal to, or greater than the exact cosine.
    /// Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function sets
    /// the [`BigExponentFloat`] to `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \cos x+\varepsilon.
    /// $$
    /// - If $\cos x$ is zero, or the result is `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\cos x$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\cos x|\rfloor-p+1}$.
    /// - If $\cos x$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\cos x|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=f(\pm\infty,p,m)=\text{NaN}$
    /// - $f(0,p,m)=1$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::cos_prec_round), this function accepts
    /// arguments with any exponent, and its result never overflows or underflows.
    ///
    /// Warning: an argument with exponent $e > 0$ is reduced modulo $\pi/2$, which takes about $e$
    /// bits of $\pi$. The time and memory needed therefore grow with the exponent, without any
    /// limit: an argument near $2^{2^{40}}$ needs about $2^{40}$ bits, or 128 GiB, of $\pi$.
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
    /// cosine of a nonzero rational number is transcendental).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = BigExponentFloat::ONE;
    /// assert_eq!(x.cos_prec_round_assign(5, Floor), Less);
    /// assert_eq!(x.to_string(), "0.531");
    ///
    /// let mut x = BigExponentFloat::ONE;
    /// assert_eq!(x.cos_prec_round_assign(5, Ceiling), Greater);
    /// assert_eq!(x.to_string(), "0.562");
    ///
    /// let mut x = BigExponentFloat::ONE;
    /// assert_eq!(x.cos_prec_round_assign(5, Nearest), Less);
    /// assert_eq!(x.to_string(), "0.531");
    ///
    /// let mut x = BigExponentFloat::ONE;
    /// assert_eq!(x.cos_prec_round_assign(20, Floor), Less);
    /// assert_eq!(x.to_string(), "0.5403023");
    ///
    /// let mut x = BigExponentFloat::ONE;
    /// assert_eq!(x.cos_prec_round_assign(20, Ceiling), Greater);
    /// assert_eq!(x.to_string(), "0.54030323");
    ///
    /// let mut x = BigExponentFloat::ONE;
    /// assert_eq!(x.cos_prec_round_assign(20, Nearest), Less);
    /// assert_eq!(x.to_string(), "0.5403023");
    /// ```
    #[inline]
    pub fn cos_prec_round_assign(&mut self, prec: u64, rm: RoundingMode) -> Ordering {
        let (result, o) = trig_helper(TrigFunction::Cos, self, prec, rm, true);
        *self = result;
        o
    }

    /// Computes the cosine of a [`BigExponentFloat`], rounding the result to the nearest value of
    /// the specified precision. The [`BigExponentFloat`] is taken by value. An [`Ordering`] is also
    /// returned, indicating whether the rounded cosine is less than, equal to, or greater than the
    /// exact cosine. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,p) = \cos x+\varepsilon.
    /// $$
    /// - If $\cos x$ is zero, or the result is `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\cos x$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2 |\cos
    ///   x|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p)=f(\pm\infty,p)=\text{NaN}$
    /// - $f(0,p)=1$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::cos_prec_round), this function accepts
    /// arguments with any exponent, and its result never overflows or underflows.
    ///
    /// Warning: an argument with exponent $e > 0$ is reduced modulo $\pi/2$, which takes about $e$
    /// bits of $\pi$. The time and memory needed therefore grow with the exponent, without any
    /// limit: an argument near $2^{2^{40}}$ needs about $2^{40}$ bits, or 128 GiB, of $\pi$.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`BigExponentFloat::cos_prec_round`] instead.
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
    /// let (y, o) = BigExponentFloat::ONE.cos_prec(5);
    /// assert_eq!(y.to_string(), "0.531");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::ONE.cos_prec(20);
    /// assert_eq!(y.to_string(), "0.5403023");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn cos_prec(self, prec: u64) -> (Self, Ordering) {
        trig_helper(TrigFunction::Cos, &self, prec, Nearest, true)
    }

    /// Computes the cosine of a [`BigExponentFloat`], rounding the result to the nearest value of
    /// the specified precision. The [`BigExponentFloat`] is taken by reference. An [`Ordering`] is
    /// also returned, indicating whether the rounded cosine is less than, equal to, or greater than
    /// the exact cosine. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever
    /// this function returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,p) = \cos x+\varepsilon.
    /// $$
    /// - If $\cos x$ is zero, or the result is `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\cos x$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2 |\cos
    ///   x|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p)=f(\pm\infty,p)=\text{NaN}$
    /// - $f(0,p)=1$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::cos_prec_round), this function accepts
    /// arguments with any exponent, and its result never overflows or underflows.
    ///
    /// Warning: an argument with exponent $e > 0$ is reduced modulo $\pi/2$, which takes about $e$
    /// bits of $\pi$. The time and memory needed therefore grow with the exponent, without any
    /// limit: an argument near $2^{2^{40}}$ needs about $2^{40}$ bits, or 128 GiB, of $\pi$.
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
    /// let (y, o) = (&BigExponentFloat::ONE).cos_prec_ref(5);
    /// assert_eq!(y.to_string(), "0.531");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = (&BigExponentFloat::ONE).cos_prec_ref(20);
    /// assert_eq!(y.to_string(), "0.5403023");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn cos_prec_ref(&self, prec: u64) -> (Self, Ordering) {
        trig_helper(TrigFunction::Cos, self, prec, Nearest, true)
    }

    /// Replaces a [`BigExponentFloat`] with its cosine, rounding the result to the nearest value of
    /// the specified precision. An [`Ordering`] is returned, indicating whether the rounded cosine
    /// is less than, equal to, or greater than the exact cosine. Although `NaN`s are not comparable
    /// to any [`BigExponentFloat`], whenever this function sets the [`BigExponentFloat`] to `NaN`
    /// it also returns `Equal`.
    ///
    /// $$
    /// f(x,p) = \cos x+\varepsilon.
    /// $$
    /// - If $\cos x$ is zero, or the result is `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\cos x$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2 |\cos
    ///   x|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p)=f(\pm\infty,p)=\text{NaN}$
    /// - $f(0,p)=1$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::cos_prec_round), this function accepts
    /// arguments with any exponent, and its result never overflows or underflows.
    ///
    /// Warning: an argument with exponent $e > 0$ is reduced modulo $\pi/2$, which takes about $e$
    /// bits of $\pi$. The time and memory needed therefore grow with the exponent, without any
    /// limit: an argument near $2^{2^{40}}$ needs about $2^{40}$ bits, or 128 GiB, of $\pi$.
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
    /// assert_eq!(x.cos_prec_assign(5), Less);
    /// assert_eq!(x.to_string(), "0.531");
    ///
    /// let mut x = BigExponentFloat::ONE;
    /// assert_eq!(x.cos_prec_assign(20), Less);
    /// assert_eq!(x.to_string(), "0.5403023");
    /// ```
    #[inline]
    pub fn cos_prec_assign(&mut self, prec: u64) -> Ordering {
        let (result, o) = trig_helper(TrigFunction::Cos, self, prec, Nearest, true);
        *self = result;
        o
    }

    /// Computes the cosine of a [`Rational`], returning a [`BigExponentFloat`], rounding the result
    /// to the specified precision and with the specified rounding mode. The [`Rational`] is taken
    /// by value. An [`Ordering`] is also returned, indicating whether the rounded cosine is less
    /// than, equal to, or greater than the exact cosine. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \cos x+\varepsilon.
    /// $$
    /// - If $\cos x$ is zero, or the result is `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\cos x$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\cos x|\rfloor-p+1}$.
    /// - If $\cos x$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\cos x|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(0,p,m)=1$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::cos_rational_prec_round), this function never
    /// overflows or underflows.
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`BigExponentFloat::cos_rational_prec`] instead.
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
    /// cosine of a nonzero rational number is transcendental).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (y, o) =
    ///     BigExponentFloat::cos_rational_prec_round(Rational::from_unsigneds(1u8, 3), 5, Floor);
    /// assert_eq!(y.to_string(), "0.938");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) =
    ///     BigExponentFloat::cos_rational_prec_round(Rational::from_unsigneds(1u8, 3), 5, Ceiling);
    /// assert_eq!(y.to_string(), "0.969");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) =
    ///     BigExponentFloat::cos_rational_prec_round(Rational::from_unsigneds(1u8, 3), 5, Nearest);
    /// assert_eq!(y.to_string(), "0.938");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) =
    ///     BigExponentFloat::cos_rational_prec_round(Rational::from_unsigneds(1u8, 3), 20, Floor);
    /// assert_eq!(y.to_string(), "0.94495678");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::cos_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(y.to_string(), "0.9449577");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::cos_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(y.to_string(), "0.94495678");
    /// assert_eq!(o, Less);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn cos_rational_prec_round(x: Rational, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        trig_rational_helper(TrigFunction::Cos, &x, prec, rm, true)
    }

    /// Computes the cosine of a [`Rational`], returning a [`BigExponentFloat`], rounding the result
    /// to the specified precision and with the specified rounding mode. The [`Rational`] is taken
    /// by reference. An [`Ordering`] is also returned, indicating whether the rounded cosine is
    /// less than, equal to, or greater than the exact cosine. Although `NaN`s are not comparable to
    /// any [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \cos x+\varepsilon.
    /// $$
    /// - If $\cos x$ is zero, or the result is `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\cos x$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\cos x|\rfloor-p+1}$.
    /// - If $\cos x$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\cos x|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(0,p,m)=1$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::cos_rational_prec_round), this function never
    /// overflows or underflows.
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
    /// cosine of a nonzero rational number is transcendental).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (y, o) = BigExponentFloat::cos_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(y.to_string(), "0.938");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::cos_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(y.to_string(), "0.969");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::cos_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(y.to_string(), "0.938");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::cos_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(y.to_string(), "0.94495678");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::cos_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(y.to_string(), "0.9449577");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::cos_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(y.to_string(), "0.94495678");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn cos_rational_prec_round_ref(
        x: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        trig_rational_helper(TrigFunction::Cos, x, prec, rm, true)
    }

    /// Computes the cosine of a [`Rational`], returning a [`BigExponentFloat`], rounding the result
    /// to the nearest value of the specified precision. The [`Rational`] is taken by value. An
    /// [`Ordering`] is also returned, indicating whether the rounded cosine is less than, equal to,
    /// or greater than the exact cosine. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,p) = \cos x+\varepsilon.
    /// $$
    /// - If $\cos x$ is zero, or the result is `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\cos x$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2 |\cos
    ///   x|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(0,p)=1$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::cos_rational_prec_round), this function never
    /// overflows or underflows.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`BigExponentFloat::cos_rational_prec_round`] instead.
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
    /// let (y, o) = BigExponentFloat::cos_rational_prec(Rational::from_unsigneds(1u8, 3), 5);
    /// assert_eq!(y.to_string(), "0.938");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::cos_rational_prec(Rational::from_unsigneds(1u8, 3), 20);
    /// assert_eq!(y.to_string(), "0.94495678");
    /// assert_eq!(o, Less);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn cos_rational_prec(x: Rational, prec: u64) -> (Self, Ordering) {
        trig_rational_helper(TrigFunction::Cos, &x, prec, Nearest, true)
    }

    /// Computes the cosine of a [`Rational`], returning a [`BigExponentFloat`], rounding the result
    /// to the nearest value of the specified precision. The [`Rational`] is taken by reference. An
    /// [`Ordering`] is also returned, indicating whether the rounded cosine is less than, equal to,
    /// or greater than the exact cosine. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,p) = \cos x+\varepsilon.
    /// $$
    /// - If $\cos x$ is zero, or the result is `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\cos x$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2 |\cos
    ///   x|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(0,p)=1$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::cos_rational_prec_round), this function never
    /// overflows or underflows.
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
    /// let (y, o) = BigExponentFloat::cos_rational_prec_ref(&Rational::from_unsigneds(1u8, 3), 5);
    /// assert_eq!(y.to_string(), "0.938");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::cos_rational_prec_ref(&Rational::from_unsigneds(1u8, 3), 20);
    /// assert_eq!(y.to_string(), "0.94495678");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn cos_rational_prec_ref(x: &Rational, prec: u64) -> (Self, Ordering) {
        trig_rational_helper(TrigFunction::Cos, x, prec, Nearest, true)
    }

    /// Computes $\cos(2\pi x/u)$, the cosine of a [`BigExponentFloat`] measured in $u$ths of a
    /// turn, rounding the result to the specified precision and with the specified rounding mode.
    /// The [`BigExponentFloat`] is taken by value. An [`Ordering`] is also returned, indicating
    /// whether the rounded cosine is less than, equal to, or greater than the exact cosine.
    /// Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,u,p,m) = \cos(2\pi x/u)+\varepsilon.
    /// $$
    /// - If $\cos(2\pi x/u)$ is zero, or the result is `NaN`, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $\cos(2\pi x/u)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\cos(2\pi x/u)|\rfloor-p+1}$.
    /// - If $\cos(2\pi x/u)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\cos(2\pi x/u)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},u,p,m)=f(\pm\infty,u,p,m)=\text{NaN}$
    /// - $f(x,0,p,m)=\text{NaN}$
    /// - $f(0,u,p,m)=1$
    /// - If $x/u$ is a multiple of $1/2$, the result is exactly $1$ or $-1$; if it is an odd
    ///   multiple of $1/4$, exactly $0$; and if it is $\pm1/6$ or $\pm1/3$ modulo $1$, exactly
    ///   $1/2$ or $-1/2$.
    ///
    /// Unlike [its `Float` counterpart](crate::Float::cos_with_period_prec_round), this function
    /// accepts arguments with any exponent, and its result never overflows or underflows. The
    /// argument is reduced exactly, modulo $u$, which for a large exponent takes time proportional
    /// to the number of bits of the exponent.
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`BigExponentFloat::cos_with_period_prec`] instead.
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
    /// let (y, o) = BigExponentFloat::ONE.cos_with_period_prec_round(7, 5, Floor);
    /// assert_eq!(y.to_string(), "0.594");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::ONE.cos_with_period_prec_round(7, 5, Ceiling);
    /// assert_eq!(y.to_string(), "0.62");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::ONE.cos_with_period_prec_round(7, 5, Nearest);
    /// assert_eq!(y.to_string(), "0.62");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::ONE.cos_with_period_prec_round(7, 20, Floor);
    /// assert_eq!(y.to_string(), "0.623489");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::ONE.cos_with_period_prec_round(7, 20, Ceiling);
    /// assert_eq!(y.to_string(), "0.62349033");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::ONE.cos_with_period_prec_round(7, 20, Nearest);
    /// assert_eq!(y.to_string(), "0.623489");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn cos_with_period_prec_round(
        self,
        u: u64,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        trig_with_period_helper(TrigFunction::Cos, &self, u, prec, rm, true)
    }

    /// Computes $\cos(2\pi x/u)$, the cosine of a [`BigExponentFloat`] measured in $u$ths of a
    /// turn, rounding the result to the specified precision and with the specified rounding mode.
    /// The [`BigExponentFloat`] is taken by reference. An [`Ordering`] is also returned, indicating
    /// whether the rounded cosine is less than, equal to, or greater than the exact cosine.
    /// Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,u,p,m) = \cos(2\pi x/u)+\varepsilon.
    /// $$
    /// - If $\cos(2\pi x/u)$ is zero, or the result is `NaN`, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $\cos(2\pi x/u)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\cos(2\pi x/u)|\rfloor-p+1}$.
    /// - If $\cos(2\pi x/u)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\cos(2\pi x/u)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},u,p,m)=f(\pm\infty,u,p,m)=\text{NaN}$
    /// - $f(x,0,p,m)=\text{NaN}$
    /// - $f(0,u,p,m)=1$
    /// - If $x/u$ is a multiple of $1/2$, the result is exactly $1$ or $-1$; if it is an odd
    ///   multiple of $1/4$, exactly $0$; and if it is $\pm1/6$ or $\pm1/3$ modulo $1$, exactly
    ///   $1/2$ or $-1/2$.
    ///
    /// Unlike [its `Float` counterpart](crate::Float::cos_with_period_prec_round), this function
    /// accepts arguments with any exponent, and its result never overflows or underflows. The
    /// argument is reduced exactly, modulo $u$, which for a large exponent takes time proportional
    /// to the number of bits of the exponent.
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
    /// let (y, o) = (&BigExponentFloat::ONE).cos_with_period_prec_round_ref(7, 5, Floor);
    /// assert_eq!(y.to_string(), "0.594");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = (&BigExponentFloat::ONE).cos_with_period_prec_round_ref(7, 5, Ceiling);
    /// assert_eq!(y.to_string(), "0.62");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = (&BigExponentFloat::ONE).cos_with_period_prec_round_ref(7, 5, Nearest);
    /// assert_eq!(y.to_string(), "0.62");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = (&BigExponentFloat::ONE).cos_with_period_prec_round_ref(7, 20, Floor);
    /// assert_eq!(y.to_string(), "0.623489");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = (&BigExponentFloat::ONE).cos_with_period_prec_round_ref(7, 20, Ceiling);
    /// assert_eq!(y.to_string(), "0.62349033");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = (&BigExponentFloat::ONE).cos_with_period_prec_round_ref(7, 20, Nearest);
    /// assert_eq!(y.to_string(), "0.623489");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn cos_with_period_prec_round_ref(
        &self,
        u: u64,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        trig_with_period_helper(TrigFunction::Cos, self, u, prec, rm, true)
    }

    /// Replaces a [`BigExponentFloat`] with its cosine measured in $u$ths of a turn, rounding the
    /// result to the specified precision and with the specified rounding mode. An [`Ordering`] is
    /// returned, indicating whether the rounded cosine is less than, equal to, or greater than the
    /// exact cosine. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this
    /// function sets the [`BigExponentFloat`] to `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,u,p,m) = \cos(2\pi x/u)+\varepsilon.
    /// $$
    /// - If $\cos(2\pi x/u)$ is zero, or the result is `NaN`, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $\cos(2\pi x/u)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\cos(2\pi x/u)|\rfloor-p+1}$.
    /// - If $\cos(2\pi x/u)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\cos(2\pi x/u)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},u,p,m)=f(\pm\infty,u,p,m)=\text{NaN}$
    /// - $f(x,0,p,m)=\text{NaN}$
    /// - $f(0,u,p,m)=1$
    /// - If $x/u$ is a multiple of $1/2$, the result is exactly $1$ or $-1$; if it is an odd
    ///   multiple of $1/4$, exactly $0$; and if it is $\pm1/6$ or $\pm1/3$ modulo $1$, exactly
    ///   $1/2$ or $-1/2$.
    ///
    /// Unlike [its `Float` counterpart](crate::Float::cos_with_period_prec_round), this function
    /// accepts arguments with any exponent, and its result never overflows or underflows. The
    /// argument is reduced exactly, modulo $u$, which for a large exponent takes time proportional
    /// to the number of bits of the exponent.
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
    /// assert_eq!(x.cos_with_period_prec_round_assign(7, 5, Floor), Less);
    /// assert_eq!(x.to_string(), "0.594");
    ///
    /// let mut x = BigExponentFloat::ONE;
    /// assert_eq!(x.cos_with_period_prec_round_assign(7, 5, Ceiling), Greater);
    /// assert_eq!(x.to_string(), "0.62");
    ///
    /// let mut x = BigExponentFloat::ONE;
    /// assert_eq!(x.cos_with_period_prec_round_assign(7, 5, Nearest), Greater);
    /// assert_eq!(x.to_string(), "0.62");
    ///
    /// let mut x = BigExponentFloat::ONE;
    /// assert_eq!(x.cos_with_period_prec_round_assign(7, 20, Floor), Less);
    /// assert_eq!(x.to_string(), "0.623489");
    ///
    /// let mut x = BigExponentFloat::ONE;
    /// assert_eq!(x.cos_with_period_prec_round_assign(7, 20, Ceiling), Greater);
    /// assert_eq!(x.to_string(), "0.62349033");
    ///
    /// let mut x = BigExponentFloat::ONE;
    /// assert_eq!(x.cos_with_period_prec_round_assign(7, 20, Nearest), Less);
    /// assert_eq!(x.to_string(), "0.623489");
    /// ```
    #[inline]
    pub fn cos_with_period_prec_round_assign(
        &mut self,
        u: u64,
        prec: u64,
        rm: RoundingMode,
    ) -> Ordering {
        let (result, o) = trig_with_period_helper(TrigFunction::Cos, self, u, prec, rm, true);
        *self = result;
        o
    }

    /// Computes $\cos(2\pi x/u)$, the cosine of a [`BigExponentFloat`] measured in $u$ths of a
    /// turn, rounding the result to the nearest value of the specified precision. The
    /// [`BigExponentFloat`] is taken by value. An [`Ordering`] is also returned, indicating whether
    /// the rounded cosine is less than, equal to, or greater than the exact cosine. Although `NaN`s
    /// are not comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it
    /// also returns `Equal`.
    ///
    /// $$
    /// f(x,u,p) = \cos(2\pi x/u)+\varepsilon.
    /// $$
    /// - If $\cos(2\pi x/u)$ is zero, or the result is `NaN`, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $\cos(2\pi x/u)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\cos(2\pi x/u)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},u,p)=f(\pm\infty,u,p)=\text{NaN}$
    /// - $f(x,0,p)=\text{NaN}$
    /// - $f(0,u,p)=1$
    /// - If $x/u$ is a multiple of $1/2$, the result is exactly $1$ or $-1$; if it is an odd
    ///   multiple of $1/4$, exactly $0$; and if it is $\pm1/6$ or $\pm1/3$ modulo $1$, exactly
    ///   $1/2$ or $-1/2$.
    ///
    /// Unlike [its `Float` counterpart](crate::Float::cos_with_period_prec_round), this function
    /// accepts arguments with any exponent, and its result never overflows or underflows. The
    /// argument is reduced exactly, modulo $u$, which for a large exponent takes time proportional
    /// to the number of bits of the exponent.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`BigExponentFloat::cos_with_period_prec_round`] instead.
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
    /// let (y, o) = BigExponentFloat::ONE.cos_with_period_prec(7, 5);
    /// assert_eq!(y.to_string(), "0.62");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::ONE.cos_with_period_prec(7, 20);
    /// assert_eq!(y.to_string(), "0.623489");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn cos_with_period_prec(self, u: u64, prec: u64) -> (Self, Ordering) {
        trig_with_period_helper(TrigFunction::Cos, &self, u, prec, Nearest, true)
    }

    /// Computes $\cos(2\pi x/u)$, the cosine of a [`BigExponentFloat`] measured in $u$ths of a
    /// turn, rounding the result to the nearest value of the specified precision. The
    /// [`BigExponentFloat`] is taken by reference. An [`Ordering`] is also returned, indicating
    /// whether the rounded cosine is less than, equal to, or greater than the exact cosine.
    /// Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,u,p) = \cos(2\pi x/u)+\varepsilon.
    /// $$
    /// - If $\cos(2\pi x/u)$ is zero, or the result is `NaN`, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $\cos(2\pi x/u)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\cos(2\pi x/u)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},u,p)=f(\pm\infty,u,p)=\text{NaN}$
    /// - $f(x,0,p)=\text{NaN}$
    /// - $f(0,u,p)=1$
    /// - If $x/u$ is a multiple of $1/2$, the result is exactly $1$ or $-1$; if it is an odd
    ///   multiple of $1/4$, exactly $0$; and if it is $\pm1/6$ or $\pm1/3$ modulo $1$, exactly
    ///   $1/2$ or $-1/2$.
    ///
    /// Unlike [its `Float` counterpart](crate::Float::cos_with_period_prec_round), this function
    /// accepts arguments with any exponent, and its result never overflows or underflows. The
    /// argument is reduced exactly, modulo $u$, which for a large exponent takes time proportional
    /// to the number of bits of the exponent.
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
    /// let (y, o) = (&BigExponentFloat::ONE).cos_with_period_prec_ref(7, 5);
    /// assert_eq!(y.to_string(), "0.62");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = (&BigExponentFloat::ONE).cos_with_period_prec_ref(7, 20);
    /// assert_eq!(y.to_string(), "0.623489");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn cos_with_period_prec_ref(&self, u: u64, prec: u64) -> (Self, Ordering) {
        trig_with_period_helper(TrigFunction::Cos, self, u, prec, Nearest, true)
    }

    /// Replaces a [`BigExponentFloat`] with its cosine measured in $u$ths of a turn, rounding the
    /// result to the nearest value of the specified precision. An [`Ordering`] is returned,
    /// indicating whether the rounded cosine is less than, equal to, or greater than the exact
    /// cosine. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this
    /// function sets the [`BigExponentFloat`] to `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,u,p) = \cos(2\pi x/u)+\varepsilon.
    /// $$
    /// - If $\cos(2\pi x/u)$ is zero, or the result is `NaN`, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $\cos(2\pi x/u)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\cos(2\pi x/u)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},u,p)=f(\pm\infty,u,p)=\text{NaN}$
    /// - $f(x,0,p)=\text{NaN}$
    /// - $f(0,u,p)=1$
    /// - If $x/u$ is a multiple of $1/2$, the result is exactly $1$ or $-1$; if it is an odd
    ///   multiple of $1/4$, exactly $0$; and if it is $\pm1/6$ or $\pm1/3$ modulo $1$, exactly
    ///   $1/2$ or $-1/2$.
    ///
    /// Unlike [its `Float` counterpart](crate::Float::cos_with_period_prec_round), this function
    /// accepts arguments with any exponent, and its result never overflows or underflows. The
    /// argument is reduced exactly, modulo $u$, which for a large exponent takes time proportional
    /// to the number of bits of the exponent.
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
    /// assert_eq!(x.cos_with_period_prec_assign(7, 5), Greater);
    /// assert_eq!(x.to_string(), "0.62");
    ///
    /// let mut x = BigExponentFloat::ONE;
    /// assert_eq!(x.cos_with_period_prec_assign(7, 20), Less);
    /// assert_eq!(x.to_string(), "0.623489");
    /// ```
    #[inline]
    pub fn cos_with_period_prec_assign(&mut self, u: u64, prec: u64) -> Ordering {
        let (result, o) = trig_with_period_helper(TrigFunction::Cos, self, u, prec, Nearest, true);
        *self = result;
        o
    }

    /// Computes $\cos(2\pi x/u)$, the cosine of a [`Rational`] measured in $u$ths of a turn,
    /// returning a [`BigExponentFloat`], rounding the result to the specified precision and with
    /// the specified rounding mode. The [`Rational`] is taken by value. An [`Ordering`] is also
    /// returned, indicating whether the rounded cosine is less than, equal to, or greater than the
    /// exact cosine. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,u,p,m) = \cos(2\pi x/u)+\varepsilon.
    /// $$
    /// - If $\cos(2\pi x/u)$ is zero, or the result is `NaN`, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $\cos(2\pi x/u)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\cos(2\pi x/u)|\rfloor-p+1}$.
    /// - If $\cos(2\pi x/u)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\cos(2\pi x/u)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,0,p,m)=\text{NaN}$
    /// - $f(0,u,p,m)=1$
    /// - If $x/u$ is a multiple of $1/2$, the result is exactly $1$ or $-1$; if it is an odd
    ///   multiple of $1/4$, exactly $0$; and if it is $\pm1/6$ or $\pm1/3$ modulo $1$, exactly
    ///   $1/2$ or $-1/2$.
    ///
    /// Unlike [its `Float` counterpart](crate::Float::cos_with_period_rational_prec_round), this
    /// function never overflows or underflows.
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`BigExponentFloat::cos_with_period_rational_prec`] instead.
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
    /// let (y, o) = BigExponentFloat::cos_with_period_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     7,
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(y.to_string(), "0.938");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::cos_with_period_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     7,
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(y.to_string(), "0.969");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::cos_with_period_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     7,
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(y.to_string(), "0.969");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::cos_with_period_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     7,
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(y.to_string(), "0.9555721");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::cos_with_period_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     7,
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(y.to_string(), "0.95557308");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::cos_with_period_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     7,
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(y.to_string(), "0.95557308");
    /// assert_eq!(o, Greater);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn cos_with_period_rational_prec_round(
        x: Rational,
        u: u64,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        trig_with_period_rational_helper(TrigFunction::Cos, &x, u, prec, rm, true)
    }

    /// Computes $\cos(2\pi x/u)$, the cosine of a [`Rational`] measured in $u$ths of a turn,
    /// returning a [`BigExponentFloat`], rounding the result to the specified precision and with
    /// the specified rounding mode. The [`Rational`] is taken by reference. An [`Ordering`] is also
    /// returned, indicating whether the rounded cosine is less than, equal to, or greater than the
    /// exact cosine. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,u,p,m) = \cos(2\pi x/u)+\varepsilon.
    /// $$
    /// - If $\cos(2\pi x/u)$ is zero, or the result is `NaN`, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $\cos(2\pi x/u)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\cos(2\pi x/u)|\rfloor-p+1}$.
    /// - If $\cos(2\pi x/u)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\cos(2\pi x/u)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,0,p,m)=\text{NaN}$
    /// - $f(0,u,p,m)=1$
    /// - If $x/u$ is a multiple of $1/2$, the result is exactly $1$ or $-1$; if it is an odd
    ///   multiple of $1/4$, exactly $0$; and if it is $\pm1/6$ or $\pm1/3$ modulo $1$, exactly
    ///   $1/2$ or $-1/2$.
    ///
    /// Unlike [its `Float` counterpart](crate::Float::cos_with_period_rational_prec_round), this
    /// function never overflows or underflows.
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
    /// let (y, o) = BigExponentFloat::cos_with_period_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     7,
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(y.to_string(), "0.938");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::cos_with_period_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     7,
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(y.to_string(), "0.969");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::cos_with_period_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     7,
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(y.to_string(), "0.969");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::cos_with_period_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     7,
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(y.to_string(), "0.9555721");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::cos_with_period_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     7,
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(y.to_string(), "0.95557308");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::cos_with_period_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     7,
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(y.to_string(), "0.95557308");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn cos_with_period_rational_prec_round_ref(
        x: &Rational,
        u: u64,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        trig_with_period_rational_helper(TrigFunction::Cos, x, u, prec, rm, true)
    }

    /// Computes $\cos(2\pi x/u)$, the cosine of a [`Rational`] measured in $u$ths of a turn,
    /// returning a [`BigExponentFloat`], rounding the result to the nearest value of the specified
    /// precision. The [`Rational`] is taken by value. An [`Ordering`] is also returned, indicating
    /// whether the rounded cosine is less than, equal to, or greater than the exact cosine.
    /// Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,u,p) = \cos(2\pi x/u)+\varepsilon.
    /// $$
    /// - If $\cos(2\pi x/u)$ is zero, or the result is `NaN`, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $\cos(2\pi x/u)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\cos(2\pi x/u)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,0,p)=\text{NaN}$
    /// - $f(0,u,p)=1$
    /// - If $x/u$ is a multiple of $1/2$, the result is exactly $1$ or $-1$; if it is an odd
    ///   multiple of $1/4$, exactly $0$; and if it is $\pm1/6$ or $\pm1/3$ modulo $1$, exactly
    ///   $1/2$ or $-1/2$.
    ///
    /// Unlike [its `Float` counterpart](crate::Float::cos_with_period_rational_prec_round), this
    /// function never overflows or underflows.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`BigExponentFloat::cos_with_period_rational_prec_round`] instead.
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
    ///     BigExponentFloat::cos_with_period_rational_prec(Rational::from_unsigneds(1u8, 3), 7, 5);
    /// assert_eq!(y.to_string(), "0.969");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::cos_with_period_rational_prec(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     7,
    ///     20,
    /// );
    /// assert_eq!(y.to_string(), "0.95557308");
    /// assert_eq!(o, Greater);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn cos_with_period_rational_prec(x: Rational, u: u64, prec: u64) -> (Self, Ordering) {
        trig_with_period_rational_helper(TrigFunction::Cos, &x, u, prec, Nearest, true)
    }

    /// Computes $\cos(2\pi x/u)$, the cosine of a [`Rational`] measured in $u$ths of a turn,
    /// returning a [`BigExponentFloat`], rounding the result to the nearest value of the specified
    /// precision. The [`Rational`] is taken by reference. An [`Ordering`] is also returned,
    /// indicating whether the rounded cosine is less than, equal to, or greater than the exact
    /// cosine. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,u,p) = \cos(2\pi x/u)+\varepsilon.
    /// $$
    /// - If $\cos(2\pi x/u)$ is zero, or the result is `NaN`, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $\cos(2\pi x/u)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\cos(2\pi x/u)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,0,p)=\text{NaN}$
    /// - $f(0,u,p)=1$
    /// - If $x/u$ is a multiple of $1/2$, the result is exactly $1$ or $-1$; if it is an odd
    ///   multiple of $1/4$, exactly $0$; and if it is $\pm1/6$ or $\pm1/3$ modulo $1$, exactly
    ///   $1/2$ or $-1/2$.
    ///
    /// Unlike [its `Float` counterpart](crate::Float::cos_with_period_rational_prec_round), this
    /// function never overflows or underflows.
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
    /// let (y, o) = BigExponentFloat::cos_with_period_rational_prec_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     7,
    ///     5,
    /// );
    /// assert_eq!(y.to_string(), "0.969");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::cos_with_period_rational_prec_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     7,
    ///     20,
    /// );
    /// assert_eq!(y.to_string(), "0.95557308");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn cos_with_period_rational_prec_ref(x: &Rational, u: u64, prec: u64) -> (Self, Ordering) {
        trig_with_period_rational_helper(TrigFunction::Cos, x, u, prec, Nearest, true)
    }

    /// Computes $\cos(\pi x)$, the cosine of a [`BigExponentFloat`] measured in half-turns,
    /// rounding the result to the specified precision and with the specified rounding mode. The
    /// [`BigExponentFloat`] is taken by value. An [`Ordering`] is also returned, indicating whether
    /// the rounded cosine is less than, equal to, or greater than the exact cosine. Although `NaN`s
    /// are not comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it
    /// also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \cos(\pi x)+\varepsilon.
    /// $$
    /// - If $\cos(\pi x)$ is zero, or the result is `NaN`, $\varepsilon$ may be ignored or assumed
    ///   to be 0.
    /// - If $\cos(\pi x)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\cos(\pi x)|\rfloor-p+1}$.
    /// - If $\cos(\pi x)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\cos(\pi x)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=f(\pm\infty,p,m)=\text{NaN}$
    /// - $f(0,p,m)=1$
    /// - If $x$ is an integer, the result is exactly $1$ or $-1$; if it is an odd multiple of
    ///   $1/2$, exactly $0$; and if it is $\pm1/3$ or $\pm2/3$ modulo $2$, exactly $1/2$ or $-1/2$.
    ///
    /// Unlike [its `Float` counterpart](crate::Float::cos_pi_prec_round), this function accepts
    /// arguments with any exponent, and its result never overflows or underflows. The argument is
    /// reduced exactly, modulo 2, which for a large exponent takes time proportional to the number
    /// of bits of the exponent.
    ///
    /// If you know you'll be using `Nearest`, consider using [`BigExponentFloat::cos_pi_prec`]
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
    /// let (y, o) = BigExponentFloat::from(0.375).cos_pi_prec_round(5, Floor);
    /// assert_eq!(y.to_string(), "0.38");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::from(0.375).cos_pi_prec_round(5, Ceiling);
    /// assert_eq!(y.to_string(), "0.391");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::from(0.375).cos_pi_prec_round(5, Nearest);
    /// assert_eq!(y.to_string(), "0.38");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::from(0.375).cos_pi_prec_round(20, Floor);
    /// assert_eq!(y.to_string(), "0.38268328");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::from(0.375).cos_pi_prec_round(20, Ceiling);
    /// assert_eq!(y.to_string(), "0.3826838");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::from(0.375).cos_pi_prec_round(20, Nearest);
    /// assert_eq!(y.to_string(), "0.38268328");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn cos_pi_prec_round(self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        trig_with_period_helper(TrigFunction::Cos, &self, 2, prec, rm, true)
    }

    /// Computes $\cos(\pi x)$, the cosine of a [`BigExponentFloat`] measured in half-turns,
    /// rounding the result to the specified precision and with the specified rounding mode. The
    /// [`BigExponentFloat`] is taken by reference. An [`Ordering`] is also returned, indicating
    /// whether the rounded cosine is less than, equal to, or greater than the exact cosine.
    /// Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \cos(\pi x)+\varepsilon.
    /// $$
    /// - If $\cos(\pi x)$ is zero, or the result is `NaN`, $\varepsilon$ may be ignored or assumed
    ///   to be 0.
    /// - If $\cos(\pi x)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\cos(\pi x)|\rfloor-p+1}$.
    /// - If $\cos(\pi x)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\cos(\pi x)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=f(\pm\infty,p,m)=\text{NaN}$
    /// - $f(0,p,m)=1$
    /// - If $x$ is an integer, the result is exactly $1$ or $-1$; if it is an odd multiple of
    ///   $1/2$, exactly $0$; and if it is $\pm1/3$ or $\pm2/3$ modulo $2$, exactly $1/2$ or $-1/2$.
    ///
    /// Unlike [its `Float` counterpart](crate::Float::cos_pi_prec_round), this function accepts
    /// arguments with any exponent, and its result never overflows or underflows. The argument is
    /// reduced exactly, modulo 2, which for a large exponent takes time proportional to the number
    /// of bits of the exponent.
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
    /// let (y, o) = (&BigExponentFloat::from(0.375)).cos_pi_prec_round_ref(5, Floor);
    /// assert_eq!(y.to_string(), "0.38");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = (&BigExponentFloat::from(0.375)).cos_pi_prec_round_ref(5, Ceiling);
    /// assert_eq!(y.to_string(), "0.391");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = (&BigExponentFloat::from(0.375)).cos_pi_prec_round_ref(5, Nearest);
    /// assert_eq!(y.to_string(), "0.38");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = (&BigExponentFloat::from(0.375)).cos_pi_prec_round_ref(20, Floor);
    /// assert_eq!(y.to_string(), "0.38268328");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = (&BigExponentFloat::from(0.375)).cos_pi_prec_round_ref(20, Ceiling);
    /// assert_eq!(y.to_string(), "0.3826838");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = (&BigExponentFloat::from(0.375)).cos_pi_prec_round_ref(20, Nearest);
    /// assert_eq!(y.to_string(), "0.38268328");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn cos_pi_prec_round_ref(&self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        trig_with_period_helper(TrigFunction::Cos, self, 2, prec, rm, true)
    }

    /// Replaces a [`BigExponentFloat`] with its cosine measured in half-turns, rounding the result
    /// to the specified precision and with the specified rounding mode. An [`Ordering`] is
    /// returned, indicating whether the rounded cosine is less than, equal to, or greater than the
    /// exact cosine. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this
    /// function sets the [`BigExponentFloat`] to `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \cos(\pi x)+\varepsilon.
    /// $$
    /// - If $\cos(\pi x)$ is zero, or the result is `NaN`, $\varepsilon$ may be ignored or assumed
    ///   to be 0.
    /// - If $\cos(\pi x)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\cos(\pi x)|\rfloor-p+1}$.
    /// - If $\cos(\pi x)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\cos(\pi x)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=f(\pm\infty,p,m)=\text{NaN}$
    /// - $f(0,p,m)=1$
    /// - If $x$ is an integer, the result is exactly $1$ or $-1$; if it is an odd multiple of
    ///   $1/2$, exactly $0$; and if it is $\pm1/3$ or $\pm2/3$ modulo $2$, exactly $1/2$ or $-1/2$.
    ///
    /// Unlike [its `Float` counterpart](crate::Float::cos_pi_prec_round), this function accepts
    /// arguments with any exponent, and its result never overflows or underflows. The argument is
    /// reduced exactly, modulo 2, which for a large exponent takes time proportional to the number
    /// of bits of the exponent.
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
    /// assert_eq!(x.cos_pi_prec_round_assign(5, Floor), Less);
    /// assert_eq!(x.to_string(), "0.38");
    ///
    /// let mut x = BigExponentFloat::from(0.375);
    /// assert_eq!(x.cos_pi_prec_round_assign(5, Ceiling), Greater);
    /// assert_eq!(x.to_string(), "0.391");
    ///
    /// let mut x = BigExponentFloat::from(0.375);
    /// assert_eq!(x.cos_pi_prec_round_assign(5, Nearest), Less);
    /// assert_eq!(x.to_string(), "0.38");
    ///
    /// let mut x = BigExponentFloat::from(0.375);
    /// assert_eq!(x.cos_pi_prec_round_assign(20, Floor), Less);
    /// assert_eq!(x.to_string(), "0.38268328");
    ///
    /// let mut x = BigExponentFloat::from(0.375);
    /// assert_eq!(x.cos_pi_prec_round_assign(20, Ceiling), Greater);
    /// assert_eq!(x.to_string(), "0.3826838");
    ///
    /// let mut x = BigExponentFloat::from(0.375);
    /// assert_eq!(x.cos_pi_prec_round_assign(20, Nearest), Less);
    /// assert_eq!(x.to_string(), "0.38268328");
    /// ```
    #[inline]
    pub fn cos_pi_prec_round_assign(&mut self, prec: u64, rm: RoundingMode) -> Ordering {
        let (result, o) = trig_with_period_helper(TrigFunction::Cos, self, 2, prec, rm, true);
        *self = result;
        o
    }

    /// Computes $\cos(\pi x)$, the cosine of a [`BigExponentFloat`] measured in half-turns,
    /// rounding the result to the nearest value of the specified precision. The
    /// [`BigExponentFloat`] is taken by value. An [`Ordering`] is also returned, indicating whether
    /// the rounded cosine is less than, equal to, or greater than the exact cosine. Although `NaN`s
    /// are not comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it
    /// also returns `Equal`.
    ///
    /// $$
    /// f(x,p) = \cos(\pi x)+\varepsilon.
    /// $$
    /// - If $\cos(\pi x)$ is zero, or the result is `NaN`, $\varepsilon$ may be ignored or assumed
    ///   to be 0.
    /// - If $\cos(\pi x)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\cos(\pi x)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p)=f(\pm\infty,p)=\text{NaN}$
    /// - $f(0,p)=1$
    /// - If $x$ is an integer, the result is exactly $1$ or $-1$; if it is an odd multiple of
    ///   $1/2$, exactly $0$; and if it is $\pm1/3$ or $\pm2/3$ modulo $2$, exactly $1/2$ or $-1/2$.
    ///
    /// Unlike [its `Float` counterpart](crate::Float::cos_pi_prec_round), this function accepts
    /// arguments with any exponent, and its result never overflows or underflows. The argument is
    /// reduced exactly, modulo 2, which for a large exponent takes time proportional to the number
    /// of bits of the exponent.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`BigExponentFloat::cos_pi_prec_round`] instead.
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
    /// let (y, o) = BigExponentFloat::from(0.375).cos_pi_prec(5);
    /// assert_eq!(y.to_string(), "0.38");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::from(0.375).cos_pi_prec(20);
    /// assert_eq!(y.to_string(), "0.38268328");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn cos_pi_prec(self, prec: u64) -> (Self, Ordering) {
        trig_with_period_helper(TrigFunction::Cos, &self, 2, prec, Nearest, true)
    }

    /// Computes $\cos(\pi x)$, the cosine of a [`BigExponentFloat`] measured in half-turns,
    /// rounding the result to the nearest value of the specified precision. The
    /// [`BigExponentFloat`] is taken by reference. An [`Ordering`] is also returned, indicating
    /// whether the rounded cosine is less than, equal to, or greater than the exact cosine.
    /// Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,p) = \cos(\pi x)+\varepsilon.
    /// $$
    /// - If $\cos(\pi x)$ is zero, or the result is `NaN`, $\varepsilon$ may be ignored or assumed
    ///   to be 0.
    /// - If $\cos(\pi x)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\cos(\pi x)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p)=f(\pm\infty,p)=\text{NaN}$
    /// - $f(0,p)=1$
    /// - If $x$ is an integer, the result is exactly $1$ or $-1$; if it is an odd multiple of
    ///   $1/2$, exactly $0$; and if it is $\pm1/3$ or $\pm2/3$ modulo $2$, exactly $1/2$ or $-1/2$.
    ///
    /// Unlike [its `Float` counterpart](crate::Float::cos_pi_prec_round), this function accepts
    /// arguments with any exponent, and its result never overflows or underflows. The argument is
    /// reduced exactly, modulo 2, which for a large exponent takes time proportional to the number
    /// of bits of the exponent.
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
    /// let (y, o) = (&BigExponentFloat::from(0.375)).cos_pi_prec_ref(5);
    /// assert_eq!(y.to_string(), "0.38");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = (&BigExponentFloat::from(0.375)).cos_pi_prec_ref(20);
    /// assert_eq!(y.to_string(), "0.38268328");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn cos_pi_prec_ref(&self, prec: u64) -> (Self, Ordering) {
        trig_with_period_helper(TrigFunction::Cos, self, 2, prec, Nearest, true)
    }

    /// Replaces a [`BigExponentFloat`] with its cosine measured in half-turns, rounding the result
    /// to the nearest value of the specified precision. An [`Ordering`] is returned, indicating
    /// whether the rounded cosine is less than, equal to, or greater than the exact cosine.
    /// Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function sets
    /// the [`BigExponentFloat`] to `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,p) = \cos(\pi x)+\varepsilon.
    /// $$
    /// - If $\cos(\pi x)$ is zero, or the result is `NaN`, $\varepsilon$ may be ignored or assumed
    ///   to be 0.
    /// - If $\cos(\pi x)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\cos(\pi x)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p)=f(\pm\infty,p)=\text{NaN}$
    /// - $f(0,p)=1$
    /// - If $x$ is an integer, the result is exactly $1$ or $-1$; if it is an odd multiple of
    ///   $1/2$, exactly $0$; and if it is $\pm1/3$ or $\pm2/3$ modulo $2$, exactly $1/2$ or $-1/2$.
    ///
    /// Unlike [its `Float` counterpart](crate::Float::cos_pi_prec_round), this function accepts
    /// arguments with any exponent, and its result never overflows or underflows. The argument is
    /// reduced exactly, modulo 2, which for a large exponent takes time proportional to the number
    /// of bits of the exponent.
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
    /// assert_eq!(x.cos_pi_prec_assign(5), Less);
    /// assert_eq!(x.to_string(), "0.38");
    ///
    /// let mut x = BigExponentFloat::from(0.375);
    /// assert_eq!(x.cos_pi_prec_assign(20), Less);
    /// assert_eq!(x.to_string(), "0.38268328");
    /// ```
    #[inline]
    pub fn cos_pi_prec_assign(&mut self, prec: u64) -> Ordering {
        let (result, o) = trig_with_period_helper(TrigFunction::Cos, self, 2, prec, Nearest, true);
        *self = result;
        o
    }

    /// Computes $\cos(\pi x)$, the cosine of a [`Rational`] measured in half-turns, returning a
    /// [`BigExponentFloat`], rounding the result to the specified precision and with the specified
    /// rounding mode. The [`Rational`] is taken by value. An [`Ordering`] is also returned,
    /// indicating whether the rounded cosine is less than, equal to, or greater than the exact
    /// cosine. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \cos(\pi x)+\varepsilon.
    /// $$
    /// - If $\cos(\pi x)$ is zero, or the result is `NaN`, $\varepsilon$ may be ignored or assumed
    ///   to be 0.
    /// - If $\cos(\pi x)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\cos(\pi x)|\rfloor-p+1}$.
    /// - If $\cos(\pi x)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\cos(\pi x)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(0,p,m)=1$
    /// - If $x$ is an integer, the result is exactly $1$ or $-1$; if it is an odd multiple of
    ///   $1/2$, exactly $0$; and if it is $\pm1/3$ or $\pm2/3$ modulo $2$, exactly $1/2$ or $-1/2$.
    ///
    /// Unlike [its `Float` counterpart](crate::Float::cos_pi_rational_prec_round), this function
    /// never overflows or underflows.
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`BigExponentFloat::cos_pi_rational_prec`] instead.
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
    /// let (y, o) = BigExponentFloat::cos_pi_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 5),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(y.to_string(), "0.781");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::cos_pi_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 5),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(y.to_string(), "0.812");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::cos_pi_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 5),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(y.to_string(), "0.812");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::cos_pi_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 5),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(y.to_string(), "0.80901623");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::cos_pi_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 5),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(y.to_string(), "0.8090172");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::cos_pi_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 5),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(y.to_string(), "0.8090172");
    /// assert_eq!(o, Greater);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn cos_pi_rational_prec_round(
        x: Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        trig_with_period_rational_helper(TrigFunction::Cos, &x, 2, prec, rm, true)
    }

    /// Computes $\cos(\pi x)$, the cosine of a [`Rational`] measured in half-turns, returning a
    /// [`BigExponentFloat`], rounding the result to the specified precision and with the specified
    /// rounding mode. The [`Rational`] is taken by reference. An [`Ordering`] is also returned,
    /// indicating whether the rounded cosine is less than, equal to, or greater than the exact
    /// cosine. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \cos(\pi x)+\varepsilon.
    /// $$
    /// - If $\cos(\pi x)$ is zero, or the result is `NaN`, $\varepsilon$ may be ignored or assumed
    ///   to be 0.
    /// - If $\cos(\pi x)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\cos(\pi x)|\rfloor-p+1}$.
    /// - If $\cos(\pi x)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\cos(\pi x)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(0,p,m)=1$
    /// - If $x$ is an integer, the result is exactly $1$ or $-1$; if it is an odd multiple of
    ///   $1/2$, exactly $0$; and if it is $\pm1/3$ or $\pm2/3$ modulo $2$, exactly $1/2$ or $-1/2$.
    ///
    /// Unlike [its `Float` counterpart](crate::Float::cos_pi_rational_prec_round), this function
    /// never overflows or underflows.
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
    /// let (y, o) = BigExponentFloat::cos_pi_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(1u8, 5),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(y.to_string(), "0.781");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::cos_pi_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(1u8, 5),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(y.to_string(), "0.812");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::cos_pi_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(1u8, 5),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(y.to_string(), "0.812");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::cos_pi_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(1u8, 5),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(y.to_string(), "0.80901623");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::cos_pi_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(1u8, 5),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(y.to_string(), "0.8090172");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::cos_pi_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(1u8, 5),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(y.to_string(), "0.8090172");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn cos_pi_rational_prec_round_ref(
        x: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        trig_with_period_rational_helper(TrigFunction::Cos, x, 2, prec, rm, true)
    }

    /// Computes $\cos(\pi x)$, the cosine of a [`Rational`] measured in half-turns, returning a
    /// [`BigExponentFloat`], rounding the result to the nearest value of the specified precision.
    /// The [`Rational`] is taken by value. An [`Ordering`] is also returned, indicating whether the
    /// rounded cosine is less than, equal to, or greater than the exact cosine. Although `NaN`s are
    /// not comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// $$
    /// f(x,p) = \cos(\pi x)+\varepsilon.
    /// $$
    /// - If $\cos(\pi x)$ is zero, or the result is `NaN`, $\varepsilon$ may be ignored or assumed
    ///   to be 0.
    /// - If $\cos(\pi x)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\cos(\pi x)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(0,p)=1$
    /// - If $x$ is an integer, the result is exactly $1$ or $-1$; if it is an odd multiple of
    ///   $1/2$, exactly $0$; and if it is $\pm1/3$ or $\pm2/3$ modulo $2$, exactly $1/2$ or $-1/2$.
    ///
    /// Unlike [its `Float` counterpart](crate::Float::cos_pi_rational_prec_round), this function
    /// never overflows or underflows.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`BigExponentFloat::cos_pi_rational_prec_round`] instead.
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
    /// let (y, o) = BigExponentFloat::cos_pi_rational_prec(Rational::from_unsigneds(1u8, 5), 5);
    /// assert_eq!(y.to_string(), "0.812");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::cos_pi_rational_prec(Rational::from_unsigneds(1u8, 5), 20);
    /// assert_eq!(y.to_string(), "0.8090172");
    /// assert_eq!(o, Greater);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn cos_pi_rational_prec(x: Rational, prec: u64) -> (Self, Ordering) {
        trig_with_period_rational_helper(TrigFunction::Cos, &x, 2, prec, Nearest, true)
    }

    /// Computes $\cos(\pi x)$, the cosine of a [`Rational`] measured in half-turns, returning a
    /// [`BigExponentFloat`], rounding the result to the nearest value of the specified precision.
    /// The [`Rational`] is taken by reference. An [`Ordering`] is also returned, indicating whether
    /// the rounded cosine is less than, equal to, or greater than the exact cosine. Although `NaN`s
    /// are not comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it
    /// also returns `Equal`.
    ///
    /// $$
    /// f(x,p) = \cos(\pi x)+\varepsilon.
    /// $$
    /// - If $\cos(\pi x)$ is zero, or the result is `NaN`, $\varepsilon$ may be ignored or assumed
    ///   to be 0.
    /// - If $\cos(\pi x)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\cos(\pi x)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(0,p)=1$
    /// - If $x$ is an integer, the result is exactly $1$ or $-1$; if it is an odd multiple of
    ///   $1/2$, exactly $0$; and if it is $\pm1/3$ or $\pm2/3$ modulo $2$, exactly $1/2$ or $-1/2$.
    ///
    /// Unlike [its `Float` counterpart](crate::Float::cos_pi_rational_prec_round), this function
    /// never overflows or underflows.
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
    ///     BigExponentFloat::cos_pi_rational_prec_ref(&Rational::from_unsigneds(1u8, 5), 5);
    /// assert_eq!(y.to_string(), "0.812");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) =
    ///     BigExponentFloat::cos_pi_rational_prec_ref(&Rational::from_unsigneds(1u8, 5), 20);
    /// assert_eq!(y.to_string(), "0.8090172");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn cos_pi_rational_prec_ref(x: &Rational, prec: u64) -> (Self, Ordering) {
        trig_with_period_rational_helper(TrigFunction::Cos, x, 2, prec, Nearest, true)
    }
}
