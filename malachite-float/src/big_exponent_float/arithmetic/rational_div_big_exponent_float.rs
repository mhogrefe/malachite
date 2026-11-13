// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Float;
use crate::big_exponent_float::BigExponentFloat;
use crate::big_exponent_float::InnerBigExponentFloat::{Infinity, NaN, Zero};
use crate::big_exponent_float::arithmetic::mul_rational::{
    finite_to_float, finite_to_integer_and_shift, float_rm, float_to_finite, from_float_result,
    shift_finite,
};
use alloc::borrow::Cow;
use core::cmp::Ordering::{self, *};
use malachite_base::num::basic::traits::{NaN as NaNTrait, Zero as ZeroTrait};
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_q::Rational;

// x / y, for a `Rational` x, rounded to `prec` bits with `rm`.
pub(crate) fn rational_div_helper(
    x: Cow<'_, Rational>,
    y: Cow<'_, BigExponentFloat>,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    rational_div_helper_with(x, y, prec, rm, true)
}

crate_test_fn! {
    // x / y, for a `Rational` x, rounded to `prec` bits with `rm`, without `Float`'s kernels: with
    // y = m * 2^k for an integer m, the `Rational` x / m is rounded and then multiplied by 2^-k.
    // Used to check and benchmark the kernel path.
    rational_div_big_exponent_float_prec_round_generic(
        x: &Rational,
        y: &BigExponentFloat,
        prec: u64,
        rm: RoundingMode,
    ) -> (BigExponentFloat, Ordering) {
        rational_div_helper_with(Cow::Borrowed(x), Cow::Borrowed(y), prec, rm, false)
    }
}

// The special cases are those of `BigExponentFloat` division of a finite value. Since there is no
// negative zero, division by zero gives NaN.
//
// With `use_kernels`, y is scaled to have exponent 0, x is divided by it with `Float`'s
// `rational_div_float_prec_round`, and the scaling is undone. If that may have overflowed or
// underflowed, which can only happen if x is extremely large or small, the generic algorithm is
// used instead.
fn rational_div_helper_with(
    x: Cow<'_, Rational>,
    y: Cow<'_, BigExponentFloat>,
    prec: u64,
    rm: RoundingMode,
    use_kernels: bool,
) -> (BigExponentFloat, Ordering) {
    assert_ne!(prec, 0);
    match &*y {
        BigExponentFloat(NaN | Zero) => return (BigExponentFloat::NAN, Equal),
        BigExponentFloat(Infinity { .. }) => return (BigExponentFloat::ZERO, Equal),
        _ => {}
    }
    if *x == 0u32 {
        return (BigExponentFloat::ZERO, Equal);
    }
    if !use_kernels {
        return rational_div_finite_generic(x, &y, prec, rm);
    }
    let exponent = y.get_exponent().unwrap().clone();
    let y = finite_to_float(y, 0);
    if let Some((quotient, o)) = from_float_result(
        Float::rational_div_float_prec_round_ref_ref(&x, &y, prec, float_rm(rm)),
        Some(&-&exponent),
    ) {
        assert!(
            rm != Exact || o == Equal,
            "Inexact BigExponentFloat division"
        );
        return (quotient, o);
    }
    rational_div_finite_generic(x, &float_to_finite(y, exponent), prec, rm)
}

// x / y for nonzero x and finite nonzero y, by the generic algorithm.
fn rational_div_finite_generic(
    x: Cow<'_, Rational>,
    y: &BigExponentFloat,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let (m, k) = finite_to_integer_and_shift(y);
    let m = Rational::from(m);
    let quotient = match x {
        Cow::Owned(x) => x / m,
        Cow::Borrowed(x) => x / m,
    };
    let (quotient, o) = BigExponentFloat::from_rational_prec_round(quotient, prec, rm);
    (shift_finite(quotient, &-k), o)
}

