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
use core::ops::{Sub, SubAssign};
use malachite_base::num::basic::traits::Zero;
use malachite_base::rounding_modes::RoundingMode::{self, *};

impl BigExponentFloat {
    /// Subtracts two [`BigExponentFloat`]s, rounding the result to the specified precision and with
    /// the specified rounding mode. Both [`BigExponentFloat`]s are taken by value. An [`Ordering`]
    /// is also returned, indicating whether the rounded difference is less than, equal to, or
    /// greater than the exact difference. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
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
    /// - $f(\text{NaN},x,p,m)=f(x,\text{NaN},p,m)=f(\infty,\infty,p,m)=f(-\infty,-\infty,p,m)=
    ///   \text{NaN}$
    /// - $f(\infty,x,p,m)=f(x,-\infty,p,m)=\infty$ if $x$ is not NaN or $\infty$
    /// - $f(-\infty,x,p,m)=f(x,\infty,p,m)=-\infty$ if $x$ is not NaN or $-\infty$
    /// - $f(x,x,p,m)=0$ if $x$ is finite
    ///
    /// If you know you'll be using `Nearest`, consider using [`BigExponentFloat::sub_prec`]
    /// instead. If you want the exact difference, consider using `-` instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n + m)$
    ///
    /// $M(n, m) = O(n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `max(self.significant_bits(), other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but `prec` is too small for an exact
    /// subtraction.
    ///
    /// This is FLINT's `arf_sub`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (diff, o) =
    ///     BigExponentFloat::from(PI).sub_prec_round(BigExponentFloat::from(E), 5, Floor);
    /// assert_eq!(format!("{diff:?}"), "0x0.6c");
    /// assert_eq!(o, Less);
    ///
    /// let (diff, o) =
    ///     BigExponentFloat::from(PI).sub_prec_round(BigExponentFloat::from(E), 5, Ceiling);
    /// assert_eq!(format!("{diff:?}"), "0x0.7");
    /// assert_eq!(o, Greater);
    ///
    /// let (diff, o) =
    ///     BigExponentFloat::from(PI).sub_prec_round(BigExponentFloat::from(E), 5, Nearest);
    /// assert_eq!(format!("{diff:?}"), "0x0.6c");
    /// assert_eq!(o, Less);
    ///
    /// let (diff, o) =
    ///     BigExponentFloat::from(PI).sub_prec_round(BigExponentFloat::from(E), 20, Floor);
    /// assert_eq!(format!("{diff:?}"), "0x0.6c5e18");
    /// assert_eq!(o, Less);
    ///
    /// let (diff, o) =
    ///     BigExponentFloat::from(PI).sub_prec_round(BigExponentFloat::from(E), 20, Ceiling);
    /// assert_eq!(format!("{diff:?}"), "0x0.6c5e2");
    /// assert_eq!(o, Greater);
    ///
    /// let (diff, o) =
    ///     BigExponentFloat::from(PI).sub_prec_round(BigExponentFloat::from(E), 20, Nearest);
    /// assert_eq!(format!("{diff:?}"), "0x0.6c5e18");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn sub_prec_round(self, other: Self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        add_helper(Cow::Owned(self), Cow::Owned(other), true, Some((prec, rm)))
    }

    /// Subtracts two [`BigExponentFloat`]s, rounding the result to the specified precision and with
    /// the specified rounding mode. The first [`BigExponentFloat`] is taken by value and the second
    /// by reference. An [`Ordering`] is also returned, indicating whether the rounded difference is
    /// less than, equal to, or greater than the exact difference. Although `NaN`s are not
    /// comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
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
    /// - $f(\text{NaN},x,p,m)=f(x,\text{NaN},p,m)=f(\infty,\infty,p,m)=f(-\infty,-\infty,p,m)=
    ///   \text{NaN}$
    /// - $f(\infty,x,p,m)=f(x,-\infty,p,m)=\infty$ if $x$ is not NaN or $\infty$
    /// - $f(-\infty,x,p,m)=f(x,\infty,p,m)=-\infty$ if $x$ is not NaN or $-\infty$
    /// - $f(x,x,p,m)=0$ if $x$ is finite
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n + m)$
    ///
    /// $M(n, m) = O(n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `max(self.significant_bits(), other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but `prec` is too small for an exact
    /// subtraction.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (diff, o) =
    ///     BigExponentFloat::from(PI).sub_prec_round_val_ref(&BigExponentFloat::from(E), 5, Floor);
    /// assert_eq!(format!("{diff:?}"), "0x0.6c");
    /// assert_eq!(o, Less);
    ///
    /// let (diff, o) = BigExponentFloat::from(PI).sub_prec_round_val_ref(
    ///     &BigExponentFloat::from(E),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{diff:?}"), "0x0.7");
    /// assert_eq!(o, Greater);
    ///
    /// let (diff, o) = BigExponentFloat::from(PI).sub_prec_round_val_ref(
    ///     &BigExponentFloat::from(E),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{diff:?}"), "0x0.6c");
    /// assert_eq!(o, Less);
    ///
    /// let (diff, o) = BigExponentFloat::from(PI).sub_prec_round_val_ref(
    ///     &BigExponentFloat::from(E),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{diff:?}"), "0x0.6c5e18");
    /// assert_eq!(o, Less);
    ///
    /// let (diff, o) = BigExponentFloat::from(PI).sub_prec_round_val_ref(
    ///     &BigExponentFloat::from(E),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{diff:?}"), "0x0.6c5e2");
    /// assert_eq!(o, Greater);
    ///
    /// let (diff, o) = BigExponentFloat::from(PI).sub_prec_round_val_ref(
    ///     &BigExponentFloat::from(E),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{diff:?}"), "0x0.6c5e18");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn sub_prec_round_val_ref(
        self,
        other: &Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        add_helper(
            Cow::Owned(self),
            Cow::Borrowed(other),
            true,
            Some((prec, rm)),
        )
    }

    /// Subtracts two [`BigExponentFloat`]s, rounding the result to the specified precision and with
    /// the specified rounding mode. The first [`BigExponentFloat`] is taken by reference and the
    /// second by value. An [`Ordering`] is also returned, indicating whether the rounded difference
    /// is less than, equal to, or greater than the exact difference. Although `NaN`s are not
    /// comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
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
    /// - $f(\text{NaN},x,p,m)=f(x,\text{NaN},p,m)=f(\infty,\infty,p,m)=f(-\infty,-\infty,p,m)=
    ///   \text{NaN}$
    /// - $f(\infty,x,p,m)=f(x,-\infty,p,m)=\infty$ if $x$ is not NaN or $\infty$
    /// - $f(-\infty,x,p,m)=f(x,\infty,p,m)=-\infty$ if $x$ is not NaN or $-\infty$
    /// - $f(x,x,p,m)=0$ if $x$ is finite
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n + m)$
    ///
    /// $M(n, m) = O(n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `max(self.significant_bits(), other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but `prec` is too small for an exact
    /// subtraction.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (diff, o) = (&BigExponentFloat::from(PI)).sub_prec_round_ref_val(
    ///     BigExponentFloat::from(E),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{diff:?}"), "0x0.6c");
    /// assert_eq!(o, Less);
    ///
    /// let (diff, o) = (&BigExponentFloat::from(PI)).sub_prec_round_ref_val(
    ///     BigExponentFloat::from(E),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{diff:?}"), "0x0.7");
    /// assert_eq!(o, Greater);
    ///
    /// let (diff, o) = (&BigExponentFloat::from(PI)).sub_prec_round_ref_val(
    ///     BigExponentFloat::from(E),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{diff:?}"), "0x0.6c");
    /// assert_eq!(o, Less);
    ///
    /// let (diff, o) = (&BigExponentFloat::from(PI)).sub_prec_round_ref_val(
    ///     BigExponentFloat::from(E),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{diff:?}"), "0x0.6c5e18");
    /// assert_eq!(o, Less);
    ///
    /// let (diff, o) = (&BigExponentFloat::from(PI)).sub_prec_round_ref_val(
    ///     BigExponentFloat::from(E),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{diff:?}"), "0x0.6c5e2");
    /// assert_eq!(o, Greater);
    ///
    /// let (diff, o) = (&BigExponentFloat::from(PI)).sub_prec_round_ref_val(
    ///     BigExponentFloat::from(E),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{diff:?}"), "0x0.6c5e18");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn sub_prec_round_ref_val(
        &self,
        other: Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        add_helper(
            Cow::Borrowed(self),
            Cow::Owned(other),
            true,
            Some((prec, rm)),
        )
    }

    /// Subtracts two [`BigExponentFloat`]s, rounding the result to the specified precision and with
    /// the specified rounding mode. Both [`BigExponentFloat`]s are taken by reference. An
    /// [`Ordering`] is also returned, indicating whether the rounded difference is less than, equal
    /// to, or greater than the exact difference. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
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
    /// - $f(\text{NaN},x,p,m)=f(x,\text{NaN},p,m)=f(\infty,\infty,p,m)=f(-\infty,-\infty,p,m)=
    ///   \text{NaN}$
    /// - $f(\infty,x,p,m)=f(x,-\infty,p,m)=\infty$ if $x$ is not NaN or $\infty$
    /// - $f(-\infty,x,p,m)=f(x,\infty,p,m)=-\infty$ if $x$ is not NaN or $-\infty$
    /// - $f(x,x,p,m)=0$ if $x$ is finite
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n + m)$
    ///
    /// $M(n, m) = O(n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `max(self.significant_bits(), other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but `prec` is too small for an exact
    /// subtraction.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (diff, o) = (&BigExponentFloat::from(PI)).sub_prec_round_ref_ref(
    ///     &BigExponentFloat::from(E),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{diff:?}"), "0x0.6c");
    /// assert_eq!(o, Less);
    ///
    /// let (diff, o) = (&BigExponentFloat::from(PI)).sub_prec_round_ref_ref(
    ///     &BigExponentFloat::from(E),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{diff:?}"), "0x0.7");
    /// assert_eq!(o, Greater);
    ///
    /// let (diff, o) = (&BigExponentFloat::from(PI)).sub_prec_round_ref_ref(
    ///     &BigExponentFloat::from(E),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{diff:?}"), "0x0.6c");
    /// assert_eq!(o, Less);
    ///
    /// let (diff, o) = (&BigExponentFloat::from(PI)).sub_prec_round_ref_ref(
    ///     &BigExponentFloat::from(E),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{diff:?}"), "0x0.6c5e18");
    /// assert_eq!(o, Less);
    ///
    /// let (diff, o) = (&BigExponentFloat::from(PI)).sub_prec_round_ref_ref(
    ///     &BigExponentFloat::from(E),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{diff:?}"), "0x0.6c5e2");
    /// assert_eq!(o, Greater);
    ///
    /// let (diff, o) = (&BigExponentFloat::from(PI)).sub_prec_round_ref_ref(
    ///     &BigExponentFloat::from(E),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{diff:?}"), "0x0.6c5e18");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn sub_prec_round_ref_ref(
        &self,
        other: &Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        add_helper(
            Cow::Borrowed(self),
            Cow::Borrowed(other),
            true,
            Some((prec, rm)),
        )
    }

    /// Subtracts a [`BigExponentFloat`] by a [`BigExponentFloat`] in place, rounding the result to
    /// the specified precision and with the specified rounding mode. The [`BigExponentFloat`] on
    /// the right-hand side is taken by value. An [`Ordering`] is returned, indicating whether the
    /// rounded difference is less than, equal to, or greater than the exact difference. Although
    /// `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function sets the
    /// [`BigExponentFloat`] to `NaN` it also returns `Equal`.
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
    /// $T(n, m) = O(n + m)$
    ///
    /// $M(n, m) = O(n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `max(self.significant_bits(), other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but `prec` is too small for an exact
    /// subtraction.
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
    ///     x.sub_prec_round_assign(BigExponentFloat::from(E), 5, Floor),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x0.6c");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.sub_prec_round_assign(BigExponentFloat::from(E), 5, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(format!("{x:?}"), "0x0.7");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.sub_prec_round_assign(BigExponentFloat::from(E), 5, Nearest),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x0.6c");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.sub_prec_round_assign(BigExponentFloat::from(E), 20, Floor),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x0.6c5e18");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.sub_prec_round_assign(BigExponentFloat::from(E), 20, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(format!("{x:?}"), "0x0.6c5e2");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.sub_prec_round_assign(BigExponentFloat::from(E), 20, Nearest),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x0.6c5e18");
    /// ```
    #[inline]
    pub fn sub_prec_round_assign(&mut self, other: Self, prec: u64, rm: RoundingMode) -> Ordering {
        let x = core::mem::replace(self, Self::ZERO);
        let (result, o) = add_helper(Cow::Owned(x), Cow::Owned(other), true, Some((prec, rm)));
        *self = result;
        o
    }

    /// Subtracts a [`BigExponentFloat`] by a [`BigExponentFloat`] in place, rounding the result to
    /// the specified precision and with the specified rounding mode. The [`BigExponentFloat`] on
    /// the right-hand side is taken by reference. An [`Ordering`] is returned, indicating whether
    /// the rounded difference is less than, equal to, or greater than the exact difference.
    /// Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function sets
    /// the [`BigExponentFloat`] to `NaN` it also returns `Equal`.
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
    /// $T(n, m) = O(n + m)$
    ///
    /// $M(n, m) = O(n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `max(self.significant_bits(), other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but `prec` is too small for an exact
    /// subtraction.
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
    ///     x.sub_prec_round_assign_ref(&BigExponentFloat::from(E), 5, Floor),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x0.6c");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.sub_prec_round_assign_ref(&BigExponentFloat::from(E), 5, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(format!("{x:?}"), "0x0.7");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.sub_prec_round_assign_ref(&BigExponentFloat::from(E), 5, Nearest),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x0.6c");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.sub_prec_round_assign_ref(&BigExponentFloat::from(E), 20, Floor),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x0.6c5e18");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.sub_prec_round_assign_ref(&BigExponentFloat::from(E), 20, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(format!("{x:?}"), "0x0.6c5e2");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.sub_prec_round_assign_ref(&BigExponentFloat::from(E), 20, Nearest),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x0.6c5e18");
    /// ```
    #[inline]
    pub fn sub_prec_round_assign_ref(
        &mut self,
        other: &Self,
        prec: u64,
        rm: RoundingMode,
    ) -> Ordering {
        let x = core::mem::replace(self, Self::ZERO);
        let (result, o) = add_helper(Cow::Owned(x), Cow::Borrowed(other), true, Some((prec, rm)));
        *self = result;
        o
    }

    /// Subtracts two [`BigExponentFloat`]s, rounding the result to the nearest value of the
    /// specified precision. Both [`BigExponentFloat`]s are taken by value. An [`Ordering`] is also
    /// returned, indicating whether the rounded difference is less than, equal to, or greater than
    /// the exact difference. Although `NaN`s are not comparable to any [`BigExponentFloat`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,y,p) = x-y+\varepsilon.
    /// $$
    /// - If $x-y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-y$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x-y|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,p)=f(x,\text{NaN},p)=f(\infty,\infty,p)=f(-\infty,-\infty,p)=\text{NaN}$
    /// - $f(\infty,x,p)=f(x,-\infty,p)=\infty$ if $x$ is not NaN or $\infty$
    /// - $f(-\infty,x,p)=f(x,\infty,p)=-\infty$ if $x$ is not NaN or $-\infty$
    /// - $f(x,x,p)=0$ if $x$ is finite
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`BigExponentFloat::sub_prec_round`] instead. If you want the exact difference, consider
    /// using `-` instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n + m)$
    ///
    /// $M(n, m) = O(n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `max(self.significant_bits(), other.significant_bits())`.
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
    /// let (diff, o) = BigExponentFloat::from(PI).sub_prec(BigExponentFloat::from(E), 5);
    /// assert_eq!(format!("{diff:?}"), "0x0.6c");
    /// assert_eq!(o, Less);
    ///
    /// let (diff, o) = BigExponentFloat::from(PI).sub_prec(BigExponentFloat::from(E), 20);
    /// assert_eq!(format!("{diff:?}"), "0x0.6c5e18");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn sub_prec(self, other: Self, prec: u64) -> (Self, Ordering) {
        add_helper(
            Cow::Owned(self),
            Cow::Owned(other),
            true,
            Some((prec, Nearest)),
        )
    }

    /// Subtracts two [`BigExponentFloat`]s, rounding the result to the nearest value of the
    /// specified precision. The first [`BigExponentFloat`] is taken by value and the second by
    /// reference. An [`Ordering`] is also returned, indicating whether the rounded difference is
    /// less than, equal to, or greater than the exact difference. Although `NaN`s are not
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
    /// - $f(\text{NaN},x,p)=f(x,\text{NaN},p)=f(\infty,\infty,p)=f(-\infty,-\infty,p)=\text{NaN}$
    /// - $f(\infty,x,p)=f(x,-\infty,p)=\infty$ if $x$ is not NaN or $\infty$
    /// - $f(-\infty,x,p)=f(x,\infty,p)=-\infty$ if $x$ is not NaN or $-\infty$
    /// - $f(x,x,p)=0$ if $x$ is finite
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n + m)$
    ///
    /// $M(n, m) = O(n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `max(self.significant_bits(), other.significant_bits())`.
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
    /// let (diff, o) = BigExponentFloat::from(PI).sub_prec_val_ref(&BigExponentFloat::from(E), 5);
    /// assert_eq!(format!("{diff:?}"), "0x0.6c");
    /// assert_eq!(o, Less);
    ///
    /// let (diff, o) = BigExponentFloat::from(PI).sub_prec_val_ref(&BigExponentFloat::from(E), 20);
    /// assert_eq!(format!("{diff:?}"), "0x0.6c5e18");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn sub_prec_val_ref(self, other: &Self, prec: u64) -> (Self, Ordering) {
        add_helper(
            Cow::Owned(self),
            Cow::Borrowed(other),
            true,
            Some((prec, Nearest)),
        )
    }

    /// Subtracts two [`BigExponentFloat`]s, rounding the result to the nearest value of the
    /// specified precision. The first [`BigExponentFloat`] is taken by reference and the second by
    /// value. An [`Ordering`] is also returned, indicating whether the rounded difference is less
    /// than, equal to, or greater than the exact difference. Although `NaN`s are not comparable to
    /// any [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,y,p) = x-y+\varepsilon.
    /// $$
    /// - If $x-y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-y$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x-y|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,p)=f(x,\text{NaN},p)=f(\infty,\infty,p)=f(-\infty,-\infty,p)=\text{NaN}$
    /// - $f(\infty,x,p)=f(x,-\infty,p)=\infty$ if $x$ is not NaN or $\infty$
    /// - $f(-\infty,x,p)=f(x,\infty,p)=-\infty$ if $x$ is not NaN or $-\infty$
    /// - $f(x,x,p)=0$ if $x$ is finite
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n + m)$
    ///
    /// $M(n, m) = O(n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `max(self.significant_bits(), other.significant_bits())`.
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
    /// let (diff, o) =
    ///     (&BigExponentFloat::from(PI)).sub_prec_ref_val(BigExponentFloat::from(E), 5);
    /// assert_eq!(format!("{diff:?}"), "0x0.6c");
    /// assert_eq!(o, Less);
    ///
    /// let (diff, o) =
    ///     (&BigExponentFloat::from(PI)).sub_prec_ref_val(BigExponentFloat::from(E), 20);
    /// assert_eq!(format!("{diff:?}"), "0x0.6c5e18");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn sub_prec_ref_val(&self, other: Self, prec: u64) -> (Self, Ordering) {
        add_helper(
            Cow::Borrowed(self),
            Cow::Owned(other),
            true,
            Some((prec, Nearest)),
        )
    }

    /// Subtracts two [`BigExponentFloat`]s, rounding the result to the nearest value of the
    /// specified precision. Both [`BigExponentFloat`]s are taken by reference. An [`Ordering`] is
    /// also returned, indicating whether the rounded difference is less than, equal to, or greater
    /// than the exact difference. Although `NaN`s are not comparable to any [`BigExponentFloat`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,y,p) = x-y+\varepsilon.
    /// $$
    /// - If $x-y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-y$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x-y|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,p)=f(x,\text{NaN},p)=f(\infty,\infty,p)=f(-\infty,-\infty,p)=\text{NaN}$
    /// - $f(\infty,x,p)=f(x,-\infty,p)=\infty$ if $x$ is not NaN or $\infty$
    /// - $f(-\infty,x,p)=f(x,\infty,p)=-\infty$ if $x$ is not NaN or $-\infty$
    /// - $f(x,x,p)=0$ if $x$ is finite
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n + m)$
    ///
    /// $M(n, m) = O(n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `max(self.significant_bits(), other.significant_bits())`.
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
    /// let (diff, o) =
    ///     (&BigExponentFloat::from(PI)).sub_prec_ref_ref(&BigExponentFloat::from(E), 5);
    /// assert_eq!(format!("{diff:?}"), "0x0.6c");
    /// assert_eq!(o, Less);
    ///
    /// let (diff, o) =
    ///     (&BigExponentFloat::from(PI)).sub_prec_ref_ref(&BigExponentFloat::from(E), 20);
    /// assert_eq!(format!("{diff:?}"), "0x0.6c5e18");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn sub_prec_ref_ref(&self, other: &Self, prec: u64) -> (Self, Ordering) {
        add_helper(
            Cow::Borrowed(self),
            Cow::Borrowed(other),
            true,
            Some((prec, Nearest)),
        )
    }

    /// Subtracts a [`BigExponentFloat`] by a [`BigExponentFloat`] in place, rounding the result to
    /// the nearest value of the specified precision. The [`BigExponentFloat`] on the right-hand
    /// side is taken by value. An [`Ordering`] is returned, indicating whether the rounded
    /// difference is less than, equal to, or greater than the exact difference. Although `NaN`s are
    /// not comparable to any [`BigExponentFloat`], whenever this function sets the
    /// [`BigExponentFloat`] to `NaN` it also returns `Equal`.
    ///
    /// $$
    /// x \gets x-y+\varepsilon.
    /// $$
    /// - If $x-y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-y$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x-y|\rfloor-p}$.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n + m)$
    ///
    /// $M(n, m) = O(n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `max(self.significant_bits(), other.significant_bits())`.
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
    /// assert_eq!(x.sub_prec_assign(BigExponentFloat::from(E), 5), Less);
    /// assert_eq!(format!("{x:?}"), "0x0.6c");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(x.sub_prec_assign(BigExponentFloat::from(E), 20), Less);
    /// assert_eq!(format!("{x:?}"), "0x0.6c5e18");
    /// ```
    #[inline]
    pub fn sub_prec_assign(&mut self, other: Self, prec: u64) -> Ordering {
        let x = core::mem::replace(self, Self::ZERO);
        let (result, o) = add_helper(
            Cow::Owned(x),
            Cow::Owned(other),
            true,
            Some((prec, Nearest)),
        );
        *self = result;
        o
    }

    /// Subtracts a [`BigExponentFloat`] by a [`BigExponentFloat`] in place, rounding the result to
    /// the nearest value of the specified precision. The [`BigExponentFloat`] on the right-hand
    /// side is taken by reference. An [`Ordering`] is returned, indicating whether the rounded
    /// difference is less than, equal to, or greater than the exact difference. Although `NaN`s are
    /// not comparable to any [`BigExponentFloat`], whenever this function sets the
    /// [`BigExponentFloat`] to `NaN` it also returns `Equal`.
    ///
    /// $$
    /// x \gets x-y+\varepsilon.
    /// $$
    /// - If $x-y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-y$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x-y|\rfloor-p}$.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n + m)$
    ///
    /// $M(n, m) = O(n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `max(self.significant_bits(), other.significant_bits())`.
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
    /// assert_eq!(x.sub_prec_assign_ref(&BigExponentFloat::from(E), 5), Less);
    /// assert_eq!(format!("{x:?}"), "0x0.6c");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(x.sub_prec_assign_ref(&BigExponentFloat::from(E), 20), Less);
    /// assert_eq!(format!("{x:?}"), "0x0.6c5e18");
    /// ```
    #[inline]
    pub fn sub_prec_assign_ref(&mut self, other: &Self, prec: u64) -> Ordering {
        let x = core::mem::replace(self, Self::ZERO);
        let (result, o) = add_helper(
            Cow::Owned(x),
            Cow::Borrowed(other),
            true,
            Some((prec, Nearest)),
        );
        *self = result;
        o
    }
}

