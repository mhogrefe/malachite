// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::big_exponent_float::BigExponentFloat;
use crate::big_exponent_float::InnerBigExponentFloat::{Finite, Infinity, NaN, Zero};
use crate::big_exponent_float::arithmetic::add::{
    finish_significand, kernel_prec, significand_prec,
};
use crate::big_exponent_float::arithmetic::div::div_prec_round_generic;
use alloc::borrow::Cow;
use core::cmp::Ordering::{self, *};
use malachite_base::num::basic::traits::{NaN as NaNTrait, One, Zero as ZeroTrait};
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_nz::integer::Integer;
use malachite_nz::natural::arithmetic::float::reciprocal::{
    reciprocal_float_significand_in_place, reciprocal_float_significand_ref,
};

crate_test_fn! {
    // 1 / x rounded to `prec` bits with `rm`, without `Float`'s kernels: the generic division of 1
    // by x. Used to check and benchmark the kernel path.
    reciprocal_prec_round_generic(
        x: &BigExponentFloat,
        prec: u64,
        rm: RoundingMode,
    ) -> (BigExponentFloat, Ordering) {
        div_prec_round_generic(&BigExponentFloat::ONE, x, prec, rm)
    }
}

// 1 / x, rounded to `prec` bits with `rm`.
pub(crate) fn reciprocal_helper(
    x: Cow<'_, BigExponentFloat>,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    assert_ne!(prec, 0);
    let (sign, exponent) = match &*x {
        BigExponentFloat(NaN | Zero) => return (BigExponentFloat::NAN, Equal),
        BigExponentFloat(Infinity { .. }) => return (BigExponentFloat::ZERO, Equal),
        BigExponentFloat(Finite { sign, exponent, .. }) => (*sign, exponent),
    };
    let exponent = Integer::ONE - exponent;
    let xs = x.significand_ref().unwrap();
    if significand_prec(xs) == 1 {
        // `Float`'s reciprocation kernels need an operand that is not a power of 2. The reciprocal
        // of such an operand, 2^(e - 1) in magnitude for its exponent e, is 2^(1 - e), which has
        // the same significand.
        let significand = x.into_owned().into_significand().unwrap();
        return (
            BigExponentFloat(Finite {
                sign,
                exponent: exponent + Integer::ONE,
                significand,
            }),
            Equal,
        );
    }
    let x_prec = kernel_prec(xs, Some(prec));
    let rm = if sign { rm } else { -rm };
    let (reciprocal, exp_offset, o) = match x {
        Cow::Owned(x) => {
            let mut xs = x.into_significand().unwrap();
            let (exp_offset, o) = reciprocal_float_significand_in_place(&mut xs, x_prec, prec, rm);
            (xs, exp_offset, o)
        }
        Cow::Borrowed(x) => {
            reciprocal_float_significand_ref(x.significand_ref().unwrap(), x_prec, prec, rm)
        }
    };
    (
        BigExponentFloat(Finite {
            sign,
            exponent: exponent + Integer::from(exp_offset),
            significand: finish_significand(reciprocal),
        }),
        if sign { o } else { o.reverse() },
    )
}

