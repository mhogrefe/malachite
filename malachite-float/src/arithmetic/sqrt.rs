// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::InnerFloat::{Finite, Infinity, NaN, Zero};
use crate::conversion::from_natural::{from_natural_zero_exponent, from_natural_zero_exponent_ref};
use crate::{
    Float, emulate_rational_to_float_fn, float_infinity, float_nan, float_negative_infinity,
    float_negative_zero, float_zero,
};
use core::cmp::Ordering::{self, *};
use malachite_base::num::arithmetic::traits::{
    CheckedLogBase2, CheckedSqrt, FloorLogBase2, Parity, Sqrt, SqrtAssign,
};
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::NaN as NanTrait;
use malachite_base::num::comparison::traits::PartialOrdAbs;
use malachite_base::num::conversion::traits::{ExactFrom, RoundingFrom};
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_nz::natural::arithmetic::float_extras::float_can_round;
use malachite_nz::natural::arithmetic::float_sqrt::{
    sqrt_float_significand_in_place, sqrt_float_significand_ref,
};
use malachite_nz::platform::Limb;
use malachite_q::Rational;

pub_crate_test! {
generic_sqrt_rational_ref(x: &Rational, prec: u64, rm: RoundingMode) -> (Float, Ordering) {
    let mut working_prec = prec + 10;
    let mut increment = Limb::WIDTH;
    let mut end_shift = x.floor_log_base_2();
    let x2;
    let reduced_x: &Rational;
    if end_shift.gt_abs(&0x3fff_0000) {
        end_shift &= !1;
        x2 = x >> end_shift;
        reduced_x = &x2;
    } else {
        end_shift = 0;
        reduced_x = x;
    }
    loop {
        let sqrt = Float::from_rational_prec_round_ref(reduced_x, working_prec, Floor).0.sqrt();
        // See algorithms.tex. Since we rounded down when computing fx, the absolute error of the
        // square root is bounded by (c_sqrt + k_fx)ulp(sqrt) <= 2ulp(sqrt).
        //
        // Experiments suggest that `working_prec` is low enough (that is, that the error is at most
        // 1 ulp), but I can only prove `working_prec - 1`.
        if float_can_round(sqrt.significand_ref().unwrap(), working_prec - 1, prec, rm) {
            let (mut sqrt, mut o) = Float::from_float_prec_round(sqrt, prec, rm);
            if end_shift != 0 {
                o = sqrt.shl_prec_round_assign_helper(end_shift >> 1, prec, rm, o);
            }
            return (sqrt, o);
        }
        working_prec += increment;
        increment = working_prec >> 1;
    }
}}

