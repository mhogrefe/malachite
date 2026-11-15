// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::big_exponent_float::BigExponentFloat;
use crate::big_exponent_float::exponential::log_base::{GeneralBase, log_general_helper};
use alloc::borrow::Cow;
use core::cmp::Ordering;
use malachite_base::num::basic::traits::Zero;
use malachite_base::rounding_modes::RoundingMode::{self, *};

impl BigExponentFloat {
    /// Computes $\log_b x$, where $x$ and the base $b$ are both [`BigExponentFloat`]s, rounding the
    /// result to the specified precision and with the specified rounding mode. The
    /// [`BigExponentFloat`] is taken by value and the base by reference. An [`Ordering`] is also
    /// returned, indicating whether the rounded logarithm is less than, equal to, or greater than
    /// the exact logarithm. Although `NaN`s are not comparable to any [`BigExponentFloat`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The base may be any [`BigExponentFloat`]: the function is $\ln x / \ln b$, with the special
    /// values of FLINT's `arf_div` (which has no negative zero, so a zero divisor gives `NaN`). In
    /// particular, a base of 1 gives `NaN`, where [its `Float`
    /// counterpart](crate::Float::log_base_float_base_prec_round) gives an infinity.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,b,p,m) = \log_b x+\varepsilon.
    /// $$
    /// - If $\log_b x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\log_b x$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\log_b x|\rfloor-p+1}$.
    /// - If $\log_b x$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\log_b x|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},b,p,m)=f(x,\text{NaN},p,m)=\text{NaN}$
    /// - $f(x,b,p,m)=\text{NaN}$ if $x<0$ or $b<0$
    /// - $f(\infty,b,p,m)=\infty$ if $b>1$, and $-\infty$ if $0<b<1$
    /// - $f(0,b,p,m)=-\infty$ if $b>1$, and $\infty$ if $0<b<1$
    /// - $f(1,b,p,m)=0$ if $b\neq 1$
    /// - $f(x,\infty,p,m)=f(x,0,p,m)=0$ for finite $x>0$, and $\text{NaN}$ for $x=0$ or $x=\infty$
    /// - $f(x,1,p,m)=\text{NaN}$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::log_base_float_base_prec_round), this
    /// function accepts arguments with any exponent, and its result never overflows or underflows.
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`BigExponentFloat::log_base_big_exponent_float_base_prec`] instead.
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
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (y, o) = BigExponentFloat::from(PI).log_base_big_exponent_float_base_prec_round(
    ///     &BigExponentFloat::from(3),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(y.to_string(), "1.0");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::from(PI).log_base_big_exponent_float_base_prec_round(
    ///     &BigExponentFloat::from(3),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(y.to_string(), "1.06");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::from(PI).log_base_big_exponent_float_base_prec_round(
    ///     &BigExponentFloat::from(3),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(y.to_string(), "1.06");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::from(PI).log_base_big_exponent_float_base_prec_round(
    ///     &BigExponentFloat::from(3),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(y.to_string(), "1.041977");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::from(PI).log_base_big_exponent_float_base_prec_round(
    ///     &BigExponentFloat::from(3),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(y.to_string(), "1.0419788");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::from(PI).log_base_big_exponent_float_base_prec_round(
    ///     &BigExponentFloat::from(3),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(y.to_string(), "1.0419788");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn log_base_big_exponent_float_base_prec_round(
        self,
        base: &Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        log_general_helper(
            Cow::Owned(self),
            GeneralBase::BigExponentFloat(base),
            prec,
            rm,
        )
    }

    /// Computes $\log_b x$, where $x$ and the base $b$ are both [`BigExponentFloat`]s, rounding the
    /// result to the specified precision and with the specified rounding mode. The
    /// [`BigExponentFloat`] is taken by reference and the base by reference. An [`Ordering`] is
    /// also returned, indicating whether the rounded logarithm is less than, equal to, or greater
    /// than the exact logarithm. Although `NaN`s are not comparable to any [`BigExponentFloat`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The base may be any [`BigExponentFloat`]: the function is $\ln x / \ln b$, with the special
    /// values of FLINT's `arf_div` (which has no negative zero, so a zero divisor gives `NaN`). In
    /// particular, a base of 1 gives `NaN`, where [its `Float`
    /// counterpart](crate::Float::log_base_float_base_prec_round) gives an infinity.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,b,p,m) = \log_b x+\varepsilon.
    /// $$
    /// - If $\log_b x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\log_b x$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\log_b x|\rfloor-p+1}$.
    /// - If $\log_b x$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\log_b x|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},b,p,m)=f(x,\text{NaN},p,m)=\text{NaN}$
    /// - $f(x,b,p,m)=\text{NaN}$ if $x<0$ or $b<0$
    /// - $f(\infty,b,p,m)=\infty$ if $b>1$, and $-\infty$ if $0<b<1$
    /// - $f(0,b,p,m)=-\infty$ if $b>1$, and $\infty$ if $0<b<1$
    /// - $f(1,b,p,m)=0$ if $b\neq 1$
    /// - $f(x,\infty,p,m)=f(x,0,p,m)=0$ for finite $x>0$, and $\text{NaN}$ for $x=0$ or $x=\infty$
    /// - $f(x,1,p,m)=\text{NaN}$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::log_base_float_base_prec_round), this
    /// function accepts arguments with any exponent, and its result never overflows or underflows.
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
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (y, o) = (&BigExponentFloat::from(PI)).log_base_big_exponent_float_base_prec_round_ref(
    ///     &BigExponentFloat::from(3),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(y.to_string(), "1.0");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = (&BigExponentFloat::from(PI)).log_base_big_exponent_float_base_prec_round_ref(
    ///     &BigExponentFloat::from(3),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(y.to_string(), "1.06");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = (&BigExponentFloat::from(PI)).log_base_big_exponent_float_base_prec_round_ref(
    ///     &BigExponentFloat::from(3),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(y.to_string(), "1.06");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = (&BigExponentFloat::from(PI)).log_base_big_exponent_float_base_prec_round_ref(
    ///     &BigExponentFloat::from(3),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(y.to_string(), "1.041977");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = (&BigExponentFloat::from(PI)).log_base_big_exponent_float_base_prec_round_ref(
    ///     &BigExponentFloat::from(3),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(y.to_string(), "1.0419788");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = (&BigExponentFloat::from(PI)).log_base_big_exponent_float_base_prec_round_ref(
    ///     &BigExponentFloat::from(3),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(y.to_string(), "1.0419788");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn log_base_big_exponent_float_base_prec_round_ref(
        &self,
        base: &Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        log_general_helper(
            Cow::Borrowed(self),
            GeneralBase::BigExponentFloat(base),
            prec,
            rm,
        )
    }

    /// Computes $\log_b x$, where $x$ and the base $b$ are both [`BigExponentFloat`]s, in place,
    /// rounding the result to the specified precision and with the specified rounding mode. The
    /// base is taken by reference. An [`Ordering`] is returned, indicating whether the rounded
    /// logarithm is less than, equal to, or greater than the exact logarithm. Although `NaN`s are
    /// not comparable to any [`BigExponentFloat`], whenever this function sets the
    /// [`BigExponentFloat`] to `NaN` it also returns `Equal`.
    ///
    /// The base may be any [`BigExponentFloat`]: the function is $\ln x / \ln b$, with the special
    /// values of FLINT's `arf_div` (which has no negative zero, so a zero divisor gives `NaN`). In
    /// particular, a base of 1 gives `NaN`, where [its `Float`
    /// counterpart](crate::Float::log_base_float_base_prec_round) gives an infinity.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets \log_b x+\varepsilon.
    /// $$
    /// - If $\log_b x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\log_b x$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\log_b x|\rfloor-p+1}$.
    /// - If $\log_b x$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\log_b x|\rfloor-p}$.
    ///
    /// Unlike [its `Float` counterpart](crate::Float::log_base_float_base_prec_round), this
    /// function accepts arguments with any exponent, and its result never overflows or underflows.
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
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with `prec` bits.
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
    ///     x.log_base_big_exponent_float_base_prec_round_assign(
    ///         &BigExponentFloat::from(3),
    ///         5,
    ///         Floor
    ///     ),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "1.0");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.log_base_big_exponent_float_base_prec_round_assign(
    ///         &BigExponentFloat::from(3),
    ///         5,
    ///         Ceiling
    ///     ),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "1.06");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.log_base_big_exponent_float_base_prec_round_assign(
    ///         &BigExponentFloat::from(3),
    ///         5,
    ///         Nearest
    ///     ),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "1.06");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.log_base_big_exponent_float_base_prec_round_assign(
    ///         &BigExponentFloat::from(3),
    ///         20,
    ///         Floor
    ///     ),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "1.041977");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.log_base_big_exponent_float_base_prec_round_assign(
    ///         &BigExponentFloat::from(3),
    ///         20,
    ///         Ceiling
    ///     ),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "1.0419788");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.log_base_big_exponent_float_base_prec_round_assign(
    ///         &BigExponentFloat::from(3),
    ///         20,
    ///         Nearest
    ///     ),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "1.0419788");
    /// ```
    #[inline]
    pub fn log_base_big_exponent_float_base_prec_round_assign(
        &mut self,
        base: &Self,
        prec: u64,
        rm: RoundingMode,
    ) -> Ordering {
        let x = core::mem::replace(self, Self::ZERO);
        let (result, o) =
            log_general_helper(Cow::Owned(x), GeneralBase::BigExponentFloat(base), prec, rm);
        *self = result;
        o
    }

    /// Computes $\log_b x$, where $x$ and the base $b$ are both [`BigExponentFloat`]s, rounding the
    /// result to the nearest value of the specified precision. The [`BigExponentFloat`] is taken by
    /// value and the base by reference. An [`Ordering`] is also returned, indicating whether the
    /// rounded logarithm is less than, equal to, or greater than the exact logarithm. Although
    /// `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function returns a
    /// `NaN` it also returns `Equal`.
    ///
    /// The base may be any [`BigExponentFloat`]: the function is $\ln x / \ln b$, with the special
    /// values of FLINT's `arf_div` (which has no negative zero, so a zero divisor gives `NaN`). In
    /// particular, a base of 1 gives `NaN`, where [its `Float`
    /// counterpart](crate::Float::log_base_float_base_prec_round) gives an infinity.
    ///
    /// $$
    /// f(x,b,p) = \log_b x+\varepsilon.
    /// $$
    /// - If $\log_b x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\log_b x$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2 |\log_b
    ///   x|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},b,p)=f(x,\text{NaN},p)=\text{NaN}$
    /// - $f(x,b,p)=\text{NaN}$ if $x<0$ or $b<0$
    /// - $f(\infty,b,p)=\infty$ if $b>1$, and $-\infty$ if $0<b<1$
    /// - $f(0,b,p)=-\infty$ if $b>1$, and $\infty$ if $0<b<1$
    /// - $f(1,b,p)=0$ if $b\neq 1$
    /// - $f(x,\infty,p)=f(x,0,p)=0$ for finite $x>0$, and $\text{NaN}$ for $x=0$ or $x=\infty$
    /// - $f(x,1,p)=\text{NaN}$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::log_base_float_base_prec_round), this
    /// function accepts arguments with any exponent, and its result never overflows or underflows.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`BigExponentFloat::log_base_big_exponent_float_base_prec_round`] instead.
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
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::num::arithmetic::traits::PowerOf2;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (y, o) = BigExponentFloat::from(PI)
    ///     .log_base_big_exponent_float_base_prec(&BigExponentFloat::from(3), 5);
    /// assert_eq!(y.to_string(), "1.06");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::from(PI)
    ///     .log_base_big_exponent_float_base_prec(&BigExponentFloat::from(3), 20);
    /// assert_eq!(y.to_string(), "1.0419788");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::power_of_2(1000000000000i64)
    ///     .log_base_big_exponent_float_base_prec(&BigExponentFloat::from(3), 10);
    /// assert_eq!(y.to_string(), "6.314e11");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn log_base_big_exponent_float_base_prec(self, base: &Self, prec: u64) -> (Self, Ordering) {
        log_general_helper(
            Cow::Owned(self),
            GeneralBase::BigExponentFloat(base),
            prec,
            Nearest,
        )
    }

    /// Computes $\log_b x$, where $x$ and the base $b$ are both [`BigExponentFloat`]s, rounding the
    /// result to the nearest value of the specified precision. The [`BigExponentFloat`] is taken by
    /// reference and the base by reference. An [`Ordering`] is also returned, indicating whether
    /// the rounded logarithm is less than, equal to, or greater than the exact logarithm. Although
    /// `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function returns a
    /// `NaN` it also returns `Equal`.
    ///
    /// The base may be any [`BigExponentFloat`]: the function is $\ln x / \ln b$, with the special
    /// values of FLINT's `arf_div` (which has no negative zero, so a zero divisor gives `NaN`). In
    /// particular, a base of 1 gives `NaN`, where [its `Float`
    /// counterpart](crate::Float::log_base_float_base_prec_round) gives an infinity.
    ///
    /// $$
    /// f(x,b,p) = \log_b x+\varepsilon.
    /// $$
    /// - If $\log_b x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\log_b x$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2 |\log_b
    ///   x|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},b,p)=f(x,\text{NaN},p)=\text{NaN}$
    /// - $f(x,b,p)=\text{NaN}$ if $x<0$ or $b<0$
    /// - $f(\infty,b,p)=\infty$ if $b>1$, and $-\infty$ if $0<b<1$
    /// - $f(0,b,p)=-\infty$ if $b>1$, and $\infty$ if $0<b<1$
    /// - $f(1,b,p)=0$ if $b\neq 1$
    /// - $f(x,\infty,p)=f(x,0,p)=0$ for finite $x>0$, and $\text{NaN}$ for $x=0$ or $x=\infty$
    /// - $f(x,1,p)=\text{NaN}$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::log_base_float_base_prec_round), this
    /// function accepts arguments with any exponent, and its result never overflows or underflows.
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
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (y, o) = (&BigExponentFloat::from(PI))
    ///     .log_base_big_exponent_float_base_prec_ref(&BigExponentFloat::from(3), 5);
    /// assert_eq!(y.to_string(), "1.06");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = (&BigExponentFloat::from(PI))
    ///     .log_base_big_exponent_float_base_prec_ref(&BigExponentFloat::from(3), 20);
    /// assert_eq!(y.to_string(), "1.0419788");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn log_base_big_exponent_float_base_prec_ref(
        &self,
        base: &Self,
        prec: u64,
    ) -> (Self, Ordering) {
        log_general_helper(
            Cow::Borrowed(self),
            GeneralBase::BigExponentFloat(base),
            prec,
            Nearest,
        )
    }

    /// Computes $\log_b x$, where $x$ and the base $b$ are both [`BigExponentFloat`]s, in place,
    /// rounding the result to the nearest value of the specified precision. The base is taken by
    /// reference. An [`Ordering`] is returned, indicating whether the rounded logarithm is less
    /// than, equal to, or greater than the exact logarithm. Although `NaN`s are not comparable to
    /// any [`BigExponentFloat`], whenever this function sets the [`BigExponentFloat`] to `NaN` it
    /// also returns `Equal`.
    ///
    /// The base may be any [`BigExponentFloat`]: the function is $\ln x / \ln b$, with the special
    /// values of FLINT's `arf_div` (which has no negative zero, so a zero divisor gives `NaN`). In
    /// particular, a base of 1 gives `NaN`, where [its `Float`
    /// counterpart](crate::Float::log_base_float_base_prec_round) gives an infinity.
    ///
    /// $$
    /// x \gets \log_b x+\varepsilon.
    /// $$
    /// - If $\log_b x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\log_b x$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2 |\log_b
    ///   x|\rfloor-p}$.
    ///
    /// Unlike [its `Float` counterpart](crate::Float::log_base_float_base_prec_round), this
    /// function accepts arguments with any exponent, and its result never overflows or underflows.
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
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.log_base_big_exponent_float_base_prec_assign(&BigExponentFloat::from(3), 5),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "1.06");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.log_base_big_exponent_float_base_prec_assign(&BigExponentFloat::from(3), 20),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "1.0419788");
    /// ```
    #[inline]
    pub fn log_base_big_exponent_float_base_prec_assign(
        &mut self,
        base: &Self,
        prec: u64,
    ) -> Ordering {
        let x = core::mem::replace(self, Self::ZERO);
        let (result, o) = log_general_helper(
            Cow::Owned(x),
            GeneralBase::BigExponentFloat(base),
            prec,
            Nearest,
        );
        *self = result;
        o
    }
}
