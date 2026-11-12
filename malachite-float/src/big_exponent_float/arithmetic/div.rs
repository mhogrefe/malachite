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
    finish_significand, kernel_prec, round_optional, significand_prec,
};
use alloc::borrow::Cow;
use core::cmp::Ordering::{self, *};
use malachite_base::num::arithmetic::traits::DivMod;
use malachite_base::num::basic::traits::{
    Infinity as InfinityTrait, NaN as NaNTrait, NegativeInfinity, One, Zero as ZeroTrait,
};
use malachite_base::num::logic::traits::{BitAccess, SignificantBits};
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_nz::integer::Integer;
use malachite_nz::natural::arithmetic::float::div::{
    div_float_significands_in_place, div_float_significands_in_place_ref,
    div_float_significands_ref_ref, div_float_significands_ref_val,
};

// x / y, rounded to `prec` bits with `rm`.
pub(crate) fn div_helper(
    x: Cow<'_, BigExponentFloat>,
    y: Cow<'_, BigExponentFloat>,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    div_helper_with(x, y, prec, rm, true)
}

crate_test_fn! {
    // x / y rounded to `prec` bits with `rm`, without `Float`'s kernels: a quotient of the
    // significands with at least `prec + 2` bits, and a sticky bit recording whether it is exact,
    // are formed with `Natural` arithmetic and then rounded. Used to check and benchmark the kernel
    // path.
    div_prec_round_generic(
        x: &BigExponentFloat,
        y: &BigExponentFloat,
        prec: u64,
        rm: RoundingMode,
    ) -> (BigExponentFloat, Ordering) {
        div_helper_with(Cow::Borrowed(x), Cow::Borrowed(y), prec, rm, false)
    }
}