impl BigExponentFloat {
    /// Divides a [`Rational`] by a [`BigExponentFloat`], rounding the result to the specified
    /// precision and with the specified rounding mode. The [`Rational`] and the
    /// [`BigExponentFloat`] are both taken by value. An [`Ordering`] is also returned, indicating
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
    /// - $f(x,\text{NaN},p,m)=f(x,0,p,m)=\text{NaN}$
    /// - $f(x,\pm\infty,p,m)=0$
    /// - $f(0,x,p,m)=0$ if $x$ is not NaN or 0
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`BigExponentFloat::rational_div_big_exponent_float_prec`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(x.significant_bits(),
    /// y.significant_bits(), prec)`.
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
    /// let (result, o) = BigExponentFloat::rational_div_big_exponent_float_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     BigExponentFloat::from(PI),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x0.1b");
    /// assert_eq!(o, Less);
    ///
    /// let (result, o) = BigExponentFloat::rational_div_big_exponent_float_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     BigExponentFloat::from(PI),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x0.1c");
    /// assert_eq!(o, Greater);
    ///
    /// let (result, o) = BigExponentFloat::rational_div_big_exponent_float_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     BigExponentFloat::from(PI),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x0.1b");
    /// assert_eq!(o, Less);
    ///
    /// let (result, o) = BigExponentFloat::rational_div_big_exponent_float_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     BigExponentFloat::from(PI),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x0.1b2994");
    /// assert_eq!(o, Less);
    ///
    /// let (result, o) = BigExponentFloat::rational_div_big_exponent_float_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     BigExponentFloat::from(PI),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x0.1b2996");
    /// assert_eq!(o, Greater);
    ///
    /// let (result, o) = BigExponentFloat::rational_div_big_exponent_float_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     BigExponentFloat::from(PI),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x0.1b2996");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn rational_div_big_exponent_float_prec_round(
        x: Rational,
        y: Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        rational_div_helper(Cow::Owned(x), Cow::Owned(y), prec, rm)
    }

    /// Divides a [`Rational`] by a [`BigExponentFloat`], rounding the result to the specified
    /// precision and with the specified rounding mode. The [`Rational`] is taken by value and the
    /// [`BigExponentFloat`] by reference. An [`Ordering`] is also returned, indicating whether the
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
    /// - $f(x,\text{NaN},p,m)=f(x,0,p,m)=\text{NaN}$
    /// - $f(x,\pm\infty,p,m)=0$
    /// - $f(0,x,p,m)=0$ if $x$ is not NaN or 0
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(x.significant_bits(),
    /// y.significant_bits(), prec)`.
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
    /// let (result, o) = BigExponentFloat::rational_div_big_exponent_float_prec_round_val_ref(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     &BigExponentFloat::from(PI),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x0.1b");
    /// assert_eq!(o, Less);
    ///
    /// let (result, o) = BigExponentFloat::rational_div_big_exponent_float_prec_round_val_ref(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     &BigExponentFloat::from(PI),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x0.1c");
    /// assert_eq!(o, Greater);
    ///
    /// let (result, o) = BigExponentFloat::rational_div_big_exponent_float_prec_round_val_ref(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     &BigExponentFloat::from(PI),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x0.1b");
    /// assert_eq!(o, Less);
    ///
    /// let (result, o) = BigExponentFloat::rational_div_big_exponent_float_prec_round_val_ref(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     &BigExponentFloat::from(PI),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x0.1b2994");
    /// assert_eq!(o, Less);
    ///
    /// let (result, o) = BigExponentFloat::rational_div_big_exponent_float_prec_round_val_ref(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     &BigExponentFloat::from(PI),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x0.1b2996");
    /// assert_eq!(o, Greater);
    ///
    /// let (result, o) = BigExponentFloat::rational_div_big_exponent_float_prec_round_val_ref(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     &BigExponentFloat::from(PI),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x0.1b2996");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn rational_div_big_exponent_float_prec_round_val_ref(
        x: Rational,
        y: &Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        rational_div_helper(Cow::Owned(x), Cow::Borrowed(y), prec, rm)
    }

