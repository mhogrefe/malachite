// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::big_exponent_float::BigExponentFloat;
use crate::big_exponent_float::arithmetic::add::add_helper;
use alloc::borrow::Cow;
use core::cmp::Ordering;
use malachite_base::num::arithmetic::traits::{Average, AverageAssign};
use malachite_base::num::basic::traits::Zero;
use malachite_base::rounding_modes::RoundingMode::{self, *};

// (x + y) / 2, rounded with `prec_rm` if it is `Some`, and exact otherwise. Since halving is exact,
// this is the rounded sum, halved. The special cases are those of addition.
pub(crate) fn average_helper(
    x: Cow<'_, BigExponentFloat>,
    y: Cow<'_, BigExponentFloat>,
    prec_rm: Option<(u64, RoundingMode)>,
) -> (BigExponentFloat, Ordering) {
    let (mut z, o) = add_helper(x, y, false, prec_rm);
    z >>= 1u32;
    (z, o)
}

impl BigExponentFloat {
    /// Computes the average (arithmetic mean) of two [`BigExponentFloat`]s, rounding the result to
    /// the specified precision and with the specified rounding mode. Both [`BigExponentFloat`]s are
    /// taken by value. An [`Ordering`] is also returned, indicating whether the rounded average is
    /// less than, equal to, or greater than the exact average. Although `NaN`s are not comparable
    /// to any [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,p,m) = \frac{x+y}{2}+\varepsilon.
    /// $$
    /// - If $\frac{x+y}{2}$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $\frac{x+y}{2}$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x+y|/2\rfloor-p+1}$.
    /// - If $\frac{x+y}{2}$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x+y|/2\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p,m)=f(x,\text{NaN},p,m)=f(\infty,-\infty,p,m)=f(-\infty,\infty,p,m)=\text{NaN}$
    ///   $f(\text{NaN},y,p,m)=f(x,\text{NaN},p,m)=f(\infty,-\infty,p,m)=f(-\infty,\infty,p,m)=\text{NaN}$
    /// - $f(\infty,y,p,m)=f(y,\infty,p,m)=\infty$ if $y$ is not `NaN` or $-\infty$
    /// - $f(-\infty,y,p,m)=f(y,-\infty,p,m)=-\infty$ if $y$ is not `NaN` or $\infty$
    ///
    /// The average is the rounded sum, halved; halving is exact, since there is no exponent range
    /// to leave.
    ///
    /// If you know you'll be using `Nearest`, consider using [`BigExponentFloat::average_prec`]
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits(), prec)` if the exponents are close; see
    /// [`BigExponentFloat::add_prec_round`] for the general case.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the average cannot be represented
    /// exactly with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (z, o) =
    ///     BigExponentFloat::from(PI).average_prec_round(BigExponentFloat::from(E), 5, Floor);
    /// assert_eq!(format!("{z:?}"), "0x2.e");
    /// assert_eq!(o, Less);
    ///
    /// let (z, o) =
    ///     BigExponentFloat::from(PI).average_prec_round(BigExponentFloat::from(E), 5, Ceiling);
    /// assert_eq!(format!("{z:?}"), "0x3.0");
    /// assert_eq!(o, Greater);
    ///
    /// let (z, o) =
    ///     BigExponentFloat::from(PI).average_prec_round(BigExponentFloat::from(E), 5, Nearest);
    /// assert_eq!(format!("{z:?}"), "0x2.e");
    /// assert_eq!(o, Less);
    ///
    /// let (z, o) =
    ///     BigExponentFloat::from(PI).average_prec_round(BigExponentFloat::from(E), 20, Floor);
    /// assert_eq!(format!("{z:?}"), "0x2.ee104");
    /// assert_eq!(o, Less);
    ///
    /// let (z, o) =
    ///     BigExponentFloat::from(PI).average_prec_round(BigExponentFloat::from(E), 20, Ceiling);
    /// assert_eq!(format!("{z:?}"), "0x2.ee108");
    /// assert_eq!(o, Greater);
    ///
    /// let (z, o) =
    ///     BigExponentFloat::from(PI).average_prec_round(BigExponentFloat::from(E), 20, Nearest);
    /// assert_eq!(format!("{z:?}"), "0x2.ee104");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn average_prec_round(self, other: Self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        average_helper(Cow::Owned(self), Cow::Owned(other), Some((prec, rm)))
    }

    /// Computes the average (arithmetic mean) of two [`BigExponentFloat`]s, rounding the result to
    /// the specified precision and with the specified rounding mode. The first [`BigExponentFloat`]
    /// is taken by value and the second by reference. An [`Ordering`] is also returned, indicating
    /// whether the rounded average is less than, equal to, or greater than the exact average.
    /// Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,p,m) = \frac{x+y}{2}+\varepsilon.
    /// $$
    /// - If $\frac{x+y}{2}$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $\frac{x+y}{2}$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x+y|/2\rfloor-p+1}$.
    /// - If $\frac{x+y}{2}$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x+y|/2\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p,m)=f(x,\text{NaN},p,m)=f(\infty,-\infty,p,m)=f(-\infty,\infty,p,m)=\text{NaN}$
    ///   $f(\text{NaN},y,p,m)=f(x,\text{NaN},p,m)=f(\infty,-\infty,p,m)=f(-\infty,\infty,p,m)=\text{NaN}$
    /// - $f(\infty,y,p,m)=f(y,\infty,p,m)=\infty$ if $y$ is not `NaN` or $-\infty$
    /// - $f(-\infty,y,p,m)=f(y,-\infty,p,m)=-\infty$ if $y$ is not `NaN` or $\infty$
    ///
    /// The average is the rounded sum, halved; halving is exact, since there is no exponent range
    /// to leave.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits(), prec)` if the exponents are close; see
    /// [`BigExponentFloat::add_prec_round`] for the general case.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the average cannot be represented
    /// exactly with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (z, o) = BigExponentFloat::from(PI).average_prec_round_val_ref(
    ///     &BigExponentFloat::from(E),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{z:?}"), "0x2.e");
    /// assert_eq!(o, Less);
    ///
    /// let (z, o) = BigExponentFloat::from(PI).average_prec_round_val_ref(
    ///     &BigExponentFloat::from(E),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{z:?}"), "0x3.0");
    /// assert_eq!(o, Greater);
    ///
    /// let (z, o) = BigExponentFloat::from(PI).average_prec_round_val_ref(
    ///     &BigExponentFloat::from(E),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{z:?}"), "0x2.e");
    /// assert_eq!(o, Less);
    ///
    /// let (z, o) = BigExponentFloat::from(PI).average_prec_round_val_ref(
    ///     &BigExponentFloat::from(E),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{z:?}"), "0x2.ee104");
    /// assert_eq!(o, Less);
    ///
    /// let (z, o) = BigExponentFloat::from(PI).average_prec_round_val_ref(
    ///     &BigExponentFloat::from(E),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{z:?}"), "0x2.ee108");
    /// assert_eq!(o, Greater);
    ///
    /// let (z, o) = BigExponentFloat::from(PI).average_prec_round_val_ref(
    ///     &BigExponentFloat::from(E),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{z:?}"), "0x2.ee104");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn average_prec_round_val_ref(
        self,
        other: &Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        average_helper(Cow::Owned(self), Cow::Borrowed(other), Some((prec, rm)))
    }

    /// Computes the average (arithmetic mean) of two [`BigExponentFloat`]s, rounding the result to
    /// the specified precision and with the specified rounding mode. The first [`BigExponentFloat`]
    /// is taken by reference and the second by value. An [`Ordering`] is also returned, indicating
    /// whether the rounded average is less than, equal to, or greater than the exact average.
    /// Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,p,m) = \frac{x+y}{2}+\varepsilon.
    /// $$
    /// - If $\frac{x+y}{2}$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $\frac{x+y}{2}$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x+y|/2\rfloor-p+1}$.
    /// - If $\frac{x+y}{2}$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x+y|/2\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p,m)=f(x,\text{NaN},p,m)=f(\infty,-\infty,p,m)=f(-\infty,\infty,p,m)=\text{NaN}$
    ///   $f(\text{NaN},y,p,m)=f(x,\text{NaN},p,m)=f(\infty,-\infty,p,m)=f(-\infty,\infty,p,m)=\text{NaN}$
    /// - $f(\infty,y,p,m)=f(y,\infty,p,m)=\infty$ if $y$ is not `NaN` or $-\infty$
    /// - $f(-\infty,y,p,m)=f(y,-\infty,p,m)=-\infty$ if $y$ is not `NaN` or $\infty$
    ///
    /// The average is the rounded sum, halved; halving is exact, since there is no exponent range
    /// to leave.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits(), prec)` if the exponents are close; see
    /// [`BigExponentFloat::add_prec_round`] for the general case.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the average cannot be represented
    /// exactly with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (z, o) = (&BigExponentFloat::from(PI)).average_prec_round_ref_val(
    ///     BigExponentFloat::from(E),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{z:?}"), "0x2.e");
    /// assert_eq!(o, Less);
    ///
    /// let (z, o) = (&BigExponentFloat::from(PI)).average_prec_round_ref_val(
    ///     BigExponentFloat::from(E),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{z:?}"), "0x3.0");
    /// assert_eq!(o, Greater);
    ///
    /// let (z, o) = (&BigExponentFloat::from(PI)).average_prec_round_ref_val(
    ///     BigExponentFloat::from(E),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{z:?}"), "0x2.e");
    /// assert_eq!(o, Less);
    ///
    /// let (z, o) = (&BigExponentFloat::from(PI)).average_prec_round_ref_val(
    ///     BigExponentFloat::from(E),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{z:?}"), "0x2.ee104");
    /// assert_eq!(o, Less);
    ///
    /// let (z, o) = (&BigExponentFloat::from(PI)).average_prec_round_ref_val(
    ///     BigExponentFloat::from(E),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{z:?}"), "0x2.ee108");
    /// assert_eq!(o, Greater);
    ///
    /// let (z, o) = (&BigExponentFloat::from(PI)).average_prec_round_ref_val(
    ///     BigExponentFloat::from(E),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{z:?}"), "0x2.ee104");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn average_prec_round_ref_val(
        &self,
        other: Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        average_helper(Cow::Borrowed(self), Cow::Owned(other), Some((prec, rm)))
    }

    /// Computes the average (arithmetic mean) of two [`BigExponentFloat`]s, rounding the result to
    /// the specified precision and with the specified rounding mode. Both [`BigExponentFloat`]s are
    /// taken by reference. An [`Ordering`] is also returned, indicating whether the rounded average
    /// is less than, equal to, or greater than the exact average. Although `NaN`s are not
    /// comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,p,m) = \frac{x+y}{2}+\varepsilon.
    /// $$
    /// - If $\frac{x+y}{2}$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $\frac{x+y}{2}$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x+y|/2\rfloor-p+1}$.
    /// - If $\frac{x+y}{2}$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x+y|/2\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p,m)=f(x,\text{NaN},p,m)=f(\infty,-\infty,p,m)=f(-\infty,\infty,p,m)=\text{NaN}$
    ///   $f(\text{NaN},y,p,m)=f(x,\text{NaN},p,m)=f(\infty,-\infty,p,m)=f(-\infty,\infty,p,m)=\text{NaN}$
    /// - $f(\infty,y,p,m)=f(y,\infty,p,m)=\infty$ if $y$ is not `NaN` or $-\infty$
    /// - $f(-\infty,y,p,m)=f(y,-\infty,p,m)=-\infty$ if $y$ is not `NaN` or $\infty$
    ///
    /// The average is the rounded sum, halved; halving is exact, since there is no exponent range
    /// to leave.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits(), prec)` if the exponents are close; see
    /// [`BigExponentFloat::add_prec_round`] for the general case.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the average cannot be represented
    /// exactly with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (z, o) = (&BigExponentFloat::from(PI)).average_prec_round_ref_ref(
    ///     &BigExponentFloat::from(E),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{z:?}"), "0x2.e");
    /// assert_eq!(o, Less);
    ///
    /// let (z, o) = (&BigExponentFloat::from(PI)).average_prec_round_ref_ref(
    ///     &BigExponentFloat::from(E),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{z:?}"), "0x3.0");
    /// assert_eq!(o, Greater);
    ///
    /// let (z, o) = (&BigExponentFloat::from(PI)).average_prec_round_ref_ref(
    ///     &BigExponentFloat::from(E),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{z:?}"), "0x2.e");
    /// assert_eq!(o, Less);
    ///
    /// let (z, o) = (&BigExponentFloat::from(PI)).average_prec_round_ref_ref(
    ///     &BigExponentFloat::from(E),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{z:?}"), "0x2.ee104");
    /// assert_eq!(o, Less);
    ///
    /// let (z, o) = (&BigExponentFloat::from(PI)).average_prec_round_ref_ref(
    ///     &BigExponentFloat::from(E),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{z:?}"), "0x2.ee108");
    /// assert_eq!(o, Greater);
    ///
    /// let (z, o) = (&BigExponentFloat::from(PI)).average_prec_round_ref_ref(
    ///     &BigExponentFloat::from(E),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{z:?}"), "0x2.ee104");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn average_prec_round_ref_ref(
        &self,
        other: &Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        average_helper(Cow::Borrowed(self), Cow::Borrowed(other), Some((prec, rm)))
    }

    /// Replaces a [`BigExponentFloat`] $x$ with the average of $x$ and another [`BigExponentFloat`]
    /// $y$, rounding the result to the specified precision and with the specified rounding mode.
    /// The [`BigExponentFloat`] on the right-hand side is taken by value. An [`Ordering`] is
    /// returned, indicating whether the rounded average is less than, equal to, or greater than the
    /// exact average. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this
    /// function sets the [`BigExponentFloat`] to `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets \frac{x+y}{2}+\varepsilon.
    /// $$
    /// - If $\frac{x+y}{2}$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $\frac{x+y}{2}$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x+y|/2\rfloor-p+1}$.
    /// - If $\frac{x+y}{2}$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x+y|/2\rfloor-p}$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits(), prec)` if the exponents are close; see
    /// [`BigExponentFloat::add_prec_round`] for the general case.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the average cannot be represented
    /// exactly with `prec` bits.
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
    ///     x.average_prec_round_assign(BigExponentFloat::from(E), 5, Floor),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x2.e");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.average_prec_round_assign(BigExponentFloat::from(E), 5, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(format!("{x:?}"), "0x3.0");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.average_prec_round_assign(BigExponentFloat::from(E), 5, Nearest),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x2.e");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.average_prec_round_assign(BigExponentFloat::from(E), 20, Floor),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x2.ee104");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.average_prec_round_assign(BigExponentFloat::from(E), 20, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(format!("{x:?}"), "0x2.ee108");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.average_prec_round_assign(BigExponentFloat::from(E), 20, Nearest),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x2.ee104");
    /// ```
    #[inline]
    pub fn average_prec_round_assign(
        &mut self,
        other: Self,
        prec: u64,
        rm: RoundingMode,
    ) -> Ordering {
        let x = core::mem::replace(self, Self::ZERO);
        let (result, o) = average_helper(Cow::Owned(x), Cow::Owned(other), Some((prec, rm)));
        *self = result;
        o
    }

    /// Replaces a [`BigExponentFloat`] $x$ with the average of $x$ and another [`BigExponentFloat`]
    /// $y$, rounding the result to the specified precision and with the specified rounding mode.
    /// The [`BigExponentFloat`] on the right-hand side is taken by reference. An [`Ordering`] is
    /// returned, indicating whether the rounded average is less than, equal to, or greater than the
    /// exact average. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this
    /// function sets the [`BigExponentFloat`] to `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets \frac{x+y}{2}+\varepsilon.
    /// $$
    /// - If $\frac{x+y}{2}$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $\frac{x+y}{2}$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x+y|/2\rfloor-p+1}$.
    /// - If $\frac{x+y}{2}$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x+y|/2\rfloor-p}$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits(), prec)` if the exponents are close; see
    /// [`BigExponentFloat::add_prec_round`] for the general case.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the average cannot be represented
    /// exactly with `prec` bits.
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
    ///     x.average_prec_round_assign_ref(&BigExponentFloat::from(E), 5, Floor),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x2.e");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.average_prec_round_assign_ref(&BigExponentFloat::from(E), 5, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(format!("{x:?}"), "0x3.0");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.average_prec_round_assign_ref(&BigExponentFloat::from(E), 5, Nearest),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x2.e");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.average_prec_round_assign_ref(&BigExponentFloat::from(E), 20, Floor),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x2.ee104");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.average_prec_round_assign_ref(&BigExponentFloat::from(E), 20, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(format!("{x:?}"), "0x2.ee108");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.average_prec_round_assign_ref(&BigExponentFloat::from(E), 20, Nearest),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x2.ee104");
    /// ```
    #[inline]
    pub fn average_prec_round_assign_ref(
        &mut self,
        other: &Self,
        prec: u64,
        rm: RoundingMode,
    ) -> Ordering {
        let x = core::mem::replace(self, Self::ZERO);
        let (result, o) = average_helper(Cow::Owned(x), Cow::Borrowed(other), Some((prec, rm)));
        *self = result;
        o
    }

    /// Computes the average (arithmetic mean) of two [`BigExponentFloat`]s, rounding the result to
    /// the nearest value of the specified precision. Both [`BigExponentFloat`]s are taken by value.
    /// An [`Ordering`] is also returned, indicating whether the rounded average is less than, equal
    /// to, or greater than the exact average. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,y,p) = \frac{x+y}{2}+\varepsilon.
    /// $$
    /// - If $\frac{x+y}{2}$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $\frac{x+y}{2}$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x+y|/2\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(x,\text{NaN},p)=f(\infty,-\infty,p)=f(-\infty,\infty,p)=\text{NaN}$
    /// - $f(\infty,y,p)=f(y,\infty,p)=\infty$ if $y$ is not `NaN` or $-\infty$
    /// - $f(-\infty,y,p)=f(y,-\infty,p)=-\infty$ if $y$ is not `NaN` or $\infty$
    ///
    /// The average is the rounded sum, halved; halving is exact, since there is no exponent range
    /// to leave.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`BigExponentFloat::average_prec_round`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits(), prec)` if the exponents are close; see
    /// [`BigExponentFloat::add_prec_round`] for the general case.
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
    /// let (z, o) = BigExponentFloat::from(PI).average_prec(BigExponentFloat::from(E), 5);
    /// assert_eq!(format!("{z:?}"), "0x2.e");
    /// assert_eq!(o, Less);
    ///
    /// let (z, o) = BigExponentFloat::from(PI).average_prec(BigExponentFloat::from(E), 20);
    /// assert_eq!(format!("{z:?}"), "0x2.ee104");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn average_prec(self, other: Self, prec: u64) -> (Self, Ordering) {
        average_helper(Cow::Owned(self), Cow::Owned(other), Some((prec, Nearest)))
    }

    /// Computes the average (arithmetic mean) of two [`BigExponentFloat`]s, rounding the result to
    /// the nearest value of the specified precision. The first [`BigExponentFloat`] is taken by
    /// value and the second by reference. An [`Ordering`] is also returned, indicating whether the
    /// rounded average is less than, equal to, or greater than the exact average. Although `NaN`s
    /// are not comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it
    /// also returns `Equal`.
    ///
    /// $$
    /// f(x,y,p) = \frac{x+y}{2}+\varepsilon.
    /// $$
    /// - If $\frac{x+y}{2}$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $\frac{x+y}{2}$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x+y|/2\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(x,\text{NaN},p)=f(\infty,-\infty,p)=f(-\infty,\infty,p)=\text{NaN}$
    /// - $f(\infty,y,p)=f(y,\infty,p)=\infty$ if $y$ is not `NaN` or $-\infty$
    /// - $f(-\infty,y,p)=f(y,-\infty,p)=-\infty$ if $y$ is not `NaN` or $\infty$
    ///
    /// The average is the rounded sum, halved; halving is exact, since there is no exponent range
    /// to leave.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits(), prec)` if the exponents are close; see
    /// [`BigExponentFloat::add_prec_round`] for the general case.
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
    /// let (z, o) = BigExponentFloat::from(PI).average_prec_val_ref(&BigExponentFloat::from(E), 5);
    /// assert_eq!(format!("{z:?}"), "0x2.e");
    /// assert_eq!(o, Less);
    ///
    /// let (z, o) =
    ///     BigExponentFloat::from(PI).average_prec_val_ref(&BigExponentFloat::from(E), 20);
    /// assert_eq!(format!("{z:?}"), "0x2.ee104");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn average_prec_val_ref(self, other: &Self, prec: u64) -> (Self, Ordering) {
        average_helper(
            Cow::Owned(self),
            Cow::Borrowed(other),
            Some((prec, Nearest)),
        )
    }

    /// Computes the average (arithmetic mean) of two [`BigExponentFloat`]s, rounding the result to
    /// the nearest value of the specified precision. The first [`BigExponentFloat`] is taken by
    /// reference and the second by value. An [`Ordering`] is also returned, indicating whether the
    /// rounded average is less than, equal to, or greater than the exact average. Although `NaN`s
    /// are not comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it
    /// also returns `Equal`.
    ///
    /// $$
    /// f(x,y,p) = \frac{x+y}{2}+\varepsilon.
    /// $$
    /// - If $\frac{x+y}{2}$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $\frac{x+y}{2}$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x+y|/2\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(x,\text{NaN},p)=f(\infty,-\infty,p)=f(-\infty,\infty,p)=\text{NaN}$
    /// - $f(\infty,y,p)=f(y,\infty,p)=\infty$ if $y$ is not `NaN` or $-\infty$
    /// - $f(-\infty,y,p)=f(y,-\infty,p)=-\infty$ if $y$ is not `NaN` or $\infty$
    ///
    /// The average is the rounded sum, halved; halving is exact, since there is no exponent range
    /// to leave.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits(), prec)` if the exponents are close; see
    /// [`BigExponentFloat::add_prec_round`] for the general case.
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
    ///     (&BigExponentFloat::from(PI)).average_prec_ref_val(BigExponentFloat::from(E), 5);
    /// assert_eq!(format!("{z:?}"), "0x2.e");
    /// assert_eq!(o, Less);
    ///
    /// let (z, o) =
    ///     (&BigExponentFloat::from(PI)).average_prec_ref_val(BigExponentFloat::from(E), 20);
    /// assert_eq!(format!("{z:?}"), "0x2.ee104");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn average_prec_ref_val(&self, other: Self, prec: u64) -> (Self, Ordering) {
        average_helper(
            Cow::Borrowed(self),
            Cow::Owned(other),
            Some((prec, Nearest)),
        )
    }

    /// Computes the average (arithmetic mean) of two [`BigExponentFloat`]s, rounding the result to
    /// the nearest value of the specified precision. Both [`BigExponentFloat`]s are taken by
    /// reference. An [`Ordering`] is also returned, indicating whether the rounded average is less
    /// than, equal to, or greater than the exact average. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,y,p) = \frac{x+y}{2}+\varepsilon.
    /// $$
    /// - If $\frac{x+y}{2}$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $\frac{x+y}{2}$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x+y|/2\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(x,\text{NaN},p)=f(\infty,-\infty,p)=f(-\infty,\infty,p)=\text{NaN}$
    /// - $f(\infty,y,p)=f(y,\infty,p)=\infty$ if $y$ is not `NaN` or $-\infty$
    /// - $f(-\infty,y,p)=f(y,-\infty,p)=-\infty$ if $y$ is not `NaN` or $\infty$
    ///
    /// The average is the rounded sum, halved; halving is exact, since there is no exponent range
    /// to leave.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits(), prec)` if the exponents are close; see
    /// [`BigExponentFloat::add_prec_round`] for the general case.
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
    ///     (&BigExponentFloat::from(PI)).average_prec_ref_ref(&BigExponentFloat::from(E), 5);
    /// assert_eq!(format!("{z:?}"), "0x2.e");
    /// assert_eq!(o, Less);
    ///
    /// let (z, o) =
    ///     (&BigExponentFloat::from(PI)).average_prec_ref_ref(&BigExponentFloat::from(E), 20);
    /// assert_eq!(format!("{z:?}"), "0x2.ee104");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn average_prec_ref_ref(&self, other: &Self, prec: u64) -> (Self, Ordering) {
        average_helper(
            Cow::Borrowed(self),
            Cow::Borrowed(other),
            Some((prec, Nearest)),
        )
    }

    /// Replaces a [`BigExponentFloat`] $x$ with the average of $x$ and another [`BigExponentFloat`]
    /// $y$, rounding the result to the nearest value of the specified precision. The
    /// [`BigExponentFloat`] on the right-hand side is taken by value. An [`Ordering`] is returned,
    /// indicating whether the rounded average is less than, equal to, or greater than the exact
    /// average. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this
    /// function sets the [`BigExponentFloat`] to `NaN` it also returns `Equal`.
    ///
    /// $$
    /// x \gets \frac{x+y}{2}+\varepsilon.
    /// $$
    /// - If $\frac{x+y}{2}$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $\frac{x+y}{2}$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x+y|/2\rfloor-p}$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits(), prec)` if the exponents are close; see
    /// [`BigExponentFloat::add_prec_round`] for the general case.
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
    /// assert_eq!(x.average_prec_assign(BigExponentFloat::from(E), 5), Less);
    /// assert_eq!(format!("{x:?}"), "0x2.e");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(x.average_prec_assign(BigExponentFloat::from(E), 20), Less);
    /// assert_eq!(format!("{x:?}"), "0x2.ee104");
    /// ```
    #[inline]
    pub fn average_prec_assign(&mut self, other: Self, prec: u64) -> Ordering {
        let x = core::mem::replace(self, Self::ZERO);
        let (result, o) = average_helper(Cow::Owned(x), Cow::Owned(other), Some((prec, Nearest)));
        *self = result;
        o
    }

    /// Replaces a [`BigExponentFloat`] $x$ with the average of $x$ and another [`BigExponentFloat`]
    /// $y$, rounding the result to the nearest value of the specified precision. The
    /// [`BigExponentFloat`] on the right-hand side is taken by reference. An [`Ordering`] is
    /// returned, indicating whether the rounded average is less than, equal to, or greater than the
    /// exact average. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this
    /// function sets the [`BigExponentFloat`] to `NaN` it also returns `Equal`.
    ///
    /// $$
    /// x \gets \frac{x+y}{2}+\varepsilon.
    /// $$
    /// - If $\frac{x+y}{2}$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $\frac{x+y}{2}$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x+y|/2\rfloor-p}$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits(), prec)` if the exponents are close; see
    /// [`BigExponentFloat::add_prec_round`] for the general case.
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
    ///     x.average_prec_assign_ref(&BigExponentFloat::from(E), 5),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x2.e");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.average_prec_assign_ref(&BigExponentFloat::from(E), 20),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x2.ee104");
    /// ```
    #[inline]
    pub fn average_prec_assign_ref(&mut self, other: &Self, prec: u64) -> Ordering {
        let x = core::mem::replace(self, Self::ZERO);
        let (result, o) =
            average_helper(Cow::Owned(x), Cow::Borrowed(other), Some((prec, Nearest)));
        *self = result;
        o
    }
}

