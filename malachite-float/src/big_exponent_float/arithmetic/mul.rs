// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::big_exponent_float::BigExponentFloat;
use crate::big_exponent_float::InnerBigExponentFloat::Finite;
use crate::big_exponent_float::arithmetic::add::{
    finish_significand, infinity_sign, kernel_prec, round_optional,
};
use alloc::borrow::Cow;
use core::cmp::Ordering::{self, *};
use core::ops::{Mul, MulAssign};
use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity, Zero};
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_nz::integer::Integer;
use malachite_nz::natural::arithmetic::float::mul::{
    mul_float_significands_in_place_ref, mul_float_significands_ref_ref,
};

// x * y, rounded with `prec_rm` if it is `Some`, and exact otherwise.
pub(crate) fn mul_helper(
    x: Cow<'_, BigExponentFloat>,
    y: Cow<'_, BigExponentFloat>,
    prec_rm: Option<(u64, RoundingMode)>,
) -> (BigExponentFloat, Ordering) {
    mul_helper_with(x, y, prec_rm, true)
}

crate_test_fn! {
    // x * y rounded to `prec` bits with `rm`, without `Float`'s kernels: the exact product of the
    // significands is formed with `Natural` arithmetic and then rounded. Used to check and
    // benchmark the kernel path.
    mul_prec_round_generic(
        x: &BigExponentFloat,
        y: &BigExponentFloat,
        prec: u64,
        rm: RoundingMode,
    ) -> (BigExponentFloat, Ordering) {
        mul_helper_with(Cow::Borrowed(x), Cow::Borrowed(y), Some((prec, rm)), false)
    }
}

// With `use_kernels`, the product is computed by `Float`'s multiplication kernels, which have fast
// paths for operands and outputs of one, two, or three limbs. They need no exponents: the output
// exponent is an offset from the sum of the operands' exponents. An exact product is the kernels'
// product at the sum of the operands' precisions, which always suffices. When an operand is owned,
// its significand's memory holds the product.
fn mul_helper_with(
    x: Cow<'_, BigExponentFloat>,
    y: Cow<'_, BigExponentFloat>,
    prec_rm: Option<(u64, RoundingMode)>,
    use_kernels: bool,
) -> (BigExponentFloat, Ordering) {
    if let Some((prec, _)) = prec_rm {
        assert_ne!(prec, 0);
    }
    // Multiplication is commutative, so an owned operand can always be put first.
    let (x, y) = if matches!(x, Cow::Borrowed(_)) && matches!(y, Cow::Owned(_)) {
        (y, x)
    } else {
        (x, y)
    };
    if x.is_nan() || y.is_nan() {
        return (BigExponentFloat::NAN, Equal);
    }
    match (infinity_sign(&x), infinity_sign(&y)) {
        (Some(x_sign), Some(y_sign)) => {
            return (
                if x_sign == y_sign {
                    BigExponentFloat::INFINITY
                } else {
                    BigExponentFloat::NEGATIVE_INFINITY
                },
                Equal,
            );
        }
        (Some(x_sign), None) | (None, Some(x_sign)) => {
            let other = if x.is_infinite() { &y } else { &x };
            return (
                if other.is_zero() {
                    BigExponentFloat::NAN
                } else if x_sign == other.is_sign_positive() {
                    BigExponentFloat::INFINITY
                } else {
                    BigExponentFloat::NEGATIVE_INFINITY
                },
                Equal,
            );
        }
        (None, None) => {}
    }
    if x.is_zero() || y.is_zero() {
        return (BigExponentFloat::ZERO, Equal);
    }
    let BigExponentFloat(Finite {
        sign: y_sign,
        exponent: y_exponent,
        significand: y_significand,
    }) = &*y
    else {
        unreachable!()
    };
    if use_kernels {
        let out_prec = prec_rm.map(|(p, _)| p);
        let y_prec = kernel_prec(y_significand, out_prec);
        let kernel_rm = |sign: bool, x_prec: u64| {
            let (prec, rm) = prec_rm.unwrap_or((x_prec + y_prec, Exact));
            (prec, if sign { rm } else { -rm })
        };
        return match x {
            Cow::Owned(BigExponentFloat(Finite {
                sign: x_sign,
                exponent: x_exponent,
                significand: mut product,
            })) => {
                let sign = x_sign == *y_sign;
                let x_prec = kernel_prec(&product, out_prec);
                let (prec, rm) = kernel_rm(sign, x_prec);
                let (exp_offset, o) = mul_float_significands_in_place_ref(
                    &mut product,
                    x_prec,
                    y_significand,
                    y_prec,
                    prec,
                    rm,
                );
                (
                    BigExponentFloat(Finite {
                        sign,
                        exponent: x_exponent + y_exponent + Integer::from(exp_offset),
                        significand: finish_significand(product),
                    }),
                    if sign { o } else { o.reverse() },
                )
            }
            Cow::Borrowed(BigExponentFloat(Finite {
                sign: x_sign,
                exponent: x_exponent,
                significand: x_significand,
            })) => {
                let sign = *x_sign == *y_sign;
                let x_prec = kernel_prec(x_significand, out_prec);
                let (prec, rm) = kernel_rm(sign, x_prec);
                let (product, exp_offset, o) = mul_float_significands_ref_ref(
                    x_significand,
                    x_prec,
                    y_significand,
                    y_prec,
                    prec,
                    rm,
                );
                (
                    BigExponentFloat(Finite {
                        sign,
                        exponent: x_exponent + y_exponent + Integer::from(exp_offset),
                        significand: finish_significand(product),
                    }),
                    if sign { o } else { o.reverse() },
                )
            }
            _ => unreachable!(),
        };
    }
    let BigExponentFloat(Finite {
        sign: x_sign,
        exponent: x_exponent,
        significand: x_significand,
    }) = &*x
    else {
        unreachable!()
    };
    let low = x_exponent + y_exponent
        - Integer::from(x_significand.significant_bits())
        - Integer::from(y_significand.significant_bits());
    let m = x_significand * y_significand;
    let exponent = low + Integer::from(m.significant_bits());
    round_optional(
        BigExponentFloat::from_parts(x_sign == y_sign, exponent, m),
        prec_rm,
    )
}