// The special cases are those of FLINT's `arf_div`. Since there is no negative zero, division by
// zero gives NaN.
//
// With `use_kernels`, the quotient is computed by `Float`'s division kernels, which need no
// exponents: the output exponent is an offset from the difference of the operands' exponents. When
// an operand is owned, the kernels may use its significand's memory.
fn div_helper_with(
    x: Cow<'_, BigExponentFloat>,
    y: Cow<'_, BigExponentFloat>,
    prec: u64,
    rm: RoundingMode,
    use_kernels: bool,
) -> (BigExponentFloat, Ordering) {
    assert_ne!(prec, 0);
    let (x_sign, x_exponent, y_sign, y_exponent) = match (&*x, &*y) {
        (BigExponentFloat(NaN), _)
        | (_, BigExponentFloat(NaN | Zero))
        | (BigExponentFloat(Infinity { .. }), BigExponentFloat(Infinity { .. })) => {
            return (BigExponentFloat::NAN, Equal);
        }
        (BigExponentFloat(Zero), _) | (_, BigExponentFloat(Infinity { .. })) => {
            return (BigExponentFloat::ZERO, Equal);
        }
        (
            BigExponentFloat(Infinity { sign: x_sign }),
            BigExponentFloat(Finite { sign: y_sign, .. }),
        ) => {
            return (
                if x_sign == y_sign {
                    BigExponentFloat::INFINITY
                } else {
                    BigExponentFloat::NEGATIVE_INFINITY
                },
                Equal,
            );
        }
        (
            BigExponentFloat(Finite {
                sign: x_sign,
                exponent: x_exponent,
                ..
            }),
            BigExponentFloat(Finite {
                sign: y_sign,
                exponent: y_exponent,
                ..
            }),
        ) => (*x_sign, x_exponent, *y_sign, y_exponent),
    };
    let sign = x_sign == y_sign;
    let exponent = x_exponent - y_exponent;
    let xs = x.significand_ref().unwrap();
    let ys = y.significand_ref().unwrap();
    if !use_kernels {
        let x_bits = xs.significant_bits();
        let y_bits = ys.significant_bits();
        let shift = (prec + 2 + y_bits).saturating_sub(x_bits);
        let (q, r) = (xs << shift).div_mod(ys);
        let mut m = q << 1u32;
        if r != 0u32 {
            m.set_bit(0);
        }
        let exponent = exponent - Integer::from(x_bits) + Integer::from(y_bits)
            - Integer::from(shift)
            - Integer::ONE
            + Integer::from(m.significant_bits());
        return round_optional(
            BigExponentFloat::from_parts(sign, exponent, m),
            Some((prec, rm)),
        );
    }
    if significand_prec(ys) == 1 {
        // `Float`'s division kernels need a divisor that is not a power of 2. Dividing by such a
        // divisor, 2^(e - 1) in magnitude for its exponent e, only changes the exponent.
        let significand = x.into_owned().into_significand().unwrap();
        return round_optional(
            BigExponentFloat(Finite {
                sign,
                exponent: exponent + Integer::ONE,
                significand,
            }),
            Some((prec, rm)),
        );
    }
    let x_prec = kernel_prec(xs, Some(prec));
    let y_prec = kernel_prec(ys, Some(prec));
    let rm = if sign { rm } else { -rm };
    let (quotient, exp_offset, o) = match (x, y) {
        (Cow::Owned(x), Cow::Owned(y)) => {
            let mut xs = x.into_significand().unwrap();
            let mut ys = y.into_significand().unwrap();
            let (exp_offset, o) =
                div_float_significands_in_place(&mut xs, x_prec, &mut ys, y_prec, prec, rm);
            (xs, exp_offset, o)
        }
        (Cow::Owned(x), Cow::Borrowed(y)) => {
            let mut xs = x.into_significand().unwrap();
            let (exp_offset, o) = div_float_significands_in_place_ref(
                &mut xs,
                x_prec,
                y.significand_ref().unwrap(),
                y_prec,
                prec,
                rm,
            );
            (xs, exp_offset, o)
        }
        (Cow::Borrowed(x), Cow::Owned(y)) => {
            let mut ys = y.into_significand().unwrap();
            div_float_significands_ref_val(
                x.significand_ref().unwrap(),
                x_prec,
                &mut ys,
                y_prec,
                prec,
                rm,
            )
        }
        (Cow::Borrowed(x), Cow::Borrowed(y)) => div_float_significands_ref_ref(
            x.significand_ref().unwrap(),
            x_prec,
            y.significand_ref().unwrap(),
            y_prec,
            prec,
            rm,
        ),
    };
    (
        BigExponentFloat(Finite {
            sign,
            exponent: exponent + Integer::from(exp_offset),
            significand: finish_significand(quotient),
        }),
        if sign { o } else { o.reverse() },
    )
}

