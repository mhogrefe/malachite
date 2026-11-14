// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Float;
use crate::big_exponent_float::BigExponentFloat;
use crate::big_exponent_float::InnerBigExponentFloat::{Finite, Infinity, NaN, Zero};
use crate::big_exponent_float::arithmetic::add::significand_prec;
use crate::big_exponent_float::arithmetic::mul_rational::{finite_to_float, from_float_result};
use crate::big_exponent_float::exponential::exp::{
    ExpArgument, MIN_SAFE_MINUS_1_EXPONENT, SAFE_EXPONENT, SAFE_EXPONENT_I64, exp_huge,
    exp_x_minus_1_series, far_below_minus_one, round_nudged, ziv_loop,
};
use alloc::borrow::Cow;
use core::cmp::{
    Ordering::{self, *},
    max,
};
use malachite_base::num::basic::traits::{
    Infinity as InfinityTrait, NaN as NaNTrait, NegativeOne, Zero as ZeroTrait,
};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_nz::integer::Integer;
use malachite_q::Rational;

// e^x - 1, rounded to `prec` bits with `rm`.
pub(crate) fn exp_x_minus_1_helper(
    x: Cow<'_, BigExponentFloat>,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    assert_ne!(prec, 0);
    let (sign, exponent, significand) = match &*x {
        BigExponentFloat(NaN) => return (BigExponentFloat::NAN, Equal),
        BigExponentFloat(Infinity { sign: true }) => return (BigExponentFloat::INFINITY, Equal),
        BigExponentFloat(Infinity { sign: false }) => {
            return (BigExponentFloat::NEGATIVE_ONE, Equal);
        }
        BigExponentFloat(Zero) => return (BigExponentFloat::ZERO, Equal),
        BigExponentFloat(Finite {
            sign,
            exponent,
            significand,
        }) => (*sign, exponent, significand),
    };
    assert_ne!(rm, Exact, "Inexact BigExponentFloat exp_x_minus_1");
    // For 0 < |x| <= 1/2, e^x - 1 = x + d with 0 < d < x^2 < 2^(2e), where e is x's exponent. If e
    // <= -max(p, q) - 2, where q is x's precision, then d < 2^(e - max(p, q) - 2).
    if *exponent <= -Integer::from(max(prec, significand_prec(significand)) + 2) {
        return round_nudged(&x, true, prec, rm);
    }
    if let Ok(e) = i32::try_from(exponent)
        && (MIN_SAFE_MINUS_1_EXPONENT..=SAFE_EXPONENT).contains(&e)
    {
        // The result is at least half of |x| in absolute value, so its exponent is at least e - 1,
        // above `Float`'s minimum.
        return from_float_result(
            finite_to_float(x, e).exp_x_minus_1_prec_round(prec, rm),
            None,
        )
        .unwrap();
    }
    if *exponent < 0u32 {
        // x is near or below the bottom of `Float`'s exponent range, and prec is huge.
        return ziv_loop(prec, rm, |wp| {
            let (s, loss) = exp_x_minus_1_series(&x, wp);
            (s, wp - loss)
        });
    }
    if !sign && far_below_minus_one(exponent, prec) {
        return round_nudged(&BigExponentFloat::NEGATIVE_ONE, true, prec, rm);
    }
    let e = u64::exact_from(exponent);
    exp_huge(&ExpArgument::BigExponentFloat(&x), e, prec, rm, true)
}

// e^x - 1 for a `Rational` x, rounded to `prec` bits with `rm`.
pub(crate) fn exp_x_minus_1_rational_helper(
    x: &Rational,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    assert_ne!(prec, 0);
    if *x == 0u32 {
        return (BigExponentFloat::ZERO, Equal);
    }
    assert_ne!(rm, Exact, "Inexact BigExponentFloat exp_x_minus_1");
    let e = x.floor_log_base_2_abs() + 1;
    if e <= SAFE_EXPONENT_I64 {
        if let Some(result) = from_float_result(
            Float::exp_x_minus_1_rational_prec_round_ref(x, prec, rm),
            None,
        ) {
            return result;
        }
        // The result underflowed `Float`'s exponent range: x is tiny. It is rounded to wp + 4 bits,
        // which adds a relative error of at most 2^(-wp - 4) to x and, with |x| tiny, about as much
        // to e^x - 1.
        return ziv_loop(prec, rm, |wp| {
            let t = BigExponentFloat::from_rational_prec_ref(x, wp + 4).0;
            let (s, loss) = exp_x_minus_1_series(&t, wp);
            (s, wp - loss - 1)
        });
    }
    let e_integer = Integer::from(e);
    if *x < 0u32 && far_below_minus_one(&e_integer, prec) {
        return round_nudged(&BigExponentFloat::NEGATIVE_ONE, true, prec, rm);
    }
    exp_huge(
        &ExpArgument::Rational(x),
        u64::exact_from(e),
        prec,
        rm,
        true,
    )
}

