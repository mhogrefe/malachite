// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::InnerFloat::{Finite, Infinity, NaN, Zero};
use crate::{
    Float, float_either_zero, float_infinity, float_nan, float_negative_infinity,
    float_negative_zero, float_zero,
};
use core::cmp::Ordering::{self, *};
use malachite_base::num::arithmetic::traits::{Sqrt, SqrtAssign};
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_nz::natural::arithmetic::float_sqrt::{
    sqrt_float_significand_in_place, sqrt_float_significand_ref,
};

impl Float {
    /// Computes the square root of a [`Float`], rounding the result to the specified precision and
    /// with the specified rounding mode. The [`Float`] is taken by value. An [`Ordering`] is also
    /// returned, indicating whether the rounded square root is less than, equal to, or greater than
    /// the exact square root. Although `NaN`s are not comparable to any [`Float`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// The square root of any nonzero negative number is `NaN`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \sqrt{x}+\varepsilon.
    /// $$
    /// - If $\sqrt{x}$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\sqrt{x}$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\sqrt{x}|\rfloor-p+1}$.
    /// - If $\sqrt{x}$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\sqrt{x}|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=\text{NaN}$
    /// - $f(\infty,p,m)=\infty$
    /// - $f(-\infty,p,m)=\text{NaN}$
    /// - $f(0.0,p,m)=0.0$
    /// - $f(-0.0,p,m)=-0.0$
    ///
    /// Neither overflow nor underflow is possible.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::sqrt_prec`] instead. If you
    /// know that your target precision is the precision of the input, consider using
    /// [`Float::sqrt_round`] instead. If both of these things are true, consider using
    /// [`Float::sqrt`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the result cannot be represented exactly with the given
    /// precision.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (sqrt, o) = Float::from(PI).sqrt_prec_round(5, Floor);
    /// assert_eq!(sqrt.to_string(), "1.75");
    /// assert_eq!(o, Less);
    ///
    /// let (sqrt, o) = Float::from(PI).sqrt_prec_round(5, Ceiling);
    /// assert_eq!(sqrt.to_string(), "1.81");
    /// assert_eq!(o, Greater);
    ///
    /// let (sqrt, o) = Float::from(PI).sqrt_prec_round(5, Nearest);
    /// assert_eq!(sqrt.to_string(), "1.75");
    /// assert_eq!(o, Less);
    ///
    /// let (sqrt, o) = Float::from(PI).sqrt_prec_round(20, Floor);
    /// assert_eq!(sqrt.to_string(), "1.772453");
    /// assert_eq!(o, Less);
    ///
    /// let (sqrt, o) = Float::from(PI).sqrt_prec_round(20, Ceiling);
    /// assert_eq!(sqrt.to_string(), "1.772455");
    /// assert_eq!(o, Greater);
    ///
    /// let (sqrt, o) = Float::from(PI).sqrt_prec_round(20, Nearest);
    /// assert_eq!(sqrt.to_string(), "1.772453");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn sqrt_prec_round(mut self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        let o = self.sqrt_prec_round_assign(prec, rm);
        (self, o)
    }

    /// Computes the square root of a [`Float`], rounding the result to the specified precision and
    /// with the specified rounding mode. The [`Float`] is taken by reference. An [`Ordering`] is
    /// also returned, indicating whether the rounded square root is less than, equal to, or greater
    /// than the exact square root. Although `NaN`s are not comparable to any [`Float`], whenever
    /// this function returns a `NaN` it also returns `Equal`.
    ///
    /// The square root of any nonzero negative number is `NaN`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \sqrt{x}+\varepsilon.
    /// $$
    /// - If $\sqrt{x}$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\sqrt{x}$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\sqrt{x}|\rfloor-p+1}$.
    /// - If $\sqrt{x}$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\sqrt{x}|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=\text{NaN}$
    /// - $f(\infty,p,m)=\infty$
    /// - $f(-\infty,p,m)=\text{NaN}$
    /// - $f(0.0,p,m)=0.0$
    /// - $f(-0.0,p,m)=-0.0$
    ///
    /// Neither overflow nor underflow is possible.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::sqrt_prec_ref`] instead. If
    /// you know that your target precision is the precision of the input, consider using
    /// [`Float::sqrt_round_ref`] instead. If both of these things are true, consider using
    /// `(&Float).sqrt()`instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the result cannot be represented exactly with the given
    /// precision.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (sqrt, o) = Float::from(PI).sqrt_prec_round_ref(5, Floor);
    /// assert_eq!(sqrt.to_string(), "1.75");
    /// assert_eq!(o, Less);
    ///
    /// let (sqrt, o) = Float::from(PI).sqrt_prec_round_ref(5, Ceiling);
    /// assert_eq!(sqrt.to_string(), "1.81");
    /// assert_eq!(o, Greater);
    ///
    /// let (sqrt, o) = Float::from(PI).sqrt_prec_round_ref(5, Nearest);
    /// assert_eq!(sqrt.to_string(), "1.75");
    /// assert_eq!(o, Less);
    ///
    /// let (sqrt, o) = Float::from(PI).sqrt_prec_round_ref(20, Floor);
    /// assert_eq!(sqrt.to_string(), "1.772453");
    /// assert_eq!(o, Less);
    ///
    /// let (sqrt, o) = Float::from(PI).sqrt_prec_round_ref(20, Ceiling);
    /// assert_eq!(sqrt.to_string(), "1.772455");
    /// assert_eq!(o, Greater);
    ///
    /// let (sqrt, o) = Float::from(PI).sqrt_prec_round_ref(20, Nearest);
    /// assert_eq!(sqrt.to_string(), "1.772453");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn sqrt_prec_round_ref(&self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        assert_ne!(prec, 0);
        match self {
            float_nan!() => (float_nan!(), Equal),
            float_infinity!() => (float_infinity!(), Equal),
            float_negative_infinity!() => (float_nan!(), Equal),
            float_zero!() => (float_zero!(), Equal),
            float_negative_zero!() => (float_negative_zero!(), Equal),
            Self(Finite {
                sign,
                exponent: x_exp,
                precision: x_prec,
                significand: x,
                ..
            }) => {
                if !sign {
                    return (float_nan!(), Equal);
                }
                let (sqrt, exp, o) = sqrt_float_significand_ref(x, *x_exp, *x_prec, prec, rm);
                (
                    Self(Finite {
                        sign: true,
                        exponent: exp,
                        precision: prec,
                        significand: sqrt,
                    }),
                    o,
                )
            }
        }
    }

    /// Computes the square root of a [`Float`], rounding the result to the nearest value of the
    /// specified precision. The [`Float`] is taken by value. An [`Ordering`] is also returned,
    /// indicating whether the rounded sqrt is less than, equal to, or greater than the exact square
    /// root. Although `NaN`s are not comparable to any [`Float`], whenever this function returns a
    /// `NaN` it also returns `Equal`.
    ///
    /// The square root of any nonzero negative number is `NaN`.
    ///
    /// If the square root is equidistant from two [`Float`]s with the specified precision, the
    /// [`Float`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a
    /// description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = \sqrt{x}+\varepsilon.
    /// $$
    /// - If $\sqrt{x}$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\sqrt{x}$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2
    ///   |\sqrt{x}|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p)=\text{NaN}$
    /// - $f(\infty,p)=\infty$
    /// - $f(-\infty,p)=\text{NaN}$
    /// - $f(0.0,p)=0.0$
    /// - $f(-0.0,p)=-0.0$
    ///
    /// Neither overflow nor underflow is possible.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::sqrt_prec_round`] instead. If you know that your target precision is the precision
    /// of the input, consider using [`Float::sqrt`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (sqrt, o) = Float::from(PI).sqrt_prec(5);
    /// assert_eq!(sqrt.to_string(), "1.75");
    /// assert_eq!(o, Less);
    ///
    /// let (sqrt, o) = Float::from(PI).sqrt_prec(20);
    /// assert_eq!(sqrt.to_string(), "1.772453");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn sqrt_prec(self, prec: u64) -> (Self, Ordering) {
        self.sqrt_prec_round(prec, Nearest)
    }

    /// Computes the square root of a [`Float`], rounding the result to the nearest value of the
    /// specified precision. The [`Float`] is taken by reference. An [`Ordering`] is also returned,
    /// indicating whether the rounded sqrt is less than, equal to, or greater than the exact square
    /// root. Although `NaN`s are not comparable to any [`Float`], whenever this function returns a
    /// `NaN` it also returns `Equal`.
    ///
    /// The square root of any nonzero negative number is `NaN`.
    ///
    /// If the square root is equidistant from two [`Float`]s with the specified precision, the
    /// [`Float`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a
    /// description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = \sqrt{x}+\varepsilon.
    /// $$
    /// - If $\sqrt{x}$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\sqrt{x}$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2
    ///   |\sqrt{x}|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p)=\text{NaN}$
    /// - $f(\infty,p)=\infty$
    /// - $f(-\infty,p)=\text{NaN}$
    /// - $f(0.0,p)=0.0$
    /// - $f(-0.0,p)=-0.0$
    ///
    /// Neither overflow nor underflow is possible.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::sqrt_prec_round_ref`] instead. If you know that your target precision is the
    /// precision of the input, consider using `(&Float).sqrt()` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (sqrt, o) = Float::from(PI).sqrt_prec_ref(5);
    /// assert_eq!(sqrt.to_string(), "1.75");
    /// assert_eq!(o, Less);
    ///
    /// let (sqrt, o) = Float::from(PI).sqrt_prec_ref(20);
    /// assert_eq!(sqrt.to_string(), "1.772453");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn sqrt_prec_ref(&self, prec: u64) -> (Self, Ordering) {
        self.sqrt_prec_round_ref(prec, Nearest)
    }

    /// Computes the square root of a [`Float`], rounding the result with the specified rounding
    /// mode. The [`Float`] is taken by value. An [`Ordering`] is also returned, indicating whether
    /// the rounded square root is less than, equal to, or greater than the exact square root.
    /// Although `NaN`s are not comparable to any [`Float`], whenever this function returns a `NaN`
    /// it also returns `Equal`.
    ///
    /// The square root of any nonzero negative number is `NaN`.
    ///
    /// The precision of the output is the precision of the input. See [`RoundingMode`] for a
    /// description of the possible rounding modes.
    ///
    /// $$
    /// f(x,m) = \sqrt{x}+\varepsilon.
    /// $$
    /// - If $\sqrt{x}$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\sqrt{x}$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\sqrt{x}|\rfloor-p+1}$, where $p$ is the precision of the input.
    /// - If $\sqrt{x}$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\sqrt{x}|\rfloor-p}$, where $p$ is the precision of the input.
    ///
    /// If the output has a precision, it is the precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN},m)=\text{NaN}$
    /// - $f(\infty,m)=\infty$
    /// - $f(-\infty,m)=\text{NaN}$
    /// - $f(0.0,m)=0.0$
    /// - $f(-0.0,m)=-0.0$
    ///
    /// Neither overflow nor underflow is possible.
    ///
    /// If you want to specify an output precision, consider using [`Float::sqrt_prec_round`]
    /// instead. If you know you'll be using the `Nearest` rounding mode, consider using
    /// [`Float::sqrt`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.get_prec()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the result cannot be represented exactly with the input
    /// precision.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (sqrt, o) = Float::from(PI).sqrt_round(Floor);
    /// assert_eq!(sqrt.to_string(), "1.772453850905515");
    /// assert_eq!(o, Less);
    ///
    /// let (sqrt, o) = Float::from(PI).sqrt_round(Ceiling);
    /// assert_eq!(sqrt.to_string(), "1.772453850905517");
    /// assert_eq!(o, Greater);
    ///
    /// let (sqrt, o) = Float::from(PI).sqrt_round(Nearest);
    /// assert_eq!(sqrt.to_string(), "1.772453850905515");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn sqrt_round(self, rm: RoundingMode) -> (Self, Ordering) {
        let prec = self.significant_bits();
        self.sqrt_prec_round(prec, rm)
    }

    /// Computes the square root of a [`Float`], rounding the result with the specified rounding
    /// mode. The [`Float`] is taken by reference. An [`Ordering`] is also returned, indicating
    /// whether the rounded square root is less than, equal to, or greater than the exact square
    /// root. Although `NaN`s are not comparable to any [`Float`], whenever this function returns a
    /// `NaN` it also returns `Equal`.
    ///
    /// The square root of any nonzero negative number is `NaN`.
    ///
    /// The precision of the output is the precision of the input. See [`RoundingMode`] for a
    /// description of the possible rounding modes.
    ///
    /// $$
    /// f(x,m) = \sqrt{x}+\varepsilon.
    /// $$
    /// - If $\sqrt{x}$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\sqrt{x}$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\sqrt{x}|\rfloor-p+1}$, where $p$ is the precision of the input.
    /// - If $\sqrt{x}$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\sqrt{x}|\rfloor-p}$, where $p$ is the precision of the input.
    ///
    /// If the output has a precision, it is the precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN},m)=\text{NaN}$
    /// - $f(\infty,m)=\infty$
    /// - $f(-\infty,m)=\text{NaN}$
    /// - $f(0.0,m)=0.0$
    /// - $f(-0.0,m)=-0.0$
    ///
    /// Neither overflow nor underflow is possible.
    ///
    /// If you want to specify an output precision, consider using [`Float::sqrt_prec_round_ref`]
    /// instead. If you know you'll be using the `Nearest` rounding mode, consider using
    /// `(&Float).sqrt()` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.get_prec()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the result cannot be represented exactly with the input
    /// precision.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (sqrt, o) = Float::from(PI).sqrt_round_ref(Floor);
    /// assert_eq!(sqrt.to_string(), "1.772453850905515");
    /// assert_eq!(o, Less);
    ///
    /// let (sqrt, o) = Float::from(PI).sqrt_round_ref(Ceiling);
    /// assert_eq!(sqrt.to_string(), "1.772453850905517");
    /// assert_eq!(o, Greater);
    ///
    /// let (sqrt, o) = Float::from(PI).sqrt_round_ref(Nearest);
    /// assert_eq!(sqrt.to_string(), "1.772453850905515");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn sqrt_round_ref(&self, rm: RoundingMode) -> (Self, Ordering) {
        let prec = self.significant_bits();
        self.sqrt_prec_round_ref(prec, rm)
    }

    /// Computes the square root of a [`Float`] in place, rounding the result to the specified
    /// precision and with the specified rounding mode. An [`Ordering`] is returned, indicating
    /// whether the rounded square root is less than, equal to, or greater than the exact square
    /// root. Although `NaN`s are not comparable to any [`Float`], whenever this function sets the
    /// [`Float`] to `NaN` it also returns `Equal`.
    ///
    /// The square root of any nonzero negative number is `NaN`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets \sqrt{x}+\varepsilon.
    /// $$
    /// - If $\sqrt{x}$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\sqrt{x}$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy|\rfloor-p+1}$.
    /// - If $\sqrt{x}$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\sqrt{x}|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// See the [`Float::sqrt_prec_round`] documentation for information on special cases, overflow,
    /// and underflow.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::sqrt_prec_assign`] instead.
    /// If you know that your target precision is the precision of the input, consider using
    /// [`Float::sqrt_round_assign`] instead. If both of these things are true, consider using
    /// [`Float::sqrt_assign`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the result cannot be represented exactly with the given
    /// precision.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.sqrt_prec_round_assign(5, Floor), Less);
    /// assert_eq!(x.to_string(), "1.75");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.sqrt_prec_round_assign(5, Ceiling), Greater);
    /// assert_eq!(x.to_string(), "1.81");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.sqrt_prec_round_assign(5, Nearest), Less);
    /// assert_eq!(x.to_string(), "1.75");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.sqrt_prec_round_assign(20, Floor), Less);
    /// assert_eq!(x.to_string(), "1.772453");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.sqrt_prec_round_assign(20, Ceiling), Greater);
    /// assert_eq!(x.to_string(), "1.772455");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.sqrt_prec_round_assign(20, Nearest), Less);
    /// assert_eq!(x.to_string(), "1.772453");
    /// ```
    #[inline]
    pub fn sqrt_prec_round_assign(&mut self, prec: u64, rm: RoundingMode) -> Ordering {
        assert_ne!(prec, 0);
        match self {
            float_nan!() | float_infinity!() | float_either_zero!() => Equal,
            float_negative_infinity!() => {
                *self = float_nan!();
                Equal
            }
            Self(Finite {
                sign,
                exponent: x_exp,
                precision: x_prec,
                significand: x,
                ..
            }) => {
                if !*sign {
                    *self = float_nan!();
                    return Equal;
                }
                let o;
                (*x_exp, o) = sqrt_float_significand_in_place(x, *x_exp, *x_prec, prec, rm);
                *x_prec = prec;
                o
            }
        }
    }

    /// Computes the square root of a [`Float`] in place, rounding the result to the nearest value
    /// of the specified precision. An [`Ordering`] is returned, indicating whether the rounded
    /// square root is less than, equal to, or greater than the exact square root. Although `NaN`s
    /// are not comparable to any [`Float`], whenever this function sets the [`Float`] to `NaN` it
    /// also returns `Equal`.
    ///
    /// The square root of any nonzero negative number is `NaN`.
    ///
    /// If the square root is equidistant from two [`Float`]s with the specified precision, the
    /// [`Float`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a
    /// description of the `Nearest` rounding mode.
    ///
    /// $$
    /// x \gets \sqrt{x}+\varepsilon.
    /// $$
    /// - If $\sqrt{x}$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\sqrt{x}$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2
    ///   |\sqrt{x}|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// See the [`Float::sqrt_prec`] documentation for information on special cases, overflow, and
    /// underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::sqrt_prec_round_assign`] instead. If you know that your target precision is the
    /// precision of the input, consider using [`Float::sqrt`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.sqrt_prec_assign(5), Less);
    /// assert_eq!(x.to_string(), "1.75");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.sqrt_prec_assign(20), Less);
    /// assert_eq!(x.to_string(), "1.772453");
    /// ```
    #[inline]
    pub fn sqrt_prec_assign(&mut self, prec: u64) -> Ordering {
        self.sqrt_prec_round_assign(prec, Nearest)
    }

    /// Computes the square root of a [`Float`] in place, rounding the result with the specified
    /// rounding mode. An [`Ordering`] is returned, indicating whether the rounded square root is
    /// less than, equal to, or greater than the exact square root. Although `NaN`s are not
    /// comparable to any [`Float`], whenever this function sets the [`Float`] to `NaN` it also
    /// returns `Equal`.
    ///
    /// The square root of any nonzero negative number is `NaN`.
    ///
    /// The precision of the output is the precision of the input. See [`RoundingMode`] for a
    /// description of the possible rounding modes.
    ///
    /// $$
    /// x \gets \sqrt{x}+\varepsilon.
    /// $$
    /// - If $\sqrt{x}$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\sqrt{x}$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\sqrt{x}|\rfloor-p+1}$, where $p$ is the maximum precision of the
    ///   inputs.
    /// - If $\sqrt{x}$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\sqrt{x}|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// If the output has a precision, it is the precision of the input.
    ///
    /// See the [`Float::sqrt_round`] documentation for information on special cases, overflow, and
    /// underflow.
    ///
    /// If you want to specify an output precision, consider using [`Float::sqrt_prec_round_assign`]
    /// instead. If you know you'll be using the `Nearest` rounding mode, consider using
    /// [`Float::sqrt_assign`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.get_prec()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the result cannot be represented exactly with the input
    /// precision.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.sqrt_round_assign(Floor), Less);
    /// assert_eq!(x.to_string(), "1.772453850905515");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.sqrt_round_assign(Ceiling), Greater);
    /// assert_eq!(x.to_string(), "1.772453850905517");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.sqrt_round_assign(Nearest), Less);
    /// assert_eq!(x.to_string(), "1.772453850905515");
    /// ```
    #[inline]
    pub fn sqrt_round_assign(&mut self, rm: RoundingMode) -> Ordering {
        let prec = self.significant_bits();
        self.sqrt_prec_round_assign(prec, rm)
    }
}