impl BigExponentFloat {
    /// Takes the reciprocal of a [`BigExponentFloat`], rounding the result to the specified
    /// precision and with the specified rounding mode. The [`BigExponentFloat`] is taken by value.
    /// An [`Ordering`] is also returned, indicating whether the rounded reciprocal is less than,
    /// equal to, or greater than the exact reciprocal. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = 1/x+\varepsilon.
    /// $$
    /// - If $1/x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $1/x$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |1/x|\rfloor-p+1}$.
    /// - If $1/x$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |1/x|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=f(0,p,m)=\text{NaN}$
    /// - $f(\pm\infty,p,m)=0$
    ///
    /// If you know you'll be using `Nearest`, consider using [`BigExponentFloat::reciprocal_prec`]
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the reciprocal cannot be represented
    /// exactly with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (reciprocal, o) = BigExponentFloat::from(PI).reciprocal_prec_round(5, Floor);
    /// assert_eq!(format!("{reciprocal:?}"), "0x0.5");
    /// assert_eq!(o, Less);
    ///
    /// let (reciprocal, o) = BigExponentFloat::from(PI).reciprocal_prec_round(5, Ceiling);
    /// assert_eq!(format!("{reciprocal:?}"), "0x0.54");
    /// assert_eq!(o, Greater);
    ///
    /// let (reciprocal, o) = BigExponentFloat::from(PI).reciprocal_prec_round(5, Nearest);
    /// assert_eq!(format!("{reciprocal:?}"), "0x0.5");
    /// assert_eq!(o, Less);
    ///
    /// let (reciprocal, o) = BigExponentFloat::from(PI).reciprocal_prec_round(20, Floor);
    /// assert_eq!(format!("{reciprocal:?}"), "0x0.517cc");
    /// assert_eq!(o, Less);
    ///
    /// let (reciprocal, o) = BigExponentFloat::from(PI).reciprocal_prec_round(20, Ceiling);
    /// assert_eq!(format!("{reciprocal:?}"), "0x0.517cc8");
    /// assert_eq!(o, Greater);
    ///
    /// let (reciprocal, o) = BigExponentFloat::from(PI).reciprocal_prec_round(20, Nearest);
    /// assert_eq!(format!("{reciprocal:?}"), "0x0.517cc");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn reciprocal_prec_round(self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        reciprocal_helper(Cow::Owned(self), prec, rm)
    }

    /// Takes the reciprocal of a [`BigExponentFloat`], rounding the result to the specified
    /// precision and with the specified rounding mode. The [`BigExponentFloat`] is taken by
    /// reference. An [`Ordering`] is also returned, indicating whether the rounded reciprocal is
    /// less than, equal to, or greater than the exact reciprocal. Although `NaN`s are not
    /// comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = 1/x+\varepsilon.
    /// $$
    /// - If $1/x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $1/x$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |1/x|\rfloor-p+1}$.
    /// - If $1/x$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |1/x|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=f(0,p,m)=\text{NaN}$
    /// - $f(\pm\infty,p,m)=0$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the reciprocal cannot be represented
    /// exactly with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (reciprocal, o) = (&BigExponentFloat::from(PI)).reciprocal_prec_round_ref(5, Floor);
    /// assert_eq!(format!("{reciprocal:?}"), "0x0.5");
    /// assert_eq!(o, Less);
    ///
    /// let (reciprocal, o) = (&BigExponentFloat::from(PI)).reciprocal_prec_round_ref(5, Ceiling);
    /// assert_eq!(format!("{reciprocal:?}"), "0x0.54");
    /// assert_eq!(o, Greater);
    ///
    /// let (reciprocal, o) = (&BigExponentFloat::from(PI)).reciprocal_prec_round_ref(5, Nearest);
    /// assert_eq!(format!("{reciprocal:?}"), "0x0.5");
    /// assert_eq!(o, Less);
    ///
    /// let (reciprocal, o) = (&BigExponentFloat::from(PI)).reciprocal_prec_round_ref(20, Floor);
    /// assert_eq!(format!("{reciprocal:?}"), "0x0.517cc");
    /// assert_eq!(o, Less);
    ///
    /// let (reciprocal, o) = (&BigExponentFloat::from(PI)).reciprocal_prec_round_ref(20, Ceiling);
    /// assert_eq!(format!("{reciprocal:?}"), "0x0.517cc8");
    /// assert_eq!(o, Greater);
    ///
    /// let (reciprocal, o) = (&BigExponentFloat::from(PI)).reciprocal_prec_round_ref(20, Nearest);
    /// assert_eq!(format!("{reciprocal:?}"), "0x0.517cc");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn reciprocal_prec_round_ref(&self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        reciprocal_helper(Cow::Borrowed(self), prec, rm)
    }

    /// Takes the reciprocal of a [`BigExponentFloat`] in place, rounding the result to the
    /// specified precision and with the specified rounding mode. An [`Ordering`] is returned,
    /// indicating whether the rounded reciprocal is less than, equal to, or greater than the exact
    /// reciprocal. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this
    /// function sets the [`BigExponentFloat`] to `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets 1/x+\varepsilon.
    /// $$
    /// - If $1/x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $1/x$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |1/x|\rfloor-p+1}$.
    /// - If $1/x$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |1/x|\rfloor-p}$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the reciprocal cannot be represented
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
    /// assert_eq!(x.reciprocal_prec_round_assign(5, Floor), Less);
    /// assert_eq!(format!("{x:?}"), "0x0.5");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(x.reciprocal_prec_round_assign(5, Ceiling), Greater);
    /// assert_eq!(format!("{x:?}"), "0x0.54");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(x.reciprocal_prec_round_assign(5, Nearest), Less);
    /// assert_eq!(format!("{x:?}"), "0x0.5");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(x.reciprocal_prec_round_assign(20, Floor), Less);
    /// assert_eq!(format!("{x:?}"), "0x0.517cc");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(x.reciprocal_prec_round_assign(20, Ceiling), Greater);
    /// assert_eq!(format!("{x:?}"), "0x0.517cc8");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(x.reciprocal_prec_round_assign(20, Nearest), Less);
    /// assert_eq!(format!("{x:?}"), "0x0.517cc");
    /// ```
    #[inline]
    pub fn reciprocal_prec_round_assign(&mut self, prec: u64, rm: RoundingMode) -> Ordering {
        let x = core::mem::replace(self, Self::ZERO);
        let (result, o) = reciprocal_helper(Cow::Owned(x), prec, rm);
        *self = result;
        o
    }

    /// Takes the reciprocal of a [`BigExponentFloat`], rounding the result to the nearest value of
    /// the specified precision. The [`BigExponentFloat`] is taken by value. An [`Ordering`] is also
    /// returned, indicating whether the rounded reciprocal is less than, equal to, or greater than
    /// the exact reciprocal. Although `NaN`s are not comparable to any [`BigExponentFloat`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,p) = 1/x+\varepsilon.
    /// $$
    /// - If $1/x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $1/x$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |1/x|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p)=f(0,p)=\text{NaN}$
    /// - $f(\pm\infty,p)=0$
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`BigExponentFloat::reciprocal_prec_round`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// prec)`.
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
    /// let (reciprocal, o) = BigExponentFloat::from(PI).reciprocal_prec(5);
    /// assert_eq!(format!("{reciprocal:?}"), "0x0.5");
    /// assert_eq!(o, Less);
    ///
    /// let (reciprocal, o) = BigExponentFloat::from(PI).reciprocal_prec(20);
    /// assert_eq!(format!("{reciprocal:?}"), "0x0.517cc");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn reciprocal_prec(self, prec: u64) -> (Self, Ordering) {
        reciprocal_helper(Cow::Owned(self), prec, Nearest)
    }

    /// Takes the reciprocal of a [`BigExponentFloat`], rounding the result to the nearest value of
    /// the specified precision. The [`BigExponentFloat`] is taken by reference. An [`Ordering`] is
    /// also returned, indicating whether the rounded reciprocal is less than, equal to, or greater
    /// than the exact reciprocal. Although `NaN`s are not comparable to any [`BigExponentFloat`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,p) = 1/x+\varepsilon.
    /// $$
    /// - If $1/x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $1/x$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |1/x|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p)=f(0,p)=\text{NaN}$
    /// - $f(\pm\infty,p)=0$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// prec)`.
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
    /// let (reciprocal, o) = (&BigExponentFloat::from(PI)).reciprocal_prec_ref(5);
    /// assert_eq!(format!("{reciprocal:?}"), "0x0.5");
    /// assert_eq!(o, Less);
    ///
    /// let (reciprocal, o) = (&BigExponentFloat::from(PI)).reciprocal_prec_ref(20);
    /// assert_eq!(format!("{reciprocal:?}"), "0x0.517cc");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn reciprocal_prec_ref(&self, prec: u64) -> (Self, Ordering) {
        reciprocal_helper(Cow::Borrowed(self), prec, Nearest)
    }

    /// Takes the reciprocal of a [`BigExponentFloat`] in place, rounding the result to the nearest
    /// value of the specified precision. An [`Ordering`] is returned, indicating whether the
    /// rounded reciprocal is less than, equal to, or greater than the exact reciprocal. Although
    /// `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function sets the
    /// [`BigExponentFloat`] to `NaN` it also returns `Equal`.
    ///
    /// $$
    /// x \gets 1/x+\varepsilon.
    /// $$
    /// - If $1/x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $1/x$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |1/x|\rfloor-p}$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// prec)`.
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
    /// assert_eq!(x.reciprocal_prec_assign(5), Less);
    /// assert_eq!(format!("{x:?}"), "0x0.5");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(x.reciprocal_prec_assign(20), Less);
    /// assert_eq!(format!("{x:?}"), "0x0.517cc");
    /// ```
    #[inline]
    pub fn reciprocal_prec_assign(&mut self, prec: u64) -> Ordering {
        let x = core::mem::replace(self, Self::ZERO);
        let (result, o) = reciprocal_helper(Cow::Owned(x), prec, Nearest);
        *self = result;
        o
    }
}