impl BigExponentFloat {
    /// Multiplies two [`BigExponentFloat`]s, rounding the result to the specified precision and
    /// with the specified rounding mode. Both [`BigExponentFloat`]s are taken by value. An
    /// [`Ordering`] is also returned, indicating whether the rounded product is less than, equal
    /// to, or greater than the exact product. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
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
    /// - $f(\text{NaN},x,p,m)=f(x,\text{NaN},p,m)=f(\pm\infty,0,p,m)=f(0,\pm\infty,p,m)=
    ///   \text{NaN}$
    /// - $f(\infty,x,p,m)=f(x,\infty,p,m)=\infty$ if $x>0$
    /// - $f(\infty,x,p,m)=f(x,\infty,p,m)=-\infty$ if $x<0$
    /// - $f(-\infty,x,p,m)=f(x,-\infty,p,m)=-\infty$ if $x>0$
    /// - $f(-\infty,x,p,m)=f(x,-\infty,p,m)=\infty$ if $x<0$
    /// - $f(0,x,p,m)=f(x,0,p,m)=0$ if $x$ is finite
    ///
    /// If you know you'll be using `Nearest`, consider using [`BigExponentFloat::mul_prec`]
    /// instead. If you want the exact product, consider using `*` instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n + m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `max(self.significant_bits(), other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but `prec` is too small for an exact
    /// multiplication.
    ///
    /// This is FLINT's `arf_mul`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (product, o) =
    ///     BigExponentFloat::from(PI).mul_prec_round(BigExponentFloat::from(E), 5, Floor);
    /// assert_eq!(format!("{product:?}"), "0x8.8");
    /// assert_eq!(o, Less);
    ///
    /// let (product, o) =
    ///     BigExponentFloat::from(PI).mul_prec_round(BigExponentFloat::from(E), 5, Ceiling);
    /// assert_eq!(format!("{product:?}"), "0x9.0");
    /// assert_eq!(o, Greater);
    ///
    /// let (product, o) =
    ///     BigExponentFloat::from(PI).mul_prec_round(BigExponentFloat::from(E), 5, Nearest);
    /// assert_eq!(format!("{product:?}"), "0x8.8");
    /// assert_eq!(o, Less);
    ///
    /// let (product, o) =
    ///     BigExponentFloat::from(PI).mul_prec_round(BigExponentFloat::from(E), 20, Floor);
    /// assert_eq!(format!("{product:?}"), "0x8.8a2c");
    /// assert_eq!(o, Less);
    ///
    /// let (product, o) =
    ///     BigExponentFloat::from(PI).mul_prec_round(BigExponentFloat::from(E), 20, Ceiling);
    /// assert_eq!(format!("{product:?}"), "0x8.8a2d");
    /// assert_eq!(o, Greater);
    ///
    /// let (product, o) =
    ///     BigExponentFloat::from(PI).mul_prec_round(BigExponentFloat::from(E), 20, Nearest);
    /// assert_eq!(format!("{product:?}"), "0x8.8a2c");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn mul_prec_round(self, other: Self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        mul_helper(Cow::Owned(self), Cow::Owned(other), Some((prec, rm)))
    }

    /// Multiplies two [`BigExponentFloat`]s, rounding the result to the specified precision and
    /// with the specified rounding mode. The first [`BigExponentFloat`] is taken by value and the
    /// second by reference. An [`Ordering`] is also returned, indicating whether the rounded
    /// product is less than, equal to, or greater than the exact product. Although `NaN`s are not
    /// comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
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
    /// - $f(\text{NaN},x,p,m)=f(x,\text{NaN},p,m)=f(\pm\infty,0,p,m)=f(0,\pm\infty,p,m)=
    ///   \text{NaN}$
    /// - $f(\infty,x,p,m)=f(x,\infty,p,m)=\infty$ if $x>0$
    /// - $f(\infty,x,p,m)=f(x,\infty,p,m)=-\infty$ if $x<0$
    /// - $f(-\infty,x,p,m)=f(x,-\infty,p,m)=-\infty$ if $x>0$
    /// - $f(-\infty,x,p,m)=f(x,-\infty,p,m)=\infty$ if $x<0$
    /// - $f(0,x,p,m)=f(x,0,p,m)=0$ if $x$ is finite
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n + m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `max(self.significant_bits(), other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but `prec` is too small for an exact
    /// multiplication.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (product, o) =
    ///     BigExponentFloat::from(PI).mul_prec_round_val_ref(&BigExponentFloat::from(E), 5, Floor);
    /// assert_eq!(format!("{product:?}"), "0x8.8");
    /// assert_eq!(o, Less);
    ///
    /// let (product, o) = BigExponentFloat::from(PI).mul_prec_round_val_ref(
    ///     &BigExponentFloat::from(E),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{product:?}"), "0x9.0");
    /// assert_eq!(o, Greater);
    ///
    /// let (product, o) = BigExponentFloat::from(PI).mul_prec_round_val_ref(
    ///     &BigExponentFloat::from(E),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{product:?}"), "0x8.8");
    /// assert_eq!(o, Less);
    ///
    /// let (product, o) = BigExponentFloat::from(PI).mul_prec_round_val_ref(
    ///     &BigExponentFloat::from(E),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{product:?}"), "0x8.8a2c");
    /// assert_eq!(o, Less);
    ///
    /// let (product, o) = BigExponentFloat::from(PI).mul_prec_round_val_ref(
    ///     &BigExponentFloat::from(E),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{product:?}"), "0x8.8a2d");
    /// assert_eq!(o, Greater);
    ///
    /// let (product, o) = BigExponentFloat::from(PI).mul_prec_round_val_ref(
    ///     &BigExponentFloat::from(E),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{product:?}"), "0x8.8a2c");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn mul_prec_round_val_ref(
        self,
        other: &Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        mul_helper(Cow::Owned(self), Cow::Borrowed(other), Some((prec, rm)))
    }

    /// Multiplies two [`BigExponentFloat`]s, rounding the result to the specified precision and
    /// with the specified rounding mode. The first [`BigExponentFloat`] is taken by reference and
    /// the second by value. An [`Ordering`] is also returned, indicating whether the rounded
    /// product is less than, equal to, or greater than the exact product. Although `NaN`s are not
    /// comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
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
    /// - $f(\text{NaN},x,p,m)=f(x,\text{NaN},p,m)=f(\pm\infty,0,p,m)=f(0,\pm\infty,p,m)=
    ///   \text{NaN}$
    /// - $f(\infty,x,p,m)=f(x,\infty,p,m)=\infty$ if $x>0$
    /// - $f(\infty,x,p,m)=f(x,\infty,p,m)=-\infty$ if $x<0$
    /// - $f(-\infty,x,p,m)=f(x,-\infty,p,m)=-\infty$ if $x>0$
    /// - $f(-\infty,x,p,m)=f(x,-\infty,p,m)=\infty$ if $x<0$
    /// - $f(0,x,p,m)=f(x,0,p,m)=0$ if $x$ is finite
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n + m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `max(self.significant_bits(), other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but `prec` is too small for an exact
    /// multiplication.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (product, o) = (&BigExponentFloat::from(PI)).mul_prec_round_ref_val(
    ///     BigExponentFloat::from(E),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{product:?}"), "0x8.8");
    /// assert_eq!(o, Less);
    ///
    /// let (product, o) = (&BigExponentFloat::from(PI)).mul_prec_round_ref_val(
    ///     BigExponentFloat::from(E),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{product:?}"), "0x9.0");
    /// assert_eq!(o, Greater);
    ///
    /// let (product, o) = (&BigExponentFloat::from(PI)).mul_prec_round_ref_val(
    ///     BigExponentFloat::from(E),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{product:?}"), "0x8.8");
    /// assert_eq!(o, Less);
    ///
    /// let (product, o) = (&BigExponentFloat::from(PI)).mul_prec_round_ref_val(
    ///     BigExponentFloat::from(E),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{product:?}"), "0x8.8a2c");
    /// assert_eq!(o, Less);
    ///
    /// let (product, o) = (&BigExponentFloat::from(PI)).mul_prec_round_ref_val(
    ///     BigExponentFloat::from(E),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{product:?}"), "0x8.8a2d");
    /// assert_eq!(o, Greater);
    ///
    /// let (product, o) = (&BigExponentFloat::from(PI)).mul_prec_round_ref_val(
    ///     BigExponentFloat::from(E),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{product:?}"), "0x8.8a2c");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn mul_prec_round_ref_val(
        &self,
        other: Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        mul_helper(Cow::Borrowed(self), Cow::Owned(other), Some((prec, rm)))
    }

    /// Multiplies two [`BigExponentFloat`]s, rounding the result to the specified precision and
    /// with the specified rounding mode. Both [`BigExponentFloat`]s are taken by reference. An
    /// [`Ordering`] is also returned, indicating whether the rounded product is less than, equal
    /// to, or greater than the exact product. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
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
    /// - $f(\text{NaN},x,p,m)=f(x,\text{NaN},p,m)=f(\pm\infty,0,p,m)=f(0,\pm\infty,p,m)=
    ///   \text{NaN}$
    /// - $f(\infty,x,p,m)=f(x,\infty,p,m)=\infty$ if $x>0$
    /// - $f(\infty,x,p,m)=f(x,\infty,p,m)=-\infty$ if $x<0$
    /// - $f(-\infty,x,p,m)=f(x,-\infty,p,m)=-\infty$ if $x>0$
    /// - $f(-\infty,x,p,m)=f(x,-\infty,p,m)=\infty$ if $x<0$
    /// - $f(0,x,p,m)=f(x,0,p,m)=0$ if $x$ is finite
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n + m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `max(self.significant_bits(), other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but `prec` is too small for an exact
    /// multiplication.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (product, o) = (&BigExponentFloat::from(PI)).mul_prec_round_ref_ref(
    ///     &BigExponentFloat::from(E),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{product:?}"), "0x8.8");
    /// assert_eq!(o, Less);
    ///
    /// let (product, o) = (&BigExponentFloat::from(PI)).mul_prec_round_ref_ref(
    ///     &BigExponentFloat::from(E),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{product:?}"), "0x9.0");
    /// assert_eq!(o, Greater);
    ///
    /// let (product, o) = (&BigExponentFloat::from(PI)).mul_prec_round_ref_ref(
    ///     &BigExponentFloat::from(E),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{product:?}"), "0x8.8");
    /// assert_eq!(o, Less);
    ///
    /// let (product, o) = (&BigExponentFloat::from(PI)).mul_prec_round_ref_ref(
    ///     &BigExponentFloat::from(E),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{product:?}"), "0x8.8a2c");
    /// assert_eq!(o, Less);
    ///
    /// let (product, o) = (&BigExponentFloat::from(PI)).mul_prec_round_ref_ref(
    ///     &BigExponentFloat::from(E),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{product:?}"), "0x8.8a2d");
    /// assert_eq!(o, Greater);
    ///
    /// let (product, o) = (&BigExponentFloat::from(PI)).mul_prec_round_ref_ref(
    ///     &BigExponentFloat::from(E),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{product:?}"), "0x8.8a2c");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn mul_prec_round_ref_ref(
        &self,
        other: &Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        mul_helper(Cow::Borrowed(self), Cow::Borrowed(other), Some((prec, rm)))
    }

    /// Multiplies a [`BigExponentFloat`] by a [`BigExponentFloat`] in place, rounding the result to
    /// the specified precision and with the specified rounding mode. The [`BigExponentFloat`] on
    /// the right-hand side is taken by value. An [`Ordering`] is returned, indicating whether the
    /// rounded product is less than, equal to, or greater than the exact product. Although `NaN`s
    /// are not comparable to any [`BigExponentFloat`], whenever this function sets the
    /// [`BigExponentFloat`] to `NaN` it also returns `Equal`.
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
    /// $T(n, m) = O(n + m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `max(self.significant_bits(), other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but `prec` is too small for an exact
    /// multiplication.
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
    ///     x.mul_prec_round_assign(BigExponentFloat::from(E), 5, Floor),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x8.8");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.mul_prec_round_assign(BigExponentFloat::from(E), 5, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(format!("{x:?}"), "0x9.0");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.mul_prec_round_assign(BigExponentFloat::from(E), 5, Nearest),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x8.8");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.mul_prec_round_assign(BigExponentFloat::from(E), 20, Floor),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x8.8a2c");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.mul_prec_round_assign(BigExponentFloat::from(E), 20, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(format!("{x:?}"), "0x8.8a2d");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.mul_prec_round_assign(BigExponentFloat::from(E), 20, Nearest),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x8.8a2c");
    /// ```
    #[inline]
    pub fn mul_prec_round_assign(&mut self, other: Self, prec: u64, rm: RoundingMode) -> Ordering {
        let x = core::mem::replace(self, Self::ZERO);
        let (result, o) = mul_helper(Cow::Owned(x), Cow::Owned(other), Some((prec, rm)));
        *self = result;
        o
    }

    /// Multiplies a [`BigExponentFloat`] by a [`BigExponentFloat`] in place, rounding the result to
    /// the specified precision and with the specified rounding mode. The [`BigExponentFloat`] on
    /// the right-hand side is taken by reference. An [`Ordering`] is returned, indicating whether
    /// the rounded product is less than, equal to, or greater than the exact product. Although
    /// `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function sets the
    /// [`BigExponentFloat`] to `NaN` it also returns `Equal`.
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
    /// $T(n, m) = O(n + m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `max(self.significant_bits(), other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but `prec` is too small for an exact
    /// multiplication.
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
    ///     x.mul_prec_round_assign_ref(&BigExponentFloat::from(E), 5, Floor),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x8.8");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.mul_prec_round_assign_ref(&BigExponentFloat::from(E), 5, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(format!("{x:?}"), "0x9.0");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.mul_prec_round_assign_ref(&BigExponentFloat::from(E), 5, Nearest),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x8.8");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.mul_prec_round_assign_ref(&BigExponentFloat::from(E), 20, Floor),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x8.8a2c");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.mul_prec_round_assign_ref(&BigExponentFloat::from(E), 20, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(format!("{x:?}"), "0x8.8a2d");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.mul_prec_round_assign_ref(&BigExponentFloat::from(E), 20, Nearest),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x8.8a2c");
    /// ```
    #[inline]
    pub fn mul_prec_round_assign_ref(
        &mut self,
        other: &Self,
        prec: u64,
        rm: RoundingMode,
    ) -> Ordering {
        let x = core::mem::replace(self, Self::ZERO);
        let (result, o) = mul_helper(Cow::Owned(x), Cow::Borrowed(other), Some((prec, rm)));
        *self = result;
        o
    }

    /// Multiplies two [`BigExponentFloat`]s, rounding the result to the nearest value of the
    /// specified precision. Both [`BigExponentFloat`]s are taken by value. An [`Ordering`] is also
    /// returned, indicating whether the rounded product is less than, equal to, or greater than the
    /// exact product. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,y,p) = xy+\varepsilon.
    /// $$
    /// - If $xy$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2 |xy|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,p)=f(x,\text{NaN},p)=f(\pm\infty,0,p)=f(0,\pm\infty,p)= \text{NaN}$
    /// - $f(\infty,x,p)=f(x,\infty,p)=\infty$ if $x>0$
    /// - $f(\infty,x,p)=f(x,\infty,p)=-\infty$ if $x<0$
    /// - $f(-\infty,x,p)=f(x,-\infty,p)=-\infty$ if $x>0$
    /// - $f(-\infty,x,p)=f(x,-\infty,p)=\infty$ if $x<0$
    /// - $f(0,x,p)=f(x,0,p)=0$ if $x$ is finite
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`BigExponentFloat::mul_prec_round`] instead. If you want the exact product, consider using
    /// `*` instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n + m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n + m \log m)$
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
    /// let (product, o) = BigExponentFloat::from(PI).mul_prec(BigExponentFloat::from(E), 5);
    /// assert_eq!(format!("{product:?}"), "0x8.8");
    /// assert_eq!(o, Less);
    ///
    /// let (product, o) = BigExponentFloat::from(PI).mul_prec(BigExponentFloat::from(E), 20);
    /// assert_eq!(format!("{product:?}"), "0x8.8a2c");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn mul_prec(self, other: Self, prec: u64) -> (Self, Ordering) {
        mul_helper(Cow::Owned(self), Cow::Owned(other), Some((prec, Nearest)))
    }

    /// Multiplies two [`BigExponentFloat`]s, rounding the result to the nearest value of the
    /// specified precision. The first [`BigExponentFloat`] is taken by value and the second by
    /// reference. An [`Ordering`] is also returned, indicating whether the rounded product is less
    /// than, equal to, or greater than the exact product. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,y,p) = xy+\varepsilon.
    /// $$
    /// - If $xy$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2 |xy|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,p)=f(x,\text{NaN},p)=f(\pm\infty,0,p)=f(0,\pm\infty,p)= \text{NaN}$
    /// - $f(\infty,x,p)=f(x,\infty,p)=\infty$ if $x>0$
    /// - $f(\infty,x,p)=f(x,\infty,p)=-\infty$ if $x<0$
    /// - $f(-\infty,x,p)=f(x,-\infty,p)=-\infty$ if $x>0$
    /// - $f(-\infty,x,p)=f(x,-\infty,p)=\infty$ if $x<0$
    /// - $f(0,x,p)=f(x,0,p)=0$ if $x$ is finite
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n + m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n + m \log m)$
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
    /// let (product, o) =
    ///     BigExponentFloat::from(PI).mul_prec_val_ref(&BigExponentFloat::from(E), 5);
    /// assert_eq!(format!("{product:?}"), "0x8.8");
    /// assert_eq!(o, Less);
    ///
    /// let (product, o) =
    ///     BigExponentFloat::from(PI).mul_prec_val_ref(&BigExponentFloat::from(E), 20);
    /// assert_eq!(format!("{product:?}"), "0x8.8a2c");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn mul_prec_val_ref(self, other: &Self, prec: u64) -> (Self, Ordering) {
        mul_helper(
            Cow::Owned(self),
            Cow::Borrowed(other),
            Some((prec, Nearest)),
        )
    }

    /// Multiplies two [`BigExponentFloat`]s, rounding the result to the nearest value of the
    /// specified precision. The first [`BigExponentFloat`] is taken by reference and the second by
    /// value. An [`Ordering`] is also returned, indicating whether the rounded product is less
    /// than, equal to, or greater than the exact product. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,y,p) = xy+\varepsilon.
    /// $$
    /// - If $xy$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2 |xy|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,p)=f(x,\text{NaN},p)=f(\pm\infty,0,p)=f(0,\pm\infty,p)= \text{NaN}$
    /// - $f(\infty,x,p)=f(x,\infty,p)=\infty$ if $x>0$
    /// - $f(\infty,x,p)=f(x,\infty,p)=-\infty$ if $x<0$
    /// - $f(-\infty,x,p)=f(x,-\infty,p)=-\infty$ if $x>0$
    /// - $f(-\infty,x,p)=f(x,-\infty,p)=\infty$ if $x<0$
    /// - $f(0,x,p)=f(x,0,p)=0$ if $x$ is finite
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n + m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n + m \log m)$
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
    /// let (product, o) =
    ///     (&BigExponentFloat::from(PI)).mul_prec_ref_val(BigExponentFloat::from(E), 5);
    /// assert_eq!(format!("{product:?}"), "0x8.8");
    /// assert_eq!(o, Less);
    ///
    /// let (product, o) =
    ///     (&BigExponentFloat::from(PI)).mul_prec_ref_val(BigExponentFloat::from(E), 20);
    /// assert_eq!(format!("{product:?}"), "0x8.8a2c");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn mul_prec_ref_val(&self, other: Self, prec: u64) -> (Self, Ordering) {
        mul_helper(
            Cow::Borrowed(self),
            Cow::Owned(other),
            Some((prec, Nearest)),
        )
    }

    /// Multiplies two [`BigExponentFloat`]s, rounding the result to the nearest value of the
    /// specified precision. Both [`BigExponentFloat`]s are taken by reference. An [`Ordering`] is
    /// also returned, indicating whether the rounded product is less than, equal to, or greater
    /// than the exact product. Although `NaN`s are not comparable to any [`BigExponentFloat`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,y,p) = xy+\varepsilon.
    /// $$
    /// - If $xy$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2 |xy|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,p)=f(x,\text{NaN},p)=f(\pm\infty,0,p)=f(0,\pm\infty,p)= \text{NaN}$
    /// - $f(\infty,x,p)=f(x,\infty,p)=\infty$ if $x>0$
    /// - $f(\infty,x,p)=f(x,\infty,p)=-\infty$ if $x<0$
    /// - $f(-\infty,x,p)=f(x,-\infty,p)=-\infty$ if $x>0$
    /// - $f(-\infty,x,p)=f(x,-\infty,p)=\infty$ if $x<0$
    /// - $f(0,x,p)=f(x,0,p)=0$ if $x$ is finite
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n + m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n + m \log m)$
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
    /// let (product, o) =
    ///     (&BigExponentFloat::from(PI)).mul_prec_ref_ref(&BigExponentFloat::from(E), 5);
    /// assert_eq!(format!("{product:?}"), "0x8.8");
    /// assert_eq!(o, Less);
    ///
    /// let (product, o) =
    ///     (&BigExponentFloat::from(PI)).mul_prec_ref_ref(&BigExponentFloat::from(E), 20);
    /// assert_eq!(format!("{product:?}"), "0x8.8a2c");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn mul_prec_ref_ref(&self, other: &Self, prec: u64) -> (Self, Ordering) {
        mul_helper(
            Cow::Borrowed(self),
            Cow::Borrowed(other),
            Some((prec, Nearest)),
        )
    }

    /// Multiplies a [`BigExponentFloat`] by a [`BigExponentFloat`] in place, rounding the result to
    /// the nearest value of the specified precision. The [`BigExponentFloat`] on the right-hand
    /// side is taken by value. An [`Ordering`] is returned, indicating whether the rounded product
    /// is less than, equal to, or greater than the exact product. Although `NaN`s are not
    /// comparable to any [`BigExponentFloat`], whenever this function sets the [`BigExponentFloat`]
    /// to `NaN` it also returns `Equal`.
    ///
    /// $$
    /// x \gets xy+\varepsilon.
    /// $$
    /// - If $xy$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2 |xy|\rfloor-p}$.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n + m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n + m \log m)$
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
    /// assert_eq!(x.mul_prec_assign(BigExponentFloat::from(E), 5), Less);
    /// assert_eq!(format!("{x:?}"), "0x8.8");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(x.mul_prec_assign(BigExponentFloat::from(E), 20), Less);
    /// assert_eq!(format!("{x:?}"), "0x8.8a2c");
    /// ```
    #[inline]
    pub fn mul_prec_assign(&mut self, other: Self, prec: u64) -> Ordering {
        let x = core::mem::replace(self, Self::ZERO);
        let (result, o) = mul_helper(Cow::Owned(x), Cow::Owned(other), Some((prec, Nearest)));
        *self = result;
        o
    }

    /// Multiplies a [`BigExponentFloat`] by a [`BigExponentFloat`] in place, rounding the result to
    /// the nearest value of the specified precision. The [`BigExponentFloat`] on the right-hand
    /// side is taken by reference. An [`Ordering`] is returned, indicating whether the rounded
    /// product is less than, equal to, or greater than the exact product. Although `NaN`s are not
    /// comparable to any [`BigExponentFloat`], whenever this function sets the [`BigExponentFloat`]
    /// to `NaN` it also returns `Equal`.
    ///
    /// $$
    /// x \gets xy+\varepsilon.
    /// $$
    /// - If $xy$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2 |xy|\rfloor-p}$.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n + m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n + m \log m)$
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
    /// assert_eq!(x.mul_prec_assign_ref(&BigExponentFloat::from(E), 5), Less);
    /// assert_eq!(format!("{x:?}"), "0x8.8");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(x.mul_prec_assign_ref(&BigExponentFloat::from(E), 20), Less);
    /// assert_eq!(format!("{x:?}"), "0x8.8a2c");
    /// ```
    #[inline]
    pub fn mul_prec_assign_ref(&mut self, other: &Self, prec: u64) -> Ordering {
        let x = core::mem::replace(self, Self::ZERO);
        let (result, o) = mul_helper(Cow::Owned(x), Cow::Borrowed(other), Some((prec, Nearest)));
        *self = result;
        o
    }
}

