// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::big_exponent_float::BigExponentFloat;
use crate::big_exponent_float::arithmetic::add::add_helper;
use crate::big_exponent_float::arithmetic::add_rational::add_rational_helper_with;
use alloc::borrow::Cow;
use core::cmp::Ordering::{self, *};
use malachite_base::num::basic::traits::{NaN, Zero};
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_q::Rational;

// x - y if x > y, and 0 otherwise, rounded with `prec_rm` if it is `Some`, and exact otherwise.
// `NaN` gives `NaN`; the other special cases follow from the comparison and from subtraction.
pub(crate) fn positive_difference_helper(
    x: Cow<'_, BigExponentFloat>,
    y: Cow<'_, BigExponentFloat>,
    prec_rm: Option<(u64, RoundingMode)>,
) -> (BigExponentFloat, Ordering) {
    if let Some((prec, _)) = prec_rm {
        assert_ne!(prec, 0);
    }
    match (*x).partial_cmp(&*y) {
        None => (BigExponentFloat::NAN, Equal),
        Some(Less | Equal) => (BigExponentFloat::ZERO, Equal),
        Some(Greater) => add_helper(x, y, true, prec_rm),
    }
}

// x - y if x > y, and 0 otherwise, for a `Rational` y, rounded to `prec` bits with `rm`. A `NaN` x
// gives `NaN`.
pub(crate) fn positive_difference_rational_helper(
    x: Cow<'_, BigExponentFloat>,
    y: Cow<'_, Rational>,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    assert_ne!(prec, 0);
    match (*x).partial_cmp(&*y) {
        None => (BigExponentFloat::NAN, Equal),
        Some(Less | Equal) => (BigExponentFloat::ZERO, Equal),
        Some(Greater) => add_rational_helper_with(x, y, true, prec, rm, true),
    }
}

// x - y if x > y, and 0 otherwise, for a `Rational` x, rounded to `prec` bits with `rm`. A `NaN` y
// gives `NaN`. The difference is computed as -(y - x), rounded in the opposite direction.
pub(crate) fn rational_positive_difference_helper(
    x: Cow<'_, Rational>,
    y: Cow<'_, BigExponentFloat>,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    assert_ne!(prec, 0);
    match (*y).partial_cmp(&*x) {
        None => (BigExponentFloat::NAN, Equal),
        Some(Greater | Equal) => (BigExponentFloat::ZERO, Equal),
        Some(Less) => {
            let (d, o) = add_rational_helper_with(y, x, true, prec, -rm, true);
            (d.negate(), o.reverse())
        }
    }
}

