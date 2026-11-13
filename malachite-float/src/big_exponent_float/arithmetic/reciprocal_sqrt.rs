// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::big_exponent_float::BigExponentFloat;
use crate::big_exponent_float::InnerBigExponentFloat::{Finite, Infinity, NaN, Zero};
use crate::big_exponent_float::arithmetic::add::round_optional;
use crate::big_exponent_float::arithmetic::mul_rational::{
    finite_to_float, finite_to_integer_and_shift, float_rm, from_float_result,
};
use alloc::borrow::Cow;
use core::cmp::Ordering::{self, *};
use malachite_base::num::arithmetic::traits::{DivMod, Parity, PowerOf2, SqrtRem, UnsignedAbs};
use malachite_base::num::basic::traits::{
    Infinity as InfinityTrait, NaN as NaNTrait, One, Zero as ZeroTrait,
};
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;

// 1 / sqrt(x), rounded to `prec` bits with `rm`.
pub(crate) fn reciprocal_sqrt_helper(
    x: Cow<'_, BigExponentFloat>,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    reciprocal_sqrt_helper_with(x, prec, rm, true)
}

crate_test_fn! {
    // 1 / sqrt(x) rounded to `prec` bits with `rm`, without `Float`'s kernels: with x = m * 2^k for
    // an integer m and an even k, the integer square root of floor(2^(2s) / m) for a large enough
    // s, and a sticky bit recording whether it is exact, are formed with `Natural` arithmetic and
    // then rounded. Used to check and benchmark the kernel path.
    reciprocal_sqrt_prec_round_generic(
        x: &BigExponentFloat,
        prec: u64,
        rm: RoundingMode,
    ) -> (BigExponentFloat, Ordering) {
        reciprocal_sqrt_helper_with(Cow::Borrowed(x), prec, rm, false)
    }
}

// The special cases are those of FLINT's `arf_rsqrt`.
//
// With `use_kernels`, x is scaled by an even power of 2 so that its exponent is 0 or 1, its
// reciprocal square root is computed with `Float`'s `reciprocal_sqrt_prec_round`, and the scaling
// is undone. The scaled result is between 2^(-1/2) and 2^(1/2), far from overflow and underflow.
fn reciprocal_sqrt_helper_with(
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
        BigExponentFloat(Infinity { sign: true }) => return (BigExponentFloat::ZERO, Equal),
        BigExponentFloat(Zero) => return (BigExponentFloat::INFINITY, Equal),
        BigExponentFloat(Finite { exponent, .. }) => exponent,
    };
    if !use_kernels {
        return reciprocal_sqrt_finite_generic(&x, prec, rm);
    }
    let parity = i32::from(exponent.odd());
    let shift = -((exponent - Integer::from(parity)) >> 1u32);
    let (y, o) = from_float_result(
        finite_to_float(x, parity).reciprocal_sqrt_prec_round(prec, float_rm(rm)),
        Some(&shift),
    )
    .unwrap();
    assert!(
        rm != Exact || o == Equal,
        "Inexact BigExponentFloat reciprocal square root"
    );
    (y, o)
}

// 1 / sqrt(x) for finite positive x, by the generic algorithm.
fn reciprocal_sqrt_finite_generic(
    x: &BigExponentFloat,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let (m, k) = finite_to_integer_and_shift(x);
    let mut m = m.unsigned_abs();
    let mut k = k;
    if k.odd() {
        m <<= 1u32;
        k -= Integer::ONE;
    }
    // 1 / sqrt(m * 2^k) = 2^(-k/2) / sqrt(m). With s = prec + 3 + ceil(bits(m) / 2), sqrt(2^(2s) /
    // m) >= 2^(prec + 2), so its integer part has at least prec + 3 bits.
    let s = prec + 3 + m.significant_bits().div_ceil(2);
    let (q, r) = Natural::power_of_2(s << 1).div_mod(&m);
    let (root, root_r) = q.sqrt_rem();
    let mut root = root << 1u32;
    if r != 0u32 || root_r != 0u32 {
        root += Natural::ONE;
    }
    // root / 2 ~ 2^s / sqrt(m), so the result is root * 2^(-s - 1 - k/2).
    let low = -(k >> 1u32) - Integer::from(s) - Integer::ONE;
    let exponent = low + Integer::from(root.significant_bits());
    round_optional(
        BigExponentFloat::from_parts(true, exponent, root),
        Some((prec, rm)),
    )
}