impl Average<Self> for BigExponentFloat {
    type Output = Self;

    /// Computes the average (arithmetic mean) of two [`BigExponentFloat`]s. The average is exact.
    /// Both [`BigExponentFloat`]s are taken by value.
    ///
    /// $$
    /// f(x,y) = \frac{x+y}{2}.
    /// $$
    ///
    /// Special cases:
    /// - $f(\text{NaN},y)=f(x,\text{NaN})=f(\infty,-\infty)=f(-\infty,\infty)=\text{NaN}$
    /// - $f(\infty,y)=f(y,\infty)=\infty$ if $y$ is not `NaN` or $-\infty$
    /// - $f(-\infty,y)=f(y,-\infty)=-\infty$ if $y$ is not `NaN` or $\infty$
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
    /// use malachite_base::num::arithmetic::traits::Average;
    /// use malachite_float::BigExponentFloat;
    ///
    /// assert_eq!(
    ///     format!(
    ///         "{:?}",
    ///         BigExponentFloat::from(1.5).average(BigExponentFloat::from(2.25))
    ///     ),
    ///     "0x1.e"
    /// );
    /// ```
    #[inline]
    fn average(self, other: Self) -> Self {
        average_helper(Cow::Owned(self), Cow::Owned(other), None).0
    }
}

