// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::big_exponent_float::BigExponentFloat;
use crate::big_exponent_float::InnerBigExponentFloat::{Finite, Infinity, NaN, Zero};
use crate::big_exponent_float::arithmetic::add::{finish_significand, kernel_prec, round_optional};
use crate::big_exponent_float::arithmetic::root::root_rational_helper;
use alloc::borrow::Cow;
use core::cmp::Ordering::{self, *};
use malachite_base::num::arithmetic::traits::{Parity, SqrtRem};
use malachite_base::num::basic::traits::{
    Infinity as InfinityTrait, NaN as NaNTrait, One, Zero as ZeroTrait,
};
use malachite_base::num::logic::traits::{BitAccess, SignificantBits};
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_nz::integer::Integer;
use malachite_nz::natural::arithmetic::float::sqrt::{
    sqrt_float_significand_in_place, sqrt_float_significand_ref,
};
use malachite_q::Rational;

// The square root of x, rounded to `prec` bits with `rm`.
pub(crate) fn sqrt_helper(
    x: Cow<'_, BigExponentFloat>,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    sqrt_helper_with(x, prec, rm, true)
}

crate_test_fn! {
    // The square root of x, rounded to `prec` bits with `rm`, without `Float`'s kernels: a square
    // root of the significand with at least `prec + 2` bits, and a sticky bit recording whether it
    // is exact, are formed with `Natural` arithmetic and then rounded. Used to check and benchmark
    // the kernel path.
    sqrt_prec_round_generic(
        x: &BigExponentFloat,
        prec: u64,
        rm: RoundingMode,
    ) -> (BigExponentFloat, Ordering) {
        sqrt_helper_with(Cow::Borrowed(x), prec, rm, false)
    }
}

// The special cases are those of FLINT's `arf_sqrt`.
//
// With `use_kernels`, the square root is computed by `Float`'s square root kernels. They depend on
// the operand's exponent only through its parity, so they are given 0 or 1, and half of the rest of
// the exponent is added to their output exponent.
fn sqrt_helper_with(
    x: Cow<'_, BigExponentFloat>,
    prec: u64,
    rm: RoundingMode,
    use_kernels: bool,
) -> (BigExponentFloat, Ordering) {
    assert_ne!(prec, 0);
    let exponent = match &*x {
        BigExponentFloat(NaN | Infinity { sign: false } | Finite { sign: false, .. }) => {
            return (BigExponentFloat::NAN, Equal);
        }
        BigExponentFloat(Infinity { sign: true }) => return (BigExponentFloat::INFINITY, Equal),
        BigExponentFloat(Zero) => return (BigExponentFloat::ZERO, Equal),
        BigExponentFloat(Finite { exponent, .. }) => exponent,
    };
    let xs = x.significand_ref().unwrap();
    if !use_kernels {
        let bits = xs.significant_bits();
        let low = exponent - Integer::from(bits);
        let mut shift = ((prec << 1) + 4).saturating_sub(bits);
        if (&low - Integer::from(shift)).odd() {
            shift += 1;
        }
        let (q, r) = (xs << shift).sqrt_rem();
        let mut m = q << 1u32;
        if r != 0u32 {
            m.set_bit(0);
        }
        let exponent = ((low - Integer::from(shift)) >> 1u32) - Integer::ONE
            + Integer::from(m.significant_bits());
        return round_optional(
            BigExponentFloat::from_parts(true, exponent, m),
            Some((prec, rm)),
        );
    }
    let x_prec = kernel_prec(xs, Some(prec));
    let parity = i32::from(exponent.odd());
    let half_exponent = (exponent - Integer::from(parity)) >> 1u32;
    let (sqrt, exp, o) = match x {
        Cow::Owned(x) => {
            let mut xs = x.into_significand().unwrap();
            let (exp, o) = sqrt_float_significand_in_place(&mut xs, parity, x_prec, prec, rm);
            (xs, exp, o)
        }
        Cow::Borrowed(x) => {
            sqrt_float_significand_ref(x.significand_ref().unwrap(), parity, x_prec, prec, rm)
        }
    };
    (
        BigExponentFloat(Finite {
            sign: true,
            exponent: half_exponent + Integer::from(exp),
            significand: finish_significand(sqrt),
        }),
        o,
    )
}