    /// Divides a [`Rational`] by a [`BigExponentFloat`], rounding the result to the specified
    /// precision and with the specified rounding mode. The [`Rational`] is taken by reference and
    /// the [`BigExponentFloat`] by value. An [`Ordering`] is also returned, indicating whether the
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
    /// - $f(x,\text{NaN},p,m)=f(x,0,p,m)=\text{NaN}$
    /// - $f(x,\pm\infty,p,m)=0$
    /// - $f(0,x,p,m)=0$ if $x$ is not NaN or 0
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(x.significant_bits(),
    /// y.significant_bits(), prec)`.
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
    /// let (result, o) = BigExponentFloat::rational_div_big_exponent_float_prec_round_ref_val(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     BigExponentFloat::from(PI),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x0.1b");
    /// assert_eq!(o, Less);
    ///
    /// let (result, o) = BigExponentFloat::rational_div_big_exponent_float_prec_round_ref_val(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     BigExponentFloat::from(PI),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x0.1c");
    /// assert_eq!(o, Greater);
    ///
    /// let (result, o) = BigExponentFloat::rational_div_big_exponent_float_prec_round_ref_val(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     BigExponentFloat::from(PI),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x0.1b");
    /// assert_eq!(o, Less);
    ///
    /// let (result, o) = BigExponentFloat::rational_div_big_exponent_float_prec_round_ref_val(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     BigExponentFloat::from(PI),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x0.1b2994");
    /// assert_eq!(o, Less);
    ///
    /// let (result, o) = BigExponentFloat::rational_div_big_exponent_float_prec_round_ref_val(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     BigExponentFloat::from(PI),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x0.1b2996");
    /// assert_eq!(o, Greater);
    ///
    /// let (result, o) = BigExponentFloat::rational_div_big_exponent_float_prec_round_ref_val(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     BigExponentFloat::from(PI),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x0.1b2996");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn rational_div_big_exponent_float_prec_round_ref_val(
        x: &Rational,
        y: Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        rational_div_helper(Cow::Borrowed(x), Cow::Owned(y), prec, rm)
    }

    /// Divides a [`Rational`] by a [`BigExponentFloat`], rounding the result to the specified
    /// precision and with the specified rounding mode. The [`Rational`] and the
    /// [`BigExponentFloat`] are both taken by reference. An [`Ordering`] is also returned,
    /// indicating whether the rounded quotient is less than, equal to, or greater than the exact
    /// quotient. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
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
    /// - $f(x,\text{NaN},p,m)=f(x,0,p,m)=\text{NaN}$
    /// - $f(x,\pm\infty,p,m)=0$
    /// - $f(0,x,p,m)=0$ if $x$ is not NaN or 0
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(x.significant_bits(),
    /// y.significant_bits(), prec)`.
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
    /// let (result, o) = BigExponentFloat::rational_div_big_exponent_float_prec_round_ref_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     &BigExponentFloat::from(PI),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x0.1b");
    /// assert_eq!(o, Less);
    ///
    /// let (result, o) = BigExponentFloat::rational_div_big_exponent_float_prec_round_ref_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     &BigExponentFloat::from(PI),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x0.1c");
    /// assert_eq!(o, Greater);
    ///
    /// let (result, o) = BigExponentFloat::rational_div_big_exponent_float_prec_round_ref_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     &BigExponentFloat::from(PI),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x0.1b");
    /// assert_eq!(o, Less);
    ///
    /// let (result, o) = BigExponentFloat::rational_div_big_exponent_float_prec_round_ref_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     &BigExponentFloat::from(PI),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x0.1b2994");
    /// assert_eq!(o, Less);
    ///
    /// let (result, o) = BigExponentFloat::rational_div_big_exponent_float_prec_round_ref_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     &BigExponentFloat::from(PI),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x0.1b2996");
    /// assert_eq!(o, Greater);
    ///
    /// let (result, o) = BigExponentFloat::rational_div_big_exponent_float_prec_round_ref_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     &BigExponentFloat::from(PI),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x0.1b2996");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn rational_div_big_exponent_float_prec_round_ref_ref(
        x: &Rational,
        y: &Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        rational_div_helper(Cow::Borrowed(x), Cow::Borrowed(y), prec, rm)
    }

    /// Divides a [`Rational`] by a [`BigExponentFloat`], rounding the result to the nearest value
    /// of the specified precision. The [`Rational`] and the [`BigExponentFloat`] are both taken by
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
    /// - $f(x,\text{NaN},p)=f(x,0,p)=\text{NaN}$
    /// - $f(x,\pm\infty,p)=0$
    /// - $f(0,x,p)=0$ if $x$ is not NaN or 0
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`BigExponentFloat::rational_div_big_exponent_float_prec_round`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(x.significant_bits(),
    /// y.significant_bits(), prec)`.
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
    /// let (result, o) = BigExponentFloat::rational_div_big_exponent_float_prec(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     BigExponentFloat::from(PI),
    ///     5,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x0.1b");
    /// assert_eq!(o, Less);
    ///
    /// let (result, o) = BigExponentFloat::rational_div_big_exponent_float_prec(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     BigExponentFloat::from(PI),
    ///     20,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x0.1b2996");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn rational_div_big_exponent_float_prec(
        x: Rational,
        y: Self,
        prec: u64,
    ) -> (Self, Ordering) {
        rational_div_helper(Cow::Owned(x), Cow::Owned(y), prec, Nearest)
    }

