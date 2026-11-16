// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::big_exponent_float::BigExponentFloat;
use crate::big_exponent_float::arithmetic::root::{root_helper, root_rational_helper};
use alloc::borrow::Cow;
use core::cmp::Ordering;
use malachite_base::num::basic::traits::Zero;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_q::Rational;

impl BigExponentFloat {
    /// Computes the cube root of a [`BigExponentFloat`], rounding the result to the specified
    /// precision and with the specified rounding mode. The [`BigExponentFloat`] is taken by value.
    /// An [`Ordering`] is also returned, indicating whether the rounded cube root is less than,
    /// equal to, or greater than the exact cube root. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \sqrt\[3\]{x}+\varepsilon.
    /// $$
    /// - If $\sqrt\[3\]{x}$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $\sqrt\[3\]{x}$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\sqrt\[3\]{x}|\rfloor-p+1}$.
    /// - If $\sqrt\[3\]{x}$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\sqrt\[3\]{x}|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=\text{NaN}$
    /// - $f(\infty,p,m)=\infty$
    /// - $f(-\infty,p,m)=-\infty$
    /// - $f(0,p,m)=0$
    ///
    /// If you know you'll be using `Nearest`, consider using [`BigExponentFloat::cbrt_prec`]
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec,
    /// self.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the cube root cannot be represented
    /// exactly with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (cbrt, o) = BigExponentFloat::from(PI).cbrt_prec_round(5, Floor);
    /// assert_eq!(cbrt.to_string(), "1.44");
    /// assert_eq!(o, Less);
    ///
    /// let (cbrt, o) = BigExponentFloat::from(PI).cbrt_prec_round(5, Ceiling);
    /// assert_eq!(cbrt.to_string(), "1.5");
    /// assert_eq!(o, Greater);
    ///
    /// let (cbrt, o) = BigExponentFloat::from(PI).cbrt_prec_round(5, Nearest);
    /// assert_eq!(cbrt.to_string(), "1.44");
    /// assert_eq!(o, Less);
    ///
    /// let (cbrt, o) = BigExponentFloat::from(PI).cbrt_prec_round(20, Floor);
    /// assert_eq!(cbrt.to_string(), "1.4645901");
    /// assert_eq!(o, Less);
    ///
    /// let (cbrt, o) = BigExponentFloat::from(PI).cbrt_prec_round(20, Ceiling);
    /// assert_eq!(cbrt.to_string(), "1.464592");
    /// assert_eq!(o, Greater);
    ///
    /// let (cbrt, o) = BigExponentFloat::from(PI).cbrt_prec_round(20, Nearest);
    /// assert_eq!(cbrt.to_string(), "1.464592");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn cbrt_prec_round(self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        root_helper(Cow::Owned(self), 3, prec, rm)
    }

    /// Computes the cube root of a [`BigExponentFloat`], rounding the result to the specified
    /// precision and with the specified rounding mode. The [`BigExponentFloat`] is taken by
    /// reference. An [`Ordering`] is also returned, indicating whether the rounded cube root is
    /// less than, equal to, or greater than the exact cube root. Although `NaN`s are not comparable
    /// to any [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \sqrt\[3\]{x}+\varepsilon.
    /// $$
    /// - If $\sqrt\[3\]{x}$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $\sqrt\[3\]{x}$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\sqrt\[3\]{x}|\rfloor-p+1}$.
    /// - If $\sqrt\[3\]{x}$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\sqrt\[3\]{x}|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=\text{NaN}$
    /// - $f(\infty,p,m)=\infty$
    /// - $f(-\infty,p,m)=-\infty$
    /// - $f(0,p,m)=0$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec,
    /// self.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the cube root cannot be represented
    /// exactly with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (cbrt, o) = (&BigExponentFloat::from(PI)).cbrt_prec_round_ref(5, Floor);
    /// assert_eq!(cbrt.to_string(), "1.44");
    /// assert_eq!(o, Less);
    ///
    /// let (cbrt, o) = (&BigExponentFloat::from(PI)).cbrt_prec_round_ref(5, Ceiling);
    /// assert_eq!(cbrt.to_string(), "1.5");
    /// assert_eq!(o, Greater);
    ///
    /// let (cbrt, o) = (&BigExponentFloat::from(PI)).cbrt_prec_round_ref(5, Nearest);
    /// assert_eq!(cbrt.to_string(), "1.44");
    /// assert_eq!(o, Less);
    ///
    /// let (cbrt, o) = (&BigExponentFloat::from(PI)).cbrt_prec_round_ref(20, Floor);
    /// assert_eq!(cbrt.to_string(), "1.4645901");
    /// assert_eq!(o, Less);
    ///
    /// let (cbrt, o) = (&BigExponentFloat::from(PI)).cbrt_prec_round_ref(20, Ceiling);
    /// assert_eq!(cbrt.to_string(), "1.464592");
    /// assert_eq!(o, Greater);
    ///
    /// let (cbrt, o) = (&BigExponentFloat::from(PI)).cbrt_prec_round_ref(20, Nearest);
    /// assert_eq!(cbrt.to_string(), "1.464592");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn cbrt_prec_round_ref(&self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        root_helper(Cow::Borrowed(self), 3, prec, rm)
    }

    /// Computes the cube root of a [`BigExponentFloat`] in place, rounding the result to the
    /// specified precision and with the specified rounding mode. An [`Ordering`] is returned,
    /// indicating whether the rounded cube root is less than, equal to, or greater than the exact
    /// cube root. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this
    /// function sets the [`BigExponentFloat`] to `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets \sqrt\[3\]{x}+\varepsilon.
    /// $$
    /// - If $\sqrt\[3\]{x}$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $\sqrt\[3\]{x}$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\sqrt\[3\]{x}|\rfloor-p+1}$.
    /// - If $\sqrt\[3\]{x}$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\sqrt\[3\]{x}|\rfloor-p}$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec,
    /// self.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the cube root cannot be represented
    /// exactly with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(x.cbrt_prec_round_assign(5, Floor), Less);
    /// assert_eq!(x.to_string(), "1.44");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(x.cbrt_prec_round_assign(5, Ceiling), Greater);
    /// assert_eq!(x.to_string(), "1.5");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(x.cbrt_prec_round_assign(5, Nearest), Less);
    /// assert_eq!(x.to_string(), "1.44");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(x.cbrt_prec_round_assign(20, Floor), Less);
    /// assert_eq!(x.to_string(), "1.4645901");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(x.cbrt_prec_round_assign(20, Ceiling), Greater);
    /// assert_eq!(x.to_string(), "1.464592");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(x.cbrt_prec_round_assign(20, Nearest), Greater);
    /// assert_eq!(x.to_string(), "1.464592");
    /// ```
    #[inline]
    pub fn cbrt_prec_round_assign(&mut self, prec: u64, rm: RoundingMode) -> Ordering {
        let x = core::mem::replace(self, Self::ZERO);
        let (result, o) = root_helper(Cow::Owned(x), 3, prec, rm);
        *self = result;
        o
    }

    /// Computes the cube root of a [`BigExponentFloat`], rounding the result to the nearest value
    /// of the specified precision. The [`BigExponentFloat`] is taken by value. An [`Ordering`] is
    /// also returned, indicating whether the rounded cube root is less than, equal to, or greater
    /// than the exact cube root. Although `NaN`s are not comparable to any [`BigExponentFloat`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,p) = \sqrt\[3\]{x}+\varepsilon.
    /// $$
    /// - If $\sqrt\[3\]{x}$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $\sqrt\[3\]{x}$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\sqrt\[3\]{x}|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p)=\text{NaN}$
    /// - $f(\infty,p)=\infty$
    /// - $f(-\infty,p)=-\infty$
    /// - $f(0,p)=0$
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`BigExponentFloat::cbrt_prec_round`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec,
    /// self.significant_bits())`.
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
    /// let (cbrt, o) = BigExponentFloat::from(PI).cbrt_prec(5);
    /// assert_eq!(cbrt.to_string(), "1.44");
    /// assert_eq!(o, Less);
    ///
    /// let (cbrt, o) = BigExponentFloat::from(PI).cbrt_prec(20);
    /// assert_eq!(cbrt.to_string(), "1.464592");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn cbrt_prec(self, prec: u64) -> (Self, Ordering) {
        root_helper(Cow::Owned(self), 3, prec, Nearest)
    }

    /// Computes the cube root of a [`BigExponentFloat`], rounding the result to the nearest value
    /// of the specified precision. The [`BigExponentFloat`] is taken by reference. An [`Ordering`]
    /// is also returned, indicating whether the rounded cube root is less than, equal to, or
    /// greater than the exact cube root. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,p) = \sqrt\[3\]{x}+\varepsilon.
    /// $$
    /// - If $\sqrt\[3\]{x}$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $\sqrt\[3\]{x}$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\sqrt\[3\]{x}|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p)=\text{NaN}$
    /// - $f(\infty,p)=\infty$
    /// - $f(-\infty,p)=-\infty$
    /// - $f(0,p)=0$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec,
    /// self.significant_bits())`.
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
    /// let (cbrt, o) = (&BigExponentFloat::from(PI)).cbrt_prec_ref(5);
    /// assert_eq!(cbrt.to_string(), "1.44");
    /// assert_eq!(o, Less);
    ///
    /// let (cbrt, o) = (&BigExponentFloat::from(PI)).cbrt_prec_ref(20);
    /// assert_eq!(cbrt.to_string(), "1.464592");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn cbrt_prec_ref(&self, prec: u64) -> (Self, Ordering) {
        root_helper(Cow::Borrowed(self), 3, prec, Nearest)
    }

    /// Computes the cube root of a [`BigExponentFloat`] in place, rounding the result to the
    /// nearest value of the specified precision. An [`Ordering`] is returned, indicating whether
    /// the rounded cube root is less than, equal to, or greater than the exact cube root. Although
    /// `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function sets the
    /// [`BigExponentFloat`] to `NaN` it also returns `Equal`.
    ///
    /// $$
    /// x \gets \sqrt\[3\]{x}+\varepsilon.
    /// $$
    /// - If $\sqrt\[3\]{x}$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $\sqrt\[3\]{x}$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\sqrt\[3\]{x}|\rfloor-p}$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec,
    /// self.significant_bits())`.
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
    /// assert_eq!(x.cbrt_prec_assign(5), Less);
    /// assert_eq!(x.to_string(), "1.44");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(x.cbrt_prec_assign(20), Greater);
    /// assert_eq!(x.to_string(), "1.464592");
    /// ```
    #[inline]
    pub fn cbrt_prec_assign(&mut self, prec: u64) -> Ordering {
        let x = core::mem::replace(self, Self::ZERO);
        let (result, o) = root_helper(Cow::Owned(x), 3, prec, Nearest);
        *self = result;
        o
    }
}