impl Mul<Self> for BigExponentFloat {
    type Output = Self;

    /// Multiplies two [`BigExponentFloat`]s. Both [`BigExponentFloat`]s are taken by value. The
    /// product is exact.
    ///
    /// $$
    /// f(x,y) = xy.
    /// $$
    ///
    /// Special cases:
    /// - $f(\text{NaN},x)=f(x,\text{NaN})=f(\pm\infty,0)=f(0,\pm\infty)=\text{NaN}$
    /// - $f(\infty,x)=f(x,\infty)=\infty$ if $x>0$
    /// - $f(\infty,x)=f(x,\infty)=-\infty$ if $x<0$
    /// - $f(-\infty,x)=f(x,-\infty)=-\infty$ if $x>0$
    /// - $f(-\infty,x)=f(x,-\infty)=\infty$ if $x<0$
    /// - $f(0,x)=f(x,0)=0$ if $x$ is finite
    ///
    /// This is FLINT's `arf_mul` with precision `ARF_PREC_EXACT`.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::BigExponentFloat;
    ///
    /// assert_eq!(
    ///     format!(
    ///         "{:?}",
    ///         BigExponentFloat::from(1.5) * BigExponentFloat::from(0.25)
    ///     ),
    ///     "0x0.6"
    /// );
    /// ```
    #[inline]
    fn mul(self, other: Self) -> Self {
        mul_helper(Cow::Owned(self), Cow::Owned(other), None).0
    }
}