    /// Divides a [`Rational`] by a [`BigExponentFloat`], rounding the result to the nearest value
    /// of the specified precision. The [`Rational`] is taken by value and the [`BigExponentFloat`]
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
    /// - $f(x,\text{NaN},p)=f(x,0,p)=\text{NaN}$
    /// - $f(x,\pm\infty,p)=0$
    /// - $f(0,x,p)=0$ if $x$ is not NaN or 0
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(x.significant_bits(),
    /// y.significant_bits(), prec)`.
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
    /// let (result, o) = BigExponentFloat::rational_div_big_exponent_float_prec_val_ref(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     &BigExponentFloat::from(PI),
    ///     5,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x0.1b");
    /// assert_eq!(o, Less);
    ///
    /// let (result, o) = BigExponentFloat::rational_div_big_exponent_float_prec_val_ref(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     &BigExponentFloat::from(PI),
    ///     20,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x0.1b2996");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn rational_div_big_exponent_float_prec_val_ref(
        x: Rational,
        y: &Self,
        prec: u64,
    ) -> (Self, Ordering) {
        rational_div_helper(Cow::Owned(x), Cow::Borrowed(y), prec, Nearest)
    }

    /// Divides a [`Rational`] by a [`BigExponentFloat`], rounding the result to the nearest value
    /// of the specified precision. The [`Rational`] is taken by reference and the
    /// [`BigExponentFloat`] by value. An [`Ordering`] is also returned, indicating whether the
    /// rounded quotient is less than, equal to, or greater than the exact quotient. Although `NaN`s
    /// are not comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it
    /// also returns `Equal`.
    ///
    /// $$
    /// f(x,y,p) = x/y+\varepsilon.
    /// $$
    /// - If $x/y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x/y$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x/y|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,\text{NaN},p)=f(x,0,p)=\text{NaN}$
    /// - $f(x,\pm\infty,p)=0$
    /// - $f(0,x,p)=0$ if $x$ is not NaN or 0
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(x.significant_bits(),
    /// y.significant_bits(), prec)`.
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
    /// let (result, o) = BigExponentFloat::rational_div_big_exponent_float_prec_ref_val(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     BigExponentFloat::from(PI),
    ///     5,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x0.1b");
    /// assert_eq!(o, Less);
    ///
    /// let (result, o) = BigExponentFloat::rational_div_big_exponent_float_prec_ref_val(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     BigExponentFloat::from(PI),
    ///     20,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x0.1b2996");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn rational_div_big_exponent_float_prec_ref_val(
        x: &Rational,
        y: Self,
        prec: u64,
    ) -> (Self, Ordering) {
        rational_div_helper(Cow::Borrowed(x), Cow::Owned(y), prec, Nearest)
    }

    /// Divides a [`Rational`] by a [`BigExponentFloat`], rounding the result to the nearest value
    /// of the specified precision. The [`Rational`] and the [`BigExponentFloat`] are both taken by
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
    /// - $f(x,\text{NaN},p)=f(x,0,p)=\text{NaN}$
    /// - $f(x,\pm\infty,p)=0$
    /// - $f(0,x,p)=0$ if $x$ is not NaN or 0
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(x.significant_bits(),
    /// y.significant_bits(), prec)`.
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
    /// let (result, o) = BigExponentFloat::rational_div_big_exponent_float_prec_ref_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     &BigExponentFloat::from(PI),
    ///     5,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x0.1b");
    /// assert_eq!(o, Less);
    ///
    /// let (result, o) = BigExponentFloat::rational_div_big_exponent_float_prec_ref_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     &BigExponentFloat::from(PI),
    ///     20,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x0.1b2996");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn rational_div_big_exponent_float_prec_ref_ref(
        x: &Rational,
        y: &Self,
        prec: u64,
    ) -> (Self, Ordering) {
        rational_div_helper(Cow::Borrowed(x), Cow::Borrowed(y), prec, Nearest)
    }
}
