// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::InnerFloat::{Finite, Infinity, NaN, Zero};
use crate::{float_nan, Float};
use core::cmp::Ordering::{self, *};
use malachite_base::num::arithmetic::traits::{
    IsPowerOf2, NegAssign, Reciprocal, ReciprocalAssign,
};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_nz::natural::arithmetic::float_reciprocal::reciprocal_float_significand_ref;

impl Float {
    /// Takes the reciprocal of a [`Float`], rounding the result to the specified precision and with
    /// the specified rounding mode. The [`Float`] is taken by value. An [`Ordering`] is also
    /// returned, indicating whether the rounded reciprocal is less than, equal to, or greater than
    /// the exact reciprocal. Although `NaN`s are not comparable to any [`Float`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = 1/x+\varepsilon.
    /// $$
    /// - If $1/x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $1/x$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |1/x|\rfloor-p+1}$.
    /// - If $1/x$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |1/x|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=\text{NaN}$
    /// - $f(\infty,p,m)=0.0$
    /// - $f(-\infty,p,m)=-0.0$
    /// - $f(0.0,p,m)=\infty$
    /// - $f(-0.0,p,m)=-\infty$
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::reciprocal_prec`] instead. If
    /// you know that your target precision is the precision of the input, consider using
    /// [`Float::reciprocal_round`] instead. If both of these things are true, consider using
    /// [`Float::reciprocal`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// prec)`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but `prec` is too small for an exact reciprocation.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (reciprocal, o) = Float::from(PI).reciprocal_prec_round(5, Floor);
    /// assert_eq!(reciprocal.to_string(), "0.31");
    /// assert_eq!(o, Less);
    ///
    /// let (reciprocal, o) = Float::from(PI).reciprocal_prec_round(5, Ceiling);
    /// assert_eq!(reciprocal.to_string(), "0.33");
    /// assert_eq!(o, Greater);
    ///
    /// let (reciprocal, o) = Float::from(PI).reciprocal_prec_round(5, Nearest);
    /// assert_eq!(reciprocal.to_string(), "0.31");
    /// assert_eq!(o, Less);
    ///
    /// let (reciprocal, o) = Float::from(PI).reciprocal_prec_round(20, Floor);
    /// assert_eq!(reciprocal.to_string(), "0.3183098");
    /// assert_eq!(o, Less);
    ///
    /// let (reciprocal, o) = Float::from(PI).reciprocal_prec_round(20, Ceiling);
    /// assert_eq!(reciprocal.to_string(), "0.3183103");
    /// assert_eq!(o, Greater);
    ///
    /// let (reciprocal, o) = Float::from(PI).reciprocal_prec_round(20, Nearest);
    /// assert_eq!(reciprocal.to_string(), "0.3183098");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn reciprocal_prec_round(mut self, prec: u64, rm: RoundingMode) -> (Float, Ordering) {
        let o = self.reciprocal_prec_round_assign(prec, rm);
        (self, o)
    }

    /// Takes the reciprocal of a [`Float`], rounding the result to the specified precision and with
    /// the specified rounding mode. The [`Float`] is taken by reference. An [`Ordering`] is also
    /// returned, indicating whether the rounded reciprocal is less than, equal to, or greater than
    /// the exact reciprocal. Although `NaN`s are not comparable to any [`Float`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = 1/x+\varepsilon.
    /// $$
    /// - If $1/x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $1/x$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |1/x|\rfloor-p+1}$.
    /// - If $1/x$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |1/x|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=\text{NaN}$
    /// - $f(\infty,p,m)=0.0$
    /// - $f(-\infty,p,m)=-0.0$
    /// - $f(0.0,p,m)=\infty$
    /// - $f(-0.0,p,m)=-\infty$
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::reciprocal_prec_ref`]
    /// instead. If you know that your target precision is the precision of the input, consider
    /// using [`Float::reciprocal_round_ref`] instead. If both of these things are true, consider
    /// using `(&Float)::reciprocal()` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// prec)`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but `prec` is too small for an exact reciprocation.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (reciprocal, o) = Float::from(PI).reciprocal_prec_round_ref(5, Floor);
    /// assert_eq!(reciprocal.to_string(), "0.31");
    /// assert_eq!(o, Less);
    ///
    /// let (reciprocal, o) = Float::from(PI).reciprocal_prec_round_ref(5, Ceiling);
    /// assert_eq!(reciprocal.to_string(), "0.33");
    /// assert_eq!(o, Greater);
    ///
    /// let (reciprocal, o) = Float::from(PI).reciprocal_prec_round_ref(5, Nearest);
    /// assert_eq!(reciprocal.to_string(), "0.31");
    /// assert_eq!(o, Less);
    ///
    /// let (reciprocal, o) = Float::from(PI).reciprocal_prec_round_ref(20, Floor);
    /// assert_eq!(reciprocal.to_string(), "0.3183098");
    /// assert_eq!(o, Less);
    ///
    /// let (reciprocal, o) = Float::from(PI).reciprocal_prec_round_ref(20, Ceiling);
    /// assert_eq!(reciprocal.to_string(), "0.3183103");
    /// assert_eq!(o, Greater);
    ///
    /// let (reciprocal, o) = Float::from(PI).reciprocal_prec_round_ref(20, Nearest);
    /// assert_eq!(reciprocal.to_string(), "0.3183098");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn reciprocal_prec_round_ref(&self, prec: u64, rm: RoundingMode) -> (Float, Ordering) {
        assert_ne!(prec, 0);
        match self {
            float_nan!() => (float_nan!(), Equal),
            Float(Zero { sign }) => (Float(Infinity { sign: *sign }), Equal),
            Float(Infinity { sign }) => (Float(Zero { sign: *sign }), Equal),
            Float(Finite {
                sign,
                exponent: exp,
                precision: x_prec,
                significand: x,
            }) => {
                if x.is_power_of_2() {
                    let (reciprocal, o) = Float::power_of_2_prec(i64::from(1 - exp), prec);
                    return if *sign {
                        (reciprocal, o)
                    } else {
                        (-reciprocal, o.reverse())
                    };
                }
                let sign = *sign;
                let (reciprocal, exp_offset, o) =
                    reciprocal_float_significand_ref(x, *x_prec, prec, if sign { rm } else { -rm });
                let exp = 1i32
                    .checked_sub(*exp)
                    .unwrap()
                    .checked_add(i32::exact_from(exp_offset))
                    .unwrap();
                (
                    Float(Finite {
                        sign,
                        exponent: exp,
                        precision: prec,
                        significand: reciprocal,
                    }),
                    if sign { o } else { o.reverse() },
                )
            }
        }
    }

    /// Takes the reciprocal of a [`Float`], rounding the result to the nearest value of the
    /// specified precision. The [`Float`] is taken by value. An [`Ordering`] is also returned,
    /// indicating whether the rounded reciprocal is less than, equal to, or greater than the exact
    /// reciprocal. Although `NaN`s are not comparable to any [`Float`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// If the reciprocal is equidistant from two [`Float`]s with the specified precision, the
    /// [`Float`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a
    /// description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = 1/x+\varepsilon.
    /// $$
    /// - If $1/x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $1/x$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2 |1/x|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p)=\text{NaN}$
    /// - $f(\infty,p)=0.0$
    /// - $f(-\infty,p)=-0.0$
    /// - $f(0.0,p)=\infty$
    /// - $f(-0.0,p)=-\infty$
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::reciprocal_prec_round`] instead. If you know that your target precision is the
    /// precision of the input, consider using [`Float::reciprocal`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// prec)`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (reciprocal, o) = Float::from(PI).reciprocal_prec(5);
    /// assert_eq!(reciprocal.to_string(), "0.31");
    /// assert_eq!(o, Less);
    ///
    /// let (reciprocal, o) = Float::from(PI).reciprocal_prec(20);
    /// assert_eq!(reciprocal.to_string(), "0.3183098");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn reciprocal_prec(self, prec: u64) -> (Float, Ordering) {
        self.reciprocal_prec_round(prec, Nearest)
    }

    /// Takes the reciprocal of a [`Float`], rounding the result to the nearest value of the
    /// specified precision. The [`Float`] is taken by reference. An [`Ordering`] is also returned,
    /// indicating whether the rounded reciprocal is less than, equal to, or greater than the exact
    /// reciprocal. Although `NaN`s are not comparable to any [`Float`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// If the reciprocal is equidistant from two [`Float`]s with the specified precision, the
    /// [`Float`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a
    /// description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = 1/x+\varepsilon.
    /// $$
    /// - If $1/x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $1/x$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2 |1/x|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p)=\text{NaN}$
    /// - $f(\infty,p)=0.0$
    /// - $f(-\infty,p)=-0.0$
    /// - $f(0.0,p)=\infty$
    /// - $f(-0.0,p)=-\infty$
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::reciprocal_prec_round_ref`] instead. If you know that your target precision is the
    /// precision of the input, consider using `(&Float)::reciprocal()` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// prec)`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (reciprocal, o) = Float::from(PI).reciprocal_prec_ref(5);
    /// assert_eq!(reciprocal.to_string(), "0.31");
    /// assert_eq!(o, Less);
    ///
    /// let (reciprocal, o) = Float::from(PI).reciprocal_prec_ref(20);
    /// assert_eq!(reciprocal.to_string(), "0.3183098");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn reciprocal_prec_ref(&self, prec: u64) -> (Float, Ordering) {
        self.reciprocal_prec_round_ref(prec, Nearest)
    }

    /// Takes the reciprocal of a [`Float`], rounding the result with the specified rounding mode.
    /// The [`Float`] is taken by value. An [`Ordering`] is also returned, indicating whether the
    /// rounded reciprocal is less than, equal to, or greater than the exact reciprocal. Although
    /// `NaN`s are not comparable to any [`Float`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// The precision of the output is the precision of the input. See [`RoundingMode`] for a
    /// description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,m) = 1/x+\varepsilon.
    /// $$
    /// - If $1/x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $1/x$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |1/x|\rfloor-p+1}$, where $p$ is the precision of the input.
    /// - If $1/x$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |1/x|\rfloor-p}$, where $p$ is the precision of the input.
    ///
    /// If the output has a precision, it is the precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN},m)=\text{NaN}$
    /// - $f(\infty,m)=0.0$
    /// - $f(-\infty,m)=-0.0$
    /// - $f(0.0,m)=\infty$
    /// - $f(-0.0,m)=-\infty$
    ///
    /// If you want to specify an output precision, consider using [`Float::reciprocal_prec_round`]
    /// instead. If you know you'll be using the `Nearest` rounding mode, consider using
    /// [`Float::reciprocal`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the precision of the input is not high enough to represent the
    /// output.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (reciprocal, o) = Float::from(PI).reciprocal_round(Floor);
    /// assert_eq!(reciprocal.to_string(), "0.3183098861837905");
    /// assert_eq!(o, Less);
    ///
    /// let (reciprocal, o) = Float::from(PI).reciprocal_round(Ceiling);
    /// assert_eq!(reciprocal.to_string(), "0.318309886183791");
    /// assert_eq!(o, Greater);
    ///
    /// let (reciprocal, o) = Float::from(PI).reciprocal_round(Nearest);
    /// assert_eq!(reciprocal.to_string(), "0.3183098861837905");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn reciprocal_round(self, rm: RoundingMode) -> (Float, Ordering) {
        let prec = self.significant_bits();
        self.reciprocal_prec_round(prec, rm)
    }

    /// Takes the reciprocal of a [`Float`], rounding the result with the specified rounding mode.
    /// The [`Float`] is taken by reference. An [`Ordering`] is also returned, indicating whether
    /// the rounded reciprocal is less than, equal to, or greater than the exact reciprocal.
    /// Although `NaN`s are not comparable to any [`Float`], whenever this function returns a `NaN`
    /// it also returns `Equal`.
    ///
    /// The precision of the output is the precision of the input. See [`RoundingMode`] for a
    /// description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,m) = 1/x+\varepsilon.
    /// $$
    /// - If $1/x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $1/x$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |1/x|\rfloor-p+1}$, where $p$ is the precision of the input.
    /// - If $1/x$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |1/x|\rfloor-p}$, where $p$ is the precision of the input.
    ///
    /// If the output has a precision, it is the precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN},m)=\text{NaN}$
    /// - $f(\infty,m)=0.0$
    /// - $f(-\infty,m)=-0.0$
    /// - $f(0.0,m)=\infty$
    /// - $f(-0.0,m)=-\infty$
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::reciprocal_prec_round_ref`] instead. If you know you'll be using the `Nearest`
    /// rounding mode, consider using `(&Float)::reciprocal()` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the precision of the input is not high enough to represent the
    /// output.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (reciprocal, o) = Float::from(PI).reciprocal_round_ref(Floor);
    /// assert_eq!(reciprocal.to_string(), "0.3183098861837905");
    /// assert_eq!(o, Less);
    ///
    /// let (reciprocal, o) = Float::from(PI).reciprocal_round_ref(Ceiling);
    /// assert_eq!(reciprocal.to_string(), "0.318309886183791");
    /// assert_eq!(o, Greater);
    ///
    /// let (reciprocal, o) = Float::from(PI).reciprocal_round_ref(Nearest);
    /// assert_eq!(reciprocal.to_string(), "0.3183098861837905");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn reciprocal_round_ref(&self, rm: RoundingMode) -> (Float, Ordering) {
        let prec = self.significant_bits();
        self.reciprocal_prec_round_ref(prec, rm)
    }

    /// Takes the reciprocal of a [`Float`] in place, rounding the result to the specified precision
    /// and with the specified rounding mode. An [`Ordering`] is returned, indicating whether the
    /// rounded reciprocal is less than, equal to, or greater than the exact reciprocal. Although
    /// `NaN`s are not comparable to any [`Float`], whenever this function sets the [`Float`] to
    /// `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets 1/x+\varepsilon.
    /// $$
    /// - If $1/x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $1/x$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy|\rfloor-p+1}$.
    /// - If $1/x$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |1/x|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// See the [`Float::reciprocal_prec_round`] documentation for information on special cases.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::reciprocal_prec_assign`]
    /// instead. If you know that your target precision is the precision of the input, consider
    /// using [`Float::reciprocal_round_assign`] instead. If both of these things are true, consider
    /// using [`Float::reciprocal_assign`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// prec)`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but `prec` is too small for an exact reciprocation;
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.reciprocal_prec_round_assign(5, Floor), Less);
    /// assert_eq!(x.to_string(), "0.31");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.reciprocal_prec_round_assign(5, Ceiling), Greater);
    /// assert_eq!(x.to_string(), "0.33");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.reciprocal_prec_round_assign(5, Nearest), Less);
    /// assert_eq!(x.to_string(), "0.31");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.reciprocal_prec_round_assign(20, Floor), Less);
    /// assert_eq!(x.to_string(), "0.3183098");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.reciprocal_prec_round_assign(20, Ceiling), Greater);
    /// assert_eq!(x.to_string(), "0.3183103");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.reciprocal_prec_round_assign(20, Nearest), Less);
    /// assert_eq!(x.to_string(), "0.3183098");
    /// ```
    #[inline]
    pub fn reciprocal_prec_round_assign(&mut self, prec: u64, rm: RoundingMode) -> Ordering {
        assert_ne!(prec, 0);
        match &mut *self {
            float_nan!() => Equal,
            Float(Zero { sign }) => {
                *self = Float(Infinity { sign: *sign });
                Equal
            }
            Float(Infinity { sign }) => {
                *self = Float(Zero { sign: *sign });
                Equal
            }
            Float(Finite {
                sign,
                exponent: exp,
                precision: x_prec,
                significand: x,
            }) => {
                if x.is_power_of_2() {
                    let sign = *sign;
                    let o;
                    (*self, o) = Float::power_of_2_prec(i64::from(1 - *exp), prec);
                    return if sign {
                        o
                    } else {
                        self.neg_assign();
                        o.reverse()
                    };
                }
                let sign = *sign;
                let (reciprocal, exp_offset, o) =
                    reciprocal_float_significand_ref(x, *x_prec, prec, if sign { rm } else { -rm });
                *exp = 1i32
                    .checked_sub(*exp)
                    .unwrap()
                    .checked_add(i32::exact_from(exp_offset))
                    .unwrap();
                *x_prec = prec;
                *x = reciprocal;
                if sign {
                    o
                } else {
                    o.reverse()
                }
            }
        }
    }

    /// Takes the reciprocal of a [`Float`] in place, rounding the result to the nearest value of
    /// the specified precision. An [`Ordering`] is returned, indicating whether the rounded
    /// reciprocal is less than, equal to, or greater than the exact reciprocal. Although `NaN`s are
    /// not comparable to any [`Float`], whenever this function sets the [`Float`] to `NaN` it also
    /// returns `Equal`.
    ///
    /// If the reciprocal is equidistant from two [`Float`]s with the specified precision, the
    /// [`Float`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a
    /// description of the `Nearest` rounding mode.
    ///
    /// $$
    /// x \gets 1/x+\varepsilon.
    /// $$
    /// - If $1/x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $1/x$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2 |1/x|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// See the [`Float::reciprocal_prec`] documentation for information on special cases.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::reciprocal_prec_round_assign`] instead. If you know that your target precision is
    /// the precision of the input, consider using [`Float::reciprocal`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// prec)`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.reciprocal_prec_assign(5), Less);
    /// assert_eq!(x.to_string(), "0.31");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.reciprocal_prec_assign(20), Less);
    /// assert_eq!(x.to_string(), "0.3183098");
    /// ```
    #[inline]
    pub fn reciprocal_prec_assign(&mut self, prec: u64) -> Ordering {
        self.reciprocal_prec_round_assign(prec, Nearest)
    }

    /// Takes the reciprocal of a [`Float`] in place, rounding the result with the specified
    /// rounding mode. An [`Ordering`] is returned, indicating whether the rounded reciprocal is
    /// less than, equal to, or greater than the exact reciprocal. Although `NaN`s are not
    /// comparable to any [`Float`], whenever this function sets the [`Float`] to `NaN` it also
    /// returns `Equal`.
    ///
    /// The precision of the output is the precision of the input. See [`RoundingMode`] for a
    /// description of the possible rounding modes.
    ///
    /// $$
    /// x \gets 1/x+\varepsilon.
    /// $$
    /// - If $1/x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $1/x$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |1/x|\rfloor-p+1}$, where $p$ is the maximum precision of the inputs.
    /// - If $1/x$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |1/x|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// If the output has a precision, it is the precision of the input.
    ///
    /// See the [`Float::reciprocal_round`] documentation for information on special cases.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::reciprocal_prec_round_assign`] instead. If you know you'll be using the `Nearest`
    /// rounding mode, consider using [`Float::reciprocal_assign`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the precision of the input is not high enough to represent the
    /// output.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.reciprocal_round_assign(Floor), Less);
    /// assert_eq!(x.to_string(), "0.3183098861837905");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.reciprocal_round_assign(Ceiling), Greater);
    /// assert_eq!(x.to_string(), "0.318309886183791");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.reciprocal_round_assign(Nearest), Less);
    /// assert_eq!(x.to_string(), "0.3183098861837905");
    /// ```
    #[inline]
    pub fn reciprocal_round_assign(&mut self, rm: RoundingMode) -> Ordering {
        let prec = self.significant_bits();
        self.reciprocal_prec_round_assign(prec, rm)
    }
}