pub(crate) fn generic_sqrt_rational(
    mut x: Rational,
    prec: u64,
    rm: RoundingMode,
) -> (Float, Ordering) {
    let mut working_prec = prec + 10;
    let mut increment = Limb::WIDTH;
    let mut end_shift = x.floor_log_base_2();
    if end_shift.gt_abs(&0x3fff_0000) {
        end_shift &= !1;
        x >>= end_shift;
    } else {
        end_shift = 0;
    }
    loop {
        let sqrt = Float::from_rational_prec_round_ref(&x, working_prec, Floor)
            .0
            .sqrt();
        // See algorithms.tex. Since we rounded down when computing fx, the absolute error of the
        // square root is bounded by (c_sqrt + k_fx)ulp(sqrt) <= 2ulp(sqrt).
        //
        // Experiments suggest that `working_prec` is low enough (that is, that the error is at most
        // 1 ulp), but I can only prove `working_prec - 1`.
        if float_can_round(sqrt.significand_ref().unwrap(), working_prec - 1, prec, rm) {
            let (mut sqrt, mut o) = Float::from_float_prec_round(sqrt, prec, rm);
            if end_shift != 0 {
                o = sqrt.shl_prec_round_assign_helper(end_shift >> 1, prec, rm, o);
            }
            return (sqrt, o);
        }
        working_prec += increment;
        increment = working_prec >> 1;
    }
}

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
    ///   2^{\lfloor\log_2 \sqrt{x}\rfloor-p+1}$.
    /// - If $\sqrt{x}$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 \sqrt{x}\rfloor-p}$.
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
    ///   2^{\lfloor\log_2 \sqrt{x}\rfloor-p+1}$.
    /// - If $\sqrt{x}$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 \sqrt{x}\rfloor-p}$.
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
            Self(NaN | Infinity { sign: false }) => (float_nan!(), Equal),
            float_infinity!() => (float_infinity!(), Equal),
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
    /// indicating whether the rounded square root is less than, equal to, or greater than the exact
    /// square root. Although `NaN`s are not comparable to any [`Float`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
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
    ///   \sqrt{x}\rfloor-p}$.
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
    /// indicating whether the rounded square root is less than, equal to, or greater than the exact
    /// square root. Although `NaN`s are not comparable to any [`Float`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
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
    ///   \sqrt{x}\rfloor-p}$.
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
    ///   2^{\lfloor\log_2 \sqrt{x}\rfloor-p+1}$, where $p$ is the precision of the input.
    /// - If $\sqrt{x}$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 \sqrt{x}\rfloor-p}$, where $p$ is the precision of the input.
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
    ///   2^{\lfloor\log_2 \sqrt{x}\rfloor-p+1}$, where $p$ is the precision of the input.
    /// - If $\sqrt{x}$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 \sqrt{x}\rfloor-p}$, where $p$ is the precision of the input.
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
    /// - If $\sqrt{x}$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 \sqrt{x}\rfloor-p}$.
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
            Self(NaN | Infinity { sign: true } | Zero { .. }) => Equal,
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
    ///   \sqrt{x}\rfloor-p}$.
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
    ///   2^{\lfloor\log_2 \sqrt{x}\rfloor-p+1}$, where $p$ is the maximum precision of the inputs.
    /// - If $\sqrt{x}$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 \sqrt{x}\rfloor-p}$, where $p$ is the maximum precision of the inputs.
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

    /// Computes the square root of a [`Rational`], rounding the result to the specified precision
    /// and with the specified rounding mode and returning the result as a [`Float`]. The
    /// [`Rational`] is taken by value. An [`Ordering`] is also returned, indicating whether the
    /// rounded square root is less than, equal to, or greater than the exact square root. Although
    /// `NaN`s are not comparable to any [`Float`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
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
    ///   2^{\lfloor\log_2 \sqrt{x}\rfloor-p+1}$.
    /// - If $\sqrt{x}$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 \sqrt{x}\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(0.0,p,m)=0.0$
    ///
    /// Overflow and underflow:
    /// - If $f(x,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$ is
    ///   returned instead, where `p` is the precision of the input.
    /// - If $f(x,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is
    ///   returned instead.
    /// - If $f(x,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`, $-(1-(1/2)^p)2^{2^{30}-1}$
    ///   is returned instead, where `p` is the precision of the input.
    /// - If $0<f(x,p,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,p,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,p,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,p,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,p,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,p,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,p,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,p,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is
    ///   returned instead.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::sqrt_rational_prec`] instead.
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
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (sqrt, o) = Float::sqrt_rational_prec_round(Rational::from_unsigneds(3u8, 5), 5, Floor);
    /// assert_eq!(sqrt.to_string(), "0.75");
    /// assert_eq!(o, Less);
    ///
    /// let (sqrt, o) =
    ///     Float::sqrt_rational_prec_round(Rational::from_unsigneds(3u8, 5), 5, Ceiling);
    /// assert_eq!(sqrt.to_string(), "0.78");
    /// assert_eq!(o, Greater);
    ///
    /// let (sqrt, o) =
    ///     Float::sqrt_rational_prec_round(Rational::from_unsigneds(3u8, 5), 5, Nearest);
    /// assert_eq!(sqrt.to_string(), "0.78");
    /// assert_eq!(o, Greater);
    ///
    /// let (sqrt, o) =
    ///     Float::sqrt_rational_prec_round(Rational::from_unsigneds(3u8, 5), 20, Floor);
    /// assert_eq!(sqrt.to_string(), "0.774596");
    /// assert_eq!(o, Less);
    ///
    /// let (sqrt, o) =
    ///     Float::sqrt_rational_prec_round(Rational::from_unsigneds(3u8, 5), 20, Ceiling);
    /// assert_eq!(sqrt.to_string(), "0.774597");
    /// assert_eq!(o, Greater);
    ///
    /// let (sqrt, o) =
    ///     Float::sqrt_rational_prec_round(Rational::from_unsigneds(3u8, 5), 20, Nearest);
    /// assert_eq!(sqrt.to_string(), "0.774596");
    /// assert_eq!(o, Less);
    /// ```
    pub fn sqrt_rational_prec_round(x: Rational, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        assert_ne!(prec, 0);
        if x < 0u32 {
            return (Self::NAN, Equal);
        }
        if let Some(sqrt) = (&x).checked_sqrt() {
            return Self::from_rational_prec_round(sqrt, prec, rm);
        }
        let (n, d) = x.numerator_and_denominator_ref();
        match (n.checked_log_base_2(), d.checked_log_base_2()) {
            (_, Some(log_d)) if log_d.even() => {
                let n = x.into_numerator();
                let n_exp = n.significant_bits();
                let mut n = from_natural_zero_exponent(n);
                if n_exp.odd() {
                    n <<= 1u32;
                }
                let (mut sqrt, o) = Self::exact_from(n).sqrt_prec_round(prec, rm);
                let o = sqrt.shr_prec_round_assign_helper(
                    i128::from(log_d >> 1) - i128::from(n_exp >> 1),
                    prec,
                    rm,
                    o,
                );
                (sqrt, o)
            }
            (Some(log_n), _) if log_n.even() => {
                let d = x.into_denominator();
                let d_exp = d.significant_bits();
                let mut d = from_natural_zero_exponent(d);
                if d_exp.odd() {
                    d <<= 1u32;
                }
                let (mut reciprocal_sqrt, o) =
                    Self::exact_from(d).reciprocal_sqrt_prec_round(prec, rm);
                let o = reciprocal_sqrt.shl_prec_round_assign_helper(
                    i128::from(log_n >> 1) - i128::from(d_exp >> 1),
                    prec,
                    rm,
                    o,
                );
                (reciprocal_sqrt, o)
            }
            _ => generic_sqrt_rational(x, prec, rm),
        }
    }

    /// Computes the square root of a [`Rational`], rounding the result to the specified precision
    /// and with the specified rounding mode and returning the result as a [`Float`]. The
    /// [`Rational`] is taken by reference. An [`Ordering`] is also returned, indicating whether the
    /// rounded square root is less than, equal to, or greater than the exact square root. Although
    /// `NaN`s are not comparable to any [`Float`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
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
    ///   2^{\lfloor\log_2 \sqrt{x}\rfloor-p+1}$.
    /// - If $\sqrt{x}$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 \sqrt{x}\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(0.0,p,m)=0.0$
    ///
    /// Overflow and underflow:
    /// - If $f(x,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$ is
    ///   returned instead, where `p` is the precision of the input.
    /// - If $f(x,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is
    ///   returned instead.
    /// - If $f(x,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`, $-(1-(1/2)^p)2^{2^{30}-1}$
    ///   is returned instead, where `p` is the precision of the input.
    /// - If $0<f(x,p,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,p,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,p,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,p,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,p,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,p,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,p,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,p,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is
    ///   returned instead.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::sqrt_rational_prec_ref`]
    /// instead.
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
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (sqrt, o) =
    ///     Float::sqrt_rational_prec_round_ref(&Rational::from_unsigneds(3u8, 5), 5, Floor);
    /// assert_eq!(sqrt.to_string(), "0.75");
    /// assert_eq!(o, Less);
    ///
    /// let (sqrt, o) =
    ///     Float::sqrt_rational_prec_round_ref(&Rational::from_unsigneds(3u8, 5), 5, Ceiling);
    /// assert_eq!(sqrt.to_string(), "0.78");
    /// assert_eq!(o, Greater);
    ///
    /// let (sqrt, o) =
    ///     Float::sqrt_rational_prec_round_ref(&Rational::from_unsigneds(3u8, 5), 5, Nearest);
    /// assert_eq!(sqrt.to_string(), "0.78");
    /// assert_eq!(o, Greater);
    ///
    /// let (sqrt, o) =
    ///     Float::sqrt_rational_prec_round_ref(&Rational::from_unsigneds(3u8, 5), 20, Floor);
    /// assert_eq!(sqrt.to_string(), "0.774596");
    /// assert_eq!(o, Less);
    ///
    /// let (sqrt, o) =
    ///     Float::sqrt_rational_prec_round_ref(&Rational::from_unsigneds(3u8, 5), 20, Ceiling);
    /// assert_eq!(sqrt.to_string(), "0.774597");
    /// assert_eq!(o, Greater);
    ///
    /// let (sqrt, o) =
    ///     Float::sqrt_rational_prec_round_ref(&Rational::from_unsigneds(3u8, 5), 20, Nearest);
    /// assert_eq!(sqrt.to_string(), "0.774596");
    /// assert_eq!(o, Less);
    /// ```
    pub fn sqrt_rational_prec_round_ref(
        x: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        assert_ne!(prec, 0);
        if *x < 0u32 {
            return (Self::NAN, Equal);
        }
        if let Some(sqrt) = x.checked_sqrt() {
            return Self::from_rational_prec_round(sqrt, prec, rm);
        }
        let (n, d) = x.numerator_and_denominator_ref();
        match (n.checked_log_base_2(), d.checked_log_base_2()) {
            (_, Some(log_d)) if log_d.even() => {
                let n_exp = n.significant_bits();
                let mut n = from_natural_zero_exponent_ref(n);
                if n_exp.odd() {
                    n <<= 1u32;
                }
                let (mut sqrt, o) = Self::exact_from(n).sqrt_prec_round(prec, rm);
                let o = sqrt.shr_prec_round_assign_helper(
                    i128::from(log_d >> 1) - i128::from(n_exp >> 1),
                    prec,
                    rm,
                    o,
                );
                (sqrt, o)
            }
            (Some(log_n), _) if log_n.even() => {
                let d_exp = d.significant_bits();
                let mut d = from_natural_zero_exponent_ref(d);
                if d_exp.odd() {
                    d <<= 1u32;
                }
                let (mut reciprocal_sqrt, o) =
                    Self::exact_from(d).reciprocal_sqrt_prec_round(prec, rm);
                let o = reciprocal_sqrt.shl_prec_round_assign_helper(
                    i128::from(log_n >> 1) - i128::from(d_exp >> 1),
                    prec,
                    rm,
                    o,
                );
                (reciprocal_sqrt, o)
            }
            _ => generic_sqrt_rational_ref(x, prec, rm),
        }
    }

    /// Computes the square root of a [`Rational`], rounding the result to the nearest value of the
    /// specified precision and returning the result as a [`Float`]. The [`Rational`] is taken by
    /// value. An [`Ordering`] is also returned, indicating whether the rounded square root is less
    /// than, equal to, or greater than the exact square root. Although `NaN`s are not comparable to
    /// any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
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
    ///   \sqrt{x}\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(0.0,p)=0.0$
    ///
    /// Overflow and underflow:
    /// - If $f(x,p)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,p)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$ is
    ///   returned instead, where `p` is the precision of the input.
    /// - If $f(x,p)\geq 2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is returned
    ///   instead.
    /// - If $f(x,p)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`, $-(1-(1/2)^p)2^{2^{30}-1}$ is
    ///   returned instead, where `p` is the precision of the input.
    /// - If $0<f(x,p)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,p)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,p)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,p,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,p)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,p)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,p)<0$, and $m$ is `Nearest`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,p)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is returned
    ///   instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::sqrt_rational_prec_round`] instead.
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
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (sqrt, o) = Float::sqrt_rational_prec(Rational::from_unsigneds(3u8, 5), 5);
    /// assert_eq!(sqrt.to_string(), "0.78");
    /// assert_eq!(o, Greater);
    ///
    /// let (sqrt, o) = Float::sqrt_rational_prec(Rational::from_unsigneds(3u8, 5), 20);
    /// assert_eq!(sqrt.to_string(), "0.774596");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn sqrt_rational_prec(x: Rational, prec: u64) -> (Self, Ordering) {
        Self::sqrt_rational_prec_round(x, prec, Nearest)
    }

    /// Computes the square root of a [`Rational`], rounding the result to the nearest value of the
    /// specified precision and returning the result as a [`Float`]. The [`Rational`] is taken by
    /// reference. An [`Ordering`] is also returned, indicating whether the rounded square root is
    /// less than, equal to, or greater than the exact square root. Although `NaN`s are not
    /// comparable to any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
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
    ///   \sqrt{x}\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(0.0,p)=0.0$
    ///
    /// Overflow and underflow:
    /// - If $f(x,p)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,p)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$ is
    ///   returned instead, where `p` is the precision of the input.
    /// - If $f(x,p)\geq 2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is returned
    ///   instead.
    /// - If $f(x,p)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`, $-(1-(1/2)^p)2^{2^{30}-1}$ is
    ///   returned instead, where `p` is the precision of the input.
    /// - If $0<f(x,p)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,p)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,p)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,p,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,p)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,p)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,p)<0$, and $m$ is `Nearest`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,p)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is returned
    ///   instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::sqrt_rational_prec_round_ref`] instead.
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
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (sqrt, o) = Float::sqrt_rational_prec_ref(&Rational::from_unsigneds(3u8, 5), 5);
    /// assert_eq!(sqrt.to_string(), "0.78");
    /// assert_eq!(o, Greater);
    ///
    /// let (sqrt, o) = Float::sqrt_rational_prec_ref(&Rational::from_unsigneds(3u8, 5), 20);
    /// assert_eq!(sqrt.to_string(), "0.774596");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn sqrt_rational_prec_ref(x: &Rational, prec: u64) -> (Self, Ordering) {
        Self::sqrt_rational_prec_round_ref(x, prec, Nearest)
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
    ///   \sqrt{x}\rfloor-p}$, where $p$ is the maximum precision of the inputs.
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
    ///   \sqrt{x}\rfloor-p}$, where $p$ is the maximum precision of the inputs.
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
    ///   \sqrt{x}\rfloor-p}$, where $p$ is the maximum precision of the inputs.
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