impl BigExponentFloat {
    /// Computes $e^x-1$, where $x$ is a [`BigExponentFloat`], rounding the result to the specified
    /// precision and with the specified rounding mode. The [`BigExponentFloat`] is taken by value.
    /// An [`Ordering`] is also returned, indicating whether the rounded value is less than, equal
    /// to, or greater than the exact value. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = e^x-1+\varepsilon.
    /// $$
    /// - If $e^x-1$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $e^x-1$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |e^x-1|\rfloor-p+1}$.
    /// - If $e^x-1$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |e^x-1|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=\text{NaN}$
    /// - $f(\infty,p,m)=\infty$
    /// - $f(-\infty,p,m)=-1$
    /// - $f(0,p,m)=0$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::exp_x_minus_1_prec_round), this function
    /// never overflows or underflows.
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`BigExponentFloat::exp_x_minus_1_prec`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{3/2} \log n \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(prec, self.significant_bits())`,
    /// and $m$ is the number of significant bits of the integer part of $x$.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with `prec` bits (which is the case for every finite nonzero input).
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (y, o) = BigExponentFloat::from(PI).exp_x_minus_1_prec_round(5, Floor);
    /// assert_eq!(format!("{y:?}"), "0x16.0");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::from(PI).exp_x_minus_1_prec_round(5, Ceiling);
    /// assert_eq!(format!("{y:?}"), "0x17.0");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::from(PI).exp_x_minus_1_prec_round(5, Nearest);
    /// assert_eq!(format!("{y:?}"), "0x16.0");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::from(PI).exp_x_minus_1_prec_round(20, Floor);
    /// assert_eq!(format!("{y:?}"), "0x16.2404");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::from(PI).exp_x_minus_1_prec_round(20, Ceiling);
    /// assert_eq!(format!("{y:?}"), "0x16.2406");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::from(PI).exp_x_minus_1_prec_round(20, Nearest);
    /// assert_eq!(format!("{y:?}"), "0x16.2404");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn exp_x_minus_1_prec_round(self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        exp_x_minus_1_helper(Cow::Owned(self), prec, rm)
    }

    /// Computes $e^x-1$, where $x$ is a [`BigExponentFloat`], rounding the result to the specified
    /// precision and with the specified rounding mode. The [`BigExponentFloat`] is taken by
    /// reference. An [`Ordering`] is also returned, indicating whether the rounded value is less
    /// than, equal to, or greater than the exact value. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = e^x-1+\varepsilon.
    /// $$
    /// - If $e^x-1$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $e^x-1$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |e^x-1|\rfloor-p+1}$.
    /// - If $e^x-1$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |e^x-1|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=\text{NaN}$
    /// - $f(\infty,p,m)=\infty$
    /// - $f(-\infty,p,m)=-1$
    /// - $f(0,p,m)=0$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::exp_x_minus_1_prec_round), this function
    /// never overflows or underflows.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{3/2} \log n \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(prec, self.significant_bits())`,
    /// and $m$ is the number of significant bits of the integer part of $x$.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with `prec` bits (which is the case for every finite nonzero input).
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (y, o) = (&BigExponentFloat::from(PI)).exp_x_minus_1_prec_round_ref(5, Floor);
    /// assert_eq!(format!("{y:?}"), "0x16.0");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = (&BigExponentFloat::from(PI)).exp_x_minus_1_prec_round_ref(5, Ceiling);
    /// assert_eq!(format!("{y:?}"), "0x17.0");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = (&BigExponentFloat::from(PI)).exp_x_minus_1_prec_round_ref(5, Nearest);
    /// assert_eq!(format!("{y:?}"), "0x16.0");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = (&BigExponentFloat::from(PI)).exp_x_minus_1_prec_round_ref(20, Floor);
    /// assert_eq!(format!("{y:?}"), "0x16.2404");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = (&BigExponentFloat::from(PI)).exp_x_minus_1_prec_round_ref(20, Ceiling);
    /// assert_eq!(format!("{y:?}"), "0x16.2406");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = (&BigExponentFloat::from(PI)).exp_x_minus_1_prec_round_ref(20, Nearest);
    /// assert_eq!(format!("{y:?}"), "0x16.2404");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn exp_x_minus_1_prec_round_ref(&self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        exp_x_minus_1_helper(Cow::Borrowed(self), prec, rm)
    }

    /// Computes $e^x-1$, where $x$ is a [`BigExponentFloat`], in place, rounding the result to the
    /// specified precision and with the specified rounding mode. An [`Ordering`] is returned,
    /// indicating whether the rounded value is less than, equal to, or greater than the exact
    /// value. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this
    /// function sets the [`BigExponentFloat`] to `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets e^x-1+\varepsilon.
    /// $$
    /// - If $e^x-1$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $e^x-1$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |e^x-1|\rfloor-p+1}$.
    /// - If $e^x-1$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |e^x-1|\rfloor-p}$.
    ///
    /// Unlike [its `Float` counterpart](crate::Float::exp_x_minus_1_prec_round), this function
    /// never overflows or underflows.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{3/2} \log n \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(prec, self.significant_bits())`,
    /// and $m$ is the number of significant bits of the integer part of $x$.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with `prec` bits (which is the case for every finite nonzero input).
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(x.exp_x_minus_1_prec_round_assign(5, Floor), Less);
    /// assert_eq!(format!("{x:?}"), "0x16.0");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(x.exp_x_minus_1_prec_round_assign(5, Ceiling), Greater);
    /// assert_eq!(format!("{x:?}"), "0x17.0");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(x.exp_x_minus_1_prec_round_assign(5, Nearest), Less);
    /// assert_eq!(format!("{x:?}"), "0x16.0");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(x.exp_x_minus_1_prec_round_assign(20, Floor), Less);
    /// assert_eq!(format!("{x:?}"), "0x16.2404");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(x.exp_x_minus_1_prec_round_assign(20, Ceiling), Greater);
    /// assert_eq!(format!("{x:?}"), "0x16.2406");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(x.exp_x_minus_1_prec_round_assign(20, Nearest), Less);
    /// assert_eq!(format!("{x:?}"), "0x16.2404");
    /// ```
    #[inline]
    pub fn exp_x_minus_1_prec_round_assign(&mut self, prec: u64, rm: RoundingMode) -> Ordering {
        let x = core::mem::replace(self, Self::ZERO);
        let (result, o) = exp_x_minus_1_helper(Cow::Owned(x), prec, rm);
        *self = result;
        o
    }

    /// Computes $e^x-1$, where $x$ is a [`BigExponentFloat`], rounding the result to the nearest
    /// value of the specified precision. The [`BigExponentFloat`] is taken by value. An
    /// [`Ordering`] is also returned, indicating whether the rounded value is less than, equal to,
    /// or greater than the exact value. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,p) = e^x-1+\varepsilon.
    /// $$
    /// - If $e^x-1$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $e^x-1$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |e^x-1|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p)=\text{NaN}$
    /// - $f(\infty,p)=\infty$
    /// - $f(-\infty,p)=-1$
    /// - $f(0,p)=0$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::exp_x_minus_1_prec_round), this function
    /// never overflows or underflows.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`BigExponentFloat::exp_x_minus_1_prec_round`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{3/2} \log n \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(prec, self.significant_bits())`,
    /// and $m$ is the number of significant bits of the integer part of $x$.
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
    /// let (y, o) = BigExponentFloat::from(PI).exp_x_minus_1_prec(5);
    /// assert_eq!(format!("{y:?}"), "0x16.0");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::from(PI).exp_x_minus_1_prec(20);
    /// assert_eq!(format!("{y:?}"), "0x16.2404");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::from(1.0e10).exp_x_minus_1_prec(10);
    /// assert_eq!(format!("{y:?}"), "0x1.da8E+3606737602");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn exp_x_minus_1_prec(self, prec: u64) -> (Self, Ordering) {
        exp_x_minus_1_helper(Cow::Owned(self), prec, Nearest)
    }

    /// Computes $e^x-1$, where $x$ is a [`BigExponentFloat`], rounding the result to the nearest
    /// value of the specified precision. The [`BigExponentFloat`] is taken by reference. An
    /// [`Ordering`] is also returned, indicating whether the rounded value is less than, equal to,
    /// or greater than the exact value. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,p) = e^x-1+\varepsilon.
    /// $$
    /// - If $e^x-1$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $e^x-1$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |e^x-1|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p)=\text{NaN}$
    /// - $f(\infty,p)=\infty$
    /// - $f(-\infty,p)=-1$
    /// - $f(0,p)=0$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::exp_x_minus_1_prec_round), this function
    /// never overflows or underflows.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{3/2} \log n \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(prec, self.significant_bits())`,
    /// and $m$ is the number of significant bits of the integer part of $x$.
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
    /// let (y, o) = (&BigExponentFloat::from(PI)).exp_x_minus_1_prec_ref(5);
    /// assert_eq!(format!("{y:?}"), "0x16.0");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = (&BigExponentFloat::from(PI)).exp_x_minus_1_prec_ref(20);
    /// assert_eq!(format!("{y:?}"), "0x16.2404");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn exp_x_minus_1_prec_ref(&self, prec: u64) -> (Self, Ordering) {
        exp_x_minus_1_helper(Cow::Borrowed(self), prec, Nearest)
    }

    /// Computes $e^x-1$, where $x$ is a [`BigExponentFloat`], in place, rounding the result to the
    /// nearest value of the specified precision. An [`Ordering`] is returned, indicating whether
    /// the rounded value is less than, equal to, or greater than the exact value. Although `NaN`s
    /// are not comparable to any [`BigExponentFloat`], whenever this function sets the
    /// [`BigExponentFloat`] to `NaN` it also returns `Equal`.
    ///
    /// $$
    /// x \gets e^x-1+\varepsilon.
    /// $$
    /// - If $e^x-1$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $e^x-1$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |e^x-1|\rfloor-p}$.
    ///
    /// Unlike [its `Float` counterpart](crate::Float::exp_x_minus_1_prec_round), this function
    /// never overflows or underflows.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{3/2} \log n \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(prec, self.significant_bits())`,
    /// and $m$ is the number of significant bits of the integer part of $x$.
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
    /// assert_eq!(x.exp_x_minus_1_prec_assign(5), Less);
    /// assert_eq!(format!("{x:?}"), "0x16.0");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(x.exp_x_minus_1_prec_assign(20), Less);
    /// assert_eq!(format!("{x:?}"), "0x16.2404");
    /// ```
    #[inline]
    pub fn exp_x_minus_1_prec_assign(&mut self, prec: u64) -> Ordering {
        let x = core::mem::replace(self, Self::ZERO);
        let (result, o) = exp_x_minus_1_helper(Cow::Owned(x), prec, Nearest);
        *self = result;
        o
    }

    #[allow(clippy::needless_pass_by_value)]
    /// Computes $e^x-1$, where $x$ is a [`Rational`], rounding the result to the specified
    /// precision and with the specified rounding mode and returning the result as a
    /// [`BigExponentFloat`]. The [`Rational`] is taken by value. An [`Ordering`] is also returned,
    /// indicating whether the rounded value is less than, equal to, or greater than the exact
    /// value.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = e^x-1+\varepsilon.
    /// $$
    /// - If $e^x-1$ is zero, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $e^x-1$ is nonzero, and $m$ is not `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2
    ///   |e^x-1|\rfloor-p+1}$.
    /// - If $e^x-1$ is nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |e^x-1|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(0,p,m)=0$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::exp_x_minus_1_rational_prec_round), this
    /// function never overflows or underflows.
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`BigExponentFloat::exp_x_minus_1_rational_prec`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{3/2} \log n \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(prec, x.significant_bits())`, and
    /// $m$ is the number of significant bits of the integer part of $x$.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with `prec` bits (which is the case for every nonzero input).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (y, o) = BigExponentFloat::exp_x_minus_1_rational_prec_round(
    ///     Rational::from_unsigneds(3u8, 5),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{y:?}"), "0x0.d");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::exp_x_minus_1_rational_prec_round(
    ///     Rational::from_unsigneds(3u8, 5),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{y:?}"), "0x0.d8");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::exp_x_minus_1_rational_prec_round(
    ///     Rational::from_unsigneds(3u8, 5),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{y:?}"), "0x0.d");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::exp_x_minus_1_rational_prec_round(
    ///     Rational::from_unsigneds(3u8, 5),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{y:?}"), "0x0.d2766");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::exp_x_minus_1_rational_prec_round(
    ///     Rational::from_unsigneds(3u8, 5),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{y:?}"), "0x0.d2767");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::exp_x_minus_1_rational_prec_round(
    ///     Rational::from_unsigneds(3u8, 5),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{y:?}"), "0x0.d2766");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn exp_x_minus_1_rational_prec_round(
        x: Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        exp_x_minus_1_rational_helper(&x, prec, rm)
    }

    /// Computes $e^x-1$, where $x$ is a [`Rational`], rounding the result to the specified
    /// precision and with the specified rounding mode and returning the result as a
    /// [`BigExponentFloat`]. The [`Rational`] is taken by reference. An [`Ordering`] is also
    /// returned, indicating whether the rounded value is less than, equal to, or greater than the
    /// exact value.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = e^x-1+\varepsilon.
    /// $$
    /// - If $e^x-1$ is zero, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $e^x-1$ is nonzero, and $m$ is not `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2
    ///   |e^x-1|\rfloor-p+1}$.
    /// - If $e^x-1$ is nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |e^x-1|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(0,p,m)=0$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::exp_x_minus_1_rational_prec_round), this
    /// function never overflows or underflows.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{3/2} \log n \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(prec, x.significant_bits())`, and
    /// $m$ is the number of significant bits of the integer part of $x$.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with `prec` bits (which is the case for every nonzero input).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (y, o) = BigExponentFloat::exp_x_minus_1_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(3u8, 5),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{y:?}"), "0x0.d");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::exp_x_minus_1_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(3u8, 5),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{y:?}"), "0x0.d8");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::exp_x_minus_1_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(3u8, 5),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{y:?}"), "0x0.d");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::exp_x_minus_1_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(3u8, 5),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{y:?}"), "0x0.d2766");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::exp_x_minus_1_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(3u8, 5),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{y:?}"), "0x0.d2767");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::exp_x_minus_1_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(3u8, 5),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{y:?}"), "0x0.d2766");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn exp_x_minus_1_rational_prec_round_ref(
        x: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        exp_x_minus_1_rational_helper(x, prec, rm)
    }

    #[allow(clippy::needless_pass_by_value)]
    /// Computes $e^x-1$, where $x$ is a [`Rational`], rounding the result to the nearest value of
    /// the specified precision and returning the result as a [`BigExponentFloat`]. The [`Rational`]
    /// is taken by value. An [`Ordering`] is also returned, indicating whether the rounded value is
    /// less than, equal to, or greater than the exact value.
    ///
    /// $$
    /// f(x,p) = e^x-1+\varepsilon.
    /// $$
    /// - If $e^x-1$ is zero, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $e^x-1$ is nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2 |e^x-1|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(0,p)=0$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::exp_x_minus_1_rational_prec_round), this
    /// function never overflows or underflows.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`BigExponentFloat::exp_x_minus_1_rational_prec_round`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{3/2} \log n \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(prec, x.significant_bits())`, and
    /// $m$ is the number of significant bits of the integer part of $x$.
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
    /// let (y, o) =
    ///     BigExponentFloat::exp_x_minus_1_rational_prec(Rational::from_unsigneds(3u8, 5), 5);
    /// assert_eq!(format!("{y:?}"), "0x0.d");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) =
    ///     BigExponentFloat::exp_x_minus_1_rational_prec(Rational::from_unsigneds(3u8, 5), 20);
    /// assert_eq!(format!("{y:?}"), "0x0.d2766");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::exp_x_minus_1_rational_prec(
    ///     Rational::from_unsigneds(30000000001u64, 3),
    ///     10,
    /// );
    /// assert_eq!(format!("{y:?}"), "0x2.96E+3606737602");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn exp_x_minus_1_rational_prec(x: Rational, prec: u64) -> (Self, Ordering) {
        exp_x_minus_1_rational_helper(&x, prec, Nearest)
    }

    /// Computes $e^x-1$, where $x$ is a [`Rational`], rounding the result to the nearest value of
    /// the specified precision and returning the result as a [`BigExponentFloat`]. The [`Rational`]
    /// is taken by reference. An [`Ordering`] is also returned, indicating whether the rounded
    /// value is less than, equal to, or greater than the exact value.
    ///
    /// $$
    /// f(x,p) = e^x-1+\varepsilon.
    /// $$
    /// - If $e^x-1$ is zero, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $e^x-1$ is nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2 |e^x-1|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(0,p)=0$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::exp_x_minus_1_rational_prec_round), this
    /// function never overflows or underflows.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{3/2} \log n \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(prec, x.significant_bits())`, and
    /// $m$ is the number of significant bits of the integer part of $x$.
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
    /// let (y, o) =
    ///     BigExponentFloat::exp_x_minus_1_rational_prec_ref(&Rational::from_unsigneds(3u8, 5), 5);
    /// assert_eq!(format!("{y:?}"), "0x0.d");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::exp_x_minus_1_rational_prec_ref(
    ///     &Rational::from_unsigneds(3u8, 5),
    ///     20,
    /// );
    /// assert_eq!(format!("{y:?}"), "0x0.d2766");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn exp_x_minus_1_rational_prec_ref(x: &Rational, prec: u64) -> (Self, Ordering) {
        exp_x_minus_1_rational_helper(x, prec, Nearest)
    }
}