impl Mul<&Self> for BigExponentFloat {
    type Output = Self;

    /// Multiplies two [`BigExponentFloat`]s. The first [`BigExponentFloat`] is taken by value and
    /// the second by reference. The product is exact.
    ///
    /// $$
    /// f(x,y) = xy.
    /// $$
    ///
    /// Special cases:
    /// - $f(\text{NaN},x)=f(x,\text{NaN})=f(\pm\infty,0)=f(0,\pm\infty)=\text{NaN}$
    /// - $f(\infty,x)=f(x,\infty)=\infty$ if $x>0$
    /// - $f(\infty,x)=f(x,\infty)=-\infty$ if $x<0$
    /// - $f(-\infty,x)=f(x,-\infty)=-\infty$ if $x>0$
    /// - $f(-\infty,x)=f(x,-\infty)=\infty$ if $x<0$
    /// - $f(0,x)=f(x,0)=0$ if $x$ is finite
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::BigExponentFloat;
    ///
    /// assert_eq!(
    ///     format!(
    ///         "{:?}",
    ///         BigExponentFloat::from(1.5) * &BigExponentFloat::from(0.25)
    ///     ),
    ///     "0x0.6"
    /// );
    /// ```
    #[inline]
    fn mul(self, other: &Self) -> Self {
        mul_helper(Cow::Owned(self), Cow::Borrowed(other), None).0
    }
}

