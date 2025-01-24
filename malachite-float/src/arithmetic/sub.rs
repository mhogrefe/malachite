// Copyright © 2025 Mikhail Hogrefe
//
// Uses code adopted from the GNU MPFR Library.
//
//      Copyright 2001, 2003-2022 Free Software Foundation, Inc.
//
//      Contributed by the AriC and Caramba projects, INRIA.
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::InnerFloat::{Infinity, NaN, Zero};
use crate::{
    float_infinity, float_nan, float_negative_infinity, float_negative_zero, float_zero, Float,
};
use core::cmp::max;
use core::cmp::Ordering::{self, *};
use core::ops::{Sub, SubAssign};
use malachite_base::num::arithmetic::traits::{CeilingLogBase2, NegAssign};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::comparison::traits::PartialOrdAbs;
use malachite_base::num::conversion::traits::{ExactFrom, SaturatingFrom};
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_nz::natural::arithmetic::float_extras::float_can_round;
use malachite_nz::platform::Limb;
use malachite_q::Rational;

// x and y must be finite, nonzero, and not equal
fn float_rational_diff_exponent_range(x: &Float, y: &Rational) -> (i64, i64) {
    let log_x_abs = i64::from(x.get_exponent().unwrap() - 1);
    let log_y_abs = y.floor_log_base_2_abs();
    let m = max(log_x_abs, log_y_abs);
    if (*x > 0) != (*y > 0) {
        (m, m + 1)
    } else if log_x_abs.abs_diff(log_y_abs) > 1 {
        (m - 1, m)
    } else {
        let mut log_x_denominator = i64::exact_from(x.get_prec().unwrap())
            .saturating_sub(log_x_abs)
            .saturating_sub(1);
        if log_x_denominator < 0 {
            log_x_denominator = 0;
        }
        let log_y_denominator = i64::exact_from(y.denominator_ref().ceiling_log_base_2());
        let min_exp = log_x_denominator
            .checked_neg()
            .unwrap()
            .checked_sub(log_y_denominator)
            .unwrap();
        if log_x_abs == log_y_abs {
            (min_exp, m - 1)
        } else {
            (min_exp, m)
        }
    }
}

// x and y must be finite, nonzero, and not sum to zero
fn float_rational_diff_sign(x: &Float, y: &Rational) -> bool {
    match ((*x > 0), (*y < 0)) {
        (true, true) => true,
        (false, false) => false,
        _ => {
            if x.gt_abs(y) {
                *x > 0
            } else {
                *y < 0
            }
        }
    }
}

fn sub_rational_prec_round_naive_ref_val(
    x: &Float,
    y: Rational,
    prec: u64,
    rm: RoundingMode,
) -> (Float, Ordering) {
    assert_ne!(prec, 0);
    match (x, y) {
        (x @ Float(NaN | Infinity { .. }), _) => (x.clone(), Equal),
        (float_negative_zero!(), y) => {
            if y == 0u32 {
                (float_negative_zero!(), Equal)
            } else {
                Float::from_rational_prec_round(-y, prec, rm)
            }
        }
        (float_zero!(), y) => Float::from_rational_prec_round(-y, prec, rm),
        (x, y) => {
            let (mut sum, o) =
                Float::from_rational_prec_round(Rational::exact_from(x) - y, prec, rm);
            if rm == Floor && sum == 0u32 {
                sum.neg_assign();
            }
            (sum, o)
        }
    }
}

fn sub_rational_prec_round_naive_ref_ref(
    x: &Float,
    y: &Rational,
    prec: u64,
    rm: RoundingMode,
) -> (Float, Ordering) {
    assert_ne!(prec, 0);
    match (x, y) {
        (x @ Float(NaN | Infinity { .. }), _) => (x.clone(), Equal),
        (float_negative_zero!(), y) => {
            if *y == 0u32 {
                (float_negative_zero!(), Equal)
            } else {
                let (f, o) = Float::from_rational_prec_round_ref(y, prec, -rm);
                (-f, o.reverse())
            }
        }
        (float_zero!(), y) => {
            let (f, o) = Float::from_rational_prec_round_ref(y, prec, -rm);
            (-f, o.reverse())
        }
        (x, y) => {
            let (mut sum, o) =
                Float::from_rational_prec_round(Rational::exact_from(x) - y, prec, rm);
            if rm == Floor && sum == 0u32 {
                sum.neg_assign();
            }
            (sum, o)
        }
    }
}