impl BigExponentFloat {
    /// Computes the cube root of a [`Rational`], returning a [`BigExponentFloat`], rounding the
    /// result to the specified precision and with the specified rounding mode. The [`Rational`] is
    /// taken by value. An [`Ordering`] is also returned, indicating whether the rounded cube root
    /// is less than, equal to, or greater than the exact cube root. Although `NaN`s are not
    /// comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \sqrt\[3\]{x}+\varepsilon.
    /// $$
    /// - If $\sqrt\[3\]{x}$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $\sqrt\[3\]{x}$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\sqrt\[3\]{x}|\rfloor-p+1}$.
    /// - If $\sqrt\[3\]{x}$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\sqrt\[3\]{x}|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(0,p,m)=0$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::cbrt_rational_prec_round), this function
    /// never overflows or underflows.
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`BigExponentFloat::cbrt_rational_prec`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec, x.significant_bits())`.
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
    /// let (cbrt, o) =
    ///     BigExponentFloat::cbrt_rational_prec_round(Rational::from_unsigneds(3u8, 5), 5, Floor);
    /// assert_eq!(cbrt.to_string(), "0.812");
    /// assert_eq!(o, Less);
    ///
    /// let (cbrt, o) = BigExponentFloat::cbrt_rational_prec_round(
    ///     Rational::from_unsigneds(3u8, 5),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(cbrt.to_string(), "0.844");
    /// assert_eq!(o, Greater);
    ///
    /// let (cbrt, o) = BigExponentFloat::cbrt_rational_prec_round(
    ///     Rational::from_unsigneds(3u8, 5),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(cbrt.to_string(), "0.844");
    /// assert_eq!(o, Greater);
    ///
    /// let (cbrt, o) =
    ///     BigExponentFloat::cbrt_rational_prec_round(Rational::from_unsigneds(3u8, 5), 20, Floor);
    /// assert_eq!(cbrt.to_string(), "0.84343243");
    /// assert_eq!(o, Less);
    ///
    /// let (cbrt, o) = BigExponentFloat::cbrt_rational_prec_round(
    ///     Rational::from_unsigneds(3u8, 5),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(cbrt.to_string(), "0.8434334");
    /// assert_eq!(o, Greater);
    ///
    /// let (cbrt, o) = BigExponentFloat::cbrt_rational_prec_round(
    ///     Rational::from_unsigneds(3u8, 5),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(cbrt.to_string(), "0.84343243");
    /// assert_eq!(o, Less);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn cbrt_rational_prec_round(x: Rational, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        root_rational_helper(&x, 3, prec, rm)
    }

    /// Computes the cube root of a [`Rational`], returning a [`BigExponentFloat`], rounding the
    /// result to the specified precision and with the specified rounding mode. The [`Rational`] is
    /// taken by reference. An [`Ordering`] is also returned, indicating whether the rounded cube
    /// root is less than, equal to, or greater than the exact cube root. Although `NaN`s are not
    /// comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \sqrt\[3\]{x}+\varepsilon.
    /// $$
    /// - If $\sqrt\[3\]{x}$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $\sqrt\[3\]{x}$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\sqrt\[3\]{x}|\rfloor-p+1}$.
    /// - If $\sqrt\[3\]{x}$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\sqrt\[3\]{x}|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(0,p,m)=0$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::cbrt_rational_prec_round), this function
    /// never overflows or underflows.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec, x.significant_bits())`.
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
    /// let (cbrt, o) = BigExponentFloat::cbrt_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(3u8, 5),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(cbrt.to_string(), "0.812");
    /// assert_eq!(o, Less);
    ///
    /// let (cbrt, o) = BigExponentFloat::cbrt_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(3u8, 5),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(cbrt.to_string(), "0.844");
    /// assert_eq!(o, Greater);
    ///
    /// let (cbrt, o) = BigExponentFloat::cbrt_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(3u8, 5),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(cbrt.to_string(), "0.844");
    /// assert_eq!(o, Greater);
    ///
    /// let (cbrt, o) = BigExponentFloat::cbrt_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(3u8, 5),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(cbrt.to_string(), "0.84343243");
    /// assert_eq!(o, Less);
    ///
    /// let (cbrt, o) = BigExponentFloat::cbrt_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(3u8, 5),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(cbrt.to_string(), "0.8434334");
    /// assert_eq!(o, Greater);
    ///
    /// let (cbrt, o) = BigExponentFloat::cbrt_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(3u8, 5),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(cbrt.to_string(), "0.84343243");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn cbrt_rational_prec_round_ref(
        x: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        root_rational_helper(x, 3, prec, rm)
    }

    /// Computes the cube root of a [`Rational`], returning a [`BigExponentFloat`], rounding the
    /// result to the nearest value of the specified precision. The [`Rational`] is taken by value.
    /// An [`Ordering`] is also returned, indicating whether the rounded cube root is less than,
    /// equal to, or greater than the exact cube root. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,p) = \sqrt\[3\]{x}+\varepsilon.
    /// $$
    /// - If $\sqrt\[3\]{x}$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $\sqrt\[3\]{x}$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\sqrt\[3\]{x}|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(0,p)=0$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::cbrt_rational_prec), this function never
    /// overflows or underflows.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`BigExponentFloat::cbrt_rational_prec_round`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec, x.significant_bits())`.
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
    /// let (cbrt, o) = BigExponentFloat::cbrt_rational_prec(Rational::from_unsigneds(3u8, 5), 5);
    /// assert_eq!(cbrt.to_string(), "0.844");
    /// assert_eq!(o, Greater);
    ///
    /// let (cbrt, o) = BigExponentFloat::cbrt_rational_prec(Rational::from_unsigneds(3u8, 5), 20);
    /// assert_eq!(cbrt.to_string(), "0.84343243");
    /// assert_eq!(o, Less);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn cbrt_rational_prec(x: Rational, prec: u64) -> (Self, Ordering) {
        root_rational_helper(&x, 3, prec, Nearest)
    }

    /// Computes the cube root of a [`Rational`], returning a [`BigExponentFloat`], rounding the
    /// result to the nearest value of the specified precision. The [`Rational`] is taken by
    /// reference. An [`Ordering`] is also returned, indicating whether the rounded cube root is
    /// less than, equal to, or greater than the exact cube root. Although `NaN`s are not comparable
    /// to any [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,p) = \sqrt\[3\]{x}+\varepsilon.
    /// $$
    /// - If $\sqrt\[3\]{x}$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $\sqrt\[3\]{x}$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\sqrt\[3\]{x}|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(0,p)=0$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::cbrt_rational_prec), this function never
    /// overflows or underflows.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec, x.significant_bits())`.
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
    /// let (cbrt, o) =
    ///     BigExponentFloat::cbrt_rational_prec_ref(&Rational::from_unsigneds(3u8, 5), 5);
    /// assert_eq!(cbrt.to_string(), "0.844");
    /// assert_eq!(o, Greater);
    ///
    /// let (cbrt, o) =
    ///     BigExponentFloat::cbrt_rational_prec_ref(&Rational::from_unsigneds(3u8, 5), 20);
    /// assert_eq!(cbrt.to_string(), "0.84343243");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn cbrt_rational_prec_ref(x: &Rational, prec: u64) -> (Self, Ordering) {
        root_rational_helper(x, 3, prec, Nearest)
    }
}
