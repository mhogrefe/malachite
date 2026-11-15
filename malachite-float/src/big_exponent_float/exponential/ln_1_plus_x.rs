// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::big_exponent_float::BigExponentFloat;
use crate::big_exponent_float::exponential::ln::{LogBase, log_1_plus_x_helper};
use core::cmp::Ordering;
use malachite_base::rounding_modes::RoundingMode::{self, *};

impl BigExponentFloat {
    /// Computes $\ln(1+x)$, where $x$ is a [`BigExponentFloat`], rounding the result to the
    /// specified precision and with the specified rounding mode. The [`BigExponentFloat`] is taken
    /// by value. An [`Ordering`] is also returned, indicating whether the rounded value is less
    /// than, equal to, or greater than the exact value. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \ln(1+x)+\varepsilon.
    /// $$
    /// - If $\ln(1+x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\ln(1+x)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\ln(1+x)|\rfloor-p+1}$.
    /// - If $\ln(1+x)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\ln(1+x)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=\text{NaN}$
    /// - $f(\infty,p,m)=\infty$
    /// - $f(-\infty,p,m)=\text{NaN}$
    /// - $f(0,p,m)=0$
    /// - $f(-1,p,m)=-\infty$
    /// - $f(x,p,m)=\text{NaN}$ if $x<-1$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::ln_1_plus_x_prec_round), this function
    /// accepts arguments with any exponent, and its result never underflows.
    ///
    /// If you know you'll be using `Nearest`, consider using [`BigExponentFloat::ln_1_plus_x_prec`]
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
    /// Panics if `prec` is zero or if `rm` is `Exact` but the result cannot be represented exactly
    /// with `prec` bits (which is the case for every finite nonzero input greater than -1).
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (y, o) = BigExponentFloat::from(PI).ln_1_plus_x_prec_round(5, Floor);
    /// assert_eq!(y.to_string(), "1.38");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::from(PI).ln_1_plus_x_prec_round(5, Ceiling);
    /// assert_eq!(y.to_string(), "1.44");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::from(PI).ln_1_plus_x_prec_round(5, Nearest);
    /// assert_eq!(y.to_string(), "1.44");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::from(PI).ln_1_plus_x_prec_round(20, Floor);
    /// assert_eq!(y.to_string(), "1.4210796");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::from(PI).ln_1_plus_x_prec_round(20, Ceiling);
    /// assert_eq!(y.to_string(), "1.42108");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::from(PI).ln_1_plus_x_prec_round(20, Nearest);
    /// assert_eq!(y.to_string(), "1.4210796");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn ln_1_plus_x_prec_round(self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        log_1_plus_x_helper(&self, LogBase::E, prec, rm)
    }

    /// Computes $\ln(1+x)$, where $x$ is a [`BigExponentFloat`], rounding the result to the
    /// specified precision and with the specified rounding mode. The [`BigExponentFloat`] is taken
    /// by reference. An [`Ordering`] is also returned, indicating whether the rounded value is less
    /// than, equal to, or greater than the exact value. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \ln(1+x)+\varepsilon.
    /// $$
    /// - If $\ln(1+x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\ln(1+x)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\ln(1+x)|\rfloor-p+1}$.
    /// - If $\ln(1+x)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\ln(1+x)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=\text{NaN}$
    /// - $f(\infty,p,m)=\infty$
    /// - $f(-\infty,p,m)=\text{NaN}$
    /// - $f(0,p,m)=0$
    /// - $f(-1,p,m)=-\infty$
    /// - $f(x,p,m)=\text{NaN}$ if $x<-1$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::ln_1_plus_x_prec_round), this function
    /// accepts arguments with any exponent, and its result never underflows.
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
    /// Panics if `prec` is zero or if `rm` is `Exact` but the result cannot be represented exactly
    /// with `prec` bits (which is the case for every finite nonzero input greater than -1).
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (y, o) = (&BigExponentFloat::from(PI)).ln_1_plus_x_prec_round_ref(5, Floor);
    /// assert_eq!(y.to_string(), "1.38");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = (&BigExponentFloat::from(PI)).ln_1_plus_x_prec_round_ref(5, Ceiling);
    /// assert_eq!(y.to_string(), "1.44");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = (&BigExponentFloat::from(PI)).ln_1_plus_x_prec_round_ref(5, Nearest);
    /// assert_eq!(y.to_string(), "1.44");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = (&BigExponentFloat::from(PI)).ln_1_plus_x_prec_round_ref(20, Floor);
    /// assert_eq!(y.to_string(), "1.4210796");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = (&BigExponentFloat::from(PI)).ln_1_plus_x_prec_round_ref(20, Ceiling);
    /// assert_eq!(y.to_string(), "1.42108");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = (&BigExponentFloat::from(PI)).ln_1_plus_x_prec_round_ref(20, Nearest);
    /// assert_eq!(y.to_string(), "1.4210796");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn ln_1_plus_x_prec_round_ref(&self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        log_1_plus_x_helper(self, LogBase::E, prec, rm)
    }

    /// Computes $\ln(1+x)$, where $x$ is a [`BigExponentFloat`], in place, rounding the result to
    /// the specified precision and with the specified rounding mode. An [`Ordering`] is returned,
    /// indicating whether the rounded value is less than, equal to, or greater than the exact
    /// value. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this
    /// function sets the [`BigExponentFloat`] to `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets \ln(1+x)+\varepsilon.
    /// $$
    /// - If $\ln(1+x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\ln(1+x)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\ln(1+x)|\rfloor-p+1}$.
    /// - If $\ln(1+x)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\ln(1+x)|\rfloor-p}$.
    ///
    /// Unlike [its `Float` counterpart](crate::Float::ln_1_plus_x_prec_round), this function
    /// accepts arguments with any exponent, and its result never underflows.
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
    /// Panics if `prec` is zero or if `rm` is `Exact` but the result cannot be represented exactly
    /// with `prec` bits (which is the case for every finite nonzero input greater than -1).
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(x.ln_1_plus_x_prec_round_assign(5, Floor), Less);
    /// assert_eq!(x.to_string(), "1.38");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(x.ln_1_plus_x_prec_round_assign(5, Ceiling), Greater);
    /// assert_eq!(x.to_string(), "1.44");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(x.ln_1_plus_x_prec_round_assign(5, Nearest), Greater);
    /// assert_eq!(x.to_string(), "1.44");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(x.ln_1_plus_x_prec_round_assign(20, Floor), Less);
    /// assert_eq!(x.to_string(), "1.4210796");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(x.ln_1_plus_x_prec_round_assign(20, Ceiling), Greater);
    /// assert_eq!(x.to_string(), "1.42108");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(x.ln_1_plus_x_prec_round_assign(20, Nearest), Less);
    /// assert_eq!(x.to_string(), "1.4210796");
    /// ```
    #[inline]
    pub fn ln_1_plus_x_prec_round_assign(&mut self, prec: u64, rm: RoundingMode) -> Ordering {
        let (result, o) = log_1_plus_x_helper(self, LogBase::E, prec, rm);
        *self = result;
        o
    }

    /// Computes $\ln(1+x)$, where $x$ is a [`BigExponentFloat`], rounding the result to the nearest
    /// value of the specified precision. The [`BigExponentFloat`] is taken by value. An
    /// [`Ordering`] is also returned, indicating whether the rounded value is less than, equal to,
    /// or greater than the exact value. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,p) = \ln(1+x)+\varepsilon.
    /// $$
    /// - If $\ln(1+x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\ln(1+x)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\ln(1+x)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p)=\text{NaN}$
    /// - $f(\infty,p)=\infty$
    /// - $f(-\infty,p)=\text{NaN}$
    /// - $f(0,p)=0$
    /// - $f(-1,p)=-\infty$
    /// - $f(x,p)=\text{NaN}$ if $x<-1$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::ln_1_plus_x_prec_round), this function
    /// accepts arguments with any exponent, and its result never underflows.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`BigExponentFloat::ln_1_plus_x_prec_round`] instead.
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
    /// use core::f64::consts::PI;
    /// use malachite_base::num::arithmetic::traits::PowerOf2;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (y, o) = BigExponentFloat::from(PI).ln_1_plus_x_prec(5);
    /// assert_eq!(y.to_string(), "1.44");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::from(PI).ln_1_plus_x_prec(20);
    /// assert_eq!(y.to_string(), "1.4210796");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::power_of_2(1000000000000i64).ln_1_plus_x_prec(10);
    /// assert_eq!(y.to_string(), "6.936e11");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn ln_1_plus_x_prec(self, prec: u64) -> (Self, Ordering) {
        log_1_plus_x_helper(&self, LogBase::E, prec, Nearest)
    }

    /// Computes $\ln(1+x)$, where $x$ is a [`BigExponentFloat`], rounding the result to the nearest
    /// value of the specified precision. The [`BigExponentFloat`] is taken by reference. An
    /// [`Ordering`] is also returned, indicating whether the rounded value is less than, equal to,
    /// or greater than the exact value. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,p) = \ln(1+x)+\varepsilon.
    /// $$
    /// - If $\ln(1+x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\ln(1+x)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\ln(1+x)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p)=\text{NaN}$
    /// - $f(\infty,p)=\infty$
    /// - $f(-\infty,p)=\text{NaN}$
    /// - $f(0,p)=0$
    /// - $f(-1,p)=-\infty$
    /// - $f(x,p)=\text{NaN}$ if $x<-1$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::ln_1_plus_x_prec_round), this function
    /// accepts arguments with any exponent, and its result never underflows.
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
    /// use core::f64::consts::PI;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (y, o) = (&BigExponentFloat::from(PI)).ln_1_plus_x_prec_ref(5);
    /// assert_eq!(y.to_string(), "1.44");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = (&BigExponentFloat::from(PI)).ln_1_plus_x_prec_ref(20);
    /// assert_eq!(y.to_string(), "1.4210796");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn ln_1_plus_x_prec_ref(&self, prec: u64) -> (Self, Ordering) {
        log_1_plus_x_helper(self, LogBase::E, prec, Nearest)
    }

    /// Computes $\ln(1+x)$, where $x$ is a [`BigExponentFloat`], in place, rounding the result to
    /// the nearest value of the specified precision. An [`Ordering`] is returned, indicating
    /// whether the rounded value is less than, equal to, or greater than the exact value. Although
    /// `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function sets the
    /// [`BigExponentFloat`] to `NaN` it also returns `Equal`.
    ///
    /// $$
    /// x \gets \ln(1+x)+\varepsilon.
    /// $$
    /// - If $\ln(1+x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\ln(1+x)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\ln(1+x)|\rfloor-p}$.
    ///
    /// Unlike [its `Float` counterpart](crate::Float::ln_1_plus_x_prec_round), this function
    /// accepts arguments with any exponent, and its result never underflows.
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
    /// use core::f64::consts::PI;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(x.ln_1_plus_x_prec_assign(5), Greater);
    /// assert_eq!(x.to_string(), "1.44");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(x.ln_1_plus_x_prec_assign(20), Less);
    /// assert_eq!(x.to_string(), "1.4210796");
    /// ```
    #[inline]
    pub fn ln_1_plus_x_prec_assign(&mut self, prec: u64) -> Ordering {
        let (result, o) = log_1_plus_x_helper(self, LogBase::E, prec, Nearest);
        *self = result;
        o
    }
}
