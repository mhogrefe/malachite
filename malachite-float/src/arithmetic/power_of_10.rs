// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Float;
use crate::{emulate_float_to_float_fn, emulate_rational_to_float_fn};
use core::cmp::Ordering;
use malachite_base::num::arithmetic::traits::{PowerOf10, PowerOf10Assign};
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::conversion::traits::{ExactFrom, RoundingFrom};
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_q::Rational;

impl Float {
    #[allow(clippy::needless_pass_by_value)]
    /// Computes $10^x$, where $x$ is a [`Float`], rounding the result to the specified precision
    /// and with the specified rounding mode. The [`Float`] is taken by value. An [`Ordering`] is
    /// also returned, indicating whether the rounded power is less than, equal to, or greater than
    /// the exact power. Although `NaN`s are not comparable to any [`Float`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = 10^x+\varepsilon.
    /// $$
    /// - If $10^x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $10^x$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 10^x\rfloor-p+1}$.
    /// - If $10^x$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 10^x\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=\text{NaN}$
    /// - $f(\infty,p,m)=\infty$
    /// - $f(-\infty,p,m)=0.0$
    /// - $f(\pm0.0,p,m)=1.0$
    ///
    /// Overflow and underflow:
    /// - If $f(x,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$ is
    ///   returned instead.
    /// - If $f(x,p,m)<2^{-2^{30}}$ and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $f(x,p,m)<2^{-2^{30}}$ and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned instead.
    /// - If $f(x,p,m)\leq2^{-2^{30}-1}$ and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,p,m)<2^{-2^{30}}$ and $m$ is `Nearest`, $2^{-2^{30}}$ is returned
    ///   instead.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::power_of_10_of_float_prec`]
    /// instead. If you know that your target precision is the precision of the input, consider
    /// using [`Float::power_of_10_of_float_round`] instead. If both of these things are true,
    /// consider using the [`PowerOf10`] implementation instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with the given precision.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (p, o) = Float::power_of_10_of_float_prec_round(Float::from(0.5), 20, Floor);
    /// assert_eq!(p.to_string(), "3.162277");
    /// assert_eq!(o, Less);
    ///
    /// let (p, o) = Float::power_of_10_of_float_prec_round(Float::from(0.5), 20, Ceiling);
    /// assert_eq!(p.to_string(), "3.162281");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn power_of_10_of_float_prec_round(
        pow: Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        Self::unsigned_pow_prec_round(10, pow, prec, rm)
    }

    /// Computes $10^x$, where $x$ is a [`Float`], rounding the result to the specified precision
    /// and with the specified rounding mode. The [`Float`] is taken by reference. An [`Ordering`]
    /// is also returned, indicating whether the rounded power is less than, equal to, or greater
    /// than the exact power. Although `NaN`s are not comparable to any [`Float`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = 10^x+\varepsilon.
    /// $$
    /// - If $10^x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $10^x$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 10^x\rfloor-p+1}$.
    /// - If $10^x$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 10^x\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=\text{NaN}$
    /// - $f(\infty,p,m)=\infty$
    /// - $f(-\infty,p,m)=0.0$
    /// - $f(\pm0.0,p,m)=1.0$
    ///
    /// Overflow and underflow:
    /// - If $f(x,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$ is
    ///   returned instead.
    /// - If $f(x,p,m)<2^{-2^{30}}$ and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $f(x,p,m)<2^{-2^{30}}$ and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned instead.
    /// - If $f(x,p,m)\leq2^{-2^{30}-1}$ and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,p,m)<2^{-2^{30}}$ and $m$ is `Nearest`, $2^{-2^{30}}$ is returned
    ///   instead.
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`Float::power_of_10_of_float_prec_ref`] instead. If you know that your target precision is
    /// the precision of the input, consider using [`Float::power_of_10_of_float_round_ref`]
    /// instead. If both of these things are true, consider using the [`PowerOf10`] implementation
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with the given precision.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (p, o) = Float::power_of_10_of_float_prec_round_ref(&Float::from(0.5), 20, Floor);
    /// assert_eq!(p.to_string(), "3.162277");
    /// assert_eq!(o, Less);
    ///
    /// let (p, o) = Float::power_of_10_of_float_prec_round_ref(&Float::from(0.5), 20, Ceiling);
    /// assert_eq!(p.to_string(), "3.162281");
    /// assert_eq!(o, Greater);
    /// ```
    pub fn power_of_10_of_float_prec_round_ref(
        pow: &Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        Self::unsigned_pow_prec_round_ref(10, pow, prec, rm)
    }

    #[allow(clippy::needless_pass_by_value)]
    /// Computes $10^x$, where $x$ is a [`Float`], rounding the result to the nearest value of the
    /// specified precision. The [`Float`] is taken by value. An [`Ordering`] is also returned,
    /// indicating whether the rounded power is less than, equal to, or greater than the exact
    /// power. Although `NaN`s are not comparable to any [`Float`], whenever this function returns a
    /// `NaN` it also returns `Equal`.
    ///
    /// If the power is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = 10^x+\varepsilon.
    /// $$
    /// - If $10^x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $10^x$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2 10^x\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p)=\text{NaN}$
    /// - $f(\infty,p)=\infty$
    /// - $f(-\infty,p)=0.0$
    /// - $f(\pm0.0,p)=1.0$
    ///
    /// Overflow and underflow:
    /// - If $f(x,p)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::power_of_10_of_float_prec_round`] instead. If you know that your target precision
    /// is the precision of the input, consider using the [`PowerOf10`] implementation instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (p, o) = Float::power_of_10_of_float_prec(Float::from(0.5), 20);
    /// assert_eq!(p.to_string(), "3.162277");
    /// assert_eq!(o, Less);
    ///
    /// let (p, o) = Float::power_of_10_of_float_prec(Float::from(0.5), 53);
    /// assert_eq!(p.to_string(), "3.1622776601683795");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn power_of_10_of_float_prec(pow: Self, prec: u64) -> (Self, Ordering) {
        Self::power_of_10_of_float_prec_round(pow, prec, Nearest)
    }

    /// Computes $10^x$, where $x$ is a [`Float`], rounding the result to the nearest value of the
    /// specified precision. The [`Float`] is taken by reference. An [`Ordering`] is also returned,
    /// indicating whether the rounded power is less than, equal to, or greater than the exact
    /// power. Although `NaN`s are not comparable to any [`Float`], whenever this function returns a
    /// `NaN` it also returns `Equal`.
    ///
    /// If the power is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = 10^x+\varepsilon.
    /// $$
    /// - If $10^x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $10^x$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2 10^x\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p)=\text{NaN}$
    /// - $f(\infty,p)=\infty$
    /// - $f(-\infty,p)=0.0$
    /// - $f(\pm0.0,p)=1.0$
    ///
    /// Overflow and underflow:
    /// - If $f(x,p)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::power_of_10_of_float_prec_round_ref`] instead. If you know that your target
    /// precision is the precision of the input, consider using the [`PowerOf10`] implementation
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (p, o) = Float::power_of_10_of_float_prec_ref(&Float::from(0.5), 20);
    /// assert_eq!(p.to_string(), "3.162277");
    /// assert_eq!(o, Less);
    ///
    /// let (p, o) = Float::power_of_10_of_float_prec_ref(&Float::from(0.5), 53);
    /// assert_eq!(p.to_string(), "3.1622776601683795");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn power_of_10_of_float_prec_ref(pow: &Self, prec: u64) -> (Self, Ordering) {
        Self::power_of_10_of_float_prec_round_ref(pow, prec, Nearest)
    }

    #[allow(clippy::needless_pass_by_value)]
    /// Computes $10^x$, where $x$ is a [`Float`], rounding the result with the specified rounding
    /// mode. The [`Float`] is taken by value. An [`Ordering`] is also returned, indicating whether
    /// the rounded power is less than, equal to, or greater than the exact power. Although `NaN`s
    /// are not comparable to any [`Float`], whenever this function returns a `NaN` it also returns
    /// `Equal`.
    ///
    /// The precision of the output is the precision of the input. See [`RoundingMode`] for a
    /// description of the possible rounding modes.
    ///
    /// $$
    /// f(x,m) = 10^x+\varepsilon.
    /// $$
    /// - If $10^x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $10^x$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 10^x\rfloor-p+1}$, where $p$ is the precision of the input.
    /// - If $10^x$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 10^x\rfloor-p}$, where $p$ is the precision of the input.
    ///
    /// If the output has a precision, it is the precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN},m)=\text{NaN}$
    /// - $f(\infty,m)=\infty$
    /// - $f(-\infty,m)=0.0$
    /// - $f(\pm0.0,m)=1.0$
    ///
    /// See the [`Float::power_of_10_of_float_prec_round`] documentation for information on overflow
    /// and underflow.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::power_of_10_of_float_prec_round`] instead. If you know you'll be using the
    /// `Nearest` rounding mode, consider using the [`PowerOf10`] implementation instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the result cannot be represented exactly with the input
    /// precision.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from_rational_prec(Rational::from_signeds(1, 2), 20).0;
    ///
    /// let (p, o) = Float::power_of_10_of_float_round(x.clone(), Floor);
    /// assert_eq!(p.to_string(), "3.162277");
    /// assert_eq!(o, Less);
    ///
    /// let (p, o) = Float::power_of_10_of_float_round(x, Ceiling);
    /// assert_eq!(p.to_string(), "3.162281");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn power_of_10_of_float_round(pow: Self, rm: RoundingMode) -> (Self, Ordering) {
        let prec = pow.significant_bits();
        Self::power_of_10_of_float_prec_round_ref(&pow, prec, rm)
    }

    /// Computes $10^x$, where $x$ is a [`Float`], rounding the result with the specified rounding
    /// mode. The [`Float`] is taken by reference. An [`Ordering`] is also returned, indicating
    /// whether the rounded power is less than, equal to, or greater than the exact power. Although
    /// `NaN`s are not comparable to any [`Float`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// The precision of the output is the precision of the input. See [`RoundingMode`] for a
    /// description of the possible rounding modes.
    ///
    /// $$
    /// f(x,m) = 10^x+\varepsilon.
    /// $$
    /// - If $10^x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $10^x$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 10^x\rfloor-p+1}$, where $p$ is the precision of the input.
    /// - If $10^x$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 10^x\rfloor-p}$, where $p$ is the precision of the input.
    ///
    /// If the output has a precision, it is the precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN},m)=\text{NaN}$
    /// - $f(\infty,m)=\infty$
    /// - $f(-\infty,m)=0.0$
    /// - $f(\pm0.0,m)=1.0$
    ///
    /// See the [`Float::power_of_10_of_float_prec_round`] documentation for information on overflow
    /// and underflow.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::power_of_10_of_float_prec_round_ref`] instead. If you know you'll be using the
    /// `Nearest` rounding mode, consider using the [`PowerOf10`] implementation instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the result cannot be represented exactly with the input
    /// precision.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from_rational_prec(Rational::from_signeds(1, 2), 20).0;
    ///
    /// let (p, o) = Float::power_of_10_of_float_round_ref(&x, Floor);
    /// assert_eq!(p.to_string(), "3.162277");
    /// assert_eq!(o, Less);
    ///
    /// let (p, o) = Float::power_of_10_of_float_round_ref(&x, Ceiling);
    /// assert_eq!(p.to_string(), "3.162281");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn power_of_10_of_float_round_ref(pow: &Self, rm: RoundingMode) -> (Self, Ordering) {
        Self::power_of_10_of_float_prec_round_ref(pow, pow.significant_bits(), rm)
    }

    /// Computes $10^x$, where $x$ is a [`Float`], in place, rounding the result to the specified
    /// precision and with the specified rounding mode. An [`Ordering`] is returned, indicating
    /// whether the rounded power is less than, equal to, or greater than the exact power. Although
    /// `NaN`s are not comparable to any [`Float`], whenever this function sets the [`Float`] to
    /// `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets 10^x+\varepsilon.
    /// $$
    /// - If $10^x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $10^x$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 10^x\rfloor-p+1}$.
    /// - If $10^x$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 10^x\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// See the [`Float::power_of_10_of_float_prec_round`] documentation for information on special
    /// cases, overflow, and underflow.
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`Float::power_of_10_of_float_prec_assign`] instead. If you know that your target precision
    /// is the precision of the input, consider using [`Float::power_of_10_of_float_round_assign`]
    /// instead. If both of these things are true, consider using the [`PowerOf10Assign`]
    /// implementation instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with the given precision.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from(0.5);
    /// assert_eq!(x.power_of_10_of_float_prec_round_assign(20, Floor), Less);
    /// assert_eq!(x.to_string(), "3.162277");
    /// ```
    #[inline]
    pub fn power_of_10_of_float_prec_round_assign(
        &mut self,
        prec: u64,
        rm: RoundingMode,
    ) -> Ordering {
        let (result, o) = Self::power_of_10_of_float_prec_round_ref(self, prec, rm);
        *self = result;
        o
    }

    /// Computes $10^x$, where $x$ is a [`Float`], in place, rounding the result to the nearest
    /// value of the specified precision. An [`Ordering`] is returned, indicating whether the
    /// rounded power is less than, equal to, or greater than the exact power. Although `NaN`s are
    /// not comparable to any [`Float`], whenever this function sets the [`Float`] to `NaN` it also
    /// returns `Equal`.
    ///
    /// If the power is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// x \gets 10^x+\varepsilon.
    /// $$
    /// - If $10^x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $10^x$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2 10^x\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// See the [`Float::power_of_10_of_float_prec`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::power_of_10_of_float_prec_round_assign`] instead. If you know that your target
    /// precision is the precision of the input, consider using the [`PowerOf10Assign`]
    /// implementation instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from(0.5);
    /// assert_eq!(x.power_of_10_of_float_prec_assign(20), Less);
    /// assert_eq!(x.to_string(), "3.162277");
    /// ```
    #[inline]
    pub fn power_of_10_of_float_prec_assign(&mut self, prec: u64) -> Ordering {
        self.power_of_10_of_float_prec_round_assign(prec, Nearest)
    }

    /// Computes $10^x$, where $x$ is a [`Float`], in place, rounding the result with the specified
    /// rounding mode. An [`Ordering`] is returned, indicating whether the rounded power is less
    /// than, equal to, or greater than the exact power. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function sets the [`Float`] to `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the precision of the input. See [`RoundingMode`] for a
    /// description of the possible rounding modes.
    ///
    /// $$
    /// x \gets 10^x+\varepsilon.
    /// $$
    /// - If $10^x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $10^x$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 10^x\rfloor-p+1}$, where $p$ is the precision of the input.
    /// - If $10^x$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 10^x\rfloor-p}$, where $p$ is the precision of the input.
    ///
    /// If the output has a precision, it is the precision of the input.
    ///
    /// See the [`Float::power_of_10_of_float_round`] documentation for information on special
    /// cases, overflow, and underflow.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::power_of_10_of_float_prec_round_assign`] instead. If you know you'll be using the
    /// `Nearest` rounding mode, consider using the [`PowerOf10Assign`] implementation instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the result cannot be represented exactly with the input
    /// precision.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from_rational_prec(Rational::from_signeds(1, 2), 20).0;
    ///
    /// let mut x = x;
    /// assert_eq!(x.power_of_10_of_float_round_assign(Ceiling), Greater);
    /// assert_eq!(x.to_string(), "3.162281");
    /// ```
    #[inline]
    pub fn power_of_10_of_float_round_assign(&mut self, rm: RoundingMode) -> Ordering {
        self.power_of_10_of_float_prec_round_assign(self.significant_bits(), rm)
    }

    #[allow(clippy::needless_pass_by_value)]
    /// Computes $10^x$, where $x$ is a [`Rational`], rounding the result to the specified precision
    /// and with the specified rounding mode and returning the result as a [`Float`]. The
    /// [`Rational`] is taken by value. An [`Ordering`] is also returned, indicating whether the
    /// rounded power is less than, equal to, or greater than the exact power.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = 10^x+\varepsilon.
    /// $$
    /// - If $m$ is not `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2 10^x\rfloor-p+1}$.
    /// - If $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2 10^x\rfloor-p}$.
    ///
    /// These bounds do not apply when the result overflows or underflows; see below.
    ///
    /// The output has precision `prec`.
    ///
    /// Special cases:
    /// - $f(0,p,m)=1$.
    ///
    /// Overflow and underflow:
    /// - If $f(x,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$ is
    ///   returned instead.
    /// - If $f(x,p,m)<2^{-2^{30}}$ and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $f(x,p,m)<2^{-2^{30}}$ and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned instead.
    /// - If $f(x,p,m)\leq2^{-2^{30}-1}$ and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,p,m)<2^{-2^{30}}$ and $m$ is `Nearest`, $2^{-2^{30}}$ is returned
    ///   instead.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::power_of_10_rational_prec`]
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with the given precision (which is the case unless $x$ is a nonnegative integer).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (p, o) =
    ///     Float::power_of_10_rational_prec_round(Rational::from_unsigneds(3u8, 5), 5, Floor);
    /// assert_eq!(p.to_string(), "3.9");
    /// assert_eq!(o, Less);
    ///
    /// let (p, o) =
    ///     Float::power_of_10_rational_prec_round(Rational::from_unsigneds(3u8, 5), 5, Ceiling);
    /// assert_eq!(p.to_string(), "4.0");
    /// assert_eq!(o, Greater);
    ///
    /// let (p, o) =
    ///     Float::power_of_10_rational_prec_round(Rational::from_unsigneds(3u8, 5), 20, Floor);
    /// assert_eq!(p.to_string(), "3.981071");
    /// assert_eq!(o, Less);
    ///
    /// let (p, o) =
    ///     Float::power_of_10_rational_prec_round(Rational::from_unsigneds(3u8, 5), 20, Ceiling);
    /// assert_eq!(p.to_string(), "3.981075");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn power_of_10_rational_prec_round(
        x: Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        Self::unsigned_pow_rational_prec_round(10, x, prec, rm)
    }

    /// Computes $10^x$, where $x$ is a [`Rational`], rounding the result to the specified precision
    /// and with the specified rounding mode and returning the result as a [`Float`]. The
    /// [`Rational`] is taken by reference. An [`Ordering`] is also returned, indicating whether the
    /// rounded power is less than, equal to, or greater than the exact power.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = 10^x+\varepsilon.
    /// $$
    /// - If $m$ is not `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2 10^x\rfloor-p+1}$.
    /// - If $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2 10^x\rfloor-p}$.
    ///
    /// These bounds do not apply when the result overflows or underflows; see below.
    ///
    /// The output has precision `prec`.
    ///
    /// Special cases:
    /// - $f(0,p,m)=1$.
    ///
    /// Overflow and underflow:
    /// - If $f(x,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$ is
    ///   returned instead.
    /// - If $f(x,p,m)<2^{-2^{30}}$ and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $f(x,p,m)<2^{-2^{30}}$ and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned instead.
    /// - If $f(x,p,m)\leq2^{-2^{30}-1}$ and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,p,m)<2^{-2^{30}}$ and $m$ is `Nearest`, $2^{-2^{30}}$ is returned
    ///   instead.
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`Float::power_of_10_rational_prec_ref`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with the given precision (which is the case unless $x$ is a nonnegative integer).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (p, o) =
    ///     Float::power_of_10_rational_prec_round_ref(&Rational::from_unsigneds(3u8, 5), 5, Floor);
    /// assert_eq!(p.to_string(), "3.9");
    /// assert_eq!(o, Less);
    ///
    /// let (p, o) = Float::power_of_10_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(3u8, 5),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(p.to_string(), "4.0");
    /// assert_eq!(o, Greater);
    ///
    /// let (p, o) = Float::power_of_10_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(3u8, 5),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(p.to_string(), "3.981071");
    /// assert_eq!(o, Less);
    ///
    /// let (p, o) = Float::power_of_10_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(3u8, 5),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(p.to_string(), "3.981075");
    /// assert_eq!(o, Greater);
    /// ```
    pub fn power_of_10_rational_prec_round_ref(
        x: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        Self::unsigned_pow_rational_prec_round_ref(10, x, prec, rm)
    }

    #[allow(clippy::needless_pass_by_value)]
    /// Computes $10^x$, where $x$ is a [`Rational`], rounding the result to the nearest value of
    /// the specified precision and returning the result as a [`Float`]. The [`Rational`] is taken
    /// by value. An [`Ordering`] is also returned, indicating whether the rounded power is less
    /// than, equal to, or greater than the exact power.
    ///
    /// If the power is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = 10^x+\varepsilon,
    /// $$
    /// where $|\varepsilon| \leq 2^{\lfloor\log_2 10^x\rfloor-p}$ (unless the result overflows or
    /// underflows; see below).
    ///
    /// The output has precision `prec`.
    ///
    /// Special cases:
    /// - $f(0,p)=1$.
    ///
    /// Overflow and underflow:
    /// - If $f(x,p)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::power_of_10_rational_prec_round`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (p, o) = Float::power_of_10_rational_prec(Rational::from_unsigneds(3u8, 5), 5);
    /// assert_eq!(p.to_string(), "4.0");
    /// assert_eq!(o, Greater);
    ///
    /// let (p, o) = Float::power_of_10_rational_prec(Rational::from_unsigneds(3u8, 5), 20);
    /// assert_eq!(p.to_string(), "3.981071");
    /// assert_eq!(o, Less);
    ///
    /// let (p, o) = Float::power_of_10_rational_prec(Rational::from(0), 10);
    /// assert_eq!(p.to_string(), "1.0");
    /// assert_eq!(o, Equal);
    /// ```
    #[inline]
    pub fn power_of_10_rational_prec(x: Rational, prec: u64) -> (Self, Ordering) {
        Self::power_of_10_rational_prec_round(x, prec, Nearest)
    }

    /// Computes $10^x$, where $x$ is a [`Rational`], rounding the result to the nearest value of
    /// the specified precision and returning the result as a [`Float`]. The [`Rational`] is taken
    /// by reference. An [`Ordering`] is also returned, indicating whether the rounded power is less
    /// than, equal to, or greater than the exact power.
    ///
    /// If the power is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = 10^x+\varepsilon,
    /// $$
    /// where $|\varepsilon| \leq 2^{\lfloor\log_2 10^x\rfloor-p}$ (unless the result overflows or
    /// underflows; see below).
    ///
    /// The output has precision `prec`.
    ///
    /// Special cases:
    /// - $f(0,p)=1$.
    ///
    /// Overflow and underflow:
    /// - If $f(x,p)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::power_of_10_rational_prec_round_ref`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (p, o) = Float::power_of_10_rational_prec_ref(&Rational::from_unsigneds(3u8, 5), 5);
    /// assert_eq!(p.to_string(), "4.0");
    /// assert_eq!(o, Greater);
    ///
    /// let (p, o) = Float::power_of_10_rational_prec_ref(&Rational::from_unsigneds(3u8, 5), 20);
    /// assert_eq!(p.to_string(), "3.981071");
    /// assert_eq!(o, Less);
    ///
    /// let (p, o) = Float::power_of_10_rational_prec_ref(&Rational::from(0), 10);
    /// assert_eq!(p.to_string(), "1.0");
    /// assert_eq!(o, Equal);
    /// ```
    #[inline]
    pub fn power_of_10_rational_prec_ref(x: &Rational, prec: u64) -> (Self, Ordering) {
        Self::power_of_10_rational_prec_round_ref(x, prec, Nearest)
    }
}