impl Average<&Self> for BigExponentFloat {
    type Output = Self;

    /// Computes the average (arithmetic mean) of two [`BigExponentFloat`]s. The average is exact.
    /// The first [`BigExponentFloat`] is taken by value and the second by reference.
    ///
    /// $$
    /// f(x,y) = \frac{x+y}{2}.
    /// $$
    ///
    /// Special cases:
    /// - $f(\text{NaN},y)=f(x,\text{NaN})=f(\infty,-\infty)=f(-\infty,\infty)=\text{NaN}$
    /// - $f(\infty,y)=f(y,\infty)=\infty$ if $y$ is not `NaN` or $-\infty$
    /// - $f(-\infty,y)=f(y,-\infty)=-\infty$ if $y$ is not `NaN` or $\infty$
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
    /// use malachite_base::num::arithmetic::traits::Average;
    /// use malachite_float::BigExponentFloat;
    ///
    /// assert_eq!(
    ///     format!(
    ///         "{:?}",
    ///         BigExponentFloat::from(1.5).average(&BigExponentFloat::from(2.25))
    ///     ),
    ///     "0x1.e"
    /// );
    /// ```
    #[inline]
    fn average(self, other: &Self) -> Self {
        average_helper(Cow::Owned(self), Cow::Borrowed(other), None).0
    }
}