impl BigExponentFloat {
    /// Divides a [`BigExponentFloat`] by a [`BigExponentFloat`], rounding the result to the
    /// specified precision and with the specified rounding mode. Both [`BigExponentFloat`]s are
    /// taken by value. An [`Ordering`] is also returned, indicating whether the rounded quotient is
    /// less than, equal to, or greater than the exact quotient. Although `NaN`s are not comparable
    /// to any [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,p,m) = x/y+\varepsilon.
    /// $$
    /// - If $x/y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x/y$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x/y|\rfloor-p+1}$.
    /// - If $x/y$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x/y|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,p,m)=f(x,\text{NaN},p,m)=f(x,0,p,m)=f(\pm\infty,\pm\infty,p,m)=
    ///   \text{NaN}$
    /// - $f(\infty,x,p,m)=\infty$ if $x>0$ and $x$ is finite
    /// - $f(\infty,x,p,m)=-\infty$ if $x<0$ and $x$ is finite
    /// - $f(-\infty,x,p,m)=-\infty$ if $x>0$ and $x$ is finite
    /// - $f(-\infty,x,p,m)=\infty$ if $x<0$ and $x$ is finite
    /// - $f(0,x,p,m)=0$ if $x$ is not NaN or 0
    /// - $f(x,\pm\infty,p,m)=0$ if $x$ is finite
    ///
    /// If you know you'll be using `Nearest`, consider using [`BigExponentFloat::div_prec`]
    /// instead.
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
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the quotient cannot be represented
    /// exactly with `prec` bits.
    ///
    /// This is FLINT's `arf_div`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (quotient, o) =
    ///     BigExponentFloat::from(PI).div_prec_round(BigExponentFloat::from(E), 5, Floor);
    /// assert_eq!(format!("{quotient:?}"), "0x1.2");
    /// assert_eq!(o, Less);
    ///
    /// let (quotient, o) =
    ///     BigExponentFloat::from(PI).div_prec_round(BigExponentFloat::from(E), 5, Ceiling);
    /// assert_eq!(format!("{quotient:?}"), "0x1.3");
    /// assert_eq!(o, Greater);
    ///
    /// let (quotient, o) =
    ///     BigExponentFloat::from(PI).div_prec_round(BigExponentFloat::from(E), 5, Nearest);
    /// assert_eq!(format!("{quotient:?}"), "0x1.2");
    /// assert_eq!(o, Less);
    ///
    /// let (quotient, o) =
    ///     BigExponentFloat::from(PI).div_prec_round(BigExponentFloat::from(E), 20, Floor);
    /// assert_eq!(format!("{quotient:?}"), "0x1.27dda");
    /// assert_eq!(o, Less);
    ///
    /// let (quotient, o) =
    ///     BigExponentFloat::from(PI).div_prec_round(BigExponentFloat::from(E), 20, Ceiling);
    /// assert_eq!(format!("{quotient:?}"), "0x1.27ddc");
    /// assert_eq!(o, Greater);
    ///
    /// let (quotient, o) =
    ///     BigExponentFloat::from(PI).div_prec_round(BigExponentFloat::from(E), 20, Nearest);
    /// assert_eq!(format!("{quotient:?}"), "0x1.27ddc");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn div_prec_round(self, other: Self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        div_helper(Cow::Owned(self), Cow::Owned(other), prec, rm)
    }

    /// Divides a [`BigExponentFloat`] by a [`BigExponentFloat`], rounding the result to the
    /// specified precision and with the specified rounding mode. The first [`BigExponentFloat`] is
    /// taken by value and the second by reference. An [`Ordering`] is also returned, indicating
    /// whether the rounded quotient is less than, equal to, or greater than the exact quotient.
    /// Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,p,m) = x/y+\varepsilon.
    /// $$
    /// - If $x/y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x/y$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x/y|\rfloor-p+1}$.
    /// - If $x/y$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x/y|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,p,m)=f(x,\text{NaN},p,m)=f(x,0,p,m)=f(\pm\infty,\pm\infty,p,m)=
    ///   \text{NaN}$
    /// - $f(\infty,x,p,m)=\infty$ if $x>0$ and $x$ is finite
    /// - $f(\infty,x,p,m)=-\infty$ if $x<0$ and $x$ is finite
    /// - $f(-\infty,x,p,m)=-\infty$ if $x>0$ and $x$ is finite
    /// - $f(-\infty,x,p,m)=\infty$ if $x<0$ and $x$ is finite
    /// - $f(0,x,p,m)=0$ if $x$ is not NaN or 0
    /// - $f(x,\pm\infty,p,m)=0$ if $x$ is finite
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
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the quotient cannot be represented
    /// exactly with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (quotient, o) =
    ///     BigExponentFloat::from(PI).div_prec_round_val_ref(&BigExponentFloat::from(E), 5, Floor);
    /// assert_eq!(format!("{quotient:?}"), "0x1.2");
    /// assert_eq!(o, Less);
    ///
    /// let (quotient, o) = BigExponentFloat::from(PI).div_prec_round_val_ref(
    ///     &BigExponentFloat::from(E),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{quotient:?}"), "0x1.3");
    /// assert_eq!(o, Greater);
    ///
    /// let (quotient, o) = BigExponentFloat::from(PI).div_prec_round_val_ref(
    ///     &BigExponentFloat::from(E),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{quotient:?}"), "0x1.2");
    /// assert_eq!(o, Less);
    ///
    /// let (quotient, o) = BigExponentFloat::from(PI).div_prec_round_val_ref(
    ///     &BigExponentFloat::from(E),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{quotient:?}"), "0x1.27dda");
    /// assert_eq!(o, Less);
    ///
    /// let (quotient, o) = BigExponentFloat::from(PI).div_prec_round_val_ref(
    ///     &BigExponentFloat::from(E),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{quotient:?}"), "0x1.27ddc");
    /// assert_eq!(o, Greater);
    ///
    /// let (quotient, o) = BigExponentFloat::from(PI).div_prec_round_val_ref(
    ///     &BigExponentFloat::from(E),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{quotient:?}"), "0x1.27ddc");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn div_prec_round_val_ref(
        self,
        other: &Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        div_helper(Cow::Owned(self), Cow::Borrowed(other), prec, rm)
    }

    /// Divides a [`BigExponentFloat`] by a [`BigExponentFloat`], rounding the result to the
    /// specified precision and with the specified rounding mode. The first [`BigExponentFloat`] is
    /// taken by reference and the second by value. An [`Ordering`] is also returned, indicating
    /// whether the rounded quotient is less than, equal to, or greater than the exact quotient.
    /// Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,p,m) = x/y+\varepsilon.
    /// $$
    /// - If $x/y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x/y$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x/y|\rfloor-p+1}$.
    /// - If $x/y$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x/y|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,p,m)=f(x,\text{NaN},p,m)=f(x,0,p,m)=f(\pm\infty,\pm\infty,p,m)=
    ///   \text{NaN}$
    /// - $f(\infty,x,p,m)=\infty$ if $x>0$ and $x$ is finite
    /// - $f(\infty,x,p,m)=-\infty$ if $x<0$ and $x$ is finite
    /// - $f(-\infty,x,p,m)=-\infty$ if $x>0$ and $x$ is finite
    /// - $f(-\infty,x,p,m)=\infty$ if $x<0$ and $x$ is finite
    /// - $f(0,x,p,m)=0$ if $x$ is not NaN or 0
    /// - $f(x,\pm\infty,p,m)=0$ if $x$ is finite
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
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the quotient cannot be represented
    /// exactly with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (quotient, o) = (&BigExponentFloat::from(PI)).div_prec_round_ref_val(
    ///     BigExponentFloat::from(E),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{quotient:?}"), "0x1.2");
    /// assert_eq!(o, Less);
    ///
    /// let (quotient, o) = (&BigExponentFloat::from(PI)).div_prec_round_ref_val(
    ///     BigExponentFloat::from(E),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{quotient:?}"), "0x1.3");
    /// assert_eq!(o, Greater);
    ///
    /// let (quotient, o) = (&BigExponentFloat::from(PI)).div_prec_round_ref_val(
    ///     BigExponentFloat::from(E),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{quotient:?}"), "0x1.2");
    /// assert_eq!(o, Less);
    ///
    /// let (quotient, o) = (&BigExponentFloat::from(PI)).div_prec_round_ref_val(
    ///     BigExponentFloat::from(E),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{quotient:?}"), "0x1.27dda");
    /// assert_eq!(o, Less);
    ///
    /// let (quotient, o) = (&BigExponentFloat::from(PI)).div_prec_round_ref_val(
    ///     BigExponentFloat::from(E),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{quotient:?}"), "0x1.27ddc");
    /// assert_eq!(o, Greater);
    ///
    /// let (quotient, o) = (&BigExponentFloat::from(PI)).div_prec_round_ref_val(
    ///     BigExponentFloat::from(E),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{quotient:?}"), "0x1.27ddc");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn div_prec_round_ref_val(
        &self,
        other: Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        div_helper(Cow::Borrowed(self), Cow::Owned(other), prec, rm)
    }

    /// Divides a [`BigExponentFloat`] by a [`BigExponentFloat`], rounding the result to the
    /// specified precision and with the specified rounding mode. Both [`BigExponentFloat`]s are
    /// taken by reference. An [`Ordering`] is also returned, indicating whether the rounded
    /// quotient is less than, equal to, or greater than the exact quotient. Although `NaN`s are not
    /// comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,p,m) = x/y+\varepsilon.
    /// $$
    /// - If $x/y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x/y$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x/y|\rfloor-p+1}$.
    /// - If $x/y$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x/y|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,p,m)=f(x,\text{NaN},p,m)=f(x,0,p,m)=f(\pm\infty,\pm\infty,p,m)=
    ///   \text{NaN}$
    /// - $f(\infty,x,p,m)=\infty$ if $x>0$ and $x$ is finite
    /// - $f(\infty,x,p,m)=-\infty$ if $x<0$ and $x$ is finite
    /// - $f(-\infty,x,p,m)=-\infty$ if $x>0$ and $x$ is finite
    /// - $f(-\infty,x,p,m)=\infty$ if $x<0$ and $x$ is finite
    /// - $f(0,x,p,m)=0$ if $x$ is not NaN or 0
    /// - $f(x,\pm\infty,p,m)=0$ if $x$ is finite
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
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the quotient cannot be represented
    /// exactly with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (quotient, o) = (&BigExponentFloat::from(PI)).div_prec_round_ref_ref(
    ///     &BigExponentFloat::from(E),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{quotient:?}"), "0x1.2");
    /// assert_eq!(o, Less);
    ///
    /// let (quotient, o) = (&BigExponentFloat::from(PI)).div_prec_round_ref_ref(
    ///     &BigExponentFloat::from(E),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{quotient:?}"), "0x1.3");
    /// assert_eq!(o, Greater);
    ///
    /// let (quotient, o) = (&BigExponentFloat::from(PI)).div_prec_round_ref_ref(
    ///     &BigExponentFloat::from(E),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{quotient:?}"), "0x1.2");
    /// assert_eq!(o, Less);
    ///
    /// let (quotient, o) = (&BigExponentFloat::from(PI)).div_prec_round_ref_ref(
    ///     &BigExponentFloat::from(E),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{quotient:?}"), "0x1.27dda");
    /// assert_eq!(o, Less);
    ///
    /// let (quotient, o) = (&BigExponentFloat::from(PI)).div_prec_round_ref_ref(
    ///     &BigExponentFloat::from(E),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{quotient:?}"), "0x1.27ddc");
    /// assert_eq!(o, Greater);
    ///
    /// let (quotient, o) = (&BigExponentFloat::from(PI)).div_prec_round_ref_ref(
    ///     &BigExponentFloat::from(E),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{quotient:?}"), "0x1.27ddc");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn div_prec_round_ref_ref(
        &self,
        other: &Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        div_helper(Cow::Borrowed(self), Cow::Borrowed(other), prec, rm)
    }

    /// Divides a [`BigExponentFloat`] by a [`BigExponentFloat`] in place, rounding the result to
    /// the specified precision and with the specified rounding mode. The [`BigExponentFloat`] on
    /// the right-hand side is taken by value. An [`Ordering`] is returned, indicating whether the
    /// rounded quotient is less than, equal to, or greater than the exact quotient. Although `NaN`s
    /// are not comparable to any [`BigExponentFloat`], whenever this function sets the
    /// [`BigExponentFloat`] to `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets x/y+\varepsilon.
    /// $$
    /// - If $x/y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x/y$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x/y|\rfloor-p+1}$.
    /// - If $x/y$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x/y|\rfloor-p}$.
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
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the quotient cannot be represented
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
    ///     x.div_prec_round_assign(BigExponentFloat::from(E), 5, Floor),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x1.2");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.div_prec_round_assign(BigExponentFloat::from(E), 5, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(format!("{x:?}"), "0x1.3");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.div_prec_round_assign(BigExponentFloat::from(E), 5, Nearest),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x1.2");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.div_prec_round_assign(BigExponentFloat::from(E), 20, Floor),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x1.27dda");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.div_prec_round_assign(BigExponentFloat::from(E), 20, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(format!("{x:?}"), "0x1.27ddc");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.div_prec_round_assign(BigExponentFloat::from(E), 20, Nearest),
    ///     Greater
    /// );
    /// assert_eq!(format!("{x:?}"), "0x1.27ddc");
    /// ```
    #[inline]
    pub fn div_prec_round_assign(&mut self, other: Self, prec: u64, rm: RoundingMode) -> Ordering {
        let x = core::mem::replace(self, Self::ZERO);
        let (result, o) = div_helper(Cow::Owned(x), Cow::Owned(other), prec, rm);
        *self = result;
        o
    }

    /// Divides a [`BigExponentFloat`] by a [`BigExponentFloat`] in place, rounding the result to
    /// the specified precision and with the specified rounding mode. The [`BigExponentFloat`] on
    /// the right-hand side is taken by reference. An [`Ordering`] is returned, indicating whether
    /// the rounded quotient is less than, equal to, or greater than the exact quotient. Although
    /// `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function sets the
    /// [`BigExponentFloat`] to `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets x/y+\varepsilon.
    /// $$
    /// - If $x/y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x/y$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x/y|\rfloor-p+1}$.
    /// - If $x/y$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x/y|\rfloor-p}$.
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
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the quotient cannot be represented
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
    ///     x.div_prec_round_assign_ref(&BigExponentFloat::from(E), 5, Floor),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x1.2");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.div_prec_round_assign_ref(&BigExponentFloat::from(E), 5, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(format!("{x:?}"), "0x1.3");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.div_prec_round_assign_ref(&BigExponentFloat::from(E), 5, Nearest),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x1.2");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.div_prec_round_assign_ref(&BigExponentFloat::from(E), 20, Floor),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x1.27dda");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.div_prec_round_assign_ref(&BigExponentFloat::from(E), 20, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(format!("{x:?}"), "0x1.27ddc");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.div_prec_round_assign_ref(&BigExponentFloat::from(E), 20, Nearest),
    ///     Greater
    /// );
    /// assert_eq!(format!("{x:?}"), "0x1.27ddc");
    /// ```
    #[inline]
    pub fn div_prec_round_assign_ref(
        &mut self,
        other: &Self,
        prec: u64,
        rm: RoundingMode,
    ) -> Ordering {
        let x = core::mem::replace(self, Self::ZERO);
        let (result, o) = div_helper(Cow::Owned(x), Cow::Borrowed(other), prec, rm);
        *self = result;
        o
    }

    /// Divides a [`BigExponentFloat`] by a [`BigExponentFloat`], rounding the result to the nearest
    /// value of the specified precision. Both [`BigExponentFloat`]s are taken by value. An
    /// [`Ordering`] is also returned, indicating whether the rounded quotient is less than, equal
    /// to, or greater than the exact quotient. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,y,p) = x/y+\varepsilon.
    /// $$
    /// - If $x/y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x/y$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x/y|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,p)=f(x,\text{NaN},p)=f(x,0,p)=f(\pm\infty,\pm\infty,p)=\text{NaN}$
    /// - $f(\infty,x,p)=\infty$ if $x>0$ and $x$ is finite
    /// - $f(\infty,x,p)=-\infty$ if $x<0$ and $x$ is finite
    /// - $f(-\infty,x,p)=-\infty$ if $x>0$ and $x$ is finite
    /// - $f(-\infty,x,p)=\infty$ if $x<0$ and $x$ is finite
    /// - $f(0,x,p)=0$ if $x$ is not NaN or 0
    /// - $f(x,\pm\infty,p)=0$ if $x$ is finite
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`BigExponentFloat::div_prec_round`] instead.
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
    /// use core::f64::consts::{E, PI};
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (quotient, o) = BigExponentFloat::from(PI).div_prec(BigExponentFloat::from(E), 5);
    /// assert_eq!(format!("{quotient:?}"), "0x1.2");
    /// assert_eq!(o, Less);
    ///
    /// let (quotient, o) = BigExponentFloat::from(PI).div_prec(BigExponentFloat::from(E), 20);
    /// assert_eq!(format!("{quotient:?}"), "0x1.27ddc");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn div_prec(self, other: Self, prec: u64) -> (Self, Ordering) {
        div_helper(Cow::Owned(self), Cow::Owned(other), prec, Nearest)
    }

    /// Divides a [`BigExponentFloat`] by a [`BigExponentFloat`], rounding the result to the nearest
    /// value of the specified precision. The first [`BigExponentFloat`] is taken by value and the
    /// second by reference. An [`Ordering`] is also returned, indicating whether the rounded
    /// quotient is less than, equal to, or greater than the exact quotient. Although `NaN`s are not
    /// comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// $$
    /// f(x,y,p) = x/y+\varepsilon.
    /// $$
    /// - If $x/y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x/y$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x/y|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,p)=f(x,\text{NaN},p)=f(x,0,p)=f(\pm\infty,\pm\infty,p)=\text{NaN}$
    /// - $f(\infty,x,p)=\infty$ if $x>0$ and $x$ is finite
    /// - $f(\infty,x,p)=-\infty$ if $x<0$ and $x$ is finite
    /// - $f(-\infty,x,p)=-\infty$ if $x>0$ and $x$ is finite
    /// - $f(-\infty,x,p)=\infty$ if $x<0$ and $x$ is finite
    /// - $f(0,x,p)=0$ if $x$ is not NaN or 0
    /// - $f(x,\pm\infty,p)=0$ if $x$ is finite
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
    /// use core::f64::consts::{E, PI};
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (quotient, o) =
    ///     BigExponentFloat::from(PI).div_prec_val_ref(&BigExponentFloat::from(E), 5);
    /// assert_eq!(format!("{quotient:?}"), "0x1.2");
    /// assert_eq!(o, Less);
    ///
    /// let (quotient, o) =
    ///     BigExponentFloat::from(PI).div_prec_val_ref(&BigExponentFloat::from(E), 20);
    /// assert_eq!(format!("{quotient:?}"), "0x1.27ddc");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn div_prec_val_ref(self, other: &Self, prec: u64) -> (Self, Ordering) {
        div_helper(Cow::Owned(self), Cow::Borrowed(other), prec, Nearest)
    }

    /// Divides a [`BigExponentFloat`] by a [`BigExponentFloat`], rounding the result to the nearest
    /// value of the specified precision. The first [`BigExponentFloat`] is taken by reference and
    /// the second by value. An [`Ordering`] is also returned, indicating whether the rounded
    /// quotient is less than, equal to, or greater than the exact quotient. Although `NaN`s are not
    /// comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// $$
    /// f(x,y,p) = x/y+\varepsilon.
    /// $$
    /// - If $x/y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x/y$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x/y|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,p)=f(x,\text{NaN},p)=f(x,0,p)=f(\pm\infty,\pm\infty,p)=\text{NaN}$
    /// - $f(\infty,x,p)=\infty$ if $x>0$ and $x$ is finite
    /// - $f(\infty,x,p)=-\infty$ if $x<0$ and $x$ is finite
    /// - $f(-\infty,x,p)=-\infty$ if $x>0$ and $x$ is finite
    /// - $f(-\infty,x,p)=\infty$ if $x<0$ and $x$ is finite
    /// - $f(0,x,p)=0$ if $x$ is not NaN or 0
    /// - $f(x,\pm\infty,p)=0$ if $x$ is finite
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
    /// use core::f64::consts::{E, PI};
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (quotient, o) =
    ///     (&BigExponentFloat::from(PI)).div_prec_ref_val(BigExponentFloat::from(E), 5);
    /// assert_eq!(format!("{quotient:?}"), "0x1.2");
    /// assert_eq!(o, Less);
    ///
    /// let (quotient, o) =
    ///     (&BigExponentFloat::from(PI)).div_prec_ref_val(BigExponentFloat::from(E), 20);
    /// assert_eq!(format!("{quotient:?}"), "0x1.27ddc");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn div_prec_ref_val(&self, other: Self, prec: u64) -> (Self, Ordering) {
        div_helper(Cow::Borrowed(self), Cow::Owned(other), prec, Nearest)
    }

    /// Divides a [`BigExponentFloat`] by a [`BigExponentFloat`], rounding the result to the nearest
    /// value of the specified precision. Both [`BigExponentFloat`]s are taken by reference. An
    /// [`Ordering`] is also returned, indicating whether the rounded quotient is less than, equal
    /// to, or greater than the exact quotient. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,y,p) = x/y+\varepsilon.
    /// $$
    /// - If $x/y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x/y$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x/y|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,p)=f(x,\text{NaN},p)=f(x,0,p)=f(\pm\infty,\pm\infty,p)=\text{NaN}$
    /// - $f(\infty,x,p)=\infty$ if $x>0$ and $x$ is finite
    /// - $f(\infty,x,p)=-\infty$ if $x<0$ and $x$ is finite
    /// - $f(-\infty,x,p)=-\infty$ if $x>0$ and $x$ is finite
    /// - $f(-\infty,x,p)=\infty$ if $x<0$ and $x$ is finite
    /// - $f(0,x,p)=0$ if $x$ is not NaN or 0
    /// - $f(x,\pm\infty,p)=0$ if $x$ is finite
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
    /// use core::f64::consts::{E, PI};
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (quotient, o) =
    ///     (&BigExponentFloat::from(PI)).div_prec_ref_ref(&BigExponentFloat::from(E), 5);
    /// assert_eq!(format!("{quotient:?}"), "0x1.2");
    /// assert_eq!(o, Less);
    ///
    /// let (quotient, o) =
    ///     (&BigExponentFloat::from(PI)).div_prec_ref_ref(&BigExponentFloat::from(E), 20);
    /// assert_eq!(format!("{quotient:?}"), "0x1.27ddc");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn div_prec_ref_ref(&self, other: &Self, prec: u64) -> (Self, Ordering) {
        div_helper(Cow::Borrowed(self), Cow::Borrowed(other), prec, Nearest)
    }

    /// Divides a [`BigExponentFloat`] by a [`BigExponentFloat`] in place, rounding the result to
    /// the nearest value of the specified precision. The [`BigExponentFloat`] on the right-hand
    /// side is taken by value. An [`Ordering`] is returned, indicating whether the rounded quotient
    /// is less than, equal to, or greater than the exact quotient. Although `NaN`s are not
    /// comparable to any [`BigExponentFloat`], whenever this function sets the [`BigExponentFloat`]
    /// to `NaN` it also returns `Equal`.
    ///
    /// $$
    /// x \gets x/y+\varepsilon.
    /// $$
    /// - If $x/y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x/y$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x/y|\rfloor-p}$.
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
    /// use core::f64::consts::{E, PI};
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(x.div_prec_assign(BigExponentFloat::from(E), 5), Less);
    /// assert_eq!(format!("{x:?}"), "0x1.2");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(x.div_prec_assign(BigExponentFloat::from(E), 20), Greater);
    /// assert_eq!(format!("{x:?}"), "0x1.27ddc");
    /// ```
    #[inline]
    pub fn div_prec_assign(&mut self, other: Self, prec: u64) -> Ordering {
        let x = core::mem::replace(self, Self::ZERO);
        let (result, o) = div_helper(Cow::Owned(x), Cow::Owned(other), prec, Nearest);
        *self = result;
        o
    }

    /// Divides a [`BigExponentFloat`] by a [`BigExponentFloat`] in place, rounding the result to
    /// the nearest value of the specified precision. The [`BigExponentFloat`] on the right-hand
    /// side is taken by reference. An [`Ordering`] is returned, indicating whether the rounded
    /// quotient is less than, equal to, or greater than the exact quotient. Although `NaN`s are not
    /// comparable to any [`BigExponentFloat`], whenever this function sets the [`BigExponentFloat`]
    /// to `NaN` it also returns `Equal`.
    ///
    /// $$
    /// x \gets x/y+\varepsilon.
    /// $$
    /// - If $x/y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x/y$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x/y|\rfloor-p}$.
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
    /// use core::f64::consts::{E, PI};
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(x.div_prec_assign_ref(&BigExponentFloat::from(E), 5), Less);
    /// assert_eq!(format!("{x:?}"), "0x1.2");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.div_prec_assign_ref(&BigExponentFloat::from(E), 20),
    ///     Greater
    /// );
    /// assert_eq!(format!("{x:?}"), "0x1.27ddc");
    /// ```
    #[inline]
    pub fn div_prec_assign_ref(&mut self, other: &Self, prec: u64) -> Ordering {
        let x = core::mem::replace(self, Self::ZERO);
        let (result, o) = div_helper(Cow::Owned(x), Cow::Borrowed(other), prec, Nearest);
        *self = result;
        o
    }
}