impl Sub<Self> for BigExponentFloat {
    type Output = Self;

    /// Subtracts two [`BigExponentFloat`]s. Both [`BigExponentFloat`]s are taken by value. The
    /// difference is exact.
    ///
    /// $$
    /// f(x,y) = x-y.
    /// $$
    ///
    /// Special cases:
    /// - $f(\text{NaN},x)=f(x,\text{NaN})=f(\infty,\infty)=f(-\infty,-\infty)=\text{NaN}$
    /// - $f(\infty,x)=f(x,-\infty)=\infty$ if $x$ is not NaN or $\infty$
    /// - $f(-\infty,x)=f(x,\infty)=-\infty$ if $x$ is not NaN or $-\infty$
    /// - $f(x,x)=0$ if $x$ is finite
    ///
    /// When both values are finite and nonzero, the significand of the difference spans the gap
    /// between their exponents, so values whose exponents are far apart give a large difference. To
    /// bound its size, use [`BigExponentFloat::sub_prec_round`] or [`BigExponentFloat::sub_prec`]
    /// instead.
    ///
    /// This is FLINT's `arf_sub` with precision `ARF_PREC_EXACT`.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())` plus the absolute difference of the two exponents.
    ///
    /// # Panics
    /// Panics if the exponents differ by $2^{64}$ or more and both values are finite and nonzero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::BigExponentFloat;
    ///
    /// assert_eq!(
    ///     format!(
    ///         "{:?}",
    ///         BigExponentFloat::from(1.5) - BigExponentFloat::from(0.25)
    ///     ),
    ///     "0x1.4"
    /// );
    /// ```
    #[inline]
    fn sub(self, other: Self) -> Self {
        add_helper(Cow::Owned(self), Cow::Owned(other), true, None).0
    }
}