impl Average<BigExponentFloat> for &BigExponentFloat {
    type Output = BigExponentFloat;

    /// Computes the average (arithmetic mean) of two [`BigExponentFloat`]s. The average is exact.
    /// The first [`BigExponentFloat`] is taken by reference and the second by value.
    ///
    /// $$
    /// f(x,y) = \frac{x+y}{2}.
    /// $$
    ///
    /// Special cases:
    /// - $f(\text{NaN},y)=f(x,\text{NaN})=f(\infty,-\infty)=f(-\infty,\infty)=\text{NaN}$
    /// - $f(\infty,y)=f(y,\infty)=\infty$ if $y$ is not `NaN` or $-\infty$
    /// - $f(-\infty,y)=f(y,-\infty)=-\infty$ if $y$ is not `NaN` or $\infty$
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
    /// use malachite_base::num::arithmetic::traits::Average;
    /// use malachite_float::BigExponentFloat;
    ///
    /// assert_eq!(
    ///     format!(
    ///         "{:?}",
    ///         (&BigExponentFloat::from(1.5)).average(BigExponentFloat::from(2.25))
    ///     ),
    ///     "0x1.e"
    /// );
    /// ```
    #[inline]
    fn average(self, other: BigExponentFloat) -> BigExponentFloat {
        average_helper(Cow::Borrowed(self), Cow::Owned(other), None).0
    }
}