impl Sqrt for Float {
    type Output = Self;

    /// Computes the square root of a [`Float`], taking it by value.
    ///
    /// If the output has a precision, it is the precision of the input. If the square root is
    /// equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s in
    /// its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// The square root of any nonzero negative number is `NaN`.
    ///
    /// $$
    /// f(x) = \sqrt{x}+\varepsilon.
    /// $$
    /// - If $\sqrt{x}$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\sqrt{x}$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2
    ///   |\sqrt{x}|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// Special cases:
    /// - $f(\text{NaN})=\text{NaN}$
    /// - $f(\infty)=\infty$
    /// - $f(-\infty)=\text{NaN}$
    /// - $f(0.0)=0.0$
    /// - $f(-0.0)=-0.0$
    ///
    /// Neither overflow nor underflow is possible.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using [`Float::sqrt_prec`]
    /// instead. If you want to specify the output precision, consider using [`Float::sqrt_round`].
    /// If you want both of these things, consider using [`Float::sqrt_prec_round`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.get_prec()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Sqrt;
    /// use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity};
    /// use malachite_float::Float;
    ///
    /// assert!(Float::NAN.sqrt().is_nan());
    /// assert_eq!(Float::INFINITY.sqrt(), Float::INFINITY);
    /// assert!(Float::NEGATIVE_INFINITY.sqrt().is_nan());
    /// assert_eq!(Float::from(1.5).sqrt(), 1.0);
    /// assert!(Float::from(-1.5).sqrt().is_nan());
    /// ```
    #[inline]
    fn sqrt(self) -> Self {
        let prec = self.significant_bits();
        self.sqrt_prec_round(prec, Nearest).0
    }
}

