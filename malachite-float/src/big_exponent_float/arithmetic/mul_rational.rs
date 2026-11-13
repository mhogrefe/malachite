// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::big_exponent_float::BigExponentFloat;
use crate::big_exponent_float::InnerBigExponentFloat::{Finite, Infinity, NaN, Zero};
use crate::big_exponent_float::arithmetic::add::{finish_significand, significand_prec};
use crate::{Float, InnerFloat};
use alloc::borrow::Cow;
use core::cmp::Ordering::{self, *};
use malachite_base::num::basic::traits::{
    Infinity as InfinityTrait, NaN as NaNTrait, NegativeInfinity, Zero as ZeroTrait,
};
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_nz::integer::Integer;
use malachite_q::Rational;

// The finite nonzero `x`, multiplied by a power of 2 so that its exponent is `exponent`, as a
// `Float` whose precision is that of x's significand.
pub(crate) fn finite_to_float(x: Cow<'_, BigExponentFloat>, exponent: i32) -> Float {
    let (sign, significand) = match x {
        Cow::Owned(BigExponentFloat(Finite {
            sign, significand, ..
        })) => (sign, significand),
        Cow::Borrowed(BigExponentFloat(Finite {
            sign, significand, ..
        })) => (*sign, significand.clone()),
        _ => unreachable!(),
    };
    Float(InnerFloat::Finite {
        sign,
        exponent,
        precision: significand_prec(&significand),
        significand,
    })
}

// The finite nonzero `Float` `x`, multiplied by a power of 2 so that its exponent is `exponent`, as
// a `BigExponentFloat`.
pub(crate) fn float_to_finite(x: Float, exponent: Integer) -> BigExponentFloat {
    let Float(InnerFloat::Finite {
        sign, significand, ..
    }) = x
    else {
        unreachable!()
    };
    BigExponentFloat(Finite {
        sign,
        exponent,
        significand: finish_significand(significand),
    })
}

// The rounding mode with which `Float`'s operations are called in place of `rm`. They panic when an
// `Exact` result is not representable, which includes results that overflow or underflow, so
// `Floor` is used instead; the `Ordering` then tells whether the result is exact.
pub(crate) const fn float_rm(rm: RoundingMode) -> RoundingMode {
    match rm {
        Exact => Floor,
        rm => rm,
    }
}

// The result of one of `Float`'s rounded operations, multiplied by 2^`shift` if `shift` is given;
// or `None` if the operation may have overflowed or underflowed: if its result is infinite, zero
// but inexact, or has `Float`'s largest or smallest exponent. A finite result with any other
// exponent is correctly rounded, as it would be with an unbounded exponent range.
pub(crate) fn from_float_result(
    (x, o): (Float, Ordering),
    shift: Option<&Integer>,
) -> Option<(BigExponentFloat, Ordering)> {
    match &x {
        Float(InnerFloat::Finite { exponent, .. })
            if *exponent > Float::MIN_EXPONENT && *exponent < Float::MAX_EXPONENT =>
        {
            let exponent = Integer::from(*exponent);
            let exponent = match shift {
                Some(shift) => exponent + shift,
                None => exponent,
            };
            Some((float_to_finite(x, exponent), o))
        }
        Float(InnerFloat::Zero { .. }) if o == Equal => Some((BigExponentFloat::ZERO, Equal)),
        _ => None,
    }
}

// The finite nonzero `x` written as m * 2^k, with m an integer: the pair (m, k).
pub(crate) fn finite_to_integer_and_shift(x: &BigExponentFloat) -> (Integer, Integer) {
    let BigExponentFloat(Finite {
        sign,
        exponent,
        significand,
    }) = x
    else {
        unreachable!()
    };
    (
        Integer::from_sign_and_abs_ref(*sign, significand),
        exponent - Integer::from(significand.significant_bits()),
    )
}

// `x` multiplied by 2^`shift`, for finite nonzero `x`.
pub(crate) fn shift_finite(mut x: BigExponentFloat, shift: &Integer) -> BigExponentFloat {
    if let BigExponentFloat(Finite { exponent, .. }) = &mut x {
        *exponent += shift;
    }
    x
}

