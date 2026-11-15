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
    /// Computes $\log_{2^k}(1+x)$, where $x$ is a [`BigExponentFloat`] and $k$ is an [`i64`],
    /// rounding the result to the specified precision and with the specified rounding mode. The
    /// [`BigExponentFloat`] is taken by value. An [`Ordering`] is also returned, indicating whether
    /// the rounded value is less than, equal to, or greater than the exact value. Although `NaN`s
    /// are not comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it
    /// also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,k,p,m) = \log_{2^k}(1+x)+\varepsilon.
    /// $$
    /// - If $\log_{2^k}(1+x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed
    ///   to be 0.
    /// - If $\log_{2^k}(1+x)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon|
    ///   < 2^{\lfloor\log_2 |\log_{2^k}(1+x)|\rfloor-p+1}$.
    /// - If $\log_{2^k}(1+x)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\log_{2^k}(1+x)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},k,p,m)=\text{NaN}$
    /// - $f(\infty,k,p,m)=\infty$ if $k>0$, and $-\infty$ if $k<0$
    /// - $f(-\infty,k,p,m)=\text{NaN}$
    /// - $f(0,k,p,m)=0$
    /// - $f(-1,k,p,m)=-\infty$ if $k>0$, and $\infty$ if $k<0$
    /// - $f(x,k,p,m)=\text{NaN}$ if $x<-1$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::log_base_power_of_2_1_plus_x_prec_round),
    /// this function accepts arguments with any exponent, and its result never underflows.
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`BigExponentFloat::log_base_power_of_2_1_plus_x_prec`] instead.
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
    /// Panics if `prec` is zero, if `pow` is zero (the base $2^0=1$ has no logarithm), or if `rm`
    /// is `Exact` but the result cannot be represented exactly with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (y, o) =
    ///     BigExponentFloat::from(PI).log_base_power_of_2_1_plus_x_prec_round(3, 5, Floor);
    /// assert_eq!(y.to_string(), "0.656");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) =
    ///     BigExponentFloat::from(PI).log_base_power_of_2_1_plus_x_prec_round(3, 5, Ceiling);
    /// assert_eq!(y.to_string(), "0.688");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) =
    ///     BigExponentFloat::from(PI).log_base_power_of_2_1_plus_x_prec_round(3, 5, Nearest);
    /// assert_eq!(y.to_string(), "0.688");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) =
    ///     BigExponentFloat::from(PI).log_base_power_of_2_1_plus_x_prec_round(3, 20, Floor);
    /// assert_eq!(y.to_string(), "0.68339443");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) =
    ///     BigExponentFloat::from(PI).log_base_power_of_2_1_plus_x_prec_round(3, 20, Ceiling);
    /// assert_eq!(y.to_string(), "0.683395");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) =
    ///     BigExponentFloat::from(PI).log_base_power_of_2_1_plus_x_prec_round(3, 20, Nearest);
    /// assert_eq!(y.to_string(), "0.683395");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn log_base_power_of_2_1_plus_x_prec_round(
        self,
        pow: i64,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        log_1_plus_x_helper(&self, LogBase::PowerOf2(pow), prec, rm)
    }

    /// Computes $\log_{2^k}(1+x)$, where $x$ is a [`BigExponentFloat`] and $k$ is an [`i64`],
    /// rounding the result to the specified precision and with the specified rounding mode. The
    /// [`BigExponentFloat`] is taken by reference. An [`Ordering`] is also returned, indicating
    /// whether the rounded value is less than, equal to, or greater than the exact value. Although
    /// `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function returns a
    /// `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,k,p,m) = \log_{2^k}(1+x)+\varepsilon.
    /// $$
    /// - If $\log_{2^k}(1+x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed
    ///   to be 0.
    /// - If $\log_{2^k}(1+x)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon|
    ///   < 2^{\lfloor\log_2 |\log_{2^k}(1+x)|\rfloor-p+1}$.
    /// - If $\log_{2^k}(1+x)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\log_{2^k}(1+x)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},k,p,m)=\text{NaN}$
    /// - $f(\infty,k,p,m)=\infty$ if $k>0$, and $-\infty$ if $k<0$
    /// - $f(-\infty,k,p,m)=\text{NaN}$
    /// - $f(0,k,p,m)=0$
    /// - $f(-1,k,p,m)=-\infty$ if $k>0$, and $\infty$ if $k<0$
    /// - $f(x,k,p,m)=\text{NaN}$ if $x<-1$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::log_base_power_of_2_1_plus_x_prec_round),
    /// this function accepts arguments with any exponent, and its result never underflows.
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
    /// Panics if `prec` is zero, if `pow` is zero (the base $2^0=1$ has no logarithm), or if `rm`
    /// is `Exact` but the result cannot be represented exactly with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (y, o) =
    ///     (&BigExponentFloat::from(PI)).log_base_power_of_2_1_plus_x_prec_round_ref(3, 5, Floor);
    /// assert_eq!(y.to_string(), "0.656");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = (&BigExponentFloat::from(PI))
    ///     .log_base_power_of_2_1_plus_x_prec_round_ref(3, 5, Ceiling);
    /// assert_eq!(y.to_string(), "0.688");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = (&BigExponentFloat::from(PI))
    ///     .log_base_power_of_2_1_plus_x_prec_round_ref(3, 5, Nearest);
    /// assert_eq!(y.to_string(), "0.688");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) =
    ///     (&BigExponentFloat::from(PI)).log_base_power_of_2_1_plus_x_prec_round_ref(3, 20, Floor);
    /// assert_eq!(y.to_string(), "0.68339443");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = (&BigExponentFloat::from(PI))
    ///     .log_base_power_of_2_1_plus_x_prec_round_ref(3, 20, Ceiling);
    /// assert_eq!(y.to_string(), "0.683395");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = (&BigExponentFloat::from(PI))
    ///     .log_base_power_of_2_1_plus_x_prec_round_ref(3, 20, Nearest);
    /// assert_eq!(y.to_string(), "0.683395");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn log_base_power_of_2_1_plus_x_prec_round_ref(
        &self,
        pow: i64,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        log_1_plus_x_helper(self, LogBase::PowerOf2(pow), prec, rm)
    }

    /// Computes $\log_{2^k}(1+x)$, where $x$ is a [`BigExponentFloat`] and $k$ is an [`i64`], in
    /// place, rounding the result to the specified precision and with the specified rounding mode.
    /// An [`Ordering`] is returned, indicating whether the rounded value is less than, equal to, or
    /// greater than the exact value. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function sets the [`BigExponentFloat`] to `NaN` it also
    /// returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets \log_{2^k}(1+x)+\varepsilon.
    /// $$
    /// - If $\log_{2^k}(1+x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed
    ///   to be 0.
    /// - If $\log_{2^k}(1+x)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon|
    ///   < 2^{\lfloor\log_2 |\log_{2^k}(1+x)|\rfloor-p+1}$.
    /// - If $\log_{2^k}(1+x)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\log_{2^k}(1+x)|\rfloor-p}$.
    ///
    /// Unlike [its `Float` counterpart](crate::Float::log_base_power_of_2_1_plus_x_prec_round),
    /// this function accepts arguments with any exponent, and its result never underflows.
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
    /// Panics if `prec` is zero, if `pow` is zero (the base $2^0=1$ has no logarithm), or if `rm`
    /// is `Exact` but the result cannot be represented exactly with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.log_base_power_of_2_1_plus_x_prec_round_assign(3, 5, Floor),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "0.656");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.log_base_power_of_2_1_plus_x_prec_round_assign(3, 5, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "0.688");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.log_base_power_of_2_1_plus_x_prec_round_assign(3, 5, Nearest),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "0.688");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.log_base_power_of_2_1_plus_x_prec_round_assign(3, 20, Floor),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "0.68339443");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.log_base_power_of_2_1_plus_x_prec_round_assign(3, 20, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "0.683395");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.log_base_power_of_2_1_plus_x_prec_round_assign(3, 20, Nearest),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "0.683395");
    /// ```
    #[inline]
    pub fn log_base_power_of_2_1_plus_x_prec_round_assign(
        &mut self,
        pow: i64,
        prec: u64,
        rm: RoundingMode,
    ) -> Ordering {
        let (result, o) = log_1_plus_x_helper(self, LogBase::PowerOf2(pow), prec, rm);
        *self = result;
        o
    }

    /// Computes $\log_{2^k}(1+x)$, where $x$ is a [`BigExponentFloat`] and $k$ is an [`i64`],
    /// rounding the result to the nearest value of the specified precision. The
    /// [`BigExponentFloat`] is taken by value. An [`Ordering`] is also returned, indicating whether
    /// the rounded value is less than, equal to, or greater than the exact value. Although `NaN`s
    /// are not comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it
    /// also returns `Equal`.
    ///
    /// $$
    /// f(x,k,p) = \log_{2^k}(1+x)+\varepsilon.
    /// $$
    /// - If $\log_{2^k}(1+x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed
    ///   to be 0.
    /// - If $\log_{2^k}(1+x)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\log_{2^k}(1+x)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},k,p)=\text{NaN}$
    /// - $f(\infty,k,p)=\infty$ if $k>0$, and $-\infty$ if $k<0$
    /// - $f(-\infty,k,p)=\text{NaN}$
    /// - $f(0,k,p)=0$
    /// - $f(-1,k,p)=-\infty$ if $k>0$, and $\infty$ if $k<0$
    /// - $f(x,k,p)=\text{NaN}$ if $x<-1$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::log_base_power_of_2_1_plus_x_prec_round),
    /// this function accepts arguments with any exponent, and its result never underflows.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`BigExponentFloat::log_base_power_of_2_1_plus_x_prec_round`] instead.
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
    /// Panics if `prec` is zero, if `pow` is zero (the base $2^0=1$ has no logarithm).
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::num::arithmetic::traits::PowerOf2;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (y, o) = BigExponentFloat::from(PI).log_base_power_of_2_1_plus_x_prec(3, 5);
    /// assert_eq!(y.to_string(), "0.688");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::from(PI).log_base_power_of_2_1_plus_x_prec(3, 20);
    /// assert_eq!(y.to_string(), "0.683395");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) =
    ///     BigExponentFloat::power_of_2(1000000000000i64).log_base_power_of_2_1_plus_x_prec(3, 10);
    /// assert_eq!(y.to_string(), "3.3340e11");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn log_base_power_of_2_1_plus_x_prec(self, pow: i64, prec: u64) -> (Self, Ordering) {
        log_1_plus_x_helper(&self, LogBase::PowerOf2(pow), prec, Nearest)
    }

    /// Computes $\log_{2^k}(1+x)$, where $x$ is a [`BigExponentFloat`] and $k$ is an [`i64`],
    /// rounding the result to the nearest value of the specified precision. The
    /// [`BigExponentFloat`] is taken by reference. An [`Ordering`] is also returned, indicating
    /// whether the rounded value is less than, equal to, or greater than the exact value. Although
    /// `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function returns a
    /// `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,k,p) = \log_{2^k}(1+x)+\varepsilon.
    /// $$
    /// - If $\log_{2^k}(1+x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed
    ///   to be 0.
    /// - If $\log_{2^k}(1+x)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\log_{2^k}(1+x)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},k,p)=\text{NaN}$
    /// - $f(\infty,k,p)=\infty$ if $k>0$, and $-\infty$ if $k<0$
    /// - $f(-\infty,k,p)=\text{NaN}$
    /// - $f(0,k,p)=0$
    /// - $f(-1,k,p)=-\infty$ if $k>0$, and $\infty$ if $k<0$
    /// - $f(x,k,p)=\text{NaN}$ if $x<-1$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::log_base_power_of_2_1_plus_x_prec_round),
    /// this function accepts arguments with any exponent, and its result never underflows.
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
    /// Panics if `prec` is zero, if `pow` is zero (the base $2^0=1$ has no logarithm).
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (y, o) = (&BigExponentFloat::from(PI)).log_base_power_of_2_1_plus_x_prec_ref(3, 5);
    /// assert_eq!(y.to_string(), "0.688");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = (&BigExponentFloat::from(PI)).log_base_power_of_2_1_plus_x_prec_ref(3, 20);
    /// assert_eq!(y.to_string(), "0.683395");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn log_base_power_of_2_1_plus_x_prec_ref(&self, pow: i64, prec: u64) -> (Self, Ordering) {
        log_1_plus_x_helper(self, LogBase::PowerOf2(pow), prec, Nearest)
    }

    /// Computes $\log_{2^k}(1+x)$, where $x$ is a [`BigExponentFloat`] and $k$ is an [`i64`], in
    /// place, rounding the result to the nearest value of the specified precision. An [`Ordering`]
    /// is returned, indicating whether the rounded value is less than, equal to, or greater than
    /// the exact value. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever
    /// this function sets the [`BigExponentFloat`] to `NaN` it also returns `Equal`.
    ///
    /// $$
    /// x \gets \log_{2^k}(1+x)+\varepsilon.
    /// $$
    /// - If $\log_{2^k}(1+x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed
    ///   to be 0.
    /// - If $\log_{2^k}(1+x)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\log_{2^k}(1+x)|\rfloor-p}$.
    ///
    /// Unlike [its `Float` counterpart](crate::Float::log_base_power_of_2_1_plus_x_prec_round),
    /// this function accepts arguments with any exponent, and its result never underflows.
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
    /// Panics if `prec` is zero, if `pow` is zero (the base $2^0=1$ has no logarithm).
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(x.log_base_power_of_2_1_plus_x_prec_assign(3, 5), Greater);
    /// assert_eq!(x.to_string(), "0.688");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(x.log_base_power_of_2_1_plus_x_prec_assign(3, 20), Greater);
    /// assert_eq!(x.to_string(), "0.683395");
    /// ```
    #[inline]
    pub fn log_base_power_of_2_1_plus_x_prec_assign(&mut self, pow: i64, prec: u64) -> Ordering {
        let (result, o) = log_1_plus_x_helper(self, LogBase::PowerOf2(pow), prec, Nearest);
        *self = result;
        o
    }
}
