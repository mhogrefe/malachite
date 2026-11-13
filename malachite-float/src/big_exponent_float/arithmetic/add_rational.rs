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
use crate::big_exponent_float::arithmetic::add::{
    add_helper_with, round_optional, significand_prec,
};
use crate::big_exponent_float::arithmetic::mul_rational::{
    finite_to_float, float_rm, float_to_finite, from_float_result,
};
use alloc::borrow::Cow;
use core::cmp::{
    Ordering::{self, *},
    min,
};
use malachite_base::num::arithmetic::traits::{CeilingLogBase2, PowerOf2};
use malachite_base::num::basic::traits::{One, Two, Zero as ZeroTrait};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_q::Rational;

// x + y, rounded to `prec` bits with `rm`.
pub(crate) fn add_rational_helper(
    x: Cow<'_, BigExponentFloat>,
    y: Cow<'_, Rational>,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    add_rational_helper_with(x, y, false, prec, rm, true)
}

crate_test_fn! {
    // x + y rounded to `prec` bits with `rm`, without `Float`'s kernels: see
    // `add_rational_finite_generic`. Used to check and benchmark the kernel path.
    add_rational_prec_round_generic(
        x: &BigExponentFloat,
        y: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (BigExponentFloat, Ordering) {
        add_rational_helper_with(Cow::Borrowed(x), Cow::Borrowed(y), false, prec, rm, false)
    }
}

// x + y, or x - y if `negate_y` is true, rounded to `prec` bits with `rm`. The special cases are
// those of `BigExponentFloat` addition of a finite value.
//
// If y is far below x, it is first replaced by a short stand-in, as described in
// `add_rational_finite_generic`. Then, with `use_kernels`, if x's exponent is in `Float`'s range,
// the sum is computed with `Float`'s `add_rational_prec_round` or `sub_rational_prec_round`. If x's
// exponent is out of range, or the result may have overflowed or underflowed, the generic algorithm
// is used instead. (`Float`'s functions would also handle a y far below x, but their working
// precision would grow to the gap between the two.)
pub(crate) fn add_rational_helper_with(
    x: Cow<'_, BigExponentFloat>,
    y: Cow<'_, Rational>,
    negate_y: bool,
    prec: u64,
    rm: RoundingMode,
    use_kernels: bool,
) -> (BigExponentFloat, Ordering) {
    assert_ne!(prec, 0);
    match &*x {
        BigExponentFloat(NaN | Infinity { .. }) => return (x.into_owned(), Equal),
        BigExponentFloat(Zero) => {
            let y = y.into_owned();
            return BigExponentFloat::from_rational_prec_round(
                if negate_y { -y } else { y },
                prec,
                rm,
            );
        }
        BigExponentFloat(Finite { .. }) => {}
    }
    if *y == 0u32 {
        return round_optional(x.into_owned(), Some((prec, rm)));
    }
    if let Some(stand_in) = far_below_stand_in(&x, &y, negate_y, prec) {
        return add_helper_with(
            x,
            Cow::Owned(stand_in),
            false,
            Some((prec, rm)),
            use_kernels,
        );
    }
    if use_kernels
        && let Some(exponent) = x
            .get_exponent()
            .and_then(|exponent| i32::try_from(exponent).ok())
            .filter(|&exponent| exponent > Float::MIN_EXPONENT && exponent < Float::MAX_EXPONENT)
    {
        let x = finite_to_float(x, exponent);
        let result = if negate_y {
            x.sub_rational_prec_round_ref_ref(&y, prec, float_rm(rm))
        } else {
            x.add_rational_prec_round_ref_ref(&y, prec, float_rm(rm))
        };
        if let Some((sum, o)) = from_float_result(result, None) {
            assert!(
                rm != Exact || o == Equal,
                "Inexact BigExponentFloat addition"
            );
            return (sum, o);
        }
        return add_rational_finite_generic(
            &float_to_finite(x, Integer::from(exponent)),
            y,
            negate_y,
            prec,
            rm,
        );
    }
    add_rational_finite_generic(&x, y, negate_y, prec, rm)
}

// If y (negated, if `negate_y` is true) is far below the finite nonzero x, a short stand-in for it:
// let g be the smaller of x's lowest bit position and e - prec - 2, where e is x's exponent. If |y|
// < 2^g, then, as explained in `add_finite_prec_round_generic`, y may be replaced by ±2^(g - 1),
// with y's sign, which is returned.
fn far_below_stand_in(
    x: &BigExponentFloat,
    y: &Rational,
    negate_y: bool,
    prec: u64,
) -> Option<BigExponentFloat> {
    let BigExponentFloat(Finite {
        exponent,
        significand,
        ..
    }) = x
    else {
        unreachable!()
    };
    let g = min(
        exponent - Integer::from(significand_prec(significand)),
        exponent - Integer::from(prec) - Integer::TWO,
    );
    (y.floor_log_base_2_abs() < g)
        .then(|| BigExponentFloat::from_parts((*y > 0u32) != negate_y, g, Natural::ONE))
}

// x + y, or x - y if `negate_y` is true, for finite nonzero x and nonzero y not far below x (see
// `far_below_stand_in`), rounded to `prec` bits with `rm`, without `Float`'s kernels.
//
// If y's denominator is a power of 2, y is converted to a `BigExponentFloat` exactly and added.
// Otherwise the sum is never exact, and it is computed as the `Rational` x + y and rounded; but if
// x is far below y, it is first replaced by a short stand-in that rounds the same way, so that the
// `Rational` sum stays small.
//
// Let y = n / d in lowest terms, so that d is not a power of 2, and let j = floor(log_2(|y|)) -
// prec - 2. Every value near y at which rounding to `prec` bits can change (the representable
// values and, for `Nearest`, the midpoints between them) is a multiple of 2^j. For an integer c, y
// - c * 2^j is (n - c * d * 2^j) / d if j >= 0, and (n * 2^-j - c * d) * 2^j / d if j < 0; either
// numerator is a nonzero integer, since the odd part of d, which is greater than 1, divides neither
// n nor n * 2^-j. So y is at least 2^h from every such multiple, where h = min(j, 0) -
// ceil(log_2(d)). If |x| < 2^h, then x + y lies strictly between y and the nearest such multiple on
// x's side, and so does y + s, where s = ±2^(h - 1) has x's sign; the two round to the same value,
// on the same side, so x may be replaced by s.
//
// Otherwise, since y is not far below x either, x's exponent and lowest bit position are both
// within prec + |log_2(|y|)| + log_2(d) + 3 bits, plus the length of x's significand, of 0, so x is
// converted to a `Rational` exactly.
fn add_rational_finite_generic(
    x: &BigExponentFloat,
    y: Cow<'_, Rational>,
    negate_y: bool,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    if let Ok(y) = BigExponentFloat::try_from(&*y) {
        return add_helper_with(
            Cow::Borrowed(x),
            Cow::Owned(y),
            negate_y,
            Some((prec, rm)),
            false,
        );
    }
    let BigExponentFloat(Finite {
        sign: x_sign,
        exponent: x_exponent,
        ..
    }) = x
    else {
        unreachable!()
    };
    let y_log = Integer::from(y.floor_log_base_2_abs());
    let y = y.into_owned();
    let y = if negate_y { -y } else { y };
    let j = y_log - Integer::from(prec) - Integer::TWO;
    let h = min(j, Integer::ZERO) - Integer::from(y.denominator_ref().ceiling_log_base_2());
    if *x_exponent <= h {
        let s = Rational::power_of_2(i64::exact_from(&(h - Integer::ONE)));
        return BigExponentFloat::from_rational_prec_round(
            if *x_sign { y + s } else { y - s },
            prec,
            rm,
        );
    }
    BigExponentFloat::from_rational_prec_round(Rational::exact_from(x) + y, prec, rm)
}

impl BigExponentFloat {
    /// Adds a [`BigExponentFloat`] and a [`Rational`], rounding the result to the specified
    /// precision and with the specified rounding mode. The [`BigExponentFloat`] and the
    /// [`Rational`] are both taken by value. An [`Ordering`] is also returned, indicating whether
    /// the rounded sum is less than, equal to, or greater than the exact sum. Although `NaN`s are
    /// not comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,p,m) = x+y+\varepsilon.
    /// $$
    /// - If $x+y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x+y$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x+y|\rfloor-p+1}$.
    /// - If $x+y$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x+y|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,p,m)=\text{NaN}$
    /// - $f(\infty,x,p,m)=\infty$
    /// - $f(-\infty,x,p,m)=-\infty$
    /// - $f(0,0,p,m)=0$
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`BigExponentFloat::add_rational_prec`] instead.
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
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the sum cannot be represented exactly
    /// with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (result, o) = BigExponentFloat::from(PI).add_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x3.6");
    /// assert_eq!(o, Less);
    ///
    /// let (result, o) = BigExponentFloat::from(PI).add_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x3.8");
    /// assert_eq!(o, Greater);
    ///
    /// let (result, o) = BigExponentFloat::from(PI).add_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x3.8");
    /// assert_eq!(o, Greater);
    ///
    /// let (result, o) = BigExponentFloat::from(PI).add_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x3.79948");
    /// assert_eq!(o, Less);
    ///
    /// let (result, o) = BigExponentFloat::from(PI).add_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x3.7994c");
    /// assert_eq!(o, Greater);
    ///
    /// let (result, o) = BigExponentFloat::from(PI).add_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x3.7994c");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn add_rational_prec_round(
        self,
        other: Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        add_rational_helper(Cow::Owned(self), Cow::Owned(other), prec, rm)
    }

    /// Adds a [`BigExponentFloat`] and a [`Rational`], rounding the result to the specified
    /// precision and with the specified rounding mode. The [`BigExponentFloat`] is taken by value
    /// and the [`Rational`] by reference. An [`Ordering`] is also returned, indicating whether the
    /// rounded sum is less than, equal to, or greater than the exact sum. Although `NaN`s are not
    /// comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,p,m) = x+y+\varepsilon.
    /// $$
    /// - If $x+y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x+y$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x+y|\rfloor-p+1}$.
    /// - If $x+y$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x+y|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,p,m)=\text{NaN}$
    /// - $f(\infty,x,p,m)=\infty$
    /// - $f(-\infty,x,p,m)=-\infty$
    /// - $f(0,0,p,m)=0$
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
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the sum cannot be represented exactly
    /// with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (result, o) = BigExponentFloat::from(PI).add_rational_prec_round_val_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x3.6");
    /// assert_eq!(o, Less);
    ///
    /// let (result, o) = BigExponentFloat::from(PI).add_rational_prec_round_val_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x3.8");
    /// assert_eq!(o, Greater);
    ///
    /// let (result, o) = BigExponentFloat::from(PI).add_rational_prec_round_val_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x3.8");
    /// assert_eq!(o, Greater);
    ///
    /// let (result, o) = BigExponentFloat::from(PI).add_rational_prec_round_val_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x3.79948");
    /// assert_eq!(o, Less);
    ///
    /// let (result, o) = BigExponentFloat::from(PI).add_rational_prec_round_val_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x3.7994c");
    /// assert_eq!(o, Greater);
    ///
    /// let (result, o) = BigExponentFloat::from(PI).add_rational_prec_round_val_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x3.7994c");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn add_rational_prec_round_val_ref(
        self,
        other: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        add_rational_helper(Cow::Owned(self), Cow::Borrowed(other), prec, rm)
    }

    /// Adds a [`BigExponentFloat`] and a [`Rational`], rounding the result to the specified
    /// precision and with the specified rounding mode. The [`BigExponentFloat`] is taken by
    /// reference and the [`Rational`] by value. An [`Ordering`] is also returned, indicating
    /// whether the rounded sum is less than, equal to, or greater than the exact sum. Although
    /// `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function returns a
    /// `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,p,m) = x+y+\varepsilon.
    /// $$
    /// - If $x+y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x+y$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x+y|\rfloor-p+1}$.
    /// - If $x+y$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x+y|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,p,m)=\text{NaN}$
    /// - $f(\infty,x,p,m)=\infty$
    /// - $f(-\infty,x,p,m)=-\infty$
    /// - $f(0,0,p,m)=0$
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
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the sum cannot be represented exactly
    /// with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (result, o) = (&BigExponentFloat::from(PI)).add_rational_prec_round_ref_val(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x3.6");
    /// assert_eq!(o, Less);
    ///
    /// let (result, o) = (&BigExponentFloat::from(PI)).add_rational_prec_round_ref_val(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x3.8");
    /// assert_eq!(o, Greater);
    ///
    /// let (result, o) = (&BigExponentFloat::from(PI)).add_rational_prec_round_ref_val(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x3.8");
    /// assert_eq!(o, Greater);
    ///
    /// let (result, o) = (&BigExponentFloat::from(PI)).add_rational_prec_round_ref_val(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x3.79948");
    /// assert_eq!(o, Less);
    ///
    /// let (result, o) = (&BigExponentFloat::from(PI)).add_rational_prec_round_ref_val(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x3.7994c");
    /// assert_eq!(o, Greater);
    ///
    /// let (result, o) = (&BigExponentFloat::from(PI)).add_rational_prec_round_ref_val(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x3.7994c");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn add_rational_prec_round_ref_val(
        &self,
        other: Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        add_rational_helper(Cow::Borrowed(self), Cow::Owned(other), prec, rm)
    }

    /// Adds a [`BigExponentFloat`] and a [`Rational`], rounding the result to the specified
    /// precision and with the specified rounding mode. The [`BigExponentFloat`] and the
    /// [`Rational`] are both taken by reference. An [`Ordering`] is also returned, indicating
    /// whether the rounded sum is less than, equal to, or greater than the exact sum. Although
    /// `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function returns a
    /// `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,p,m) = x+y+\varepsilon.
    /// $$
    /// - If $x+y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x+y$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x+y|\rfloor-p+1}$.
    /// - If $x+y$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x+y|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,p,m)=\text{NaN}$
    /// - $f(\infty,x,p,m)=\infty$
    /// - $f(-\infty,x,p,m)=-\infty$
    /// - $f(0,0,p,m)=0$
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
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the sum cannot be represented exactly
    /// with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (result, o) = (&BigExponentFloat::from(PI)).add_rational_prec_round_ref_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x3.6");
    /// assert_eq!(o, Less);
    ///
    /// let (result, o) = (&BigExponentFloat::from(PI)).add_rational_prec_round_ref_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x3.8");
    /// assert_eq!(o, Greater);
    ///
    /// let (result, o) = (&BigExponentFloat::from(PI)).add_rational_prec_round_ref_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x3.8");
    /// assert_eq!(o, Greater);
    ///
    /// let (result, o) = (&BigExponentFloat::from(PI)).add_rational_prec_round_ref_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x3.79948");
    /// assert_eq!(o, Less);
    ///
    /// let (result, o) = (&BigExponentFloat::from(PI)).add_rational_prec_round_ref_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x3.7994c");
    /// assert_eq!(o, Greater);
    ///
    /// let (result, o) = (&BigExponentFloat::from(PI)).add_rational_prec_round_ref_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x3.7994c");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn add_rational_prec_round_ref_ref(
        &self,
        other: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        add_rational_helper(Cow::Borrowed(self), Cow::Borrowed(other), prec, rm)
    }

    /// Adds a [`Rational`] to a [`BigExponentFloat`] in place, rounding the result to the specified
    /// precision and with the specified rounding mode. The [`Rational`] is taken by value. An
    /// [`Ordering`] is returned, indicating whether the rounded sum is less than, equal to, or
    /// greater than the exact sum. Although `NaN`s are not comparable to any [`BigExponentFloat`],
    /// whenever this function sets the [`BigExponentFloat`] to `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets x+y+\varepsilon.
    /// $$
    /// - If $x+y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x+y$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x+y|\rfloor-p+1}$.
    /// - If $x+y$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x+y|\rfloor-p}$.
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
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the sum cannot be represented exactly
    /// with `prec` bits.
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
    ///     x.add_rational_prec_round_assign(Rational::from_unsigneds(1u8, 3), 5, Floor),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x3.6");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.add_rational_prec_round_assign(Rational::from_unsigneds(1u8, 3), 5, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(format!("{x:?}"), "0x3.8");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.add_rational_prec_round_assign(Rational::from_unsigneds(1u8, 3), 5, Nearest),
    ///     Greater
    /// );
    /// assert_eq!(format!("{x:?}"), "0x3.8");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.add_rational_prec_round_assign(Rational::from_unsigneds(1u8, 3), 20, Floor),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x3.79948");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.add_rational_prec_round_assign(Rational::from_unsigneds(1u8, 3), 20, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(format!("{x:?}"), "0x3.7994c");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.add_rational_prec_round_assign(Rational::from_unsigneds(1u8, 3), 20, Nearest),
    ///     Greater
    /// );
    /// assert_eq!(format!("{x:?}"), "0x3.7994c");
    /// ```
    #[inline]
    pub fn add_rational_prec_round_assign(
        &mut self,
        other: Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> Ordering {
        let x = core::mem::replace(self, Self::ZERO);
        let (result, o) = add_rational_helper(Cow::Owned(x), Cow::Owned(other), prec, rm);
        *self = result;
        o
    }

    /// Adds a [`Rational`] to a [`BigExponentFloat`] in place, rounding the result to the specified
    /// precision and with the specified rounding mode. The [`Rational`] is taken by reference. An
    /// [`Ordering`] is returned, indicating whether the rounded sum is less than, equal to, or
    /// greater than the exact sum. Although `NaN`s are not comparable to any [`BigExponentFloat`],
    /// whenever this function sets the [`BigExponentFloat`] to `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets x+y+\varepsilon.
    /// $$
    /// - If $x+y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x+y$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x+y|\rfloor-p+1}$.
    /// - If $x+y$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x+y|\rfloor-p}$.
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
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the sum cannot be represented exactly
    /// with `prec` bits.
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
    ///     x.add_rational_prec_round_assign_ref(&Rational::from_unsigneds(1u8, 3), 5, Floor),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x3.6");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.add_rational_prec_round_assign_ref(&Rational::from_unsigneds(1u8, 3), 5, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(format!("{x:?}"), "0x3.8");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.add_rational_prec_round_assign_ref(&Rational::from_unsigneds(1u8, 3), 5, Nearest),
    ///     Greater
    /// );
    /// assert_eq!(format!("{x:?}"), "0x3.8");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.add_rational_prec_round_assign_ref(&Rational::from_unsigneds(1u8, 3), 20, Floor),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x3.79948");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.add_rational_prec_round_assign_ref(&Rational::from_unsigneds(1u8, 3), 20, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(format!("{x:?}"), "0x3.7994c");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.add_rational_prec_round_assign_ref(&Rational::from_unsigneds(1u8, 3), 20, Nearest),
    ///     Greater
    /// );
    /// assert_eq!(format!("{x:?}"), "0x3.7994c");
    /// ```
    #[inline]
    pub fn add_rational_prec_round_assign_ref(
        &mut self,
        other: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> Ordering {
        let x = core::mem::replace(self, Self::ZERO);
        let (result, o) = add_rational_helper(Cow::Owned(x), Cow::Borrowed(other), prec, rm);
        *self = result;
        o
    }

    /// Adds a [`BigExponentFloat`] and a [`Rational`], rounding the result to the nearest value of
    /// the specified precision. The [`BigExponentFloat`] and the [`Rational`] are both taken by
    /// value. An [`Ordering`] is also returned, indicating whether the rounded sum is less than,
    /// equal to, or greater than the exact sum. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,y,p) = x+y+\varepsilon.
    /// $$
    /// - If $x+y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x+y$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x+y|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,p)=\text{NaN}$
    /// - $f(\infty,x,p)=\infty$
    /// - $f(-\infty,x,p)=-\infty$
    /// - $f(0,0,p)=0$
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`BigExponentFloat::add_rational_prec_round`] instead.
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
    ///     BigExponentFloat::from(PI).add_rational_prec(Rational::from_unsigneds(1u8, 3), 5);
    /// assert_eq!(format!("{result:?}"), "0x3.8");
    /// assert_eq!(o, Greater);
    ///
    /// let (result, o) =
    ///     BigExponentFloat::from(PI).add_rational_prec(Rational::from_unsigneds(1u8, 3), 20);
    /// assert_eq!(format!("{result:?}"), "0x3.7994c");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn add_rational_prec(self, other: Rational, prec: u64) -> (Self, Ordering) {
        add_rational_helper(Cow::Owned(self), Cow::Owned(other), prec, Nearest)
    }

    /// Adds a [`BigExponentFloat`] and a [`Rational`], rounding the result to the nearest value of
    /// the specified precision. The [`BigExponentFloat`] is taken by value and the [`Rational`] by
    /// reference. An [`Ordering`] is also returned, indicating whether the rounded sum is less
    /// than, equal to, or greater than the exact sum. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,y,p) = x+y+\varepsilon.
    /// $$
    /// - If $x+y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x+y$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x+y|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,p)=\text{NaN}$
    /// - $f(\infty,x,p)=\infty$
    /// - $f(-\infty,x,p)=-\infty$
    /// - $f(0,0,p)=0$
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
    ///     .add_rational_prec_val_ref(&Rational::from_unsigneds(1u8, 3), 5);
    /// assert_eq!(format!("{result:?}"), "0x3.8");
    /// assert_eq!(o, Greater);
    ///
    /// let (result, o) = BigExponentFloat::from(PI)
    ///     .add_rational_prec_val_ref(&Rational::from_unsigneds(1u8, 3), 20);
    /// assert_eq!(format!("{result:?}"), "0x3.7994c");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn add_rational_prec_val_ref(self, other: &Rational, prec: u64) -> (Self, Ordering) {
        add_rational_helper(Cow::Owned(self), Cow::Borrowed(other), prec, Nearest)
    }

    /// Adds a [`BigExponentFloat`] and a [`Rational`], rounding the result to the nearest value of
    /// the specified precision. The [`BigExponentFloat`] is taken by reference and the [`Rational`]
    /// by value. An [`Ordering`] is also returned, indicating whether the rounded sum is less than,
    /// equal to, or greater than the exact sum. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,y,p) = x+y+\varepsilon.
    /// $$
    /// - If $x+y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x+y$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x+y|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,p)=\text{NaN}$
    /// - $f(\infty,x,p)=\infty$
    /// - $f(-\infty,x,p)=-\infty$
    /// - $f(0,0,p)=0$
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
    ///     .add_rational_prec_ref_val(Rational::from_unsigneds(1u8, 3), 5);
    /// assert_eq!(format!("{result:?}"), "0x3.8");
    /// assert_eq!(o, Greater);
    ///
    /// let (result, o) = (&BigExponentFloat::from(PI))
    ///     .add_rational_prec_ref_val(Rational::from_unsigneds(1u8, 3), 20);
    /// assert_eq!(format!("{result:?}"), "0x3.7994c");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn add_rational_prec_ref_val(&self, other: Rational, prec: u64) -> (Self, Ordering) {
        add_rational_helper(Cow::Borrowed(self), Cow::Owned(other), prec, Nearest)
    }

    /// Adds a [`BigExponentFloat`] and a [`Rational`], rounding the result to the nearest value of
    /// the specified precision. The [`BigExponentFloat`] and the [`Rational`] are both taken by
    /// reference. An [`Ordering`] is also returned, indicating whether the rounded sum is less
    /// than, equal to, or greater than the exact sum. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,y,p) = x+y+\varepsilon.
    /// $$
    /// - If $x+y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x+y$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x+y|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,p)=\text{NaN}$
    /// - $f(\infty,x,p)=\infty$
    /// - $f(-\infty,x,p)=-\infty$
    /// - $f(0,0,p)=0$
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
    ///     .add_rational_prec_ref_ref(&Rational::from_unsigneds(1u8, 3), 5);
    /// assert_eq!(format!("{result:?}"), "0x3.8");
    /// assert_eq!(o, Greater);
    ///
    /// let (result, o) = (&BigExponentFloat::from(PI))
    ///     .add_rational_prec_ref_ref(&Rational::from_unsigneds(1u8, 3), 20);
    /// assert_eq!(format!("{result:?}"), "0x3.7994c");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn add_rational_prec_ref_ref(&self, other: &Rational, prec: u64) -> (Self, Ordering) {
        add_rational_helper(Cow::Borrowed(self), Cow::Borrowed(other), prec, Nearest)
    }

    /// Adds a [`Rational`] to a [`BigExponentFloat`] in place, rounding the result to the nearest
    /// value of the specified precision. The [`Rational`] is taken by value. An [`Ordering`] is
    /// returned, indicating whether the rounded sum is less than, equal to, or greater than the
    /// exact sum. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this
    /// function sets the [`BigExponentFloat`] to `NaN` it also returns `Equal`.
    ///
    /// $$
    /// x \gets x+y+\varepsilon.
    /// $$
    /// - If $x+y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x+y$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x+y|\rfloor-p}$.
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
    ///     x.add_rational_prec_assign(Rational::from_unsigneds(1u8, 3), 5),
    ///     Greater
    /// );
    /// assert_eq!(format!("{x:?}"), "0x3.8");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.add_rational_prec_assign(Rational::from_unsigneds(1u8, 3), 20),
    ///     Greater
    /// );
    /// assert_eq!(format!("{x:?}"), "0x3.7994c");
    /// ```
    #[inline]
    pub fn add_rational_prec_assign(&mut self, other: Rational, prec: u64) -> Ordering {
        let x = core::mem::replace(self, Self::ZERO);
        let (result, o) = add_rational_helper(Cow::Owned(x), Cow::Owned(other), prec, Nearest);
        *self = result;
        o
    }

    /// Adds a [`Rational`] to a [`BigExponentFloat`] in place, rounding the result to the nearest
    /// value of the specified precision. The [`Rational`] is taken by reference. An [`Ordering`] is
    /// returned, indicating whether the rounded sum is less than, equal to, or greater than the
    /// exact sum. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this
    /// function sets the [`BigExponentFloat`] to `NaN` it also returns `Equal`.
    ///
    /// $$
    /// x \gets x+y+\varepsilon.
    /// $$
    /// - If $x+y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x+y$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x+y|\rfloor-p}$.
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
    ///     x.add_rational_prec_assign_ref(&Rational::from_unsigneds(1u8, 3), 5),
    ///     Greater
    /// );
    /// assert_eq!(format!("{x:?}"), "0x3.8");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.add_rational_prec_assign_ref(&Rational::from_unsigneds(1u8, 3), 20),
    ///     Greater
    /// );
    /// assert_eq!(format!("{x:?}"), "0x3.7994c");
    /// ```
    #[inline]
    pub fn add_rational_prec_assign_ref(&mut self, other: &Rational, prec: u64) -> Ordering {
        let x = core::mem::replace(self, Self::ZERO);
        let (result, o) = add_rational_helper(Cow::Owned(x), Cow::Borrowed(other), prec, Nearest);
        *self = result;
        o
    }
}