impl PowerOf10<Self> for Float {
    /// Computes $10^x$, where $x$ is a [`Float`], taking it by value.
    ///
    /// If the output has a precision, it is the precision of the input. If the power is equidistant
    /// from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s in its binary
    /// expansion is chosen. See [`RoundingMode`] for a description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x) = 10^x+\varepsilon.
    /// $$
    /// - If $10^x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $10^x$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2 10^x\rfloor-p}$,
    ///   where $p$ is the precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN})=\text{NaN}$
    /// - $f(\infty)=\infty$
    /// - $f(-\infty)=0.0$
    /// - $f(\pm0.0)=1.0$
    ///
    /// See the [`Float::power_of_10_of_float_round`] documentation for information on overflow and
    /// underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::power_of_10_of_float_round`] instead. If you want to specify the output precision,
    /// consider using [`Float::power_of_10_of_float_prec`]. If you want both of these things,
    /// consider using [`Float::power_of_10_of_float_prec_round`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::PowerOf10;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let x = Float::from_rational_prec(Rational::from_signeds(1, 2), 20).0;
    ///
    /// assert_eq!(Float::power_of_10(x).to_string(), "3.162277");
    /// ```
    #[inline]
    fn power_of_10(pow: Self) -> Self {
        Self::power_of_10_of_float_round(pow, Nearest).0
    }
}