impl BigExponentFloat {
    /// Computes the positive difference of two [`BigExponentFloat`]s, $x-y$ if $x>y$ and 0
    /// otherwise, rounding the result to the specified precision and with the specified rounding
    /// mode. Both [`BigExponentFloat`]s are taken by value. An [`Ordering`] is also returned,
    /// indicating whether the rounded positive difference is less than, equal to, or greater than
    /// the exact positive difference. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,p,m) = \max(x-y,0)+\varepsilon.
    /// $$
    /// - If $\max(x-y,0)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $\max(x-y,0)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 (x-y)\rfloor-p+1}$.
    /// - If $\max(x-y,0)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 (x-y)\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p,m)=f(x,\text{NaN},p,m)=\text{NaN}$
    /// - $f(x,y,p,m)=0$ if $x\leq y$, including $f(\infty,\infty,p,m)$ and $f(-\infty,-\infty,p,m)$
    /// - $f(\infty,y,p,m)=\infty$ if $y$ is not `NaN` or $\infty$
    /// - $f(x,-\infty,p,m)=\infty$ if $x$ is not `NaN` or $-\infty$
    ///
    /// This is the positive difference, `mpfr_dim` and C's `fdim`.
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`BigExponentFloat::positive_difference_prec`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits(), prec)` if the exponents are close; see
    /// [`BigExponentFloat::sub_prec_round`] for the general case.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the positive difference cannot be
    /// represented exactly with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (z, o) = BigExponentFloat::from(PI).positive_difference_prec_round(
    ///     BigExponentFloat::from(E),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{z:?}"), "0x0.6c");
    /// assert_eq!(o, Less);
    ///
    /// let (z, o) = BigExponentFloat::from(PI).positive_difference_prec_round(
    ///     BigExponentFloat::from(E),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{z:?}"), "0x0.7");
    /// assert_eq!(o, Greater);
    ///
    /// let (z, o) = BigExponentFloat::from(PI).positive_difference_prec_round(
    ///     BigExponentFloat::from(E),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{z:?}"), "0x0.6c");
    /// assert_eq!(o, Less);
    ///
    /// let (z, o) = BigExponentFloat::from(PI).positive_difference_prec_round(
    ///     BigExponentFloat::from(E),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{z:?}"), "0x0.6c5e18");
    /// assert_eq!(o, Less);
    ///
    /// let (z, o) = BigExponentFloat::from(PI).positive_difference_prec_round(
    ///     BigExponentFloat::from(E),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{z:?}"), "0x0.6c5e2");
    /// assert_eq!(o, Greater);
    ///
    /// let (z, o) = BigExponentFloat::from(PI).positive_difference_prec_round(
    ///     BigExponentFloat::from(E),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{z:?}"), "0x0.6c5e18");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn positive_difference_prec_round(
        self,
        other: Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        positive_difference_helper(Cow::Owned(self), Cow::Owned(other), Some((prec, rm)))
    }

    /// Computes the positive difference of two [`BigExponentFloat`]s, $x-y$ if $x>y$ and 0
    /// otherwise, rounding the result to the specified precision and with the specified rounding
    /// mode. The first [`BigExponentFloat`] is taken by value and the second by reference. An
    /// [`Ordering`] is also returned, indicating whether the rounded positive difference is less
    /// than, equal to, or greater than the exact positive difference. Although `NaN`s are not
    /// comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,p,m) = \max(x-y,0)+\varepsilon.
    /// $$
    /// - If $\max(x-y,0)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $\max(x-y,0)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 (x-y)\rfloor-p+1}$.
    /// - If $\max(x-y,0)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 (x-y)\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p,m)=f(x,\text{NaN},p,m)=\text{NaN}$
    /// - $f(x,y,p,m)=0$ if $x\leq y$, including $f(\infty,\infty,p,m)$ and $f(-\infty,-\infty,p,m)$
    /// - $f(\infty,y,p,m)=\infty$ if $y$ is not `NaN` or $\infty$
    /// - $f(x,-\infty,p,m)=\infty$ if $x$ is not `NaN` or $-\infty$
    ///
    /// This is the positive difference, `mpfr_dim` and C's `fdim`.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits(), prec)` if the exponents are close; see
    /// [`BigExponentFloat::sub_prec_round`] for the general case.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the positive difference cannot be
    /// represented exactly with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (z, o) = BigExponentFloat::from(PI).positive_difference_prec_round_val_ref(
    ///     &BigExponentFloat::from(E),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{z:?}"), "0x0.6c");
    /// assert_eq!(o, Less);
    ///
    /// let (z, o) = BigExponentFloat::from(PI).positive_difference_prec_round_val_ref(
    ///     &BigExponentFloat::from(E),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{z:?}"), "0x0.7");
    /// assert_eq!(o, Greater);
    ///
    /// let (z, o) = BigExponentFloat::from(PI).positive_difference_prec_round_val_ref(
    ///     &BigExponentFloat::from(E),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{z:?}"), "0x0.6c");
    /// assert_eq!(o, Less);
    ///
    /// let (z, o) = BigExponentFloat::from(PI).positive_difference_prec_round_val_ref(
    ///     &BigExponentFloat::from(E),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{z:?}"), "0x0.6c5e18");
    /// assert_eq!(o, Less);
    ///
    /// let (z, o) = BigExponentFloat::from(PI).positive_difference_prec_round_val_ref(
    ///     &BigExponentFloat::from(E),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{z:?}"), "0x0.6c5e2");
    /// assert_eq!(o, Greater);
    ///
    /// let (z, o) = BigExponentFloat::from(PI).positive_difference_prec_round_val_ref(
    ///     &BigExponentFloat::from(E),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{z:?}"), "0x0.6c5e18");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn positive_difference_prec_round_val_ref(
        self,
        other: &Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        positive_difference_helper(Cow::Owned(self), Cow::Borrowed(other), Some((prec, rm)))
    }

    /// Computes the positive difference of two [`BigExponentFloat`]s, $x-y$ if $x>y$ and 0
    /// otherwise, rounding the result to the specified precision and with the specified rounding
    /// mode. The first [`BigExponentFloat`] is taken by reference and the second by value. An
    /// [`Ordering`] is also returned, indicating whether the rounded positive difference is less
    /// than, equal to, or greater than the exact positive difference. Although `NaN`s are not
    /// comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,p,m) = \max(x-y,0)+\varepsilon.
    /// $$
    /// - If $\max(x-y,0)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $\max(x-y,0)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 (x-y)\rfloor-p+1}$.
    /// - If $\max(x-y,0)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 (x-y)\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p,m)=f(x,\text{NaN},p,m)=\text{NaN}$
    /// - $f(x,y,p,m)=0$ if $x\leq y$, including $f(\infty,\infty,p,m)$ and $f(-\infty,-\infty,p,m)$
    /// - $f(\infty,y,p,m)=\infty$ if $y$ is not `NaN` or $\infty$
    /// - $f(x,-\infty,p,m)=\infty$ if $x$ is not `NaN` or $-\infty$
    ///
    /// This is the positive difference, `mpfr_dim` and C's `fdim`.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits(), prec)` if the exponents are close; see
    /// [`BigExponentFloat::sub_prec_round`] for the general case.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the positive difference cannot be
    /// represented exactly with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (z, o) = (&BigExponentFloat::from(PI)).positive_difference_prec_round_ref_val(
    ///     BigExponentFloat::from(E),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{z:?}"), "0x0.6c");
    /// assert_eq!(o, Less);
    ///
    /// let (z, o) = (&BigExponentFloat::from(PI)).positive_difference_prec_round_ref_val(
    ///     BigExponentFloat::from(E),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{z:?}"), "0x0.7");
    /// assert_eq!(o, Greater);
    ///
    /// let (z, o) = (&BigExponentFloat::from(PI)).positive_difference_prec_round_ref_val(
    ///     BigExponentFloat::from(E),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{z:?}"), "0x0.6c");
    /// assert_eq!(o, Less);
    ///
    /// let (z, o) = (&BigExponentFloat::from(PI)).positive_difference_prec_round_ref_val(
    ///     BigExponentFloat::from(E),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{z:?}"), "0x0.6c5e18");
    /// assert_eq!(o, Less);
    ///
    /// let (z, o) = (&BigExponentFloat::from(PI)).positive_difference_prec_round_ref_val(
    ///     BigExponentFloat::from(E),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{z:?}"), "0x0.6c5e2");
    /// assert_eq!(o, Greater);
    ///
    /// let (z, o) = (&BigExponentFloat::from(PI)).positive_difference_prec_round_ref_val(
    ///     BigExponentFloat::from(E),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{z:?}"), "0x0.6c5e18");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn positive_difference_prec_round_ref_val(
        &self,
        other: Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        positive_difference_helper(Cow::Borrowed(self), Cow::Owned(other), Some((prec, rm)))
    }

    /// Computes the positive difference of two [`BigExponentFloat`]s, $x-y$ if $x>y$ and 0
    /// otherwise, rounding the result to the specified precision and with the specified rounding
    /// mode. Both [`BigExponentFloat`]s are taken by reference. An [`Ordering`] is also returned,
    /// indicating whether the rounded positive difference is less than, equal to, or greater than
    /// the exact positive difference. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,p,m) = \max(x-y,0)+\varepsilon.
    /// $$
    /// - If $\max(x-y,0)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $\max(x-y,0)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 (x-y)\rfloor-p+1}$.
    /// - If $\max(x-y,0)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 (x-y)\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p,m)=f(x,\text{NaN},p,m)=\text{NaN}$
    /// - $f(x,y,p,m)=0$ if $x\leq y$, including $f(\infty,\infty,p,m)$ and $f(-\infty,-\infty,p,m)$
    /// - $f(\infty,y,p,m)=\infty$ if $y$ is not `NaN` or $\infty$
    /// - $f(x,-\infty,p,m)=\infty$ if $x$ is not `NaN` or $-\infty$
    ///
    /// This is the positive difference, `mpfr_dim` and C's `fdim`.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits(), prec)` if the exponents are close; see
    /// [`BigExponentFloat::sub_prec_round`] for the general case.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the positive difference cannot be
    /// represented exactly with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (z, o) = (&BigExponentFloat::from(PI)).positive_difference_prec_round_ref_ref(
    ///     &BigExponentFloat::from(E),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{z:?}"), "0x0.6c");
    /// assert_eq!(o, Less);
    ///
    /// let (z, o) = (&BigExponentFloat::from(PI)).positive_difference_prec_round_ref_ref(
    ///     &BigExponentFloat::from(E),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{z:?}"), "0x0.7");
    /// assert_eq!(o, Greater);
    ///
    /// let (z, o) = (&BigExponentFloat::from(PI)).positive_difference_prec_round_ref_ref(
    ///     &BigExponentFloat::from(E),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{z:?}"), "0x0.6c");
    /// assert_eq!(o, Less);
    ///
    /// let (z, o) = (&BigExponentFloat::from(PI)).positive_difference_prec_round_ref_ref(
    ///     &BigExponentFloat::from(E),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{z:?}"), "0x0.6c5e18");
    /// assert_eq!(o, Less);
    ///
    /// let (z, o) = (&BigExponentFloat::from(PI)).positive_difference_prec_round_ref_ref(
    ///     &BigExponentFloat::from(E),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{z:?}"), "0x0.6c5e2");
    /// assert_eq!(o, Greater);
    ///
    /// let (z, o) = (&BigExponentFloat::from(PI)).positive_difference_prec_round_ref_ref(
    ///     &BigExponentFloat::from(E),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{z:?}"), "0x0.6c5e18");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn positive_difference_prec_round_ref_ref(
        &self,
        other: &Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        positive_difference_helper(Cow::Borrowed(self), Cow::Borrowed(other), Some((prec, rm)))
    }

    /// Replaces a [`BigExponentFloat`] $x$ with the positive difference of $x$ and another
    /// [`BigExponentFloat`] $y$, $x-y$ if $x>y$ and 0 otherwise, rounding the result to the
    /// specified precision and with the specified rounding mode. The [`BigExponentFloat`] on the
    /// right-hand side is taken by value. An [`Ordering`] is returned, indicating whether the
    /// rounded positive difference is less than, equal to, or greater than the exact positive
    /// difference. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this
    /// function sets the [`BigExponentFloat`] to `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets \max(x-y,0)+\varepsilon.
    /// $$
    /// - If $\max(x-y,0)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $\max(x-y,0)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 (x-y)\rfloor-p+1}$.
    /// - If $\max(x-y,0)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 (x-y)\rfloor-p}$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits(), prec)` if the exponents are close; see
    /// [`BigExponentFloat::sub_prec_round`] for the general case.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the positive difference cannot be
    /// represented exactly with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.positive_difference_prec_round_assign(BigExponentFloat::from(E), 5, Floor),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x0.6c");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.positive_difference_prec_round_assign(BigExponentFloat::from(E), 5, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(format!("{x:?}"), "0x0.7");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.positive_difference_prec_round_assign(BigExponentFloat::from(E), 5, Nearest),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x0.6c");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.positive_difference_prec_round_assign(BigExponentFloat::from(E), 20, Floor),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x0.6c5e18");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.positive_difference_prec_round_assign(BigExponentFloat::from(E), 20, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(format!("{x:?}"), "0x0.6c5e2");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.positive_difference_prec_round_assign(BigExponentFloat::from(E), 20, Nearest),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x0.6c5e18");
    /// ```
    #[inline]
    pub fn positive_difference_prec_round_assign(
        &mut self,
        other: Self,
        prec: u64,
        rm: RoundingMode,
    ) -> Ordering {
        let x = core::mem::replace(self, Self::ZERO);
        let (result, o) =
            positive_difference_helper(Cow::Owned(x), Cow::Owned(other), Some((prec, rm)));
        *self = result;
        o
    }

    /// Replaces a [`BigExponentFloat`] $x$ with the positive difference of $x$ and another
    /// [`BigExponentFloat`] $y$, $x-y$ if $x>y$ and 0 otherwise, rounding the result to the
    /// specified precision and with the specified rounding mode. The [`BigExponentFloat`] on the
    /// right-hand side is taken by reference. An [`Ordering`] is returned, indicating whether the
    /// rounded positive difference is less than, equal to, or greater than the exact positive
    /// difference. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this
    /// function sets the [`BigExponentFloat`] to `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets \max(x-y,0)+\varepsilon.
    /// $$
    /// - If $\max(x-y,0)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $\max(x-y,0)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 (x-y)\rfloor-p+1}$.
    /// - If $\max(x-y,0)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 (x-y)\rfloor-p}$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits(), prec)` if the exponents are close; see
    /// [`BigExponentFloat::sub_prec_round`] for the general case.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the positive difference cannot be
    /// represented exactly with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.positive_difference_prec_round_assign_ref(&BigExponentFloat::from(E), 5, Floor),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x0.6c");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.positive_difference_prec_round_assign_ref(&BigExponentFloat::from(E), 5, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(format!("{x:?}"), "0x0.7");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.positive_difference_prec_round_assign_ref(&BigExponentFloat::from(E), 5, Nearest),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x0.6c");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.positive_difference_prec_round_assign_ref(&BigExponentFloat::from(E), 20, Floor),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x0.6c5e18");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.positive_difference_prec_round_assign_ref(&BigExponentFloat::from(E), 20, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(format!("{x:?}"), "0x0.6c5e2");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.positive_difference_prec_round_assign_ref(&BigExponentFloat::from(E), 20, Nearest),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x0.6c5e18");
    /// ```
    #[inline]
    pub fn positive_difference_prec_round_assign_ref(
        &mut self,
        other: &Self,
        prec: u64,
        rm: RoundingMode,
    ) -> Ordering {
        let x = core::mem::replace(self, Self::ZERO);
        let (result, o) =
            positive_difference_helper(Cow::Owned(x), Cow::Borrowed(other), Some((prec, rm)));
        *self = result;
        o
    }

    /// Computes the positive difference of two [`BigExponentFloat`]s, $x-y$ if $x>y$ and 0
    /// otherwise, rounding the result to the nearest value of the specified precision. Both
    /// [`BigExponentFloat`]s are taken by value. An [`Ordering`] is also returned, indicating
    /// whether the rounded positive difference is less than, equal to, or greater than the exact
    /// positive difference. Although `NaN`s are not comparable to any [`BigExponentFloat`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,y,p) = \max(x-y,0)+\varepsilon.
    /// $$
    /// - If $\max(x-y,0)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $\max(x-y,0)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   (x-y)\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(x,\text{NaN},p)=\text{NaN}$
    /// - $f(x,y,p)=0$ if $x\leq y$, including $f(\infty,\infty,p)$ and $f(-\infty,-\infty,p)$
    /// - $f(\infty,y,p)=\infty$ if $y$ is not `NaN` or $\infty$
    /// - $f(x,-\infty,p)=\infty$ if $x$ is not `NaN` or $-\infty$
    ///
    /// This is the positive difference, `mpfr_dim` and C's `fdim`.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`BigExponentFloat::positive_difference_prec_round`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits(), prec)` if the exponents are close; see
    /// [`BigExponentFloat::sub_prec_round`] for the general case.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (z, o) =
    ///     BigExponentFloat::from(PI).positive_difference_prec(BigExponentFloat::from(E), 5);
    /// assert_eq!(format!("{z:?}"), "0x0.6c");
    /// assert_eq!(o, Less);
    ///
    /// let (z, o) =
    ///     BigExponentFloat::from(PI).positive_difference_prec(BigExponentFloat::from(E), 20);
    /// assert_eq!(format!("{z:?}"), "0x0.6c5e18");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn positive_difference_prec(self, other: Self, prec: u64) -> (Self, Ordering) {
        positive_difference_helper(Cow::Owned(self), Cow::Owned(other), Some((prec, Nearest)))
    }

    /// Computes the positive difference of two [`BigExponentFloat`]s, $x-y$ if $x>y$ and 0
    /// otherwise, rounding the result to the nearest value of the specified precision. The first
    /// [`BigExponentFloat`] is taken by value and the second by reference. An [`Ordering`] is also
    /// returned, indicating whether the rounded positive difference is less than, equal to, or
    /// greater than the exact positive difference. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,y,p) = \max(x-y,0)+\varepsilon.
    /// $$
    /// - If $\max(x-y,0)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $\max(x-y,0)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   (x-y)\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(x,\text{NaN},p)=\text{NaN}$
    /// - $f(x,y,p)=0$ if $x\leq y$, including $f(\infty,\infty,p)$ and $f(-\infty,-\infty,p)$
    /// - $f(\infty,y,p)=\infty$ if $y$ is not `NaN` or $\infty$
    /// - $f(x,-\infty,p)=\infty$ if $x$ is not `NaN` or $-\infty$
    ///
    /// This is the positive difference, `mpfr_dim` and C's `fdim`.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits(), prec)` if the exponents are close; see
    /// [`BigExponentFloat::sub_prec_round`] for the general case.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (z, o) = BigExponentFloat::from(PI)
    ///     .positive_difference_prec_val_ref(&BigExponentFloat::from(E), 5);
    /// assert_eq!(format!("{z:?}"), "0x0.6c");
    /// assert_eq!(o, Less);
    ///
    /// let (z, o) = BigExponentFloat::from(PI)
    ///     .positive_difference_prec_val_ref(&BigExponentFloat::from(E), 20);
    /// assert_eq!(format!("{z:?}"), "0x0.6c5e18");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn positive_difference_prec_val_ref(self, other: &Self, prec: u64) -> (Self, Ordering) {
        positive_difference_helper(
            Cow::Owned(self),
            Cow::Borrowed(other),
            Some((prec, Nearest)),
        )
    }

    /// Computes the positive difference of two [`BigExponentFloat`]s, $x-y$ if $x>y$ and 0
    /// otherwise, rounding the result to the nearest value of the specified precision. The first
    /// [`BigExponentFloat`] is taken by reference and the second by value. An [`Ordering`] is also
    /// returned, indicating whether the rounded positive difference is less than, equal to, or
    /// greater than the exact positive difference. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,y,p) = \max(x-y,0)+\varepsilon.
    /// $$
    /// - If $\max(x-y,0)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $\max(x-y,0)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   (x-y)\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(x,\text{NaN},p)=\text{NaN}$
    /// - $f(x,y,p)=0$ if $x\leq y$, including $f(\infty,\infty,p)$ and $f(-\infty,-\infty,p)$
    /// - $f(\infty,y,p)=\infty$ if $y$ is not `NaN` or $\infty$
    /// - $f(x,-\infty,p)=\infty$ if $x$ is not `NaN` or $-\infty$
    ///
    /// This is the positive difference, `mpfr_dim` and C's `fdim`.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits(), prec)` if the exponents are close; see
    /// [`BigExponentFloat::sub_prec_round`] for the general case.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (z, o) = (&BigExponentFloat::from(PI))
    ///     .positive_difference_prec_ref_val(BigExponentFloat::from(E), 5);
    /// assert_eq!(format!("{z:?}"), "0x0.6c");
    /// assert_eq!(o, Less);
    ///
    /// let (z, o) = (&BigExponentFloat::from(PI))
    ///     .positive_difference_prec_ref_val(BigExponentFloat::from(E), 20);
    /// assert_eq!(format!("{z:?}"), "0x0.6c5e18");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn positive_difference_prec_ref_val(&self, other: Self, prec: u64) -> (Self, Ordering) {
        positive_difference_helper(
            Cow::Borrowed(self),
            Cow::Owned(other),
            Some((prec, Nearest)),
        )
    }

    /// Computes the positive difference of two [`BigExponentFloat`]s, $x-y$ if $x>y$ and 0
    /// otherwise, rounding the result to the nearest value of the specified precision. Both
    /// [`BigExponentFloat`]s are taken by reference. An [`Ordering`] is also returned, indicating
    /// whether the rounded positive difference is less than, equal to, or greater than the exact
    /// positive difference. Although `NaN`s are not comparable to any [`BigExponentFloat`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,y,p) = \max(x-y,0)+\varepsilon.
    /// $$
    /// - If $\max(x-y,0)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $\max(x-y,0)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   (x-y)\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(x,\text{NaN},p)=\text{NaN}$
    /// - $f(x,y,p)=0$ if $x\leq y$, including $f(\infty,\infty,p)$ and $f(-\infty,-\infty,p)$
    /// - $f(\infty,y,p)=\infty$ if $y$ is not `NaN` or $\infty$
    /// - $f(x,-\infty,p)=\infty$ if $x$ is not `NaN` or $-\infty$
    ///
    /// This is the positive difference, `mpfr_dim` and C's `fdim`.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits(), prec)` if the exponents are close; see
    /// [`BigExponentFloat::sub_prec_round`] for the general case.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (z, o) = (&BigExponentFloat::from(PI))
    ///     .positive_difference_prec_ref_ref(&BigExponentFloat::from(E), 5);
    /// assert_eq!(format!("{z:?}"), "0x0.6c");
    /// assert_eq!(o, Less);
    ///
    /// let (z, o) = (&BigExponentFloat::from(PI))
    ///     .positive_difference_prec_ref_ref(&BigExponentFloat::from(E), 20);
    /// assert_eq!(format!("{z:?}"), "0x0.6c5e18");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn positive_difference_prec_ref_ref(&self, other: &Self, prec: u64) -> (Self, Ordering) {
        positive_difference_helper(
            Cow::Borrowed(self),
            Cow::Borrowed(other),
            Some((prec, Nearest)),
        )
    }

    /// Replaces a [`BigExponentFloat`] $x$ with the positive difference of $x$ and another
    /// [`BigExponentFloat`] $y$, $x-y$ if $x>y$ and 0 otherwise, rounding the result to the nearest
    /// value of the specified precision. The [`BigExponentFloat`] on the right-hand side is taken
    /// by value. An [`Ordering`] is returned, indicating whether the rounded positive difference is
    /// less than, equal to, or greater than the exact positive difference. Although `NaN`s are not
    /// comparable to any [`BigExponentFloat`], whenever this function sets the [`BigExponentFloat`]
    /// to `NaN` it also returns `Equal`.
    ///
    /// $$
    /// x \gets \max(x-y,0)+\varepsilon.
    /// $$
    /// - If $\max(x-y,0)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $\max(x-y,0)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   (x-y)\rfloor-p}$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits(), prec)` if the exponents are close; see
    /// [`BigExponentFloat::sub_prec_round`] for the general case.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.positive_difference_prec_assign(BigExponentFloat::from(E), 5),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x0.6c");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.positive_difference_prec_assign(BigExponentFloat::from(E), 20),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x0.6c5e18");
    /// ```
    #[inline]
    pub fn positive_difference_prec_assign(&mut self, other: Self, prec: u64) -> Ordering {
        let x = core::mem::replace(self, Self::ZERO);
        let (result, o) =
            positive_difference_helper(Cow::Owned(x), Cow::Owned(other), Some((prec, Nearest)));
        *self = result;
        o
    }

    /// Replaces a [`BigExponentFloat`] $x$ with the positive difference of $x$ and another
    /// [`BigExponentFloat`] $y$, $x-y$ if $x>y$ and 0 otherwise, rounding the result to the nearest
    /// value of the specified precision. The [`BigExponentFloat`] on the right-hand side is taken
    /// by reference. An [`Ordering`] is returned, indicating whether the rounded positive
    /// difference is less than, equal to, or greater than the exact positive difference. Although
    /// `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function sets the
    /// [`BigExponentFloat`] to `NaN` it also returns `Equal`.
    ///
    /// $$
    /// x \gets \max(x-y,0)+\varepsilon.
    /// $$
    /// - If $\max(x-y,0)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $\max(x-y,0)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   (x-y)\rfloor-p}$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits(), prec)` if the exponents are close; see
    /// [`BigExponentFloat::sub_prec_round`] for the general case.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.positive_difference_prec_assign_ref(&BigExponentFloat::from(E), 5),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x0.6c");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.positive_difference_prec_assign_ref(&BigExponentFloat::from(E), 20),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x0.6c5e18");
    /// ```
    #[inline]
    pub fn positive_difference_prec_assign_ref(&mut self, other: &Self, prec: u64) -> Ordering {
        let x = core::mem::replace(self, Self::ZERO);
        let (result, o) =
            positive_difference_helper(Cow::Owned(x), Cow::Borrowed(other), Some((prec, Nearest)));
        *self = result;
        o
    }

    /// Computes the positive difference of two [`BigExponentFloat`]s, $x-y$ if $x>y$ and 0
    /// otherwise. The result is exact. Both [`BigExponentFloat`]s are taken by value.
    ///
    /// $$
    /// f(x,y) = \max(x-y,0).
    /// $$
    ///
    /// Special cases:
    /// - $f(\text{NaN},y)=f(x,\text{NaN})=\text{NaN}$
    /// - $f(x,y)=0$ if $x\leq y$, including $f(\infty,\infty)$ and $f(-\infty,-\infty)$
    /// - $f(\infty,y)=\infty$ if $y$ is not `NaN` or $\infty$
    /// - $f(x,-\infty)=\infty$ if $x$ is not `NaN` or $-\infty$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())` if the exponents are close; see
    /// [`BigExponentFloat::add_prec_round`] for the general case.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::BigExponentFloat;
    ///
    /// assert_eq!(
    ///     format!(
    ///         "{:?}",
    ///         BigExponentFloat::from(2.25).positive_difference(BigExponentFloat::from(1.5))
    ///     ),
    ///     "0x0.c"
    /// );
    /// ```
    #[inline]
    pub fn positive_difference(self, other: Self) -> Self {
        positive_difference_helper(Cow::Owned(self), Cow::Owned(other), None).0
    }

    /// Computes the positive difference of two [`BigExponentFloat`]s, $x-y$ if $x>y$ and 0
    /// otherwise. The result is exact. The first [`BigExponentFloat`] is taken by value and the
    /// second by reference.
    ///
    /// $$
    /// f(x,y) = \max(x-y,0).
    /// $$
    ///
    /// Special cases:
    /// - $f(\text{NaN},y)=f(x,\text{NaN})=\text{NaN}$
    /// - $f(x,y)=0$ if $x\leq y$, including $f(\infty,\infty)$ and $f(-\infty,-\infty)$
    /// - $f(\infty,y)=\infty$ if $y$ is not `NaN` or $\infty$
    /// - $f(x,-\infty)=\infty$ if $x$ is not `NaN` or $-\infty$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())` if the exponents are close; see
    /// [`BigExponentFloat::add_prec_round`] for the general case.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::BigExponentFloat;
    ///
    /// assert_eq!(
    ///     format!(
    ///         "{:?}",
    ///         BigExponentFloat::from(2.25)
    ///             .positive_difference_val_ref(&BigExponentFloat::from(1.5))
    ///     ),
    ///     "0x0.c"
    /// );
    /// ```
    #[inline]
    pub fn positive_difference_val_ref(self, other: &Self) -> Self {
        positive_difference_helper(Cow::Owned(self), Cow::Borrowed(other), None).0
    }

    /// Computes the positive difference of two [`BigExponentFloat`]s, $x-y$ if $x>y$ and 0
    /// otherwise. The result is exact. The first [`BigExponentFloat`] is taken by reference and the
    /// second by value.
    ///
    /// $$
    /// f(x,y) = \max(x-y,0).
    /// $$
    ///
    /// Special cases:
    /// - $f(\text{NaN},y)=f(x,\text{NaN})=\text{NaN}$
    /// - $f(x,y)=0$ if $x\leq y$, including $f(\infty,\infty)$ and $f(-\infty,-\infty)$
    /// - $f(\infty,y)=\infty$ if $y$ is not `NaN` or $\infty$
    /// - $f(x,-\infty)=\infty$ if $x$ is not `NaN` or $-\infty$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())` if the exponents are close; see
    /// [`BigExponentFloat::add_prec_round`] for the general case.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::BigExponentFloat;
    ///
    /// assert_eq!(
    ///     format!(
    ///         "{:?}",
    ///         (&BigExponentFloat::from(2.25))
    ///             .positive_difference_ref_val(BigExponentFloat::from(1.5))
    ///     ),
    ///     "0x0.c"
    /// );
    /// ```
    #[inline]
    pub fn positive_difference_ref_val(&self, other: Self) -> Self {
        positive_difference_helper(Cow::Borrowed(self), Cow::Owned(other), None).0
    }

    /// Computes the positive difference of two [`BigExponentFloat`]s, $x-y$ if $x>y$ and 0
    /// otherwise. The result is exact. Both [`BigExponentFloat`]s are taken by reference.
    ///
    /// $$
    /// f(x,y) = \max(x-y,0).
    /// $$
    ///
    /// Special cases:
    /// - $f(\text{NaN},y)=f(x,\text{NaN})=\text{NaN}$
    /// - $f(x,y)=0$ if $x\leq y$, including $f(\infty,\infty)$ and $f(-\infty,-\infty)$
    /// - $f(\infty,y)=\infty$ if $y$ is not `NaN` or $\infty$
    /// - $f(x,-\infty)=\infty$ if $x$ is not `NaN` or $-\infty$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())` if the exponents are close; see
    /// [`BigExponentFloat::add_prec_round`] for the general case.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::BigExponentFloat;
    ///
    /// assert_eq!(
    ///     format!(
    ///         "{:?}",
    ///         (&BigExponentFloat::from(2.25))
    ///             .positive_difference_ref_ref(&BigExponentFloat::from(1.5))
    ///     ),
    ///     "0x0.c"
    /// );
    /// ```
    #[inline]
    pub fn positive_difference_ref_ref(&self, other: &Self) -> Self {
        positive_difference_helper(Cow::Borrowed(self), Cow::Borrowed(other), None).0
    }

    /// Computes the positive difference of two [`BigExponentFloat`]s, $x-y$ if $x>y$ and 0
    /// otherwise. The result is exact. The [`BigExponentFloat`] on the right-hand side is taken by
    /// value.
    ///
    /// $$
    /// x \gets \max(x-y,0).
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())` if the exponents are close; see
    /// [`BigExponentFloat::add_prec_round`] for the general case.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::BigExponentFloat;
    ///
    /// let mut x = BigExponentFloat::from(2.25);
    /// x.positive_difference_assign(BigExponentFloat::from(1.5));
    /// assert_eq!(format!("{x:?}"), "0x0.c");
    /// ```
    #[inline]
    pub fn positive_difference_assign(&mut self, other: Self) {
        let x = core::mem::replace(self, Self::ZERO);
        *self = positive_difference_helper(Cow::Owned(x), Cow::Owned(other), None).0;
    }

    /// Computes the positive difference of two [`BigExponentFloat`]s, $x-y$ if $x>y$ and 0
    /// otherwise. The result is exact. The [`BigExponentFloat`] on the right-hand side is taken by
    /// reference.
    ///
    /// $$
    /// x \gets \max(x-y,0).
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())` if the exponents are close; see
    /// [`BigExponentFloat::add_prec_round`] for the general case.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::BigExponentFloat;
    ///
    /// let mut x = BigExponentFloat::from(2.25);
    /// x.positive_difference_assign_ref(&BigExponentFloat::from(1.5));
    /// assert_eq!(format!("{x:?}"), "0x0.c");
    /// ```
    #[inline]
    pub fn positive_difference_assign_ref(&mut self, other: &Self) {
        let x = core::mem::replace(self, Self::ZERO);
        *self = positive_difference_helper(Cow::Owned(x), Cow::Borrowed(other), None).0;
    }
}

