// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::big_exponent_float::BigExponentFloat;
use crate::big_exponent_float::exponential::log_base::{GeneralBase, log_general_1_plus_x_helper};
use core::cmp::Ordering;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_q::Rational;

impl BigExponentFloat {
    /// Computes $\log_b(1+x)$, where $x$ is a [`BigExponentFloat`] and $b$ is a [`Rational`],
    /// rounding the result to the specified precision and with the specified rounding mode. The
    /// [`BigExponentFloat`] is taken by value and the base by reference. An [`Ordering`] is also
    /// returned, indicating whether the rounded value is less than, equal to, or greater than the
    /// exact value. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,b,p,m) = \log_b(1+x)+\varepsilon.
    /// $$
    /// - If $\log_b(1+x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $\log_b(1+x)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\log_b(1+x)|\rfloor-p+1}$.
    /// - If $\log_b(1+x)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\log_b(1+x)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},b,p,m)=\text{NaN}$
    /// - $f(\infty,b,p,m)=\infty$
    /// - $f(-\infty,b,p,m)=\text{NaN}$
    /// - $f(0,b,p,m)=0$
    /// - $f(-1,b,p,m)=-\infty$
    /// - $f(x,b,p,m)=\text{NaN}$ if $x<-1$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::log_base_rational_base_1_plus_x_prec_round),
    /// this function accepts arguments with any exponent, and its result never overflows or
    /// underflows.
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`BigExponentFloat::log_base_rational_base_1_plus_x_prec`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^2 \log\log n + m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `max(self.significant_bits(), base.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, if `base` is less than or equal to 1, or if `rm` is `Exact` but
    /// the result cannot be represented exactly with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (y, o) = BigExponentFloat::from(PI).log_base_rational_base_1_plus_x_prec_round(
    ///     &Rational::from_unsigneds(3u8, 2),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{y:?}"), "0x3.8");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::from(PI).log_base_rational_base_1_plus_x_prec_round(
    ///     &Rational::from_unsigneds(3u8, 2),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{y:?}"), "0x3.a");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::from(PI).log_base_rational_base_1_plus_x_prec_round(
    ///     &Rational::from_unsigneds(3u8, 2),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{y:?}"), "0x3.8");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::from(PI).log_base_rational_base_1_plus_x_prec_round(
    ///     &Rational::from_unsigneds(3u8, 2),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{y:?}"), "0x3.813b8");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::from(PI).log_base_rational_base_1_plus_x_prec_round(
    ///     &Rational::from_unsigneds(3u8, 2),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{y:?}"), "0x3.813bc");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::from(PI).log_base_rational_base_1_plus_x_prec_round(
    ///     &Rational::from_unsigneds(3u8, 2),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{y:?}"), "0x3.813b8");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn log_base_rational_base_1_plus_x_prec_round(
        self,
        base: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        log_general_1_plus_x_helper(&self, GeneralBase::Rational(base), prec, rm)
    }

    /// Computes $\log_b(1+x)$, where $x$ is a [`BigExponentFloat`] and $b$ is a [`Rational`],
    /// rounding the result to the specified precision and with the specified rounding mode. The
    /// [`BigExponentFloat`] is taken by reference and the base by reference. An [`Ordering`] is
    /// also returned, indicating whether the rounded value is less than, equal to, or greater than
    /// the exact value. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever
    /// this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,b,p,m) = \log_b(1+x)+\varepsilon.
    /// $$
    /// - If $\log_b(1+x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $\log_b(1+x)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\log_b(1+x)|\rfloor-p+1}$.
    /// - If $\log_b(1+x)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\log_b(1+x)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},b,p,m)=\text{NaN}$
    /// - $f(\infty,b,p,m)=\infty$
    /// - $f(-\infty,b,p,m)=\text{NaN}$
    /// - $f(0,b,p,m)=0$
    /// - $f(-1,b,p,m)=-\infty$
    /// - $f(x,b,p,m)=\text{NaN}$ if $x<-1$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::log_base_rational_base_1_plus_x_prec_round),
    /// this function accepts arguments with any exponent, and its result never overflows or
    /// underflows.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^2 \log\log n + m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `max(self.significant_bits(), base.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, if `base` is less than or equal to 1, or if `rm` is `Exact` but
    /// the result cannot be represented exactly with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (y, o) = (&BigExponentFloat::from(PI)).log_base_rational_base_1_plus_x_prec_round_ref(
    ///     &Rational::from_unsigneds(3u8, 2),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{y:?}"), "0x3.8");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = (&BigExponentFloat::from(PI)).log_base_rational_base_1_plus_x_prec_round_ref(
    ///     &Rational::from_unsigneds(3u8, 2),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{y:?}"), "0x3.a");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = (&BigExponentFloat::from(PI)).log_base_rational_base_1_plus_x_prec_round_ref(
    ///     &Rational::from_unsigneds(3u8, 2),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{y:?}"), "0x3.8");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = (&BigExponentFloat::from(PI)).log_base_rational_base_1_plus_x_prec_round_ref(
    ///     &Rational::from_unsigneds(3u8, 2),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{y:?}"), "0x3.813b8");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = (&BigExponentFloat::from(PI)).log_base_rational_base_1_plus_x_prec_round_ref(
    ///     &Rational::from_unsigneds(3u8, 2),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{y:?}"), "0x3.813bc");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = (&BigExponentFloat::from(PI)).log_base_rational_base_1_plus_x_prec_round_ref(
    ///     &Rational::from_unsigneds(3u8, 2),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{y:?}"), "0x3.813b8");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn log_base_rational_base_1_plus_x_prec_round_ref(
        &self,
        base: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        log_general_1_plus_x_helper(self, GeneralBase::Rational(base), prec, rm)
    }