impl PowerOf10<&Self> for Float {
    /// Computes $10^x$, where $x$ is a [`Float`], taking it by reference.
    ///
    /// If the output has a precision, it is the precision of the input. If the power is equidistant
    /// from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s in its binary
    /// expansion is chosen. See [`RoundingMode`] for a description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x) = 10^x+\varepsilon.
    /// $$
    /// - If $10^x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $10^x$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2 10^x\rfloor-p}$,
    ///   where $p$ is the precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN})=\text{NaN}$
    /// - $f(\infty)=\infty$
    /// - $f(-\infty)=0.0$
    /// - $f(\pm0.0)=1.0$
    ///
    /// See the [`Float::power_of_10_of_float_round`] documentation for information on overflow and
    /// underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::power_of_10_of_float_round_ref`] instead. If you want to specify the output
    /// precision, consider using [`Float::power_of_10_of_float_prec_ref`]. If you want both of
    /// these things, consider using [`Float::power_of_10_of_float_prec_round_ref`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::PowerOf10;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let x = Float::from_rational_prec(Rational::from_signeds(1, 2), 20).0;
    ///
    /// assert_eq!(Float::power_of_10(&x).to_string(), "3.162277");
    /// ```
    #[inline]
    fn power_of_10(pow: &Self) -> Self {
        Self::power_of_10_of_float_round_ref(pow, Nearest).0
    }
}