impl BigExponentFloat {
    /// Computes the positive difference of a [`BigExponentFloat`] and a [`Rational`], $x-y$ if
    /// $x>y$ and 0 otherwise, rounding the result to the specified precision and with the specified
    /// rounding mode. The [`BigExponentFloat`] and the [`Rational`] are both taken by value. An
    /// [`Ordering`] is also returned, indicating whether the rounded positive difference is less
    /// than, equal to, or greater than the exact positive difference. Although `NaN`s are not
    /// comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,p,m) = \max(x-y,0)+\varepsilon.
    /// $$
    /// - If $\max(x-y,0)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $\max(x-y,0)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 (x-y)\rfloor-p+1}$.
    /// - If $\max(x-y,0)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 (x-y)\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p,m)=\text{NaN}$
    /// - $f(\infty,y,p,m)=\infty$
    /// - $f(-\infty,y,p,m)=0$
    /// - $f(x,y,p,m)=0$ if $x\leq y$
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`BigExponentFloat::positive_difference_rational_prec`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(self.significant_bits(),
    /// other.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the positive difference cannot be
    /// represented exactly with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (r, o) = BigExponentFloat::from(PI).positive_difference_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{r:?}"), "0x2.c");
    /// assert_eq!(o, Less);
    ///
    /// let (r, o) = BigExponentFloat::from(PI).positive_difference_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{r:?}"), "0x2.e");
    /// assert_eq!(o, Greater);
    ///
    /// let (r, o) = BigExponentFloat::from(PI).positive_difference_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{r:?}"), "0x2.c");
    /// assert_eq!(o, Less);
    ///
    /// let (r, o) = BigExponentFloat::from(PI).positive_difference_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{r:?}"), "0x2.ceea");
    /// assert_eq!(o, Less);
    ///
    /// let (r, o) = BigExponentFloat::from(PI).positive_difference_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{r:?}"), "0x2.ceea4");
    /// assert_eq!(o, Greater);
    ///
    /// let (r, o) = BigExponentFloat::from(PI).positive_difference_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{r:?}"), "0x2.ceea");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn positive_difference_rational_prec_round(
        self,
        other: Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        positive_difference_rational_helper(Cow::Owned(self), Cow::Owned(other), prec, rm)
    }