impl Sub<&Self> for BigExponentFloat {
    type Output = Self;

    /// Subtracts two [`BigExponentFloat`]s. The first [`BigExponentFloat`] is taken by value and
    /// the second by reference. The difference is exact.
    ///
    /// $$
    /// f(x,y) = x-y.
    /// $$
    ///
    /// Special cases:
    /// - $f(\text{NaN},x)=f(x,\text{NaN})=f(\infty,\infty)=f(-\infty,-\infty)=\text{NaN}$
    /// - $f(\infty,x)=f(x,-\infty)=\infty$ if $x$ is not NaN or $\infty$
    /// - $f(-\infty,x)=f(x,\infty)=-\infty$ if $x$ is not NaN or $-\infty$
    /// - $f(x,x)=0$ if $x$ is finite
    ///
    /// When both values are finite and nonzero, the significand of the difference spans the gap
    /// between their exponents, so values whose exponents are far apart give a large difference. To
    /// bound its size, use [`BigExponentFloat::sub_prec_round`] or [`BigExponentFloat::sub_prec`]
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())` plus the absolute difference of the two exponents.
    ///
    /// # Panics
    /// Panics if the exponents differ by $2^{64}$ or more and both values are finite and nonzero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::BigExponentFloat;
    ///
    /// assert_eq!(
    ///     format!(
    ///         "{:?}",
    ///         BigExponentFloat::from(1.5) - &BigExponentFloat::from(0.25)
    ///     ),
    ///     "0x1.4"
    /// );
    /// ```
    #[inline]
    fn sub(self, other: &Self) -> Self {
        add_helper(Cow::Owned(self), Cow::Borrowed(other), true, None).0
    }
}

