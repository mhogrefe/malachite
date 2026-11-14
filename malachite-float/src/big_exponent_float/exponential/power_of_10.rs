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
use crate::big_exponent_float::arithmetic::mul_rational::{
    finite_to_float, from_float_result, shift_finite,
};
use crate::big_exponent_float::arithmetic::round_to_integer::finite_is_integer;
use crate::big_exponent_float::exponential::exp::{
    SAFE_EXPONENT, SAFE_EXPONENT_I64, exp_small_approx, round_nudged, ziv_loop,
};
use alloc::borrow::Cow;
use core::cmp::Ordering::{self, *};
use malachite_base::num::basic::traits::{
    Infinity as InfinityTrait, NaN as NaNTrait, One, Zero as ZeroTrait,
};
use malachite_base::num::conversion::traits::RoundingFrom;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_nz::integer::Integer;
use malachite_q::Rational;

// The largest exponent of an argument for which 10^x uses `Float`'s functions.
const SAFE_EXPONENT_10_I64: i64 = SAFE_EXPONENT_I64 - 1;

// An approximation of 10^f for a nonzero f with |f| < 1, and an `err` as in `ziv_loop`. If f is in
// `Float`'s exponent range, `Float`'s correctly rounded 10^f has an error of at most half an ulp.
// Otherwise f is tiny and 10^f = e^t with t = f log 10, which is computed with wp + 4 bits; its
// relative error of less than 2^(-wp - 3) changes e^t by less than a quarter of an ulp.
pub(crate) fn power_of_10_fraction_approx(
    f: &BigExponentFloat,
    wp: u64,
) -> (BigExponentFloat, u64) {
    let e = f.get_exponent().unwrap();
    if let Ok(e) = i32::try_from(e)
        && e >= Float::MIN_EXPONENT
    {
        let y = Float::power_of_10_of_float_prec(finite_to_float(Cow::Borrowed(f), e), wp).0;
        (BigExponentFloat::from(y), wp - 1)
    } else {
        let ln_10 = BigExponentFloat::from(Float::ln_10_prec(wp + 4).0);
        let t = f.mul_prec_ref_val(ln_10, wp + 4).0;
        let (y, err) = exp_small_approx(&t, wp);
        (y, err - 1)
    }
}

// An approximation of 10^(n + f) = 5^n 10^f 2^n, for an integer n, given an approximation of 10^f
// with an error of at most 2^(EXP - err); and an `err` for the result. 5^n is correctly rounded to
// wp bits, so its relative error is at most 2^(-wp), as is that of the product's rounding; the
// relative error of 10^f is below 2^(1 - err). For err <= wp the total relative error is a little
// more than 2^(2 - err), but less than 2^(3 - err), so the error is below 2^(EXP + 3 - err).
pub(crate) fn power_of_10_approx(
    n: &Integer,
    (y, err): (BigExponentFloat, u64),
    wp: u64,
) -> (BigExponentFloat, u64) {
    let power_of_5 = BigExponentFloat::from(5u32)
        .pow_integer_prec_val_ref(n, wp)
        .0;
    (shift_finite(power_of_5.mul_prec(y, wp).0, n), err - 3)
}

// 10^n for an integer n, rounded to `prec` bits with `rm`: 5^n is correctly rounded (and is exact
// when it can be), and multiplying it by 2^n changes neither the rounding nor the `Ordering`.
pub(crate) fn power_of_10_integer(
    n: &Integer,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let (power_of_5, o) = BigExponentFloat::from(5u32).pow_integer_prec_round_val_ref(n, prec, rm);
    (shift_finite(power_of_5, n), o)
}