    /// Computes $\log_b(1+x)$, where $x$ is a [`BigExponentFloat`] and $b$ is a [`Rational`], in
    /// place, rounding the result to the specified precision and with the specified rounding mode.
    /// The base is taken by reference. An [`Ordering`] is returned, indicating whether the rounded
    /// value is less than, equal to, or greater than the exact value. Although `NaN`s are not
    /// comparable to any [`BigExponentFloat`], whenever this function sets the [`BigExponentFloat`]
    /// to `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets \log_b(1+x)+\varepsilon.
    /// $$
    /// - If $\log_b(1+x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $\log_b(1+x)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\log_b(1+x)|\rfloor-p+1}$.
    /// - If $\log_b(1+x)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\log_b(1+x)|\rfloor-p}$.
    ///
    /// Unlike [its `Float` counterpart](crate::Float::log_base_rational_base_1_plus_x_prec_round),
    /// this function accepts arguments with any exponent, and its result never overflows or
    /// underflows.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^2 \log\log n + m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `max(self.significant_bits(), base.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, if `base` is less than or equal to 1, or if `rm` is `Exact` but
    /// the result cannot be represented exactly with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.log_base_rational_base_1_plus_x_prec_round_assign(
    ///         &Rational::from_unsigneds(3u8, 2),
    ///         5,
    ///         Floor
    ///     ),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x3.8");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.log_base_rational_base_1_plus_x_prec_round_assign(
    ///         &Rational::from_unsigneds(3u8, 2),
    ///         5,
    ///         Ceiling
    ///     ),
    ///     Greater
    /// );
    /// assert_eq!(format!("{x:?}"), "0x3.a");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.log_base_rational_base_1_plus_x_prec_round_assign(
    ///         &Rational::from_unsigneds(3u8, 2),
    ///         5,
    ///         Nearest
    ///     ),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x3.8");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.log_base_rational_base_1_plus_x_prec_round_assign(
    ///         &Rational::from_unsigneds(3u8, 2),
    ///         20,
    ///         Floor
    ///     ),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x3.813b8");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.log_base_rational_base_1_plus_x_prec_round_assign(
    ///         &Rational::from_unsigneds(3u8, 2),
    ///         20,
    ///         Ceiling
    ///     ),
    ///     Greater
    /// );
    /// assert_eq!(format!("{x:?}"), "0x3.813bc");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.log_base_rational_base_1_plus_x_prec_round_assign(
    ///         &Rational::from_unsigneds(3u8, 2),
    ///         20,
    ///         Nearest
    ///     ),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x3.813b8");
    /// ```
    #[inline]
    pub fn log_base_rational_base_1_plus_x_prec_round_assign(
        &mut self,
        base: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> Ordering {
        let (result, o) = log_general_1_plus_x_helper(self, GeneralBase::Rational(base), prec, rm);
        *self = result;
        o
    }

    /// Computes $\log_b(1+x)$, where $x$ is a [`BigExponentFloat`] and $b$ is a [`Rational`],
    /// rounding the result to the nearest value of the specified precision. The
    /// [`BigExponentFloat`] is taken by value and the base by reference. An [`Ordering`] is also
    /// returned, indicating whether the rounded value is less than, equal to, or greater than the
    /// exact value. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,b,p) = \log_b(1+x)+\varepsilon.
    /// $$
    /// - If $\log_b(1+x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $\log_b(1+x)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\log_b(1+x)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},b,p)=\text{NaN}$
    /// - $f(\infty,b,p)=\infty$
    /// - $f(-\infty,b,p)=\text{NaN}$
    /// - $f(0,b,p)=0$
    /// - $f(-1,b,p)=-\infty$
    /// - $f(x,b,p)=\text{NaN}$ if $x<-1$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::log_base_rational_base_1_plus_x_prec_round),
    /// this function accepts arguments with any exponent, and its result never overflows or
    /// underflows.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`BigExponentFloat::log_base_rational_base_1_plus_x_prec_round`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^2 \log\log n + m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `max(self.significant_bits(), base.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero or if `base` is less than or equal to 1.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::num::arithmetic::traits::PowerOf2;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (y, o) = BigExponentFloat::from(PI)
    ///     .log_base_rational_base_1_plus_x_prec(&Rational::from_unsigneds(3u8, 2), 5);
    /// assert_eq!(format!("{y:?}"), "0x3.8");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::from(PI)
    ///     .log_base_rational_base_1_plus_x_prec(&Rational::from_unsigneds(3u8, 2), 20);
    /// assert_eq!(format!("{y:?}"), "0x3.813b8");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::power_of_2(1000000000000i64)
    ///     .log_base_rational_base_1_plus_x_prec(&Rational::from_unsigneds(3u8, 2), 10);
    /// assert_eq!(format!("{y:?}"), "0x1.8eE+10");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn log_base_rational_base_1_plus_x_prec(
        self,
        base: &Rational,
        prec: u64,
    ) -> (Self, Ordering) {
        log_general_1_plus_x_helper(&self, GeneralBase::Rational(base), prec, Nearest)
    }