impl Mul<BigExponentFloat> for &BigExponentFloat {
    type Output = BigExponentFloat;

    /// Multiplies two [`BigExponentFloat`]s. The first [`BigExponentFloat`] is taken by reference
    /// and the second by value. The product is exact.
    ///
    /// $$
    /// f(x,y) = xy.
    /// $$
    ///
    /// Special cases:
    /// - $f(\text{NaN},x)=f(x,\text{NaN})=f(\pm\infty,0)=f(0,\pm\infty)=\text{NaN}$
    /// - $f(\infty,x)=f(x,\infty)=\infty$ if $x>0$
    /// - $f(\infty,x)=f(x,\infty)=-\infty$ if $x<0$
    /// - $f(-\infty,x)=f(x,-\infty)=-\infty$ if $x>0$
    /// - $f(-\infty,x)=f(x,-\infty)=\infty$ if $x<0$
    /// - $f(0,x)=f(x,0)=0$ if $x$ is finite
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::BigExponentFloat;
    ///
    /// assert_eq!(
    ///     format!(
    ///         "{:?}",
    ///         (&BigExponentFloat::from(1.5)) * BigExponentFloat::from(0.25)
    ///     ),
    ///     "0x0.6"
    /// );
    /// ```
    #[inline]
    fn mul(self, other: BigExponentFloat) -> BigExponentFloat {
        mul_helper(Cow::Borrowed(self), Cow::Owned(other), None).0
    }
}