    /// Computes the positive difference of a [`BigExponentFloat`] and a [`Rational`], $x-y$ if
    /// $x>y$ and 0 otherwise, rounding the result to the specified precision and with the specified
    /// rounding mode. The [`BigExponentFloat`] is taken by value and the [`Rational`] by reference.
    /// An [`Ordering`] is also returned, indicating whether the rounded positive difference is less
    /// than, equal to, or greater than the exact positive difference. Although `NaN`s are not
    /// comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,p,m) = \max(x-y,0)+\varepsilon.
    /// $$
    /// - If $\max(x-y,0)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $\max(x-y,0)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 (x-y)\rfloor-p+1}$.
    /// - If $\max(x-y,0)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 (x-y)\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p,m)=\text{NaN}$
    /// - $f(\infty,y,p,m)=\infty$
    /// - $f(-\infty,y,p,m)=0$
    /// - $f(x,y,p,m)=0$ if $x\leq y$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(self.significant_bits(),
    /// other.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the positive difference cannot be
    /// represented exactly with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (r, o) = BigExponentFloat::from(PI).positive_difference_rational_prec_round_val_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{r:?}"), "0x2.c");
    /// assert_eq!(o, Less);
    ///
    /// let (r, o) = BigExponentFloat::from(PI).positive_difference_rational_prec_round_val_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{r:?}"), "0x2.e");
    /// assert_eq!(o, Greater);
    ///
    /// let (r, o) = BigExponentFloat::from(PI).positive_difference_rational_prec_round_val_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{r:?}"), "0x2.c");
    /// assert_eq!(o, Less);
    ///
    /// let (r, o) = BigExponentFloat::from(PI).positive_difference_rational_prec_round_val_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{r:?}"), "0x2.ceea");
    /// assert_eq!(o, Less);
    ///
    /// let (r, o) = BigExponentFloat::from(PI).positive_difference_rational_prec_round_val_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{r:?}"), "0x2.ceea4");
    /// assert_eq!(o, Greater);
    ///
    /// let (r, o) = BigExponentFloat::from(PI).positive_difference_rational_prec_round_val_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{r:?}"), "0x2.ceea");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn positive_difference_rational_prec_round_val_ref(
        self,
        other: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        positive_difference_rational_helper(Cow::Owned(self), Cow::Borrowed(other), prec, rm)
    }

    /// Computes the positive difference of a [`BigExponentFloat`] and a [`Rational`], $x-y$ if
    /// $x>y$ and 0 otherwise, rounding the result to the specified precision and with the specified
    /// rounding mode. The [`BigExponentFloat`] is taken by reference and the [`Rational`] by value.
    /// An [`Ordering`] is also returned, indicating whether the rounded positive difference is less
    /// than, equal to, or greater than the exact positive difference. Although `NaN`s are not
    /// comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,p,m) = \max(x-y,0)+\varepsilon.
    /// $$
    /// - If $\max(x-y,0)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $\max(x-y,0)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 (x-y)\rfloor-p+1}$.
    /// - If $\max(x-y,0)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 (x-y)\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p,m)=\text{NaN}$
    /// - $f(\infty,y,p,m)=\infty$
    /// - $f(-\infty,y,p,m)=0$
    /// - $f(x,y,p,m)=0$ if $x\leq y$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(self.significant_bits(),
    /// other.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the positive difference cannot be
    /// represented exactly with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (r, o) = (&BigExponentFloat::from(PI)).positive_difference_rational_prec_round_ref_val(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{r:?}"), "0x2.c");
    /// assert_eq!(o, Less);
    ///
    /// let (r, o) = (&BigExponentFloat::from(PI)).positive_difference_rational_prec_round_ref_val(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{r:?}"), "0x2.e");
    /// assert_eq!(o, Greater);
    ///
    /// let (r, o) = (&BigExponentFloat::from(PI)).positive_difference_rational_prec_round_ref_val(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{r:?}"), "0x2.c");
    /// assert_eq!(o, Less);
    ///
    /// let (r, o) = (&BigExponentFloat::from(PI)).positive_difference_rational_prec_round_ref_val(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{r:?}"), "0x2.ceea");
    /// assert_eq!(o, Less);
    ///
    /// let (r, o) = (&BigExponentFloat::from(PI)).positive_difference_rational_prec_round_ref_val(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{r:?}"), "0x2.ceea4");
    /// assert_eq!(o, Greater);
    ///
    /// let (r, o) = (&BigExponentFloat::from(PI)).positive_difference_rational_prec_round_ref_val(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{r:?}"), "0x2.ceea");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn positive_difference_rational_prec_round_ref_val(
        &self,
        other: Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        positive_difference_rational_helper(Cow::Borrowed(self), Cow::Owned(other), prec, rm)
    }

    /// Computes the positive difference of a [`BigExponentFloat`] and a [`Rational`], $x-y$ if
    /// $x>y$ and 0 otherwise, rounding the result to the specified precision and with the specified
    /// rounding mode. The [`BigExponentFloat`] and the [`Rational`] are both taken by reference. An
    /// [`Ordering`] is also returned, indicating whether the rounded positive difference is less
    /// than, equal to, or greater than the exact positive difference. Although `NaN`s are not
    /// comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,p,m) = \max(x-y,0)+\varepsilon.
    /// $$
    /// - If $\max(x-y,0)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $\max(x-y,0)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 (x-y)\rfloor-p+1}$.
    /// - If $\max(x-y,0)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 (x-y)\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p,m)=\text{NaN}$
    /// - $f(\infty,y,p,m)=\infty$
    /// - $f(-\infty,y,p,m)=0$
    /// - $f(x,y,p,m)=0$ if $x\leq y$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(self.significant_bits(),
    /// other.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the positive difference cannot be
    /// represented exactly with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (r, o) = (&BigExponentFloat::from(PI)).positive_difference_rational_prec_round_ref_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{r:?}"), "0x2.c");
    /// assert_eq!(o, Less);
    ///
    /// let (r, o) = (&BigExponentFloat::from(PI)).positive_difference_rational_prec_round_ref_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{r:?}"), "0x2.e");
    /// assert_eq!(o, Greater);
    ///
    /// let (r, o) = (&BigExponentFloat::from(PI)).positive_difference_rational_prec_round_ref_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{r:?}"), "0x2.c");
    /// assert_eq!(o, Less);
    ///
    /// let (r, o) = (&BigExponentFloat::from(PI)).positive_difference_rational_prec_round_ref_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{r:?}"), "0x2.ceea");
    /// assert_eq!(o, Less);
    ///
    /// let (r, o) = (&BigExponentFloat::from(PI)).positive_difference_rational_prec_round_ref_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{r:?}"), "0x2.ceea4");
    /// assert_eq!(o, Greater);
    ///
    /// let (r, o) = (&BigExponentFloat::from(PI)).positive_difference_rational_prec_round_ref_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{r:?}"), "0x2.ceea");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn positive_difference_rational_prec_round_ref_ref(
        &self,
        other: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        positive_difference_rational_helper(Cow::Borrowed(self), Cow::Borrowed(other), prec, rm)
    }

    /// Replaces a [`BigExponentFloat`] $x$ with the positive difference of $x$ and a [`Rational`]
    /// $y$, $x-y$ if $x>y$ and 0 otherwise, rounding the result to the specified precision and with
    /// the specified rounding mode. The [`Rational`] on the right-hand side is taken by value. An
    /// [`Ordering`] is returned, indicating whether the rounded positive difference is less than,
    /// equal to, or greater than the exact positive difference. Although `NaN`s are not comparable
    /// to any [`BigExponentFloat`], whenever this function sets the [`BigExponentFloat`] to `NaN`
    /// it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets \max(x-y,0)+\varepsilon.
    /// $$
    /// - If $\max(x-y,0)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $\max(x-y,0)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 (x-y)\rfloor-p+1}$.
    /// - If $\max(x-y,0)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 (x-y)\rfloor-p}$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(self.significant_bits(),
    /// other.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the positive difference cannot be
    /// represented exactly with `prec` bits.
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
    ///     x.positive_difference_rational_prec_round_assign(
    ///         Rational::from_unsigneds(1u8, 3),
    ///         5,
    ///         Floor
    ///     ),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x2.c");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.positive_difference_rational_prec_round_assign(
    ///         Rational::from_unsigneds(1u8, 3),
    ///         5,
    ///         Ceiling
    ///     ),
    ///     Greater
    /// );
    /// assert_eq!(format!("{x:?}"), "0x2.e");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.positive_difference_rational_prec_round_assign(
    ///         Rational::from_unsigneds(1u8, 3),
    ///         5,
    ///         Nearest
    ///     ),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x2.c");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.positive_difference_rational_prec_round_assign(
    ///         Rational::from_unsigneds(1u8, 3),
    ///         20,
    ///         Floor
    ///     ),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x2.ceea");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.positive_difference_rational_prec_round_assign(
    ///         Rational::from_unsigneds(1u8, 3),
    ///         20,
    ///         Ceiling
    ///     ),
    ///     Greater
    /// );
    /// assert_eq!(format!("{x:?}"), "0x2.ceea4");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.positive_difference_rational_prec_round_assign(
    ///         Rational::from_unsigneds(1u8, 3),
    ///         20,
    ///         Nearest
    ///     ),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x2.ceea");
    /// ```
    #[inline]
    pub fn positive_difference_rational_prec_round_assign(
        &mut self,
        other: Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> Ordering {
        let x = core::mem::replace(self, Self::ZERO);
        let (result, o) =
            positive_difference_rational_helper(Cow::Owned(x), Cow::Owned(other), prec, rm);
        *self = result;
        o
    }

    /// Replaces a [`BigExponentFloat`] $x$ with the positive difference of $x$ and a [`Rational`]
    /// $y$, $x-y$ if $x>y$ and 0 otherwise, rounding the result to the specified precision and with
    /// the specified rounding mode. The [`Rational`] on the right-hand side is taken by reference.
    /// An [`Ordering`] is returned, indicating whether the rounded positive difference is less
    /// than, equal to, or greater than the exact positive difference. Although `NaN`s are not
    /// comparable to any [`BigExponentFloat`], whenever this function sets the [`BigExponentFloat`]
    /// to `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets \max(x-y,0)+\varepsilon.
    /// $$
    /// - If $\max(x-y,0)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $\max(x-y,0)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 (x-y)\rfloor-p+1}$.
    /// - If $\max(x-y,0)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 (x-y)\rfloor-p}$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(self.significant_bits(),
    /// other.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the positive difference cannot be
    /// represented exactly with `prec` bits.
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
    ///     x.positive_difference_rational_prec_round_assign_ref(
    ///         &Rational::from_unsigneds(1u8, 3),
    ///         5,
    ///         Floor
    ///     ),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x2.c");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.positive_difference_rational_prec_round_assign_ref(
    ///         &Rational::from_unsigneds(1u8, 3),
    ///         5,
    ///         Ceiling
    ///     ),
    ///     Greater
    /// );
    /// assert_eq!(format!("{x:?}"), "0x2.e");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.positive_difference_rational_prec_round_assign_ref(
    ///         &Rational::from_unsigneds(1u8, 3),
    ///         5,
    ///         Nearest
    ///     ),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x2.c");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.positive_difference_rational_prec_round_assign_ref(
    ///         &Rational::from_unsigneds(1u8, 3),
    ///         20,
    ///         Floor
    ///     ),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x2.ceea");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.positive_difference_rational_prec_round_assign_ref(
    ///         &Rational::from_unsigneds(1u8, 3),
    ///         20,
    ///         Ceiling
    ///     ),
    ///     Greater
    /// );
    /// assert_eq!(format!("{x:?}"), "0x2.ceea4");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.positive_difference_rational_prec_round_assign_ref(
    ///         &Rational::from_unsigneds(1u8, 3),
    ///         20,
    ///         Nearest
    ///     ),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x2.ceea");
    /// ```
    #[inline]
    pub fn positive_difference_rational_prec_round_assign_ref(
        &mut self,
        other: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> Ordering {
        let x = core::mem::replace(self, Self::ZERO);
        let (result, o) =
            positive_difference_rational_helper(Cow::Owned(x), Cow::Borrowed(other), prec, rm);
        *self = result;
        o
    }

    /// Computes the positive difference of a [`BigExponentFloat`] and a [`Rational`], $x-y$ if
    /// $x>y$ and 0 otherwise, rounding the result to the nearest value of the specified precision.
    /// The [`BigExponentFloat`] and the [`Rational`] are both taken by value. An [`Ordering`] is
    /// also returned, indicating whether the rounded positive difference is less than, equal to, or
    /// greater than the exact positive difference. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,y,p) = \max(x-y,0)+\varepsilon.
    /// $$
    /// - If $\max(x-y,0)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $\max(x-y,0)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   (x-y)\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=\text{NaN}$
    /// - $f(\infty,y,p)=\infty$
    /// - $f(-\infty,y,p)=0$
    /// - $f(x,y,p)=0$ if $x\leq y$
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`BigExponentFloat::positive_difference_rational_prec_round`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(self.significant_bits(),
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
    /// let (r, o) = BigExponentFloat::from(PI)
    ///     .positive_difference_rational_prec(Rational::from_unsigneds(1u8, 3), 5);
    /// assert_eq!(format!("{r:?}"), "0x2.c");
    /// assert_eq!(o, Less);
    ///
    /// let (r, o) = BigExponentFloat::from(PI)
    ///     .positive_difference_rational_prec(Rational::from_unsigneds(1u8, 3), 20);
    /// assert_eq!(format!("{r:?}"), "0x2.ceea");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn positive_difference_rational_prec(self, other: Rational, prec: u64) -> (Self, Ordering) {
        positive_difference_rational_helper(Cow::Owned(self), Cow::Owned(other), prec, Nearest)
    }

    /// Computes the positive difference of a [`BigExponentFloat`] and a [`Rational`], $x-y$ if
    /// $x>y$ and 0 otherwise, rounding the result to the nearest value of the specified precision.
    /// The [`BigExponentFloat`] is taken by value and the [`Rational`] by reference. An
    /// [`Ordering`] is also returned, indicating whether the rounded positive difference is less
    /// than, equal to, or greater than the exact positive difference. Although `NaN`s are not
    /// comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// $$
    /// f(x,y,p) = \max(x-y,0)+\varepsilon.
    /// $$
    /// - If $\max(x-y,0)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $\max(x-y,0)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   (x-y)\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=\text{NaN}$
    /// - $f(\infty,y,p)=\infty$
    /// - $f(-\infty,y,p)=0$
    /// - $f(x,y,p)=0$ if $x\leq y$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(self.significant_bits(),
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
    /// let (r, o) = BigExponentFloat::from(PI)
    ///     .positive_difference_rational_prec_val_ref(&Rational::from_unsigneds(1u8, 3), 5);
    /// assert_eq!(format!("{r:?}"), "0x2.c");
    /// assert_eq!(o, Less);
    ///
    /// let (r, o) = BigExponentFloat::from(PI)
    ///     .positive_difference_rational_prec_val_ref(&Rational::from_unsigneds(1u8, 3), 20);
    /// assert_eq!(format!("{r:?}"), "0x2.ceea");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn positive_difference_rational_prec_val_ref(
        self,
        other: &Rational,
        prec: u64,
    ) -> (Self, Ordering) {
        positive_difference_rational_helper(Cow::Owned(self), Cow::Borrowed(other), prec, Nearest)
    }

    /// Computes the positive difference of a [`BigExponentFloat`] and a [`Rational`], $x-y$ if
    /// $x>y$ and 0 otherwise, rounding the result to the nearest value of the specified precision.
    /// The [`BigExponentFloat`] is taken by reference and the [`Rational`] by value. An
    /// [`Ordering`] is also returned, indicating whether the rounded positive difference is less
    /// than, equal to, or greater than the exact positive difference. Although `NaN`s are not
    /// comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// $$
    /// f(x,y,p) = \max(x-y,0)+\varepsilon.
    /// $$
    /// - If $\max(x-y,0)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $\max(x-y,0)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   (x-y)\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=\text{NaN}$
    /// - $f(\infty,y,p)=\infty$
    /// - $f(-\infty,y,p)=0$
    /// - $f(x,y,p)=0$ if $x\leq y$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(self.significant_bits(),
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
    /// let (r, o) = (&BigExponentFloat::from(PI))
    ///     .positive_difference_rational_prec_ref_val(Rational::from_unsigneds(1u8, 3), 5);
    /// assert_eq!(format!("{r:?}"), "0x2.c");
    /// assert_eq!(o, Less);
    ///
    /// let (r, o) = (&BigExponentFloat::from(PI))
    ///     .positive_difference_rational_prec_ref_val(Rational::from_unsigneds(1u8, 3), 20);
    /// assert_eq!(format!("{r:?}"), "0x2.ceea");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn positive_difference_rational_prec_ref_val(
        &self,
        other: Rational,
        prec: u64,
    ) -> (Self, Ordering) {
        positive_difference_rational_helper(Cow::Borrowed(self), Cow::Owned(other), prec, Nearest)
    }

    /// Computes the positive difference of a [`BigExponentFloat`] and a [`Rational`], $x-y$ if
    /// $x>y$ and 0 otherwise, rounding the result to the nearest value of the specified precision.
    /// The [`BigExponentFloat`] and the [`Rational`] are both taken by reference. An [`Ordering`]
    /// is also returned, indicating whether the rounded positive difference is less than, equal to,
    /// or greater than the exact positive difference. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,y,p) = \max(x-y,0)+\varepsilon.
    /// $$
    /// - If $\max(x-y,0)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $\max(x-y,0)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   (x-y)\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=\text{NaN}$
    /// - $f(\infty,y,p)=\infty$
    /// - $f(-\infty,y,p)=0$
    /// - $f(x,y,p)=0$ if $x\leq y$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(self.significant_bits(),
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
    /// let (r, o) = (&BigExponentFloat::from(PI))
    ///     .positive_difference_rational_prec_ref_ref(&Rational::from_unsigneds(1u8, 3), 5);
    /// assert_eq!(format!("{r:?}"), "0x2.c");
    /// assert_eq!(o, Less);
    ///
    /// let (r, o) = (&BigExponentFloat::from(PI))
    ///     .positive_difference_rational_prec_ref_ref(&Rational::from_unsigneds(1u8, 3), 20);
    /// assert_eq!(format!("{r:?}"), "0x2.ceea");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn positive_difference_rational_prec_ref_ref(
        &self,
        other: &Rational,
        prec: u64,
    ) -> (Self, Ordering) {
        positive_difference_rational_helper(
            Cow::Borrowed(self),
            Cow::Borrowed(other),
            prec,
            Nearest,
        )
    }

    /// Replaces a [`BigExponentFloat`] $x$ with the positive difference of $x$ and a [`Rational`]
    /// $y$, $x-y$ if $x>y$ and 0 otherwise, rounding the result to the nearest value of the
    /// specified precision. The [`Rational`] on the right-hand side is taken by value. An
    /// [`Ordering`] is returned, indicating whether the rounded positive difference is less than,
    /// equal to, or greater than the exact positive difference. Although `NaN`s are not comparable
    /// to any [`BigExponentFloat`], whenever this function sets the [`BigExponentFloat`] to `NaN`
    /// it also returns `Equal`.
    ///
    /// $$
    /// x \gets \max(x-y,0)+\varepsilon.
    /// $$
    /// - If $\max(x-y,0)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $\max(x-y,0)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   (x-y)\rfloor-p}$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(self.significant_bits(),
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
    ///     x.positive_difference_rational_prec_assign(Rational::from_unsigneds(1u8, 3), 5),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x2.c");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.positive_difference_rational_prec_assign(Rational::from_unsigneds(1u8, 3), 20),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x2.ceea");
    /// ```
    #[inline]
    pub fn positive_difference_rational_prec_assign(
        &mut self,
        other: Rational,
        prec: u64,
    ) -> Ordering {
        let x = core::mem::replace(self, Self::ZERO);
        let (result, o) =
            positive_difference_rational_helper(Cow::Owned(x), Cow::Owned(other), prec, Nearest);
        *self = result;
        o
    }

    /// Replaces a [`BigExponentFloat`] $x$ with the positive difference of $x$ and a [`Rational`]
    /// $y$, $x-y$ if $x>y$ and 0 otherwise, rounding the result to the nearest value of the
    /// specified precision. The [`Rational`] on the right-hand side is taken by reference. An
    /// [`Ordering`] is returned, indicating whether the rounded positive difference is less than,
    /// equal to, or greater than the exact positive difference. Although `NaN`s are not comparable
    /// to any [`BigExponentFloat`], whenever this function sets the [`BigExponentFloat`] to `NaN`
    /// it also returns `Equal`.
    ///
    /// $$
    /// x \gets \max(x-y,0)+\varepsilon.
    /// $$
    /// - If $\max(x-y,0)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $\max(x-y,0)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   (x-y)\rfloor-p}$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(self.significant_bits(),
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
    ///     x.positive_difference_rational_prec_assign_ref(&Rational::from_unsigneds(1u8, 3), 5),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x2.c");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.positive_difference_rational_prec_assign_ref(&Rational::from_unsigneds(1u8, 3), 20),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x2.ceea");
    /// ```
    #[inline]
    pub fn positive_difference_rational_prec_assign_ref(
        &mut self,
        other: &Rational,
        prec: u64,
    ) -> Ordering {
        let x = core::mem::replace(self, Self::ZERO);
        let (result, o) =
            positive_difference_rational_helper(Cow::Owned(x), Cow::Borrowed(other), prec, Nearest);
        *self = result;
        o
    }

    /// Computes the positive difference of a [`Rational`] and a [`BigExponentFloat`], $x-y$ if
    /// $x>y$ and 0 otherwise, rounding the result to the specified precision and with the specified
    /// rounding mode. The [`Rational`] and the [`BigExponentFloat`] are both taken by value. An
    /// [`Ordering`] is also returned, indicating whether the rounded positive difference is less
    /// than, equal to, or greater than the exact positive difference. Although `NaN`s are not
    /// comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,p,m) = \max(x-y,0)+\varepsilon.
    /// $$
    /// - If $\max(x-y,0)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $\max(x-y,0)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 (x-y)\rfloor-p+1}$.
    /// - If $\max(x-y,0)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 (x-y)\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,\text{NaN},p,m)=\text{NaN}$
    /// - $f(x,\infty,p,m)=0$
    /// - $f(x,-\infty,p,m)=\infty$
    /// - $f(x,y,p,m)=0$ if $x\leq y$
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`BigExponentFloat::rational_positive_difference_big_exponent_float_prec`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(x.significant_bits(),
    /// y.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the positive difference cannot be
    /// represented exactly with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::E;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (r, o) = BigExponentFloat::rational_positive_difference_big_exponent_float_prec_round(
    ///     Rational::from_unsigneds(22u8, 7),
    ///     BigExponentFloat::from(E),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{r:?}"), "0x0.6c");
    /// assert_eq!(o, Less);
    ///
    /// let (r, o) = BigExponentFloat::rational_positive_difference_big_exponent_float_prec_round(
    ///     Rational::from_unsigneds(22u8, 7),
    ///     BigExponentFloat::from(E),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{r:?}"), "0x0.7");
    /// assert_eq!(o, Greater);
    ///
    /// let (r, o) = BigExponentFloat::rational_positive_difference_big_exponent_float_prec_round(
    ///     Rational::from_unsigneds(22u8, 7),
    ///     BigExponentFloat::from(E),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{r:?}"), "0x0.6c");
    /// assert_eq!(o, Less);
    ///
    /// let (r, o) = BigExponentFloat::rational_positive_difference_big_exponent_float_prec_round(
    ///     Rational::from_unsigneds(22u8, 7),
    ///     BigExponentFloat::from(E),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{r:?}"), "0x0.6cb0f");
    /// assert_eq!(o, Less);
    ///
    /// let (r, o) = BigExponentFloat::rational_positive_difference_big_exponent_float_prec_round(
    ///     Rational::from_unsigneds(22u8, 7),
    ///     BigExponentFloat::from(E),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{r:?}"), "0x0.6cb0f8");
    /// assert_eq!(o, Greater);
    ///
    /// let (r, o) = BigExponentFloat::rational_positive_difference_big_exponent_float_prec_round(
    ///     Rational::from_unsigneds(22u8, 7),
    ///     BigExponentFloat::from(E),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{r:?}"), "0x0.6cb0f8");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn rational_positive_difference_big_exponent_float_prec_round(
        x: Rational,
        y: Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        rational_positive_difference_helper(Cow::Owned(x), Cow::Owned(y), prec, rm)
    }

    /// Computes the positive difference of a [`Rational`] and a [`BigExponentFloat`], $x-y$ if
    /// $x>y$ and 0 otherwise, rounding the result to the specified precision and with the specified
    /// rounding mode. The [`Rational`] is taken by value and the [`BigExponentFloat`] by reference.
    /// An [`Ordering`] is also returned, indicating whether the rounded positive difference is less
    /// than, equal to, or greater than the exact positive difference. Although `NaN`s are not
    /// comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,p,m) = \max(x-y,0)+\varepsilon.
    /// $$
    /// - If $\max(x-y,0)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $\max(x-y,0)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 (x-y)\rfloor-p+1}$.
    /// - If $\max(x-y,0)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 (x-y)\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,\text{NaN},p,m)=\text{NaN}$
    /// - $f(x,\infty,p,m)=0$
    /// - $f(x,-\infty,p,m)=\infty$
    /// - $f(x,y,p,m)=0$ if $x\leq y$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(x.significant_bits(),
    /// y.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the positive difference cannot be
    /// represented exactly with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::E;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (r, o) =
    ///     BigExponentFloat::rational_positive_difference_big_exponent_float_prec_round_val_ref(
    ///         Rational::from_unsigneds(22u8, 7),
    ///         &BigExponentFloat::from(E),
    ///         5,
    ///         Floor,
    ///     );
    /// assert_eq!(format!("{r:?}"), "0x0.6c");
    /// assert_eq!(o, Less);
    ///
    /// let (r, o) =
    ///     BigExponentFloat::rational_positive_difference_big_exponent_float_prec_round_val_ref(
    ///         Rational::from_unsigneds(22u8, 7),
    ///         &BigExponentFloat::from(E),
    ///         5,
    ///         Ceiling,
    ///     );
    /// assert_eq!(format!("{r:?}"), "0x0.7");
    /// assert_eq!(o, Greater);
    ///
    /// let (r, o) =
    ///     BigExponentFloat::rational_positive_difference_big_exponent_float_prec_round_val_ref(
    ///         Rational::from_unsigneds(22u8, 7),
    ///         &BigExponentFloat::from(E),
    ///         5,
    ///         Nearest,
    ///     );
    /// assert_eq!(format!("{r:?}"), "0x0.6c");
    /// assert_eq!(o, Less);
    ///
    /// let (r, o) =
    ///     BigExponentFloat::rational_positive_difference_big_exponent_float_prec_round_val_ref(
    ///         Rational::from_unsigneds(22u8, 7),
    ///         &BigExponentFloat::from(E),
    ///         20,
    ///         Floor,
    ///     );
    /// assert_eq!(format!("{r:?}"), "0x0.6cb0f");
    /// assert_eq!(o, Less);
    ///
    /// let (r, o) =
    ///     BigExponentFloat::rational_positive_difference_big_exponent_float_prec_round_val_ref(
    ///         Rational::from_unsigneds(22u8, 7),
    ///         &BigExponentFloat::from(E),
    ///         20,
    ///         Ceiling,
    ///     );
    /// assert_eq!(format!("{r:?}"), "0x0.6cb0f8");
    /// assert_eq!(o, Greater);
    ///
    /// let (r, o) =
    ///     BigExponentFloat::rational_positive_difference_big_exponent_float_prec_round_val_ref(
    ///         Rational::from_unsigneds(22u8, 7),
    ///         &BigExponentFloat::from(E),
    ///         20,
    ///         Nearest,
    ///     );
    /// assert_eq!(format!("{r:?}"), "0x0.6cb0f8");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn rational_positive_difference_big_exponent_float_prec_round_val_ref(
        x: Rational,
        y: &Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        rational_positive_difference_helper(Cow::Owned(x), Cow::Borrowed(y), prec, rm)
    }

    /// Computes the positive difference of a [`Rational`] and a [`BigExponentFloat`], $x-y$ if
    /// $x>y$ and 0 otherwise, rounding the result to the specified precision and with the specified
    /// rounding mode. The [`Rational`] is taken by reference and the [`BigExponentFloat`] by value.
    /// An [`Ordering`] is also returned, indicating whether the rounded positive difference is less
    /// than, equal to, or greater than the exact positive difference. Although `NaN`s are not
    /// comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,p,m) = \max(x-y,0)+\varepsilon.
    /// $$
    /// - If $\max(x-y,0)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $\max(x-y,0)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 (x-y)\rfloor-p+1}$.
    /// - If $\max(x-y,0)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 (x-y)\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,\text{NaN},p,m)=\text{NaN}$
    /// - $f(x,\infty,p,m)=0$
    /// - $f(x,-\infty,p,m)=\infty$
    /// - $f(x,y,p,m)=0$ if $x\leq y$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(x.significant_bits(),
    /// y.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the positive difference cannot be
    /// represented exactly with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::E;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (r, o) =
    ///     BigExponentFloat::rational_positive_difference_big_exponent_float_prec_round_ref_val(
    ///         &Rational::from_unsigneds(22u8, 7),
    ///         BigExponentFloat::from(E),
    ///         5,
    ///         Floor,
    ///     );
    /// assert_eq!(format!("{r:?}"), "0x0.6c");
    /// assert_eq!(o, Less);
    ///
    /// let (r, o) =
    ///     BigExponentFloat::rational_positive_difference_big_exponent_float_prec_round_ref_val(
    ///         &Rational::from_unsigneds(22u8, 7),
    ///         BigExponentFloat::from(E),
    ///         5,
    ///         Ceiling,
    ///     );
    /// assert_eq!(format!("{r:?}"), "0x0.7");
    /// assert_eq!(o, Greater);
    ///
    /// let (r, o) =
    ///     BigExponentFloat::rational_positive_difference_big_exponent_float_prec_round_ref_val(
    ///         &Rational::from_unsigneds(22u8, 7),
    ///         BigExponentFloat::from(E),
    ///         5,
    ///         Nearest,
    ///     );
    /// assert_eq!(format!("{r:?}"), "0x0.6c");
    /// assert_eq!(o, Less);
    ///
    /// let (r, o) =
    ///     BigExponentFloat::rational_positive_difference_big_exponent_float_prec_round_ref_val(
    ///         &Rational::from_unsigneds(22u8, 7),
    ///         BigExponentFloat::from(E),
    ///         20,
    ///         Floor,
    ///     );
    /// assert_eq!(format!("{r:?}"), "0x0.6cb0f");
    /// assert_eq!(o, Less);
    ///
    /// let (r, o) =
    ///     BigExponentFloat::rational_positive_difference_big_exponent_float_prec_round_ref_val(
    ///         &Rational::from_unsigneds(22u8, 7),
    ///         BigExponentFloat::from(E),
    ///         20,
    ///         Ceiling,
    ///     );
    /// assert_eq!(format!("{r:?}"), "0x0.6cb0f8");
    /// assert_eq!(o, Greater);
    ///
    /// let (r, o) =
    ///     BigExponentFloat::rational_positive_difference_big_exponent_float_prec_round_ref_val(
    ///         &Rational::from_unsigneds(22u8, 7),
    ///         BigExponentFloat::from(E),
    ///         20,
    ///         Nearest,
    ///     );
    /// assert_eq!(format!("{r:?}"), "0x0.6cb0f8");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn rational_positive_difference_big_exponent_float_prec_round_ref_val(
        x: &Rational,
        y: Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        rational_positive_difference_helper(Cow::Borrowed(x), Cow::Owned(y), prec, rm)
    }

    /// Computes the positive difference of a [`Rational`] and a [`BigExponentFloat`], $x-y$ if
    /// $x>y$ and 0 otherwise, rounding the result to the specified precision and with the specified
    /// rounding mode. The [`Rational`] and the [`BigExponentFloat`] are both taken by reference. An
    /// [`Ordering`] is also returned, indicating whether the rounded positive difference is less
    /// than, equal to, or greater than the exact positive difference. Although `NaN`s are not
    /// comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,p,m) = \max(x-y,0)+\varepsilon.
    /// $$
    /// - If $\max(x-y,0)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $\max(x-y,0)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 (x-y)\rfloor-p+1}$.
    /// - If $\max(x-y,0)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 (x-y)\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,\text{NaN},p,m)=\text{NaN}$
    /// - $f(x,\infty,p,m)=0$
    /// - $f(x,-\infty,p,m)=\infty$
    /// - $f(x,y,p,m)=0$ if $x\leq y$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(x.significant_bits(),
    /// y.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the positive difference cannot be
    /// represented exactly with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::E;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (r, o) =
    ///     BigExponentFloat::rational_positive_difference_big_exponent_float_prec_round_ref_ref(
    ///         &Rational::from_unsigneds(22u8, 7),
    ///         &BigExponentFloat::from(E),
    ///         5,
    ///         Floor,
    ///     );
    /// assert_eq!(format!("{r:?}"), "0x0.6c");
    /// assert_eq!(o, Less);
    ///
    /// let (r, o) =
    ///     BigExponentFloat::rational_positive_difference_big_exponent_float_prec_round_ref_ref(
    ///         &Rational::from_unsigneds(22u8, 7),
    ///         &BigExponentFloat::from(E),
    ///         5,
    ///         Ceiling,
    ///     );
    /// assert_eq!(format!("{r:?}"), "0x0.7");
    /// assert_eq!(o, Greater);
    ///
    /// let (r, o) =
    ///     BigExponentFloat::rational_positive_difference_big_exponent_float_prec_round_ref_ref(
    ///         &Rational::from_unsigneds(22u8, 7),
    ///         &BigExponentFloat::from(E),
    ///         5,
    ///         Nearest,
    ///     );
    /// assert_eq!(format!("{r:?}"), "0x0.6c");
    /// assert_eq!(o, Less);
    ///
    /// let (r, o) =
    ///     BigExponentFloat::rational_positive_difference_big_exponent_float_prec_round_ref_ref(
    ///         &Rational::from_unsigneds(22u8, 7),
    ///         &BigExponentFloat::from(E),
    ///         20,
    ///         Floor,
    ///     );
    /// assert_eq!(format!("{r:?}"), "0x0.6cb0f");
    /// assert_eq!(o, Less);
    ///
    /// let (r, o) =
    ///     BigExponentFloat::rational_positive_difference_big_exponent_float_prec_round_ref_ref(
    ///         &Rational::from_unsigneds(22u8, 7),
    ///         &BigExponentFloat::from(E),
    ///         20,
    ///         Ceiling,
    ///     );
    /// assert_eq!(format!("{r:?}"), "0x0.6cb0f8");
    /// assert_eq!(o, Greater);
    ///
    /// let (r, o) =
    ///     BigExponentFloat::rational_positive_difference_big_exponent_float_prec_round_ref_ref(
    ///         &Rational::from_unsigneds(22u8, 7),
    ///         &BigExponentFloat::from(E),
    ///         20,
    ///         Nearest,
    ///     );
    /// assert_eq!(format!("{r:?}"), "0x0.6cb0f8");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn rational_positive_difference_big_exponent_float_prec_round_ref_ref(
        x: &Rational,
        y: &Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        rational_positive_difference_helper(Cow::Borrowed(x), Cow::Borrowed(y), prec, rm)
    }

    /// Computes the positive difference of a [`Rational`] and a [`BigExponentFloat`], $x-y$ if
    /// $x>y$ and 0 otherwise, rounding the result to the nearest value of the specified precision.
    /// The [`Rational`] and the [`BigExponentFloat`] are both taken by value. An [`Ordering`] is
    /// also returned, indicating whether the rounded positive difference is less than, equal to, or
    /// greater than the exact positive difference. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,y,p) = \max(x-y,0)+\varepsilon.
    /// $$
    /// - If $\max(x-y,0)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $\max(x-y,0)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   (x-y)\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,\text{NaN},p)=\text{NaN}$
    /// - $f(x,\infty,p)=0$
    /// - $f(x,-\infty,p)=\infty$
    /// - $f(x,y,p)=0$ if $x\leq y$
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`BigExponentFloat::rational_positive_difference_big_exponent_float_prec_round`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(x.significant_bits(),
    /// y.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::E;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (r, o) = BigExponentFloat::rational_positive_difference_big_exponent_float_prec(
    ///     Rational::from_unsigneds(22u8, 7),
    ///     BigExponentFloat::from(E),
    ///     5,
    /// );
    /// assert_eq!(format!("{r:?}"), "0x0.6c");
    /// assert_eq!(o, Less);
    ///
    /// let (r, o) = BigExponentFloat::rational_positive_difference_big_exponent_float_prec(
    ///     Rational::from_unsigneds(22u8, 7),
    ///     BigExponentFloat::from(E),
    ///     20,
    /// );
    /// assert_eq!(format!("{r:?}"), "0x0.6cb0f8");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn rational_positive_difference_big_exponent_float_prec(
        x: Rational,
        y: Self,
        prec: u64,
    ) -> (Self, Ordering) {
        rational_positive_difference_helper(Cow::Owned(x), Cow::Owned(y), prec, Nearest)
    }

    /// Computes the positive difference of a [`Rational`] and a [`BigExponentFloat`], $x-y$ if
    /// $x>y$ and 0 otherwise, rounding the result to the nearest value of the specified precision.
    /// The [`Rational`] is taken by value and the [`BigExponentFloat`] by reference. An
    /// [`Ordering`] is also returned, indicating whether the rounded positive difference is less
    /// than, equal to, or greater than the exact positive difference. Although `NaN`s are not
    /// comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// $$
    /// f(x,y,p) = \max(x-y,0)+\varepsilon.
    /// $$
    /// - If $\max(x-y,0)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $\max(x-y,0)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   (x-y)\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,\text{NaN},p)=\text{NaN}$
    /// - $f(x,\infty,p)=0$
    /// - $f(x,-\infty,p)=\infty$
    /// - $f(x,y,p)=0$ if $x\leq y$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(x.significant_bits(),
    /// y.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::E;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (r, o) = BigExponentFloat::rational_positive_difference_big_exponent_float_prec_val_ref(
    ///     Rational::from_unsigneds(22u8, 7),
    ///     &BigExponentFloat::from(E),
    ///     5,
    /// );
    /// assert_eq!(format!("{r:?}"), "0x0.6c");
    /// assert_eq!(o, Less);
    ///
    /// let (r, o) = BigExponentFloat::rational_positive_difference_big_exponent_float_prec_val_ref(
    ///     Rational::from_unsigneds(22u8, 7),
    ///     &BigExponentFloat::from(E),
    ///     20,
    /// );
    /// assert_eq!(format!("{r:?}"), "0x0.6cb0f8");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn rational_positive_difference_big_exponent_float_prec_val_ref(
        x: Rational,
        y: &Self,
        prec: u64,
    ) -> (Self, Ordering) {
        rational_positive_difference_helper(Cow::Owned(x), Cow::Borrowed(y), prec, Nearest)
    }

    /// Computes the positive difference of a [`Rational`] and a [`BigExponentFloat`], $x-y$ if
    /// $x>y$ and 0 otherwise, rounding the result to the nearest value of the specified precision.
    /// The [`Rational`] is taken by reference and the [`BigExponentFloat`] by value. An
    /// [`Ordering`] is also returned, indicating whether the rounded positive difference is less
    /// than, equal to, or greater than the exact positive difference. Although `NaN`s are not
    /// comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// $$
    /// f(x,y,p) = \max(x-y,0)+\varepsilon.
    /// $$
    /// - If $\max(x-y,0)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $\max(x-y,0)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   (x-y)\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,\text{NaN},p)=\text{NaN}$
    /// - $f(x,\infty,p)=0$
    /// - $f(x,-\infty,p)=\infty$
    /// - $f(x,y,p)=0$ if $x\leq y$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(x.significant_bits(),
    /// y.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::E;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (r, o) = BigExponentFloat::rational_positive_difference_big_exponent_float_prec_ref_val(
    ///     &Rational::from_unsigneds(22u8, 7),
    ///     BigExponentFloat::from(E),
    ///     5,
    /// );
    /// assert_eq!(format!("{r:?}"), "0x0.6c");
    /// assert_eq!(o, Less);
    ///
    /// let (r, o) = BigExponentFloat::rational_positive_difference_big_exponent_float_prec_ref_val(
    ///     &Rational::from_unsigneds(22u8, 7),
    ///     BigExponentFloat::from(E),
    ///     20,
    /// );
    /// assert_eq!(format!("{r:?}"), "0x0.6cb0f8");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn rational_positive_difference_big_exponent_float_prec_ref_val(
        x: &Rational,
        y: Self,
        prec: u64,
    ) -> (Self, Ordering) {
        rational_positive_difference_helper(Cow::Borrowed(x), Cow::Owned(y), prec, Nearest)
    }

    /// Computes the positive difference of a [`Rational`] and a [`BigExponentFloat`], $x-y$ if
    /// $x>y$ and 0 otherwise, rounding the result to the nearest value of the specified precision.
    /// The [`Rational`] and the [`BigExponentFloat`] are both taken by reference. An [`Ordering`]
    /// is also returned, indicating whether the rounded positive difference is less than, equal to,
    /// or greater than the exact positive difference. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,y,p) = \max(x-y,0)+\varepsilon.
    /// $$
    /// - If $\max(x-y,0)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $\max(x-y,0)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   (x-y)\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,\text{NaN},p)=\text{NaN}$
    /// - $f(x,\infty,p)=0$
    /// - $f(x,-\infty,p)=\infty$
    /// - $f(x,y,p)=0$ if $x\leq y$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(x.significant_bits(),
    /// y.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::E;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (r, o) = BigExponentFloat::rational_positive_difference_big_exponent_float_prec_ref_ref(
    ///     &Rational::from_unsigneds(22u8, 7),
    ///     &BigExponentFloat::from(E),
    ///     5,
    /// );
    /// assert_eq!(format!("{r:?}"), "0x0.6c");
    /// assert_eq!(o, Less);
    ///
    /// let (r, o) = BigExponentFloat::rational_positive_difference_big_exponent_float_prec_ref_ref(
    ///     &Rational::from_unsigneds(22u8, 7),
    ///     &BigExponentFloat::from(E),
    ///     20,
    /// );
    /// assert_eq!(format!("{r:?}"), "0x0.6cb0f8");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn rational_positive_difference_big_exponent_float_prec_ref_ref(
        x: &Rational,
        y: &Self,
        prec: u64,
    ) -> (Self, Ordering) {
        rational_positive_difference_helper(Cow::Borrowed(x), Cow::Borrowed(y), prec, Nearest)
    }
}