impl BigExponentFloat {
    /// Takes the square root of a [`BigExponentFloat`], rounding the result to the specified
    /// precision and with the specified rounding mode. The [`BigExponentFloat`] is taken by value.
    /// An [`Ordering`] is also returned, indicating whether the rounded square root is less than,
    /// equal to, or greater than the exact square root. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \sqrt{x}+\varepsilon.
    /// $$
    /// - If $\sqrt{x}$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\sqrt{x}$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 \sqrt{x}\rfloor-p+1}$.
    /// - If $\sqrt{x}$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 \sqrt{x}\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=f(-\infty,p,m)=\text{NaN}$
    /// - $f(x,p,m)=\text{NaN}$ if $x<0$
    /// - $f(\infty,p,m)=\infty$
    /// - $f(0,p,m)=0$
    ///
    /// If you know you'll be using `Nearest`, consider using [`BigExponentFloat::sqrt_prec`]
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the square root cannot be represented
    /// exactly with `prec` bits.
    ///
    /// This is FLINT's `arf_sqrt`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (sqrt, o) = BigExponentFloat::from(PI).sqrt_prec_round(5, Floor);
    /// assert_eq!(sqrt.to_string(), "1.8");
    /// assert_eq!(o, Less);
    ///
    /// let (sqrt, o) = BigExponentFloat::from(PI).sqrt_prec_round(5, Ceiling);
    /// assert_eq!(sqrt.to_string(), "1.81");
    /// assert_eq!(o, Greater);
    ///
    /// let (sqrt, o) = BigExponentFloat::from(PI).sqrt_prec_round(5, Nearest);
    /// assert_eq!(sqrt.to_string(), "1.8");
    /// assert_eq!(o, Less);
    ///
    /// let (sqrt, o) = BigExponentFloat::from(PI).sqrt_prec_round(20, Floor);
    /// assert_eq!(sqrt.to_string(), "1.772453");
    /// assert_eq!(o, Less);
    ///
    /// let (sqrt, o) = BigExponentFloat::from(PI).sqrt_prec_round(20, Ceiling);
    /// assert_eq!(sqrt.to_string(), "1.7724552");
    /// assert_eq!(o, Greater);
    ///
    /// let (sqrt, o) = BigExponentFloat::from(PI).sqrt_prec_round(20, Nearest);
    /// assert_eq!(sqrt.to_string(), "1.772453");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn sqrt_prec_round(self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        sqrt_helper(Cow::Owned(self), prec, rm)
    }

    /// Takes the square root of a [`BigExponentFloat`], rounding the result to the specified
    /// precision and with the specified rounding mode. The [`BigExponentFloat`] is taken by
    /// reference. An [`Ordering`] is also returned, indicating whether the rounded square root is
    /// less than, equal to, or greater than the exact square root. Although `NaN`s are not
    /// comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \sqrt{x}+\varepsilon.
    /// $$
    /// - If $\sqrt{x}$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\sqrt{x}$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 \sqrt{x}\rfloor-p+1}$.
    /// - If $\sqrt{x}$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 \sqrt{x}\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=f(-\infty,p,m)=\text{NaN}$
    /// - $f(x,p,m)=\text{NaN}$ if $x<0$
    /// - $f(\infty,p,m)=\infty$
    /// - $f(0,p,m)=0$
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the square root cannot be represented
    /// exactly with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (sqrt, o) = (&BigExponentFloat::from(PI)).sqrt_prec_round_ref(5, Floor);
    /// assert_eq!(sqrt.to_string(), "1.8");
    /// assert_eq!(o, Less);
    ///
    /// let (sqrt, o) = (&BigExponentFloat::from(PI)).sqrt_prec_round_ref(5, Ceiling);
    /// assert_eq!(sqrt.to_string(), "1.81");
    /// assert_eq!(o, Greater);
    ///
    /// let (sqrt, o) = (&BigExponentFloat::from(PI)).sqrt_prec_round_ref(5, Nearest);
    /// assert_eq!(sqrt.to_string(), "1.8");
    /// assert_eq!(o, Less);
    ///
    /// let (sqrt, o) = (&BigExponentFloat::from(PI)).sqrt_prec_round_ref(20, Floor);
    /// assert_eq!(sqrt.to_string(), "1.772453");
    /// assert_eq!(o, Less);
    ///
    /// let (sqrt, o) = (&BigExponentFloat::from(PI)).sqrt_prec_round_ref(20, Ceiling);
    /// assert_eq!(sqrt.to_string(), "1.7724552");
    /// assert_eq!(o, Greater);
    ///
    /// let (sqrt, o) = (&BigExponentFloat::from(PI)).sqrt_prec_round_ref(20, Nearest);
    /// assert_eq!(sqrt.to_string(), "1.772453");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn sqrt_prec_round_ref(&self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        sqrt_helper(Cow::Borrowed(self), prec, rm)
    }

    /// Takes the square root of a [`BigExponentFloat`] in place, rounding the result to the
    /// specified precision and with the specified rounding mode. An [`Ordering`] is returned,
    /// indicating whether the rounded square root is less than, equal to, or greater than the exact
    /// square root. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this
    /// function sets the [`BigExponentFloat`] to `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets \sqrt{x}+\varepsilon.
    /// $$
    /// - If $\sqrt{x}$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\sqrt{x}$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 \sqrt{x}\rfloor-p+1}$.
    /// - If $\sqrt{x}$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 \sqrt{x}\rfloor-p}$.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the square root cannot be represented
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
    /// assert_eq!(x.sqrt_prec_round_assign(5, Floor), Less);
    /// assert_eq!(x.to_string(), "1.8");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(x.sqrt_prec_round_assign(5, Ceiling), Greater);
    /// assert_eq!(x.to_string(), "1.81");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(x.sqrt_prec_round_assign(5, Nearest), Less);
    /// assert_eq!(x.to_string(), "1.8");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(x.sqrt_prec_round_assign(20, Floor), Less);
    /// assert_eq!(x.to_string(), "1.772453");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(x.sqrt_prec_round_assign(20, Ceiling), Greater);
    /// assert_eq!(x.to_string(), "1.7724552");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(x.sqrt_prec_round_assign(20, Nearest), Less);
    /// assert_eq!(x.to_string(), "1.772453");
    /// ```
    #[inline]
    pub fn sqrt_prec_round_assign(&mut self, prec: u64, rm: RoundingMode) -> Ordering {
        let x = core::mem::replace(self, Self::ZERO);
        let (result, o) = sqrt_helper(Cow::Owned(x), prec, rm);
        *self = result;
        o
    }

    /// Takes the square root of a [`BigExponentFloat`], rounding the result to the nearest value of
    /// the specified precision. The [`BigExponentFloat`] is taken by value. An [`Ordering`] is also
    /// returned, indicating whether the rounded square root is less than, equal to, or greater than
    /// the exact square root. Although `NaN`s are not comparable to any [`BigExponentFloat`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,p) = \sqrt{x}+\varepsilon.
    /// $$
    /// - If $\sqrt{x}$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\sqrt{x}$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   \sqrt{x}\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p)=f(-\infty,p)=\text{NaN}$
    /// - $f(x,p)=\text{NaN}$ if $x<0$
    /// - $f(\infty,p)=\infty$
    /// - $f(0,p)=0$
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`BigExponentFloat::sqrt_prec_round`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `self.significant_bits()`.
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
    /// let (sqrt, o) = BigExponentFloat::from(PI).sqrt_prec(5);
    /// assert_eq!(sqrt.to_string(), "1.8");
    /// assert_eq!(o, Less);
    ///
    /// let (sqrt, o) = BigExponentFloat::from(PI).sqrt_prec(20);
    /// assert_eq!(sqrt.to_string(), "1.772453");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn sqrt_prec(self, prec: u64) -> (Self, Ordering) {
        sqrt_helper(Cow::Owned(self), prec, Nearest)
    }

    /// Takes the square root of a [`BigExponentFloat`], rounding the result to the nearest value of
    /// the specified precision. The [`BigExponentFloat`] is taken by reference. An [`Ordering`] is
    /// also returned, indicating whether the rounded square root is less than, equal to, or greater
    /// than the exact square root. Although `NaN`s are not comparable to any [`BigExponentFloat`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,p) = \sqrt{x}+\varepsilon.
    /// $$
    /// - If $\sqrt{x}$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\sqrt{x}$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   \sqrt{x}\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p)=f(-\infty,p)=\text{NaN}$
    /// - $f(x,p)=\text{NaN}$ if $x<0$
    /// - $f(\infty,p)=\infty$
    /// - $f(0,p)=0$
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `self.significant_bits()`.
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
    /// let (sqrt, o) = (&BigExponentFloat::from(PI)).sqrt_prec_ref(5);
    /// assert_eq!(sqrt.to_string(), "1.8");
    /// assert_eq!(o, Less);
    ///
    /// let (sqrt, o) = (&BigExponentFloat::from(PI)).sqrt_prec_ref(20);
    /// assert_eq!(sqrt.to_string(), "1.772453");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn sqrt_prec_ref(&self, prec: u64) -> (Self, Ordering) {
        sqrt_helper(Cow::Borrowed(self), prec, Nearest)
    }

    /// Takes the square root of a [`BigExponentFloat`] in place, rounding the result to the nearest
    /// value of the specified precision. An [`Ordering`] is returned, indicating whether the
    /// rounded square root is less than, equal to, or greater than the exact square root. Although
    /// `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function sets the
    /// [`BigExponentFloat`] to `NaN` it also returns `Equal`.
    ///
    /// $$
    /// x \gets \sqrt{x}+\varepsilon.
    /// $$
    /// - If $\sqrt{x}$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\sqrt{x}$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   \sqrt{x}\rfloor-p}$.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `self.significant_bits()`.
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
    /// assert_eq!(x.sqrt_prec_assign(5), Less);
    /// assert_eq!(x.to_string(), "1.8");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(x.sqrt_prec_assign(20), Less);
    /// assert_eq!(x.to_string(), "1.772453");
    /// ```
    #[inline]
    pub fn sqrt_prec_assign(&mut self, prec: u64) -> Ordering {
        let x = core::mem::replace(self, Self::ZERO);
        let (result, o) = sqrt_helper(Cow::Owned(x), prec, Nearest);
        *self = result;
        o
    }
}