impl Average<&BigExponentFloat> for &BigExponentFloat {
    type Output = BigExponentFloat;

    /// Computes the average (arithmetic mean) of two [`BigExponentFloat`]s. The average is exact.
    /// Both [`BigExponentFloat`]s are taken by reference.
    ///
    /// $$
    /// f(x,y) = \frac{x+y}{2}.
    /// $$
    ///
    /// Special cases:
    /// - $f(\text{NaN},y)=f(x,\text{NaN})=f(\infty,-\infty)=f(-\infty,\infty)=\text{NaN}$
    /// - $f(\infty,y)=f(y,\infty)=\infty$ if $y$ is not `NaN` or $-\infty$
    /// - $f(-\infty,y)=f(y,-\infty)=-\infty$ if $y$ is not `NaN` or $\infty$
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
    /// use malachite_base::num::arithmetic::traits::Average;
    /// use malachite_float::BigExponentFloat;
    ///
    /// assert_eq!(
    ///     format!(
    ///         "{:?}",
    ///         (&BigExponentFloat::from(1.5)).average(&BigExponentFloat::from(2.25))
    ///     ),
    ///     "0x1.e"
    /// );
    /// ```
    #[inline]
    fn average(self, other: &BigExponentFloat) -> BigExponentFloat {
        average_helper(Cow::Borrowed(self), Cow::Borrowed(other), None).0
    }
}