impl Mul<&BigExponentFloat> for &BigExponentFloat {
    type Output = BigExponentFloat;

    /// Multiplies two [`BigExponentFloat`]s. Both [`BigExponentFloat`]s are taken by reference. The
    /// product is exact.
    ///
    /// $$
    /// f(x,y) = xy.
    /// $$
    ///
    /// Special cases:
    /// - $f(\text{NaN},x)=f(x,\text{NaN})=f(\pm\infty,0)=f(0,\pm\infty)=\text{NaN}$
    /// - $f(\infty,x)=f(x,\infty)=\infty$ if $x>0$
    /// - $f(\infty,x)=f(x,\infty)=-\infty$ if $x<0$
    /// - $f(-\infty,x)=f(x,-\infty)=-\infty$ if $x>0$
    /// - $f(-\infty,x)=f(x,-\infty)=\infty$ if $x<0$
    /// - $f(0,x)=f(x,0)=0$ if $x$ is finite
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::BigExponentFloat;
    ///
    /// assert_eq!(
    ///     format!(
    ///         "{:?}",
    ///         (&BigExponentFloat::from(1.5)) * &BigExponentFloat::from(0.25)
    ///     ),
    ///     "0x0.6"
    /// );
    /// ```
    #[inline]
    fn mul(self, other: &BigExponentFloat) -> BigExponentFloat {
        mul_helper(Cow::Borrowed(self), Cow::Borrowed(other), None).0
    }
}

