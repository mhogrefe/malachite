// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::big_exponent_float::BigExponentFloat;
use crate::big_exponent_float::InnerBigExponentFloat::{Infinity, Zero};
use crate::big_exponent_float::arithmetic::mul_rational::{
    finite_to_float, finite_to_integer_and_shift, float_rm, float_to_finite, from_float_result,
    shift_finite,
};
use alloc::borrow::Cow;
use core::cmp::Ordering::{self, *};
use malachite_base::num::basic::traits::{
    Infinity as InfinityTrait, NaN as NaNTrait, NegativeInfinity, Zero as ZeroTrait,
};
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_q::Rational;

// x / y, rounded to `prec` bits with `rm`.
pub(crate) fn div_rational_helper(
    x: Cow<'_, BigExponentFloat>,
    y: Cow<'_, Rational>,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    div_rational_helper_with(x, y, prec, rm, true)
}

crate_test_fn! {
    // x / y rounded to `prec` bits with `rm`, without `Float`'s kernels: with x = m * 2^k for an
    // integer m, the `Rational` m / y is rounded and then multiplied by 2^k. Used to check and
    // benchmark the kernel path.
    div_rational_prec_round_generic(
        x: &BigExponentFloat,
        y: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (BigExponentFloat, Ordering) {
        div_rational_helper_with(Cow::Borrowed(x), Cow::Borrowed(y), prec, rm, false)
    }
}

// The special cases are those of `BigExponentFloat` division by a finite value. Since there is no
// negative zero, division by zero gives NaN.
//
// With `use_kernels`, x is scaled to have exponent 0 and divided by y with `Float`'s
// `div_rational_prec_round`, and the scaling is undone. If that may have overflowed or underflowed,
// which can only happen if y is extremely large or small, the generic algorithm is used instead.
fn div_rational_helper_with(
    x: Cow<'_, BigExponentFloat>,
    y: Cow<'_, Rational>,
    prec: u64,
    rm: RoundingMode,
    use_kernels: bool,
) -> (BigExponentFloat, Ordering) {
    assert_ne!(prec, 0);
    if x.is_nan() || *y == 0u32 {
        return (BigExponentFloat::NAN, Equal);
    }
    match &*x {
        BigExponentFloat(Infinity { sign }) => {
            return (
                if *sign == (*y > 0u32) {
                    BigExponentFloat::INFINITY
                } else {
                    BigExponentFloat::NEGATIVE_INFINITY
                },
                Equal,
            );
        }
        BigExponentFloat(Zero) => return (BigExponentFloat::ZERO, Equal),
        _ => {}
    }
    if !use_kernels {
        return div_rational_finite_generic(&x, y, prec, rm);
    }
    let exponent = x.get_exponent().unwrap().clone();
    let x = finite_to_float(x, 0);
    if let Some((quotient, o)) = from_float_result(
        x.div_rational_prec_round_ref_ref(&y, prec, float_rm(rm)),
        Some(&exponent),
    ) {
        assert!(
            rm != Exact || o == Equal,
            "Inexact BigExponentFloat division"
        );
        return (quotient, o);
    }
    div_rational_finite_generic(&float_to_finite(x, exponent), y, prec, rm)
}

// x / y for finite nonzero x and nonzero y, by the generic algorithm.
fn div_rational_finite_generic(
    x: &BigExponentFloat,
    y: Cow<'_, Rational>,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let (m, k) = finite_to_integer_and_shift(x);
    let m = Rational::from(m);
    let quotient = match y {
        Cow::Owned(y) => m / y,
        Cow::Borrowed(y) => m / y,
    };
    let (quotient, o) = BigExponentFloat::from_rational_prec_round(quotient, prec, rm);
    (shift_finite(quotient, &k), o)
}

impl BigExponentFloat {
    /// Divides a [`BigExponentFloat`] by a [`Rational`], rounding the result to the specified
    /// precision and with the specified rounding mode. The [`BigExponentFloat`] and the
    /// [`Rational`] are both taken by value. An [`Ordering`] is also returned, indicating whether
    /// the rounded quotient is less than, equal to, or greater than the exact quotient. Although
    /// `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function returns a
    /// `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,p,m) = x/y+\varepsilon.
    /// $$
    /// - If $x/y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x/y$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x/y|\rfloor-p+1}$.
    /// - If $x/y$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x/y|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,p,m)=f(x,0,p,m)=\text{NaN}$
    /// - $f(\infty,x,p,m)=\infty$ if $x>0$
    /// - $f(\infty,x,p,m)=-\infty$ if $x<0$
    /// - $f(-\infty,x,p,m)=-\infty$ if $x>0$
    /// - $f(-\infty,x,p,m)=\infty$ if $x<0$
    /// - $f(0,x,p,m)=0$ if $x\neq0$
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`BigExponentFloat::div_rational_prec`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the quotient cannot be represented
    /// exactly with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (result, o) = BigExponentFloat::from(PI).div_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x9.0");
    /// assert_eq!(o, Less);
    ///
    /// let (result, o) = BigExponentFloat::from(PI).div_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x9.8");
    /// assert_eq!(o, Greater);
    ///
    /// let (result, o) = BigExponentFloat::from(PI).div_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x9.8");
    /// assert_eq!(o, Greater);
    ///
    /// let (result, o) = BigExponentFloat::from(PI).div_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x9.6cbe");
    /// assert_eq!(o, Less);
    ///
    /// let (result, o) = BigExponentFloat::from(PI).div_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x9.6cbf");
    /// assert_eq!(o, Greater);
    ///
    /// let (result, o) = BigExponentFloat::from(PI).div_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x9.6cbe");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn div_rational_prec_round(
        self,
        other: Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        div_rational_helper(Cow::Owned(self), Cow::Owned(other), prec, rm)
    }

    /// Divides a [`BigExponentFloat`] by a [`Rational`], rounding the result to the specified
    /// precision and with the specified rounding mode. The [`BigExponentFloat`] is taken by value
    /// and the [`Rational`] by reference. An [`Ordering`] is also returned, indicating whether the
    /// rounded quotient is less than, equal to, or greater than the exact quotient. Although `NaN`s
    /// are not comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it
    /// also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,p,m) = x/y+\varepsilon.
    /// $$
    /// - If $x/y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x/y$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x/y|\rfloor-p+1}$.
    /// - If $x/y$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x/y|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,p,m)=f(x,0,p,m)=\text{NaN}$
    /// - $f(\infty,x,p,m)=\infty$ if $x>0$
    /// - $f(\infty,x,p,m)=-\infty$ if $x<0$
    /// - $f(-\infty,x,p,m)=-\infty$ if $x>0$
    /// - $f(-\infty,x,p,m)=\infty$ if $x<0$
    /// - $f(0,x,p,m)=0$ if $x\neq0$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the quotient cannot be represented
    /// exactly with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (result, o) = BigExponentFloat::from(PI).div_rational_prec_round_val_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x9.0");
    /// assert_eq!(o, Less);
    ///
    /// let (result, o) = BigExponentFloat::from(PI).div_rational_prec_round_val_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x9.8");
    /// assert_eq!(o, Greater);
    ///
    /// let (result, o) = BigExponentFloat::from(PI).div_rational_prec_round_val_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x9.8");
    /// assert_eq!(o, Greater);
    ///
    /// let (result, o) = BigExponentFloat::from(PI).div_rational_prec_round_val_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x9.6cbe");
    /// assert_eq!(o, Less);
    ///
    /// let (result, o) = BigExponentFloat::from(PI).div_rational_prec_round_val_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x9.6cbf");
    /// assert_eq!(o, Greater);
    ///
    /// let (result, o) = BigExponentFloat::from(PI).div_rational_prec_round_val_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x9.6cbe");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn div_rational_prec_round_val_ref(
        self,
        other: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        div_rational_helper(Cow::Owned(self), Cow::Borrowed(other), prec, rm)
    }

    /// Divides a [`BigExponentFloat`] by a [`Rational`], rounding the result to the specified
    /// precision and with the specified rounding mode. The [`BigExponentFloat`] is taken by
    /// reference and the [`Rational`] by value. An [`Ordering`] is also returned, indicating
    /// whether the rounded quotient is less than, equal to, or greater than the exact quotient.
    /// Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,p,m) = x/y+\varepsilon.
    /// $$
    /// - If $x/y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x/y$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x/y|\rfloor-p+1}$.
    /// - If $x/y$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x/y|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,p,m)=f(x,0,p,m)=\text{NaN}$
    /// - $f(\infty,x,p,m)=\infty$ if $x>0$
    /// - $f(\infty,x,p,m)=-\infty$ if $x<0$
    /// - $f(-\infty,x,p,m)=-\infty$ if $x>0$
    /// - $f(-\infty,x,p,m)=\infty$ if $x<0$
    /// - $f(0,x,p,m)=0$ if $x\neq0$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the quotient cannot be represented
    /// exactly with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (result, o) = (&BigExponentFloat::from(PI)).div_rational_prec_round_ref_val(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x9.0");
    /// assert_eq!(o, Less);
    ///
    /// let (result, o) = (&BigExponentFloat::from(PI)).div_rational_prec_round_ref_val(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x9.8");
    /// assert_eq!(o, Greater);
    ///
    /// let (result, o) = (&BigExponentFloat::from(PI)).div_rational_prec_round_ref_val(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x9.8");
    /// assert_eq!(o, Greater);
    ///
    /// let (result, o) = (&BigExponentFloat::from(PI)).div_rational_prec_round_ref_val(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x9.6cbe");
    /// assert_eq!(o, Less);
    ///
    /// let (result, o) = (&BigExponentFloat::from(PI)).div_rational_prec_round_ref_val(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x9.6cbf");
    /// assert_eq!(o, Greater);
    ///
    /// let (result, o) = (&BigExponentFloat::from(PI)).div_rational_prec_round_ref_val(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x9.6cbe");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn div_rational_prec_round_ref_val(
        &self,
        other: Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        div_rational_helper(Cow::Borrowed(self), Cow::Owned(other), prec, rm)
    }

    /// Divides a [`BigExponentFloat`] by a [`Rational`], rounding the result to the specified
    /// precision and with the specified rounding mode. The [`BigExponentFloat`] and the
    /// [`Rational`] are both taken by reference. An [`Ordering`] is also returned, indicating
    /// whether the rounded quotient is less than, equal to, or greater than the exact quotient.
    /// Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,p,m) = x/y+\varepsilon.
    /// $$
    /// - If $x/y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x/y$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x/y|\rfloor-p+1}$.
    /// - If $x/y$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x/y|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,p,m)=f(x,0,p,m)=\text{NaN}$
    /// - $f(\infty,x,p,m)=\infty$ if $x>0$
    /// - $f(\infty,x,p,m)=-\infty$ if $x<0$
    /// - $f(-\infty,x,p,m)=-\infty$ if $x>0$
    /// - $f(-\infty,x,p,m)=\infty$ if $x<0$
    /// - $f(0,x,p,m)=0$ if $x\neq0$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the quotient cannot be represented
    /// exactly with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (result, o) = (&BigExponentFloat::from(PI)).div_rational_prec_round_ref_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x9.0");
    /// assert_eq!(o, Less);
    ///
    /// let (result, o) = (&BigExponentFloat::from(PI)).div_rational_prec_round_ref_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x9.8");
    /// assert_eq!(o, Greater);
    ///
    /// let (result, o) = (&BigExponentFloat::from(PI)).div_rational_prec_round_ref_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x9.8");
    /// assert_eq!(o, Greater);
    ///
    /// let (result, o) = (&BigExponentFloat::from(PI)).div_rational_prec_round_ref_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x9.6cbe");
    /// assert_eq!(o, Less);
    ///
    /// let (result, o) = (&BigExponentFloat::from(PI)).div_rational_prec_round_ref_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x9.6cbf");
    /// assert_eq!(o, Greater);
    ///
    /// let (result, o) = (&BigExponentFloat::from(PI)).div_rational_prec_round_ref_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x9.6cbe");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn div_rational_prec_round_ref_ref(
        &self,
        other: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        div_rational_helper(Cow::Borrowed(self), Cow::Borrowed(other), prec, rm)
    }

    /// Divides a [`BigExponentFloat`] by a [`Rational`] in place, rounding the result to the
    /// specified precision and with the specified rounding mode. The [`Rational`] is taken by
    /// value. An [`Ordering`] is returned, indicating whether the rounded quotient is less than,
    /// equal to, or greater than the exact quotient. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function sets the [`BigExponentFloat`] to `NaN` it also
    /// returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets x/y+\varepsilon.
    /// $$
    /// - If $x/y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x/y$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x/y|\rfloor-p+1}$.
    /// - If $x/y$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x/y|\rfloor-p}$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the quotient cannot be represented
    /// exactly with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.div_rational_prec_round_assign(Rational::from_unsigneds(1u8, 3), 5, Floor),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x9.0");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.div_rational_prec_round_assign(Rational::from_unsigneds(1u8, 3), 5, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(format!("{x:?}"), "0x9.8");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.div_rational_prec_round_assign(Rational::from_unsigneds(1u8, 3), 5, Nearest),
    ///     Greater
    /// );
    /// assert_eq!(format!("{x:?}"), "0x9.8");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.div_rational_prec_round_assign(Rational::from_unsigneds(1u8, 3), 20, Floor),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x9.6cbe");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.div_rational_prec_round_assign(Rational::from_unsigneds(1u8, 3), 20, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(format!("{x:?}"), "0x9.6cbf");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.div_rational_prec_round_assign(Rational::from_unsigneds(1u8, 3), 20, Nearest),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x9.6cbe");
    /// ```
    #[inline]
    pub fn div_rational_prec_round_assign(
        &mut self,
        other: Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> Ordering {
        let x = core::mem::replace(self, Self::ZERO);
        let (result, o) = div_rational_helper(Cow::Owned(x), Cow::Owned(other), prec, rm);
        *self = result;
        o
    }

    /// Divides a [`BigExponentFloat`] by a [`Rational`] in place, rounding the result to the
    /// specified precision and with the specified rounding mode. The [`Rational`] is taken by
    /// reference. An [`Ordering`] is returned, indicating whether the rounded quotient is less
    /// than, equal to, or greater than the exact quotient. Although `NaN`s are not comparable to
    /// any [`BigExponentFloat`], whenever this function sets the [`BigExponentFloat`] to `NaN` it
    /// also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets x/y+\varepsilon.
    /// $$
    /// - If $x/y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x/y$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x/y|\rfloor-p+1}$.
    /// - If $x/y$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x/y|\rfloor-p}$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the quotient cannot be represented
    /// exactly with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.div_rational_prec_round_assign_ref(&Rational::from_unsigneds(1u8, 3), 5, Floor),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x9.0");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.div_rational_prec_round_assign_ref(&Rational::from_unsigneds(1u8, 3), 5, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(format!("{x:?}"), "0x9.8");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.div_rational_prec_round_assign_ref(&Rational::from_unsigneds(1u8, 3), 5, Nearest),
    ///     Greater
    /// );
    /// assert_eq!(format!("{x:?}"), "0x9.8");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.div_rational_prec_round_assign_ref(&Rational::from_unsigneds(1u8, 3), 20, Floor),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x9.6cbe");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.div_rational_prec_round_assign_ref(&Rational::from_unsigneds(1u8, 3), 20, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(format!("{x:?}"), "0x9.6cbf");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.div_rational_prec_round_assign_ref(&Rational::from_unsigneds(1u8, 3), 20, Nearest),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x9.6cbe");
    /// ```
    #[inline]
    pub fn div_rational_prec_round_assign_ref(
        &mut self,
        other: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> Ordering {
        let x = core::mem::replace(self, Self::ZERO);
        let (result, o) = div_rational_helper(Cow::Owned(x), Cow::Borrowed(other), prec, rm);
        *self = result;
        o
    }

    /// Divides a [`BigExponentFloat`] by a [`Rational`], rounding the result to the nearest value
    /// of the specified precision. The [`BigExponentFloat`] and the [`Rational`] are both taken by
    /// value. An [`Ordering`] is also returned, indicating whether the rounded quotient is less
    /// than, equal to, or greater than the exact quotient. Although `NaN`s are not comparable to
    /// any [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,y,p) = x/y+\varepsilon.
    /// $$
    /// - If $x/y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x/y$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x/y|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,p)=f(x,0,p)=\text{NaN}$
    /// - $f(\infty,x,p)=\infty$ if $x>0$
    /// - $f(\infty,x,p)=-\infty$ if $x<0$
    /// - $f(-\infty,x,p)=-\infty$ if $x>0$
    /// - $f(-\infty,x,p)=\infty$ if $x<0$
    /// - $f(0,x,p)=0$ if $x\neq0$
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`BigExponentFloat::div_rational_prec_round`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (result, o) =
    ///     BigExponentFloat::from(PI).div_rational_prec(Rational::from_unsigneds(1u8, 3), 5);
    /// assert_eq!(format!("{result:?}"), "0x9.8");
    /// assert_eq!(o, Greater);
    ///
    /// let (result, o) =
    ///     BigExponentFloat::from(PI).div_rational_prec(Rational::from_unsigneds(1u8, 3), 20);
    /// assert_eq!(format!("{result:?}"), "0x9.6cbe");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn div_rational_prec(self, other: Rational, prec: u64) -> (Self, Ordering) {
        div_rational_helper(Cow::Owned(self), Cow::Owned(other), prec, Nearest)
    }

    /// Divides a [`BigExponentFloat`] by a [`Rational`], rounding the result to the nearest value
    /// of the specified precision. The [`BigExponentFloat`] is taken by value and the [`Rational`]
    /// by reference. An [`Ordering`] is also returned, indicating whether the rounded quotient is
    /// less than, equal to, or greater than the exact quotient. Although `NaN`s are not comparable
    /// to any [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,y,p) = x/y+\varepsilon.
    /// $$
    /// - If $x/y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x/y$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x/y|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,p)=f(x,0,p)=\text{NaN}$
    /// - $f(\infty,x,p)=\infty$ if $x>0$
    /// - $f(\infty,x,p)=-\infty$ if $x<0$
    /// - $f(-\infty,x,p)=-\infty$ if $x>0$
    /// - $f(-\infty,x,p)=\infty$ if $x<0$
    /// - $f(0,x,p)=0$ if $x\neq0$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (result, o) = BigExponentFloat::from(PI)
    ///     .div_rational_prec_val_ref(&Rational::from_unsigneds(1u8, 3), 5);
    /// assert_eq!(format!("{result:?}"), "0x9.8");
    /// assert_eq!(o, Greater);
    ///
    /// let (result, o) = BigExponentFloat::from(PI)
    ///     .div_rational_prec_val_ref(&Rational::from_unsigneds(1u8, 3), 20);
    /// assert_eq!(format!("{result:?}"), "0x9.6cbe");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn div_rational_prec_val_ref(self, other: &Rational, prec: u64) -> (Self, Ordering) {
        div_rational_helper(Cow::Owned(self), Cow::Borrowed(other), prec, Nearest)
    }

    /// Divides a [`BigExponentFloat`] by a [`Rational`], rounding the result to the nearest value
    /// of the specified precision. The [`BigExponentFloat`] is taken by reference and the
    /// [`Rational`] by value. An [`Ordering`] is also returned, indicating whether the rounded
    /// quotient is less than, equal to, or greater than the exact quotient. Although `NaN`s are not
    /// comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// $$
    /// f(x,y,p) = x/y+\varepsilon.
    /// $$
    /// - If $x/y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x/y$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x/y|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,p)=f(x,0,p)=\text{NaN}$
    /// - $f(\infty,x,p)=\infty$ if $x>0$
    /// - $f(\infty,x,p)=-\infty$ if $x<0$
    /// - $f(-\infty,x,p)=-\infty$ if $x>0$
    /// - $f(-\infty,x,p)=\infty$ if $x<0$
    /// - $f(0,x,p)=0$ if $x\neq0$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (result, o) = (&BigExponentFloat::from(PI))
    ///     .div_rational_prec_ref_val(Rational::from_unsigneds(1u8, 3), 5);
    /// assert_eq!(format!("{result:?}"), "0x9.8");
    /// assert_eq!(o, Greater);
    ///
    /// let (result, o) = (&BigExponentFloat::from(PI))
    ///     .div_rational_prec_ref_val(Rational::from_unsigneds(1u8, 3), 20);
    /// assert_eq!(format!("{result:?}"), "0x9.6cbe");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn div_rational_prec_ref_val(&self, other: Rational, prec: u64) -> (Self, Ordering) {
        div_rational_helper(Cow::Borrowed(self), Cow::Owned(other), prec, Nearest)
    }

    /// Divides a [`BigExponentFloat`] by a [`Rational`], rounding the result to the nearest value
    /// of the specified precision. The [`BigExponentFloat`] and the [`Rational`] are both taken by
    /// reference. An [`Ordering`] is also returned, indicating whether the rounded quotient is less
    /// than, equal to, or greater than the exact quotient. Although `NaN`s are not comparable to
    /// any [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,y,p) = x/y+\varepsilon.
    /// $$
    /// - If $x/y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x/y$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x/y|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,p)=f(x,0,p)=\text{NaN}$
    /// - $f(\infty,x,p)=\infty$ if $x>0$
    /// - $f(\infty,x,p)=-\infty$ if $x<0$
    /// - $f(-\infty,x,p)=-\infty$ if $x>0$
    /// - $f(-\infty,x,p)=\infty$ if $x<0$
    /// - $f(0,x,p)=0$ if $x\neq0$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (result, o) = (&BigExponentFloat::from(PI))
    ///     .div_rational_prec_ref_ref(&Rational::from_unsigneds(1u8, 3), 5);
    /// assert_eq!(format!("{result:?}"), "0x9.8");
    /// assert_eq!(o, Greater);
    ///
    /// let (result, o) = (&BigExponentFloat::from(PI))
    ///     .div_rational_prec_ref_ref(&Rational::from_unsigneds(1u8, 3), 20);
    /// assert_eq!(format!("{result:?}"), "0x9.6cbe");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn div_rational_prec_ref_ref(&self, other: &Rational, prec: u64) -> (Self, Ordering) {
        div_rational_helper(Cow::Borrowed(self), Cow::Borrowed(other), prec, Nearest)
    }

    /// Divides a [`BigExponentFloat`] by a [`Rational`] in place, rounding the result to the
    /// nearest value of the specified precision. The [`Rational`] is taken by value. An
    /// [`Ordering`] is returned, indicating whether the rounded quotient is less than, equal to, or
    /// greater than the exact quotient. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function sets the [`BigExponentFloat`] to `NaN` it also
    /// returns `Equal`.
    ///
    /// $$
    /// x \gets x/y+\varepsilon.
    /// $$
    /// - If $x/y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x/y$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x/y|\rfloor-p}$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.div_rational_prec_assign(Rational::from_unsigneds(1u8, 3), 5),
    ///     Greater
    /// );
    /// assert_eq!(format!("{x:?}"), "0x9.8");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.div_rational_prec_assign(Rational::from_unsigneds(1u8, 3), 20),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x9.6cbe");
    /// ```
    #[inline]
    pub fn div_rational_prec_assign(&mut self, other: Rational, prec: u64) -> Ordering {
        let x = core::mem::replace(self, Self::ZERO);
        let (result, o) = div_rational_helper(Cow::Owned(x), Cow::Owned(other), prec, Nearest);
        *self = result;
        o
    }

    /// Divides a [`BigExponentFloat`] by a [`Rational`] in place, rounding the result to the
    /// nearest value of the specified precision. The [`Rational`] is taken by reference. An
    /// [`Ordering`] is returned, indicating whether the rounded quotient is less than, equal to, or
    /// greater than the exact quotient. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function sets the [`BigExponentFloat`] to `NaN` it also
    /// returns `Equal`.
    ///
    /// $$
    /// x \gets x/y+\varepsilon.
    /// $$
    /// - If $x/y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x/y$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x/y|\rfloor-p}$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.div_rational_prec_assign_ref(&Rational::from_unsigneds(1u8, 3), 5),
    ///     Greater
    /// );
    /// assert_eq!(format!("{x:?}"), "0x9.8");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.div_rational_prec_assign_ref(&Rational::from_unsigneds(1u8, 3), 20),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x9.6cbe");
    /// ```
    #[inline]
    pub fn div_rational_prec_assign_ref(&mut self, other: &Rational, prec: u64) -> Ordering {
        let x = core::mem::replace(self, Self::ZERO);
        let (result, o) = div_rational_helper(Cow::Owned(x), Cow::Borrowed(other), prec, Nearest);
        *self = result;
        o
    }
}