impl Sqrt for &Float {
    type Output = Float;

    /// Computes the square root of a [`Float`], taking it by reference.
    ///
    /// If the output has a precision, it is the precision of the input. If the square root is
    /// equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s in
    /// its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// The square root of any nonzero negative number is `NaN`.
    ///
    /// $$
    /// f(x) = \sqrt{x}+\varepsilon.
    /// $$
    /// - If $\sqrt{x}$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\sqrt{x}$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2
    ///   |\sqrt{x}|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// Special cases:
    /// - $f(\text{NaN})=\text{NaN}$
    /// - $f(\infty)=\infty$
    /// - $f(-\infty)=\text{NaN}$
    /// - $f(0.0)=0.0$
    /// - $f(-0.0)=-0.0$
    ///
    /// Neither overflow nor underflow is possible.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::sqrt_prec_ref`] instead. If you want to specify the output precision, consider
    /// using [`Float::sqrt_round_ref`]. If you want both of these things, consider using
    /// [`Float::sqrt_prec_round_ref`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.get_prec()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Sqrt;
    /// use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity};
    /// use malachite_float::Float;
    ///
    /// assert!((&Float::NAN).sqrt().is_nan());
    /// assert_eq!((&Float::INFINITY).sqrt(), Float::INFINITY);
    /// assert!((&Float::NEGATIVE_INFINITY).sqrt().is_nan());
    /// assert_eq!((&Float::from(1.5)).sqrt(), 1.0);
    /// assert!((&Float::from(-1.5)).sqrt().is_nan());
    /// ```
    #[inline]
    fn sqrt(self) -> Float {
        let prec = self.significant_bits();
        self.sqrt_prec_round_ref(prec, Nearest).0
    }
}