// 10^x, rounded to `prec` bits with `rm`.
pub(crate) fn power_of_10_helper(
    x: Cow<'_, BigExponentFloat>,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    assert_ne!(prec, 0);
    let (sign, exponent) = match &*x {
        BigExponentFloat(NaN) => return (BigExponentFloat::NAN, Equal),
        BigExponentFloat(Infinity { sign: true }) => return (BigExponentFloat::INFINITY, Equal),
        BigExponentFloat(Infinity { sign: false }) => return (BigExponentFloat::ZERO, Equal),
        BigExponentFloat(Zero) => return (BigExponentFloat::ONE, Equal),
        BigExponentFloat(Finite { sign, exponent, .. }) => (*sign, exponent),
    };
    if finite_is_integer(&x) {
        return power_of_10_integer(&Integer::try_from(&*x).unwrap(), prec, rm);
    }
    assert_ne!(rm, Exact, "Inexact BigExponentFloat power of 10");
    // If |x| < 2^(-prec - 4), 10^x is 1 plus or minus less than 2^(-prec - 1), since log 10 < 4.
    if *exponent <= -Integer::from(prec + 4) {
        return round_nudged(&BigExponentFloat::ONE, sign, prec, rm);
    }
    if let Ok(e) = i32::try_from(exponent)
        && (Float::MIN_EXPONENT..SAFE_EXPONENT).contains(&e)
    {
        return from_float_result(
            Float::power_of_10_of_float_prec_round(finite_to_float(x, e), prec, rm),
            None,
        )
        .unwrap();
    }
    if *exponent < 0u32 {
        // x is below `Float`'s exponent range, and prec is huge.
        return ziv_loop(prec, rm, |wp| power_of_10_fraction_approx(&x, wp));
    }
    // x = n + f with an integer n and 0 < f < 1.
    let n = Integer::rounding_from(&*x, Floor).0;
    let f = x.into_owned() - BigExponentFloat::from(n.clone());
    ziv_loop(prec, rm, |wp| {
        power_of_10_approx(&n, power_of_10_fraction_approx(&f, wp), wp)
    })
}

// 10^x for a `Rational` x, rounded to `prec` bits with `rm`.
pub(crate) fn power_of_10_rational_helper(
    x: &Rational,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    assert_ne!(prec, 0);
    if let Ok(n) = Integer::try_from(x) {
        return power_of_10_integer(&n, prec, rm);
    }
    assert_ne!(rm, Exact, "Inexact BigExponentFloat power of 10");
    if x.floor_log_base_2_abs() < SAFE_EXPONENT_10_I64 {
        return from_float_result(
            Float::power_of_10_rational_prec_round_ref(x, prec, rm),
            None,
        )
        .unwrap();
    }
    // x = n + f with an integer n and 0 < f < 1.
    let n = Integer::rounding_from(x, Floor).0;
    let f = x - Rational::from(&n);
    ziv_loop(prec, rm, |wp| {
        let y = BigExponentFloat::from(Float::power_of_10_rational_prec_ref(&f, wp).0);
        power_of_10_approx(&n, (y, wp - 1), wp)
    })
}