impl PowerOf10Assign for Float {
    /// Computes $10^x$, where $x$ is a [`Float`], in place.
    ///
    /// If the output has a precision, it is the precision of the input. If the power is equidistant
    /// from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s in its binary
    /// expansion is chosen. See [`RoundingMode`] for a description of the `Nearest` rounding mode.
    ///
    /// $$
    /// x \gets 10^x+\varepsilon.
    /// $$
    /// - If $10^x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $10^x$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2 10^x\rfloor-p}$,
    ///   where $p$ is the precision of the input.
    ///
    /// See the [`Float::power_of_10_of_float_round`] documentation for information on special
    /// cases, overflow, and underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::power_of_10_of_float_round_assign`] instead. If you want to specify the output
    /// precision, consider using [`Float::power_of_10_of_float_prec_assign`]. If you want both of
    /// these things, consider using [`Float::power_of_10_of_float_prec_round_assign`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::PowerOf10Assign;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let mut x = Float::from_rational_prec(Rational::from_signeds(1, 2), 20).0;
    /// x.power_of_10_assign();
    /// assert_eq!(x.to_string(), "3.162277");
    /// ```
    #[inline]
    fn power_of_10_assign(&mut self) {
        self.power_of_10_of_float_round_assign(Nearest);
    }
}