// x * y, rounded to `prec` bits with `rm`.
pub(crate) fn mul_rational_helper(
    x: Cow<'_, BigExponentFloat>,
    y: Cow<'_, Rational>,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    mul_rational_helper_with(x, y, prec, rm, true)
}

crate_test_fn! {
    // x * y rounded to `prec` bits with `rm`, without `Float`'s kernels: with x = m * 2^k for an
    // integer m, the `Rational` m * y is rounded and then multiplied by 2^k. Used to check and
    // benchmark the kernel path.
    mul_rational_prec_round_generic(
        x: &BigExponentFloat,
        y: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (BigExponentFloat, Ordering) {
        mul_rational_helper_with(Cow::Borrowed(x), Cow::Borrowed(y), prec, rm, false)
    }
}

// The special cases are those of `BigExponentFloat` multiplication by a finite value.
//
// With `use_kernels`, x is scaled to have exponent 0 and multiplied by y with `Float`'s
// `mul_rational_prec_round`, and the scaling is undone. If that may have overflowed or underflowed,
// which can only happen if y is extremely large or small, the generic algorithm is used instead.
fn mul_rational_helper_with(
    x: Cow<'_, BigExponentFloat>,
    y: Cow<'_, Rational>,
    prec: u64,
    rm: RoundingMode,
    use_kernels: bool,
) -> (BigExponentFloat, Ordering) {
    assert_ne!(prec, 0);
    match &*x {
        BigExponentFloat(NaN) => return (BigExponentFloat::NAN, Equal),
        BigExponentFloat(Infinity { sign }) => {
            return (
                if *y == 0u32 {
                    BigExponentFloat::NAN
                } else if *sign == (*y > 0u32) {
                    BigExponentFloat::INFINITY
                } else {
                    BigExponentFloat::NEGATIVE_INFINITY
                },
                Equal,
            );
        }
        BigExponentFloat(Zero) => return (BigExponentFloat::ZERO, Equal),
        BigExponentFloat(Finite { .. }) => {}
    }
    if *y == 0u32 {
        return (BigExponentFloat::ZERO, Equal);
    }
    if !use_kernels {
        return mul_rational_finite_generic(&x, y, prec, rm);
    }
    let exponent = x.get_exponent().unwrap().clone();
    let x = finite_to_float(x, 0);
    if let Some((product, o)) = from_float_result(
        x.mul_rational_prec_round_ref_ref(&y, prec, float_rm(rm)),
        Some(&exponent),
    ) {
        assert!(
            rm != Exact || o == Equal,
            "Inexact BigExponentFloat multiplication"
        );
        return (product, o);
    }
    mul_rational_finite_generic(&float_to_finite(x, exponent), y, prec, rm)
}

// x * y for finite nonzero x and nonzero y, by the generic algorithm.
fn mul_rational_finite_generic(
    x: &BigExponentFloat,
    y: Cow<'_, Rational>,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let (m, k) = finite_to_integer_and_shift(x);
    let m = Rational::from(m);
    let product = match y {
        Cow::Owned(y) => m * y,
        Cow::Borrowed(y) => m * y,
    };
    let (product, o) = BigExponentFloat::from_rational_prec_round(product, prec, rm);
    (shift_finite(product, &k), o)
}

impl BigExponentFloat {
    /// Multiplies a [`BigExponentFloat`] by a [`Rational`], rounding the result to the specified
    /// precision and with the specified rounding mode. The [`BigExponentFloat`] and the
    /// [`Rational`] are both taken by value. An [`Ordering`] is also returned, indicating whether
    /// the rounded product is less than, equal to, or greater than the exact product. Although
    /// `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function returns a
    /// `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,p,m) = xy+\varepsilon.
    /// $$
    /// - If $xy$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy|\rfloor-p+1}$.
    /// - If $xy$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |xy|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,p,m)=f(\pm\infty,0,p,m)=\text{NaN}$
    /// - $f(\infty,x,p,m)=\infty$ if $x>0$
    /// - $f(\infty,x,p,m)=-\infty$ if $x<0$
    /// - $f(-\infty,x,p,m)=-\infty$ if $x>0$
    /// - $f(-\infty,x,p,m)=\infty$ if $x<0$
    /// - $f(0,x,p,m)=0$
    /// - $f(x,0,p,m)=0$ if $x$ is finite
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`BigExponentFloat::mul_rational_prec`] instead.
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
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the product cannot be represented
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
    /// let (result, o) = BigExponentFloat::from(PI).mul_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x1.0");
    /// assert_eq!(o, Less);
    ///
    /// let (result, o) = BigExponentFloat::from(PI).mul_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x1.1");
    /// assert_eq!(o, Greater);
    ///
    /// let (result, o) = BigExponentFloat::from(PI).mul_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x1.1");
    /// assert_eq!(o, Greater);
    ///
    /// let (result, o) = BigExponentFloat::from(PI).mul_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x1.0c152");
    /// assert_eq!(o, Less);
    ///
    /// let (result, o) = BigExponentFloat::from(PI).mul_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x1.0c154");
    /// assert_eq!(o, Greater);
    ///
    /// let (result, o) = BigExponentFloat::from(PI).mul_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x1.0c152");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn mul_rational_prec_round(
        self,
        other: Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        mul_rational_helper(Cow::Owned(self), Cow::Owned(other), prec, rm)
    }

    /// Multiplies a [`BigExponentFloat`] by a [`Rational`], rounding the result to the specified
    /// precision and with the specified rounding mode. The [`BigExponentFloat`] is taken by value
    /// and the [`Rational`] by reference. An [`Ordering`] is also returned, indicating whether the
    /// rounded product is less than, equal to, or greater than the exact product. Although `NaN`s
    /// are not comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it
    /// also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,p,m) = xy+\varepsilon.
    /// $$
    /// - If $xy$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy|\rfloor-p+1}$.
    /// - If $xy$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |xy|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,p,m)=f(\pm\infty,0,p,m)=\text{NaN}$
    /// - $f(\infty,x,p,m)=\infty$ if $x>0$
    /// - $f(\infty,x,p,m)=-\infty$ if $x<0$
    /// - $f(-\infty,x,p,m)=-\infty$ if $x>0$
    /// - $f(-\infty,x,p,m)=\infty$ if $x<0$
    /// - $f(0,x,p,m)=0$
    /// - $f(x,0,p,m)=0$ if $x$ is finite
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
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the product cannot be represented
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
    /// let (result, o) = BigExponentFloat::from(PI).mul_rational_prec_round_val_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x1.0");
    /// assert_eq!(o, Less);
    ///
    /// let (result, o) = BigExponentFloat::from(PI).mul_rational_prec_round_val_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x1.1");
    /// assert_eq!(o, Greater);
    ///
    /// let (result, o) = BigExponentFloat::from(PI).mul_rational_prec_round_val_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x1.1");
    /// assert_eq!(o, Greater);
    ///
    /// let (result, o) = BigExponentFloat::from(PI).mul_rational_prec_round_val_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x1.0c152");
    /// assert_eq!(o, Less);
    ///
    /// let (result, o) = BigExponentFloat::from(PI).mul_rational_prec_round_val_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x1.0c154");
    /// assert_eq!(o, Greater);
    ///
    /// let (result, o) = BigExponentFloat::from(PI).mul_rational_prec_round_val_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x1.0c152");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn mul_rational_prec_round_val_ref(
        self,
        other: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        mul_rational_helper(Cow::Owned(self), Cow::Borrowed(other), prec, rm)
    }

    /// Multiplies a [`BigExponentFloat`] by a [`Rational`], rounding the result to the specified
    /// precision and with the specified rounding mode. The [`BigExponentFloat`] is taken by
    /// reference and the [`Rational`] by value. An [`Ordering`] is also returned, indicating
    /// whether the rounded product is less than, equal to, or greater than the exact product.
    /// Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,p,m) = xy+\varepsilon.
    /// $$
    /// - If $xy$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy|\rfloor-p+1}$.
    /// - If $xy$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |xy|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,p,m)=f(\pm\infty,0,p,m)=\text{NaN}$
    /// - $f(\infty,x,p,m)=\infty$ if $x>0$
    /// - $f(\infty,x,p,m)=-\infty$ if $x<0$
    /// - $f(-\infty,x,p,m)=-\infty$ if $x>0$
    /// - $f(-\infty,x,p,m)=\infty$ if $x<0$
    /// - $f(0,x,p,m)=0$
    /// - $f(x,0,p,m)=0$ if $x$ is finite
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
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the product cannot be represented
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
    /// let (result, o) = (&BigExponentFloat::from(PI)).mul_rational_prec_round_ref_val(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x1.0");
    /// assert_eq!(o, Less);
    ///
    /// let (result, o) = (&BigExponentFloat::from(PI)).mul_rational_prec_round_ref_val(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x1.1");
    /// assert_eq!(o, Greater);
    ///
    /// let (result, o) = (&BigExponentFloat::from(PI)).mul_rational_prec_round_ref_val(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x1.1");
    /// assert_eq!(o, Greater);
    ///
    /// let (result, o) = (&BigExponentFloat::from(PI)).mul_rational_prec_round_ref_val(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x1.0c152");
    /// assert_eq!(o, Less);
    ///
    /// let (result, o) = (&BigExponentFloat::from(PI)).mul_rational_prec_round_ref_val(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x1.0c154");
    /// assert_eq!(o, Greater);
    ///
    /// let (result, o) = (&BigExponentFloat::from(PI)).mul_rational_prec_round_ref_val(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x1.0c152");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn mul_rational_prec_round_ref_val(
        &self,
        other: Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        mul_rational_helper(Cow::Borrowed(self), Cow::Owned(other), prec, rm)
    }

    /// Multiplies a [`BigExponentFloat`] by a [`Rational`], rounding the result to the specified
    /// precision and with the specified rounding mode. The [`BigExponentFloat`] and the
    /// [`Rational`] are both taken by reference. An [`Ordering`] is also returned, indicating
    /// whether the rounded product is less than, equal to, or greater than the exact product.
    /// Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,p,m) = xy+\varepsilon.
    /// $$
    /// - If $xy$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy|\rfloor-p+1}$.
    /// - If $xy$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |xy|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,p,m)=f(\pm\infty,0,p,m)=\text{NaN}$
    /// - $f(\infty,x,p,m)=\infty$ if $x>0$
    /// - $f(\infty,x,p,m)=-\infty$ if $x<0$
    /// - $f(-\infty,x,p,m)=-\infty$ if $x>0$
    /// - $f(-\infty,x,p,m)=\infty$ if $x<0$
    /// - $f(0,x,p,m)=0$
    /// - $f(x,0,p,m)=0$ if $x$ is finite
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
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the product cannot be represented
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
    /// let (result, o) = (&BigExponentFloat::from(PI)).mul_rational_prec_round_ref_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x1.0");
    /// assert_eq!(o, Less);
    ///
    /// let (result, o) = (&BigExponentFloat::from(PI)).mul_rational_prec_round_ref_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x1.1");
    /// assert_eq!(o, Greater);
    ///
    /// let (result, o) = (&BigExponentFloat::from(PI)).mul_rational_prec_round_ref_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x1.1");
    /// assert_eq!(o, Greater);
    ///
    /// let (result, o) = (&BigExponentFloat::from(PI)).mul_rational_prec_round_ref_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x1.0c152");
    /// assert_eq!(o, Less);
    ///
    /// let (result, o) = (&BigExponentFloat::from(PI)).mul_rational_prec_round_ref_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x1.0c154");
    /// assert_eq!(o, Greater);
    ///
    /// let (result, o) = (&BigExponentFloat::from(PI)).mul_rational_prec_round_ref_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x1.0c152");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn mul_rational_prec_round_ref_ref(
        &self,
        other: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        mul_rational_helper(Cow::Borrowed(self), Cow::Borrowed(other), prec, rm)
    }

    /// Multiplies a [`BigExponentFloat`] by a [`Rational`] in place, rounding the result to the
    /// specified precision and with the specified rounding mode. The [`Rational`] is taken by
    /// value. An [`Ordering`] is returned, indicating whether the rounded product is less than,
    /// equal to, or greater than the exact product. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function sets the [`BigExponentFloat`] to `NaN` it also
    /// returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets xy+\varepsilon.
    /// $$
    /// - If $xy$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy|\rfloor-p+1}$.
    /// - If $xy$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |xy|\rfloor-p}$.
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
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the product cannot be represented
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
    ///     x.mul_rational_prec_round_assign(Rational::from_unsigneds(1u8, 3), 5, Floor),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x1.0");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.mul_rational_prec_round_assign(Rational::from_unsigneds(1u8, 3), 5, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(format!("{x:?}"), "0x1.1");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.mul_rational_prec_round_assign(Rational::from_unsigneds(1u8, 3), 5, Nearest),
    ///     Greater
    /// );
    /// assert_eq!(format!("{x:?}"), "0x1.1");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.mul_rational_prec_round_assign(Rational::from_unsigneds(1u8, 3), 20, Floor),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x1.0c152");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.mul_rational_prec_round_assign(Rational::from_unsigneds(1u8, 3), 20, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(format!("{x:?}"), "0x1.0c154");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.mul_rational_prec_round_assign(Rational::from_unsigneds(1u8, 3), 20, Nearest),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x1.0c152");
    /// ```
    #[inline]
    pub fn mul_rational_prec_round_assign(
        &mut self,
        other: Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> Ordering {
        let x = core::mem::replace(self, Self::ZERO);
        let (result, o) = mul_rational_helper(Cow::Owned(x), Cow::Owned(other), prec, rm);
        *self = result;
        o
    }

    /// Multiplies a [`BigExponentFloat`] by a [`Rational`] in place, rounding the result to the
    /// specified precision and with the specified rounding mode. The [`Rational`] is taken by
    /// reference. An [`Ordering`] is returned, indicating whether the rounded product is less than,
    /// equal to, or greater than the exact product. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function sets the [`BigExponentFloat`] to `NaN` it also
    /// returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets xy+\varepsilon.
    /// $$
    /// - If $xy$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy|\rfloor-p+1}$.
    /// - If $xy$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |xy|\rfloor-p}$.
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
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the product cannot be represented
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
    ///     x.mul_rational_prec_round_assign_ref(&Rational::from_unsigneds(1u8, 3), 5, Floor),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x1.0");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.mul_rational_prec_round_assign_ref(&Rational::from_unsigneds(1u8, 3), 5, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(format!("{x:?}"), "0x1.1");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.mul_rational_prec_round_assign_ref(&Rational::from_unsigneds(1u8, 3), 5, Nearest),
    ///     Greater
    /// );
    /// assert_eq!(format!("{x:?}"), "0x1.1");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.mul_rational_prec_round_assign_ref(&Rational::from_unsigneds(1u8, 3), 20, Floor),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x1.0c152");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.mul_rational_prec_round_assign_ref(&Rational::from_unsigneds(1u8, 3), 20, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(format!("{x:?}"), "0x1.0c154");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.mul_rational_prec_round_assign_ref(&Rational::from_unsigneds(1u8, 3), 20, Nearest),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x1.0c152");
    /// ```
    #[inline]
    pub fn mul_rational_prec_round_assign_ref(
        &mut self,
        other: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> Ordering {
        let x = core::mem::replace(self, Self::ZERO);
        let (result, o) = mul_rational_helper(Cow::Owned(x), Cow::Borrowed(other), prec, rm);
        *self = result;
        o
    }

    /// Multiplies a [`BigExponentFloat`] by a [`Rational`], rounding the result to the nearest
    /// value of the specified precision. The [`BigExponentFloat`] and the [`Rational`] are both
    /// taken by value. An [`Ordering`] is also returned, indicating whether the rounded product is
    /// less than, equal to, or greater than the exact product. Although `NaN`s are not comparable
    /// to any [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,y,p) = xy+\varepsilon.
    /// $$
    /// - If $xy$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2 |xy|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,p)=f(\pm\infty,0,p)=\text{NaN}$
    /// - $f(\infty,x,p)=\infty$ if $x>0$
    /// - $f(\infty,x,p)=-\infty$ if $x<0$
    /// - $f(-\infty,x,p)=-\infty$ if $x>0$
    /// - $f(-\infty,x,p)=\infty$ if $x<0$
    /// - $f(0,x,p)=0$
    /// - $f(x,0,p)=0$ if $x$ is finite
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`BigExponentFloat::mul_rational_prec_round`] instead.
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
    ///     BigExponentFloat::from(PI).mul_rational_prec(Rational::from_unsigneds(1u8, 3), 5);
    /// assert_eq!(format!("{result:?}"), "0x1.1");
    /// assert_eq!(o, Greater);
    ///
    /// let (result, o) =
    ///     BigExponentFloat::from(PI).mul_rational_prec(Rational::from_unsigneds(1u8, 3), 20);
    /// assert_eq!(format!("{result:?}"), "0x1.0c152");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn mul_rational_prec(self, other: Rational, prec: u64) -> (Self, Ordering) {
        mul_rational_helper(Cow::Owned(self), Cow::Owned(other), prec, Nearest)
    }

    /// Multiplies a [`BigExponentFloat`] by a [`Rational`], rounding the result to the nearest
    /// value of the specified precision. The [`BigExponentFloat`] is taken by value and the
    /// [`Rational`] by reference. An [`Ordering`] is also returned, indicating whether the rounded
    /// product is less than, equal to, or greater than the exact product. Although `NaN`s are not
    /// comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// $$
    /// f(x,y,p) = xy+\varepsilon.
    /// $$
    /// - If $xy$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2 |xy|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,p)=f(\pm\infty,0,p)=\text{NaN}$
    /// - $f(\infty,x,p)=\infty$ if $x>0$
    /// - $f(\infty,x,p)=-\infty$ if $x<0$
    /// - $f(-\infty,x,p)=-\infty$ if $x>0$
    /// - $f(-\infty,x,p)=\infty$ if $x<0$
    /// - $f(0,x,p)=0$
    /// - $f(x,0,p)=0$ if $x$ is finite
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
    ///     .mul_rational_prec_val_ref(&Rational::from_unsigneds(1u8, 3), 5);
    /// assert_eq!(format!("{result:?}"), "0x1.1");
    /// assert_eq!(o, Greater);
    ///
    /// let (result, o) = BigExponentFloat::from(PI)
    ///     .mul_rational_prec_val_ref(&Rational::from_unsigneds(1u8, 3), 20);
    /// assert_eq!(format!("{result:?}"), "0x1.0c152");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn mul_rational_prec_val_ref(self, other: &Rational, prec: u64) -> (Self, Ordering) {
        mul_rational_helper(Cow::Owned(self), Cow::Borrowed(other), prec, Nearest)
    }

    /// Multiplies a [`BigExponentFloat`] by a [`Rational`], rounding the result to the nearest
    /// value of the specified precision. The [`BigExponentFloat`] is taken by reference and the
    /// [`Rational`] by value. An [`Ordering`] is also returned, indicating whether the rounded
    /// product is less than, equal to, or greater than the exact product. Although `NaN`s are not
    /// comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// $$
    /// f(x,y,p) = xy+\varepsilon.
    /// $$
    /// - If $xy$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2 |xy|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,p)=f(\pm\infty,0,p)=\text{NaN}$
    /// - $f(\infty,x,p)=\infty$ if $x>0$
    /// - $f(\infty,x,p)=-\infty$ if $x<0$
    /// - $f(-\infty,x,p)=-\infty$ if $x>0$
    /// - $f(-\infty,x,p)=\infty$ if $x<0$
    /// - $f(0,x,p)=0$
    /// - $f(x,0,p)=0$ if $x$ is finite
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
    ///     .mul_rational_prec_ref_val(Rational::from_unsigneds(1u8, 3), 5);
    /// assert_eq!(format!("{result:?}"), "0x1.1");
    /// assert_eq!(o, Greater);
    ///
    /// let (result, o) = (&BigExponentFloat::from(PI))
    ///     .mul_rational_prec_ref_val(Rational::from_unsigneds(1u8, 3), 20);
    /// assert_eq!(format!("{result:?}"), "0x1.0c152");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn mul_rational_prec_ref_val(&self, other: Rational, prec: u64) -> (Self, Ordering) {
        mul_rational_helper(Cow::Borrowed(self), Cow::Owned(other), prec, Nearest)
    }

    /// Multiplies a [`BigExponentFloat`] by a [`Rational`], rounding the result to the nearest
    /// value of the specified precision. The [`BigExponentFloat`] and the [`Rational`] are both
    /// taken by reference. An [`Ordering`] is also returned, indicating whether the rounded product
    /// is less than, equal to, or greater than the exact product. Although `NaN`s are not
    /// comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// $$
    /// f(x,y,p) = xy+\varepsilon.
    /// $$
    /// - If $xy$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2 |xy|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,p)=f(\pm\infty,0,p)=\text{NaN}$
    /// - $f(\infty,x,p)=\infty$ if $x>0$
    /// - $f(\infty,x,p)=-\infty$ if $x<0$
    /// - $f(-\infty,x,p)=-\infty$ if $x>0$
    /// - $f(-\infty,x,p)=\infty$ if $x<0$
    /// - $f(0,x,p)=0$
    /// - $f(x,0,p)=0$ if $x$ is finite
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
    ///     .mul_rational_prec_ref_ref(&Rational::from_unsigneds(1u8, 3), 5);
    /// assert_eq!(format!("{result:?}"), "0x1.1");
    /// assert_eq!(o, Greater);
    ///
    /// let (result, o) = (&BigExponentFloat::from(PI))
    ///     .mul_rational_prec_ref_ref(&Rational::from_unsigneds(1u8, 3), 20);
    /// assert_eq!(format!("{result:?}"), "0x1.0c152");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn mul_rational_prec_ref_ref(&self, other: &Rational, prec: u64) -> (Self, Ordering) {
        mul_rational_helper(Cow::Borrowed(self), Cow::Borrowed(other), prec, Nearest)
    }

    /// Multiplies a [`BigExponentFloat`] by a [`Rational`] in place, rounding the result to the
    /// nearest value of the specified precision. The [`Rational`] is taken by value. An
    /// [`Ordering`] is returned, indicating whether the rounded product is less than, equal to, or
    /// greater than the exact product. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function sets the [`BigExponentFloat`] to `NaN` it also
    /// returns `Equal`.
    ///
    /// $$
    /// x \gets xy+\varepsilon.
    /// $$
    /// - If $xy$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2 |xy|\rfloor-p}$.
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
    ///     x.mul_rational_prec_assign(Rational::from_unsigneds(1u8, 3), 5),
    ///     Greater
    /// );
    /// assert_eq!(format!("{x:?}"), "0x1.1");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.mul_rational_prec_assign(Rational::from_unsigneds(1u8, 3), 20),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x1.0c152");
    /// ```
    #[inline]
    pub fn mul_rational_prec_assign(&mut self, other: Rational, prec: u64) -> Ordering {
        let x = core::mem::replace(self, Self::ZERO);
        let (result, o) = mul_rational_helper(Cow::Owned(x), Cow::Owned(other), prec, Nearest);
        *self = result;
        o
    }

    /// Multiplies a [`BigExponentFloat`] by a [`Rational`] in place, rounding the result to the
    /// nearest value of the specified precision. The [`Rational`] is taken by reference. An
    /// [`Ordering`] is returned, indicating whether the rounded product is less than, equal to, or
    /// greater than the exact product. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function sets the [`BigExponentFloat`] to `NaN` it also
    /// returns `Equal`.
    ///
    /// $$
    /// x \gets xy+\varepsilon.
    /// $$
    /// - If $xy$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2 |xy|\rfloor-p}$.
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
    ///     x.mul_rational_prec_assign_ref(&Rational::from_unsigneds(1u8, 3), 5),
    ///     Greater
    /// );
    /// assert_eq!(format!("{x:?}"), "0x1.1");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.mul_rational_prec_assign_ref(&Rational::from_unsigneds(1u8, 3), 20),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x1.0c152");
    /// ```
    #[inline]
    pub fn mul_rational_prec_assign_ref(&mut self, other: &Rational, prec: u64) -> Ordering {
        let x = core::mem::replace(self, Self::ZERO);
        let (result, o) = mul_rational_helper(Cow::Owned(x), Cow::Borrowed(other), prec, Nearest);
        *self = result;
        o
    }
}