impl Reciprocal for Float {
    type Output = Float;

    /// Takes the reciprocal of a [`Float`], taking it by value.
    ///
    /// If the output has a precision, it is the precision of the input. If the reciprocal is
    /// equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s in
    /// its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// $$
    /// f(x,y) = 1/x+\varepsilon.
    /// $$
    /// - If $1/x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $1/x$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2 |1/x|\rfloor-p}$,
    ///   where $p$ is the maximum precision of the inputs.
    ///
    /// Special cases:
    /// - $f(\text{NaN})=\text{NaN}$
    /// - $f(\infty)=0.0$
    /// - $f(-\infty)=-0.0$
    /// - $f(0.0)=\infty$
    /// - $f(-0.0)=-\infty$
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::reciprocal_prec`] instead. If you want to specify the output precision, consider
    /// using [`Float::reciprocal_round`]. If you want both of these things, consider using
    /// [`Float::reciprocal_prec_round`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Reciprocal;
    /// use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity};
    /// use malachite_float::Float;
    ///
    /// assert!(Float::NAN.reciprocal().is_nan());
    /// assert_eq!(Float::INFINITY.reciprocal().to_string(), "0.0");
    /// assert_eq!(Float::NEGATIVE_INFINITY.reciprocal().to_string(), "-0.0");
    /// assert_eq!(Float::from(1.5).reciprocal().to_string(), "0.8");
    /// assert_eq!(Float::from(-1.5).reciprocal().to_string(), "-0.8");
    /// ```
    #[inline]
    fn reciprocal(self) -> Float {
        let prec = self.significant_bits();
        self.reciprocal_prec_round(prec, Nearest).0
    }
}

