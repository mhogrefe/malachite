// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::big_exponent_float::BigExponentFloat;
use crate::big_exponent_float::arithmetic::add_rational::add_rational_helper_with;
use alloc::borrow::Cow;
use core::cmp::Ordering;
use malachite_base::num::basic::traits::Zero;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_q::Rational;

// x - y, rounded to `prec` bits with `rm`.
pub(crate) fn sub_rational_helper(
    x: Cow<'_, BigExponentFloat>,
    y: Cow<'_, Rational>,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    add_rational_helper_with(x, y, true, prec, rm, true)
}

crate_test_fn! {
    // x - y rounded to `prec` bits with `rm`, without `Float`'s kernels. Used to check and
    // benchmark the kernel path.
    sub_rational_prec_round_generic(
        x: &BigExponentFloat,
        y: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (BigExponentFloat, Ordering) {
        add_rational_helper_with(Cow::Borrowed(x), Cow::Borrowed(y), true, prec, rm, false)
    }
}

impl BigExponentFloat {
    /// Subtracts a [`Rational`] from a [`BigExponentFloat`], rounding the result to the specified
    /// precision and with the specified rounding mode. The [`BigExponentFloat`] and the
    /// [`Rational`] are both taken by value. An [`Ordering`] is also returned, indicating whether
    /// the rounded difference is less than, equal to, or greater than the exact difference.
    /// Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,p,m) = x-y+\varepsilon.
    /// $$
    /// - If $x-y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-y$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x-y|\rfloor-p+1}$.
    /// - If $x-y$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x-y|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,p,m)=\text{NaN}$
    /// - $f(\infty,x,p,m)=\infty$
    /// - $f(-\infty,x,p,m)=-\infty$
    /// - $f(0,0,p,m)=0$
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`BigExponentFloat::sub_rational_prec`] instead.
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
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the difference cannot be represented
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
    /// let (result, o) = BigExponentFloat::from(PI).sub_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x2.c");
    /// assert_eq!(o, Less);
    ///
    /// let (result, o) = BigExponentFloat::from(PI).sub_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x2.e");
    /// assert_eq!(o, Greater);
    ///
    /// let (result, o) = BigExponentFloat::from(PI).sub_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x2.c");
    /// assert_eq!(o, Less);
    ///
    /// let (result, o) = BigExponentFloat::from(PI).sub_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x2.ceea");
    /// assert_eq!(o, Less);
    ///
    /// let (result, o) = BigExponentFloat::from(PI).sub_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x2.ceea4");
    /// assert_eq!(o, Greater);
    ///
    /// let (result, o) = BigExponentFloat::from(PI).sub_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x2.ceea");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn sub_rational_prec_round(
        self,
        other: Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        sub_rational_helper(Cow::Owned(self), Cow::Owned(other), prec, rm)
    }

    /// Subtracts a [`Rational`] from a [`BigExponentFloat`], rounding the result to the specified
    /// precision and with the specified rounding mode. The [`BigExponentFloat`] is taken by value
    /// and the [`Rational`] by reference. An [`Ordering`] is also returned, indicating whether the
    /// rounded difference is less than, equal to, or greater than the exact difference. Although
    /// `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function returns a
    /// `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,p,m) = x-y+\varepsilon.
    /// $$
    /// - If $x-y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-y$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x-y|\rfloor-p+1}$.
    /// - If $x-y$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x-y|\rfloor-p}$.
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
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the difference cannot be represented
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
    /// let (result, o) = BigExponentFloat::from(PI).sub_rational_prec_round_val_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x2.c");
    /// assert_eq!(o, Less);
    ///
    /// let (result, o) = BigExponentFloat::from(PI).sub_rational_prec_round_val_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x2.e");
    /// assert_eq!(o, Greater);
    ///
    /// let (result, o) = BigExponentFloat::from(PI).sub_rational_prec_round_val_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x2.c");
    /// assert_eq!(o, Less);
    ///
    /// let (result, o) = BigExponentFloat::from(PI).sub_rational_prec_round_val_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x2.ceea");
    /// assert_eq!(o, Less);
    ///
    /// let (result, o) = BigExponentFloat::from(PI).sub_rational_prec_round_val_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x2.ceea4");
    /// assert_eq!(o, Greater);
    ///
    /// let (result, o) = BigExponentFloat::from(PI).sub_rational_prec_round_val_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x2.ceea");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn sub_rational_prec_round_val_ref(
        self,
        other: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        sub_rational_helper(Cow::Owned(self), Cow::Borrowed(other), prec, rm)
    }

    /// Subtracts a [`Rational`] from a [`BigExponentFloat`], rounding the result to the specified
    /// precision and with the specified rounding mode. The [`BigExponentFloat`] is taken by
    /// reference and the [`Rational`] by value. An [`Ordering`] is also returned, indicating
    /// whether the rounded difference is less than, equal to, or greater than the exact difference.
    /// Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,p,m) = x-y+\varepsilon.
    /// $$
    /// - If $x-y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-y$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x-y|\rfloor-p+1}$.
    /// - If $x-y$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x-y|\rfloor-p}$.
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
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the difference cannot be represented
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
    /// let (result, o) = (&BigExponentFloat::from(PI)).sub_rational_prec_round_ref_val(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x2.c");
    /// assert_eq!(o, Less);
    ///
    /// let (result, o) = (&BigExponentFloat::from(PI)).sub_rational_prec_round_ref_val(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x2.e");
    /// assert_eq!(o, Greater);
    ///
    /// let (result, o) = (&BigExponentFloat::from(PI)).sub_rational_prec_round_ref_val(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x2.c");
    /// assert_eq!(o, Less);
    ///
    /// let (result, o) = (&BigExponentFloat::from(PI)).sub_rational_prec_round_ref_val(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x2.ceea");
    /// assert_eq!(o, Less);
    ///
    /// let (result, o) = (&BigExponentFloat::from(PI)).sub_rational_prec_round_ref_val(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x2.ceea4");
    /// assert_eq!(o, Greater);
    ///
    /// let (result, o) = (&BigExponentFloat::from(PI)).sub_rational_prec_round_ref_val(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x2.ceea");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn sub_rational_prec_round_ref_val(
        &self,
        other: Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        sub_rational_helper(Cow::Borrowed(self), Cow::Owned(other), prec, rm)
    }

    /// Subtracts a [`Rational`] from a [`BigExponentFloat`], rounding the result to the specified
    /// precision and with the specified rounding mode. The [`BigExponentFloat`] and the
    /// [`Rational`] are both taken by reference. An [`Ordering`] is also returned, indicating
    /// whether the rounded difference is less than, equal to, or greater than the exact difference.
    /// Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,p,m) = x-y+\varepsilon.
    /// $$
    /// - If $x-y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-y$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x-y|\rfloor-p+1}$.
    /// - If $x-y$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x-y|\rfloor-p}$.
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
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the difference cannot be represented
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
    /// let (result, o) = (&BigExponentFloat::from(PI)).sub_rational_prec_round_ref_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x2.c");
    /// assert_eq!(o, Less);
    ///
    /// let (result, o) = (&BigExponentFloat::from(PI)).sub_rational_prec_round_ref_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x2.e");
    /// assert_eq!(o, Greater);
    ///
    /// let (result, o) = (&BigExponentFloat::from(PI)).sub_rational_prec_round_ref_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x2.c");
    /// assert_eq!(o, Less);
    ///
    /// let (result, o) = (&BigExponentFloat::from(PI)).sub_rational_prec_round_ref_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x2.ceea");
    /// assert_eq!(o, Less);
    ///
    /// let (result, o) = (&BigExponentFloat::from(PI)).sub_rational_prec_round_ref_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x2.ceea4");
    /// assert_eq!(o, Greater);
    ///
    /// let (result, o) = (&BigExponentFloat::from(PI)).sub_rational_prec_round_ref_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x2.ceea");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn sub_rational_prec_round_ref_ref(
        &self,
        other: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        sub_rational_helper(Cow::Borrowed(self), Cow::Borrowed(other), prec, rm)
    }

    /// Subtracts a [`Rational`] from a [`BigExponentFloat`] in place, rounding the result to the
    /// specified precision and with the specified rounding mode. The [`Rational`] is taken by
    /// value. An [`Ordering`] is returned, indicating whether the rounded difference is less than,
    /// equal to, or greater than the exact difference. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function sets the [`BigExponentFloat`] to `NaN` it also
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
    /// - If $x-y$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x-y|\rfloor-p}$.
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
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the difference cannot be represented
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
    ///     x.sub_rational_prec_round_assign(Rational::from_unsigneds(1u8, 3), 5, Floor),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x2.c");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.sub_rational_prec_round_assign(Rational::from_unsigneds(1u8, 3), 5, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(format!("{x:?}"), "0x2.e");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.sub_rational_prec_round_assign(Rational::from_unsigneds(1u8, 3), 5, Nearest),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x2.c");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.sub_rational_prec_round_assign(Rational::from_unsigneds(1u8, 3), 20, Floor),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x2.ceea");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.sub_rational_prec_round_assign(Rational::from_unsigneds(1u8, 3), 20, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(format!("{x:?}"), "0x2.ceea4");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.sub_rational_prec_round_assign(Rational::from_unsigneds(1u8, 3), 20, Nearest),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x2.ceea");
    /// ```
    #[inline]
    pub fn sub_rational_prec_round_assign(
        &mut self,
        other: Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> Ordering {
        let x = core::mem::replace(self, Self::ZERO);
        let (result, o) = sub_rational_helper(Cow::Owned(x), Cow::Owned(other), prec, rm);
        *self = result;
        o
    }

    /// Subtracts a [`Rational`] from a [`BigExponentFloat`] in place, rounding the result to the
    /// specified precision and with the specified rounding mode. The [`Rational`] is taken by
    /// reference. An [`Ordering`] is returned, indicating whether the rounded difference is less
    /// than, equal to, or greater than the exact difference. Although `NaN`s are not comparable to
    /// any [`BigExponentFloat`], whenever this function sets the [`BigExponentFloat`] to `NaN` it
    /// also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets x-y+\varepsilon.
    /// $$
    /// - If $x-y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-y$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x-y|\rfloor-p+1}$.
    /// - If $x-y$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x-y|\rfloor-p}$.
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
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the difference cannot be represented
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
    ///     x.sub_rational_prec_round_assign_ref(&Rational::from_unsigneds(1u8, 3), 5, Floor),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x2.c");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.sub_rational_prec_round_assign_ref(&Rational::from_unsigneds(1u8, 3), 5, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(format!("{x:?}"), "0x2.e");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.sub_rational_prec_round_assign_ref(&Rational::from_unsigneds(1u8, 3), 5, Nearest),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x2.c");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.sub_rational_prec_round_assign_ref(&Rational::from_unsigneds(1u8, 3), 20, Floor),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x2.ceea");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.sub_rational_prec_round_assign_ref(&Rational::from_unsigneds(1u8, 3), 20, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(format!("{x:?}"), "0x2.ceea4");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.sub_rational_prec_round_assign_ref(&Rational::from_unsigneds(1u8, 3), 20, Nearest),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x2.ceea");
    /// ```
    #[inline]
    pub fn sub_rational_prec_round_assign_ref(
        &mut self,
        other: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> Ordering {
        let x = core::mem::replace(self, Self::ZERO);
        let (result, o) = sub_rational_helper(Cow::Owned(x), Cow::Borrowed(other), prec, rm);
        *self = result;
        o
    }

    /// Subtracts a [`Rational`] from a [`BigExponentFloat`], rounding the result to the nearest
    /// value of the specified precision. The [`BigExponentFloat`] and the [`Rational`] are both
    /// taken by value. An [`Ordering`] is also returned, indicating whether the rounded difference
    /// is less than, equal to, or greater than the exact difference. Although `NaN`s are not
    /// comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// $$
    /// f(x,y,p) = x-y+\varepsilon.
    /// $$
    /// - If $x-y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-y$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x-y|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,p)=\text{NaN}$
    /// - $f(\infty,x,p)=\infty$
    /// - $f(-\infty,x,p)=-\infty$
    /// - $f(0,0,p)=0$
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`BigExponentFloat::sub_rational_prec_round`] instead.
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
    ///     BigExponentFloat::from(PI).sub_rational_prec(Rational::from_unsigneds(1u8, 3), 5);
    /// assert_eq!(format!("{result:?}"), "0x2.c");
    /// assert_eq!(o, Less);
    ///
    /// let (result, o) =
    ///     BigExponentFloat::from(PI).sub_rational_prec(Rational::from_unsigneds(1u8, 3), 20);
    /// assert_eq!(format!("{result:?}"), "0x2.ceea");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn sub_rational_prec(self, other: Rational, prec: u64) -> (Self, Ordering) {
        sub_rational_helper(Cow::Owned(self), Cow::Owned(other), prec, Nearest)
    }

    /// Subtracts a [`Rational`] from a [`BigExponentFloat`], rounding the result to the nearest
    /// value of the specified precision. The [`BigExponentFloat`] is taken by value and the
    /// [`Rational`] by reference. An [`Ordering`] is also returned, indicating whether the rounded
    /// difference is less than, equal to, or greater than the exact difference. Although `NaN`s are
    /// not comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// $$
    /// f(x,y,p) = x-y+\varepsilon.
    /// $$
    /// - If $x-y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-y$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x-y|\rfloor-p}$.
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
    ///     .sub_rational_prec_val_ref(&Rational::from_unsigneds(1u8, 3), 5);
    /// assert_eq!(format!("{result:?}"), "0x2.c");
    /// assert_eq!(o, Less);
    ///
    /// let (result, o) = BigExponentFloat::from(PI)
    ///     .sub_rational_prec_val_ref(&Rational::from_unsigneds(1u8, 3), 20);
    /// assert_eq!(format!("{result:?}"), "0x2.ceea");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn sub_rational_prec_val_ref(self, other: &Rational, prec: u64) -> (Self, Ordering) {
        sub_rational_helper(Cow::Owned(self), Cow::Borrowed(other), prec, Nearest)
    }

    /// Subtracts a [`Rational`] from a [`BigExponentFloat`], rounding the result to the nearest
    /// value of the specified precision. The [`BigExponentFloat`] is taken by reference and the
    /// [`Rational`] by value. An [`Ordering`] is also returned, indicating whether the rounded
    /// difference is less than, equal to, or greater than the exact difference. Although `NaN`s are
    /// not comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// $$
    /// f(x,y,p) = x-y+\varepsilon.
    /// $$
    /// - If $x-y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-y$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x-y|\rfloor-p}$.
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
    ///     .sub_rational_prec_ref_val(Rational::from_unsigneds(1u8, 3), 5);
    /// assert_eq!(format!("{result:?}"), "0x2.c");
    /// assert_eq!(o, Less);
    ///
    /// let (result, o) = (&BigExponentFloat::from(PI))
    ///     .sub_rational_prec_ref_val(Rational::from_unsigneds(1u8, 3), 20);
    /// assert_eq!(format!("{result:?}"), "0x2.ceea");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn sub_rational_prec_ref_val(&self, other: Rational, prec: u64) -> (Self, Ordering) {
        sub_rational_helper(Cow::Borrowed(self), Cow::Owned(other), prec, Nearest)
    }

    /// Subtracts a [`Rational`] from a [`BigExponentFloat`], rounding the result to the nearest
    /// value of the specified precision. The [`BigExponentFloat`] and the [`Rational`] are both
    /// taken by reference. An [`Ordering`] is also returned, indicating whether the rounded
    /// difference is less than, equal to, or greater than the exact difference. Although `NaN`s are
    /// not comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// $$
    /// f(x,y,p) = x-y+\varepsilon.
    /// $$
    /// - If $x-y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-y$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x-y|\rfloor-p}$.
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
    ///     .sub_rational_prec_ref_ref(&Rational::from_unsigneds(1u8, 3), 5);
    /// assert_eq!(format!("{result:?}"), "0x2.c");
    /// assert_eq!(o, Less);
    ///
    /// let (result, o) = (&BigExponentFloat::from(PI))
    ///     .sub_rational_prec_ref_ref(&Rational::from_unsigneds(1u8, 3), 20);
    /// assert_eq!(format!("{result:?}"), "0x2.ceea");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn sub_rational_prec_ref_ref(&self, other: &Rational, prec: u64) -> (Self, Ordering) {
        sub_rational_helper(Cow::Borrowed(self), Cow::Borrowed(other), prec, Nearest)
    }

    /// Subtracts a [`Rational`] from a [`BigExponentFloat`] in place, rounding the result to the
    /// nearest value of the specified precision. The [`Rational`] is taken by value. An
    /// [`Ordering`] is returned, indicating whether the rounded difference is less than, equal to,
    /// or greater than the exact difference. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function sets the [`BigExponentFloat`] to `NaN` it also
    /// returns `Equal`.
    ///
    /// $$
    /// x \gets x-y+\varepsilon.
    /// $$
    /// - If $x-y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-y$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x-y|\rfloor-p}$.
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
    ///     x.sub_rational_prec_assign(Rational::from_unsigneds(1u8, 3), 5),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x2.c");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.sub_rational_prec_assign(Rational::from_unsigneds(1u8, 3), 20),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x2.ceea");
    /// ```
    #[inline]
    pub fn sub_rational_prec_assign(&mut self, other: Rational, prec: u64) -> Ordering {
        let x = core::mem::replace(self, Self::ZERO);
        let (result, o) = sub_rational_helper(Cow::Owned(x), Cow::Owned(other), prec, Nearest);
        *self = result;
        o
    }

    /// Subtracts a [`Rational`] from a [`BigExponentFloat`] in place, rounding the result to the
    /// nearest value of the specified precision. The [`Rational`] is taken by reference. An
    /// [`Ordering`] is returned, indicating whether the rounded difference is less than, equal to,
    /// or greater than the exact difference. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function sets the [`BigExponentFloat`] to `NaN` it also
    /// returns `Equal`.
    ///
    /// $$
    /// x \gets x-y+\varepsilon.
    /// $$
    /// - If $x-y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-y$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x-y|\rfloor-p}$.
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
    ///     x.sub_rational_prec_assign_ref(&Rational::from_unsigneds(1u8, 3), 5),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x2.c");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.sub_rational_prec_assign_ref(&Rational::from_unsigneds(1u8, 3), 20),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x2.ceea");
    /// ```
    #[inline]
    pub fn sub_rational_prec_assign_ref(&mut self, other: &Rational, prec: u64) -> Ordering {
        let x = core::mem::replace(self, Self::ZERO);
        let (result, o) = sub_rational_helper(Cow::Owned(x), Cow::Borrowed(other), prec, Nearest);
        *self = result;
        o
    }
}