impl BigExponentFloat {
    /// Computes $10^x$, where $x$ is a [`BigExponentFloat`], rounding the result to the specified
    /// precision and with the specified rounding mode. The [`BigExponentFloat`] is taken by value.
    /// An [`Ordering`] is also returned, indicating whether the rounded power is less than, equal
    /// to, or greater than the exact power. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
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
    /// Special cases:
    /// - $f(\text{NaN},p,m)=\text{NaN}$
    /// - $f(\infty,p,m)=\infty$
    /// - $f(-\infty,p,m)=0$
    /// - $f(0,p,m)=1$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::power_of_10_of_float_prec_round), this
    /// function never overflows or underflows.
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`BigExponentFloat::power_of_10_of_big_exponent_float_prec`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{3/2} \log n \log\log n + mn \log n \log\log n)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(prec, x.significant_bits())`, and
    /// $m$ is the number of significant bits of the integer part of $x$.
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
    /// let (p, o) = BigExponentFloat::power_of_10_of_big_exponent_float_prec_round(
    ///     BigExponentFloat::from(PI),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{p:?}"), "0x5.4E+2");
    /// assert_eq!(o, Less);
    ///
    /// let (p, o) = BigExponentFloat::power_of_10_of_big_exponent_float_prec_round(
    ///     BigExponentFloat::from(PI),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{p:?}"), "0x5.8E+2");
    /// assert_eq!(o, Greater);
    ///
    /// let (p, o) = BigExponentFloat::power_of_10_of_big_exponent_float_prec_round(
    ///     BigExponentFloat::from(PI),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{p:?}"), "0x5.8E+2");
    /// assert_eq!(o, Greater);
    ///
    /// let (p, o) = BigExponentFloat::power_of_10_of_big_exponent_float_prec_round(
    ///     BigExponentFloat::from(PI),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{p:?}"), "0x569.748");
    /// assert_eq!(o, Less);
    ///
    /// let (p, o) = BigExponentFloat::power_of_10_of_big_exponent_float_prec_round(
    ///     BigExponentFloat::from(PI),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{p:?}"), "0x569.75");
    /// assert_eq!(o, Greater);
    ///
    /// let (p, o) = BigExponentFloat::power_of_10_of_big_exponent_float_prec_round(
    ///     BigExponentFloat::from(PI),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{p:?}"), "0x569.748");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn power_of_10_of_big_exponent_float_prec_round(
        x: Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        power_of_10_helper(Cow::Owned(x), prec, rm)
    }

    /// Computes $10^x$, where $x$ is a [`BigExponentFloat`], rounding the result to the specified
    /// precision and with the specified rounding mode. The [`BigExponentFloat`] is taken by
    /// reference. An [`Ordering`] is also returned, indicating whether the rounded power is less
    /// than, equal to, or greater than the exact power. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
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
    /// Special cases:
    /// - $f(\text{NaN},p,m)=\text{NaN}$
    /// - $f(\infty,p,m)=\infty$
    /// - $f(-\infty,p,m)=0$
    /// - $f(0,p,m)=1$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::power_of_10_of_float_prec_round), this
    /// function never overflows or underflows.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{3/2} \log n \log\log n + mn \log n \log\log n)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(prec, x.significant_bits())`, and
    /// $m$ is the number of significant bits of the integer part of $x$.
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
    /// let (p, o) = BigExponentFloat::power_of_10_of_big_exponent_float_prec_round_ref(
    ///     &BigExponentFloat::from(PI),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{p:?}"), "0x5.4E+2");
    /// assert_eq!(o, Less);
    ///
    /// let (p, o) = BigExponentFloat::power_of_10_of_big_exponent_float_prec_round_ref(
    ///     &BigExponentFloat::from(PI),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{p:?}"), "0x5.8E+2");
    /// assert_eq!(o, Greater);
    ///
    /// let (p, o) = BigExponentFloat::power_of_10_of_big_exponent_float_prec_round_ref(
    ///     &BigExponentFloat::from(PI),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{p:?}"), "0x5.8E+2");
    /// assert_eq!(o, Greater);
    ///
    /// let (p, o) = BigExponentFloat::power_of_10_of_big_exponent_float_prec_round_ref(
    ///     &BigExponentFloat::from(PI),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{p:?}"), "0x569.748");
    /// assert_eq!(o, Less);
    ///
    /// let (p, o) = BigExponentFloat::power_of_10_of_big_exponent_float_prec_round_ref(
    ///     &BigExponentFloat::from(PI),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{p:?}"), "0x569.75");
    /// assert_eq!(o, Greater);
    ///
    /// let (p, o) = BigExponentFloat::power_of_10_of_big_exponent_float_prec_round_ref(
    ///     &BigExponentFloat::from(PI),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{p:?}"), "0x569.748");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn power_of_10_of_big_exponent_float_prec_round_ref(
        x: &Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        power_of_10_helper(Cow::Borrowed(x), prec, rm)
    }

    /// Computes $10^x$, where $x$ is a [`BigExponentFloat`], in place, rounding the result to the
    /// specified precision and with the specified rounding mode. An [`Ordering`] is returned,
    /// indicating whether the rounded power is less than, equal to, or greater than the exact
    /// power. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this
    /// function sets the [`BigExponentFloat`] to `NaN` it also returns `Equal`.
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
    /// Unlike [its `Float` counterpart](crate::Float::power_of_10_of_float_prec_round), this
    /// function never overflows or underflows.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{3/2} \log n \log\log n + mn \log n \log\log n)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(prec, self.significant_bits())`,
    /// and $m$ is the number of significant bits of the integer part of $x$.
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
    ///     x.power_of_10_of_big_exponent_float_prec_round_assign(5, Floor),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x5.4E+2");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.power_of_10_of_big_exponent_float_prec_round_assign(5, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(format!("{x:?}"), "0x5.8E+2");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.power_of_10_of_big_exponent_float_prec_round_assign(5, Nearest),
    ///     Greater
    /// );
    /// assert_eq!(format!("{x:?}"), "0x5.8E+2");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.power_of_10_of_big_exponent_float_prec_round_assign(20, Floor),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x569.748");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.power_of_10_of_big_exponent_float_prec_round_assign(20, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(format!("{x:?}"), "0x569.75");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.power_of_10_of_big_exponent_float_prec_round_assign(20, Nearest),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x569.748");
    /// ```
    #[inline]
    pub fn power_of_10_of_big_exponent_float_prec_round_assign(
        &mut self,
        prec: u64,
        rm: RoundingMode,
    ) -> Ordering {
        let x = core::mem::replace(self, Self::ZERO);
        let (result, o) = power_of_10_helper(Cow::Owned(x), prec, rm);
        *self = result;
        o
    }

    /// Computes $10^x$, where $x$ is a [`BigExponentFloat`], rounding the result to the nearest
    /// value of the specified precision. The [`BigExponentFloat`] is taken by value. An
    /// [`Ordering`] is also returned, indicating whether the rounded power is less than, equal to,
    /// or greater than the exact power. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,p) = 10^x+\varepsilon.
    /// $$
    /// - If $10^x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $10^x$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   10^x\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p)=\text{NaN}$
    /// - $f(\infty,p)=\infty$
    /// - $f(-\infty,p)=0$
    /// - $f(0,p)=1$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::power_of_10_of_float_prec_round), this
    /// function never overflows or underflows.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`BigExponentFloat::power_of_10_of_big_exponent_float_prec_round`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{3/2} \log n \log\log n + mn \log n \log\log n)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(prec, x.significant_bits())`, and
    /// $m$ is the number of significant bits of the integer part of $x$.
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
    /// let (p, o) =
    ///     BigExponentFloat::power_of_10_of_big_exponent_float_prec(BigExponentFloat::from(PI), 5);
    /// assert_eq!(format!("{p:?}"), "0x5.8E+2");
    /// assert_eq!(o, Greater);
    ///
    /// let (p, o) = BigExponentFloat::power_of_10_of_big_exponent_float_prec(
    ///     BigExponentFloat::from(PI),
    ///     20,
    /// );
    /// assert_eq!(format!("{p:?}"), "0x569.748");
    /// assert_eq!(o, Less);
    ///
    /// let (p, o) = BigExponentFloat::power_of_10_of_big_exponent_float_prec(
    ///     BigExponentFloat::from(1.0e10),
    ///     10,
    /// );
    /// assert_eq!(format!("{p:?}"), "0x1.d5E+8304820237");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn power_of_10_of_big_exponent_float_prec(x: Self, prec: u64) -> (Self, Ordering) {
        power_of_10_helper(Cow::Owned(x), prec, Nearest)
    }

    /// Computes $10^x$, where $x$ is a [`BigExponentFloat`], rounding the result to the nearest
    /// value of the specified precision. The [`BigExponentFloat`] is taken by reference. An
    /// [`Ordering`] is also returned, indicating whether the rounded power is less than, equal to,
    /// or greater than the exact power. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,p) = 10^x+\varepsilon.
    /// $$
    /// - If $10^x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $10^x$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   10^x\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p)=\text{NaN}$
    /// - $f(\infty,p)=\infty$
    /// - $f(-\infty,p)=0$
    /// - $f(0,p)=1$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::power_of_10_of_float_prec_round), this
    /// function never overflows or underflows.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{3/2} \log n \log\log n + mn \log n \log\log n)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(prec, x.significant_bits())`, and
    /// $m$ is the number of significant bits of the integer part of $x$.
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
    /// let (p, o) = BigExponentFloat::power_of_10_of_big_exponent_float_prec_ref(
    ///     &BigExponentFloat::from(PI),
    ///     5,
    /// );
    /// assert_eq!(format!("{p:?}"), "0x5.8E+2");
    /// assert_eq!(o, Greater);
    ///
    /// let (p, o) = BigExponentFloat::power_of_10_of_big_exponent_float_prec_ref(
    ///     &BigExponentFloat::from(PI),
    ///     20,
    /// );
    /// assert_eq!(format!("{p:?}"), "0x569.748");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn power_of_10_of_big_exponent_float_prec_ref(x: &Self, prec: u64) -> (Self, Ordering) {
        power_of_10_helper(Cow::Borrowed(x), prec, Nearest)
    }

    /// Computes $10^x$, where $x$ is a [`BigExponentFloat`], in place, rounding the result to the
    /// nearest value of the specified precision. An [`Ordering`] is returned, indicating whether
    /// the rounded power is less than, equal to, or greater than the exact power. Although `NaN`s
    /// are not comparable to any [`BigExponentFloat`], whenever this function sets the
    /// [`BigExponentFloat`] to `NaN` it also returns `Equal`.
    ///
    /// $$
    /// x \gets 10^x+\varepsilon.
    /// $$
    /// - If $10^x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $10^x$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   10^x\rfloor-p}$.
    ///
    /// Unlike [its `Float` counterpart](crate::Float::power_of_10_of_float_prec_round), this
    /// function never overflows or underflows.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{3/2} \log n \log\log n + mn \log n \log\log n)$
    ///
    /// $M(n, m) = O(n \log n + m)$
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
    /// assert_eq!(x.power_of_10_of_big_exponent_float_prec_assign(5), Greater);
    /// assert_eq!(format!("{x:?}"), "0x5.8E+2");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(x.power_of_10_of_big_exponent_float_prec_assign(20), Less);
    /// assert_eq!(format!("{x:?}"), "0x569.748");
    /// ```
    #[inline]
    pub fn power_of_10_of_big_exponent_float_prec_assign(&mut self, prec: u64) -> Ordering {
        let x = core::mem::replace(self, Self::ZERO);
        let (result, o) = power_of_10_helper(Cow::Owned(x), prec, Nearest);
        *self = result;
        o
    }

    #[allow(clippy::needless_pass_by_value)]
    /// Computes $10^x$, where $x$ is a [`Rational`], rounding the result to the specified precision
    /// and with the specified rounding mode and returning the result as a [`BigExponentFloat`]. The
    /// [`Rational`] is taken by value. An [`Ordering`] is also returned, indicating whether the
    /// rounded power is less than, equal to, or greater than the exact power.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = 10^x+\varepsilon.
    /// $$
    /// - If $10^x$ is zero, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $10^x$ is nonzero, and $m$ is not `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2
    ///   10^x\rfloor-p+1}$.
    /// - If $10^x$ is nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   10^x\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(0,p,m)=1$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::power_of_10_rational_prec_round), this
    /// function never overflows or underflows.
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`BigExponentFloat::power_of_10_rational_prec`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{3/2} \log n \log\log n + mn \log n \log\log n)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(prec, x.significant_bits())`, and
    /// $m$ is the number of significant bits of the integer part of $x$.
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
    /// let (p, o) = BigExponentFloat::power_of_10_rational_prec_round(
    ///     Rational::from_unsigneds(3u8, 5),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{p:?}"), "0x3.e");
    /// assert_eq!(o, Less);
    ///
    /// let (p, o) = BigExponentFloat::power_of_10_rational_prec_round(
    ///     Rational::from_unsigneds(3u8, 5),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{p:?}"), "0x4.0");
    /// assert_eq!(o, Greater);
    ///
    /// let (p, o) = BigExponentFloat::power_of_10_rational_prec_round(
    ///     Rational::from_unsigneds(3u8, 5),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{p:?}"), "0x4.0");
    /// assert_eq!(o, Greater);
    ///
    /// let (p, o) = BigExponentFloat::power_of_10_rational_prec_round(
    ///     Rational::from_unsigneds(3u8, 5),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{p:?}"), "0x3.fb278");
    /// assert_eq!(o, Less);
    ///
    /// let (p, o) = BigExponentFloat::power_of_10_rational_prec_round(
    ///     Rational::from_unsigneds(3u8, 5),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{p:?}"), "0x3.fb27c");
    /// assert_eq!(o, Greater);
    ///
    /// let (p, o) = BigExponentFloat::power_of_10_rational_prec_round(
    ///     Rational::from_unsigneds(3u8, 5),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{p:?}"), "0x3.fb278");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn power_of_10_rational_prec_round(
        x: Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        power_of_10_rational_helper(&x, prec, rm)
    }

    /// Computes $10^x$, where $x$ is a [`Rational`], rounding the result to the specified precision
    /// and with the specified rounding mode and returning the result as a [`BigExponentFloat`]. The
    /// [`Rational`] is taken by reference. An [`Ordering`] is also returned, indicating whether the
    /// rounded power is less than, equal to, or greater than the exact power.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = 10^x+\varepsilon.
    /// $$
    /// - If $10^x$ is zero, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $10^x$ is nonzero, and $m$ is not `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2
    ///   10^x\rfloor-p+1}$.
    /// - If $10^x$ is nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   10^x\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(0,p,m)=1$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::power_of_10_rational_prec_round), this
    /// function never overflows or underflows.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{3/2} \log n \log\log n + mn \log n \log\log n)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(prec, x.significant_bits())`, and
    /// $m$ is the number of significant bits of the integer part of $x$.
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
    /// let (p, o) = BigExponentFloat::power_of_10_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(3u8, 5),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{p:?}"), "0x3.e");
    /// assert_eq!(o, Less);
    ///
    /// let (p, o) = BigExponentFloat::power_of_10_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(3u8, 5),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{p:?}"), "0x4.0");
    /// assert_eq!(o, Greater);
    ///
    /// let (p, o) = BigExponentFloat::power_of_10_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(3u8, 5),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{p:?}"), "0x4.0");
    /// assert_eq!(o, Greater);
    ///
    /// let (p, o) = BigExponentFloat::power_of_10_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(3u8, 5),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{p:?}"), "0x3.fb278");
    /// assert_eq!(o, Less);
    ///
    /// let (p, o) = BigExponentFloat::power_of_10_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(3u8, 5),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{p:?}"), "0x3.fb27c");
    /// assert_eq!(o, Greater);
    ///
    /// let (p, o) = BigExponentFloat::power_of_10_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(3u8, 5),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{p:?}"), "0x3.fb278");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn power_of_10_rational_prec_round_ref(
        x: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        power_of_10_rational_helper(x, prec, rm)
    }

    #[allow(clippy::needless_pass_by_value)]
    /// Computes $10^x$, where $x$ is a [`Rational`], rounding the result to the nearest value of
    /// the specified precision and returning the result as a [`BigExponentFloat`]. The [`Rational`]
    /// is taken by value. An [`Ordering`] is also returned, indicating whether the rounded power is
    /// less than, equal to, or greater than the exact power.
    ///
    /// $$
    /// f(x,p) = 10^x+\varepsilon.
    /// $$
    /// - If $10^x$ is zero, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $10^x$ is nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2 10^x\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(0,p)=1$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::power_of_10_rational_prec_round), this
    /// function never overflows or underflows.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`BigExponentFloat::power_of_10_rational_prec_round`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{3/2} \log n \log\log n + mn \log n \log\log n)$
    ///
    /// $M(n, m) = O(n \log n + m)$
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
    /// let (p, o) =
    ///     BigExponentFloat::power_of_10_rational_prec(Rational::from_unsigneds(3u8, 5), 5);
    /// assert_eq!(format!("{p:?}"), "0x4.0");
    /// assert_eq!(o, Greater);
    ///
    /// let (p, o) =
    ///     BigExponentFloat::power_of_10_rational_prec(Rational::from_unsigneds(3u8, 5), 20);
    /// assert_eq!(format!("{p:?}"), "0x3.fb278");
    /// assert_eq!(o, Less);
    ///
    /// let (p, o) = BigExponentFloat::power_of_10_rational_prec(
    ///     Rational::from_unsigneds(30000000001u64, 3),
    ///     10,
    /// );
    /// assert_eq!(format!("{p:?}"), "0x3.f3E+8304820237");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn power_of_10_rational_prec(x: Rational, prec: u64) -> (Self, Ordering) {
        power_of_10_rational_helper(&x, prec, Nearest)
    }

    /// Computes $10^x$, where $x$ is a [`Rational`], rounding the result to the nearest value of
    /// the specified precision and returning the result as a [`BigExponentFloat`]. The [`Rational`]
    /// is taken by reference. An [`Ordering`] is also returned, indicating whether the rounded
    /// power is less than, equal to, or greater than the exact power.
    ///
    /// $$
    /// f(x,p) = 10^x+\varepsilon.
    /// $$
    /// - If $10^x$ is zero, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $10^x$ is nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2 10^x\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(0,p)=1$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::power_of_10_rational_prec_round), this
    /// function never overflows or underflows.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{3/2} \log n \log\log n + mn \log n \log\log n)$
    ///
    /// $M(n, m) = O(n \log n + m)$
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
    /// let (p, o) =
    ///     BigExponentFloat::power_of_10_rational_prec_ref(&Rational::from_unsigneds(3u8, 5), 5);
    /// assert_eq!(format!("{p:?}"), "0x4.0");
    /// assert_eq!(o, Greater);
    ///
    /// let (p, o) =
    ///     BigExponentFloat::power_of_10_rational_prec_ref(&Rational::from_unsigneds(3u8, 5), 20);
    /// assert_eq!(format!("{p:?}"), "0x3.fb278");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn power_of_10_rational_prec_ref(x: &Rational, prec: u64) -> (Self, Ordering) {
        power_of_10_rational_helper(x, prec, Nearest)
    }
}