impl Reciprocal for &Float {
    type Output = Float;

    /// Takes the reciprocal of a [`Float`], taking it by reference.
    ///
    /// If the output has a precision, it is the precision of the input. If the reciprocal is
    /// equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s in
    /// its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// $$
    /// f(x,y) = 1/x+\varepsilon.
    /// $$
    /// - If $1/x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $1/x$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2 |1/x|\rfloor-p}$,
    ///   where $p$ is the maximum precision of the inputs.
    ///
    /// Special cases:
    /// - $f(\text{NaN})=\text{NaN}$
    /// - $f(\infty)=0.0$
    /// - $f(-\infty)=-0.0$
    /// - $f(0.0)=\infty$
    /// - $f(-0.0)=-\infty$
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::reciprocal_prec_ref`] instead. If you want to specify the output precision,
    /// consider using [`Float::reciprocal_round_ref`]. If you want both of these things, consider
    /// using [`Float::reciprocal_prec_round_ref`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Reciprocal;
    /// use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity};
    /// use malachite_float::Float;
    ///
    /// assert!((&Float::NAN).reciprocal().is_nan());
    /// assert_eq!((&Float::INFINITY).reciprocal().to_string(), "0.0");
    /// assert_eq!((&Float::NEGATIVE_INFINITY).reciprocal().to_string(), "-0.0");
    /// assert_eq!((&Float::from(1.5)).reciprocal().to_string(), "0.8");
    /// assert_eq!((&Float::from(-1.5)).reciprocal().to_string(), "-0.8");
    /// ```
    #[inline]
    fn reciprocal(self) -> Float {
        let prec = self.significant_bits();
        self.reciprocal_prec_round_ref(prec, Nearest).0
    }
}