// This is equivalent to `mpfr_exp10` from `exp10.c`, MPFR 4.3.0, which likewise delegates to
// `mpfr_ui_pow`.

/// Computes $10^x$, where $x$ is a primitive float, returning the result as a primitive float of
/// the same type. Using this function is more accurate than using `x.exp2()` or the `exp2` function
/// provided by `libm`.
///
/// $$
/// f(x) = 10^x+\varepsilon.
/// $$
/// - If $10^x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
/// - If $10^x$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2 10^x\rfloor-p}$, where
///   $p$ is the precision of the output (typically 24 if `T` is a [`f32`] and 53 if `T` is a
///   [`f64`], but less if the output is subnormal).
///
/// Special cases:
/// - $f(\text{NaN})=\text{NaN}$
/// - $f(\infty)=\infty$
/// - $f(-\infty)=0.0$
/// - $f(\pm0.0)=1.0$
///
/// Overflow and underflow are possible: a large positive `x` gives $\infty$, and a large negative
/// `x` gives `0.0`.
///
/// # Worst-case complexity
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use malachite_base::num::float::NiceFloat;
/// use malachite_float::arithmetic::power_of_10::primitive_float_power_of_10;
///
/// assert_eq!(
///     NiceFloat(primitive_float_power_of_10(0.5f64)),
///     NiceFloat(3.1622776601683795)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_power_of_10(-3.0f64)),
///     NiceFloat(0.001)
/// );
/// ```
#[inline]
#[allow(clippy::type_repetition_in_bounds)]
pub fn primitive_float_power_of_10<T: PrimitiveFloat>(x: T) -> T
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    emulate_float_to_float_fn(|x2, prec| Float::unsigned_pow_prec(10, x2, prec), x)
}