impl Float {
    /// Subtracts two [`Float`]s, rounding the result to the specified precision and with the
    /// specified rounding mode. Both [`Float`]s are taken by value. An [`Ordering`] is also
    /// returned, indicating whether the rounded difference is less than, equal to, or greater than
    /// the exact difference. Although `NaN`s are not comparable to any [`Float`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,p,m) = x-y+\varepsilon.
    /// $$
    /// - If $x-y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-y$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x-y|\rfloor-p+1}$.
    /// - If $x-y$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x-y|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,p,m)=f(x,\text{NaN},p,m)=f(\infty,\infty,p,m)=f(-\infty,-\infty,p,m)=
    ///     \text{NaN}$
    /// - $f(\infty,x,p,m)=\infty$ if $x$ is not NaN or $\infty$
    /// - $f(x,-\infty,p,m)=\infty$ if $x$ is not NaN or $-\infty$
    /// - $f(-\infty,x,p,m)=-\infty$ if $x$ is not NaN or $-\infty$
    /// - $f(x,\infty,p,m)=-\infty$ if $x$ is not NaN or $\infty$
    /// - $f(0.0,-0.0,p,m)=0.0$
    /// - $f(-0.0,0.0,p,m)=-0.0$
    /// - $f(0.0,0.0,p,m)=f(-0.0,-0.0,p,m)=0.0$ if $m$ is not `Floor`
    /// - $f(0.0,0.0,p,m)=f(-0.0,-0.0,p,m)=-0.0$ if $m$ is `Floor`
    /// - $f(x,x,p,m)=0.0$ if $x$ is finite and nonzero and $m$ is not `Floor`
    /// - $f(x,x,p,m)=-0.0$ if $x$ is finite and nonzero and $m$ is `Floor`
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,y,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$
    ///   is returned instead, where `p` is the precision of the input.
    /// - If $f(x,y,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is
    ///   returned instead.
    /// - If $f(x,y,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`,
    ///   $-(1-(1/2)^p)2^{2^{30}-1}$ is returned instead, where `p` is the precision of the input.
    /// - If $0<f(x,y,p,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,y,p,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,y,p,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,p,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,p,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is
    ///   returned instead.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::sub_prec`] instead. If you
    /// know that your target precision is the maximum of the precisions of the two inputs, consider
    /// using [`Float::sub_round`] instead. If both of these things are true, consider using `-`
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but `prec` is too small for an exact subtraction.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (sum, o) = Float::from(PI).sub_prec_round(Float::from(E), 5, Floor);
    /// assert_eq!(sum.to_string(), "0.42");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) = Float::from(PI).sub_prec_round(Float::from(E), 5, Ceiling);
    /// assert_eq!(sum.to_string(), "0.44");
    /// assert_eq!(o, Greater);
    ///
    /// let (sum, o) = Float::from(PI).sub_prec_round(Float::from(E), 5, Nearest);
    /// assert_eq!(sum.to_string(), "0.42");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) = Float::from(PI).sub_prec_round(Float::from(E), 20, Floor);
    /// assert_eq!(sum.to_string(), "0.4233108");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) = Float::from(PI).sub_prec_round(Float::from(E), 20, Ceiling);
    /// assert_eq!(sum.to_string(), "0.4233112");
    /// assert_eq!(o, Greater);
    ///
    /// let (sum, o) = Float::from(PI).sub_prec_round(Float::from(E), 20, Nearest);
    /// assert_eq!(sum.to_string(), "0.4233108");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn sub_prec_round(
        mut self,
        other: Float,
        prec: u64,
        rm: RoundingMode,
    ) -> (Float, Ordering) {
        let o = self.sub_prec_round_assign(other, prec, rm);
        (self, o)
    }

    /// Subtracts two [`Float`]s, rounding the result to the specified precision and with the
    /// specified rounding mode. The first [`Float`] is taken by value and the second by reference.
    /// An [`Ordering`] is also returned, indicating whether the rounded difference is less than,
    /// equal to, or greater than the exact difference. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,p,m) = x-y+\varepsilon.
    /// $$
    /// - If $x-y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-y$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x-y|\rfloor-p+1}$.
    /// - If $x-y$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x-y|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,p,m)=f(x,\text{NaN},p,m)=f(\infty,\infty,p,m)=f(-\infty,-\infty,p,m)=
    ///     \text{NaN}$
    /// - $f(\infty,x,p,m)=\infty$ if $x$ is not NaN or $\infty$
    /// - $f(x,-\infty,p,m)=\infty$ if $x$ is not NaN or $-\infty$
    /// - $f(-\infty,x,p,m)=-\infty$ if $x$ is not NaN or $-\infty$
    /// - $f(x,\infty,p,m)=-\infty$ if $x$ is not NaN or $\infty$
    /// - $f(0.0,-0.0,p,m)=0.0$
    /// - $f(-0.0,0.0,p,m)=-0.0$
    /// - $f(0.0,0.0,p,m)=f(-0.0,-0.0,p,m)=0.0$ if $m$ is not `Floor`
    /// - $f(0.0,0.0,p,m)=f(-0.0,-0.0,p,m)=-0.0$ if $m$ is `Floor`
    /// - $f(x,x,p,m)=0.0$ if $x$ is finite and nonzero and $m$ is not `Floor`
    /// - $f(x,x,p,m)=-0.0$ if $x$ is finite and nonzero and $m$ is `Floor`
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,y,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$
    ///   is returned instead, where `p` is the precision of the input.
    /// - If $f(x,y,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is
    ///   returned instead.
    /// - If $f(x,y,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`,
    ///   $-(1-(1/2)^p)2^{2^{30}-1}$ is returned instead, where `p` is the precision of the input.
    /// - If $0<f(x,y,p,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,y,p,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,y,p,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,p,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,p,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is
    ///   returned instead.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::sub_prec_val_ref`] instead.
    /// If you know that your target precision is the maximum of the precisions of the two inputs,
    /// consider using [`Float::sub_round_val_ref`] instead. If both of these things are true,
    /// consider using `-` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but `prec` is too small for an exact subtraction.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (sum, o) = Float::from(PI).sub_prec_round_val_ref(&Float::from(E), 5, Floor);
    /// assert_eq!(sum.to_string(), "0.42");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) = Float::from(PI).sub_prec_round_val_ref(&Float::from(E), 5, Ceiling);
    /// assert_eq!(sum.to_string(), "0.44");
    /// assert_eq!(o, Greater);
    ///
    /// let (sum, o) = Float::from(PI).sub_prec_round_val_ref(&Float::from(E), 5, Nearest);
    /// assert_eq!(sum.to_string(), "0.42");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) = Float::from(PI).sub_prec_round_val_ref(&Float::from(E), 20, Floor);
    /// assert_eq!(sum.to_string(), "0.4233108");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) = Float::from(PI).sub_prec_round_val_ref(&Float::from(E), 20, Ceiling);
    /// assert_eq!(sum.to_string(), "0.4233112");
    /// assert_eq!(o, Greater);
    ///
    /// let (sum, o) = Float::from(PI).sub_prec_round_val_ref(&Float::from(E), 20, Nearest);
    /// assert_eq!(sum.to_string(), "0.4233108");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn sub_prec_round_val_ref(
        mut self,
        other: &Float,
        prec: u64,
        rm: RoundingMode,
    ) -> (Float, Ordering) {
        let o = self.sub_prec_round_assign_ref(other, prec, rm);
        (self, o)
    }

    /// Subtracts two [`Float`]s, rounding the result to the specified precision and with the
    /// specified rounding mode. The first [`Float`] is taken by reference and the second by value.
    /// An [`Ordering`] is also returned, indicating whether the rounded difference is less than,
    /// equal to, or greater than the exact difference. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,p,m) = x-y+\varepsilon.
    /// $$
    /// - If $x-y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-y$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x-y|\rfloor-p+1}$.
    /// - If $x-y$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x-y|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,p,m)=f(x,\text{NaN},p,m)=f(\infty,\infty,p,m)=f(-\infty,-\infty,p,m)=
    ///     \text{NaN}$
    /// - $f(\infty,x,p,m)=\infty$ if $x$ is not NaN or $\infty$
    /// - $f(x,-\infty,p,m)=\infty$ if $x$ is not NaN or $-\infty$
    /// - $f(-\infty,x,p,m)=-\infty$ if $x$ is not NaN or $-\infty$
    /// - $f(x,\infty,p,m)=-\infty$ if $x$ is not NaN or $\infty$
    /// - $f(0.0,-0.0,p,m)=0.0$
    /// - $f(-0.0,0.0,p,m)=-0.0$
    /// - $f(0.0,0.0,p,m)=f(-0.0,-0.0,p,m)=0.0$ if $m$ is not `Floor`
    /// - $f(0.0,0.0,p,m)=f(-0.0,-0.0,p,m)=-0.0$ if $m$ is `Floor`
    /// - $f(x,x,p,m)=0.0$ if $x$ is finite and nonzero and $m$ is not `Floor`
    /// - $f(x,x,p,m)=-0.0$ if $x$ is finite and nonzero and $m$ is `Floor`
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,y,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$
    ///   is returned instead, where `p` is the precision of the input.
    /// - If $f(x,y,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is
    ///   returned instead.
    /// - If $f(x,y,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`,
    ///   $-(1-(1/2)^p)2^{2^{30}-1}$ is returned instead, where `p` is the precision of the input.
    /// - If $0<f(x,y,p,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,y,p,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,y,p,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,p,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,p,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is
    ///   returned instead.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::sub_prec_ref_val`] instead.
    /// If you know that your target precision is the maximum of the precisions of the two inputs,
    /// consider using [`Float::sub_round_ref_val`] instead. If both of these things are true,
    /// consider using `-` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but `prec` is too small for an exact subtraction.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (sum, o) = Float::from(PI).sub_prec_round_ref_val(Float::from(E), 5, Floor);
    /// assert_eq!(sum.to_string(), "0.42");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) = Float::from(PI).sub_prec_round_ref_val(Float::from(E), 5, Ceiling);
    /// assert_eq!(sum.to_string(), "0.44");
    /// assert_eq!(o, Greater);
    ///
    /// let (sum, o) = Float::from(PI).sub_prec_round_ref_val(Float::from(E), 5, Nearest);
    /// assert_eq!(sum.to_string(), "0.42");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) = Float::from(PI).sub_prec_round_ref_val(Float::from(E), 20, Floor);
    /// assert_eq!(sum.to_string(), "0.4233108");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) = Float::from(PI).sub_prec_round_ref_val(Float::from(E), 20, Ceiling);
    /// assert_eq!(sum.to_string(), "0.4233112");
    /// assert_eq!(o, Greater);
    ///
    /// let (sum, o) = Float::from(PI).sub_prec_round_ref_val(Float::from(E), 20, Nearest);
    /// assert_eq!(sum.to_string(), "0.4233108");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn sub_prec_round_ref_val(
        &self,
        other: Float,
        prec: u64,
        rm: RoundingMode,
    ) -> (Float, Ordering) {
        self.add_prec_round_ref_val(-other, prec, rm)
    }

    /// Subtracts two [`Float`]s, rounding the result to the specified precision and with the
    /// specified rounding mode. Both [`Float`]s are taken by reference. An [`Ordering`] is also
    /// returned, indicating whether the rounded difference is less than, equal to, or greater than
    /// the exact difference. Although `NaN`s are not comparable to any [`Float`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,p,m) = x-y+\varepsilon.
    /// $$
    /// - If $x-y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-y$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x-y|\rfloor-p+1}$.
    /// - If $x-y$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x-y|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,p,m)=f(x,\text{NaN},p,m)=f(\infty,\infty,p,m)=f(-\infty,-\infty,p,m)=
    ///     \text{NaN}$
    /// - $f(\infty,x,p,m)=\infty$ if $x$ is not NaN or $\infty$
    /// - $f(x,-\infty,p,m)=\infty$ if $x$ is not NaN or $-\infty$
    /// - $f(-\infty,x,p,m)=-\infty$ if $x$ is not NaN or $-\infty$
    /// - $f(x,\infty,p,m)=-\infty$ if $x$ is not NaN or $\infty$
    /// - $f(0.0,-0.0,p,m)=0.0$
    /// - $f(-0.0,0.0,p,m)=-0.0$
    /// - $f(0.0,0.0,p,m)=f(-0.0,-0.0,p,m)=0.0$ if $m$ is not `Floor`
    /// - $f(0.0,0.0,p,m)=f(-0.0,-0.0,p,m)=-0.0$ if $m$ is `Floor`
    /// - $f(x,x,p,m)=0.0$ if $x$ is finite and nonzero and $m$ is not `Floor`
    /// - $f(x,x,p,m)=-0.0$ if $x$ is finite and nonzero and $m$ is `Floor`
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,y,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$
    ///   is returned instead, where `p` is the precision of the input.
    /// - If $f(x,y,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is
    ///   returned instead.
    /// - If $f(x,y,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`,
    ///   $-(1-(1/2)^p)2^{2^{30}-1}$ is returned instead, where `p` is the precision of the input.
    /// - If $0<f(x,y,p,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,y,p,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,y,p,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,p,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,p,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is
    ///   returned instead.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::sub_prec_ref_ref`] instead.
    /// If you know that your target precision is the maximum of the precisions of the two inputs,
    /// consider using [`Float::sub_round_ref_ref`] instead. If both of these things are true,
    /// consider using `-` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but `prec` is too small for an exact subtraction.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (sum, o) = Float::from(PI).sub_prec_round_ref_ref(&Float::from(E), 5, Floor);
    /// assert_eq!(sum.to_string(), "0.42");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) = Float::from(PI).sub_prec_round_ref_ref(&Float::from(E), 5, Ceiling);
    /// assert_eq!(sum.to_string(), "0.44");
    /// assert_eq!(o, Greater);
    ///
    /// let (sum, o) = Float::from(PI).sub_prec_round_ref_ref(&Float::from(E), 5, Nearest);
    /// assert_eq!(sum.to_string(), "0.42");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) = Float::from(PI).sub_prec_round_ref_ref(&Float::from(E), 20, Floor);
    /// assert_eq!(sum.to_string(), "0.4233108");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) = Float::from(PI).sub_prec_round_ref_ref(&Float::from(E), 20, Ceiling);
    /// assert_eq!(sum.to_string(), "0.4233112");
    /// assert_eq!(o, Greater);
    ///
    /// let (sum, o) = Float::from(PI).sub_prec_round_ref_ref(&Float::from(E), 20, Nearest);
    /// assert_eq!(sum.to_string(), "0.4233108");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn sub_prec_round_ref_ref(
        &self,
        other: &Float,
        prec: u64,
        rm: RoundingMode,
    ) -> (Float, Ordering) {
        self.add_prec_round_ref_ref_helper(other, prec, rm, true)
    }

    /// Subtracts two [`Float`]s, rounding the result to the nearest value of the specified
    /// precision. Both [`Float`]s are taken by value. An [`Ordering`] is also returned, indicating
    /// whether the rounded difference is less than, equal to, or greater than the exact difference.
    /// Although `NaN`s are not comparable to any [`Float`], whenever this function returns a `NaN`
    /// it also returns `Equal`.
    ///
    /// If the difference is equidistant from two [`Float`]s with the specified precision, the
    /// [`Float`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a
    /// description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,y,p) = x-y+\varepsilon.
    /// $$
    /// - If $x-y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-y$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2 |x-y|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,p)=f(x,\text{NaN},p)=f(\infty,\infty,p)=f(-\infty,-\infty,p)=\text{NaN}$
    /// - $f(\infty,x,p)=\infty$ if $x$ is not NaN or $\infty$
    /// - $f(x,-\infty,p)=\infty$ if $x$ is not NaN or $-\infty$
    /// - $f(-\infty,x,p)=-\infty$ if $x$ is not NaN or $-\infty$
    /// - $f(x,\infty,p)=-\infty$ if $x$ is not NaN or $\infty$
    /// - $f(0.0,-0.0,p)=0.0$
    /// - $f(-0.0,0.0,p)=-0.0$
    /// - $f(0.0,0.0,p)=f(-0.0,-0.0,p,m)=0.0$ if $m$ is not `Floor`
    /// - $f(0.0,0.0,p)=f(-0.0,-0.0,p,m)=-0.0$ if $m$ is `Floor`
    /// - $f(x,x,p)=0.0$ if $x$ is finite and nonzero and $m$ is not `Floor`
    /// - $f(x,x,p)=-0.0$ if $x$ is finite and nonzero and $m$ is `Floor`
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,p)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,y,p)\geq 2^{2^{30}-1}$, $-\infty$ is returned instead.
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::sub_prec_round`] instead. If you know that your target precision is the maximum of
    /// the precisions of the two inputs, consider using `-` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (sum, o) = Float::from(PI).sub_prec(Float::from(E), 5);
    /// assert_eq!(sum.to_string(), "0.42");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) = Float::from(PI).sub_prec(Float::from(E), 20);
    /// assert_eq!(sum.to_string(), "0.4233108");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn sub_prec(self, other: Float, prec: u64) -> (Float, Ordering) {
        self.sub_prec_round(other, prec, Nearest)
    }

    /// Subtracts two [`Float`]s, rounding the result to the nearest value of the specified
    /// precision. The first [`Float`] is taken by value and the second by reference. An
    /// [`Ordering`] is also returned, indicating whether the rounded difference is less than, equal
    /// to, or greater than the exact difference. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// If the difference is equidistant from two [`Float`]s with the specified precision, the
    /// [`Float`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a
    /// description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,y,p) = x-y+\varepsilon.
    /// $$
    /// - If $x-y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-y$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2 |x-y|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,p)=f(x,\text{NaN},p)=f(\infty,\infty,p)=f(-\infty,-\infty,p)=\text{NaN}$
    /// - $f(\infty,x,p)=\infty$ if $x$ is not NaN or $\infty$
    /// - $f(x,-\infty,p)=\infty$ if $x$ is not NaN or $-\infty$
    /// - $f(-\infty,x,p)=-\infty$ if $x$ is not NaN or $-\infty$
    /// - $f(x,\infty,p)=-\infty$ if $x$ is not NaN or $\infty$
    /// - $f(0.0,-0.0,p)=0.0$
    /// - $f(-0.0,0.0,p)=-0.0$
    /// - $f(0.0,0.0,p)=f(-0.0,-0.0,p,m)=0.0$ if $m$ is not `Floor`
    /// - $f(0.0,0.0,p)=f(-0.0,-0.0,p,m)=-0.0$ if $m$ is `Floor`
    /// - $f(x,x,p)=0.0$ if $x$ is finite and nonzero and $m$ is not `Floor`
    /// - $f(x,x,p)=-0.0$ if $x$ is finite and nonzero and $m$ is `Floor`
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,p)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,y,p)\geq 2^{2^{30}-1}$, $-\infty$ is returned instead.
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::sub_prec_round_val_ref`] instead. If you know that your target precision is the
    /// maximum of the precisions of the two inputs, consider using `-` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (sum, o) = Float::from(PI).sub_prec_val_ref(&Float::from(E), 5);
    /// assert_eq!(sum.to_string(), "0.42");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) = Float::from(PI).sub_prec_val_ref(&Float::from(E), 20);
    /// assert_eq!(sum.to_string(), "0.4233108");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn sub_prec_val_ref(self, other: &Float, prec: u64) -> (Float, Ordering) {
        self.sub_prec_round_val_ref(other, prec, Nearest)
    }

    /// Subtracts two [`Float`]s, rounding the result to the nearest value of the specified
    /// precision. The first [`Float`] is taken by reference and the second by value. An
    /// [`Ordering`] is also returned, indicating whether the rounded difference is less than, equal
    /// to, or greater than the exact difference. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// If the difference is equidistant from two [`Float`]s with the specified precision, the
    /// [`Float`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a
    /// description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,y,p) = x-y+\varepsilon.
    /// $$
    /// - If $x-y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-y$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2 |x-y|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,p)=f(x,\text{NaN},p)=f(\infty,\infty,p)=f(-\infty,-\infty,p)=\text{NaN}$
    /// - $f(\infty,x,p)=\infty$ if $x$ is not NaN or $\infty$
    /// - $f(x,-\infty,p)=\infty$ if $x$ is not NaN or $-\infty$
    /// - $f(-\infty,x,p)=-\infty$ if $x$ is not NaN or $-\infty$
    /// - $f(x,\infty,p)=-\infty$ if $x$ is not NaN or $\infty$
    /// - $f(0.0,-0.0,p)=0.0$
    /// - $f(-0.0,0.0,p)=-0.0$
    /// - $f(0.0,0.0,p)=f(-0.0,-0.0,p,m)=0.0$ if $m$ is not `Floor`
    /// - $f(0.0,0.0,p)=f(-0.0,-0.0,p,m)=-0.0$ if $m$ is `Floor`
    /// - $f(x,x,p)=0.0$ if $x$ is finite and nonzero and $m$ is not `Floor`
    /// - $f(x,x,p)=-0.0$ if $x$ is finite and nonzero and $m$ is `Floor`
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,p)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,y,p)\geq 2^{2^{30}-1}$, $-\infty$ is returned instead.
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::sub_prec_round_ref_val`] instead. If you know that your target precision is the
    /// maximum of the precisions of the two inputs, consider using `-` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (sum, o) = Float::from(PI).sub_prec_ref_val(Float::from(E), 5);
    /// assert_eq!(sum.to_string(), "0.42");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) = Float::from(PI).sub_prec_ref_val(Float::from(E), 20);
    /// assert_eq!(sum.to_string(), "0.4233108");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn sub_prec_ref_val(&self, other: Float, prec: u64) -> (Float, Ordering) {
        self.sub_prec_round_ref_val(other, prec, Nearest)
    }

    /// Subtracts two [`Float`]s, rounding the result to the nearest value of the specified
    /// precision. Both [`Float`]s are taken by reference. An [`Ordering`] is also returned,
    /// indicating whether the rounded difference is less than, equal to, or greater than the exact
    /// difference. Although `NaN`s are not comparable to any [`Float`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// If the difference is equidistant from two [`Float`]s with the specified precision, the
    /// [`Float`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a
    /// description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,y,p) = x-y+\varepsilon.
    /// $$
    /// - If $x-y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-y$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2 |x-y|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,p)=f(x,\text{NaN},p)=f(\infty,\infty,p)=f(-\infty,-\infty,p)=\text{NaN}$
    /// - $f(\infty,x,p)=\infty$ if $x$ is not NaN or $\infty$
    /// - $f(x,-\infty,p)=\infty$ if $x$ is not NaN or $-\infty$
    /// - $f(-\infty,x,p)=-\infty$ if $x$ is not NaN or $-\infty$
    /// - $f(x,\infty,p)=-\infty$ if $x$ is not NaN or $\infty$
    /// - $f(0.0,-0.0,p)=0.0$
    /// - $f(-0.0,0.0,p)=-0.0$
    /// - $f(0.0,0.0,p)=f(-0.0,-0.0,p,m)=0.0$ if $m$ is not `Floor`
    /// - $f(0.0,0.0,p)=f(-0.0,-0.0,p,m)=-0.0$ if $m$ is `Floor`
    /// - $f(x,x,p)=0.0$ if $x$ is finite and nonzero and $m$ is not `Floor`
    /// - $f(x,x,p)=-0.0$ if $x$ is finite and nonzero and $m$ is `Floor`
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,p)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,y,p)\geq 2^{2^{30}-1}$, $-\infty$ is returned instead.
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::sub_prec_round_ref_ref`] instead. If you know that your target precision is the
    /// maximum of the precisions of the two inputs, consider using `-` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (sum, o) = Float::from(PI).sub_prec_ref_ref(&Float::from(E), 5);
    /// assert_eq!(sum.to_string(), "0.42");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) = Float::from(PI).sub_prec_ref_ref(&Float::from(E), 20);
    /// assert_eq!(sum.to_string(), "0.4233108");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn sub_prec_ref_ref(&self, other: &Float, prec: u64) -> (Float, Ordering) {
        self.sub_prec_round_ref_ref(other, prec, Nearest)
    }

    /// Subtracts two [`Float`]s, rounding the result with the specified rounding mode. Both
    /// [`Float`]s are taken by value. An [`Ordering`] is also returned, indicating whether the
    /// rounded difference is less than, equal to, or greater than the exact difference. Although
    /// `NaN`s are not comparable to any [`Float`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// The precision of the output is the maximum of the precision of the inputs. See
    /// [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,m) = x-y+\varepsilon.
    /// $$
    /// - If $x-y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-y$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x-y|\rfloor-p+1}$, where $p$ is the maximum precision of the inputs.
    /// - If $x-y$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x-y|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,m)=f(x,\text{NaN},m)=f(\infty,\infty,m)=f(-\infty,-\infty,m)=
    ///     \text{NaN}$
    /// - $f(\infty,x,m)=\infty$ if $x$ is not NaN or $\infty$
    /// - $f(x,-\infty,m)=\infty$ if $x$ is not NaN or $-\infty$
    /// - $f(-\infty,x,m)=-\infty$ if $x$ is not NaN or $-\infty$
    /// - $f(x,\infty,m)=-\infty$ if $x$ is not NaN or $\infty$
    /// - $f(0.0,-0.0,m)=0.0$
    /// - $f(-0.0,0.0,m)=-0.0$
    /// - $f(0.0,0.0,m)=f(-0.0,-0.0,m)=0.0$ if $m$ is not `Floor`
    /// - $f(0.0,0.0,m)=f(-0.0,-0.0,m)=-0.0$ if $m$ is `Floor`
    /// - $f(x,x,m)=0.0$ if $x$ is finite and nonzero and $m$ is not `Floor`
    /// - $f(x,x,m)=-0.0$ if $x$ is finite and nonzero and $m$ is `Floor`
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,y,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$ is
    ///   returned instead, where `p` is the precision of the input.
    /// - If $f(x,y,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is
    ///   returned instead.
    /// - If $f(x,y,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`, $-(1-(1/2)^p)2^{2^{30}-1}$
    ///   is returned instead, where `p` is the precision of the input.
    /// - If $0<f(x,y,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,y,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,y,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is
    ///   returned instead.
    ///
    /// If you want to specify an output precision, consider using [`Float::sub_prec_round`]
    /// instead. If you know you'll be using the `Nearest` rounding mode, consider using `-`
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the maximum precision of the inputs is not high enough to
    /// represent the output.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (sum, o) = Float::from(PI).sub_round(Float::from(-E), Floor);
    /// assert_eq!(sum.to_string(), "5.859874482048838");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) = Float::from(PI).sub_round(Float::from(-E), Ceiling);
    /// assert_eq!(sum.to_string(), "5.859874482048839");
    /// assert_eq!(o, Greater);
    ///
    /// let (sum, o) = Float::from(PI).sub_round(Float::from(-E), Nearest);
    /// assert_eq!(sum.to_string(), "5.859874482048838");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn sub_round(self, other: Float, rm: RoundingMode) -> (Float, Ordering) {
        let prec = max(self.significant_bits(), other.significant_bits());
        Float::sub_prec_round(self, other, prec, rm)
    }

    /// Subtracts two [`Float`]s, rounding the result with the specified rounding mode. The
    /// [`Float`] is taken by value and the [`Rational`] by reference. An [`Ordering`] is also
    /// returned, indicating whether the rounded difference is less than, equal to, or greater than
    /// the exact difference. Although `NaN`s are not comparable to any [`Float`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the maximum of the precision of the inputs. See
    /// [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,m) = x-y+\varepsilon.
    /// $$
    /// - If $x-y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-y$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x-y|\rfloor-p+1}$, where $p$ is the maximum precision of the inputs.
    /// - If $x-y$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x-y|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,m)=f(x,\text{NaN},m)=f(\infty,\infty,m)=f(-\infty,-\infty,m)=
    ///     \text{NaN}$
    /// - $f(\infty,x,m)=\infty$ if $x$ is not NaN or $\infty$
    /// - $f(x,-\infty,m)=\infty$ if $x$ is not NaN or $-\infty$
    /// - $f(-\infty,x,m)=-\infty$ if $x$ is not NaN or $-\infty$
    /// - $f(x,\infty,m)=-\infty$ if $x$ is not NaN or $\infty$
    /// - $f(0.0,-0.0,m)=0.0$
    /// - $f(-0.0,0.0,m)=-0.0$
    /// - $f(0.0,0.0,m)=f(-0.0,-0.0,m)=0.0$ if $m$ is not `Floor`
    /// - $f(0.0,0.0,m)=f(-0.0,-0.0,m)=-0.0$ if $m$ is `Floor`
    /// - $f(x,x,m)=0.0$ if $x$ is finite and nonzero and $m$ is not `Floor`
    /// - $f(x,x,m)=-0.0$ if $x$ is finite and nonzero and $m$ is `Floor`
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,y,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$ is
    ///   returned instead, where `p` is the precision of the input.
    /// - If $f(x,y,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is
    ///   returned instead.
    /// - If $f(x,y,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`, $-(1-(1/2)^p)2^{2^{30}-1}$
    ///   is returned instead, where `p` is the precision of the input.
    /// - If $0<f(x,y,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,y,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,y,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is
    ///   returned instead.
    ///
    /// If you want to specify an output precision, consider using [`Float::sub_prec_round_val_ref`]
    /// instead. If you know you'll be using the `Nearest` rounding mode, consider using `-`
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`, and $m$ is `other.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the maximum precision of the inputs is not high enough to
    /// represent the output.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (sum, o) = Float::from(PI).sub_round_val_ref(&Float::from(-E), Floor);
    /// assert_eq!(sum.to_string(), "5.859874482048838");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) = Float::from(PI).sub_round_val_ref(&Float::from(-E), Ceiling);
    /// assert_eq!(sum.to_string(), "5.859874482048839");
    /// assert_eq!(o, Greater);
    ///
    /// let (sum, o) = Float::from(PI).sub_round_val_ref(&Float::from(-E), Nearest);
    /// assert_eq!(sum.to_string(), "5.859874482048838");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn sub_round_val_ref(self, other: &Float, rm: RoundingMode) -> (Float, Ordering) {
        let prec = max(self.significant_bits(), other.significant_bits());
        self.sub_prec_round_val_ref(other, prec, rm)
    }

    /// Subtracts two [`Float`]s, rounding the result with the specified rounding mode. The
    /// [`Float`] is taken by reference and the [`Rational`] by value. An [`Ordering`] is also
    /// returned, indicating whether the rounded difference is less than, equal to, or greater than
    /// the exact difference. Although `NaN`s are not comparable to any [`Float`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the maximum of the precision of the inputs. See
    /// [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,m) = x-y+\varepsilon.
    /// $$
    /// - If $x-y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-y$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x-y|\rfloor-p+1}$, where $p$ is the maximum precision of the inputs.
    /// - If $x-y$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x-y|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,m)=f(x,\text{NaN},m)=f(\infty,\infty,m)=f(-\infty,-\infty,m)=
    ///     \text{NaN}$
    /// - $f(\infty,x,m)=\infty$ if $x$ is not NaN or $\infty$
    /// - $f(x,-\infty,m)=\infty$ if $x$ is not NaN or $-\infty$
    /// - $f(-\infty,x,m)=-\infty$ if $x$ is not NaN or $-\infty$
    /// - $f(x,\infty,m)=-\infty$ if $x$ is not NaN or $\infty$
    /// - $f(0.0,-0.0,m)=0.0$
    /// - $f(-0.0,0.0,m)=-0.0$
    /// - $f(0.0,0.0,m)=f(-0.0,-0.0,m)=0.0$ if $m$ is not `Floor`
    /// - $f(0.0,0.0,m)=f(-0.0,-0.0,m)=-0.0$ if $m$ is `Floor`
    /// - $f(x,x,m)=0.0$ if $x$ is finite and nonzero and $m$ is not `Floor`
    /// - $f(x,x,m)=-0.0$ if $x$ is finite and nonzero and $m$ is `Floor`
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,y,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$ is
    ///   returned instead, where `p` is the precision of the input.
    /// - If $f(x,y,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is
    ///   returned instead.
    /// - If $f(x,y,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`, $-(1-(1/2)^p)2^{2^{30}-1}$
    ///   is returned instead, where `p` is the precision of the input.
    /// - If $0<f(x,y,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,y,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,y,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is
    ///   returned instead.
    ///
    /// If you want to specify an output precision, consider using [`Float::sub_prec_round_ref_val`]
    /// instead. If you know you'll be using the `Nearest` rounding mode, consider using `-`
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`, and $m$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the maximum precision of the inputs is not high enough to
    /// represent the output.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (sum, o) = Float::from(PI).sub_round_ref_val(Float::from(-E), Floor);
    /// assert_eq!(sum.to_string(), "5.859874482048838");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) = Float::from(PI).sub_round_ref_val(Float::from(-E), Ceiling);
    /// assert_eq!(sum.to_string(), "5.859874482048839");
    /// assert_eq!(o, Greater);
    ///
    /// let (sum, o) = Float::from(PI).sub_round_ref_val(Float::from(-E), Nearest);
    /// assert_eq!(sum.to_string(), "5.859874482048838");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn sub_round_ref_val(&self, other: Float, rm: RoundingMode) -> (Float, Ordering) {
        let prec = max(self.significant_bits(), other.significant_bits());
        self.sub_prec_round_ref_val(other, prec, rm)
    }

    /// Subtracts two [`Float`]s, rounding the result with the specified rounding mode. Both
    /// [`Float`]s are taken by reference. An [`Ordering`] is also returned, indicating whether the
    /// rounded difference is less than, equal to, or greater than the exact difference. Although
    /// `NaN`s are not comparable to any [`Float`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// The precision of the output is the maximum of the precision of the inputs. See
    /// [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,m) = x-y+\varepsilon.
    /// $$
    /// - If $x-y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-y$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x-y|\rfloor-p+1}$, where $p$ is the maximum precision of the inputs.
    /// - If $x-y$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x-y|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,m)=f(x,\text{NaN},m)=f(\infty,\infty,m)=f(-\infty,-\infty,m)=
    ///     \text{NaN}$
    /// - $f(\infty,x,m)=\infty$ if $x$ is not NaN or $\infty$
    /// - $f(x,-\infty,m)=\infty$ if $x$ is not NaN or $-\infty$
    /// - $f(-\infty,x,m)=-\infty$ if $x$ is not NaN or $-\infty$
    /// - $f(x,\infty,m)=-\infty$ if $x$ is not NaN or $\infty$
    /// - $f(0.0,-0.0,m)=0.0$
    /// - $f(-0.0,0.0,m)=-0.0$
    /// - $f(0.0,0.0,m)=f(-0.0,-0.0,m)=0.0$ if $m$ is not `Floor`
    /// - $f(0.0,0.0,m)=f(-0.0,-0.0,m)=-0.0$ if $m$ is `Floor`
    /// - $f(x,x,m)=0.0$ if $x$ is finite and nonzero and $m$ is not `Floor`
    /// - $f(x,x,m)=-0.0$ if $x$ is finite and nonzero and $m$ is `Floor`
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,y,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$ is
    ///   returned instead, where `p` is the precision of the input.
    /// - If $f(x,y,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is
    ///   returned instead.
    /// - If $f(x,y,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`, $-(1-(1/2)^p)2^{2^{30}-1}$
    ///   is returned instead, where `p` is the precision of the input.
    /// - If $0<f(x,y,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,y,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,y,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is
    ///   returned instead.
    ///
    /// If you want to specify an output precision, consider using [`Float::sub_prec_round_ref_ref`]
    /// instead. If you know you'll be using the `Nearest` rounding mode, consider using `-`
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the maximum precision of the inputs is not high enough to
    /// represent the output.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (sum, o) = Float::from(PI).sub_round_ref_ref(&Float::from(-E), Floor);
    /// assert_eq!(sum.to_string(), "5.859874482048838");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) = Float::from(PI).sub_round_ref_ref(&Float::from(-E), Ceiling);
    /// assert_eq!(sum.to_string(), "5.859874482048839");
    /// assert_eq!(o, Greater);
    ///
    /// let (sum, o) = Float::from(PI).sub_round_ref_ref(&Float::from(-E), Nearest);
    /// assert_eq!(sum.to_string(), "5.859874482048838");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn sub_round_ref_ref(&self, other: &Float, rm: RoundingMode) -> (Float, Ordering) {
        let prec = max(self.significant_bits(), other.significant_bits());
        self.sub_prec_round_ref_ref(other, prec, rm)
    }

    /// Subtracts a [`Float`] by a [`Float`] in place, rounding the result to the specified
    /// precision and with the specified rounding mode. The [`Float`] on the right-hand side is
    /// taken by value. An [`Ordering`] is returned, indicating whether the rounded difference is
    /// less than, equal to, or greater than the exact difference. Although `NaN`s are not
    /// comparable to any [`Float`], whenever this function sets the [`Float`] to `NaN` it also
    /// returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets x-y+\varepsilon.
    /// $$
    /// - If $x-y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-y$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x-y|\rfloor-p+1}$.
    /// - If $x-y$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x-y|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// See the [`Float::sub_prec_round`] documentation for information on special cases, overflow,
    /// and underflow.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::sub_prec_assign`] instead. If
    /// you know that your target precision is the maximum of the precisions of the two inputs,
    /// consider using [`Float::sub_round_assign`] instead. If both of these things are true,
    /// consider using `-=` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but `prec` is too small for an exact subtraction.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.sub_prec_round_assign(Float::from(E), 5, Floor), Less);
    /// assert_eq!(x.to_string(), "0.42");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.sub_prec_round_assign(Float::from(E), 5, Ceiling), Greater);
    /// assert_eq!(x.to_string(), "0.44");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.sub_prec_round_assign(Float::from(E), 5, Nearest), Less);
    /// assert_eq!(x.to_string(), "0.42");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.sub_prec_round_assign(Float::from(E), 20, Floor), Less);
    /// assert_eq!(x.to_string(), "0.4233108");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.sub_prec_round_assign(Float::from(E), 20, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "0.4233112");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.sub_prec_round_assign(Float::from(E), 20, Nearest), Less);
    /// assert_eq!(x.to_string(), "0.4233108");
    /// ```
    #[inline]
    pub fn sub_prec_round_assign(&mut self, other: Float, prec: u64, rm: RoundingMode) -> Ordering {
        self.add_prec_round_assign_helper(other, prec, rm, true)
    }

    /// Subtracts a [`Float`] by a [`Float`] in place, rounding the result to the specified
    /// precision and with the specified rounding mode. The [`Float`] on the right-hand side is
    /// taken by reference. An [`Ordering`] is returned, indicating whether the rounded difference
    /// is less than, equal to, or greater than the exact difference. Although `NaN`s are not
    /// comparable to any [`Float`], whenever this function sets the [`Float`] to `NaN` it also
    /// returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets x-y+\varepsilon.
    /// $$
    /// - If $x-y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-y$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x-y|\rfloor-p+1}$.
    /// - If $x-y$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x-y|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// See the [`Float::sub_prec_round`] documentation for information on special cases, overflow,
    /// and underflow.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::sub_prec_assign_ref`]
    /// instead. If you know that your target precision is the maximum of the precisions of the two
    /// inputs, consider using [`Float::sub_round_assign`] instead. If both of these things are
    /// true, consider using `-=` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but `prec` is too small for an exact subtraction.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.sub_prec_round_assign_ref(&Float::from(E), 5, Floor), Less);
    /// assert_eq!(x.to_string(), "0.42");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.sub_prec_round_assign_ref(&Float::from(E), 5, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "0.44");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.sub_prec_round_assign_ref(&Float::from(E), 5, Nearest),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "0.42");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.sub_prec_round_assign_ref(&Float::from(E), 20, Floor),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "0.4233108");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.sub_prec_round_assign_ref(&Float::from(E), 20, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "0.4233112");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.sub_prec_round_assign_ref(&Float::from(E), 20, Nearest),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "0.4233108");
    /// ```
    #[inline]
    pub fn sub_prec_round_assign_ref(
        &mut self,
        other: &Float,
        prec: u64,
        rm: RoundingMode,
    ) -> Ordering {
        self.add_prec_round_assign_ref_helper(other, prec, rm, true)
    }

    /// Subtracts a [`Float`] by a [`Float`] in place, rounding the result to the nearest value of
    /// the specified precision. The [`Float`] on the right-hand side is taken by value. An
    /// [`Ordering`] is returned, indicating whether the rounded difference is less than, equal to,
    /// or greater than the exact difference. Although `NaN`s are not comparable to any [`Float`],
    /// whenever this function sets the [`Float`] to `NaN` it also returns `Equal`.
    ///
    /// If the difference is equidistant from two [`Float`]s with the specified precision, the
    /// [`Float`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a
    /// description of the `Nearest` rounding mode.
    ///
    /// $$
    /// x \gets x-y+\varepsilon.
    /// $$
    /// - If $x-y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-y$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2 |x-y|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// See the [`Float::sub_prec`] documentation for information on special cases, overflow, and
    /// underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::sub_prec_round_assign`] instead. If you know that your target precision is the
    /// maximum of the precisions of the two inputs, consider using `-=` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.sub_prec_assign(Float::from(E), 5), Less);
    /// assert_eq!(x.to_string(), "0.42");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.sub_prec_assign(Float::from(E), 20), Less);
    /// assert_eq!(x.to_string(), "0.4233108");
    /// ```
    #[inline]
    pub fn sub_prec_assign(&mut self, other: Float, prec: u64) -> Ordering {
        self.sub_prec_round_assign(other, prec, Nearest)
    }

    /// Subtracts a [`Float`] by a [`Float`] in place, rounding the result to the nearest value of
    /// the specified precision. The [`Float`] on the right-hand side is taken by reference. An
    /// [`Ordering`] is returned, indicating whether the rounded difference is less than, equal to,
    /// or greater than the exact difference. Although `NaN`s are not comparable to any [`Float`],
    /// whenever this function sets the [`Float`] to `NaN` it also returns `Equal`.
    ///
    /// If the difference is equidistant from two [`Float`]s with the specified precision, the
    /// [`Float`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a
    /// description of the `Nearest` rounding mode.
    ///
    /// $$
    /// x \gets x-y+\varepsilon.
    /// $$
    /// - If $x-y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-y$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2 |x-y|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// See the [`Float::sub_prec`] documentation for information on special cases, overflow, and
    /// underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::sub_prec_round_assign_ref`] instead. If you know that your target precision is the
    /// maximum of the precisions of the two inputs, consider using `-=` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.sub_prec_assign_ref(&Float::from(E), 5), Less);
    /// assert_eq!(x.to_string(), "0.42");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.sub_prec_assign_ref(&Float::from(E), 20), Less);
    /// assert_eq!(x.to_string(), "0.4233108");
    /// ```
    #[inline]
    pub fn sub_prec_assign_ref(&mut self, other: &Float, prec: u64) -> Ordering {
        self.sub_prec_round_assign_ref(other, prec, Nearest)
    }

    /// Subtracts a [`Float`] by a [`Float`] in place, rounding the result with the specified
    /// rounding mode. The [`Float`] on the right-hand side is taken by value. An [`Ordering`] is
    /// returned, indicating whether the rounded difference is less than, equal to, or greater than
    /// the exact difference. Although `NaN`s are not comparable to any [`Float`], whenever this
    /// function sets the [`Float`] to `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the maximum of the precision of the inputs. See
    /// [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets x-y+\varepsilon.
    /// $$
    /// - If $x-y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-y$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x-y|\rfloor-p+1}$, where $p$ is the maximum precision of the inputs.
    /// - If $x-y$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x-y|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs.
    ///
    /// See the [`Float::sub_round`] documentation for information on special cases, overflow, and
    /// underflow.
    ///
    /// If you want to specify an output precision, consider using [`Float::sub_prec_round_assign`]
    /// instead. If you know you'll be using the `Nearest` rounding mode, consider using `-=`
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the maximum precision of the inputs is not high enough to
    /// represent the output.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.sub_round_assign(Float::from(-E), Floor), Less);
    /// assert_eq!(x.to_string(), "5.859874482048838");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.sub_round_assign(Float::from(-E), Ceiling), Greater);
    /// assert_eq!(x.to_string(), "5.859874482048839");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.sub_round_assign(Float::from(-E), Nearest), Less);
    /// assert_eq!(x.to_string(), "5.859874482048838");
    /// ```
    #[inline]
    pub fn sub_round_assign(&mut self, other: Float, rm: RoundingMode) -> Ordering {
        let prec = max(self.significant_bits(), other.significant_bits());
        self.sub_prec_round_assign(other, prec, rm)
    }

    /// Subtracts a [`Float`] by a [`Float`] in place, rounding the result with the specified
    /// rounding mode. The [`Float`] on the right-hand side is taken by reference. An [`Ordering`]
    /// is returned, indicating whether the rounded difference is less than, equal to, or greater
    /// than the exact difference. Although `NaN`s are not comparable to any [`Float`], whenever
    /// this function sets the [`Float`] to `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the maximum of the precision of the inputs. See
    /// [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets x-y+\varepsilon.
    /// $$
    /// - If $x-y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-y$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x-y|\rfloor-p+1}$, where $p$ is the maximum precision of the inputs.
    /// - If $x-y$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x-y|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs.
    ///
    /// See the [`Float::sub_round`] documentation for information on special cases, overflow, and
    /// underflow.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::sub_prec_round_assign_ref`] instead. If you know you'll be using the `Nearest`
    /// rounding mode, consider using `-=` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`, and $m$ is `other.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the maximum precision of the inputs is not high enough to
    /// represent the output.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.sub_round_assign_ref(&Float::from(-E), Floor), Less);
    /// assert_eq!(x.to_string(), "5.859874482048838");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.sub_round_assign_ref(&Float::from(-E), Ceiling), Greater);
    /// assert_eq!(x.to_string(), "5.859874482048839");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.sub_round_assign_ref(&Float::from(-E), Nearest), Less);
    /// assert_eq!(x.to_string(), "5.859874482048838");
    /// ```
    #[inline]
    pub fn sub_round_assign_ref(&mut self, other: &Float, rm: RoundingMode) -> Ordering {
        let prec = max(self.significant_bits(), other.significant_bits());
        self.sub_prec_round_assign_ref(other, prec, rm)
    }

    /// Subtracts a [`Float`] by a [`Rational`], rounding the result to the specified precision and
    /// with the specified rounding mode. The [`Float`] and the [`Rational`] are both taken by
    /// value. An [`Ordering`] is also returned, indicating whether the rounded difference is less
    /// than, equal to, or greater than the exact difference. Although `NaN`s are not comparable to
    /// any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,p,m) = x-y+\varepsilon.
    /// $$
    /// - If $x-y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-y$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x-y|\rfloor-p+1}$.
    /// - If $x-y$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x-y|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,p,m)=\text{NaN}$
    /// - $f(\infty,x,p,m)=\infty$
    /// - $f(-\infty,x,p,m)=-\infty$
    /// - $f(0.0,0,p,m)=0.0$
    /// - $f(-0.0,0,p,m)=-0.0$
    /// - $f(x,x,p,m)=0.0$ if $x$ is nonzero and $m$ is not `Floor`
    /// - $f(x,x,p,m)=-0.0$ if $x$ is nonzero and $m$ is `Floor`
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,y,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$
    ///   is returned instead, where `p` is the precision of the input.
    /// - If $f(x,y,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is
    ///   returned instead.
    /// - If $f(x,y,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`,
    ///   $-(1-(1/2)^p)2^{2^{30}-1}$ is returned instead, where `p` is the precision of the input.
    /// - If $0<f(x,y,p,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,y,p,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,y,p,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,p,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,p,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is
    ///   returned instead.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::sub_rational_prec`] instead.
    /// If you know that your target precision is the precision of the [`Float`] input, consider
    /// using [`Float::sub_rational_round`] instead. If both of these things are true, consider
    /// using `-` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(other.significant_bits(),
    /// prec)`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but `prec` is too small for an exact subtraction.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (sum, o) =
    ///     Float::from(PI).sub_rational_prec_round(Rational::from_unsigneds(1u8, 3), 5, Floor);
    /// assert_eq!(sum.to_string(), "2.8");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) =
    ///     Float::from(PI).sub_rational_prec_round(Rational::from_unsigneds(1u8, 3), 5, Ceiling);
    /// assert_eq!(sum.to_string(), "2.9");
    /// assert_eq!(o, Greater);
    ///
    /// let (sum, o) =
    ///     Float::from(PI).sub_rational_prec_round(Rational::from_unsigneds(1u8, 3), 5, Nearest);
    /// assert_eq!(sum.to_string(), "2.8");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) =
    ///     Float::from(PI).sub_rational_prec_round(Rational::from_unsigneds(1u8, 3), 20, Floor);
    /// assert_eq!(sum.to_string(), "2.808258");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) =
    ///     Float::from(PI).sub_rational_prec_round(Rational::from_unsigneds(1u8, 3), 20, Ceiling);
    /// assert_eq!(sum.to_string(), "2.808262");
    /// assert_eq!(o, Greater);
    ///
    /// let (sum, o) =
    ///     Float::from(PI).sub_rational_prec_round(Rational::from_unsigneds(1u8, 3), 20, Nearest);
    /// assert_eq!(sum.to_string(), "2.808258");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn sub_rational_prec_round(
        mut self,
        other: Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Float, Ordering) {
        let o = self.sub_rational_prec_round_assign(other, prec, rm);
        (self, o)
    }

    /// Subtracts a [`Float`] by a [`Rational`], rounding the result to the specified precision and
    /// with the specified rounding mode. The [`Float`] is taken by value and the [`Rational`] by
    /// reference. An [`Ordering`] is also returned, indicating whether the rounded difference is
    /// less than, equal to, or greater than the exact difference. Although `NaN`s are not
    /// comparable to any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,p,m) = x-y+\varepsilon.
    /// $$
    /// - If $x-y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-y$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x-y|\rfloor-p+1}$.
    /// - If $x-y$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x-y|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,p,m)=\text{NaN}$
    /// - $f(\infty,x,p,m)=\infty$
    /// - $f(-\infty,x,p,m)=-\infty$
    /// - $f(0.0,0,p,m)=0.0$
    /// - $f(-0.0,0,p,m)=-0.0$
    /// - $f(x,x,p,m)=0.0$ if $x$ is nonzero and $m$ is not `Floor`
    /// - $f(x,x,p,m)=-0.0$ if $x$ is nonzero and $m$ is `Floor`
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,y,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$
    ///   is returned instead, where `p` is the precision of the input.
    /// - If $f(x,y,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is
    ///   returned instead.
    /// - If $f(x,y,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`,
    ///   $-(1-(1/2)^p)2^{2^{30}-1}$ is returned instead, where `p` is the precision of the input.
    /// - If $0<f(x,y,p,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,y,p,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,y,p,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,p,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,p,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is
    ///   returned instead.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::sub_rational_prec_val_ref`]
    /// instead. If you know that your target precision is the precision of the [`Float`] input,
    /// consider using [`Float::sub_rational_round_val_ref`] instead. If both of these things are
    /// true, consider using `-` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(other.significant_bits(),
    /// prec)`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but `prec` is too small for an exact subtraction.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (sum, o) = Float::from(PI).sub_rational_prec_round_val_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(sum.to_string(), "2.8");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) = Float::from(PI).sub_rational_prec_round_val_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(sum.to_string(), "2.9");
    /// assert_eq!(o, Greater);
    ///
    /// let (sum, o) = Float::from(PI).sub_rational_prec_round_val_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(sum.to_string(), "2.8");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) = Float::from(PI).sub_rational_prec_round_val_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(sum.to_string(), "2.808258");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) = Float::from(PI).sub_rational_prec_round_val_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(sum.to_string(), "2.808262");
    /// assert_eq!(o, Greater);
    ///
    /// let (sum, o) = Float::from(PI).sub_rational_prec_round_val_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(sum.to_string(), "2.808258");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn sub_rational_prec_round_val_ref(
        mut self,
        other: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Float, Ordering) {
        let o = self.sub_rational_prec_round_assign_ref(other, prec, rm);
        (self, o)
    }

    /// Subtracts a [`Float`] by a [`Rational`], rounding the result to the specified precision and
    /// with the specified rounding mode. The [`Float`] is taken by reference and the [`Rational`]
    /// by value. An [`Ordering`] is also returned, indicating whether the rounded difference is
    /// less than, equal to, or greater than the exact difference. Although `NaN`s are not
    /// comparable to any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,p,m) = x-y+\varepsilon.
    /// $$
    /// - If $x-y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-y$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x-y|\rfloor-p+1}$.
    /// - If $x-y$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x-y|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,p,m)=\text{NaN}$
    /// - $f(\infty,x,p,m)=\infty$
    /// - $f(-\infty,x,p,m)=-\infty$
    /// - $f(0.0,0,p,m)=0.0$
    /// - $f(-0.0,0,p,m)=-0.0$
    /// - $f(x,x,p,m)=0.0$ if $x$ is nonzero and $m$ is not `Floor`
    /// - $f(x,x,p,m)=-0.0$ if $x$ is nonzero and $m$ is `Floor`
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,y,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$
    ///   is returned instead, where `p` is the precision of the input.
    /// - If $f(x,y,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is
    ///   returned instead.
    /// - If $f(x,y,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`,
    ///   $-(1-(1/2)^p)2^{2^{30}-1}$ is returned instead, where `p` is the precision of the input.
    /// - If $0<f(x,y,p,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,y,p,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,y,p,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,p,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,p,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is
    ///   returned instead.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::sub_rational_prec_ref_val`]
    /// instead. If you know that your target precision is the precision of the [`Float`] input,
    /// consider using [`Float::sub_rational_round_ref_val`] instead. If both of these things are
    /// true, consider using `-` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(other.significant_bits(),
    /// prec)`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but `prec` is too small for an exact subtraction.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (sum, o) = Float::from(PI).sub_rational_prec_round_ref_val(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(sum.to_string(), "2.8");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) = Float::from(PI).sub_rational_prec_round_ref_val(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(sum.to_string(), "2.9");
    /// assert_eq!(o, Greater);
    ///
    /// let (sum, o) = Float::from(PI).sub_rational_prec_round_ref_val(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(sum.to_string(), "2.8");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) = Float::from(PI).sub_rational_prec_round_ref_val(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(sum.to_string(), "2.808258");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) = Float::from(PI).sub_rational_prec_round_ref_val(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(sum.to_string(), "2.808262");
    /// assert_eq!(o, Greater);
    ///
    /// let (sum, o) = Float::from(PI).sub_rational_prec_round_ref_val(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(sum.to_string(), "2.808258");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn sub_rational_prec_round_ref_val(
        &self,
        other: Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Float, Ordering) {
        assert_ne!(prec, 0);
        match (self, other) {
            (float_nan!(), _) => (float_nan!(), Equal),
            (float_infinity!(), _) => (float_infinity!(), Equal),
            (float_negative_infinity!(), _) => (float_negative_infinity!(), Equal),
            (float_negative_zero!(), y) => {
                let (diff, o) = Float::from_rational_prec_round(y, prec, -rm);
                (-diff, o.reverse())
            }
            (float_zero!(), y) => {
                if y == 0u32 {
                    (float_zero!(), Equal)
                } else {
                    let (diff, o) = Float::from_rational_prec_round(y, prec, -rm);
                    (-diff, o.reverse())
                }
            }
            (_, y) if y == 0 => Float::from_float_prec_round_ref(self, prec, rm),
            (x, y) => {
                if *x == y {
                    return (
                        if rm == Floor {
                            float_negative_zero!()
                        } else {
                            float_zero!()
                        },
                        Equal,
                    );
                }
                let (min_exponent, max_exponent) = float_rational_diff_exponent_range(x, &y);
                if min_exponent >= i64::from(Float::MAX_EXPONENT) {
                    assert!(rm != Exact, "Inexact Float subtraction");
                    return match (float_rational_diff_sign(x, &y), rm) {
                        (true, Ceiling | Up | Nearest) => (float_infinity!(), Greater),
                        (true, _) => (Float::max_finite_value_with_prec(prec), Less),
                        (false, Floor | Up | Nearest) => (float_negative_infinity!(), Less),
                        (false, _) => (-Float::max_finite_value_with_prec(prec), Greater),
                    };
                }
                if max_exponent > i64::from(Float::MAX_EXPONENT) - 2
                    || min_exponent < i64::from(Float::MIN_EXPONENT - 2)
                {
                    // If we can't rule out overflow or underflow, use slow-but-correct naive
                    // algorithm.
                    return sub_rational_prec_round_naive_ref_val(x, y, prec, rm);
                }
                let mut working_prec = prec + 10;
                let mut increment = Limb::WIDTH;
                // working_prec grows as O([(1 + sqrt(3)) / 2] ^ n) ≈ O(1.366 ^ n).
                loop {
                    // Error <= 1/2 ulp(q)
                    let (q, o) = Float::from_rational_prec_ref(&y, working_prec);
                    if o == Equal {
                        // Result is exact so we can subtract it directly!
                        return self.sub_prec_round_ref_val(q, prec, rm);
                    }
                    let q_exp = q.get_exponent().unwrap();
                    let mut t = x.sub_prec_ref_val(q, working_prec).0;
                    // Error on t is <= 1/2 ulp(t).
                    // ```
                    // Error / ulp(t)      <= 1/2 + 1/2 * 2^(EXP(q)-EXP(t))
                    // If EXP(q)-EXP(t)>0, <= 2^(EXP(q)-EXP(t)-1)*(1+2^-(EXP(q)-EXP(t)))
                    //                     <= 2^(EXP(q)-EXP(t))
                    // If EXP(q)-EXP(t)<0, <= 2^0
                    // ```
                    // We can get 0, but we can't round since q is inexact
                    if t != 0 {
                        let m = u64::saturating_from(q_exp - t.get_exponent().unwrap())
                            .checked_add(1)
                            .unwrap();
                        if working_prec >= m
                            && float_can_round(
                                t.significand_ref().unwrap(),
                                working_prec - m,
                                prec,
                                rm,
                            )
                        {
                            let o = t.set_prec_round(prec, rm);
                            return (t, o);
                        }
                    }
                    working_prec += increment;
                    increment = working_prec >> 1;
                }
            }
        }
    }

    /// Subtracts a [`Float`] by a [`Rational`], rounding the result to the specified precision and
    /// with the specified rounding mode. The [`Float`] and the [`Rational`] are both taken by
    /// reference. An [`Ordering`] is also returned, indicating whether the rounded difference is
    /// less than, equal to, or greater than the exact difference. Although `NaN`s are not
    /// comparable to any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,p,m) = x-y+\varepsilon.
    /// $$
    /// - If $x-y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-y$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x-y|\rfloor-p+1}$.
    /// - If $x-y$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x-y|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,p,m)=\text{NaN}$
    /// - $f(\infty,x,p,m)=\infty$
    /// - $f(-\infty,x,p,m)=-\infty$
    /// - $f(0.0,0,p,m)=0.0$
    /// - $f(-0.0,0,p,m)=-0.0$
    /// - $f(x,x,p,m)=0.0$ if $x$ is nonzero and $m$ is not `Floor`
    /// - $f(x,x,p,m)=-0.0$ if $x$ is nonzero and $m$ is `Floor`
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,y,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$
    ///   is returned instead, where `p` is the precision of the input.
    /// - If $f(x,y,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is
    ///   returned instead.
    /// - If $f(x,y,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`,
    ///   $-(1-(1/2)^p)2^{2^{30}-1}$ is returned instead, where `p` is the precision of the input.
    /// - If $0<f(x,y,p,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,y,p,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,y,p,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,p,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,p,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is
    ///   returned instead.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::sub_rational_prec_ref_ref`]
    /// instead. If you know that your target precision is the precision of the [`Float`] input,
    /// consider using [`Float::sub_rational_round_ref_ref`] instead. If both of these things are
    /// true, consider using `-` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(other.significant_bits(),
    /// prec)`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but `prec` is too small for an exact subtraction.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (sum, o) = Float::from(PI).sub_rational_prec_round_ref_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(sum.to_string(), "2.8");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) = Float::from(PI).sub_rational_prec_round_ref_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(sum.to_string(), "2.9");
    /// assert_eq!(o, Greater);
    ///
    /// let (sum, o) = Float::from(PI).sub_rational_prec_round_ref_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(sum.to_string(), "2.8");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) = Float::from(PI).sub_rational_prec_round_ref_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(sum.to_string(), "2.808258");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) = Float::from(PI).sub_rational_prec_round_ref_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(sum.to_string(), "2.808262");
    /// assert_eq!(o, Greater);
    ///
    /// let (sum, o) = Float::from(PI).sub_rational_prec_round_ref_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(sum.to_string(), "2.808258");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn sub_rational_prec_round_ref_ref(
        &self,
        other: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Float, Ordering) {
        assert_ne!(prec, 0);
        match (self, other) {
            (float_nan!(), _) => (float_nan!(), Equal),
            (float_infinity!(), _) => (float_infinity!(), Equal),
            (float_negative_infinity!(), _) => (float_negative_infinity!(), Equal),
            (float_negative_zero!(), y) => {
                let (diff, o) = Float::from_rational_prec_round_ref(y, prec, -rm);
                (-diff, o.reverse())
            }
            (float_zero!(), y) => {
                if *y == 0u32 {
                    (float_zero!(), Equal)
                } else {
                    let (diff, o) = Float::from_rational_prec_round_ref(y, prec, -rm);
                    (-diff, o.reverse())
                }
            }
            (_, y) if *y == 0 => Float::from_float_prec_round_ref(self, prec, rm),
            (x, y) => {
                if x == y {
                    return (
                        if rm == Floor {
                            float_negative_zero!()
                        } else {
                            float_zero!()
                        },
                        Equal,
                    );
                }
                let (min_exponent, max_exponent) = float_rational_diff_exponent_range(x, y);
                if min_exponent >= i64::from(Float::MAX_EXPONENT) {
                    assert!(rm != Exact, "Inexact Float subtraction");
                    return match (float_rational_diff_sign(x, y), rm) {
                        (true, Ceiling | Up | Nearest) => (float_infinity!(), Greater),
                        (true, _) => (Float::max_finite_value_with_prec(prec), Less),
                        (false, Floor | Up | Nearest) => (float_negative_infinity!(), Less),
                        (false, _) => (-Float::max_finite_value_with_prec(prec), Greater),
                    };
                }
                if max_exponent > i64::from(Float::MAX_EXPONENT) - 2
                    || min_exponent < i64::from(Float::MIN_EXPONENT - 2)
                {
                    // If we can't rule out overflow or underflow, use slow-but-correct naive
                    // algorithm.
                    return sub_rational_prec_round_naive_ref_ref(x, y, prec, rm);
                }
                let mut working_prec = prec + 10;
                let mut increment = Limb::WIDTH;
                // working_prec grows as O([(1 + sqrt(3)) / 2] ^ n) ≈ O(1.366 ^ n).
                loop {
                    // Error <= 1/2 ulp(q)
                    let (q, o) = Float::from_rational_prec_ref(y, working_prec);
                    if o == Equal {
                        // Result is exact so we can subtract it directly!
                        return self.sub_prec_round_ref_val(q, prec, rm);
                    }
                    let q_exp = q.get_exponent().unwrap();
                    let mut t = x.sub_prec_ref_val(q, working_prec).0;
                    // Error on t is <= 1/2 ulp(t).
                    // ```
                    // Error / ulp(t)      <= 1/2 + 1/2 * 2^(EXP(q)-EXP(t))
                    // If EXP(q)-EXP(t)>0, <= 2^(EXP(q)-EXP(t)-1)*(1+2^-(EXP(q)-EXP(t)))
                    //                     <= 2^(EXP(q)-EXP(t))
                    // If EXP(q)-EXP(t)<0, <= 2^0
                    // ```
                    // We can get 0, but we can't round since q is inexact
                    if t != 0 {
                        let m = u64::saturating_from(q_exp - t.get_exponent().unwrap())
                            .checked_add(1)
                            .unwrap();
                        if working_prec >= m
                            && float_can_round(
                                t.significand_ref().unwrap(),
                                working_prec - m,
                                prec,
                                rm,
                            )
                        {
                            let o = t.set_prec_round(prec, rm);
                            return (t, o);
                        }
                    }
                    working_prec += increment;
                    increment = working_prec >> 1;
                }
            }
        }
    }

    /// Subtracts a [`Float`] by a [`Rational`], rounding the result to the nearest value of the
    /// specified precision. The [`Float`] and the [`Rational`] are both are taken by value. An
    /// [`Ordering`] is also returned, indicating whether the rounded difference is less than, equal
    /// to, or greater than the exact difference. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// If the difference is equidistant from two [`Float`]s with the specified precision, the
    /// [`Float`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a
    /// description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,y,p) = x-y+\varepsilon.
    /// $$
    /// - If $x-y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-y$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2 |x-y|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,p)=\text{NaN}$
    /// - $f(\infty,x,p)=\infty$
    /// - $f(-\infty,x,p)=-\infty$
    /// - $f(0.0,0,p)=0.0$
    /// - $f(-0.0,0,p)=-0.0$
    /// - $f(x,x,p)=0.0$ if $x$ is nonzero
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,p)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,y,p)\geq 2^{2^{30}-1}$, $-\infty$ is returned instead.
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::sub_rational_prec_round`] instead. If you know that your target precision is the
    /// precision of the [`Float`] input, consider using `-` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(other.significant_bits(),
    /// prec)`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::num::conversion::traits::ExactFrom;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (sum, o) = Float::from(PI).sub_rational_prec(Rational::exact_from(1.5), 5);
    /// assert_eq!(sum.to_string(), "1.62");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) = Float::from(PI).sub_rational_prec(Rational::exact_from(1.5), 20);
    /// assert_eq!(sum.to_string(), "1.641592");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn sub_rational_prec(self, other: Rational, prec: u64) -> (Float, Ordering) {
        self.sub_rational_prec_round(other, prec, Nearest)
    }

    /// Subtracts a [`Float`] by a [`Rational`], rounding the result to the nearest value of the
    /// specified precision. The [`Float`] is taken by value and the [`Rational`] by reference. An
    /// [`Ordering`] is also returned, indicating whether the rounded difference is less than, equal
    /// to, or greater than the exact difference. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// If the difference is equidistant from two [`Float`]s with the specified precision, the
    /// [`Float`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a
    /// description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,y,p) = x-y+\varepsilon.
    /// $$
    /// - If $x-y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-y$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2 |x-y|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,p)=\text{NaN}$
    /// - $f(\infty,x,p)=\infty$
    /// - $f(-\infty,x,p)=-\infty$
    /// - $f(0.0,0,p)=0.0$
    /// - $f(-0.0,0,p)=-0.0$
    /// - $f(x,x,p)=0.0$ if $x$ is nonzero
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,p)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,y,p)\geq 2^{2^{30}-1}$, $-\infty$ is returned instead.
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::sub_rational_prec_round_val_ref`] instead. If you know that your target precision
    /// is the precision of the [`Float`] input, consider using `-` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(other.significant_bits(),
    /// prec)`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::num::conversion::traits::ExactFrom;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (sum, o) = Float::from(PI).sub_rational_prec_val_ref(&Rational::exact_from(1.5), 5);
    /// assert_eq!(sum.to_string(), "1.62");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) = Float::from(PI).sub_rational_prec_val_ref(&Rational::exact_from(1.5), 20);
    /// assert_eq!(sum.to_string(), "1.641592");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn sub_rational_prec_val_ref(self, other: &Rational, prec: u64) -> (Float, Ordering) {
        self.sub_rational_prec_round_val_ref(other, prec, Nearest)
    }

    /// Subtracts a [`Float`] by a [`Rational`], rounding the result to the nearest value of the
    /// specified precision. The [`Float`] is taken by reference and the [`Rational`] by value. An
    /// [`Ordering`] is also returned, indicating whether the rounded difference is less than, equal
    /// to, or greater than the exact difference. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// If the difference is equidistant from two [`Float`]s with the specified precision, the
    /// [`Float`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a
    /// description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,y,p) = x-y+\varepsilon.
    /// $$
    /// - If $x-y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-y$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2 |x-y|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,p)=\text{NaN}$
    /// - $f(\infty,x,p)=\infty$
    /// - $f(-\infty,x,p)=-\infty$
    /// - $f(0.0,0,p)=0.0$
    /// - $f(-0.0,0,p)=-0.0$
    /// - $f(x,x,p)=0.0$ if $x$ is nonzero
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,p)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,y,p)\geq 2^{2^{30}-1}$, $-\infty$ is returned instead.
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::sub_rational_prec_round_ref_val`] instead. If you know that your target precision
    /// is the precision of the [`Float`] input, consider using `-` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(other.significant_bits(),
    /// prec)`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::num::conversion::traits::ExactFrom;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (sum, o) = Float::from(PI).sub_rational_prec_ref_val(Rational::exact_from(1.5), 5);
    /// assert_eq!(sum.to_string(), "1.62");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) = Float::from(PI).sub_rational_prec_ref_val(Rational::exact_from(1.5), 20);
    /// assert_eq!(sum.to_string(), "1.641592");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn sub_rational_prec_ref_val(&self, other: Rational, prec: u64) -> (Float, Ordering) {
        self.sub_rational_prec_round_ref_val(other, prec, Nearest)
    }

    /// Subtracts a [`Float`] by a [`Rational`], rounding the result to the nearest value of the
    /// specified precision. The [`Float`] and the [`Rational`] are both are taken by reference. An
    /// [`Ordering`] is also returned, indicating whether the rounded difference is less than, equal
    /// to, or greater than the exact difference. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// If the difference is equidistant from two [`Float`]s with the specified precision, the
    /// [`Float`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a
    /// description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,y,p) = x-y+\varepsilon.
    /// $$
    /// - If $x-y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-y$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2 |x-y|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,p)=\text{NaN}$
    /// - $f(\infty,x,p)=\infty$
    /// - $f(-\infty,x,p)=-\infty$
    /// - $f(0.0,0,p)=0.0$
    /// - $f(-0.0,0,p)=-0.0$
    /// - $f(x,x,p)=0.0$ if $x$ is nonzero
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,p)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,y,p)\geq 2^{2^{30}-1}$, $-\infty$ is returned instead.
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::sub_rational_prec_round_ref_ref`] instead. If you know that your target precision
    /// is the precision of the [`Float`] input, consider using `-` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(other.significant_bits(),
    /// prec)`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::num::conversion::traits::ExactFrom;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (sum, o) = Float::from(PI).sub_rational_prec_ref_ref(&Rational::exact_from(1.5), 5);
    /// assert_eq!(sum.to_string(), "1.62");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) = Float::from(PI).sub_rational_prec_ref_ref(&Rational::exact_from(1.5), 20);
    /// assert_eq!(sum.to_string(), "1.641592");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn sub_rational_prec_ref_ref(&self, other: &Rational, prec: u64) -> (Float, Ordering) {
        self.sub_rational_prec_round_ref_ref(other, prec, Nearest)
    }

    /// Subtracts a [`Float`] by a [`Rational`], rounding the result with the specified rounding
    /// mode. The [`Float`] and the [`Rational`] are both are taken by value. An [`Ordering`] is
    /// also returned, indicating whether the rounded difference is less than, equal to, or greater
    /// than the exact difference. Although `NaN`s are not comparable to any [`Float`], whenever
    /// this function returns a `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the precision of the [`Float`] input. See [`RoundingMode`]
    /// for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,m) = x-y+\varepsilon.
    /// $$
    /// - If $x-y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-y$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x-y|\rfloor-p+1}$, where $p$ is the precision of the input [`Float`].
    /// - If $x-y$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x-y|\rfloor-p}$, where $p$ is the precision of the input [`Float`].
    ///
    /// If the output has a precision, it is the precision of the [`Float`] input.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,m)=\text{NaN}$
    /// - $f(\infty,x,m)=\infty$ if $x$ is not NaN or $-\infty$
    /// - $f(-\infty,x,m)=-\infty$ if $x$ is not NaN or $\infty$
    /// - $f(0.0,0,m)=0.0$
    /// - $f(-0.0,0,m)=-0.0$
    /// - $f(x,0,m)=x$ if $x$ is not NaN and $x$ is nonzero
    /// - $f(0.0,x,m)=f(-0.0,x,m)=-x$ if $x$ is not NaN and $x$ is nonzero
    /// - $f(x,x,m)=0.0$ if $x$ is nonzero and $m$ is not `Floor`
    /// - $f(x,x,m)=-0.0$ if $x$ is nonzero and $m$ is `Floor`
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,y,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$ is
    ///   returned instead, where `p` is the precision of the input.
    /// - If $f(x,y,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is
    ///   returned instead.
    /// - If $f(x,y,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`, $-(1-(1/2)^p)2^{2^{30}-1}$
    ///   is returned instead, where `p` is the precision of the input.
    /// - If $0<f(x,y,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,y,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,y,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is
    ///   returned instead.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::sub_rational_prec_round`] instead. If you know you'll be using the `Nearest`
    /// rounding mode, consider using `-` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the precision of the [`Float`] input is not high enough to
    /// represent the output.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (sum, o) = Float::from(PI).sub_rational_round(Rational::from_unsigneds(1u8, 3), Floor);
    /// assert_eq!(sum.to_string(), "2.808259320256457");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) =
    ///     Float::from(PI).sub_rational_round(Rational::from_unsigneds(1u8, 3), Ceiling);
    /// assert_eq!(sum.to_string(), "2.808259320256461");
    /// assert_eq!(o, Greater);
    ///
    /// let (sum, o) =
    ///     Float::from(PI).sub_rational_round(Rational::from_unsigneds(1u8, 3), Nearest);
    /// assert_eq!(sum.to_string(), "2.808259320256461");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn sub_rational_round(self, other: Rational, rm: RoundingMode) -> (Float, Ordering) {
        let prec = self.significant_bits();
        self.sub_rational_prec_round(other, prec, rm)
    }

    /// Subtracts a [`Float`] by a [`Rational`], rounding the result with the specified rounding
    /// mode. The [`Float`] is taken by value and the [`Rational`] by reference. An [`Ordering`] is
    /// also returned, indicating whether the rounded difference is less than, equal to, or greater
    /// than the exact difference. Although `NaN`s are not comparable to any [`Float`], whenever
    /// this function returns a `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the precision of the [`Float`] input. See [`RoundingMode`]
    /// for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,m) = x-y+\varepsilon.
    /// $$
    /// - If $x-y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-y$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x-y|\rfloor-p+1}$, where $p$ is the precision of the input [`Float`].
    /// - If $x-y$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x-y|\rfloor-p}$, where $p$ is the precision of the input [`Float`].
    ///
    /// If the output has a precision, it is the precision of the [`Float`] input.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,m)=\text{NaN}$
    /// - $f(\infty,x,m)=\infty$ if $x$ is not NaN or $-\infty$
    /// - $f(-\infty,x,m)=-\infty$ if $x$ is not NaN or $\infty$
    /// - $f(0.0,0,m)=0.0$
    /// - $f(-0.0,0,m)=-0.0$
    /// - $f(x,0,m)=x$ if $x$ is not NaN and $x$ is nonzero
    /// - $f(0.0,x,m)=f(-0.0,x,m)=-x$ if $x$ is not NaN and $x$ is nonzero
    /// - $f(x,x,m)=0.0$ if $x$ is nonzero and $m$ is not `Floor`
    /// - $f(x,x,m)=-0.0$ if $x$ is nonzero and $m$ is `Floor`
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,y,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$ is
    ///   returned instead, where `p` is the precision of the input.
    /// - If $f(x,y,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is
    ///   returned instead.
    /// - If $f(x,y,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`, $-(1-(1/2)^p)2^{2^{30}-1}$
    ///   is returned instead, where `p` is the precision of the input.
    /// - If $0<f(x,y,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,y,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,y,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is
    ///   returned instead.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::sub_rational_prec_round_val_ref`] instead. If you know you'll be using the
    /// `Nearest` rounding mode, consider using `-` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the precision of the [`Float`] input is not high enough to
    /// represent the output.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (sum, o) =
    ///     Float::from(PI).sub_rational_round_val_ref(&Rational::from_unsigneds(1u8, 3), Floor);
    /// assert_eq!(sum.to_string(), "2.808259320256457");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) =
    ///     Float::from(PI).sub_rational_round_val_ref(&Rational::from_unsigneds(1u8, 3), Ceiling);
    /// assert_eq!(sum.to_string(), "2.808259320256461");
    /// assert_eq!(o, Greater);
    ///
    /// let (sum, o) =
    ///     Float::from(PI).sub_rational_round_val_ref(&Rational::from_unsigneds(1u8, 3), Nearest);
    /// assert_eq!(sum.to_string(), "2.808259320256461");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn sub_rational_round_val_ref(
        self,
        other: &Rational,
        rm: RoundingMode,
    ) -> (Float, Ordering) {
        let prec = self.significant_bits();
        self.sub_rational_prec_round_val_ref(other, prec, rm)
    }

    /// Subtracts a [`Float`] by a [`Rational`], rounding the result with the specified rounding
    /// mode. The [`Float`] is taken by reference and the [`Rational`] by value. An [`Ordering`] is
    /// also returned, indicating whether the rounded difference is less than, equal to, or greater
    /// than the exact difference. Although `NaN`s are not comparable to any [`Float`], whenever
    /// this function returns a `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the precision of the [`Float`] input. See [`RoundingMode`]
    /// for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,m) = x-y+\varepsilon.
    /// $$
    /// - If $x-y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-y$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x-y|\rfloor-p+1}$, where $p$ is the precision of the input [`Float`].
    /// - If $x-y$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x-y|\rfloor-p}$, where $p$ is the precision of the input [`Float`].
    ///
    /// If the output has a precision, it is the precision of the [`Float`] input.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,m)=\text{NaN}$
    /// - $f(\infty,x,m)=\infty$ if $x$ is not NaN or $-\infty$
    /// - $f(-\infty,x,m)=-\infty$ if $x$ is not NaN or $\infty$
    /// - $f(0.0,0,m)=0.0$
    /// - $f(-0.0,0,m)=-0.0$
    /// - $f(x,0,m)=x$ if $x$ is not NaN and $x$ is nonzero
    /// - $f(0.0,x,m)=f(-0.0,x,m)=-x$ if $x$ is not NaN and $x$ is nonzero
    /// - $f(x,x,m)=0.0$ if $x$ is nonzero and $m$ is not `Floor`
    /// - $f(x,x,m)=-0.0$ if $x$ is nonzero and $m$ is `Floor`
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,y,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$ is
    ///   returned instead, where `p` is the precision of the input.
    /// - If $f(x,y,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is
    ///   returned instead.
    /// - If $f(x,y,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`, $-(1-(1/2)^p)2^{2^{30}-1}$
    ///   is returned instead, where `p` is the precision of the input.
    /// - If $0<f(x,y,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,y,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,y,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is
    ///   returned instead.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::sub_rational_prec_round_ref_val`] instead. If you know you'll be using the
    /// `Nearest` rounding mode, consider using `-` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the precision of the [`Float`] input is not high enough to
    /// represent the output.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (sum, o) =
    ///     Float::from(PI).sub_rational_round_ref_val(Rational::from_unsigneds(1u8, 3), Floor);
    /// assert_eq!(sum.to_string(), "2.808259320256457");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) =
    ///     Float::from(PI).sub_rational_round_ref_val(Rational::from_unsigneds(1u8, 3), Ceiling);
    /// assert_eq!(sum.to_string(), "2.808259320256461");
    /// assert_eq!(o, Greater);
    ///
    /// let (sum, o) =
    ///     Float::from(PI).sub_rational_round_ref_val(Rational::from_unsigneds(1u8, 3), Nearest);
    /// assert_eq!(sum.to_string(), "2.808259320256461");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn sub_rational_round_ref_val(
        &self,
        other: Rational,
        rm: RoundingMode,
    ) -> (Float, Ordering) {
        let prec = self.significant_bits();
        self.sub_rational_prec_round_ref_val(other, prec, rm)
    }

    /// Subtracts a [`Float`] by a [`Rational`], rounding the result with the specified rounding
    /// mode. The [`Float`] and the [`Rational`] are both are taken by reference. An [`Ordering`] is
    /// also returned, indicating whether the rounded difference is less than, equal to, or greater
    /// than the exact difference. Although `NaN`s are not comparable to any [`Float`], whenever
    /// this function returns a `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the precision of the [`Float`] input. See [`RoundingMode`]
    /// for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,m) = x-y+\varepsilon.
    /// $$
    /// - If $x-y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-y$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x-y|\rfloor-p+1}$, where $p$ is the precision of the input [`Float`].
    /// - If $x-y$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x-y|\rfloor-p}$, where $p$ is the precision of the input [`Float`].
    ///
    /// If the output has a precision, it is the precision of the [`Float`] input.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,m)=\text{NaN}$
    /// - $f(\infty,x,m)=\infty$ if $x$ is not NaN or $-\infty$
    /// - $f(-\infty,x,m)=-\infty$ if $x$ is not NaN or $\infty$
    /// - $f(0.0,0,m)=0.0$
    /// - $f(-0.0,0,m)=-0.0$
    /// - $f(x,0,m)=x$ if $x$ is not NaN and $x$ is nonzero
    /// - $f(0.0,x,m)=f(-0.0,x,m)=-x$ if $x$ is not NaN and $x$ is nonzero
    /// - $f(x,x,m)=0.0$ if $x$ is nonzero and $m$ is not `Floor`
    /// - $f(x,x,m)=-0.0$ if $x$ is nonzero and $m$ is `Floor`
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,y,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$ is
    ///   returned instead, where `p` is the precision of the input.
    /// - If $f(x,y,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is
    ///   returned instead.
    /// - If $f(x,y,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`, $-(1-(1/2)^p)2^{2^{30}-1}$
    ///   is returned instead, where `p` is the precision of the input.
    /// - If $0<f(x,y,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,y,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,y,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is
    ///   returned instead.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::sub_rational_prec_round_ref_ref`] instead. If you know you'll be using the
    /// `Nearest` rounding mode, consider using `-` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the precision of the [`Float`] input is not high enough to
    /// represent the output.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (sum, o) =
    ///     Float::from(PI).sub_rational_round_ref_ref(&Rational::from_unsigneds(1u8, 3), Floor);
    /// assert_eq!(sum.to_string(), "2.808259320256457");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) =
    ///     Float::from(PI).sub_rational_round_ref_ref(&Rational::from_unsigneds(1u8, 3), Ceiling);
    /// assert_eq!(sum.to_string(), "2.808259320256461");
    /// assert_eq!(o, Greater);
    ///
    /// let (sum, o) =
    ///     Float::from(PI).sub_rational_round_ref_ref(&Rational::from_unsigneds(1u8, 3), Nearest);
    /// assert_eq!(sum.to_string(), "2.808259320256461");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn sub_rational_round_ref_ref(
        &self,
        other: &Rational,
        rm: RoundingMode,
    ) -> (Float, Ordering) {
        let prec = self.significant_bits();
        self.sub_rational_prec_round_ref_ref(other, prec, rm)
    }

    /// Subtracts a [`Rational`] by a [`Float`] in place, rounding the result to the specified
    /// precision and with the specified rounding mode. The [`Rational`] is taken by value. An
    /// [`Ordering`] is returned, indicating whether the rounded difference is less than, equal to,
    /// or greater than the exact difference. Although `NaN`s are not comparable to any [`Float`],
    /// whenever this function sets the [`Float`] to `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets x-y+\varepsilon.
    /// $$
    /// - If $x-y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-y$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x-y|\rfloor-p+1}$.
    /// - If $x-y$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x-y|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// See the [`Float::sub_rational_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::sub_rational_prec_assign`]
    /// instead. If you know that your target precision is the precision of the [`Float`] input,
    /// consider using [`Float::sub_rational_round_assign`] instead. If both of these things are
    /// true, consider using `-=` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(other.significant_bits(),
    /// prec)`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but `prec` is too small for an exact subtraction.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.sub_rational_prec_round_assign(Rational::from_unsigneds(1u8, 3), 5, Floor),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "2.8");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.sub_rational_prec_round_assign(Rational::from_unsigneds(1u8, 3), 5, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "2.9");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.sub_rational_prec_round_assign(Rational::from_unsigneds(1u8, 3), 5, Nearest),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "2.8");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.sub_rational_prec_round_assign(Rational::from_unsigneds(1u8, 3), 20, Floor),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "2.808258");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.sub_rational_prec_round_assign(Rational::from_unsigneds(1u8, 3), 20, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "2.808262");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.sub_rational_prec_round_assign(Rational::from_unsigneds(1u8, 3), 20, Nearest),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "2.808258");
    /// ```
    ///
    /// This is mpfr_sub_q from gmp_op.c, MPFR 4.2.0.
    #[inline]
    pub fn sub_rational_prec_round_assign(
        &mut self,
        other: Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> Ordering {
        assert_ne!(prec, 0);
        match (&mut *self, other) {
            (Float(NaN | Infinity { .. }), _) => Equal,
            (float_negative_zero!(), y) => {
                let o;
                (*self, o) = Float::from_rational_prec_round(y, prec, -rm);
                self.neg_assign();
                o.reverse()
            }
            (float_zero!(), y) => {
                if y == 0u32 {
                    Equal
                } else {
                    let o;
                    (*self, o) = Float::from_rational_prec_round(y, prec, -rm);
                    self.neg_assign();
                    o.reverse()
                }
            }
            (_, y) if y == 0 => self.set_prec_round(prec, rm),
            (x, y) => {
                if *x == y {
                    *self = if rm == Floor {
                        float_negative_zero!()
                    } else {
                        float_zero!()
                    };
                    return Equal;
                }
                let (min_exponent, max_exponent) = float_rational_diff_exponent_range(x, &y);
                if min_exponent >= i64::from(Float::MAX_EXPONENT) {
                    assert!(rm != Exact, "Inexact Float subtraction");
                    return match (float_rational_diff_sign(x, &y), rm) {
                        (true, Ceiling | Up | Nearest) => {
                            *self = float_infinity!();
                            Greater
                        }
                        (true, _) => {
                            *self = Float::max_finite_value_with_prec(prec);
                            Less
                        }
                        (false, Floor | Up | Nearest) => {
                            *self = float_negative_infinity!();
                            Less
                        }
                        (false, _) => {
                            *self = -Float::max_finite_value_with_prec(prec);
                            Greater
                        }
                    };
                }
                if max_exponent > i64::from(Float::MAX_EXPONENT) - 2
                    || min_exponent < i64::from(Float::MIN_EXPONENT - 2)
                {
                    // If we can't rule out overflow or underflow, use slow-but-correct naive
                    // algorithm.
                    let (diff, o) = sub_rational_prec_round_naive_ref_val(&*x, y, prec, rm);
                    *self = diff;
                    return o;
                }
                let mut working_prec = prec + 10;
                let mut increment = Limb::WIDTH;
                // working_prec grows as O([(1 + sqrt(3)) / 2] ^ n) ≈ O(1.366 ^ n).
                loop {
                    // Error <= 1/2 ulp(q)
                    let (q, o) = Float::from_rational_prec_ref(&y, working_prec);
                    if o == Equal {
                        // Result is exact so we can subtract it directly!
                        return self.sub_prec_round_assign(q, prec, rm);
                    }
                    let q_exp = q.get_exponent().unwrap();
                    let t = x.sub_prec_ref_val(q, working_prec).0;
                    // Error on t is <= 1/2 ulp(t).
                    // ```
                    // Error / ulp(t)      <= 1/2 + 1/2 * 2^(EXP(q)-EXP(t))
                    // If EXP(q)-EXP(t)>0, <= 2^(EXP(q)-EXP(t)-1)*(1+2^-(EXP(q)-EXP(t)))
                    //                     <= 2^(EXP(q)-EXP(t))
                    // If EXP(q)-EXP(t)<0, <= 2^0
                    // ```
                    // We can get 0, but we can't round since q is inexact
                    if t != 0 {
                        let m = u64::saturating_from(q_exp - t.get_exponent().unwrap())
                            .checked_add(1)
                            .unwrap();
                        if working_prec >= m
                            && float_can_round(
                                t.significand_ref().unwrap(),
                                working_prec - m,
                                prec,
                                rm,
                            )
                        {
                            *self = t;
                            return self.set_prec_round(prec, rm);
                        }
                    }
                    working_prec += increment;
                    increment = working_prec >> 1;
                }
            }
        }
    }

    /// Subtracts a [`Rational`] by a [`Float`] in place, rounding the result to the specified
    /// precision and with the specified rounding mode. The [`Rational`] is taken by reference. An
    /// [`Ordering`] is returned, indicating whether the rounded difference is less than, equal to,
    /// or greater than the exact difference. Although `NaN`s are not comparable to any [`Float`],
    /// whenever this function sets the [`Float`] to `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets x-y+\varepsilon.
    /// $$
    /// - If $x-y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-y$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x-y|\rfloor-p+1}$.
    /// - If $x-y$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x-y|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// See the [`Float::sub_rational_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`Float::sub_rational_prec_assign_ref`] instead. If you know that your target precision is
    /// the precision of the [`Float`] input, consider using
    /// [`Float::sub_rational_round_assign_ref`] instead. If both of these things are true, consider
    /// using `-=` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(other.significant_bits(),
    /// prec)`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but `prec` is too small for an exact subtraction.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.sub_rational_prec_round_assign_ref(&Rational::from_unsigneds(1u8, 3), 5, Floor),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "2.8");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.sub_rational_prec_round_assign_ref(&Rational::from_unsigneds(1u8, 3), 5, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "2.9");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.sub_rational_prec_round_assign_ref(&Rational::from_unsigneds(1u8, 3), 5, Nearest),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "2.8");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.sub_rational_prec_round_assign_ref(&Rational::from_unsigneds(1u8, 3), 20, Floor),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "2.808258");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.sub_rational_prec_round_assign_ref(&Rational::from_unsigneds(1u8, 3), 20, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "2.808262");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.sub_rational_prec_round_assign_ref(&Rational::from_unsigneds(1u8, 3), 20, Nearest),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "2.808258");
    /// ```
    #[inline]
    pub fn sub_rational_prec_round_assign_ref(
        &mut self,
        other: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> Ordering {
        assert_ne!(prec, 0);
        match (&mut *self, other) {
            (Float(NaN | Infinity { .. }), _) => Equal,
            (float_negative_zero!(), y) => {
                let o;
                (*self, o) = Float::from_rational_prec_round_ref(y, prec, -rm);
                self.neg_assign();
                o.reverse()
            }
            (float_zero!(), y) => {
                if *y == 0u32 {
                    Equal
                } else {
                    let o;
                    (*self, o) = Float::from_rational_prec_round_ref(y, prec, -rm);
                    self.neg_assign();
                    o.reverse()
                }
            }
            (_, y) if *y == 0 => self.set_prec_round(prec, rm),
            (x, y) => {
                if *x == *y {
                    *self = if rm == Floor {
                        float_negative_zero!()
                    } else {
                        float_zero!()
                    };
                    return Equal;
                }
                let (min_exponent, max_exponent) = float_rational_diff_exponent_range(x, y);
                if min_exponent >= i64::from(Float::MAX_EXPONENT) {
                    assert!(rm != Exact, "Inexact Float subtraction");
                    return match (float_rational_diff_sign(x, y), rm) {
                        (true, Ceiling | Up | Nearest) => {
                            *self = float_infinity!();
                            Greater
                        }
                        (true, _) => {
                            *self = Float::max_finite_value_with_prec(prec);
                            Less
                        }
                        (false, Floor | Up | Nearest) => {
                            *self = float_negative_infinity!();
                            Less
                        }
                        (false, _) => {
                            *self = -Float::max_finite_value_with_prec(prec);
                            Greater
                        }
                    };
                }
                if max_exponent > i64::from(Float::MAX_EXPONENT) - 2
                    || min_exponent < i64::from(Float::MIN_EXPONENT - 2)
                {
                    // If we can't rule out overflow or underflow, use slow-but-correct naive
                    // algorithm.
                    let (diff, o) = sub_rational_prec_round_naive_ref_ref(&*x, y, prec, rm);
                    *self = diff;
                    return o;
                }
                let mut working_prec = prec + 10;
                let mut increment = Limb::WIDTH;
                // working_prec grows as O([(1 + sqrt(3)) / 2] ^ n) ≈ O(1.366 ^ n).
                loop {
                    // Error <= 1/2 ulp(q)
                    let (q, o) = Float::from_rational_prec_ref(y, working_prec);
                    if o == Equal {
                        // Result is exact so we can subtract it directly!
                        return self.sub_prec_round_assign(q, prec, rm);
                    }
                    let q_exp = q.get_exponent().unwrap();
                    let t = x.sub_prec_ref_val(q, working_prec).0;
                    // Error on t is <= 1/2 ulp(t).
                    // ```
                    // Error / ulp(t)      <= 1/2 + 1/2 * 2^(EXP(q)-EXP(t))
                    // If EXP(q)-EXP(t)>0, <= 2^(EXP(q)-EXP(t)-1)*(1+2^-(EXP(q)-EXP(t)))
                    //                     <= 2^(EXP(q)-EXP(t))
                    // If EXP(q)-EXP(t)<0, <= 2^0
                    // ```
                    // We can get 0, but we can't round since q is inexact
                    if t != 0 {
                        let m = u64::saturating_from(q_exp - t.get_exponent().unwrap())
                            .checked_add(1)
                            .unwrap();
                        if working_prec >= m
                            && float_can_round(
                                t.significand_ref().unwrap(),
                                working_prec - m,
                                prec,
                                rm,
                            )
                        {
                            *self = t;
                            return self.set_prec_round(prec, rm);
                        }
                    }
                    working_prec += increment;
                    increment = working_prec >> 1;
                }
            }
        }
    }

    /// Subtracts a [`Rational`] by a [`Float`] in place, rounding the result to the nearest value
    /// of the specified precision. The [`Rational`] is taken by value. An [`Ordering`] is returned,
    /// indicating whether the rounded difference is less than, equal to, or greater than the exact
    /// difference. Although `NaN`s are not comparable to any [`Float`], whenever this function sets
    /// the [`Float`] to `NaN` it also returns `Equal`.
    ///
    /// If the difference is equidistant from two [`Float`]s with the specified precision, the
    /// [`Float`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a
    /// description of the `Nearest` rounding mode.
    ///
    /// $$
    /// x \gets x-y+\varepsilon.
    /// $$
    /// - If $x-y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-y$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2 |x-y|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// See the [`Float::sub_rational_prec`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::sub_rational_prec_round_assign`] instead. If you know that your target precision is
    /// the maximum of the precisions of the two inputs, consider using `-=` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(other.significant_bits(),
    /// prec)`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::num::conversion::traits::ExactFrom;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.sub_rational_prec_assign(Rational::exact_from(1.5), 5),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "1.62");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.sub_rational_prec_assign(Rational::exact_from(1.5), 20),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "1.641592");
    /// ```
    #[inline]
    pub fn sub_rational_prec_assign(&mut self, other: Rational, prec: u64) -> Ordering {
        self.sub_rational_prec_round_assign(other, prec, Nearest)
    }

    /// Subtracts a [`Rational`] by a [`Float`] in place, rounding the result to the nearest value
    /// of the specified precision. The [`Rational`] is taken by reference. An [`Ordering`] is
    /// returned, indicating whether the rounded difference is less than, equal to, or greater than
    /// the exact difference. Although `NaN`s are not comparable to any [`Float`], whenever this
    /// function sets the [`Float`] to `NaN` it also returns `Equal`.
    ///
    /// If the difference is equidistant from two [`Float`]s with the specified precision, the
    /// [`Float`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a
    /// description of the `Nearest` rounding mode.
    ///
    /// $$
    /// x \gets x-y+\varepsilon.
    /// $$
    /// - If $x-y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-y$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2 |x-y|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// See the [`Float::sub_rational_prec_val_ref`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::sub_rational_prec_round_assign_ref`] instead. If you know that your target
    /// precision is the maximum of the precisions of the two inputs, consider using `-=` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(other.significant_bits(),
    /// prec)`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::num::conversion::traits::ExactFrom;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.sub_rational_prec_assign_ref(&Rational::exact_from(1.5), 5),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "1.62");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.sub_rational_prec_assign_ref(&Rational::exact_from(1.5), 20),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "1.641592");
    /// ```
    #[inline]
    pub fn sub_rational_prec_assign_ref(&mut self, other: &Rational, prec: u64) -> Ordering {
        self.sub_rational_prec_round_assign_ref(other, prec, Nearest)
    }

    /// Subtracts a [`Rational`] by a [`Float`] in place, rounding the result with the specified
    /// rounding mode. The [`Rational`] is taken by value. An [`Ordering`] is returned, indicating
    /// whether the rounded difference is less than, equal to, or greater than the exact difference.
    /// Although `NaN`s are not comparable to any [`Float`], whenever this function sets the
    /// [`Float`] to `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the precision of the input [`Float`]. See [`RoundingMode`]
    /// for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets x-y+\varepsilon.
    /// $$
    /// - If $x-y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-y$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x-y|\rfloor-p+1}$, where $p$ is the precision of the input [`Float`].
    /// - If $x-y$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x-y|\rfloor-p}$, where $p$ is the precision of the input [`Float`].
    ///
    /// If the output has a precision, it is the precision of the input [`Float`].
    ///
    /// See the [`Float::sub_rational_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::sub_rational_prec_round_assign`] instead. If you know you'll be using the `Nearest`
    /// rounding mode, consider using `-=` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the precision of the input [`Float`] is not high enough to
    /// represent the output.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.sub_rational_round_assign(Rational::from_unsigneds(1u8, 3), Floor),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "2.808259320256457");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.sub_rational_round_assign(Rational::from_unsigneds(1u8, 3), Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "2.808259320256461");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.sub_rational_round_assign(Rational::from_unsigneds(1u8, 3), Nearest),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "2.808259320256461");
    /// ```
    #[inline]
    pub fn sub_rational_round_assign(&mut self, other: Rational, rm: RoundingMode) -> Ordering {
        let prec = self.significant_bits();
        self.sub_rational_prec_round_assign(other, prec, rm)
    }

    /// Subtracts a [`Rational`] by a [`Float`] in place, rounding the result with the specified
    /// rounding mode. The [`Rational`] is taken by reference. An [`Ordering`] is returned,
    /// indicating whether the rounded difference is less than, equal to, or greater than the exact
    /// difference. Although `NaN`s are not comparable to any [`Float`], whenever this function sets
    /// the [`Float`] to `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the precision of the input [`Float`]. See [`RoundingMode`]
    /// for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets x-y+\varepsilon.
    /// $$
    /// - If $x-y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-y$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x-y|\rfloor-p+1}$, where $p$ is the precision of the input [`Float`].
    /// - If $x-y$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x-y|\rfloor-p}$, where $p$ is the precision of the input [`Float`].
    ///
    /// If the output has a precision, it is the precision of the input [`Float`].
    ///
    /// See the [`Float::sub_rational_round_val_ref`] documentation for information on special
    /// cases, overflow, and underflow.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::sub_rational_prec_round_assign_ref`] instead. If you know you'll be using the
    /// `Nearest` rounding mode, consider using `-=` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the precision of the input [`Float`] is not high enough to
    /// represent the output.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.sub_rational_round_assign_ref(&Rational::from_unsigneds(1u8, 3), Floor),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "2.808259320256457");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.sub_rational_round_assign_ref(&Rational::from_unsigneds(1u8, 3), Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "2.808259320256461");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.sub_rational_round_assign_ref(&Rational::from_unsigneds(1u8, 3), Nearest),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "2.808259320256461");
    /// ```
    #[inline]
    pub fn sub_rational_round_assign_ref(
        &mut self,
        other: &Rational,
        rm: RoundingMode,
    ) -> Ordering {
        let prec = self.significant_bits();
        self.sub_rational_prec_round_assign_ref(other, prec, rm)
    }
}

impl Sub<Float> for Float {
    type Output = Float;

    /// Subtracts two [`Float`]s, taking both by value.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs. If the
    /// difference is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,y) = x-y+\varepsilon.
    /// $$
    /// - If $x-y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-y$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2 |x-y|\rfloor-p}$,
    ///   where $p$ is the maximum precision of the inputs.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x)=f(x,\text{NaN})=f(\infty,\infty)=f(-\infty,-\infty)=\text{NaN}$
    /// - $f(\infty,x)=\infty$ if $x$ is not NaN or $\infty$
    /// - $f(x,-\infty)=\infty$ if $x$ is not NaN or $-\infty$
    /// - $f(-\infty,x)=-\infty$ if $x$ is not NaN or $-\infty$
    /// - $f(x,\infty)=-\infty$ if $x$ is not NaN or $\infty$
    /// - $f(0.0,-0.0)=0.0$
    /// - $f(-0.0,0.0)=-0.0$
    /// - $f(0.0,0.0)=f(-0.0,-0.0)=0.0$
    /// - $f(x,0.0)=f(x,-0.0)=x$ if $x$ is not NaN and $x$ is nonzero
    /// - $f(0.0,x)=f(-0.0,x)=-x$ if $x$ is not NaN and $x$ is nonzero
    /// - $f(x,x)=0.0$ if $x$ is finite and nonzero
    ///
    /// Overflow and underflow:
    /// - If $f(x,y)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,y)\geq 2^{2^{30}-1}$, $-\infty$ is returned instead.
    /// - If $0<f(x,y)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using [`Float::sub_prec`]
    /// instead. If you want to specify the output precision, consider using [`Float::sub_round`].
    /// If you want both of these things, consider using [`Float::sub_prec_round`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity};
    /// use malachite_float::Float;
    ///
    /// assert!((Float::from(1.5) - Float::NAN).is_nan());
    /// assert_eq!(Float::from(1.5) - Float::INFINITY, Float::NEGATIVE_INFINITY);
    /// assert_eq!(Float::from(1.5) - Float::NEGATIVE_INFINITY, Float::INFINITY);
    /// assert!((Float::INFINITY - Float::INFINITY).is_nan());
    ///
    /// assert_eq!(Float::from(1.5) - Float::from(2.5), -1.0);
    /// assert_eq!(Float::from(1.5) - Float::from(-2.5), 4.0);
    /// assert_eq!(Float::from(-1.5) - Float::from(2.5), -4.0);
    /// assert_eq!(Float::from(-1.5) - Float::from(-2.5), 1.0);
    /// ```
    ///
    #[inline]
    fn sub(self, other: Float) -> Float {
        let prec = max(self.significant_bits(), other.significant_bits());
        self.sub_prec_round(other, prec, Nearest).0
    }
}

impl Sub<&Float> for Float {
    type Output = Float;

    /// Subtracts two [`Float`]s, taking the first by value and the second by reference.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs. If the
    /// difference is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,y) = x-y+\varepsilon.
    /// $$
    /// - If $x-y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-y$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2 |x-y|\rfloor-p}$,
    ///   where $p$ is the maximum precision of the inputs.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x)=f(x,\text{NaN})=f(\infty,\infty)=f(-\infty,-\infty)=\text{NaN}$
    /// - $f(\infty,x)=\infty$ if $x$ is not NaN or $\infty$
    /// - $f(x,-\infty)=\infty$ if $x$ is not NaN or $-\infty$
    /// - $f(-\infty,x)=-\infty$ if $x$ is not NaN or $-\infty$
    /// - $f(x,\infty)=-\infty$ if $x$ is not NaN or $\infty$
    /// - $f(0.0,-0.0)=0.0$
    /// - $f(-0.0,0.0)=-0.0$
    /// - $f(0.0,0.0)=f(-0.0,-0.0)=0.0$
    /// - $f(x,0.0)=f(x,-0.0)=x$ if $x$ is not NaN and $x$ is nonzero
    /// - $f(0.0,x)=f(-0.0,x)=-x$ if $x$ is not NaN and $x$ is nonzero
    /// - $f(x,x)=0.0$ if $x$ is finite and nonzero
    ///
    /// Overflow and underflow:
    /// - If $f(x,y)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,y)\geq 2^{2^{30}-1}$, $-\infty$ is returned instead.
    /// - If $0<f(x,y)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::sub_prec_val_ref`] instead. If you want to specify the output precision, consider
    /// using [`Float::sub_round_val_ref`]. If you want both of these things, consider using
    /// [`Float::sub_prec_round_val_ref`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`, and $m$ is `other.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity};
    /// use malachite_float::Float;
    ///
    /// assert!((Float::from(1.5) - &Float::NAN).is_nan());
    /// assert_eq!(
    ///     Float::from(1.5) - &Float::INFINITY,
    ///     Float::NEGATIVE_INFINITY
    /// );
    /// assert_eq!(
    ///     Float::from(1.5) - &Float::NEGATIVE_INFINITY,
    ///     Float::INFINITY
    /// );
    /// assert!((Float::INFINITY - &Float::INFINITY).is_nan());
    ///
    /// assert_eq!(Float::from(1.5) - &Float::from(2.5), -1.0);
    /// assert_eq!(Float::from(1.5) - &Float::from(-2.5), 4.0);
    /// assert_eq!(Float::from(-1.5) - &Float::from(2.5), -4.0);
    /// assert_eq!(Float::from(-1.5) - &Float::from(-2.5), 1.0);
    /// ```
    #[inline]
    fn sub(self, other: &Float) -> Float {
        let prec = max(self.significant_bits(), other.significant_bits());
        self.sub_prec_round_val_ref(other, prec, Nearest).0
    }
}

impl Sub<Float> for &Float {
    type Output = Float;

    /// Subtracts two [`Float`]s, taking the first by reference and the second by value.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs. If the
    /// difference is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,y) = x-y+\varepsilon.
    /// $$
    /// - If $x-y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-y$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2 |x-y|\rfloor-p}$,
    ///   where $p$ is the maximum precision of the inputs.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x)=f(x,\text{NaN})=f(\infty,\infty)=f(-\infty,-\infty)=\text{NaN}$
    /// - $f(\infty,x)=\infty$ if $x$ is not NaN or $\infty$
    /// - $f(x,-\infty)=\infty$ if $x$ is not NaN or $-\infty$
    /// - $f(-\infty,x)=-\infty$ if $x$ is not NaN or $-\infty$
    /// - $f(x,\infty)=-\infty$ if $x$ is not NaN or $\infty$
    /// - $f(0.0,-0.0)=0.0$
    /// - $f(-0.0,0.0)=-0.0$
    /// - $f(0.0,0.0)=f(-0.0,-0.0)=0.0$
    /// - $f(x,0.0)=f(x,-0.0)=x$ if $x$ is not NaN and $x$ is nonzero
    /// - $f(0.0,x)=f(-0.0,x)=-x$ if $x$ is not NaN and $x$ is nonzero
    /// - $f(x,x)=0.0$ if $x$ is finite and nonzero
    ///
    /// Overflow and underflow:
    /// - If $f(x,y)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,y)\geq 2^{2^{30}-1}$, $-\infty$ is returned instead.
    /// - If $0<f(x,y)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::sub_prec_ref_val`] instead. If you want to specify the output precision, consider
    /// using [`Float::sub_round_ref_val`]. If you want both of these things, consider using
    /// [`Float::sub_prec_round_ref_val`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`, and $m$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity};
    /// use malachite_float::Float;
    ///
    /// assert!((&Float::from(1.5) - Float::NAN).is_nan());
    /// assert_eq!(
    ///     &Float::from(1.5) - Float::INFINITY,
    ///     Float::NEGATIVE_INFINITY
    /// );
    /// assert_eq!(
    ///     &Float::from(1.5) - Float::NEGATIVE_INFINITY,
    ///     Float::INFINITY
    /// );
    /// assert!((&Float::INFINITY - Float::INFINITY).is_nan());
    ///
    /// assert_eq!(&Float::from(1.5) - Float::from(2.5), -1.0);
    /// assert_eq!(&Float::from(1.5) - Float::from(-2.5), 4.0);
    /// assert_eq!(&Float::from(-1.5) - Float::from(2.5), -4.0);
    /// assert_eq!(&Float::from(-1.5) - Float::from(-2.5), 1.0);
    /// ```
    #[inline]
    fn sub(self, other: Float) -> Float {
        let prec = max(self.significant_bits(), other.significant_bits());
        self.sub_prec_round_ref_val(other, prec, Nearest).0
    }
}

impl Sub<&Float> for &Float {
    type Output = Float;

    /// Subtracts two [`Float`]s, taking both by reference.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs. If the
    /// difference is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,y) = x-y+\varepsilon.
    /// $$
    /// - If $x-y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-y$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2 |x-y|\rfloor-p}$,
    ///   where $p$ is the maximum precision of the inputs.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x)=f(x,\text{NaN})=f(\infty,\infty)=f(-\infty,-\infty)=\text{NaN}$
    /// - $f(\infty,x)=\infty$ if $x$ is not NaN or $\infty$
    /// - $f(x,-\infty)=\infty$ if $x$ is not NaN or $-\infty$
    /// - $f(-\infty,x)=-\infty$ if $x$ is not NaN or $-\infty$
    /// - $f(x,\infty)=-\infty$ if $x$ is not NaN or $\infty$
    /// - $f(0.0,-0.0)=0.0$
    /// - $f(-0.0,0.0)=-0.0$
    /// - $f(0.0,0.0)=f(-0.0,-0.0)=0.0$
    /// - $f(x,0.0)=f(x,-0.0)=x$ if $x$ is not NaN and $x$ is nonzero
    /// - $f(0.0,x)=f(-0.0,x)=-x$ if $x$ is not NaN and $x$ is nonzero
    /// - $f(x,x)=0.0$ if $x$ is finite and nonzero
    ///
    /// Overflow and underflow:
    /// - If $f(x,y)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,y)\geq 2^{2^{30}-1}$, $-\infty$ is returned instead.
    /// - If $0<f(x,y)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using [`Float::sub_prec`]
    /// instead. If you want to specify the output precision, consider using [`Float::sub_round`].
    /// If you want both of these things, consider using [`Float::sub_prec_round`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity};
    /// use malachite_float::Float;
    ///
    /// assert!((&Float::from(1.5) - &Float::NAN).is_nan());
    /// assert_eq!(
    ///     &Float::from(1.5) - &Float::INFINITY,
    ///     Float::NEGATIVE_INFINITY
    /// );
    /// assert_eq!(
    ///     &Float::from(1.5) - &Float::NEGATIVE_INFINITY,
    ///     Float::INFINITY
    /// );
    /// assert!((&Float::INFINITY - &Float::INFINITY).is_nan());
    ///
    /// assert_eq!(&Float::from(1.5) - &Float::from(2.5), -1.0);
    /// assert_eq!(&Float::from(1.5) - &Float::from(-2.5), 4.0);
    /// assert_eq!(&Float::from(-1.5) - &Float::from(2.5), -4.0);
    /// assert_eq!(&Float::from(-1.5) - &Float::from(-2.5), 1.0);
    /// ```
    #[inline]
    fn sub(self, other: &Float) -> Float {
        let prec = max(self.significant_bits(), other.significant_bits());
        self.sub_prec_round_ref_ref(other, prec, Nearest).0
    }
}

impl SubAssign<Float> for Float {
    /// Subtracts a [`Float`] by a [`Float`] in place, taking the [`Float`] on the right-hand side
    /// by value.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs. If the
    /// difference is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// x\gets = x-y+\varepsilon.
    /// $$
    /// - If $x-y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-y$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2 |x-y|\rfloor-p}$,
    ///   where $p$ is the maximum precision of the inputs.
    ///
    /// See the `-` documentation for information on special cases, overflow, and underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::sub_prec_assign`] instead. If you want to specify the output precision, consider
    /// using [`Float::sub_round_assign`]. If you want both of these things, consider using
    /// [`Float::sub_prec_round_assign`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity};
    /// use malachite_float::Float;
    ///
    /// let mut x = Float::from(1.5);
    /// x -= Float::NAN;
    /// assert!(x.is_nan());
    ///
    /// let mut x = Float::from(1.5);
    /// x -= Float::INFINITY;
    /// assert_eq!(x, Float::NEGATIVE_INFINITY);
    ///
    /// let mut x = Float::from(1.5);
    /// x -= Float::NEGATIVE_INFINITY;
    /// assert_eq!(x, Float::INFINITY);
    ///
    /// let mut x = Float::INFINITY;
    /// x -= Float::INFINITY;
    /// assert!(x.is_nan());
    ///
    /// let mut x = Float::from(1.5);
    /// x -= Float::from(2.5);
    /// assert_eq!(x, -1.0);
    ///
    /// let mut x = Float::from(1.5);
    /// x -= Float::from(-2.5);
    /// assert_eq!(x, 4.0);
    ///
    /// let mut x = Float::from(-1.5);
    /// x -= Float::from(2.5);
    /// assert_eq!(x, -4.0);
    ///
    /// let mut x = Float::from(-1.5);
    /// x -= Float::from(-2.5);
    /// assert_eq!(x, 1.0);
    /// ```
    #[inline]
    fn sub_assign(&mut self, other: Float) {
        let prec = max(self.significant_bits(), other.significant_bits());
        self.sub_prec_round_assign(other, prec, Nearest);
    }
}

impl SubAssign<&Float> for Float {
    /// Subtracts a [`Float`] by a [`Float`] in place, taking the [`Float`] on the right-hand side
    /// by reference.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs. If the
    /// difference is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// x\gets = x-y+\varepsilon.
    /// $$
    /// - If $x-y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-y$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2 |x-y|\rfloor-p}$,
    ///   where $p$ is the maximum precision of the inputs.
    ///
    /// See the `-` documentation for information on special cases, overflow, and underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::sub_prec_assign`] instead. If you want to specify the output precision, consider
    /// using [`Float::sub_round_assign`]. If you want both of these things, consider using
    /// [`Float::sub_prec_round_assign`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity};
    /// use malachite_float::Float;
    ///
    /// let mut x = Float::from(1.5);
    /// x -= &Float::NAN;
    /// assert!(x.is_nan());
    ///
    /// let mut x = Float::from(1.5);
    /// x -= &Float::INFINITY;
    /// assert_eq!(x, Float::NEGATIVE_INFINITY);
    ///
    /// let mut x = Float::from(1.5);
    /// x -= &Float::NEGATIVE_INFINITY;
    /// assert_eq!(x, Float::INFINITY);
    ///
    /// let mut x = Float::INFINITY;
    /// x -= &Float::INFINITY;
    /// assert!(x.is_nan());
    ///
    /// let mut x = Float::from(1.5);
    /// x -= &Float::from(2.5);
    /// assert_eq!(x, -1.0);
    ///
    /// let mut x = Float::from(1.5);
    /// x -= &Float::from(-2.5);
    /// assert_eq!(x, 4.0);
    ///
    /// let mut x = Float::from(-1.5);
    /// x -= &Float::from(2.5);
    /// assert_eq!(x, -4.0);
    ///
    /// let mut x = Float::from(-1.5);
    /// x -= &Float::from(-2.5);
    /// assert_eq!(x, 1.0);
    /// ```
    #[inline]
    fn sub_assign(&mut self, other: &Float) {
        let prec = max(self.significant_bits(), other.significant_bits());
        self.sub_prec_round_assign_ref(other, prec, Nearest);
    }
}

impl Sub<Rational> for Float {
    type Output = Float;

    /// Subtracts a [`Float`] by a [`Rational`], taking both by value.
    ///
    /// If the output has a precision, it is the precision of the input [`Float`]. If the difference
    /// is equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s
    /// in its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// $$
    /// f(x,y) = x-y+\varepsilon.
    /// $$
    /// - If $x-y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-y$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2 |x-y|\rfloor-p}$,
    ///   where $p$ is the precision of the input [`Float`].
    ///
    /// Special cases:
    /// - $f(\text{NaN},x)=\text{NaN}$
    /// - $f(\infty,x)=\infty$
    /// - $f(-\infty,x)=-\infty$
    /// - $f(0.0,0)=0.0$
    /// - $f(-0.0,0)=-0.0$
    /// - $f(x,0)=x$
    /// - $f(0.0,x)=f(-0.0,x)=-x$
    /// - $f(x,x)=0.0$ if $x$ is nonzero
    ///
    /// Overflow and underflow:
    /// - If $f(x,y)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,y)\geq 2^{2^{30}-1}$, $-\infty$ is returned instead.
    /// - If $0<f(x,y)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::sub_rational_prec`] instead. If you want to specify the output precision, consider
    /// using [`Float::sub_rational_round`]. If you want both of these things, consider using
    /// [`Float::sub_rational_prec_round`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity};
    /// use malachite_base::num::conversion::traits::ExactFrom;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// assert!((Float::NAN - Rational::exact_from(1.5)).is_nan());
    /// assert_eq!(Float::INFINITY - Rational::exact_from(1.5), Float::INFINITY);
    /// assert_eq!(
    ///     Float::NEGATIVE_INFINITY - Rational::exact_from(1.5),
    ///     Float::NEGATIVE_INFINITY
    /// );
    ///
    /// assert_eq!(Float::from(2.5) - Rational::exact_from(1.5), 1.0);
    /// assert_eq!(Float::from(2.5) - Rational::exact_from(-1.5), 4.0);
    /// assert_eq!(Float::from(-2.5) - Rational::exact_from(1.5), -4.0);
    /// assert_eq!(Float::from(-2.5) - Rational::exact_from(-1.5), -1.0);
    /// ```
    #[inline]
    fn sub(self, other: Rational) -> Float {
        let prec = self.significant_bits();
        self.sub_rational_prec_round(other, prec, Nearest).0
    }
}

impl Sub<&Rational> for Float {
    type Output = Float;

    /// Subtracts a [`Float`] by a [`Rational`], taking the first by value and the second by
    /// reference.
    ///
    /// If the output has a precision, it is the precision of the input [`Float`]. If the difference
    /// is equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s
    /// in its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// $$
    /// f(x,y) = x-y+\varepsilon.
    /// $$
    /// - If $x-y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-y$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2 |x-y|\rfloor-p}$,
    ///   where $p$ is the precision of the input [`Float`].
    ///
    /// Special cases:
    /// - $f(\text{NaN},x)=\text{NaN}$
    /// - $f(\infty,x)=\infty$
    /// - $f(-\infty,x)=-\infty$
    /// - $f(0.0,0)=0.0$
    /// - $f(-0.0,0)=-0.0$
    /// - $f(x,0)=x$
    /// - $f(0.0,x)=f(-0.0,x)=-x$
    /// - $f(x,x)=0.0$ if $x$ is nonzero
    ///
    /// Overflow and underflow:
    /// - If $f(x,y)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,y)\geq 2^{2^{30}-1}$, $-\infty$ is returned instead.
    /// - If $0<f(x,y)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::sub_rational_prec_val_ref`] instead. If you want to specify the output precision,
    /// consider using [`Float::sub_rational_round_val_ref`]. If you want both of these things,
    /// consider using [`Float::sub_rational_prec_round_val_ref`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity};
    /// use malachite_base::num::conversion::traits::ExactFrom;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// assert!((Float::NAN - &Rational::exact_from(1.5)).is_nan());
    /// assert_eq!(
    ///     Float::INFINITY - &Rational::exact_from(1.5),
    ///     Float::INFINITY
    /// );
    /// assert_eq!(
    ///     Float::NEGATIVE_INFINITY - &Rational::exact_from(1.5),
    ///     Float::NEGATIVE_INFINITY
    /// );
    ///
    /// assert_eq!(Float::from(2.5) - &Rational::exact_from(1.5), 1.0);
    /// assert_eq!(Float::from(2.5) - &Rational::exact_from(-1.5), 4.0);
    /// assert_eq!(Float::from(-2.5) - &Rational::exact_from(1.5), -4.0);
    /// assert_eq!(Float::from(-2.5) - &Rational::exact_from(-1.5), -1.0);
    /// ```
    #[inline]
    fn sub(self, other: &Rational) -> Float {
        let prec = self.significant_bits();
        self.sub_rational_prec_round_val_ref(other, prec, Nearest).0
    }
}

impl Sub<Rational> for &Float {
    type Output = Float;

    /// Subtracts a [`Float`] by a [`Rational`], taking the first by reference and the second by
    /// value.
    ///
    /// If the output has a precision, it is the precision of the input [`Float`]. If the difference
    /// is equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s
    /// in its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// $$
    /// f(x,y) = x-y+\varepsilon.
    /// $$
    /// - If $x-y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-y$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2 |x-y|\rfloor-p}$,
    ///   where $p$ is the precision of the input [`Float`].
    ///
    /// Special cases:
    /// - $f(\text{NaN},x)=\text{NaN}$
    /// - $f(\infty,x)=\infty$
    /// - $f(-\infty,x)=-\infty$
    /// - $f(0.0,0)=0.0$
    /// - $f(-0.0,0)=-0.0$
    /// - $f(x,0)=x$
    /// - $f(0.0,x)=f(-0.0,x)=-x$
    /// - $f(x,x)=0.0$ if $x$ is nonzero
    ///
    /// Overflow and underflow:
    /// - If $f(x,y)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,y)\geq 2^{2^{30}-1}$, $-\infty$ is returned instead.
    /// - If $0<f(x,y)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::sub_rational_prec_ref_val`] instead. If you want to specify the output precision,
    /// consider using [`Float::sub_rational_round_ref_val`]. If you want both of these things,
    /// consider using [`Float::sub_rational_prec_round_ref_val`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity};
    /// use malachite_base::num::conversion::traits::ExactFrom;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// assert!((&Float::NAN - Rational::exact_from(1.5)).is_nan());
    /// assert_eq!(
    ///     &Float::INFINITY - Rational::exact_from(1.5),
    ///     Float::INFINITY
    /// );
    /// assert_eq!(
    ///     &Float::NEGATIVE_INFINITY - Rational::exact_from(1.5),
    ///     Float::NEGATIVE_INFINITY
    /// );
    ///
    /// assert_eq!(&Float::from(2.5) - Rational::exact_from(1.5), 1.0);
    /// assert_eq!(&Float::from(2.5) - Rational::exact_from(-1.5), 4.0);
    /// assert_eq!(&Float::from(-2.5) - Rational::exact_from(1.5), -4.0);
    /// assert_eq!(&Float::from(-2.5) - Rational::exact_from(-1.5), -1.0);
    /// ```
    #[inline]
    fn sub(self, other: Rational) -> Float {
        let prec = self.significant_bits();
        self.sub_rational_prec_round_ref_val(other, prec, Nearest).0
    }
}

impl Sub<&Rational> for &Float {
    type Output = Float;

    /// Subtracts a [`Float`] by a [`Rational`], taking both by reference.
    ///
    /// If the output has a precision, it is the precision of the input [`Float`]. If the difference
    /// is equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s
    /// in its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// $$
    /// f(x,y) = x-y+\varepsilon.
    /// $$
    /// - If $x-y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-y$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2 |x-y|\rfloor-p}$,
    ///   where $p$ is the precision of the input [`Float`].
    ///
    /// Special cases:
    /// - $f(\text{NaN},x)=\text{NaN}$
    /// - $f(\infty,x)=\infty$
    /// - $f(-\infty,x)=-\infty$
    /// - $f(0.0,0)=0.0$
    /// - $f(-0.0,0)=-0.0$
    /// - $f(x,0)=x$
    /// - $f(0.0,x)=f(-0.0,x)=-x$
    /// - $f(x,x)=0.0$ if $x$ is nonzero
    ///
    /// Overflow and underflow:
    /// - If $f(x,y)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,y)\geq 2^{2^{30}-1}$, $-\infty$ is returned instead.
    /// - If $0<f(x,y)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::sub_rational_prec_ref_ref`] instead. If you want to specify the output precision,
    /// consider using [`Float::sub_rational_round_ref_ref`]. If you want both of these things,
    /// consider using [`Float::sub_rational_prec_round_ref_ref`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity};
    /// use malachite_base::num::conversion::traits::ExactFrom;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// assert!((&Float::NAN - Rational::exact_from(1.5)).is_nan());
    /// assert_eq!(
    ///     &Float::INFINITY - Rational::exact_from(1.5),
    ///     Float::INFINITY
    /// );
    /// assert_eq!(
    ///     &Float::NEGATIVE_INFINITY - Rational::exact_from(1.5),
    ///     Float::NEGATIVE_INFINITY
    /// );
    ///
    /// assert_eq!(&Float::from(2.5) - &Rational::exact_from(1.5), 1.0);
    /// assert_eq!(&Float::from(2.5) - &Rational::exact_from(-1.5), 4.0);
    /// assert_eq!(&Float::from(-2.5) - &Rational::exact_from(1.5), -4.0);
    /// assert_eq!(&Float::from(-2.5) - &Rational::exact_from(-1.5), -1.0);
    /// ```
    #[inline]
    fn sub(self, other: &Rational) -> Float {
        let prec = self.significant_bits();
        self.sub_rational_prec_round_ref_ref(other, prec, Nearest).0
    }
}

impl SubAssign<Rational> for Float {
    /// Subtracts a [`Rational`] by a [`Float`] in place, taking the [`Rational`] by value.
    ///
    /// If the output has a precision, it is the precision of the input [`Float`]. If the difference
    /// is equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s
    /// in its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// $$
    /// x\gets = x-y+\varepsilon.
    /// $$
    /// - If $x-y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-y$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2 |x-y|\rfloor-p}$,
    ///   where $p$ is the precision of the input [`Float`].
    ///
    /// See the `-` documentation for information on special cases, overflow, and underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::sub_rational_prec_assign`] instead. If you want to specify the output precision,
    /// consider using [`Float::sub_rational_round_assign`]. If you want both of these things,
    /// consider using [`Float::sub_rational_prec_round_assign`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity};
    /// use malachite_base::num::conversion::traits::ExactFrom;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let mut x = Float::NAN;
    /// x -= Rational::exact_from(1.5);
    /// assert!(x.is_nan());
    ///
    /// let mut x = Float::INFINITY;
    /// x -= Rational::exact_from(1.5);
    /// assert_eq!(x, Float::INFINITY);
    ///
    /// let mut x = Float::NEGATIVE_INFINITY;
    /// x -= Rational::exact_from(1.5);
    /// assert_eq!(x, Float::NEGATIVE_INFINITY);
    ///
    /// let mut x = Float::from(2.5);
    /// x -= Rational::exact_from(1.5);
    /// assert_eq!(x, 1.0);
    ///
    /// let mut x = Float::from(2.5);
    /// x -= Rational::exact_from(-1.5);
    /// assert_eq!(x, 4.0);
    ///
    /// let mut x = Float::from(-2.5);
    /// x -= Rational::exact_from(1.5);
    /// assert_eq!(x, -4.0);
    ///
    /// let mut x = Float::from(-2.5);
    /// x -= Rational::exact_from(-1.5);
    /// assert_eq!(x, -1.0);
    /// ```
    #[inline]
    fn sub_assign(&mut self, other: Rational) {
        let prec = self.significant_bits();
        self.sub_rational_prec_round_assign(other, prec, Nearest);
    }
}

impl SubAssign<&Rational> for Float {
    /// Subtracts a [`Rational`] by a [`Float`] in place, taking the [`Rational`] by reference.
    ///
    /// If the output has a precision, it is the precision of the input [`Float`]. If the difference
    /// is equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s
    /// in its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// $$
    /// x\gets = x-y+\varepsilon.
    /// $$
    /// - If $x-y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-y$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2 |x-y|\rfloor-p}$,
    ///   where $p$ is the precision of the input [`Float`].
    ///
    /// See the `-` documentation for information on special cases, overflow, and underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::sub_rational_prec_assign_ref`] instead. If you want to specify the output
    /// precision, consider using [`Float::sub_rational_round_assign_ref`]. If you want both of
    /// these things, consider using [`Float::sub_rational_prec_round_assign_ref`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity};
    /// use malachite_base::num::conversion::traits::ExactFrom;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let mut x = Float::NAN;
    /// x -= &Rational::exact_from(1.5);
    /// assert!(x.is_nan());
    ///
    /// let mut x = Float::INFINITY;
    /// x -= &Rational::exact_from(1.5);
    /// assert_eq!(x, Float::INFINITY);
    ///
    /// let mut x = Float::NEGATIVE_INFINITY;
    /// x -= &Rational::exact_from(1.5);
    /// assert_eq!(x, Float::NEGATIVE_INFINITY);
    ///
    /// let mut x = Float::from(2.5);
    /// x -= &Rational::exact_from(1.5);
    /// assert_eq!(x, 1.0);
    ///
    /// let mut x = Float::from(2.5);
    /// x -= &Rational::exact_from(-1.5);
    /// assert_eq!(x, 4.0);
    ///
    /// let mut x = Float::from(-2.5);
    /// x -= &Rational::exact_from(1.5);
    /// assert_eq!(x, -4.0);
    ///
    /// let mut x = Float::from(-2.5);
    /// x -= &Rational::exact_from(-1.5);
    /// assert_eq!(x, -1.0);
    /// ```
    #[inline]
    fn sub_assign(&mut self, other: &Rational) {
        let prec = self.significant_bits();
        self.sub_rational_prec_round_assign_ref(other, prec, Nearest);
    }
}

impl Sub<Float> for Rational {
    type Output = Float;

    /// Subtracts a [`Rational`] by a [`Float`], taking both by value.
    ///
    /// If the output has a precision, it is the precision of the input [`Float`]. If the difference
    /// is equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s
    /// in its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// $$
    /// f(x,y) = x-y+\varepsilon.
    /// $$
    /// - If $x-y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-y$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2 |x-y|\rfloor-p}$,
    ///   where $p$ is the precision of the input [`Float`].
    ///
    /// Special cases:
    /// - $f(x,\text{NaN})=\text{NaN}$
    /// - $f(x,\infty)=-\infty$
    /// - $f(x,-\infty)=\infty$
    /// - $f(0,0.0)=-0.0$
    /// - $f(0,-0.0)=0.0$
    /// - $f(x,0.0)=f(x,-0.0)=x$
    /// - $f(0,x)=-x$
    /// - $f(x,x)=0.0$ if $x$ is nonzero
    ///
    /// Overflow and underflow:
    /// - If $f(x,y)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,y)\geq 2^{2^{30}-1}$, $-\infty$ is returned instead.
    /// - If $0<f(x,y)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity};
    /// use malachite_base::num::conversion::traits::ExactFrom;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// assert!((Rational::exact_from(1.5) - Float::NAN).is_nan());
    /// assert_eq!(
    ///     Rational::exact_from(1.5) - Float::INFINITY,
    ///     Float::NEGATIVE_INFINITY
    /// );
    /// assert_eq!(
    ///     Rational::exact_from(1.5) - Float::NEGATIVE_INFINITY,
    ///     Float::INFINITY
    /// );
    ///
    /// assert_eq!(Rational::exact_from(1.5) - Float::from(2.5), -1.0);
    /// assert_eq!(Rational::exact_from(1.5) - Float::from(-2.5), 4.0);
    /// assert_eq!(Rational::exact_from(-1.5) - Float::from(2.5), -4.0);
    /// assert_eq!(Rational::exact_from(-1.5) - Float::from(-2.5), 1.0);
    /// ```
    #[inline]
    fn sub(self, other: Float) -> Float {
        let prec = other.significant_bits();
        -other.sub_rational_prec_round(self, prec, Nearest).0
    }
}

impl Sub<&Float> for Rational {
    type Output = Float;

    /// Subtracts a [`Rational`] by a [`Float`], taking the [`Rational`] by value and the [`Float`]
    /// by reference.
    ///
    /// If the output has a precision, it is the precision of the input [`Float`]. If the difference
    /// is equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s
    /// in its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// $$
    /// f(x,y) = x-y+\varepsilon.
    /// $$
    /// - If $x-y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-y$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2 |x-y|\rfloor-p}$,
    ///   where $p$ is the precision of the input [`Float`].
    ///
    /// Special cases:
    /// - $f(x,\text{NaN})=\text{NaN}$
    /// - $f(x,\infty)=-\infty$
    /// - $f(x,-\infty)=\infty$
    /// - $f(0,0.0)=-0.0$
    /// - $f(0,-0.0)=0.0$
    /// - $f(x,0.0)=f(x,-0.0)=x$
    /// - $f(0,x)=-x$
    /// - $f(x,x)=0.0$ if $x$ is nonzero
    ///
    /// Overflow and underflow:
    /// - If $f(x,y)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,y)\geq 2^{2^{30}-1}$, $-\infty$ is returned instead.
    /// - If $0<f(x,y)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity};
    /// use malachite_base::num::conversion::traits::ExactFrom;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// assert!((Rational::exact_from(1.5) - &Float::NAN).is_nan());
    /// assert_eq!(
    ///     Rational::exact_from(1.5) - &Float::INFINITY,
    ///     Float::NEGATIVE_INFINITY
    /// );
    /// assert_eq!(
    ///     Rational::exact_from(1.5) - &Float::NEGATIVE_INFINITY,
    ///     Float::INFINITY
    /// );
    ///
    /// assert_eq!(Rational::exact_from(1.5) - &Float::from(2.5), -1.0);
    /// assert_eq!(Rational::exact_from(1.5) - &Float::from(-2.5), 4.0);
    /// assert_eq!(Rational::exact_from(-1.5) - &Float::from(2.5), -4.0);
    /// assert_eq!(Rational::exact_from(-1.5) - &Float::from(-2.5), 1.0);
    /// ```
    #[inline]
    fn sub(self, other: &Float) -> Float {
        let prec = other.significant_bits();
        -other.sub_rational_prec_round_ref_val(self, prec, Nearest).0
    }
}

impl Sub<Float> for &Rational {
    type Output = Float;

    /// Subtracts a [`Rational`] by a [`Float`], taking the [`Rational`] by value and the [`Float`]
    /// by reference.
    ///
    /// If the output has a precision, it is the precision of the input [`Float`]. If the difference
    /// is equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s
    /// in its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// $$
    /// f(x,y) = x-y+\varepsilon.
    /// $$
    /// - If $x-y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-y$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2 |x-y|\rfloor-p}$,
    ///   where $p$ is the precision of the input [`Float`].
    ///
    /// Special cases:
    /// - $f(x,\text{NaN})=\text{NaN}$
    /// - $f(x,\infty)=-\infty$
    /// - $f(x,-\infty)=\infty$
    /// - $f(0,0.0)=-0.0$
    /// - $f(0,-0.0)=0.0$
    /// - $f(x,0.0)=f(x,-0.0)=x$
    /// - $f(0,x)=-x$
    /// - $f(x,x)=0.0$ if $x$ is nonzero
    ///
    /// Overflow and underflow:
    /// - If $f(x,y)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,y)\geq 2^{2^{30}-1}$, $-\infty$ is returned instead.
    /// - If $0<f(x,y)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity};
    /// use malachite_base::num::conversion::traits::ExactFrom;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// assert!((&Rational::exact_from(1.5) - Float::NAN).is_nan());
    /// assert_eq!(
    ///     &Rational::exact_from(1.5) - Float::INFINITY,
    ///     Float::NEGATIVE_INFINITY
    /// );
    /// assert_eq!(
    ///     &Rational::exact_from(1.5) - Float::NEGATIVE_INFINITY,
    ///     Float::INFINITY
    /// );
    ///
    /// assert_eq!(&Rational::exact_from(1.5) - Float::from(2.5), -1.0);
    /// assert_eq!(&Rational::exact_from(1.5) - Float::from(-2.5), 4.0);
    /// assert_eq!(&Rational::exact_from(-1.5) - Float::from(2.5), -4.0);
    /// assert_eq!(&Rational::exact_from(-1.5) - Float::from(-2.5), 1.0);
    /// ```
    #[inline]
    fn sub(self, other: Float) -> Float {
        let prec = other.significant_bits();
        -other.sub_rational_prec_round_val_ref(self, prec, Nearest).0
    }
}

impl Sub<&Float> for &Rational {
    type Output = Float;

    /// Subtracts a [`Rational`] by a [`Float`], taking both by reference.
    ///
    /// If the output has a precision, it is the precision of the input [`Float`]. If the difference
    /// is equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s
    /// in its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// $$
    /// f(x,y) = x-y+\varepsilon.
    /// $$
    /// - If $x-y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-y$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2 |x-y|\rfloor-p}$,
    ///   where $p$ is the precision of the input [`Float`].
    ///
    /// Special cases:
    /// - $f(x,\text{NaN})=\text{NaN}$
    /// - $f(x,\infty)=-\infty$
    /// - $f(x,-\infty)=\infty$
    /// - $f(0,0.0)=-0.0$
    /// - $f(0,-0.0)=0.0$
    /// - $f(x,0.0)=f(x,-0.0)=x$
    /// - $f(0,x)=-x$
    /// - $f(x,x)=0.0$ if $x$ is nonzero
    ///
    /// Overflow and underflow:
    /// - If $f(x,y)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,y)\geq 2^{2^{30}-1}$, $-\infty$ is returned instead.
    /// - If $0<f(x,y)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity};
    /// use malachite_base::num::conversion::traits::ExactFrom;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// assert!((&Rational::exact_from(1.5) - &Float::NAN).is_nan());
    /// assert_eq!(
    ///     &Rational::exact_from(1.5) - &Float::INFINITY,
    ///     Float::NEGATIVE_INFINITY
    /// );
    /// assert_eq!(
    ///     &Rational::exact_from(1.5) - &Float::NEGATIVE_INFINITY,
    ///     Float::INFINITY
    /// );
    ///
    /// assert_eq!(&Rational::exact_from(1.5) - &Float::from(2.5), -1.0);
    /// assert_eq!(&Rational::exact_from(1.5) - &Float::from(-2.5), 4.0);
    /// assert_eq!(&Rational::exact_from(-1.5) - &Float::from(2.5), -4.0);
    /// assert_eq!(&Rational::exact_from(-1.5) - &Float::from(-2.5), 1.0);
    /// ```
    #[inline]
    fn sub(self, other: &Float) -> Float {
        let prec = other.significant_bits();
        -other.sub_rational_prec_round_ref_ref(self, prec, Nearest).0
    }
}