impl BigExponentFloat {
    /// Computes the square root of a [`Rational`], returning a [`BigExponentFloat`], rounding the
    /// result to the specified precision and with the specified rounding mode. The [`Rational`] is
    /// taken by value. An [`Ordering`] is also returned, indicating whether the rounded square root
    /// is less than, equal to, or greater than the exact square root. Although `NaN`s are not
    /// comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \sqrt{x}+\varepsilon.
    /// $$
    /// - If $\sqrt{x}$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\sqrt{x}$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 \sqrt{x}\rfloor-p+1}$.
    /// - If $\sqrt{x}$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 \sqrt{x}\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,p,m)=\text{NaN}$ if $x<0$
    /// - $f(0,p,m)=0$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::sqrt_rational_prec_round), this function
    /// never overflows or underflows.
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`BigExponentFloat::sqrt_rational_prec`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `x.significant_bits()`.
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
    /// let (sqrt, o) =
    ///     BigExponentFloat::sqrt_rational_prec_round(Rational::from_unsigneds(3u8, 5), 5, Floor);
    /// assert_eq!(sqrt.to_string(), "0.75");
    /// assert_eq!(o, Less);
    ///
    /// let (sqrt, o) = BigExponentFloat::sqrt_rational_prec_round(
    ///     Rational::from_unsigneds(3u8, 5),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(sqrt.to_string(), "0.781");
    /// assert_eq!(o, Greater);
    ///
    /// let (sqrt, o) = BigExponentFloat::sqrt_rational_prec_round(
    ///     Rational::from_unsigneds(3u8, 5),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(sqrt.to_string(), "0.781");
    /// assert_eq!(o, Greater);
    ///
    /// let (sqrt, o) =
    ///     BigExponentFloat::sqrt_rational_prec_round(Rational::from_unsigneds(3u8, 5), 20, Floor);
    /// assert_eq!(sqrt.to_string(), "0.77459621");
    /// assert_eq!(o, Less);
    ///
    /// let (sqrt, o) = BigExponentFloat::sqrt_rational_prec_round(
    ///     Rational::from_unsigneds(3u8, 5),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(sqrt.to_string(), "0.774597");
    /// assert_eq!(o, Greater);
    ///
    /// let (sqrt, o) = BigExponentFloat::sqrt_rational_prec_round(
    ///     Rational::from_unsigneds(3u8, 5),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(sqrt.to_string(), "0.77459621");
    /// assert_eq!(o, Less);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn sqrt_rational_prec_round(x: Rational, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        root_rational_helper(&x, 2, prec, rm)
    }

    /// Computes the square root of a [`Rational`], returning a [`BigExponentFloat`], rounding the
    /// result to the specified precision and with the specified rounding mode. The [`Rational`] is
    /// taken by reference. An [`Ordering`] is also returned, indicating whether the rounded square
    /// root is less than, equal to, or greater than the exact square root. Although `NaN`s are not
    /// comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \sqrt{x}+\varepsilon.
    /// $$
    /// - If $\sqrt{x}$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\sqrt{x}$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 \sqrt{x}\rfloor-p+1}$.
    /// - If $\sqrt{x}$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 \sqrt{x}\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,p,m)=\text{NaN}$ if $x<0$
    /// - $f(0,p,m)=0$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::sqrt_rational_prec_round), this function
    /// never overflows or underflows.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `x.significant_bits()`.
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
    /// let (sqrt, o) = BigExponentFloat::sqrt_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(3u8, 5),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(sqrt.to_string(), "0.75");
    /// assert_eq!(o, Less);
    ///
    /// let (sqrt, o) = BigExponentFloat::sqrt_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(3u8, 5),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(sqrt.to_string(), "0.781");
    /// assert_eq!(o, Greater);
    ///
    /// let (sqrt, o) = BigExponentFloat::sqrt_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(3u8, 5),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(sqrt.to_string(), "0.781");
    /// assert_eq!(o, Greater);
    ///
    /// let (sqrt, o) = BigExponentFloat::sqrt_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(3u8, 5),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(sqrt.to_string(), "0.77459621");
    /// assert_eq!(o, Less);
    ///
    /// let (sqrt, o) = BigExponentFloat::sqrt_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(3u8, 5),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(sqrt.to_string(), "0.774597");
    /// assert_eq!(o, Greater);
    ///
    /// let (sqrt, o) = BigExponentFloat::sqrt_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(3u8, 5),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(sqrt.to_string(), "0.77459621");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn sqrt_rational_prec_round_ref(
        x: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        root_rational_helper(x, 2, prec, rm)
    }

    /// Computes the square root of a [`Rational`], returning a [`BigExponentFloat`], rounding the
    /// result to the nearest value of the specified precision. The [`Rational`] is taken by value.
    /// An [`Ordering`] is also returned, indicating whether the rounded square root is less than,
    /// equal to, or greater than the exact square root. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,p) = \sqrt{x}+\varepsilon.
    /// $$
    /// - If $\sqrt{x}$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\sqrt{x}$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   \sqrt{x}\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,p)=\text{NaN}$ if $x<0$
    /// - $f(0,p)=0$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::sqrt_rational_prec), this function never
    /// overflows or underflows.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`BigExponentFloat::sqrt_rational_prec_round`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `x.significant_bits()`.
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
    /// let (sqrt, o) = BigExponentFloat::sqrt_rational_prec(Rational::from_unsigneds(3u8, 5), 5);
    /// assert_eq!(sqrt.to_string(), "0.781");
    /// assert_eq!(o, Greater);
    ///
    /// let (sqrt, o) = BigExponentFloat::sqrt_rational_prec(Rational::from_unsigneds(3u8, 5), 20);
    /// assert_eq!(sqrt.to_string(), "0.77459621");
    /// assert_eq!(o, Less);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn sqrt_rational_prec(x: Rational, prec: u64) -> (Self, Ordering) {
        root_rational_helper(&x, 2, prec, Nearest)
    }

    /// Computes the square root of a [`Rational`], returning a [`BigExponentFloat`], rounding the
    /// result to the nearest value of the specified precision. The [`Rational`] is taken by
    /// reference. An [`Ordering`] is also returned, indicating whether the rounded square root is
    /// less than, equal to, or greater than the exact square root. Although `NaN`s are not
    /// comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// $$
    /// f(x,p) = \sqrt{x}+\varepsilon.
    /// $$
    /// - If $\sqrt{x}$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\sqrt{x}$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   \sqrt{x}\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,p)=\text{NaN}$ if $x<0$
    /// - $f(0,p)=0$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::sqrt_rational_prec), this function never
    /// overflows or underflows.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `x.significant_bits()`.
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
    /// let (sqrt, o) =
    ///     BigExponentFloat::sqrt_rational_prec_ref(&Rational::from_unsigneds(3u8, 5), 5);
    /// assert_eq!(sqrt.to_string(), "0.781");
    /// assert_eq!(o, Greater);
    ///
    /// let (sqrt, o) =
    ///     BigExponentFloat::sqrt_rational_prec_ref(&Rational::from_unsigneds(3u8, 5), 20);
    /// assert_eq!(sqrt.to_string(), "0.77459621");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn sqrt_rational_prec_ref(x: &Rational, prec: u64) -> (Self, Ordering) {
        root_rational_helper(x, 2, prec, Nearest)
    }
}