/// Computes $10^x$, where $x$ is a [`Rational`], returning the result as a primitive float.
///
/// $$
/// f(x) = 10^x+\varepsilon.
/// $$
/// - If $10^x$ is infinite or zero, $\varepsilon$ may be ignored or assumed to be 0.
/// - If $10^x$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2 10^x\rfloor-p}$, where
///   $p$ is the precision of the output (typically 24 if `T` is a [`f32`] and 53 if `T` is a
///   [`f64`], but less if the output is subnormal).
///
/// Special cases:
/// - $f(0)=1$
///
/// Overflow and underflow are possible: a large positive `x` gives $\infty$, and a large negative
/// `x` gives `0.0`.
///
/// # Worst-case complexity
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use malachite_base::num::float::NiceFloat;
/// use malachite_float::arithmetic::power_of_10::primitive_float_power_of_10_rational;
/// use malachite_q::Rational;
///
/// assert_eq!(
///     NiceFloat(primitive_float_power_of_10_rational::<f32>(
///         &Rational::from_signeds(1, 3)
///     )),
///     NiceFloat(2.1544347)
/// );
/// ```
#[inline]
#[allow(clippy::type_repetition_in_bounds)]
pub fn primitive_float_power_of_10_rational<T: PrimitiveFloat>(x: &Rational) -> T
where
    Float: PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    emulate_rational_to_float_fn(
        |q, prec| Float::unsigned_pow_rational_prec_ref(10, q, prec),
        x,
    )
}