impl AverageAssign<Self> for BigExponentFloat {
    /// Computes the average (arithmetic mean) of two [`BigExponentFloat`]s. The average is exact.
    /// The [`BigExponentFloat`] on the right-hand side is taken by value.
    ///
    /// $$
    /// x \gets \frac{x+y}{2}.
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
    /// use malachite_base::num::arithmetic::traits::AverageAssign;
    /// use malachite_float::BigExponentFloat;
    ///
    /// let mut x = BigExponentFloat::from(1.5);
    /// x.average_assign(BigExponentFloat::from(2.25));
    /// assert_eq!(format!("{x:?}"), "0x1.e");
    /// ```
    #[inline]
    fn average_assign(&mut self, other: Self) {
        let x = core::mem::replace(self, Self::ZERO);
        *self = average_helper(Cow::Owned(x), Cow::Owned(other), None).0;
    }
}

impl AverageAssign<&Self> for BigExponentFloat {
    /// Computes the average (arithmetic mean) of two [`BigExponentFloat`]s. The average is exact.
    /// The [`BigExponentFloat`] on the right-hand side is taken by reference.
    ///
    /// $$
    /// x \gets \frac{x+y}{2}.
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
    /// use malachite_base::num::arithmetic::traits::AverageAssign;
    /// use malachite_float::BigExponentFloat;
    ///
    /// let mut x = BigExponentFloat::from(1.5);
    /// x.average_assign(&BigExponentFloat::from(2.25));
    /// assert_eq!(format!("{x:?}"), "0x1.e");
    /// ```
    #[inline]
    fn average_assign(&mut self, other: &Self) {
        let x = core::mem::replace(self, Self::ZERO);
        *self = average_helper(Cow::Owned(x), Cow::Borrowed(other), None).0;
    }
}