    /// Computes $\log_b(1+x)$, where $x$ is a [`BigExponentFloat`] and $b$ is a [`Rational`],
    /// rounding the result to the nearest value of the specified precision. The
    /// [`BigExponentFloat`] is taken by reference and the base by reference. An [`Ordering`] is
    /// also returned, indicating whether the rounded value is less than, equal to, or greater than
    /// the exact value. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever
    /// this function returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,b,p) = \log_b(1+x)+\varepsilon.
    /// $$
    /// - If $\log_b(1+x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $\log_b(1+x)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\log_b(1+x)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},b,p)=\text{NaN}$
    /// - $f(\infty,b,p)=\infty$
    /// - $f(-\infty,b,p)=\text{NaN}$
    /// - $f(0,b,p)=0$
    /// - $f(-1,b,p)=-\infty$
    /// - $f(x,b,p)=\text{NaN}$ if $x<-1$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::log_base_rational_base_1_plus_x_prec_round),
    /// this function accepts arguments with any exponent, and its result never overflows or
    /// underflows.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^2 \log\log n + m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `max(self.significant_bits(), base.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero or if `base` is less than or equal to 1.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (y, o) = (&BigExponentFloat::from(PI))
    ///     .log_base_rational_base_1_plus_x_prec_ref(&Rational::from_unsigneds(3u8, 2), 5);
    /// assert_eq!(format!("{y:?}"), "0x3.8");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = (&BigExponentFloat::from(PI))
    ///     .log_base_rational_base_1_plus_x_prec_ref(&Rational::from_unsigneds(3u8, 2), 20);
    /// assert_eq!(format!("{y:?}"), "0x3.813b8");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn log_base_rational_base_1_plus_x_prec_ref(
        &self,
        base: &Rational,
        prec: u64,
    ) -> (Self, Ordering) {
        log_general_1_plus_x_helper(self, GeneralBase::Rational(base), prec, Nearest)
    }

    /// Computes $\log_b(1+x)$, where $x$ is a [`BigExponentFloat`] and $b$ is a [`Rational`], in
    /// place, rounding the result to the nearest value of the specified precision. The base is
    /// taken by reference. An [`Ordering`] is returned, indicating whether the rounded value is
    /// less than, equal to, or greater than the exact value. Although `NaN`s are not comparable to
    /// any [`BigExponentFloat`], whenever this function sets the [`BigExponentFloat`] to `NaN` it
    /// also returns `Equal`.
    ///
    /// $$
    /// x \gets \log_b(1+x)+\varepsilon.
    /// $$
    /// - If $\log_b(1+x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $\log_b(1+x)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\log_b(1+x)|\rfloor-p}$.
    ///
    /// Unlike [its `Float` counterpart](crate::Float::log_base_rational_base_1_plus_x_prec_round),
    /// this function accepts arguments with any exponent, and its result never overflows or
    /// underflows.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^2 \log\log n + m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `max(self.significant_bits(), base.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero or if `base` is less than or equal to 1.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.log_base_rational_base_1_plus_x_prec_assign(&Rational::from_unsigneds(3u8, 2), 5),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x3.8");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.log_base_rational_base_1_plus_x_prec_assign(&Rational::from_unsigneds(3u8, 2), 20),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x3.813b8");
    /// ```
    #[inline]
    pub fn log_base_rational_base_1_plus_x_prec_assign(
        &mut self,
        base: &Rational,
        prec: u64,
    ) -> Ordering {
        let (result, o) =
            log_general_1_plus_x_helper(self, GeneralBase::Rational(base), prec, Nearest);
        *self = result;
        o
    }
}