impl Sub<BigExponentFloat> for &BigExponentFloat {
    type Output = BigExponentFloat;

    /// Subtracts two [`BigExponentFloat`]s. The first [`BigExponentFloat`] is taken by reference
    /// and the second by value. The difference is exact.
    ///
    /// $$
    /// f(x,y) = x-y.
    /// $$
    ///
    /// Special cases:
    /// - $f(\text{NaN},x)=f(x,\text{NaN})=f(\infty,\infty)=f(-\infty,-\infty)=\text{NaN}$
    /// - $f(\infty,x)=f(x,-\infty)=\infty$ if $x$ is not NaN or $\infty$
    /// - $f(-\infty,x)=f(x,\infty)=-\infty$ if $x$ is not NaN or $-\infty$
    /// - $f(x,x)=0$ if $x$ is finite
    ///
    /// When both values are finite and nonzero, the significand of the difference spans the gap
    /// between their exponents, so values whose exponents are far apart give a large difference. To
    /// bound its size, use [`BigExponentFloat::sub_prec_round`] or [`BigExponentFloat::sub_prec`]
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())` plus the absolute difference of the two exponents.
    ///
    /// # Panics
    /// Panics if the exponents differ by $2^{64}$ or more and both values are finite and nonzero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::BigExponentFloat;
    ///
    /// assert_eq!(
    ///     format!(
    ///         "{:?}",
    ///         (&BigExponentFloat::from(1.5)) - BigExponentFloat::from(0.25)
    ///     ),
    ///     "0x1.4"
    /// );
    /// ```
    #[inline]
    fn sub(self, other: BigExponentFloat) -> BigExponentFloat {
        add_helper(Cow::Borrowed(self), Cow::Owned(other), true, None).0
    }
}

