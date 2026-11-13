// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::big_exponent_float::BigExponentFloat;
use crate::big_exponent_float::arithmetic::mul_add_mul::mul_add_mul_helper;
use alloc::borrow::Cow;
use core::cmp::Ordering;
use malachite_base::num::arithmetic::traits::{MulSubMul, MulSubMulAssign};
use malachite_base::num::basic::traits::Zero;
use malachite_base::rounding_modes::RoundingMode::{self, *};

impl BigExponentFloat {
    /// Subtracts the product of two [`BigExponentFloat`]s from the product of two other
    /// [`BigExponentFloat`]s, rounding the result to the specified precision and with the specified
    /// rounding mode. All four [`BigExponentFloat`]s are taken by value. An [`Ordering`] is also
    /// returned, indicating whether the rounded result is less than, equal to, or greater than the
    /// exact result. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// The products are computed exactly, so only one rounding occurs.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,z,w,p,m) = xy-zw+\varepsilon.
    /// $$
    /// - If $xy-zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy-zw$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy-zw|\rfloor-p+1}$.
    /// - If $xy-zw$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |xy-zw|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,y,z,w,p,m)=\text{NaN}$ if any argument is NaN, if either product multiplies an
    ///   infinity by zero, or if $xy$ and $zw$ are infinities of the same sign
    /// - $f(x,y,z,w,p,m)=\infty$ if $xy$ is $\infty$ or $zw$ is $-\infty$, and the other is not the
    ///   same infinity
    /// - $f(x,y,z,w,p,m)=-\infty$ if $xy$ is $-\infty$ or $zw$ is $\infty$, and the other is not
    ///   the same infinity
    /// - $f(x,y,z,w,p,m)=0$ if $xy=zw$ and both are finite
    ///
    /// If you know you'll be using `Nearest`, consider using [`BigExponentFloat::mul_sub_mul_prec`]
    /// instead. If you want the exact result, consider using [`MulSubMul::mul_sub_mul`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n + m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is the sum of the
    /// significant bits of the inputs.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, LN_2, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (result, o) = BigExponentFloat::from(PI).mul_sub_mul_prec_round(
    ///     BigExponentFloat::from(E),
    ///     BigExponentFloat::from(SQRT_2),
    ///     BigExponentFloat::from(LN_2),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x7.8");
    /// assert_eq!(o, Less);
    ///
    /// let (result, o) = BigExponentFloat::from(PI).mul_sub_mul_prec_round(
    ///     BigExponentFloat::from(E),
    ///     BigExponentFloat::from(SQRT_2),
    ///     BigExponentFloat::from(LN_2),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x7.c");
    /// assert_eq!(o, Greater);
    ///
    /// let (result, o) = BigExponentFloat::from(PI).mul_sub_mul_prec_round(
    ///     BigExponentFloat::from(E),
    ///     BigExponentFloat::from(SQRT_2),
    ///     BigExponentFloat::from(LN_2),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x7.8");
    /// assert_eq!(o, Less);
    ///
    /// let (result, o) = BigExponentFloat::from(PI).mul_sub_mul_prec_round(
    ///     BigExponentFloat::from(E),
    ///     BigExponentFloat::from(SQRT_2),
    ///     BigExponentFloat::from(LN_2),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x7.8f398");
    /// assert_eq!(o, Less);
    ///
    /// let (result, o) = BigExponentFloat::from(PI).mul_sub_mul_prec_round(
    ///     BigExponentFloat::from(E),
    ///     BigExponentFloat::from(SQRT_2),
    ///     BigExponentFloat::from(LN_2),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x7.8f3a");
    /// assert_eq!(o, Greater);
    ///
    /// let (result, o) = BigExponentFloat::from(PI).mul_sub_mul_prec_round(
    ///     BigExponentFloat::from(E),
    ///     BigExponentFloat::from(SQRT_2),
    ///     BigExponentFloat::from(LN_2),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x7.8f3a");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn mul_sub_mul_prec_round(
        self,
        y: Self,
        z: Self,
        w: Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        mul_add_mul_helper(
            Cow::Owned(self),
            Cow::Owned(y),
            Cow::Owned(z),
            Cow::Owned(w),
            true,
            Some((prec, rm)),
        )
    }

    /// Subtracts the product of two [`BigExponentFloat`]s from the product of two other
    /// [`BigExponentFloat`]s, rounding the result to the specified precision and with the specified
    /// rounding mode. The first, second, and third [`BigExponentFloat`]s are taken by value and the
    /// fourth by reference. An [`Ordering`] is also returned, indicating whether the rounded result
    /// is less than, equal to, or greater than the exact result. Although `NaN`s are not comparable
    /// to any [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The products are computed exactly, so only one rounding occurs.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,z,w,p,m) = xy-zw+\varepsilon.
    /// $$
    /// - If $xy-zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy-zw$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy-zw|\rfloor-p+1}$.
    /// - If $xy-zw$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |xy-zw|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,y,z,w,p,m)=\text{NaN}$ if any argument is NaN, if either product multiplies an
    ///   infinity by zero, or if $xy$ and $zw$ are infinities of the same sign
    /// - $f(x,y,z,w,p,m)=\infty$ if $xy$ is $\infty$ or $zw$ is $-\infty$, and the other is not the
    ///   same infinity
    /// - $f(x,y,z,w,p,m)=-\infty$ if $xy$ is $-\infty$ or $zw$ is $\infty$, and the other is not
    ///   the same infinity
    /// - $f(x,y,z,w,p,m)=0$ if $xy=zw$ and both are finite
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n + m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is the sum of the
    /// significant bits of the inputs.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, LN_2, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (result, o) = BigExponentFloat::from(PI).mul_sub_mul_prec_round_val_val_val_ref(
    ///     BigExponentFloat::from(E),
    ///     BigExponentFloat::from(SQRT_2),
    ///     &BigExponentFloat::from(LN_2),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x7.8");
    /// assert_eq!(o, Less);
    ///
    /// let (result, o) = BigExponentFloat::from(PI).mul_sub_mul_prec_round_val_val_val_ref(
    ///     BigExponentFloat::from(E),
    ///     BigExponentFloat::from(SQRT_2),
    ///     &BigExponentFloat::from(LN_2),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x7.c");
    /// assert_eq!(o, Greater);
    ///
    /// let (result, o) = BigExponentFloat::from(PI).mul_sub_mul_prec_round_val_val_val_ref(
    ///     BigExponentFloat::from(E),
    ///     BigExponentFloat::from(SQRT_2),
    ///     &BigExponentFloat::from(LN_2),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x7.8");
    /// assert_eq!(o, Less);
    ///
    /// let (result, o) = BigExponentFloat::from(PI).mul_sub_mul_prec_round_val_val_val_ref(
    ///     BigExponentFloat::from(E),
    ///     BigExponentFloat::from(SQRT_2),
    ///     &BigExponentFloat::from(LN_2),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x7.8f398");
    /// assert_eq!(o, Less);
    ///
    /// let (result, o) = BigExponentFloat::from(PI).mul_sub_mul_prec_round_val_val_val_ref(
    ///     BigExponentFloat::from(E),
    ///     BigExponentFloat::from(SQRT_2),
    ///     &BigExponentFloat::from(LN_2),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x7.8f3a");
    /// assert_eq!(o, Greater);
    ///
    /// let (result, o) = BigExponentFloat::from(PI).mul_sub_mul_prec_round_val_val_val_ref(
    ///     BigExponentFloat::from(E),
    ///     BigExponentFloat::from(SQRT_2),
    ///     &BigExponentFloat::from(LN_2),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x7.8f3a");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn mul_sub_mul_prec_round_val_val_val_ref(
        self,
        y: Self,
        z: Self,
        w: &Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        mul_add_mul_helper(
            Cow::Owned(self),
            Cow::Owned(y),
            Cow::Owned(z),
            Cow::Borrowed(w),
            true,
            Some((prec, rm)),
        )
    }

    /// Subtracts the product of two [`BigExponentFloat`]s from the product of two other
    /// [`BigExponentFloat`]s, rounding the result to the specified precision and with the specified
    /// rounding mode. The first, second, and fourth [`BigExponentFloat`]s are taken by value and
    /// the third by reference. An [`Ordering`] is also returned, indicating whether the rounded
    /// result is less than, equal to, or greater than the exact result. Although `NaN`s are not
    /// comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// The products are computed exactly, so only one rounding occurs.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,z,w,p,m) = xy-zw+\varepsilon.
    /// $$
    /// - If $xy-zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy-zw$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy-zw|\rfloor-p+1}$.
    /// - If $xy-zw$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |xy-zw|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,y,z,w,p,m)=\text{NaN}$ if any argument is NaN, if either product multiplies an
    ///   infinity by zero, or if $xy$ and $zw$ are infinities of the same sign
    /// - $f(x,y,z,w,p,m)=\infty$ if $xy$ is $\infty$ or $zw$ is $-\infty$, and the other is not the
    ///   same infinity
    /// - $f(x,y,z,w,p,m)=-\infty$ if $xy$ is $-\infty$ or $zw$ is $\infty$, and the other is not
    ///   the same infinity
    /// - $f(x,y,z,w,p,m)=0$ if $xy=zw$ and both are finite
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n + m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is the sum of the
    /// significant bits of the inputs.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, LN_2, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (result, o) = BigExponentFloat::from(PI).mul_sub_mul_prec_round_val_val_ref_val(
    ///     BigExponentFloat::from(E),
    ///     &BigExponentFloat::from(SQRT_2),
    ///     BigExponentFloat::from(LN_2),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x7.8");
    /// assert_eq!(o, Less);
    ///
    /// let (result, o) = BigExponentFloat::from(PI).mul_sub_mul_prec_round_val_val_ref_val(
    ///     BigExponentFloat::from(E),
    ///     &BigExponentFloat::from(SQRT_2),
    ///     BigExponentFloat::from(LN_2),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x7.c");
    /// assert_eq!(o, Greater);
    ///
    /// let (result, o) = BigExponentFloat::from(PI).mul_sub_mul_prec_round_val_val_ref_val(
    ///     BigExponentFloat::from(E),
    ///     &BigExponentFloat::from(SQRT_2),
    ///     BigExponentFloat::from(LN_2),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x7.8");
    /// assert_eq!(o, Less);
    ///
    /// let (result, o) = BigExponentFloat::from(PI).mul_sub_mul_prec_round_val_val_ref_val(
    ///     BigExponentFloat::from(E),
    ///     &BigExponentFloat::from(SQRT_2),
    ///     BigExponentFloat::from(LN_2),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x7.8f398");
    /// assert_eq!(o, Less);
    ///
    /// let (result, o) = BigExponentFloat::from(PI).mul_sub_mul_prec_round_val_val_ref_val(
    ///     BigExponentFloat::from(E),
    ///     &BigExponentFloat::from(SQRT_2),
    ///     BigExponentFloat::from(LN_2),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x7.8f3a");
    /// assert_eq!(o, Greater);
    ///
    /// let (result, o) = BigExponentFloat::from(PI).mul_sub_mul_prec_round_val_val_ref_val(
    ///     BigExponentFloat::from(E),
    ///     &BigExponentFloat::from(SQRT_2),
    ///     BigExponentFloat::from(LN_2),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x7.8f3a");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn mul_sub_mul_prec_round_val_val_ref_val(
        self,
        y: Self,
        z: &Self,
        w: Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        mul_add_mul_helper(
            Cow::Owned(self),
            Cow::Owned(y),
            Cow::Borrowed(z),
            Cow::Owned(w),
            true,
            Some((prec, rm)),
        )
    }

    /// Subtracts the product of two [`BigExponentFloat`]s from the product of two other
    /// [`BigExponentFloat`]s, rounding the result to the specified precision and with the specified
    /// rounding mode. The first and second [`BigExponentFloat`]s are taken by value and the third
    /// and fourth by reference. An [`Ordering`] is also returned, indicating whether the rounded
    /// result is less than, equal to, or greater than the exact result. Although `NaN`s are not
    /// comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// The products are computed exactly, so only one rounding occurs.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,z,w,p,m) = xy-zw+\varepsilon.
    /// $$
    /// - If $xy-zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy-zw$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy-zw|\rfloor-p+1}$.
    /// - If $xy-zw$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |xy-zw|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,y,z,w,p,m)=\text{NaN}$ if any argument is NaN, if either product multiplies an
    ///   infinity by zero, or if $xy$ and $zw$ are infinities of the same sign
    /// - $f(x,y,z,w,p,m)=\infty$ if $xy$ is $\infty$ or $zw$ is $-\infty$, and the other is not the
    ///   same infinity
    /// - $f(x,y,z,w,p,m)=-\infty$ if $xy$ is $-\infty$ or $zw$ is $\infty$, and the other is not
    ///   the same infinity
    /// - $f(x,y,z,w,p,m)=0$ if $xy=zw$ and both are finite
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n + m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is the sum of the
    /// significant bits of the inputs.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, LN_2, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (result, o) = BigExponentFloat::from(PI).mul_sub_mul_prec_round_val_val_ref_ref(
    ///     BigExponentFloat::from(E),
    ///     &BigExponentFloat::from(SQRT_2),
    ///     &BigExponentFloat::from(LN_2),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x7.8");
    /// assert_eq!(o, Less);
    ///
    /// let (result, o) = BigExponentFloat::from(PI).mul_sub_mul_prec_round_val_val_ref_ref(
    ///     BigExponentFloat::from(E),
    ///     &BigExponentFloat::from(SQRT_2),
    ///     &BigExponentFloat::from(LN_2),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x7.c");
    /// assert_eq!(o, Greater);
    ///
    /// let (result, o) = BigExponentFloat::from(PI).mul_sub_mul_prec_round_val_val_ref_ref(
    ///     BigExponentFloat::from(E),
    ///     &BigExponentFloat::from(SQRT_2),
    ///     &BigExponentFloat::from(LN_2),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x7.8");
    /// assert_eq!(o, Less);
    ///
    /// let (result, o) = BigExponentFloat::from(PI).mul_sub_mul_prec_round_val_val_ref_ref(
    ///     BigExponentFloat::from(E),
    ///     &BigExponentFloat::from(SQRT_2),
    ///     &BigExponentFloat::from(LN_2),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x7.8f398");
    /// assert_eq!(o, Less);
    ///
    /// let (result, o) = BigExponentFloat::from(PI).mul_sub_mul_prec_round_val_val_ref_ref(
    ///     BigExponentFloat::from(E),
    ///     &BigExponentFloat::from(SQRT_2),
    ///     &BigExponentFloat::from(LN_2),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x7.8f3a");
    /// assert_eq!(o, Greater);
    ///
    /// let (result, o) = BigExponentFloat::from(PI).mul_sub_mul_prec_round_val_val_ref_ref(
    ///     BigExponentFloat::from(E),
    ///     &BigExponentFloat::from(SQRT_2),
    ///     &BigExponentFloat::from(LN_2),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x7.8f3a");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn mul_sub_mul_prec_round_val_val_ref_ref(
        self,
        y: Self,
        z: &Self,
        w: &Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        mul_add_mul_helper(
            Cow::Owned(self),
            Cow::Owned(y),
            Cow::Borrowed(z),
            Cow::Borrowed(w),
            true,
            Some((prec, rm)),
        )
    }

    /// Subtracts the product of two [`BigExponentFloat`]s from the product of two other
    /// [`BigExponentFloat`]s, rounding the result to the specified precision and with the specified
    /// rounding mode. The first, third, and fourth [`BigExponentFloat`]s are taken by value and the
    /// second by reference. An [`Ordering`] is also returned, indicating whether the rounded result
    /// is less than, equal to, or greater than the exact result. Although `NaN`s are not comparable
    /// to any [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The products are computed exactly, so only one rounding occurs.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,z,w,p,m) = xy-zw+\varepsilon.
    /// $$
    /// - If $xy-zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy-zw$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy-zw|\rfloor-p+1}$.
    /// - If $xy-zw$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |xy-zw|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,y,z,w,p,m)=\text{NaN}$ if any argument is NaN, if either product multiplies an
    ///   infinity by zero, or if $xy$ and $zw$ are infinities of the same sign
    /// - $f(x,y,z,w,p,m)=\infty$ if $xy$ is $\infty$ or $zw$ is $-\infty$, and the other is not the
    ///   same infinity
    /// - $f(x,y,z,w,p,m)=-\infty$ if $xy$ is $-\infty$ or $zw$ is $\infty$, and the other is not
    ///   the same infinity
    /// - $f(x,y,z,w,p,m)=0$ if $xy=zw$ and both are finite
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n + m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is the sum of the
    /// significant bits of the inputs.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, LN_2, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (result, o) = BigExponentFloat::from(PI).mul_sub_mul_prec_round_val_ref_val_val(
    ///     &BigExponentFloat::from(E),
    ///     BigExponentFloat::from(SQRT_2),
    ///     BigExponentFloat::from(LN_2),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x7.8");
    /// assert_eq!(o, Less);
    ///
    /// let (result, o) = BigExponentFloat::from(PI).mul_sub_mul_prec_round_val_ref_val_val(
    ///     &BigExponentFloat::from(E),
    ///     BigExponentFloat::from(SQRT_2),
    ///     BigExponentFloat::from(LN_2),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x7.c");
    /// assert_eq!(o, Greater);
    ///
    /// let (result, o) = BigExponentFloat::from(PI).mul_sub_mul_prec_round_val_ref_val_val(
    ///     &BigExponentFloat::from(E),
    ///     BigExponentFloat::from(SQRT_2),
    ///     BigExponentFloat::from(LN_2),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x7.8");
    /// assert_eq!(o, Less);
    ///
    /// let (result, o) = BigExponentFloat::from(PI).mul_sub_mul_prec_round_val_ref_val_val(
    ///     &BigExponentFloat::from(E),
    ///     BigExponentFloat::from(SQRT_2),
    ///     BigExponentFloat::from(LN_2),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x7.8f398");
    /// assert_eq!(o, Less);
    ///
    /// let (result, o) = BigExponentFloat::from(PI).mul_sub_mul_prec_round_val_ref_val_val(
    ///     &BigExponentFloat::from(E),
    ///     BigExponentFloat::from(SQRT_2),
    ///     BigExponentFloat::from(LN_2),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x7.8f3a");
    /// assert_eq!(o, Greater);
    ///
    /// let (result, o) = BigExponentFloat::from(PI).mul_sub_mul_prec_round_val_ref_val_val(
    ///     &BigExponentFloat::from(E),
    ///     BigExponentFloat::from(SQRT_2),
    ///     BigExponentFloat::from(LN_2),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x7.8f3a");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn mul_sub_mul_prec_round_val_ref_val_val(
        self,
        y: &Self,
        z: Self,
        w: Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        mul_add_mul_helper(
            Cow::Owned(self),
            Cow::Borrowed(y),
            Cow::Owned(z),
            Cow::Owned(w),
            true,
            Some((prec, rm)),
        )
    }

    /// Subtracts the product of two [`BigExponentFloat`]s from the product of two other
    /// [`BigExponentFloat`]s, rounding the result to the specified precision and with the specified
    /// rounding mode. The first and third [`BigExponentFloat`]s are taken by value and the second
    /// and fourth by reference. An [`Ordering`] is also returned, indicating whether the rounded
    /// result is less than, equal to, or greater than the exact result. Although `NaN`s are not
    /// comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// The products are computed exactly, so only one rounding occurs.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,z,w,p,m) = xy-zw+\varepsilon.
    /// $$
    /// - If $xy-zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy-zw$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy-zw|\rfloor-p+1}$.
    /// - If $xy-zw$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |xy-zw|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,y,z,w,p,m)=\text{NaN}$ if any argument is NaN, if either product multiplies an
    ///   infinity by zero, or if $xy$ and $zw$ are infinities of the same sign
    /// - $f(x,y,z,w,p,m)=\infty$ if $xy$ is $\infty$ or $zw$ is $-\infty$, and the other is not the
    ///   same infinity
    /// - $f(x,y,z,w,p,m)=-\infty$ if $xy$ is $-\infty$ or $zw$ is $\infty$, and the other is not
    ///   the same infinity
    /// - $f(x,y,z,w,p,m)=0$ if $xy=zw$ and both are finite
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n + m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is the sum of the
    /// significant bits of the inputs.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, LN_2, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (result, o) = BigExponentFloat::from(PI).mul_sub_mul_prec_round_val_ref_val_ref(
    ///     &BigExponentFloat::from(E),
    ///     BigExponentFloat::from(SQRT_2),
    ///     &BigExponentFloat::from(LN_2),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x7.8");
    /// assert_eq!(o, Less);
    ///
    /// let (result, o) = BigExponentFloat::from(PI).mul_sub_mul_prec_round_val_ref_val_ref(
    ///     &BigExponentFloat::from(E),
    ///     BigExponentFloat::from(SQRT_2),
    ///     &BigExponentFloat::from(LN_2),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x7.c");
    /// assert_eq!(o, Greater);
    ///
    /// let (result, o) = BigExponentFloat::from(PI).mul_sub_mul_prec_round_val_ref_val_ref(
    ///     &BigExponentFloat::from(E),
    ///     BigExponentFloat::from(SQRT_2),
    ///     &BigExponentFloat::from(LN_2),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x7.8");
    /// assert_eq!(o, Less);
    ///
    /// let (result, o) = BigExponentFloat::from(PI).mul_sub_mul_prec_round_val_ref_val_ref(
    ///     &BigExponentFloat::from(E),
    ///     BigExponentFloat::from(SQRT_2),
    ///     &BigExponentFloat::from(LN_2),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x7.8f398");
    /// assert_eq!(o, Less);
    ///
    /// let (result, o) = BigExponentFloat::from(PI).mul_sub_mul_prec_round_val_ref_val_ref(
    ///     &BigExponentFloat::from(E),
    ///     BigExponentFloat::from(SQRT_2),
    ///     &BigExponentFloat::from(LN_2),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x7.8f3a");
    /// assert_eq!(o, Greater);
    ///
    /// let (result, o) = BigExponentFloat::from(PI).mul_sub_mul_prec_round_val_ref_val_ref(
    ///     &BigExponentFloat::from(E),
    ///     BigExponentFloat::from(SQRT_2),
    ///     &BigExponentFloat::from(LN_2),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x7.8f3a");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn mul_sub_mul_prec_round_val_ref_val_ref(
        self,
        y: &Self,
        z: Self,
        w: &Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        mul_add_mul_helper(
            Cow::Owned(self),
            Cow::Borrowed(y),
            Cow::Owned(z),
            Cow::Borrowed(w),
            true,
            Some((prec, rm)),
        )
    }

    /// Subtracts the product of two [`BigExponentFloat`]s from the product of two other
    /// [`BigExponentFloat`]s, rounding the result to the specified precision and with the specified
    /// rounding mode. The first and fourth [`BigExponentFloat`]s are taken by value and the second
    /// and third by reference. An [`Ordering`] is also returned, indicating whether the rounded
    /// result is less than, equal to, or greater than the exact result. Although `NaN`s are not
    /// comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// The products are computed exactly, so only one rounding occurs.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,z,w,p,m) = xy-zw+\varepsilon.
    /// $$
    /// - If $xy-zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy-zw$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy-zw|\rfloor-p+1}$.
    /// - If $xy-zw$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |xy-zw|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,y,z,w,p,m)=\text{NaN}$ if any argument is NaN, if either product multiplies an
    ///   infinity by zero, or if $xy$ and $zw$ are infinities of the same sign
    /// - $f(x,y,z,w,p,m)=\infty$ if $xy$ is $\infty$ or $zw$ is $-\infty$, and the other is not the
    ///   same infinity
    /// - $f(x,y,z,w,p,m)=-\infty$ if $xy$ is $-\infty$ or $zw$ is $\infty$, and the other is not
    ///   the same infinity
    /// - $f(x,y,z,w,p,m)=0$ if $xy=zw$ and both are finite
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n + m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is the sum of the
    /// significant bits of the inputs.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, LN_2, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (result, o) = BigExponentFloat::from(PI).mul_sub_mul_prec_round_val_ref_ref_val(
    ///     &BigExponentFloat::from(E),
    ///     &BigExponentFloat::from(SQRT_2),
    ///     BigExponentFloat::from(LN_2),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x7.8");
    /// assert_eq!(o, Less);
    ///
    /// let (result, o) = BigExponentFloat::from(PI).mul_sub_mul_prec_round_val_ref_ref_val(
    ///     &BigExponentFloat::from(E),
    ///     &BigExponentFloat::from(SQRT_2),
    ///     BigExponentFloat::from(LN_2),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x7.c");
    /// assert_eq!(o, Greater);
    ///
    /// let (result, o) = BigExponentFloat::from(PI).mul_sub_mul_prec_round_val_ref_ref_val(
    ///     &BigExponentFloat::from(E),
    ///     &BigExponentFloat::from(SQRT_2),
    ///     BigExponentFloat::from(LN_2),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x7.8");
    /// assert_eq!(o, Less);
    ///
    /// let (result, o) = BigExponentFloat::from(PI).mul_sub_mul_prec_round_val_ref_ref_val(
    ///     &BigExponentFloat::from(E),
    ///     &BigExponentFloat::from(SQRT_2),
    ///     BigExponentFloat::from(LN_2),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x7.8f398");
    /// assert_eq!(o, Less);
    ///
    /// let (result, o) = BigExponentFloat::from(PI).mul_sub_mul_prec_round_val_ref_ref_val(
    ///     &BigExponentFloat::from(E),
    ///     &BigExponentFloat::from(SQRT_2),
    ///     BigExponentFloat::from(LN_2),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x7.8f3a");
    /// assert_eq!(o, Greater);
    ///
    /// let (result, o) = BigExponentFloat::from(PI).mul_sub_mul_prec_round_val_ref_ref_val(
    ///     &BigExponentFloat::from(E),
    ///     &BigExponentFloat::from(SQRT_2),
    ///     BigExponentFloat::from(LN_2),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x7.8f3a");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn mul_sub_mul_prec_round_val_ref_ref_val(
        self,
        y: &Self,
        z: &Self,
        w: Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        mul_add_mul_helper(
            Cow::Owned(self),
            Cow::Borrowed(y),
            Cow::Borrowed(z),
            Cow::Owned(w),
            true,
            Some((prec, rm)),
        )
    }

    /// Subtracts the product of two [`BigExponentFloat`]s from the product of two other
    /// [`BigExponentFloat`]s, rounding the result to the specified precision and with the specified
    /// rounding mode. The first [`BigExponentFloat`] is taken by value and the second, third, and
    /// fourth by reference. An [`Ordering`] is also returned, indicating whether the rounded result
    /// is less than, equal to, or greater than the exact result. Although `NaN`s are not comparable
    /// to any [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The products are computed exactly, so only one rounding occurs.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,z,w,p,m) = xy-zw+\varepsilon.
    /// $$
    /// - If $xy-zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy-zw$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy-zw|\rfloor-p+1}$.
    /// - If $xy-zw$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |xy-zw|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,y,z,w,p,m)=\text{NaN}$ if any argument is NaN, if either product multiplies an
    ///   infinity by zero, or if $xy$ and $zw$ are infinities of the same sign
    /// - $f(x,y,z,w,p,m)=\infty$ if $xy$ is $\infty$ or $zw$ is $-\infty$, and the other is not the
    ///   same infinity
    /// - $f(x,y,z,w,p,m)=-\infty$ if $xy$ is $-\infty$ or $zw$ is $\infty$, and the other is not
    ///   the same infinity
    /// - $f(x,y,z,w,p,m)=0$ if $xy=zw$ and both are finite
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n + m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is the sum of the
    /// significant bits of the inputs.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, LN_2, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (result, o) = BigExponentFloat::from(PI).mul_sub_mul_prec_round_val_ref_ref_ref(
    ///     &BigExponentFloat::from(E),
    ///     &BigExponentFloat::from(SQRT_2),
    ///     &BigExponentFloat::from(LN_2),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x7.8");
    /// assert_eq!(o, Less);
    ///
    /// let (result, o) = BigExponentFloat::from(PI).mul_sub_mul_prec_round_val_ref_ref_ref(
    ///     &BigExponentFloat::from(E),
    ///     &BigExponentFloat::from(SQRT_2),
    ///     &BigExponentFloat::from(LN_2),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x7.c");
    /// assert_eq!(o, Greater);
    ///
    /// let (result, o) = BigExponentFloat::from(PI).mul_sub_mul_prec_round_val_ref_ref_ref(
    ///     &BigExponentFloat::from(E),
    ///     &BigExponentFloat::from(SQRT_2),
    ///     &BigExponentFloat::from(LN_2),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x7.8");
    /// assert_eq!(o, Less);
    ///
    /// let (result, o) = BigExponentFloat::from(PI).mul_sub_mul_prec_round_val_ref_ref_ref(
    ///     &BigExponentFloat::from(E),
    ///     &BigExponentFloat::from(SQRT_2),
    ///     &BigExponentFloat::from(LN_2),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x7.8f398");
    /// assert_eq!(o, Less);
    ///
    /// let (result, o) = BigExponentFloat::from(PI).mul_sub_mul_prec_round_val_ref_ref_ref(
    ///     &BigExponentFloat::from(E),
    ///     &BigExponentFloat::from(SQRT_2),
    ///     &BigExponentFloat::from(LN_2),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x7.8f3a");
    /// assert_eq!(o, Greater);
    ///
    /// let (result, o) = BigExponentFloat::from(PI).mul_sub_mul_prec_round_val_ref_ref_ref(
    ///     &BigExponentFloat::from(E),
    ///     &BigExponentFloat::from(SQRT_2),
    ///     &BigExponentFloat::from(LN_2),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x7.8f3a");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn mul_sub_mul_prec_round_val_ref_ref_ref(
        self,
        y: &Self,
        z: &Self,
        w: &Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        mul_add_mul_helper(
            Cow::Owned(self),
            Cow::Borrowed(y),
            Cow::Borrowed(z),
            Cow::Borrowed(w),
            true,
            Some((prec, rm)),
        )
    }

    /// Subtracts the product of two [`BigExponentFloat`]s from the product of two other
    /// [`BigExponentFloat`]s, rounding the result to the specified precision and with the specified
    /// rounding mode. All four [`BigExponentFloat`]s are taken by reference. An [`Ordering`] is
    /// also returned, indicating whether the rounded result is less than, equal to, or greater than
    /// the exact result. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever
    /// this function returns a `NaN` it also returns `Equal`.
    ///
    /// The products are computed exactly, so only one rounding occurs.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,z,w,p,m) = xy-zw+\varepsilon.
    /// $$
    /// - If $xy-zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy-zw$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy-zw|\rfloor-p+1}$.
    /// - If $xy-zw$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |xy-zw|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,y,z,w,p,m)=\text{NaN}$ if any argument is NaN, if either product multiplies an
    ///   infinity by zero, or if $xy$ and $zw$ are infinities of the same sign
    /// - $f(x,y,z,w,p,m)=\infty$ if $xy$ is $\infty$ or $zw$ is $-\infty$, and the other is not the
    ///   same infinity
    /// - $f(x,y,z,w,p,m)=-\infty$ if $xy$ is $-\infty$ or $zw$ is $\infty$, and the other is not
    ///   the same infinity
    /// - $f(x,y,z,w,p,m)=0$ if $xy=zw$ and both are finite
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n + m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is the sum of the
    /// significant bits of the inputs.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, LN_2, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (result, o) = (&BigExponentFloat::from(PI)).mul_sub_mul_prec_round_ref_ref_ref_ref(
    ///     &BigExponentFloat::from(E),
    ///     &BigExponentFloat::from(SQRT_2),
    ///     &BigExponentFloat::from(LN_2),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x7.8");
    /// assert_eq!(o, Less);
    ///
    /// let (result, o) = (&BigExponentFloat::from(PI)).mul_sub_mul_prec_round_ref_ref_ref_ref(
    ///     &BigExponentFloat::from(E),
    ///     &BigExponentFloat::from(SQRT_2),
    ///     &BigExponentFloat::from(LN_2),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x7.c");
    /// assert_eq!(o, Greater);
    ///
    /// let (result, o) = (&BigExponentFloat::from(PI)).mul_sub_mul_prec_round_ref_ref_ref_ref(
    ///     &BigExponentFloat::from(E),
    ///     &BigExponentFloat::from(SQRT_2),
    ///     &BigExponentFloat::from(LN_2),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x7.8");
    /// assert_eq!(o, Less);
    ///
    /// let (result, o) = (&BigExponentFloat::from(PI)).mul_sub_mul_prec_round_ref_ref_ref_ref(
    ///     &BigExponentFloat::from(E),
    ///     &BigExponentFloat::from(SQRT_2),
    ///     &BigExponentFloat::from(LN_2),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x7.8f398");
    /// assert_eq!(o, Less);
    ///
    /// let (result, o) = (&BigExponentFloat::from(PI)).mul_sub_mul_prec_round_ref_ref_ref_ref(
    ///     &BigExponentFloat::from(E),
    ///     &BigExponentFloat::from(SQRT_2),
    ///     &BigExponentFloat::from(LN_2),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x7.8f3a");
    /// assert_eq!(o, Greater);
    ///
    /// let (result, o) = (&BigExponentFloat::from(PI)).mul_sub_mul_prec_round_ref_ref_ref_ref(
    ///     &BigExponentFloat::from(E),
    ///     &BigExponentFloat::from(SQRT_2),
    ///     &BigExponentFloat::from(LN_2),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x7.8f3a");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn mul_sub_mul_prec_round_ref_ref_ref_ref(
        &self,
        y: &Self,
        z: &Self,
        w: &Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        mul_add_mul_helper(
            Cow::Borrowed(self),
            Cow::Borrowed(y),
            Cow::Borrowed(z),
            Cow::Borrowed(w),
            true,
            Some((prec, rm)),
        )
    }

    /// Subtracts the product of two [`BigExponentFloat`]s from a [`BigExponentFloat`] multiplied by
    /// a [`BigExponentFloat`], in place, rounding the result to the specified precision and with
    /// the specified rounding mode. The [`BigExponentFloat`]s on the right-hand side are taken by
    /// value. An [`Ordering`] is returned, indicating whether the rounded result is less than,
    /// equal to, or greater than the exact result. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function sets the [`BigExponentFloat`] to `NaN` it also
    /// returns `Equal`.
    ///
    /// The products are computed exactly, so only one rounding occurs.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets xy-zw+\varepsilon.
    /// $$
    /// - If $xy-zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy-zw$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy-zw|\rfloor-p+1}$.
    /// - If $xy-zw$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |xy-zw|\rfloor-p}$.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n + m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is the sum of the
    /// significant bits of the inputs.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, LN_2, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.mul_sub_mul_prec_round_assign(
    ///         BigExponentFloat::from(E),
    ///         BigExponentFloat::from(SQRT_2),
    ///         BigExponentFloat::from(LN_2),
    ///         5,
    ///         Floor
    ///     ),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x7.8");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.mul_sub_mul_prec_round_assign(
    ///         BigExponentFloat::from(E),
    ///         BigExponentFloat::from(SQRT_2),
    ///         BigExponentFloat::from(LN_2),
    ///         5,
    ///         Ceiling
    ///     ),
    ///     Greater
    /// );
    /// assert_eq!(format!("{x:?}"), "0x7.c");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.mul_sub_mul_prec_round_assign(
    ///         BigExponentFloat::from(E),
    ///         BigExponentFloat::from(SQRT_2),
    ///         BigExponentFloat::from(LN_2),
    ///         5,
    ///         Nearest
    ///     ),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x7.8");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.mul_sub_mul_prec_round_assign(
    ///         BigExponentFloat::from(E),
    ///         BigExponentFloat::from(SQRT_2),
    ///         BigExponentFloat::from(LN_2),
    ///         20,
    ///         Floor
    ///     ),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x7.8f398");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.mul_sub_mul_prec_round_assign(
    ///         BigExponentFloat::from(E),
    ///         BigExponentFloat::from(SQRT_2),
    ///         BigExponentFloat::from(LN_2),
    ///         20,
    ///         Ceiling
    ///     ),
    ///     Greater
    /// );
    /// assert_eq!(format!("{x:?}"), "0x7.8f3a");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.mul_sub_mul_prec_round_assign(
    ///         BigExponentFloat::from(E),
    ///         BigExponentFloat::from(SQRT_2),
    ///         BigExponentFloat::from(LN_2),
    ///         20,
    ///         Nearest
    ///     ),
    ///     Greater
    /// );
    /// assert_eq!(format!("{x:?}"), "0x7.8f3a");
    /// ```
    #[inline]
    pub fn mul_sub_mul_prec_round_assign(
        &mut self,
        y: Self,
        z: Self,
        w: Self,
        prec: u64,
        rm: RoundingMode,
    ) -> Ordering {
        let x = core::mem::replace(self, Self::ZERO);
        let (result, o) = mul_add_mul_helper(
            Cow::Owned(x),
            Cow::Owned(y),
            Cow::Owned(z),
            Cow::Owned(w),
            true,
            Some((prec, rm)),
        );
        *self = result;
        o
    }

    /// Subtracts the product of two [`BigExponentFloat`]s from a [`BigExponentFloat`] multiplied by
    /// a [`BigExponentFloat`], in place, rounding the result to the specified precision and with
    /// the specified rounding mode. On the right-hand side, the first and second
    /// [`BigExponentFloat`]s are taken by value and the third by reference. An [`Ordering`] is
    /// returned, indicating whether the rounded result is less than, equal to, or greater than the
    /// exact result. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this
    /// function sets the [`BigExponentFloat`] to `NaN` it also returns `Equal`.
    ///
    /// The products are computed exactly, so only one rounding occurs.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets xy-zw+\varepsilon.
    /// $$
    /// - If $xy-zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy-zw$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy-zw|\rfloor-p+1}$.
    /// - If $xy-zw$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |xy-zw|\rfloor-p}$.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n + m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is the sum of the
    /// significant bits of the inputs.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, LN_2, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.mul_sub_mul_prec_round_assign_val_val_ref(
    ///         BigExponentFloat::from(E),
    ///         BigExponentFloat::from(SQRT_2),
    ///         &BigExponentFloat::from(LN_2),
    ///         5,
    ///         Floor
    ///     ),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x7.8");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.mul_sub_mul_prec_round_assign_val_val_ref(
    ///         BigExponentFloat::from(E),
    ///         BigExponentFloat::from(SQRT_2),
    ///         &BigExponentFloat::from(LN_2),
    ///         5,
    ///         Ceiling
    ///     ),
    ///     Greater
    /// );
    /// assert_eq!(format!("{x:?}"), "0x7.c");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.mul_sub_mul_prec_round_assign_val_val_ref(
    ///         BigExponentFloat::from(E),
    ///         BigExponentFloat::from(SQRT_2),
    ///         &BigExponentFloat::from(LN_2),
    ///         5,
    ///         Nearest
    ///     ),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x7.8");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.mul_sub_mul_prec_round_assign_val_val_ref(
    ///         BigExponentFloat::from(E),
    ///         BigExponentFloat::from(SQRT_2),
    ///         &BigExponentFloat::from(LN_2),
    ///         20,
    ///         Floor
    ///     ),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x7.8f398");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.mul_sub_mul_prec_round_assign_val_val_ref(
    ///         BigExponentFloat::from(E),
    ///         BigExponentFloat::from(SQRT_2),
    ///         &BigExponentFloat::from(LN_2),
    ///         20,
    ///         Ceiling
    ///     ),
    ///     Greater
    /// );
    /// assert_eq!(format!("{x:?}"), "0x7.8f3a");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.mul_sub_mul_prec_round_assign_val_val_ref(
    ///         BigExponentFloat::from(E),
    ///         BigExponentFloat::from(SQRT_2),
    ///         &BigExponentFloat::from(LN_2),
    ///         20,
    ///         Nearest
    ///     ),
    ///     Greater
    /// );
    /// assert_eq!(format!("{x:?}"), "0x7.8f3a");
    /// ```
    #[inline]
    pub fn mul_sub_mul_prec_round_assign_val_val_ref(
        &mut self,
        y: Self,
        z: Self,
        w: &Self,
        prec: u64,
        rm: RoundingMode,
    ) -> Ordering {
        let x = core::mem::replace(self, Self::ZERO);
        let (result, o) = mul_add_mul_helper(
            Cow::Owned(x),
            Cow::Owned(y),
            Cow::Owned(z),
            Cow::Borrowed(w),
            true,
            Some((prec, rm)),
        );
        *self = result;
        o
    }

    /// Subtracts the product of two [`BigExponentFloat`]s from a [`BigExponentFloat`] multiplied by
    /// a [`BigExponentFloat`], in place, rounding the result to the specified precision and with
    /// the specified rounding mode. On the right-hand side, the first and third
    /// [`BigExponentFloat`]s are taken by value and the second by reference. An [`Ordering`] is
    /// returned, indicating whether the rounded result is less than, equal to, or greater than the
    /// exact result. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this
    /// function sets the [`BigExponentFloat`] to `NaN` it also returns `Equal`.
    ///
    /// The products are computed exactly, so only one rounding occurs.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets xy-zw+\varepsilon.
    /// $$
    /// - If $xy-zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy-zw$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy-zw|\rfloor-p+1}$.
    /// - If $xy-zw$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |xy-zw|\rfloor-p}$.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n + m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is the sum of the
    /// significant bits of the inputs.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, LN_2, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.mul_sub_mul_prec_round_assign_val_ref_val(
    ///         BigExponentFloat::from(E),
    ///         &BigExponentFloat::from(SQRT_2),
    ///         BigExponentFloat::from(LN_2),
    ///         5,
    ///         Floor
    ///     ),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x7.8");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.mul_sub_mul_prec_round_assign_val_ref_val(
    ///         BigExponentFloat::from(E),
    ///         &BigExponentFloat::from(SQRT_2),
    ///         BigExponentFloat::from(LN_2),
    ///         5,
    ///         Ceiling
    ///     ),
    ///     Greater
    /// );
    /// assert_eq!(format!("{x:?}"), "0x7.c");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.mul_sub_mul_prec_round_assign_val_ref_val(
    ///         BigExponentFloat::from(E),
    ///         &BigExponentFloat::from(SQRT_2),
    ///         BigExponentFloat::from(LN_2),
    ///         5,
    ///         Nearest
    ///     ),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x7.8");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.mul_sub_mul_prec_round_assign_val_ref_val(
    ///         BigExponentFloat::from(E),
    ///         &BigExponentFloat::from(SQRT_2),
    ///         BigExponentFloat::from(LN_2),
    ///         20,
    ///         Floor
    ///     ),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x7.8f398");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.mul_sub_mul_prec_round_assign_val_ref_val(
    ///         BigExponentFloat::from(E),
    ///         &BigExponentFloat::from(SQRT_2),
    ///         BigExponentFloat::from(LN_2),
    ///         20,
    ///         Ceiling
    ///     ),
    ///     Greater
    /// );
    /// assert_eq!(format!("{x:?}"), "0x7.8f3a");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.mul_sub_mul_prec_round_assign_val_ref_val(
    ///         BigExponentFloat::from(E),
    ///         &BigExponentFloat::from(SQRT_2),
    ///         BigExponentFloat::from(LN_2),
    ///         20,
    ///         Nearest
    ///     ),
    ///     Greater
    /// );
    /// assert_eq!(format!("{x:?}"), "0x7.8f3a");
    /// ```
    #[inline]
    pub fn mul_sub_mul_prec_round_assign_val_ref_val(
        &mut self,
        y: Self,
        z: &Self,
        w: Self,
        prec: u64,
        rm: RoundingMode,
    ) -> Ordering {
        let x = core::mem::replace(self, Self::ZERO);
        let (result, o) = mul_add_mul_helper(
            Cow::Owned(x),
            Cow::Owned(y),
            Cow::Borrowed(z),
            Cow::Owned(w),
            true,
            Some((prec, rm)),
        );
        *self = result;
        o
    }

    /// Subtracts the product of two [`BigExponentFloat`]s from a [`BigExponentFloat`] multiplied by
    /// a [`BigExponentFloat`], in place, rounding the result to the specified precision and with
    /// the specified rounding mode. On the right-hand side, the first [`BigExponentFloat`] is taken
    /// by value and the second and third by reference. An [`Ordering`] is returned, indicating
    /// whether the rounded result is less than, equal to, or greater than the exact result.
    /// Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function sets
    /// the [`BigExponentFloat`] to `NaN` it also returns `Equal`.
    ///
    /// The products are computed exactly, so only one rounding occurs.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets xy-zw+\varepsilon.
    /// $$
    /// - If $xy-zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy-zw$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy-zw|\rfloor-p+1}$.
    /// - If $xy-zw$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |xy-zw|\rfloor-p}$.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n + m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is the sum of the
    /// significant bits of the inputs.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, LN_2, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.mul_sub_mul_prec_round_assign_val_ref_ref(
    ///         BigExponentFloat::from(E),
    ///         &BigExponentFloat::from(SQRT_2),
    ///         &BigExponentFloat::from(LN_2),
    ///         5,
    ///         Floor
    ///     ),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x7.8");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.mul_sub_mul_prec_round_assign_val_ref_ref(
    ///         BigExponentFloat::from(E),
    ///         &BigExponentFloat::from(SQRT_2),
    ///         &BigExponentFloat::from(LN_2),
    ///         5,
    ///         Ceiling
    ///     ),
    ///     Greater
    /// );
    /// assert_eq!(format!("{x:?}"), "0x7.c");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.mul_sub_mul_prec_round_assign_val_ref_ref(
    ///         BigExponentFloat::from(E),
    ///         &BigExponentFloat::from(SQRT_2),
    ///         &BigExponentFloat::from(LN_2),
    ///         5,
    ///         Nearest
    ///     ),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x7.8");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.mul_sub_mul_prec_round_assign_val_ref_ref(
    ///         BigExponentFloat::from(E),
    ///         &BigExponentFloat::from(SQRT_2),
    ///         &BigExponentFloat::from(LN_2),
    ///         20,
    ///         Floor
    ///     ),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x7.8f398");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.mul_sub_mul_prec_round_assign_val_ref_ref(
    ///         BigExponentFloat::from(E),
    ///         &BigExponentFloat::from(SQRT_2),
    ///         &BigExponentFloat::from(LN_2),
    ///         20,
    ///         Ceiling
    ///     ),
    ///     Greater
    /// );
    /// assert_eq!(format!("{x:?}"), "0x7.8f3a");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.mul_sub_mul_prec_round_assign_val_ref_ref(
    ///         BigExponentFloat::from(E),
    ///         &BigExponentFloat::from(SQRT_2),
    ///         &BigExponentFloat::from(LN_2),
    ///         20,
    ///         Nearest
    ///     ),
    ///     Greater
    /// );
    /// assert_eq!(format!("{x:?}"), "0x7.8f3a");
    /// ```
    #[inline]
    pub fn mul_sub_mul_prec_round_assign_val_ref_ref(
        &mut self,
        y: Self,
        z: &Self,
        w: &Self,
        prec: u64,
        rm: RoundingMode,
    ) -> Ordering {
        let x = core::mem::replace(self, Self::ZERO);
        let (result, o) = mul_add_mul_helper(
            Cow::Owned(x),
            Cow::Owned(y),
            Cow::Borrowed(z),
            Cow::Borrowed(w),
            true,
            Some((prec, rm)),
        );
        *self = result;
        o
    }

    /// Subtracts the product of two [`BigExponentFloat`]s from a [`BigExponentFloat`] multiplied by
    /// a [`BigExponentFloat`], in place, rounding the result to the specified precision and with
    /// the specified rounding mode. On the right-hand side, the second and third
    /// [`BigExponentFloat`]s are taken by value and the first by reference. An [`Ordering`] is
    /// returned, indicating whether the rounded result is less than, equal to, or greater than the
    /// exact result. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this
    /// function sets the [`BigExponentFloat`] to `NaN` it also returns `Equal`.
    ///
    /// The products are computed exactly, so only one rounding occurs.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets xy-zw+\varepsilon.
    /// $$
    /// - If $xy-zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy-zw$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy-zw|\rfloor-p+1}$.
    /// - If $xy-zw$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |xy-zw|\rfloor-p}$.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n + m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is the sum of the
    /// significant bits of the inputs.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, LN_2, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.mul_sub_mul_prec_round_assign_ref_val_val(
    ///         &BigExponentFloat::from(E),
    ///         BigExponentFloat::from(SQRT_2),
    ///         BigExponentFloat::from(LN_2),
    ///         5,
    ///         Floor
    ///     ),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x7.8");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.mul_sub_mul_prec_round_assign_ref_val_val(
    ///         &BigExponentFloat::from(E),
    ///         BigExponentFloat::from(SQRT_2),
    ///         BigExponentFloat::from(LN_2),
    ///         5,
    ///         Ceiling
    ///     ),
    ///     Greater
    /// );
    /// assert_eq!(format!("{x:?}"), "0x7.c");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.mul_sub_mul_prec_round_assign_ref_val_val(
    ///         &BigExponentFloat::from(E),
    ///         BigExponentFloat::from(SQRT_2),
    ///         BigExponentFloat::from(LN_2),
    ///         5,
    ///         Nearest
    ///     ),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x7.8");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.mul_sub_mul_prec_round_assign_ref_val_val(
    ///         &BigExponentFloat::from(E),
    ///         BigExponentFloat::from(SQRT_2),
    ///         BigExponentFloat::from(LN_2),
    ///         20,
    ///         Floor
    ///     ),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x7.8f398");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.mul_sub_mul_prec_round_assign_ref_val_val(
    ///         &BigExponentFloat::from(E),
    ///         BigExponentFloat::from(SQRT_2),
    ///         BigExponentFloat::from(LN_2),
    ///         20,
    ///         Ceiling
    ///     ),
    ///     Greater
    /// );
    /// assert_eq!(format!("{x:?}"), "0x7.8f3a");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.mul_sub_mul_prec_round_assign_ref_val_val(
    ///         &BigExponentFloat::from(E),
    ///         BigExponentFloat::from(SQRT_2),
    ///         BigExponentFloat::from(LN_2),
    ///         20,
    ///         Nearest
    ///     ),
    ///     Greater
    /// );
    /// assert_eq!(format!("{x:?}"), "0x7.8f3a");
    /// ```
    #[inline]
    pub fn mul_sub_mul_prec_round_assign_ref_val_val(
        &mut self,
        y: &Self,
        z: Self,
        w: Self,
        prec: u64,
        rm: RoundingMode,
    ) -> Ordering {
        let x = core::mem::replace(self, Self::ZERO);
        let (result, o) = mul_add_mul_helper(
            Cow::Owned(x),
            Cow::Borrowed(y),
            Cow::Owned(z),
            Cow::Owned(w),
            true,
            Some((prec, rm)),
        );
        *self = result;
        o
    }

    /// Subtracts the product of two [`BigExponentFloat`]s from a [`BigExponentFloat`] multiplied by
    /// a [`BigExponentFloat`], in place, rounding the result to the specified precision and with
    /// the specified rounding mode. On the right-hand side, the second [`BigExponentFloat`] is
    /// taken by value and the first and third by reference. An [`Ordering`] is returned, indicating
    /// whether the rounded result is less than, equal to, or greater than the exact result.
    /// Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function sets
    /// the [`BigExponentFloat`] to `NaN` it also returns `Equal`.
    ///
    /// The products are computed exactly, so only one rounding occurs.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets xy-zw+\varepsilon.
    /// $$
    /// - If $xy-zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy-zw$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy-zw|\rfloor-p+1}$.
    /// - If $xy-zw$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |xy-zw|\rfloor-p}$.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n + m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is the sum of the
    /// significant bits of the inputs.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, LN_2, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.mul_sub_mul_prec_round_assign_ref_val_ref(
    ///         &BigExponentFloat::from(E),
    ///         BigExponentFloat::from(SQRT_2),
    ///         &BigExponentFloat::from(LN_2),
    ///         5,
    ///         Floor
    ///     ),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x7.8");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.mul_sub_mul_prec_round_assign_ref_val_ref(
    ///         &BigExponentFloat::from(E),
    ///         BigExponentFloat::from(SQRT_2),
    ///         &BigExponentFloat::from(LN_2),
    ///         5,
    ///         Ceiling
    ///     ),
    ///     Greater
    /// );
    /// assert_eq!(format!("{x:?}"), "0x7.c");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.mul_sub_mul_prec_round_assign_ref_val_ref(
    ///         &BigExponentFloat::from(E),
    ///         BigExponentFloat::from(SQRT_2),
    ///         &BigExponentFloat::from(LN_2),
    ///         5,
    ///         Nearest
    ///     ),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x7.8");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.mul_sub_mul_prec_round_assign_ref_val_ref(
    ///         &BigExponentFloat::from(E),
    ///         BigExponentFloat::from(SQRT_2),
    ///         &BigExponentFloat::from(LN_2),
    ///         20,
    ///         Floor
    ///     ),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x7.8f398");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.mul_sub_mul_prec_round_assign_ref_val_ref(
    ///         &BigExponentFloat::from(E),
    ///         BigExponentFloat::from(SQRT_2),
    ///         &BigExponentFloat::from(LN_2),
    ///         20,
    ///         Ceiling
    ///     ),
    ///     Greater
    /// );
    /// assert_eq!(format!("{x:?}"), "0x7.8f3a");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.mul_sub_mul_prec_round_assign_ref_val_ref(
    ///         &BigExponentFloat::from(E),
    ///         BigExponentFloat::from(SQRT_2),
    ///         &BigExponentFloat::from(LN_2),
    ///         20,
    ///         Nearest
    ///     ),
    ///     Greater
    /// );
    /// assert_eq!(format!("{x:?}"), "0x7.8f3a");
    /// ```
    #[inline]
    pub fn mul_sub_mul_prec_round_assign_ref_val_ref(
        &mut self,
        y: &Self,
        z: Self,
        w: &Self,
        prec: u64,
        rm: RoundingMode,
    ) -> Ordering {
        let x = core::mem::replace(self, Self::ZERO);
        let (result, o) = mul_add_mul_helper(
            Cow::Owned(x),
            Cow::Borrowed(y),
            Cow::Owned(z),
            Cow::Borrowed(w),
            true,
            Some((prec, rm)),
        );
        *self = result;
        o
    }

    /// Subtracts the product of two [`BigExponentFloat`]s from a [`BigExponentFloat`] multiplied by
    /// a [`BigExponentFloat`], in place, rounding the result to the specified precision and with
    /// the specified rounding mode. On the right-hand side, the third [`BigExponentFloat`] is taken
    /// by value and the first and second by reference. An [`Ordering`] is returned, indicating
    /// whether the rounded result is less than, equal to, or greater than the exact result.
    /// Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function sets
    /// the [`BigExponentFloat`] to `NaN` it also returns `Equal`.
    ///
    /// The products are computed exactly, so only one rounding occurs.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets xy-zw+\varepsilon.
    /// $$
    /// - If $xy-zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy-zw$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy-zw|\rfloor-p+1}$.
    /// - If $xy-zw$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |xy-zw|\rfloor-p}$.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n + m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is the sum of the
    /// significant bits of the inputs.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, LN_2, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.mul_sub_mul_prec_round_assign_ref_ref_val(
    ///         &BigExponentFloat::from(E),
    ///         &BigExponentFloat::from(SQRT_2),
    ///         BigExponentFloat::from(LN_2),
    ///         5,
    ///         Floor
    ///     ),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x7.8");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.mul_sub_mul_prec_round_assign_ref_ref_val(
    ///         &BigExponentFloat::from(E),
    ///         &BigExponentFloat::from(SQRT_2),
    ///         BigExponentFloat::from(LN_2),
    ///         5,
    ///         Ceiling
    ///     ),
    ///     Greater
    /// );
    /// assert_eq!(format!("{x:?}"), "0x7.c");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.mul_sub_mul_prec_round_assign_ref_ref_val(
    ///         &BigExponentFloat::from(E),
    ///         &BigExponentFloat::from(SQRT_2),
    ///         BigExponentFloat::from(LN_2),
    ///         5,
    ///         Nearest
    ///     ),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x7.8");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.mul_sub_mul_prec_round_assign_ref_ref_val(
    ///         &BigExponentFloat::from(E),
    ///         &BigExponentFloat::from(SQRT_2),
    ///         BigExponentFloat::from(LN_2),
    ///         20,
    ///         Floor
    ///     ),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x7.8f398");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.mul_sub_mul_prec_round_assign_ref_ref_val(
    ///         &BigExponentFloat::from(E),
    ///         &BigExponentFloat::from(SQRT_2),
    ///         BigExponentFloat::from(LN_2),
    ///         20,
    ///         Ceiling
    ///     ),
    ///     Greater
    /// );
    /// assert_eq!(format!("{x:?}"), "0x7.8f3a");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.mul_sub_mul_prec_round_assign_ref_ref_val(
    ///         &BigExponentFloat::from(E),
    ///         &BigExponentFloat::from(SQRT_2),
    ///         BigExponentFloat::from(LN_2),
    ///         20,
    ///         Nearest
    ///     ),
    ///     Greater
    /// );
    /// assert_eq!(format!("{x:?}"), "0x7.8f3a");
    /// ```
    #[inline]
    pub fn mul_sub_mul_prec_round_assign_ref_ref_val(
        &mut self,
        y: &Self,
        z: &Self,
        w: Self,
        prec: u64,
        rm: RoundingMode,
    ) -> Ordering {
        let x = core::mem::replace(self, Self::ZERO);
        let (result, o) = mul_add_mul_helper(
            Cow::Owned(x),
            Cow::Borrowed(y),
            Cow::Borrowed(z),
            Cow::Owned(w),
            true,
            Some((prec, rm)),
        );
        *self = result;
        o
    }

    /// Subtracts the product of two [`BigExponentFloat`]s from a [`BigExponentFloat`] multiplied by
    /// a [`BigExponentFloat`], in place, rounding the result to the specified precision and with
    /// the specified rounding mode. The [`BigExponentFloat`]s on the right-hand side are taken by
    /// reference. An [`Ordering`] is returned, indicating whether the rounded result is less than,
    /// equal to, or greater than the exact result. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function sets the [`BigExponentFloat`] to `NaN` it also
    /// returns `Equal`.
    ///
    /// The products are computed exactly, so only one rounding occurs.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets xy-zw+\varepsilon.
    /// $$
    /// - If $xy-zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy-zw$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy-zw|\rfloor-p+1}$.
    /// - If $xy-zw$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |xy-zw|\rfloor-p}$.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n + m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is the sum of the
    /// significant bits of the inputs.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, LN_2, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.mul_sub_mul_prec_round_assign_ref_ref_ref(
    ///         &BigExponentFloat::from(E),
    ///         &BigExponentFloat::from(SQRT_2),
    ///         &BigExponentFloat::from(LN_2),
    ///         5,
    ///         Floor
    ///     ),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x7.8");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.mul_sub_mul_prec_round_assign_ref_ref_ref(
    ///         &BigExponentFloat::from(E),
    ///         &BigExponentFloat::from(SQRT_2),
    ///         &BigExponentFloat::from(LN_2),
    ///         5,
    ///         Ceiling
    ///     ),
    ///     Greater
    /// );
    /// assert_eq!(format!("{x:?}"), "0x7.c");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.mul_sub_mul_prec_round_assign_ref_ref_ref(
    ///         &BigExponentFloat::from(E),
    ///         &BigExponentFloat::from(SQRT_2),
    ///         &BigExponentFloat::from(LN_2),
    ///         5,
    ///         Nearest
    ///     ),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x7.8");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.mul_sub_mul_prec_round_assign_ref_ref_ref(
    ///         &BigExponentFloat::from(E),
    ///         &BigExponentFloat::from(SQRT_2),
    ///         &BigExponentFloat::from(LN_2),
    ///         20,
    ///         Floor
    ///     ),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x7.8f398");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.mul_sub_mul_prec_round_assign_ref_ref_ref(
    ///         &BigExponentFloat::from(E),
    ///         &BigExponentFloat::from(SQRT_2),
    ///         &BigExponentFloat::from(LN_2),
    ///         20,
    ///         Ceiling
    ///     ),
    ///     Greater
    /// );
    /// assert_eq!(format!("{x:?}"), "0x7.8f3a");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.mul_sub_mul_prec_round_assign_ref_ref_ref(
    ///         &BigExponentFloat::from(E),
    ///         &BigExponentFloat::from(SQRT_2),
    ///         &BigExponentFloat::from(LN_2),
    ///         20,
    ///         Nearest
    ///     ),
    ///     Greater
    /// );
    /// assert_eq!(format!("{x:?}"), "0x7.8f3a");
    /// ```
    #[inline]
    pub fn mul_sub_mul_prec_round_assign_ref_ref_ref(
        &mut self,
        y: &Self,
        z: &Self,
        w: &Self,
        prec: u64,
        rm: RoundingMode,
    ) -> Ordering {
        let x = core::mem::replace(self, Self::ZERO);
        let (result, o) = mul_add_mul_helper(
            Cow::Owned(x),
            Cow::Borrowed(y),
            Cow::Borrowed(z),
            Cow::Borrowed(w),
            true,
            Some((prec, rm)),
        );
        *self = result;
        o
    }

    /// Subtracts the product of two [`BigExponentFloat`]s from the product of two other
    /// [`BigExponentFloat`]s, rounding the result to the nearest value of the specified precision.
    /// All four [`BigExponentFloat`]s are taken by value. An [`Ordering`] is also returned,
    /// indicating whether the rounded result is less than, equal to, or greater than the exact
    /// result. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// The products are computed exactly, so only one rounding occurs.
    ///
    /// $$
    /// f(x,y,z,w,p) = xy-zw+\varepsilon.
    /// $$
    /// - If $xy-zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy-zw$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |xy-zw|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,y,z,w,p)=\text{NaN}$ if any argument is NaN, if either product multiplies an infinity
    ///   by zero, or if $xy$ and $zw$ are infinities of the same sign
    /// - $f(x,y,z,w,p)=\infty$ if $xy$ is $\infty$ or $zw$ is $-\infty$, and the other is not the
    ///   same infinity
    /// - $f(x,y,z,w,p)=-\infty$ if $xy$ is $-\infty$ or $zw$ is $\infty$, and the other is not the
    ///   same infinity
    /// - $f(x,y,z,w,p)=0$ if $xy=zw$ and both are finite
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`BigExponentFloat::mul_sub_mul_prec_round`] instead. If you want the exact result, consider
    /// using [`MulSubMul::mul_sub_mul`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n + m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is the sum of the
    /// significant bits of the inputs.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, LN_2, PI, SQRT_2};
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (result, o) = BigExponentFloat::from(PI).mul_sub_mul_prec(
    ///     BigExponentFloat::from(E),
    ///     BigExponentFloat::from(SQRT_2),
    ///     BigExponentFloat::from(LN_2),
    ///     5,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x7.8");
    /// assert_eq!(o, Less);
    ///
    /// let (result, o) = BigExponentFloat::from(PI).mul_sub_mul_prec(
    ///     BigExponentFloat::from(E),
    ///     BigExponentFloat::from(SQRT_2),
    ///     BigExponentFloat::from(LN_2),
    ///     20,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x7.8f3a");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn mul_sub_mul_prec(self, y: Self, z: Self, w: Self, prec: u64) -> (Self, Ordering) {
        mul_add_mul_helper(
            Cow::Owned(self),
            Cow::Owned(y),
            Cow::Owned(z),
            Cow::Owned(w),
            true,
            Some((prec, Nearest)),
        )
    }

    /// Subtracts the product of two [`BigExponentFloat`]s from the product of two other
    /// [`BigExponentFloat`]s, rounding the result to the nearest value of the specified precision.
    /// The first, second, and third [`BigExponentFloat`]s are taken by value and the fourth by
    /// reference. An [`Ordering`] is also returned, indicating whether the rounded result is less
    /// than, equal to, or greater than the exact result. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The products are computed exactly, so only one rounding occurs.
    ///
    /// $$
    /// f(x,y,z,w,p) = xy-zw+\varepsilon.
    /// $$
    /// - If $xy-zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy-zw$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |xy-zw|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,y,z,w,p)=\text{NaN}$ if any argument is NaN, if either product multiplies an infinity
    ///   by zero, or if $xy$ and $zw$ are infinities of the same sign
    /// - $f(x,y,z,w,p)=\infty$ if $xy$ is $\infty$ or $zw$ is $-\infty$, and the other is not the
    ///   same infinity
    /// - $f(x,y,z,w,p)=-\infty$ if $xy$ is $-\infty$ or $zw$ is $\infty$, and the other is not the
    ///   same infinity
    /// - $f(x,y,z,w,p)=0$ if $xy=zw$ and both are finite
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n + m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is the sum of the
    /// significant bits of the inputs.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, LN_2, PI, SQRT_2};
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (result, o) = BigExponentFloat::from(PI).mul_sub_mul_prec_val_val_val_ref(
    ///     BigExponentFloat::from(E),
    ///     BigExponentFloat::from(SQRT_2),
    ///     &BigExponentFloat::from(LN_2),
    ///     5,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x7.8");
    /// assert_eq!(o, Less);
    ///
    /// let (result, o) = BigExponentFloat::from(PI).mul_sub_mul_prec_val_val_val_ref(
    ///     BigExponentFloat::from(E),
    ///     BigExponentFloat::from(SQRT_2),
    ///     &BigExponentFloat::from(LN_2),
    ///     20,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x7.8f3a");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn mul_sub_mul_prec_val_val_val_ref(
        self,
        y: Self,
        z: Self,
        w: &Self,
        prec: u64,
    ) -> (Self, Ordering) {
        mul_add_mul_helper(
            Cow::Owned(self),
            Cow::Owned(y),
            Cow::Owned(z),
            Cow::Borrowed(w),
            true,
            Some((prec, Nearest)),
        )
    }

    /// Subtracts the product of two [`BigExponentFloat`]s from the product of two other
    /// [`BigExponentFloat`]s, rounding the result to the nearest value of the specified precision.
    /// The first, second, and fourth [`BigExponentFloat`]s are taken by value and the third by
    /// reference. An [`Ordering`] is also returned, indicating whether the rounded result is less
    /// than, equal to, or greater than the exact result. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The products are computed exactly, so only one rounding occurs.
    ///
    /// $$
    /// f(x,y,z,w,p) = xy-zw+\varepsilon.
    /// $$
    /// - If $xy-zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy-zw$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |xy-zw|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,y,z,w,p)=\text{NaN}$ if any argument is NaN, if either product multiplies an infinity
    ///   by zero, or if $xy$ and $zw$ are infinities of the same sign
    /// - $f(x,y,z,w,p)=\infty$ if $xy$ is $\infty$ or $zw$ is $-\infty$, and the other is not the
    ///   same infinity
    /// - $f(x,y,z,w,p)=-\infty$ if $xy$ is $-\infty$ or $zw$ is $\infty$, and the other is not the
    ///   same infinity
    /// - $f(x,y,z,w,p)=0$ if $xy=zw$ and both are finite
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n + m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is the sum of the
    /// significant bits of the inputs.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, LN_2, PI, SQRT_2};
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (result, o) = BigExponentFloat::from(PI).mul_sub_mul_prec_val_val_ref_val(
    ///     BigExponentFloat::from(E),
    ///     &BigExponentFloat::from(SQRT_2),
    ///     BigExponentFloat::from(LN_2),
    ///     5,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x7.8");
    /// assert_eq!(o, Less);
    ///
    /// let (result, o) = BigExponentFloat::from(PI).mul_sub_mul_prec_val_val_ref_val(
    ///     BigExponentFloat::from(E),
    ///     &BigExponentFloat::from(SQRT_2),
    ///     BigExponentFloat::from(LN_2),
    ///     20,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x7.8f3a");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn mul_sub_mul_prec_val_val_ref_val(
        self,
        y: Self,
        z: &Self,
        w: Self,
        prec: u64,
    ) -> (Self, Ordering) {
        mul_add_mul_helper(
            Cow::Owned(self),
            Cow::Owned(y),
            Cow::Borrowed(z),
            Cow::Owned(w),
            true,
            Some((prec, Nearest)),
        )
    }

    /// Subtracts the product of two [`BigExponentFloat`]s from the product of two other
    /// [`BigExponentFloat`]s, rounding the result to the nearest value of the specified precision.
    /// The first and second [`BigExponentFloat`]s are taken by value and the third and fourth by
    /// reference. An [`Ordering`] is also returned, indicating whether the rounded result is less
    /// than, equal to, or greater than the exact result. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The products are computed exactly, so only one rounding occurs.
    ///
    /// $$
    /// f(x,y,z,w,p) = xy-zw+\varepsilon.
    /// $$
    /// - If $xy-zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy-zw$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |xy-zw|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,y,z,w,p)=\text{NaN}$ if any argument is NaN, if either product multiplies an infinity
    ///   by zero, or if $xy$ and $zw$ are infinities of the same sign
    /// - $f(x,y,z,w,p)=\infty$ if $xy$ is $\infty$ or $zw$ is $-\infty$, and the other is not the
    ///   same infinity
    /// - $f(x,y,z,w,p)=-\infty$ if $xy$ is $-\infty$ or $zw$ is $\infty$, and the other is not the
    ///   same infinity
    /// - $f(x,y,z,w,p)=0$ if $xy=zw$ and both are finite
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n + m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is the sum of the
    /// significant bits of the inputs.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, LN_2, PI, SQRT_2};
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (result, o) = BigExponentFloat::from(PI).mul_sub_mul_prec_val_val_ref_ref(
    ///     BigExponentFloat::from(E),
    ///     &BigExponentFloat::from(SQRT_2),
    ///     &BigExponentFloat::from(LN_2),
    ///     5,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x7.8");
    /// assert_eq!(o, Less);
    ///
    /// let (result, o) = BigExponentFloat::from(PI).mul_sub_mul_prec_val_val_ref_ref(
    ///     BigExponentFloat::from(E),
    ///     &BigExponentFloat::from(SQRT_2),
    ///     &BigExponentFloat::from(LN_2),
    ///     20,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x7.8f3a");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn mul_sub_mul_prec_val_val_ref_ref(
        self,
        y: Self,
        z: &Self,
        w: &Self,
        prec: u64,
    ) -> (Self, Ordering) {
        mul_add_mul_helper(
            Cow::Owned(self),
            Cow::Owned(y),
            Cow::Borrowed(z),
            Cow::Borrowed(w),
            true,
            Some((prec, Nearest)),
        )
    }

    /// Subtracts the product of two [`BigExponentFloat`]s from the product of two other
    /// [`BigExponentFloat`]s, rounding the result to the nearest value of the specified precision.
    /// The first, third, and fourth [`BigExponentFloat`]s are taken by value and the second by
    /// reference. An [`Ordering`] is also returned, indicating whether the rounded result is less
    /// than, equal to, or greater than the exact result. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The products are computed exactly, so only one rounding occurs.
    ///
    /// $$
    /// f(x,y,z,w,p) = xy-zw+\varepsilon.
    /// $$
    /// - If $xy-zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy-zw$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |xy-zw|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,y,z,w,p)=\text{NaN}$ if any argument is NaN, if either product multiplies an infinity
    ///   by zero, or if $xy$ and $zw$ are infinities of the same sign
    /// - $f(x,y,z,w,p)=\infty$ if $xy$ is $\infty$ or $zw$ is $-\infty$, and the other is not the
    ///   same infinity
    /// - $f(x,y,z,w,p)=-\infty$ if $xy$ is $-\infty$ or $zw$ is $\infty$, and the other is not the
    ///   same infinity
    /// - $f(x,y,z,w,p)=0$ if $xy=zw$ and both are finite
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n + m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is the sum of the
    /// significant bits of the inputs.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, LN_2, PI, SQRT_2};
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (result, o) = BigExponentFloat::from(PI).mul_sub_mul_prec_val_ref_val_val(
    ///     &BigExponentFloat::from(E),
    ///     BigExponentFloat::from(SQRT_2),
    ///     BigExponentFloat::from(LN_2),
    ///     5,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x7.8");
    /// assert_eq!(o, Less);
    ///
    /// let (result, o) = BigExponentFloat::from(PI).mul_sub_mul_prec_val_ref_val_val(
    ///     &BigExponentFloat::from(E),
    ///     BigExponentFloat::from(SQRT_2),
    ///     BigExponentFloat::from(LN_2),
    ///     20,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x7.8f3a");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn mul_sub_mul_prec_val_ref_val_val(
        self,
        y: &Self,
        z: Self,
        w: Self,
        prec: u64,
    ) -> (Self, Ordering) {
        mul_add_mul_helper(
            Cow::Owned(self),
            Cow::Borrowed(y),
            Cow::Owned(z),
            Cow::Owned(w),
            true,
            Some((prec, Nearest)),
        )
    }

    /// Subtracts the product of two [`BigExponentFloat`]s from the product of two other
    /// [`BigExponentFloat`]s, rounding the result to the nearest value of the specified precision.
    /// The first and third [`BigExponentFloat`]s are taken by value and the second and fourth by
    /// reference. An [`Ordering`] is also returned, indicating whether the rounded result is less
    /// than, equal to, or greater than the exact result. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The products are computed exactly, so only one rounding occurs.
    ///
    /// $$
    /// f(x,y,z,w,p) = xy-zw+\varepsilon.
    /// $$
    /// - If $xy-zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy-zw$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |xy-zw|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,y,z,w,p)=\text{NaN}$ if any argument is NaN, if either product multiplies an infinity
    ///   by zero, or if $xy$ and $zw$ are infinities of the same sign
    /// - $f(x,y,z,w,p)=\infty$ if $xy$ is $\infty$ or $zw$ is $-\infty$, and the other is not the
    ///   same infinity
    /// - $f(x,y,z,w,p)=-\infty$ if $xy$ is $-\infty$ or $zw$ is $\infty$, and the other is not the
    ///   same infinity
    /// - $f(x,y,z,w,p)=0$ if $xy=zw$ and both are finite
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n + m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is the sum of the
    /// significant bits of the inputs.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, LN_2, PI, SQRT_2};
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (result, o) = BigExponentFloat::from(PI).mul_sub_mul_prec_val_ref_val_ref(
    ///     &BigExponentFloat::from(E),
    ///     BigExponentFloat::from(SQRT_2),
    ///     &BigExponentFloat::from(LN_2),
    ///     5,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x7.8");
    /// assert_eq!(o, Less);
    ///
    /// let (result, o) = BigExponentFloat::from(PI).mul_sub_mul_prec_val_ref_val_ref(
    ///     &BigExponentFloat::from(E),
    ///     BigExponentFloat::from(SQRT_2),
    ///     &BigExponentFloat::from(LN_2),
    ///     20,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x7.8f3a");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn mul_sub_mul_prec_val_ref_val_ref(
        self,
        y: &Self,
        z: Self,
        w: &Self,
        prec: u64,
    ) -> (Self, Ordering) {
        mul_add_mul_helper(
            Cow::Owned(self),
            Cow::Borrowed(y),
            Cow::Owned(z),
            Cow::Borrowed(w),
            true,
            Some((prec, Nearest)),
        )
    }

    /// Subtracts the product of two [`BigExponentFloat`]s from the product of two other
    /// [`BigExponentFloat`]s, rounding the result to the nearest value of the specified precision.
    /// The first and fourth [`BigExponentFloat`]s are taken by value and the second and third by
    /// reference. An [`Ordering`] is also returned, indicating whether the rounded result is less
    /// than, equal to, or greater than the exact result. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The products are computed exactly, so only one rounding occurs.
    ///
    /// $$
    /// f(x,y,z,w,p) = xy-zw+\varepsilon.
    /// $$
    /// - If $xy-zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy-zw$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |xy-zw|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,y,z,w,p)=\text{NaN}$ if any argument is NaN, if either product multiplies an infinity
    ///   by zero, or if $xy$ and $zw$ are infinities of the same sign
    /// - $f(x,y,z,w,p)=\infty$ if $xy$ is $\infty$ or $zw$ is $-\infty$, and the other is not the
    ///   same infinity
    /// - $f(x,y,z,w,p)=-\infty$ if $xy$ is $-\infty$ or $zw$ is $\infty$, and the other is not the
    ///   same infinity
    /// - $f(x,y,z,w,p)=0$ if $xy=zw$ and both are finite
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n + m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is the sum of the
    /// significant bits of the inputs.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, LN_2, PI, SQRT_2};
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (result, o) = BigExponentFloat::from(PI).mul_sub_mul_prec_val_ref_ref_val(
    ///     &BigExponentFloat::from(E),
    ///     &BigExponentFloat::from(SQRT_2),
    ///     BigExponentFloat::from(LN_2),
    ///     5,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x7.8");
    /// assert_eq!(o, Less);
    ///
    /// let (result, o) = BigExponentFloat::from(PI).mul_sub_mul_prec_val_ref_ref_val(
    ///     &BigExponentFloat::from(E),
    ///     &BigExponentFloat::from(SQRT_2),
    ///     BigExponentFloat::from(LN_2),
    ///     20,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x7.8f3a");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn mul_sub_mul_prec_val_ref_ref_val(
        self,
        y: &Self,
        z: &Self,
        w: Self,
        prec: u64,
    ) -> (Self, Ordering) {
        mul_add_mul_helper(
            Cow::Owned(self),
            Cow::Borrowed(y),
            Cow::Borrowed(z),
            Cow::Owned(w),
            true,
            Some((prec, Nearest)),
        )
    }

    /// Subtracts the product of two [`BigExponentFloat`]s from the product of two other
    /// [`BigExponentFloat`]s, rounding the result to the nearest value of the specified precision.
    /// The first [`BigExponentFloat`] is taken by value and the second, third, and fourth by
    /// reference. An [`Ordering`] is also returned, indicating whether the rounded result is less
    /// than, equal to, or greater than the exact result. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The products are computed exactly, so only one rounding occurs.
    ///
    /// $$
    /// f(x,y,z,w,p) = xy-zw+\varepsilon.
    /// $$
    /// - If $xy-zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy-zw$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |xy-zw|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,y,z,w,p)=\text{NaN}$ if any argument is NaN, if either product multiplies an infinity
    ///   by zero, or if $xy$ and $zw$ are infinities of the same sign
    /// - $f(x,y,z,w,p)=\infty$ if $xy$ is $\infty$ or $zw$ is $-\infty$, and the other is not the
    ///   same infinity
    /// - $f(x,y,z,w,p)=-\infty$ if $xy$ is $-\infty$ or $zw$ is $\infty$, and the other is not the
    ///   same infinity
    /// - $f(x,y,z,w,p)=0$ if $xy=zw$ and both are finite
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n + m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is the sum of the
    /// significant bits of the inputs.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, LN_2, PI, SQRT_2};
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (result, o) = BigExponentFloat::from(PI).mul_sub_mul_prec_val_ref_ref_ref(
    ///     &BigExponentFloat::from(E),
    ///     &BigExponentFloat::from(SQRT_2),
    ///     &BigExponentFloat::from(LN_2),
    ///     5,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x7.8");
    /// assert_eq!(o, Less);
    ///
    /// let (result, o) = BigExponentFloat::from(PI).mul_sub_mul_prec_val_ref_ref_ref(
    ///     &BigExponentFloat::from(E),
    ///     &BigExponentFloat::from(SQRT_2),
    ///     &BigExponentFloat::from(LN_2),
    ///     20,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x7.8f3a");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn mul_sub_mul_prec_val_ref_ref_ref(
        self,
        y: &Self,
        z: &Self,
        w: &Self,
        prec: u64,
    ) -> (Self, Ordering) {
        mul_add_mul_helper(
            Cow::Owned(self),
            Cow::Borrowed(y),
            Cow::Borrowed(z),
            Cow::Borrowed(w),
            true,
            Some((prec, Nearest)),
        )
    }

    /// Subtracts the product of two [`BigExponentFloat`]s from the product of two other
    /// [`BigExponentFloat`]s, rounding the result to the nearest value of the specified precision.
    /// All four [`BigExponentFloat`]s are taken by reference. An [`Ordering`] is also returned,
    /// indicating whether the rounded result is less than, equal to, or greater than the exact
    /// result. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// The products are computed exactly, so only one rounding occurs.
    ///
    /// $$
    /// f(x,y,z,w,p) = xy-zw+\varepsilon.
    /// $$
    /// - If $xy-zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy-zw$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |xy-zw|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,y,z,w,p)=\text{NaN}$ if any argument is NaN, if either product multiplies an infinity
    ///   by zero, or if $xy$ and $zw$ are infinities of the same sign
    /// - $f(x,y,z,w,p)=\infty$ if $xy$ is $\infty$ or $zw$ is $-\infty$, and the other is not the
    ///   same infinity
    /// - $f(x,y,z,w,p)=-\infty$ if $xy$ is $-\infty$ or $zw$ is $\infty$, and the other is not the
    ///   same infinity
    /// - $f(x,y,z,w,p)=0$ if $xy=zw$ and both are finite
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n + m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is the sum of the
    /// significant bits of the inputs.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, LN_2, PI, SQRT_2};
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (result, o) = (&BigExponentFloat::from(PI)).mul_sub_mul_prec_ref_ref_ref_ref(
    ///     &BigExponentFloat::from(E),
    ///     &BigExponentFloat::from(SQRT_2),
    ///     &BigExponentFloat::from(LN_2),
    ///     5,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x7.8");
    /// assert_eq!(o, Less);
    ///
    /// let (result, o) = (&BigExponentFloat::from(PI)).mul_sub_mul_prec_ref_ref_ref_ref(
    ///     &BigExponentFloat::from(E),
    ///     &BigExponentFloat::from(SQRT_2),
    ///     &BigExponentFloat::from(LN_2),
    ///     20,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x7.8f3a");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn mul_sub_mul_prec_ref_ref_ref_ref(
        &self,
        y: &Self,
        z: &Self,
        w: &Self,
        prec: u64,
    ) -> (Self, Ordering) {
        mul_add_mul_helper(
            Cow::Borrowed(self),
            Cow::Borrowed(y),
            Cow::Borrowed(z),
            Cow::Borrowed(w),
            true,
            Some((prec, Nearest)),
        )
    }

    /// Subtracts the product of two [`BigExponentFloat`]s from a [`BigExponentFloat`] multiplied by
    /// a [`BigExponentFloat`], in place, rounding the result to the nearest value of the specified
    /// precision. The [`BigExponentFloat`]s on the right-hand side are taken by value. An
    /// [`Ordering`] is returned, indicating whether the rounded result is less than, equal to, or
    /// greater than the exact result. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function sets the [`BigExponentFloat`] to `NaN` it also
    /// returns `Equal`.
    ///
    /// The products are computed exactly, so only one rounding occurs.
    ///
    /// $$
    /// x \gets xy-zw+\varepsilon.
    /// $$
    /// - If $xy-zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy-zw$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |xy-zw|\rfloor-p}$.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n + m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is the sum of the
    /// significant bits of the inputs.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, LN_2, PI, SQRT_2};
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.mul_sub_mul_prec_assign(
    ///         BigExponentFloat::from(E),
    ///         BigExponentFloat::from(SQRT_2),
    ///         BigExponentFloat::from(LN_2),
    ///         5
    ///     ),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x7.8");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.mul_sub_mul_prec_assign(
    ///         BigExponentFloat::from(E),
    ///         BigExponentFloat::from(SQRT_2),
    ///         BigExponentFloat::from(LN_2),
    ///         20
    ///     ),
    ///     Greater
    /// );
    /// assert_eq!(format!("{x:?}"), "0x7.8f3a");
    /// ```
    #[inline]
    pub fn mul_sub_mul_prec_assign(&mut self, y: Self, z: Self, w: Self, prec: u64) -> Ordering {
        let x = core::mem::replace(self, Self::ZERO);
        let (result, o) = mul_add_mul_helper(
            Cow::Owned(x),
            Cow::Owned(y),
            Cow::Owned(z),
            Cow::Owned(w),
            true,
            Some((prec, Nearest)),
        );
        *self = result;
        o
    }

    /// Subtracts the product of two [`BigExponentFloat`]s from a [`BigExponentFloat`] multiplied by
    /// a [`BigExponentFloat`], in place, rounding the result to the nearest value of the specified
    /// precision. On the right-hand side, the first and second [`BigExponentFloat`]s are taken by
    /// value and the third by reference. An [`Ordering`] is returned, indicating whether the
    /// rounded result is less than, equal to, or greater than the exact result. Although `NaN`s are
    /// not comparable to any [`BigExponentFloat`], whenever this function sets the
    /// [`BigExponentFloat`] to `NaN` it also returns `Equal`.
    ///
    /// The products are computed exactly, so only one rounding occurs.
    ///
    /// $$
    /// x \gets xy-zw+\varepsilon.
    /// $$
    /// - If $xy-zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy-zw$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |xy-zw|\rfloor-p}$.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n + m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is the sum of the
    /// significant bits of the inputs.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, LN_2, PI, SQRT_2};
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.mul_sub_mul_prec_assign_val_val_ref(
    ///         BigExponentFloat::from(E),
    ///         BigExponentFloat::from(SQRT_2),
    ///         &BigExponentFloat::from(LN_2),
    ///         5
    ///     ),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x7.8");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.mul_sub_mul_prec_assign_val_val_ref(
    ///         BigExponentFloat::from(E),
    ///         BigExponentFloat::from(SQRT_2),
    ///         &BigExponentFloat::from(LN_2),
    ///         20
    ///     ),
    ///     Greater
    /// );
    /// assert_eq!(format!("{x:?}"), "0x7.8f3a");
    /// ```
    #[inline]
    pub fn mul_sub_mul_prec_assign_val_val_ref(
        &mut self,
        y: Self,
        z: Self,
        w: &Self,
        prec: u64,
    ) -> Ordering {
        let x = core::mem::replace(self, Self::ZERO);
        let (result, o) = mul_add_mul_helper(
            Cow::Owned(x),
            Cow::Owned(y),
            Cow::Owned(z),
            Cow::Borrowed(w),
            true,
            Some((prec, Nearest)),
        );
        *self = result;
        o
    }

    /// Subtracts the product of two [`BigExponentFloat`]s from a [`BigExponentFloat`] multiplied by
    /// a [`BigExponentFloat`], in place, rounding the result to the nearest value of the specified
    /// precision. On the right-hand side, the first and third [`BigExponentFloat`]s are taken by
    /// value and the second by reference. An [`Ordering`] is returned, indicating whether the
    /// rounded result is less than, equal to, or greater than the exact result. Although `NaN`s are
    /// not comparable to any [`BigExponentFloat`], whenever this function sets the
    /// [`BigExponentFloat`] to `NaN` it also returns `Equal`.
    ///
    /// The products are computed exactly, so only one rounding occurs.
    ///
    /// $$
    /// x \gets xy-zw+\varepsilon.
    /// $$
    /// - If $xy-zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy-zw$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |xy-zw|\rfloor-p}$.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n + m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is the sum of the
    /// significant bits of the inputs.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, LN_2, PI, SQRT_2};
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.mul_sub_mul_prec_assign_val_ref_val(
    ///         BigExponentFloat::from(E),
    ///         &BigExponentFloat::from(SQRT_2),
    ///         BigExponentFloat::from(LN_2),
    ///         5
    ///     ),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x7.8");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.mul_sub_mul_prec_assign_val_ref_val(
    ///         BigExponentFloat::from(E),
    ///         &BigExponentFloat::from(SQRT_2),
    ///         BigExponentFloat::from(LN_2),
    ///         20
    ///     ),
    ///     Greater
    /// );
    /// assert_eq!(format!("{x:?}"), "0x7.8f3a");
    /// ```
    #[inline]
    pub fn mul_sub_mul_prec_assign_val_ref_val(
        &mut self,
        y: Self,
        z: &Self,
        w: Self,
        prec: u64,
    ) -> Ordering {
        let x = core::mem::replace(self, Self::ZERO);
        let (result, o) = mul_add_mul_helper(
            Cow::Owned(x),
            Cow::Owned(y),
            Cow::Borrowed(z),
            Cow::Owned(w),
            true,
            Some((prec, Nearest)),
        );
        *self = result;
        o
    }

    /// Subtracts the product of two [`BigExponentFloat`]s from a [`BigExponentFloat`] multiplied by
    /// a [`BigExponentFloat`], in place, rounding the result to the nearest value of the specified
    /// precision. On the right-hand side, the first [`BigExponentFloat`] is taken by value and the
    /// second and third by reference. An [`Ordering`] is returned, indicating whether the rounded
    /// result is less than, equal to, or greater than the exact result. Although `NaN`s are not
    /// comparable to any [`BigExponentFloat`], whenever this function sets the [`BigExponentFloat`]
    /// to `NaN` it also returns `Equal`.
    ///
    /// The products are computed exactly, so only one rounding occurs.
    ///
    /// $$
    /// x \gets xy-zw+\varepsilon.
    /// $$
    /// - If $xy-zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy-zw$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |xy-zw|\rfloor-p}$.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n + m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is the sum of the
    /// significant bits of the inputs.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, LN_2, PI, SQRT_2};
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.mul_sub_mul_prec_assign_val_ref_ref(
    ///         BigExponentFloat::from(E),
    ///         &BigExponentFloat::from(SQRT_2),
    ///         &BigExponentFloat::from(LN_2),
    ///         5
    ///     ),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x7.8");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.mul_sub_mul_prec_assign_val_ref_ref(
    ///         BigExponentFloat::from(E),
    ///         &BigExponentFloat::from(SQRT_2),
    ///         &BigExponentFloat::from(LN_2),
    ///         20
    ///     ),
    ///     Greater
    /// );
    /// assert_eq!(format!("{x:?}"), "0x7.8f3a");
    /// ```
    #[inline]
    pub fn mul_sub_mul_prec_assign_val_ref_ref(
        &mut self,
        y: Self,
        z: &Self,
        w: &Self,
        prec: u64,
    ) -> Ordering {
        let x = core::mem::replace(self, Self::ZERO);
        let (result, o) = mul_add_mul_helper(
            Cow::Owned(x),
            Cow::Owned(y),
            Cow::Borrowed(z),
            Cow::Borrowed(w),
            true,
            Some((prec, Nearest)),
        );
        *self = result;
        o
    }

    /// Subtracts the product of two [`BigExponentFloat`]s from a [`BigExponentFloat`] multiplied by
    /// a [`BigExponentFloat`], in place, rounding the result to the nearest value of the specified
    /// precision. On the right-hand side, the second and third [`BigExponentFloat`]s are taken by
    /// value and the first by reference. An [`Ordering`] is returned, indicating whether the
    /// rounded result is less than, equal to, or greater than the exact result. Although `NaN`s are
    /// not comparable to any [`BigExponentFloat`], whenever this function sets the
    /// [`BigExponentFloat`] to `NaN` it also returns `Equal`.
    ///
    /// The products are computed exactly, so only one rounding occurs.
    ///
    /// $$
    /// x \gets xy-zw+\varepsilon.
    /// $$
    /// - If $xy-zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy-zw$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |xy-zw|\rfloor-p}$.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n + m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is the sum of the
    /// significant bits of the inputs.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, LN_2, PI, SQRT_2};
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.mul_sub_mul_prec_assign_ref_val_val(
    ///         &BigExponentFloat::from(E),
    ///         BigExponentFloat::from(SQRT_2),
    ///         BigExponentFloat::from(LN_2),
    ///         5
    ///     ),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x7.8");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.mul_sub_mul_prec_assign_ref_val_val(
    ///         &BigExponentFloat::from(E),
    ///         BigExponentFloat::from(SQRT_2),
    ///         BigExponentFloat::from(LN_2),
    ///         20
    ///     ),
    ///     Greater
    /// );
    /// assert_eq!(format!("{x:?}"), "0x7.8f3a");
    /// ```
    #[inline]
    pub fn mul_sub_mul_prec_assign_ref_val_val(
        &mut self,
        y: &Self,
        z: Self,
        w: Self,
        prec: u64,
    ) -> Ordering {
        let x = core::mem::replace(self, Self::ZERO);
        let (result, o) = mul_add_mul_helper(
            Cow::Owned(x),
            Cow::Borrowed(y),
            Cow::Owned(z),
            Cow::Owned(w),
            true,
            Some((prec, Nearest)),
        );
        *self = result;
        o
    }

    /// Subtracts the product of two [`BigExponentFloat`]s from a [`BigExponentFloat`] multiplied by
    /// a [`BigExponentFloat`], in place, rounding the result to the nearest value of the specified
    /// precision. On the right-hand side, the second [`BigExponentFloat`] is taken by value and the
    /// first and third by reference. An [`Ordering`] is returned, indicating whether the rounded
    /// result is less than, equal to, or greater than the exact result. Although `NaN`s are not
    /// comparable to any [`BigExponentFloat`], whenever this function sets the [`BigExponentFloat`]
    /// to `NaN` it also returns `Equal`.
    ///
    /// The products are computed exactly, so only one rounding occurs.
    ///
    /// $$
    /// x \gets xy-zw+\varepsilon.
    /// $$
    /// - If $xy-zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy-zw$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |xy-zw|\rfloor-p}$.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n + m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is the sum of the
    /// significant bits of the inputs.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, LN_2, PI, SQRT_2};
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.mul_sub_mul_prec_assign_ref_val_ref(
    ///         &BigExponentFloat::from(E),
    ///         BigExponentFloat::from(SQRT_2),
    ///         &BigExponentFloat::from(LN_2),
    ///         5
    ///     ),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x7.8");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.mul_sub_mul_prec_assign_ref_val_ref(
    ///         &BigExponentFloat::from(E),
    ///         BigExponentFloat::from(SQRT_2),
    ///         &BigExponentFloat::from(LN_2),
    ///         20
    ///     ),
    ///     Greater
    /// );
    /// assert_eq!(format!("{x:?}"), "0x7.8f3a");
    /// ```
    #[inline]
    pub fn mul_sub_mul_prec_assign_ref_val_ref(
        &mut self,
        y: &Self,
        z: Self,
        w: &Self,
        prec: u64,
    ) -> Ordering {
        let x = core::mem::replace(self, Self::ZERO);
        let (result, o) = mul_add_mul_helper(
            Cow::Owned(x),
            Cow::Borrowed(y),
            Cow::Owned(z),
            Cow::Borrowed(w),
            true,
            Some((prec, Nearest)),
        );
        *self = result;
        o
    }

    /// Subtracts the product of two [`BigExponentFloat`]s from a [`BigExponentFloat`] multiplied by
    /// a [`BigExponentFloat`], in place, rounding the result to the nearest value of the specified
    /// precision. On the right-hand side, the third [`BigExponentFloat`] is taken by value and the
    /// first and second by reference. An [`Ordering`] is returned, indicating whether the rounded
    /// result is less than, equal to, or greater than the exact result. Although `NaN`s are not
    /// comparable to any [`BigExponentFloat`], whenever this function sets the [`BigExponentFloat`]
    /// to `NaN` it also returns `Equal`.
    ///
    /// The products are computed exactly, so only one rounding occurs.
    ///
    /// $$
    /// x \gets xy-zw+\varepsilon.
    /// $$
    /// - If $xy-zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy-zw$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |xy-zw|\rfloor-p}$.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n + m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is the sum of the
    /// significant bits of the inputs.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, LN_2, PI, SQRT_2};
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.mul_sub_mul_prec_assign_ref_ref_val(
    ///         &BigExponentFloat::from(E),
    ///         &BigExponentFloat::from(SQRT_2),
    ///         BigExponentFloat::from(LN_2),
    ///         5
    ///     ),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x7.8");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.mul_sub_mul_prec_assign_ref_ref_val(
    ///         &BigExponentFloat::from(E),
    ///         &BigExponentFloat::from(SQRT_2),
    ///         BigExponentFloat::from(LN_2),
    ///         20
    ///     ),
    ///     Greater
    /// );
    /// assert_eq!(format!("{x:?}"), "0x7.8f3a");
    /// ```
    #[inline]
    pub fn mul_sub_mul_prec_assign_ref_ref_val(
        &mut self,
        y: &Self,
        z: &Self,
        w: Self,
        prec: u64,
    ) -> Ordering {
        let x = core::mem::replace(self, Self::ZERO);
        let (result, o) = mul_add_mul_helper(
            Cow::Owned(x),
            Cow::Borrowed(y),
            Cow::Borrowed(z),
            Cow::Owned(w),
            true,
            Some((prec, Nearest)),
        );
        *self = result;
        o
    }

    /// Subtracts the product of two [`BigExponentFloat`]s from a [`BigExponentFloat`] multiplied by
    /// a [`BigExponentFloat`], in place, rounding the result to the nearest value of the specified
    /// precision. The [`BigExponentFloat`]s on the right-hand side are taken by reference. An
    /// [`Ordering`] is returned, indicating whether the rounded result is less than, equal to, or
    /// greater than the exact result. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function sets the [`BigExponentFloat`] to `NaN` it also
    /// returns `Equal`.
    ///
    /// The products are computed exactly, so only one rounding occurs.
    ///
    /// $$
    /// x \gets xy-zw+\varepsilon.
    /// $$
    /// - If $xy-zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy-zw$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |xy-zw|\rfloor-p}$.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n + m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is the sum of the
    /// significant bits of the inputs.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, LN_2, PI, SQRT_2};
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.mul_sub_mul_prec_assign_ref_ref_ref(
    ///         &BigExponentFloat::from(E),
    ///         &BigExponentFloat::from(SQRT_2),
    ///         &BigExponentFloat::from(LN_2),
    ///         5
    ///     ),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x7.8");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.mul_sub_mul_prec_assign_ref_ref_ref(
    ///         &BigExponentFloat::from(E),
    ///         &BigExponentFloat::from(SQRT_2),
    ///         &BigExponentFloat::from(LN_2),
    ///         20
    ///     ),
    ///     Greater
    /// );
    /// assert_eq!(format!("{x:?}"), "0x7.8f3a");
    /// ```
    #[inline]
    pub fn mul_sub_mul_prec_assign_ref_ref_ref(
        &mut self,
        y: &Self,
        z: &Self,
        w: &Self,
        prec: u64,
    ) -> Ordering {
        let x = core::mem::replace(self, Self::ZERO);
        let (result, o) = mul_add_mul_helper(
            Cow::Owned(x),
            Cow::Borrowed(y),
            Cow::Borrowed(z),
            Cow::Borrowed(w),
            true,
            Some((prec, Nearest)),
        );
        *self = result;
        o
    }
}

impl MulSubMul<Self, Self, Self> for BigExponentFloat {
    type Output = Self;

    /// Subtracts the product of two [`BigExponentFloat`]s from the product of two other
    /// [`BigExponentFloat`]s. All four [`BigExponentFloat`]s are taken by value. The result is
    /// exact.
    ///
    /// $$
    /// f(x,y,z,w) = xy-zw.
    /// $$
    ///
    /// Special cases:
    /// - $f(x,y,z,w)=\text{NaN}$ if any argument is NaN, if either product multiplies an infinity
    ///   by zero, or if $xy$ and $zw$ are infinities of the same sign
    /// - $f(x,y,z,w)=\infty$ if $xy$ is $\infty$ or $zw$ is $-\infty$, and the other is not the
    ///   same infinity
    /// - $f(x,y,z,w)=-\infty$ if $xy$ is $-\infty$ or $zw$ is $\infty$, and the other is not the
    ///   same infinity
    /// - $f(x,y,z,w)=0$ if $xy=zw$ and both are finite
    ///
    /// When the two terms being added are finite and nonzero, the significand of the result spans
    /// the gap between their exponents. To bound its size, use
    /// [`BigExponentFloat::mul_sub_mul_prec_round`] or [`BigExponentFloat::mul_sub_mul_prec`]
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the sum of the significant bits of
    /// the inputs plus the absolute difference of the exponents of the two terms being added.
    ///
    /// # Panics
    /// Panics if the exponents of the two terms being added differ by $2^{64}$ or more and both
    /// terms are finite and nonzero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::MulSubMul;
    /// use malachite_float::BigExponentFloat;
    ///
    /// assert_eq!(
    ///     format!(
    ///         "{:?}",
    ///         BigExponentFloat::from(1.5).mul_sub_mul(
    ///             BigExponentFloat::from(0.25),
    ///             BigExponentFloat::from(3.0),
    ///             BigExponentFloat::from(-0.5)
    ///         )
    ///     ),
    ///     "0x1.e"
    /// );
    /// ```
    #[inline]
    fn mul_sub_mul(self, y: Self, z: Self, w: Self) -> Self {
        mul_add_mul_helper(
            Cow::Owned(self),
            Cow::Owned(y),
            Cow::Owned(z),
            Cow::Owned(w),
            true,
            None,
        )
        .0
    }
}

impl MulSubMul<Self, Self, &Self> for BigExponentFloat {
    type Output = Self;

    /// Subtracts the product of two [`BigExponentFloat`]s from the product of two other
    /// [`BigExponentFloat`]s. The first, second, and third [`BigExponentFloat`]s are taken by value
    /// and the fourth by reference. The result is exact.
    ///
    /// $$
    /// f(x,y,z,w) = xy-zw.
    /// $$
    ///
    /// Special cases:
    /// - $f(x,y,z,w)=\text{NaN}$ if any argument is NaN, if either product multiplies an infinity
    ///   by zero, or if $xy$ and $zw$ are infinities of the same sign
    /// - $f(x,y,z,w)=\infty$ if $xy$ is $\infty$ or $zw$ is $-\infty$, and the other is not the
    ///   same infinity
    /// - $f(x,y,z,w)=-\infty$ if $xy$ is $-\infty$ or $zw$ is $\infty$, and the other is not the
    ///   same infinity
    /// - $f(x,y,z,w)=0$ if $xy=zw$ and both are finite
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the sum of the significant bits of
    /// the inputs plus the absolute difference of the exponents of the two terms being added.
    ///
    /// # Panics
    /// Panics if the exponents of the two terms being added differ by $2^{64}$ or more and both
    /// terms are finite and nonzero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::MulSubMul;
    /// use malachite_float::BigExponentFloat;
    ///
    /// assert_eq!(
    ///     format!(
    ///         "{:?}",
    ///         BigExponentFloat::from(1.5).mul_sub_mul(
    ///             BigExponentFloat::from(0.25),
    ///             BigExponentFloat::from(3.0),
    ///             &BigExponentFloat::from(-0.5)
    ///         )
    ///     ),
    ///     "0x1.e"
    /// );
    /// ```
    #[inline]
    fn mul_sub_mul(self, y: Self, z: Self, w: &Self) -> Self {
        mul_add_mul_helper(
            Cow::Owned(self),
            Cow::Owned(y),
            Cow::Owned(z),
            Cow::Borrowed(w),
            true,
            None,
        )
        .0
    }
}

impl MulSubMul<Self, &Self, Self> for BigExponentFloat {
    type Output = Self;

    /// Subtracts the product of two [`BigExponentFloat`]s from the product of two other
    /// [`BigExponentFloat`]s. The first, second, and fourth [`BigExponentFloat`]s are taken by
    /// value and the third by reference. The result is exact.
    ///
    /// $$
    /// f(x,y,z,w) = xy-zw.
    /// $$
    ///
    /// Special cases:
    /// - $f(x,y,z,w)=\text{NaN}$ if any argument is NaN, if either product multiplies an infinity
    ///   by zero, or if $xy$ and $zw$ are infinities of the same sign
    /// - $f(x,y,z,w)=\infty$ if $xy$ is $\infty$ or $zw$ is $-\infty$, and the other is not the
    ///   same infinity
    /// - $f(x,y,z,w)=-\infty$ if $xy$ is $-\infty$ or $zw$ is $\infty$, and the other is not the
    ///   same infinity
    /// - $f(x,y,z,w)=0$ if $xy=zw$ and both are finite
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the sum of the significant bits of
    /// the inputs plus the absolute difference of the exponents of the two terms being added.
    ///
    /// # Panics
    /// Panics if the exponents of the two terms being added differ by $2^{64}$ or more and both
    /// terms are finite and nonzero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::MulSubMul;
    /// use malachite_float::BigExponentFloat;
    ///
    /// assert_eq!(
    ///     format!(
    ///         "{:?}",
    ///         BigExponentFloat::from(1.5).mul_sub_mul(
    ///             BigExponentFloat::from(0.25),
    ///             &BigExponentFloat::from(3.0),
    ///             BigExponentFloat::from(-0.5)
    ///         )
    ///     ),
    ///     "0x1.e"
    /// );
    /// ```
    #[inline]
    fn mul_sub_mul(self, y: Self, z: &Self, w: Self) -> Self {
        mul_add_mul_helper(
            Cow::Owned(self),
            Cow::Owned(y),
            Cow::Borrowed(z),
            Cow::Owned(w),
            true,
            None,
        )
        .0
    }
}

impl MulSubMul<Self, &Self, &Self> for BigExponentFloat {
    type Output = Self;

    /// Subtracts the product of two [`BigExponentFloat`]s from the product of two other
    /// [`BigExponentFloat`]s. The first and second [`BigExponentFloat`]s are taken by value and the
    /// third and fourth by reference. The result is exact.
    ///
    /// $$
    /// f(x,y,z,w) = xy-zw.
    /// $$
    ///
    /// Special cases:
    /// - $f(x,y,z,w)=\text{NaN}$ if any argument is NaN, if either product multiplies an infinity
    ///   by zero, or if $xy$ and $zw$ are infinities of the same sign
    /// - $f(x,y,z,w)=\infty$ if $xy$ is $\infty$ or $zw$ is $-\infty$, and the other is not the
    ///   same infinity
    /// - $f(x,y,z,w)=-\infty$ if $xy$ is $-\infty$ or $zw$ is $\infty$, and the other is not the
    ///   same infinity
    /// - $f(x,y,z,w)=0$ if $xy=zw$ and both are finite
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the sum of the significant bits of
    /// the inputs plus the absolute difference of the exponents of the two terms being added.
    ///
    /// # Panics
    /// Panics if the exponents of the two terms being added differ by $2^{64}$ or more and both
    /// terms are finite and nonzero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::MulSubMul;
    /// use malachite_float::BigExponentFloat;
    ///
    /// assert_eq!(
    ///     format!(
    ///         "{:?}",
    ///         BigExponentFloat::from(1.5).mul_sub_mul(
    ///             BigExponentFloat::from(0.25),
    ///             &BigExponentFloat::from(3.0),
    ///             &BigExponentFloat::from(-0.5)
    ///         )
    ///     ),
    ///     "0x1.e"
    /// );
    /// ```
    #[inline]
    fn mul_sub_mul(self, y: Self, z: &Self, w: &Self) -> Self {
        mul_add_mul_helper(
            Cow::Owned(self),
            Cow::Owned(y),
            Cow::Borrowed(z),
            Cow::Borrowed(w),
            true,
            None,
        )
        .0
    }
}

impl MulSubMul<&Self, Self, Self> for BigExponentFloat {
    type Output = Self;

    /// Subtracts the product of two [`BigExponentFloat`]s from the product of two other
    /// [`BigExponentFloat`]s. The first, third, and fourth [`BigExponentFloat`]s are taken by value
    /// and the second by reference. The result is exact.
    ///
    /// $$
    /// f(x,y,z,w) = xy-zw.
    /// $$
    ///
    /// Special cases:
    /// - $f(x,y,z,w)=\text{NaN}$ if any argument is NaN, if either product multiplies an infinity
    ///   by zero, or if $xy$ and $zw$ are infinities of the same sign
    /// - $f(x,y,z,w)=\infty$ if $xy$ is $\infty$ or $zw$ is $-\infty$, and the other is not the
    ///   same infinity
    /// - $f(x,y,z,w)=-\infty$ if $xy$ is $-\infty$ or $zw$ is $\infty$, and the other is not the
    ///   same infinity
    /// - $f(x,y,z,w)=0$ if $xy=zw$ and both are finite
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the sum of the significant bits of
    /// the inputs plus the absolute difference of the exponents of the two terms being added.
    ///
    /// # Panics
    /// Panics if the exponents of the two terms being added differ by $2^{64}$ or more and both
    /// terms are finite and nonzero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::MulSubMul;
    /// use malachite_float::BigExponentFloat;
    ///
    /// assert_eq!(
    ///     format!(
    ///         "{:?}",
    ///         BigExponentFloat::from(1.5).mul_sub_mul(
    ///             &BigExponentFloat::from(0.25),
    ///             BigExponentFloat::from(3.0),
    ///             BigExponentFloat::from(-0.5)
    ///         )
    ///     ),
    ///     "0x1.e"
    /// );
    /// ```
    #[inline]
    fn mul_sub_mul(self, y: &Self, z: Self, w: Self) -> Self {
        mul_add_mul_helper(
            Cow::Owned(self),
            Cow::Borrowed(y),
            Cow::Owned(z),
            Cow::Owned(w),
            true,
            None,
        )
        .0
    }
}

impl MulSubMul<&Self, Self, &Self> for BigExponentFloat {
    type Output = Self;

    /// Subtracts the product of two [`BigExponentFloat`]s from the product of two other
    /// [`BigExponentFloat`]s. The first and third [`BigExponentFloat`]s are taken by value and the
    /// second and fourth by reference. The result is exact.
    ///
    /// $$
    /// f(x,y,z,w) = xy-zw.
    /// $$
    ///
    /// Special cases:
    /// - $f(x,y,z,w)=\text{NaN}$ if any argument is NaN, if either product multiplies an infinity
    ///   by zero, or if $xy$ and $zw$ are infinities of the same sign
    /// - $f(x,y,z,w)=\infty$ if $xy$ is $\infty$ or $zw$ is $-\infty$, and the other is not the
    ///   same infinity
    /// - $f(x,y,z,w)=-\infty$ if $xy$ is $-\infty$ or $zw$ is $\infty$, and the other is not the
    ///   same infinity
    /// - $f(x,y,z,w)=0$ if $xy=zw$ and both are finite
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the sum of the significant bits of
    /// the inputs plus the absolute difference of the exponents of the two terms being added.
    ///
    /// # Panics
    /// Panics if the exponents of the two terms being added differ by $2^{64}$ or more and both
    /// terms are finite and nonzero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::MulSubMul;
    /// use malachite_float::BigExponentFloat;
    ///
    /// assert_eq!(
    ///     format!(
    ///         "{:?}",
    ///         BigExponentFloat::from(1.5).mul_sub_mul(
    ///             &BigExponentFloat::from(0.25),
    ///             BigExponentFloat::from(3.0),
    ///             &BigExponentFloat::from(-0.5)
    ///         )
    ///     ),
    ///     "0x1.e"
    /// );
    /// ```
    #[inline]
    fn mul_sub_mul(self, y: &Self, z: Self, w: &Self) -> Self {
        mul_add_mul_helper(
            Cow::Owned(self),
            Cow::Borrowed(y),
            Cow::Owned(z),
            Cow::Borrowed(w),
            true,
            None,
        )
        .0
    }
}

impl MulSubMul<&Self, &Self, Self> for BigExponentFloat {
    type Output = Self;

    /// Subtracts the product of two [`BigExponentFloat`]s from the product of two other
    /// [`BigExponentFloat`]s. The first and fourth [`BigExponentFloat`]s are taken by value and the
    /// second and third by reference. The result is exact.
    ///
    /// $$
    /// f(x,y,z,w) = xy-zw.
    /// $$
    ///
    /// Special cases:
    /// - $f(x,y,z,w)=\text{NaN}$ if any argument is NaN, if either product multiplies an infinity
    ///   by zero, or if $xy$ and $zw$ are infinities of the same sign
    /// - $f(x,y,z,w)=\infty$ if $xy$ is $\infty$ or $zw$ is $-\infty$, and the other is not the
    ///   same infinity
    /// - $f(x,y,z,w)=-\infty$ if $xy$ is $-\infty$ or $zw$ is $\infty$, and the other is not the
    ///   same infinity
    /// - $f(x,y,z,w)=0$ if $xy=zw$ and both are finite
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the sum of the significant bits of
    /// the inputs plus the absolute difference of the exponents of the two terms being added.
    ///
    /// # Panics
    /// Panics if the exponents of the two terms being added differ by $2^{64}$ or more and both
    /// terms are finite and nonzero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::MulSubMul;
    /// use malachite_float::BigExponentFloat;
    ///
    /// assert_eq!(
    ///     format!(
    ///         "{:?}",
    ///         BigExponentFloat::from(1.5).mul_sub_mul(
    ///             &BigExponentFloat::from(0.25),
    ///             &BigExponentFloat::from(3.0),
    ///             BigExponentFloat::from(-0.5)
    ///         )
    ///     ),
    ///     "0x1.e"
    /// );
    /// ```
    #[inline]
    fn mul_sub_mul(self, y: &Self, z: &Self, w: Self) -> Self {
        mul_add_mul_helper(
            Cow::Owned(self),
            Cow::Borrowed(y),
            Cow::Borrowed(z),
            Cow::Owned(w),
            true,
            None,
        )
        .0
    }
}

impl MulSubMul<&Self, &Self, &Self> for BigExponentFloat {
    type Output = Self;

    /// Subtracts the product of two [`BigExponentFloat`]s from the product of two other
    /// [`BigExponentFloat`]s. The first [`BigExponentFloat`] is taken by value and the second,
    /// third, and fourth by reference. The result is exact.
    ///
    /// $$
    /// f(x,y,z,w) = xy-zw.
    /// $$
    ///
    /// Special cases:
    /// - $f(x,y,z,w)=\text{NaN}$ if any argument is NaN, if either product multiplies an infinity
    ///   by zero, or if $xy$ and $zw$ are infinities of the same sign
    /// - $f(x,y,z,w)=\infty$ if $xy$ is $\infty$ or $zw$ is $-\infty$, and the other is not the
    ///   same infinity
    /// - $f(x,y,z,w)=-\infty$ if $xy$ is $-\infty$ or $zw$ is $\infty$, and the other is not the
    ///   same infinity
    /// - $f(x,y,z,w)=0$ if $xy=zw$ and both are finite
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the sum of the significant bits of
    /// the inputs plus the absolute difference of the exponents of the two terms being added.
    ///
    /// # Panics
    /// Panics if the exponents of the two terms being added differ by $2^{64}$ or more and both
    /// terms are finite and nonzero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::MulSubMul;
    /// use malachite_float::BigExponentFloat;
    ///
    /// assert_eq!(
    ///     format!(
    ///         "{:?}",
    ///         BigExponentFloat::from(1.5).mul_sub_mul(
    ///             &BigExponentFloat::from(0.25),
    ///             &BigExponentFloat::from(3.0),
    ///             &BigExponentFloat::from(-0.5)
    ///         )
    ///     ),
    ///     "0x1.e"
    /// );
    /// ```
    #[inline]
    fn mul_sub_mul(self, y: &Self, z: &Self, w: &Self) -> Self {
        mul_add_mul_helper(
            Cow::Owned(self),
            Cow::Borrowed(y),
            Cow::Borrowed(z),
            Cow::Borrowed(w),
            true,
            None,
        )
        .0
    }
}

impl MulSubMul<&BigExponentFloat, &BigExponentFloat, &BigExponentFloat> for &BigExponentFloat {
    type Output = BigExponentFloat;

    /// Subtracts the product of two [`BigExponentFloat`]s from the product of two other
    /// [`BigExponentFloat`]s. All four [`BigExponentFloat`]s are taken by reference. The result is
    /// exact.
    ///
    /// $$
    /// f(x,y,z,w) = xy-zw.
    /// $$
    ///
    /// Special cases:
    /// - $f(x,y,z,w)=\text{NaN}$ if any argument is NaN, if either product multiplies an infinity
    ///   by zero, or if $xy$ and $zw$ are infinities of the same sign
    /// - $f(x,y,z,w)=\infty$ if $xy$ is $\infty$ or $zw$ is $-\infty$, and the other is not the
    ///   same infinity
    /// - $f(x,y,z,w)=-\infty$ if $xy$ is $-\infty$ or $zw$ is $\infty$, and the other is not the
    ///   same infinity
    /// - $f(x,y,z,w)=0$ if $xy=zw$ and both are finite
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the sum of the significant bits of
    /// the inputs plus the absolute difference of the exponents of the two terms being added.
    ///
    /// # Panics
    /// Panics if the exponents of the two terms being added differ by $2^{64}$ or more and both
    /// terms are finite and nonzero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::MulSubMul;
    /// use malachite_float::BigExponentFloat;
    ///
    /// assert_eq!(
    ///     format!(
    ///         "{:?}",
    ///         (&BigExponentFloat::from(1.5)).mul_sub_mul(
    ///             &BigExponentFloat::from(0.25),
    ///             &BigExponentFloat::from(3.0),
    ///             &BigExponentFloat::from(-0.5)
    ///         )
    ///     ),
    ///     "0x1.e"
    /// );
    /// ```
    #[inline]
    fn mul_sub_mul(
        self,
        y: &BigExponentFloat,
        z: &BigExponentFloat,
        w: &BigExponentFloat,
    ) -> BigExponentFloat {
        mul_add_mul_helper(
            Cow::Borrowed(self),
            Cow::Borrowed(y),
            Cow::Borrowed(z),
            Cow::Borrowed(w),
            true,
            None,
        )
        .0
    }
}

impl MulSubMulAssign<Self, Self, Self> for BigExponentFloat {
    /// Subtracts the product of two [`BigExponentFloat`]s from a [`BigExponentFloat`] multiplied by
    /// a [`BigExponentFloat`], in place. The [`BigExponentFloat`]s on the right-hand side are taken
    /// by value. The result is exact.
    ///
    /// $$
    /// x \gets xy-zw.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the sum of the significant bits of
    /// the inputs plus the absolute difference of the exponents of the two terms being added.
    ///
    /// # Panics
    /// Panics if the exponents of the two terms being added differ by $2^{64}$ or more and both
    /// terms are finite and nonzero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::MulSubMulAssign;
    /// use malachite_float::BigExponentFloat;
    ///
    /// let mut x = BigExponentFloat::from(1.5);
    /// x.mul_sub_mul_assign(
    ///     BigExponentFloat::from(0.25),
    ///     BigExponentFloat::from(3.0),
    ///     BigExponentFloat::from(-0.5),
    /// );
    /// assert_eq!(format!("{x:?}"), "0x1.e");
    /// ```
    #[inline]
    fn mul_sub_mul_assign(&mut self, y: Self, z: Self, w: Self) {
        let x = core::mem::replace(self, Self::ZERO);
        *self = mul_add_mul_helper(
            Cow::Owned(x),
            Cow::Owned(y),
            Cow::Owned(z),
            Cow::Owned(w),
            true,
            None,
        )
        .0;
    }
}

impl MulSubMulAssign<Self, Self, &Self> for BigExponentFloat {
    /// Subtracts the product of two [`BigExponentFloat`]s from a [`BigExponentFloat`] multiplied by
    /// a [`BigExponentFloat`], in place. On the right-hand side, the first and second
    /// [`BigExponentFloat`]s are taken by value and the third by reference. The result is exact.
    ///
    /// $$
    /// x \gets xy-zw.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the sum of the significant bits of
    /// the inputs plus the absolute difference of the exponents of the two terms being added.
    ///
    /// # Panics
    /// Panics if the exponents of the two terms being added differ by $2^{64}$ or more and both
    /// terms are finite and nonzero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::MulSubMulAssign;
    /// use malachite_float::BigExponentFloat;
    ///
    /// let mut x = BigExponentFloat::from(1.5);
    /// x.mul_sub_mul_assign(
    ///     BigExponentFloat::from(0.25),
    ///     BigExponentFloat::from(3.0),
    ///     &BigExponentFloat::from(-0.5),
    /// );
    /// assert_eq!(format!("{x:?}"), "0x1.e");
    /// ```
    #[inline]
    fn mul_sub_mul_assign(&mut self, y: Self, z: Self, w: &Self) {
        let x = core::mem::replace(self, Self::ZERO);
        *self = mul_add_mul_helper(
            Cow::Owned(x),
            Cow::Owned(y),
            Cow::Owned(z),
            Cow::Borrowed(w),
            true,
            None,
        )
        .0;
    }
}

impl MulSubMulAssign<Self, &Self, Self> for BigExponentFloat {
    /// Subtracts the product of two [`BigExponentFloat`]s from a [`BigExponentFloat`] multiplied by
    /// a [`BigExponentFloat`], in place. On the right-hand side, the first and third
    /// [`BigExponentFloat`]s are taken by value and the second by reference. The result is exact.
    ///
    /// $$
    /// x \gets xy-zw.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the sum of the significant bits of
    /// the inputs plus the absolute difference of the exponents of the two terms being added.
    ///
    /// # Panics
    /// Panics if the exponents of the two terms being added differ by $2^{64}$ or more and both
    /// terms are finite and nonzero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::MulSubMulAssign;
    /// use malachite_float::BigExponentFloat;
    ///
    /// let mut x = BigExponentFloat::from(1.5);
    /// x.mul_sub_mul_assign(
    ///     BigExponentFloat::from(0.25),
    ///     &BigExponentFloat::from(3.0),
    ///     BigExponentFloat::from(-0.5),
    /// );
    /// assert_eq!(format!("{x:?}"), "0x1.e");
    /// ```
    #[inline]
    fn mul_sub_mul_assign(&mut self, y: Self, z: &Self, w: Self) {
        let x = core::mem::replace(self, Self::ZERO);
        *self = mul_add_mul_helper(
            Cow::Owned(x),
            Cow::Owned(y),
            Cow::Borrowed(z),
            Cow::Owned(w),
            true,
            None,
        )
        .0;
    }
}

impl MulSubMulAssign<Self, &Self, &Self> for BigExponentFloat {
    /// Subtracts the product of two [`BigExponentFloat`]s from a [`BigExponentFloat`] multiplied by
    /// a [`BigExponentFloat`], in place. On the right-hand side, the first [`BigExponentFloat`] is
    /// taken by value and the second and third by reference. The result is exact.
    ///
    /// $$
    /// x \gets xy-zw.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the sum of the significant bits of
    /// the inputs plus the absolute difference of the exponents of the two terms being added.
    ///
    /// # Panics
    /// Panics if the exponents of the two terms being added differ by $2^{64}$ or more and both
    /// terms are finite and nonzero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::MulSubMulAssign;
    /// use malachite_float::BigExponentFloat;
    ///
    /// let mut x = BigExponentFloat::from(1.5);
    /// x.mul_sub_mul_assign(
    ///     BigExponentFloat::from(0.25),
    ///     &BigExponentFloat::from(3.0),
    ///     &BigExponentFloat::from(-0.5),
    /// );
    /// assert_eq!(format!("{x:?}"), "0x1.e");
    /// ```
    #[inline]
    fn mul_sub_mul_assign(&mut self, y: Self, z: &Self, w: &Self) {
        let x = core::mem::replace(self, Self::ZERO);
        *self = mul_add_mul_helper(
            Cow::Owned(x),
            Cow::Owned(y),
            Cow::Borrowed(z),
            Cow::Borrowed(w),
            true,
            None,
        )
        .0;
    }
}

impl MulSubMulAssign<&Self, Self, Self> for BigExponentFloat {
    /// Subtracts the product of two [`BigExponentFloat`]s from a [`BigExponentFloat`] multiplied by
    /// a [`BigExponentFloat`], in place. On the right-hand side, the second and third
    /// [`BigExponentFloat`]s are taken by value and the first by reference. The result is exact.
    ///
    /// $$
    /// x \gets xy-zw.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the sum of the significant bits of
    /// the inputs plus the absolute difference of the exponents of the two terms being added.
    ///
    /// # Panics
    /// Panics if the exponents of the two terms being added differ by $2^{64}$ or more and both
    /// terms are finite and nonzero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::MulSubMulAssign;
    /// use malachite_float::BigExponentFloat;
    ///
    /// let mut x = BigExponentFloat::from(1.5);
    /// x.mul_sub_mul_assign(
    ///     &BigExponentFloat::from(0.25),
    ///     BigExponentFloat::from(3.0),
    ///     BigExponentFloat::from(-0.5),
    /// );
    /// assert_eq!(format!("{x:?}"), "0x1.e");
    /// ```
    #[inline]
    fn mul_sub_mul_assign(&mut self, y: &Self, z: Self, w: Self) {
        let x = core::mem::replace(self, Self::ZERO);
        *self = mul_add_mul_helper(
            Cow::Owned(x),
            Cow::Borrowed(y),
            Cow::Owned(z),
            Cow::Owned(w),
            true,
            None,
        )
        .0;
    }
}

impl MulSubMulAssign<&Self, Self, &Self> for BigExponentFloat {
    /// Subtracts the product of two [`BigExponentFloat`]s from a [`BigExponentFloat`] multiplied by
    /// a [`BigExponentFloat`], in place. On the right-hand side, the second [`BigExponentFloat`] is
    /// taken by value and the first and third by reference. The result is exact.
    ///
    /// $$
    /// x \gets xy-zw.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the sum of the significant bits of
    /// the inputs plus the absolute difference of the exponents of the two terms being added.
    ///
    /// # Panics
    /// Panics if the exponents of the two terms being added differ by $2^{64}$ or more and both
    /// terms are finite and nonzero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::MulSubMulAssign;
    /// use malachite_float::BigExponentFloat;
    ///
    /// let mut x = BigExponentFloat::from(1.5);
    /// x.mul_sub_mul_assign(
    ///     &BigExponentFloat::from(0.25),
    ///     BigExponentFloat::from(3.0),
    ///     &BigExponentFloat::from(-0.5),
    /// );
    /// assert_eq!(format!("{x:?}"), "0x1.e");
    /// ```
    #[inline]
    fn mul_sub_mul_assign(&mut self, y: &Self, z: Self, w: &Self) {
        let x = core::mem::replace(self, Self::ZERO);
        *self = mul_add_mul_helper(
            Cow::Owned(x),
            Cow::Borrowed(y),
            Cow::Owned(z),
            Cow::Borrowed(w),
            true,
            None,
        )
        .0;
    }
}

impl MulSubMulAssign<&Self, &Self, Self> for BigExponentFloat {
    /// Subtracts the product of two [`BigExponentFloat`]s from a [`BigExponentFloat`] multiplied by
    /// a [`BigExponentFloat`], in place. On the right-hand side, the third [`BigExponentFloat`] is
    /// taken by value and the first and second by reference. The result is exact.
    ///
    /// $$
    /// x \gets xy-zw.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the sum of the significant bits of
    /// the inputs plus the absolute difference of the exponents of the two terms being added.
    ///
    /// # Panics
    /// Panics if the exponents of the two terms being added differ by $2^{64}$ or more and both
    /// terms are finite and nonzero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::MulSubMulAssign;
    /// use malachite_float::BigExponentFloat;
    ///
    /// let mut x = BigExponentFloat::from(1.5);
    /// x.mul_sub_mul_assign(
    ///     &BigExponentFloat::from(0.25),
    ///     &BigExponentFloat::from(3.0),
    ///     BigExponentFloat::from(-0.5),
    /// );
    /// assert_eq!(format!("{x:?}"), "0x1.e");
    /// ```
    #[inline]
    fn mul_sub_mul_assign(&mut self, y: &Self, z: &Self, w: Self) {
        let x = core::mem::replace(self, Self::ZERO);
        *self = mul_add_mul_helper(
            Cow::Owned(x),
            Cow::Borrowed(y),
            Cow::Borrowed(z),
            Cow::Owned(w),
            true,
            None,
        )
        .0;
    }
}

impl MulSubMulAssign<&Self, &Self, &Self> for BigExponentFloat {
    /// Subtracts the product of two [`BigExponentFloat`]s from a [`BigExponentFloat`] multiplied by
    /// a [`BigExponentFloat`], in place. The [`BigExponentFloat`]s on the right-hand side are taken
    /// by reference. The result is exact.
    ///
    /// $$
    /// x \gets xy-zw.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the sum of the significant bits of
    /// the inputs plus the absolute difference of the exponents of the two terms being added.
    ///
    /// # Panics
    /// Panics if the exponents of the two terms being added differ by $2^{64}$ or more and both
    /// terms are finite and nonzero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::MulSubMulAssign;
    /// use malachite_float::BigExponentFloat;
    ///
    /// let mut x = BigExponentFloat::from(1.5);
    /// x.mul_sub_mul_assign(
    ///     &BigExponentFloat::from(0.25),
    ///     &BigExponentFloat::from(3.0),
    ///     &BigExponentFloat::from(-0.5),
    /// );
    /// assert_eq!(format!("{x:?}"), "0x1.e");
    /// ```
    #[inline]
    fn mul_sub_mul_assign(&mut self, y: &Self, z: &Self, w: &Self) {
        let x = core::mem::replace(self, Self::ZERO);
        *self = mul_add_mul_helper(
            Cow::Owned(x),
            Cow::Borrowed(y),
            Cow::Borrowed(z),
            Cow::Borrowed(w),
            true,
            None,
        )
        .0;
    }
}