impl SqrtAssign for Float {
    /// Computes the square root of a [`Float`] in place.
    ///
    /// If the output has a precision, it is the precision of the input. If the square root is
    /// equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s in
    /// its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// The square root of any nonzero negative number is `NaN`.
    ///
    /// $$
    /// x\gets = \sqrt{x}+\varepsilon.
    /// $$
    /// - If $\sqrt{x}$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\sqrt{x}$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2
    ///   |\sqrt{x}|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// See the [`Float::sqrt`] documentation for information on special cases, overflow, and
    /// underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::sqrt_prec_assign`] instead. If you want to specify the output precision, consider
    /// using [`Float::sqrt_round_assign`]. If you want both of these things, consider using
    /// [`Float::sqrt_prec_round_assign`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.get_prec()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::SqrtAssign;
    /// use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity};
    /// use malachite_float::Float;
    ///
    /// let mut x = Float::NAN;
    /// x.sqrt_assign();
    /// assert!(x.is_nan());
    ///
    /// let mut x = Float::INFINITY;
    /// x.sqrt_assign();
    /// assert_eq!(x, Float::INFINITY);
    ///
    /// let mut x = Float::NEGATIVE_INFINITY;
    /// x.sqrt_assign();
    /// assert!(x.is_nan());
    ///
    /// let mut x = Float::from(1.5);
    /// x.sqrt_assign();
    /// assert_eq!(x, 1.0);
    ///
    /// let mut x = Float::from(-1.5);
    /// x.sqrt_assign();
    /// assert!(x.is_nan());
    /// ```
    #[inline]
    fn sqrt_assign(&mut self) {
        let prec = self.significant_bits();
        self.sqrt_prec_round_assign(prec, Nearest);
    }
}