impl Sub<&BigExponentFloat> for &BigExponentFloat {
    type Output = BigExponentFloat;

    /// Subtracts two [`BigExponentFloat`]s. Both [`BigExponentFloat`]s are taken by reference. The
    /// difference is exact.
    ///
    /// $$
    /// f(x,y) = x-y.
    /// $$
    ///
    /// Special cases:
    /// - $f(\text{NaN},x)=f(x,\text{NaN})=f(\infty,\infty)=f(-\infty,-\infty)=\text{NaN}$
    /// - $f(\infty,x)=f(x,-\infty)=\infty$ if $x$ is not NaN or $\infty$
    /// - $f(-\infty,x)=f(x,\infty)=-\infty$ if $x$ is not NaN or $-\infty$
    /// - $f(x,x)=0$ if $x$ is finite
    ///
    /// When both values are finite and nonzero, the significand of the difference spans the gap
    /// between their exponents, so values whose exponents are far apart give a large difference. To
    /// bound its size, use [`BigExponentFloat::sub_prec_round`] or [`BigExponentFloat::sub_prec`]
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())` plus the absolute difference of the two exponents.
    ///
    /// # Panics
    /// Panics if the exponents differ by $2^{64}$ or more and both values are finite and nonzero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::BigExponentFloat;
    ///
    /// assert_eq!(
    ///     format!(
    ///         "{:?}",
    ///         (&BigExponentFloat::from(1.5)) - &BigExponentFloat::from(0.25)
    ///     ),
    ///     "0x1.4"
    /// );
    /// ```
    #[inline]
    fn sub(self, other: &BigExponentFloat) -> BigExponentFloat {
        add_helper(Cow::Borrowed(self), Cow::Borrowed(other), true, None).0
    }
}