impl MulAssign<Self> for BigExponentFloat {
    /// Multiplies a [`BigExponentFloat`] by a [`BigExponentFloat`] in place. The
    /// [`BigExponentFloat`] on the right-hand side is taken by value. The product is exact.
    ///
    /// $$
    /// x \gets xy.
    /// $$
    ///
    /// See the `*` documentation for details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::BigExponentFloat;
    ///
    /// let mut x = BigExponentFloat::from(1.5);
    /// x *= BigExponentFloat::from(0.25);
    /// assert_eq!(format!("{x:?}"), "0x0.6");
    /// ```
    #[inline]
    fn mul_assign(&mut self, other: Self) {
        let x = core::mem::replace(self, Self::ZERO);
        *self = mul_helper(Cow::Owned(x), Cow::Owned(other), None).0;
    }
}

impl MulAssign<&Self> for BigExponentFloat {
    /// Multiplies a [`BigExponentFloat`] by a [`BigExponentFloat`] in place. The
    /// [`BigExponentFloat`] on the right-hand side is taken by reference. The product is exact.
    ///
    /// $$
    /// x \gets xy.
    /// $$
    ///
    /// See the `*` documentation for details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::BigExponentFloat;
    ///
    /// let mut x = BigExponentFloat::from(1.5);
    /// x *= &BigExponentFloat::from(0.25);
    /// assert_eq!(format!("{x:?}"), "0x0.6");
    /// ```
    #[inline]
    fn mul_assign(&mut self, other: &Self) {
        let x = core::mem::replace(self, Self::ZERO);
        *self = mul_helper(Cow::Owned(x), Cow::Borrowed(other), None).0;
    }
}