impl BigExponentFloat {
    /// Computes the reciprocal of the square root of a [`BigExponentFloat`], rounding the result to
    /// the specified precision and with the specified rounding mode. The [`BigExponentFloat`] is
    /// taken by value. An [`Ordering`] is also returned, indicating whether the rounded reciprocal
    /// square root is less than, equal to, or greater than the exact reciprocal square root.
    /// Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = 1/\sqrt{x}+\varepsilon.
    /// $$
    /// - If $1/\sqrt{x}$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $1/\sqrt{x}$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 1/\sqrt{x}\rfloor-p+1}$.
    /// - If $1/\sqrt{x}$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 1/\sqrt{x}\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=f(-\infty,p,m)=\text{NaN}$
    /// - $f(x,p,m)=\text{NaN}$ if $x<0$
    /// - $f(\infty,p,m)=0$
    /// - $f(0,p,m)=\infty$
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`BigExponentFloat::reciprocal_sqrt_prec`] instead.
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
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the reciprocal square root cannot be
    /// represented exactly with `prec` bits.
    ///
    /// This is FLINT's `arf_rsqrt`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (rsqrt, o) = BigExponentFloat::from(PI).reciprocal_sqrt_prec_round(5, Floor);
    /// assert_eq!(format!("{rsqrt:?}"), "0x0.9");
    /// assert_eq!(o, Less);
    ///
    /// let (rsqrt, o) = BigExponentFloat::from(PI).reciprocal_sqrt_prec_round(5, Ceiling);
    /// assert_eq!(format!("{rsqrt:?}"), "0x0.98");
    /// assert_eq!(o, Greater);
    ///
    /// let (rsqrt, o) = BigExponentFloat::from(PI).reciprocal_sqrt_prec_round(5, Nearest);
    /// assert_eq!(format!("{rsqrt:?}"), "0x0.9");
    /// assert_eq!(o, Less);
    ///
    /// let (rsqrt, o) = BigExponentFloat::from(PI).reciprocal_sqrt_prec_round(20, Floor);
    /// assert_eq!(format!("{rsqrt:?}"), "0x0.906eb");
    /// assert_eq!(o, Less);
    ///
    /// let (rsqrt, o) = BigExponentFloat::from(PI).reciprocal_sqrt_prec_round(20, Ceiling);
    /// assert_eq!(format!("{rsqrt:?}"), "0x0.906ec");
    /// assert_eq!(o, Greater);
    ///
    /// let (rsqrt, o) = BigExponentFloat::from(PI).reciprocal_sqrt_prec_round(20, Nearest);
    /// assert_eq!(format!("{rsqrt:?}"), "0x0.906ec");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn reciprocal_sqrt_prec_round(self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        reciprocal_sqrt_helper(Cow::Owned(self), prec, rm)
    }

    /// Computes the reciprocal of the square root of a [`BigExponentFloat`], rounding the result to
    /// the specified precision and with the specified rounding mode. The [`BigExponentFloat`] is
    /// taken by reference. An [`Ordering`] is also returned, indicating whether the rounded
    /// reciprocal square root is less than, equal to, or greater than the exact reciprocal square
    /// root. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = 1/\sqrt{x}+\varepsilon.
    /// $$
    /// - If $1/\sqrt{x}$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $1/\sqrt{x}$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 1/\sqrt{x}\rfloor-p+1}$.
    /// - If $1/\sqrt{x}$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 1/\sqrt{x}\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=f(-\infty,p,m)=\text{NaN}$
    /// - $f(x,p,m)=\text{NaN}$ if $x<0$
    /// - $f(\infty,p,m)=0$
    /// - $f(0,p,m)=\infty$
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
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the reciprocal square root cannot be
    /// represented exactly with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (rsqrt, o) = (&BigExponentFloat::from(PI)).reciprocal_sqrt_prec_round_ref(5, Floor);
    /// assert_eq!(format!("{rsqrt:?}"), "0x0.9");
    /// assert_eq!(o, Less);
    ///
    /// let (rsqrt, o) = (&BigExponentFloat::from(PI)).reciprocal_sqrt_prec_round_ref(5, Ceiling);
    /// assert_eq!(format!("{rsqrt:?}"), "0x0.98");
    /// assert_eq!(o, Greater);
    ///
    /// let (rsqrt, o) = (&BigExponentFloat::from(PI)).reciprocal_sqrt_prec_round_ref(5, Nearest);
    /// assert_eq!(format!("{rsqrt:?}"), "0x0.9");
    /// assert_eq!(o, Less);
    ///
    /// let (rsqrt, o) = (&BigExponentFloat::from(PI)).reciprocal_sqrt_prec_round_ref(20, Floor);
    /// assert_eq!(format!("{rsqrt:?}"), "0x0.906eb");
    /// assert_eq!(o, Less);
    ///
    /// let (rsqrt, o) = (&BigExponentFloat::from(PI)).reciprocal_sqrt_prec_round_ref(20, Ceiling);
    /// assert_eq!(format!("{rsqrt:?}"), "0x0.906ec");
    /// assert_eq!(o, Greater);
    ///
    /// let (rsqrt, o) = (&BigExponentFloat::from(PI)).reciprocal_sqrt_prec_round_ref(20, Nearest);
    /// assert_eq!(format!("{rsqrt:?}"), "0x0.906ec");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn reciprocal_sqrt_prec_round_ref(&self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        reciprocal_sqrt_helper(Cow::Borrowed(self), prec, rm)
    }

    /// Computes the reciprocal of the square root of a [`BigExponentFloat`] in place, rounding the
    /// result to the specified precision and with the specified rounding mode. An [`Ordering`] is
    /// returned, indicating whether the rounded reciprocal square root is less than, equal to, or
    /// greater than the exact reciprocal square root. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function sets the [`BigExponentFloat`] to `NaN` it also
    /// returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets 1/\sqrt{x}+\varepsilon.
    /// $$
    /// - If $1/\sqrt{x}$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $1/\sqrt{x}$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 1/\sqrt{x}\rfloor-p+1}$.
    /// - If $1/\sqrt{x}$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 1/\sqrt{x}\rfloor-p}$.
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
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the reciprocal square root cannot be
    /// represented exactly with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(x.reciprocal_sqrt_prec_round_assign(5, Floor), Less);
    /// assert_eq!(format!("{x:?}"), "0x0.9");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(x.reciprocal_sqrt_prec_round_assign(5, Ceiling), Greater);
    /// assert_eq!(format!("{x:?}"), "0x0.98");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(x.reciprocal_sqrt_prec_round_assign(5, Nearest), Less);
    /// assert_eq!(format!("{x:?}"), "0x0.9");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(x.reciprocal_sqrt_prec_round_assign(20, Floor), Less);
    /// assert_eq!(format!("{x:?}"), "0x0.906eb");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(x.reciprocal_sqrt_prec_round_assign(20, Ceiling), Greater);
    /// assert_eq!(format!("{x:?}"), "0x0.906ec");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(x.reciprocal_sqrt_prec_round_assign(20, Nearest), Greater);
    /// assert_eq!(format!("{x:?}"), "0x0.906ec");
    /// ```
    #[inline]
    pub fn reciprocal_sqrt_prec_round_assign(&mut self, prec: u64, rm: RoundingMode) -> Ordering {
        let x = core::mem::replace(self, Self::ZERO);
        let (result, o) = reciprocal_sqrt_helper(Cow::Owned(x), prec, rm);
        *self = result;
        o
    }

    /// Computes the reciprocal of the square root of a [`BigExponentFloat`], rounding the result to
    /// the nearest value of the specified precision. The [`BigExponentFloat`] is taken by value. An
    /// [`Ordering`] is also returned, indicating whether the rounded reciprocal square root is less
    /// than, equal to, or greater than the exact reciprocal square root. Although `NaN`s are not
    /// comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// $$
    /// f(x,p) = 1/\sqrt{x}+\varepsilon.
    /// $$
    /// - If $1/\sqrt{x}$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $1/\sqrt{x}$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   1/\sqrt{x}\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p)=f(-\infty,p)=\text{NaN}$
    /// - $f(x,p)=\text{NaN}$ if $x<0$
    /// - $f(\infty,p)=0$
    /// - $f(0,p)=\infty$
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`BigExponentFloat::reciprocal_sqrt_prec_round`] instead.
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
    /// let (rsqrt, o) = BigExponentFloat::from(PI).reciprocal_sqrt_prec(5);
    /// assert_eq!(format!("{rsqrt:?}"), "0x0.9");
    /// assert_eq!(o, Less);
    ///
    /// let (rsqrt, o) = BigExponentFloat::from(PI).reciprocal_sqrt_prec(20);
    /// assert_eq!(format!("{rsqrt:?}"), "0x0.906ec");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn reciprocal_sqrt_prec(self, prec: u64) -> (Self, Ordering) {
        reciprocal_sqrt_helper(Cow::Owned(self), prec, Nearest)
    }

    /// Computes the reciprocal of the square root of a [`BigExponentFloat`], rounding the result to
    /// the nearest value of the specified precision. The [`BigExponentFloat`] is taken by
    /// reference. An [`Ordering`] is also returned, indicating whether the rounded reciprocal
    /// square root is less than, equal to, or greater than the exact reciprocal square root.
    /// Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,p) = 1/\sqrt{x}+\varepsilon.
    /// $$
    /// - If $1/\sqrt{x}$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $1/\sqrt{x}$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   1/\sqrt{x}\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p)=f(-\infty,p)=\text{NaN}$
    /// - $f(x,p)=\text{NaN}$ if $x<0$
    /// - $f(\infty,p)=0$
    /// - $f(0,p)=\infty$
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
    /// let (rsqrt, o) = (&BigExponentFloat::from(PI)).reciprocal_sqrt_prec_ref(5);
    /// assert_eq!(format!("{rsqrt:?}"), "0x0.9");
    /// assert_eq!(o, Less);
    ///
    /// let (rsqrt, o) = (&BigExponentFloat::from(PI)).reciprocal_sqrt_prec_ref(20);
    /// assert_eq!(format!("{rsqrt:?}"), "0x0.906ec");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn reciprocal_sqrt_prec_ref(&self, prec: u64) -> (Self, Ordering) {
        reciprocal_sqrt_helper(Cow::Borrowed(self), prec, Nearest)
    }

    /// Computes the reciprocal of the square root of a [`BigExponentFloat`] in place, rounding the
    /// result to the nearest value of the specified precision. An [`Ordering`] is returned,
    /// indicating whether the rounded reciprocal square root is less than, equal to, or greater
    /// than the exact reciprocal square root. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function sets the [`BigExponentFloat`] to `NaN` it also
    /// returns `Equal`.
    ///
    /// $$
    /// x \gets 1/\sqrt{x}+\varepsilon.
    /// $$
    /// - If $1/\sqrt{x}$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $1/\sqrt{x}$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   1/\sqrt{x}\rfloor-p}$.
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
    /// assert_eq!(x.reciprocal_sqrt_prec_assign(5), Less);
    /// assert_eq!(format!("{x:?}"), "0x0.9");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(x.reciprocal_sqrt_prec_assign(20), Greater);
    /// assert_eq!(format!("{x:?}"), "0x0.906ec");
    /// ```
    #[inline]
    pub fn reciprocal_sqrt_prec_assign(&mut self, prec: u64) -> Ordering {
        let x = core::mem::replace(self, Self::ZERO);
        let (result, o) = reciprocal_sqrt_helper(Cow::Owned(x), prec, Nearest);
        *self = result;
        o
    }
}