impl SubAssign<Self> for BigExponentFloat {
    /// Subtracts a [`BigExponentFloat`] by a [`BigExponentFloat`] in place. The
    /// [`BigExponentFloat`] on the right-hand side is taken by value. The difference is exact.
    ///
    /// $$
    /// x \gets x-y.
    /// $$
    ///
    /// See the `-` documentation for details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())` plus the absolute difference of the two exponents.
    ///
    /// # Panics
    /// Panics if the exponents differ by $2^{64}$ or more and both values are finite and nonzero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::BigExponentFloat;
    ///
    /// let mut x = BigExponentFloat::from(1.5);
    /// x -= BigExponentFloat::from(0.25);
    /// assert_eq!(format!("{x:?}"), "0x1.4");
    /// ```
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        let x = core::mem::replace(self, Self::ZERO);
        *self = add_helper(Cow::Owned(x), Cow::Owned(other), true, None).0;
    }
}

impl SubAssign<&Self> for BigExponentFloat {
    /// Subtracts a [`BigExponentFloat`] by a [`BigExponentFloat`] in place. The
    /// [`BigExponentFloat`] on the right-hand side is taken by reference. The difference is exact.
    ///
    /// $$
    /// x \gets x-y.
    /// $$
    ///
    /// See the `-` documentation for details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())` plus the absolute difference of the two exponents.
    ///
    /// # Panics
    /// Panics if the exponents differ by $2^{64}$ or more and both values are finite and nonzero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::BigExponentFloat;
    ///
    /// let mut x = BigExponentFloat::from(1.5);
    /// x -= &BigExponentFloat::from(0.25);
    /// assert_eq!(format!("{x:?}"), "0x1.4");
    /// ```
    #[inline]
    fn sub_assign(&mut self, other: &Self) {
        let x = core::mem::replace(self, Self::ZERO);
        *self = add_helper(Cow::Owned(x), Cow::Borrowed(other), true, None).0;
    }
}