impl ReciprocalAssign for Float {
    /// Takes the reciprocal of a [`Float`] in place.
    ///
    /// If the output has a precision, it is the precision of the input. If the reciprocal is
    /// equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s in
    /// its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// $$
    /// x\gets = 1/x+\varepsilon.
    /// $$
    /// - If $1/x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $1/x$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2 |1/x|\rfloor-p}$,
    ///   where $p$ is the maximum precision of the inputs.
    ///
    /// See the [`Float::reciprocal`] documentation for information on special cases.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::reciprocal_prec_assign`] instead. If you want to specify the output precision,
    /// consider using [`Float::reciprocal_round_assign`]. If you want both of these things,
    /// consider using [`Float::reciprocal_prec_round_assign`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::ReciprocalAssign;
    /// use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity};
    /// use malachite_float::Float;
    ///
    /// let mut x = Float::NAN;
    /// x.reciprocal_assign();
    /// assert!(x.is_nan());
    ///
    /// let mut x = Float::INFINITY;
    /// x.reciprocal_assign();
    /// assert_eq!(x.to_string(), "0.0");
    ///
    /// let mut x = Float::NEGATIVE_INFINITY;
    /// x.reciprocal_assign();
    /// assert_eq!(x.to_string(), "-0.0");
    ///
    /// let mut x = Float::from(1.5);
    /// x.reciprocal_assign();
    /// assert_eq!(x.to_string(), "0.8");
    ///
    /// let mut x = Float::from(-1.5);
    /// x.reciprocal_assign();
    /// assert_eq!(x.to_string(), "-0.8");
    /// ```
    #[inline]
    fn reciprocal_assign(&mut self) {
        let prec = self.significant_bits();
        self.reciprocal_prec_round_assign(prec, Nearest);
    }
}