/// Computes the square root of a [`Rational`], returning a primitive float result.
///
/// If the square root is equidistant from two primitive floats, the primitive float with fewer 1s
/// in its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
/// rounding mode.
///
/// The square root of any negative number is `NaN`.
///
/// $$
/// f(x) = \sqrt{x}+\varepsilon.
/// $$
/// - If $\sqrt{x}$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
/// - If $\sqrt{x}$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2
///   \sqrt{x}\rfloor-p}$, where $p$ is precision of the output (typically 24 if `T` is a [`f32`]
///   and 53 if `T` is a [`f64`], but less if the output is subnormal).
///
/// Special cases:
/// - $f(0)=0.0$
///
/// Overflow:
/// - If the absolute value of the result is too large to represent, $\infty$ is returned instead.
/// - If the absolute value of the result is too small to represent, 0.0 is returned instead.
///
/// # Worst-case complexity
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use malachite_base::num::basic::traits::Zero;
/// use malachite_base::num::float::NiceFloat;
/// use malachite_float::arithmetic::sqrt::primitive_float_sqrt_rational;
/// use malachite_q::Rational;
///
/// assert_eq!(
///     NiceFloat(primitive_float_sqrt_rational::<f64>(&Rational::ZERO)),
///     NiceFloat(0.0)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_sqrt_rational::<f64>(
///         &Rational::from_unsigneds(1u8, 3)
///     )),
///     NiceFloat(0.5773502691896257)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_sqrt_rational::<f64>(&Rational::from(10000))),
///     NiceFloat(100.0)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_sqrt_rational::<f64>(&Rational::from(
///         -10000
///     ))),
///     NiceFloat(f64::NAN)
/// );
/// ```
#[inline]
#[allow(clippy::type_repetition_in_bounds)]
pub fn primitive_float_sqrt_rational<T: PrimitiveFloat>(x: &Rational) -> T
where
    Float: PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    emulate_rational_to_float_fn(Float::sqrt_rational_prec_ref, x)
}
