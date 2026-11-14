// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::big_exponent_float::BigExponentFloat;
use crate::big_exponent_float::exponential::log_base::{GeneralBase, log_general_rational_helper};
use core::cmp::Ordering;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_q::Rational;

impl BigExponentFloat {
    #[allow(clippy::needless_pass_by_value)]
    /// Computes $\log_b x$, where $x$ is a [`Rational`] and the base $b$ is a [`BigExponentFloat`],
    /// rounding the result to the specified precision and with the specified rounding mode and
    /// returning the result as a [`BigExponentFloat`]. The [`Rational`] and the base are both taken
    /// by value. An [`Ordering`] is also returned, indicating whether the rounded logarithm is less
    /// than, equal to, or greater than the exact logarithm. Although `NaN`s are not comparable to
    /// any [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The base may be any [`BigExponentFloat`]: the function is $\ln x / \ln b$, with the special
    /// values of FLINT's `arf_div` (which has no negative zero, so a zero divisor gives `NaN`). In
    /// particular, a base of 1 gives `NaN`, where [its `Float`
    /// counterpart](crate::Float::log_base_rational_float_base_prec_round) gives an infinity.
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
    /// - $f(x,\text{NaN},p,m)=\text{NaN}$
    /// - $f(x,b,p,m)=\text{NaN}$ if $x<0$ or $b<0$
    /// - $f(0,b,p,m)=-\infty$ if $b>1$, and $\infty$ if $0<b<1$
    /// - $f(1,b,p,m)=0$ if $b\neq 1$
    /// - $f(x,\infty,p,m)=f(x,0,p,m)=0$ for $x>0$, and $\text{NaN}$ for $x=0$
    /// - $f(x,1,p,m)=\text{NaN}$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::log_base_rational_float_base_prec_round),
    /// this function accepts bases with any exponent, and its result never overflows or underflows.
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`BigExponentFloat::log_base_rational_big_exponent_float_base_prec`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^2 \log\log n + m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `max(x.significant_bits(), base.significant_bits())`.
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
    /// let (y, o) = BigExponentFloat::log_base_rational_big_exponent_float_base_prec_round(
    ///     Rational::from_unsigneds(3u8, 5),
    ///     BigExponentFloat::from(3),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{y:?}"), "-0x0.78");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::log_base_rational_big_exponent_float_base_prec_round(
    ///     Rational::from_unsigneds(3u8, 5),
    ///     BigExponentFloat::from(3),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{y:?}"), "-0x0.74");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::log_base_rational_big_exponent_float_base_prec_round(
    ///     Rational::from_unsigneds(3u8, 5),
    ///     BigExponentFloat::from(3),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{y:?}"), "-0x0.78");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::log_base_rational_big_exponent_float_base_prec_round(
    ///     Rational::from_unsigneds(3u8, 5),
    ///     BigExponentFloat::from(3),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{y:?}"), "-0x0.770888");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::log_base_rational_big_exponent_float_base_prec_round(
    ///     Rational::from_unsigneds(3u8, 5),
    ///     BigExponentFloat::from(3),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{y:?}"), "-0x0.77088");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::log_base_rational_big_exponent_float_base_prec_round(
    ///     Rational::from_unsigneds(3u8, 5),
    ///     BigExponentFloat::from(3),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{y:?}"), "-0x0.77088");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn log_base_rational_big_exponent_float_base_prec_round(
        x: Rational,
        base: Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        log_general_rational_helper(&x, GeneralBase::BigExponentFloat(&base), prec, rm)
    }

    /// Computes $\log_b x$, where $x$ is a [`Rational`] and the base $b$ is a [`BigExponentFloat`],
    /// rounding the result to the specified precision and with the specified rounding mode and
    /// returning the result as a [`BigExponentFloat`]. The [`Rational`] and the base are both taken
    /// by reference. An [`Ordering`] is also returned, indicating whether the rounded logarithm is
    /// less than, equal to, or greater than the exact logarithm. Although `NaN`s are not comparable
    /// to any [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The base may be any [`BigExponentFloat`]: the function is $\ln x / \ln b$, with the special
    /// values of FLINT's `arf_div` (which has no negative zero, so a zero divisor gives `NaN`). In
    /// particular, a base of 1 gives `NaN`, where [its `Float`
    /// counterpart](crate::Float::log_base_rational_float_base_prec_round) gives an infinity.
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
    /// - $f(x,\text{NaN},p,m)=\text{NaN}$
    /// - $f(x,b,p,m)=\text{NaN}$ if $x<0$ or $b<0$
    /// - $f(0,b,p,m)=-\infty$ if $b>1$, and $\infty$ if $0<b<1$
    /// - $f(1,b,p,m)=0$ if $b\neq 1$
    /// - $f(x,\infty,p,m)=f(x,0,p,m)=0$ for $x>0$, and $\text{NaN}$ for $x=0$
    /// - $f(x,1,p,m)=\text{NaN}$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::log_base_rational_float_base_prec_round),
    /// this function accepts bases with any exponent, and its result never overflows or underflows.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^2 \log\log n + m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `max(x.significant_bits(), base.significant_bits())`.
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
    /// let (y, o) = BigExponentFloat::log_base_rational_big_exponent_float_base_prec_round_ref(
    ///     &Rational::from_unsigneds(3u8, 5),
    ///     &BigExponentFloat::from(3),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{y:?}"), "-0x0.78");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::log_base_rational_big_exponent_float_base_prec_round_ref(
    ///     &Rational::from_unsigneds(3u8, 5),
    ///     &BigExponentFloat::from(3),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{y:?}"), "-0x0.74");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::log_base_rational_big_exponent_float_base_prec_round_ref(
    ///     &Rational::from_unsigneds(3u8, 5),
    ///     &BigExponentFloat::from(3),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{y:?}"), "-0x0.78");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::log_base_rational_big_exponent_float_base_prec_round_ref(
    ///     &Rational::from_unsigneds(3u8, 5),
    ///     &BigExponentFloat::from(3),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{y:?}"), "-0x0.770888");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::log_base_rational_big_exponent_float_base_prec_round_ref(
    ///     &Rational::from_unsigneds(3u8, 5),
    ///     &BigExponentFloat::from(3),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{y:?}"), "-0x0.77088");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::log_base_rational_big_exponent_float_base_prec_round_ref(
    ///     &Rational::from_unsigneds(3u8, 5),
    ///     &BigExponentFloat::from(3),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{y:?}"), "-0x0.77088");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn log_base_rational_big_exponent_float_base_prec_round_ref(
        x: &Rational,
        base: &Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        log_general_rational_helper(x, GeneralBase::BigExponentFloat(base), prec, rm)
    }

    #[allow(clippy::needless_pass_by_value)]
    /// Computes $\log_b x$, where $x$ is a [`Rational`] and the base $b$ is a [`BigExponentFloat`],
    /// rounding the result to the nearest value of the specified precision and returning the result
    /// as a [`BigExponentFloat`]. The [`Rational`] and the base are both taken by value. An
    /// [`Ordering`] is also returned, indicating whether the rounded logarithm is less than, equal
    /// to, or greater than the exact logarithm. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The base may be any [`BigExponentFloat`]: the function is $\ln x / \ln b$, with the special
    /// values of FLINT's `arf_div` (which has no negative zero, so a zero divisor gives `NaN`). In
    /// particular, a base of 1 gives `NaN`, where [its `Float`
    /// counterpart](crate::Float::log_base_rational_float_base_prec_round) gives an infinity.
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
    /// - $f(x,\text{NaN},p)=\text{NaN}$
    /// - $f(x,b,p)=\text{NaN}$ if $x<0$ or $b<0$
    /// - $f(0,b,p)=-\infty$ if $b>1$, and $\infty$ if $0<b<1$
    /// - $f(1,b,p)=0$ if $b\neq 1$
    /// - $f(x,\infty,p)=f(x,0,p)=0$ for $x>0$, and $\text{NaN}$ for $x=0$
    /// - $f(x,1,p)=\text{NaN}$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::log_base_rational_float_base_prec_round),
    /// this function accepts bases with any exponent, and its result never overflows or underflows.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`BigExponentFloat::log_base_rational_big_exponent_float_base_prec_round`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^2 \log\log n + m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `max(x.significant_bits(), base.significant_bits())`.
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
    /// let (y, o) = BigExponentFloat::log_base_rational_big_exponent_float_base_prec(
    ///     Rational::from_unsigneds(3u8, 5),
    ///     BigExponentFloat::from(3),
    ///     5,
    /// );
    /// assert_eq!(format!("{y:?}"), "-0x0.78");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::log_base_rational_big_exponent_float_base_prec(
    ///     Rational::from_unsigneds(3u8, 5),
    ///     BigExponentFloat::from(3),
    ///     20,
    /// );
    /// assert_eq!(format!("{y:?}"), "-0x0.77088");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn log_base_rational_big_exponent_float_base_prec(
        x: Rational,
        base: Self,
        prec: u64,
    ) -> (Self, Ordering) {
        log_general_rational_helper(&x, GeneralBase::BigExponentFloat(&base), prec, Nearest)
    }

    /// Computes $\log_b x$, where $x$ is a [`Rational`] and the base $b$ is a [`BigExponentFloat`],
    /// rounding the result to the nearest value of the specified precision and returning the result
    /// as a [`BigExponentFloat`]. The [`Rational`] and the base are both taken by reference. An
    /// [`Ordering`] is also returned, indicating whether the rounded logarithm is less than, equal
    /// to, or greater than the exact logarithm. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The base may be any [`BigExponentFloat`]: the function is $\ln x / \ln b$, with the special
    /// values of FLINT's `arf_div` (which has no negative zero, so a zero divisor gives `NaN`). In
    /// particular, a base of 1 gives `NaN`, where [its `Float`
    /// counterpart](crate::Float::log_base_rational_float_base_prec_round) gives an infinity.
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
    /// - $f(x,\text{NaN},p)=\text{NaN}$
    /// - $f(x,b,p)=\text{NaN}$ if $x<0$ or $b<0$
    /// - $f(0,b,p)=-\infty$ if $b>1$, and $\infty$ if $0<b<1$
    /// - $f(1,b,p)=0$ if $b\neq 1$
    /// - $f(x,\infty,p)=f(x,0,p)=0$ for $x>0$, and $\text{NaN}$ for $x=0$
    /// - $f(x,1,p)=\text{NaN}$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::log_base_rational_float_base_prec_round),
    /// this function accepts bases with any exponent, and its result never overflows or underflows.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^2 \log\log n + m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `max(x.significant_bits(), base.significant_bits())`.
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
    /// let (y, o) = BigExponentFloat::log_base_rational_big_exponent_float_base_prec_ref(
    ///     &Rational::from_unsigneds(3u8, 5),
    ///     &BigExponentFloat::from(3),
    ///     5,
    /// );
    /// assert_eq!(format!("{y:?}"), "-0x0.78");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::log_base_rational_big_exponent_float_base_prec_ref(
    ///     &Rational::from_unsigneds(3u8, 5),
    ///     &BigExponentFloat::from(3),
    ///     20,
    /// );
    /// assert_eq!(format!("{y:?}"), "-0x0.77088");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn log_base_rational_big_exponent_float_base_prec_ref(
        x: &Rational,
        base: &Self,
        prec: u64,
    ) -> (Self, Ordering) {
        log_general_rational_helper(x, GeneralBase::BigExponentFloat(base), prec, Nearest)
    }
}
