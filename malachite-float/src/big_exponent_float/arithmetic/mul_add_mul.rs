// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::big_exponent_float::BigExponentFloat;
use crate::big_exponent_float::arithmetic::add::{add_helper, add_prec_round_generic};
use crate::big_exponent_float::arithmetic::mul::mul_helper;
use alloc::borrow::Cow;
use core::cmp::Ordering::{self, *};
use malachite_base::num::arithmetic::traits::{MulAddMul, MulAddMulAssign};
use malachite_base::num::basic::traits::{NaN, Zero};
use malachite_base::rounding_modes::RoundingMode::{self, *};

// x * y + z * w, or x * y - z * w if `negate` is true, rounded with `prec_rm` if it is `Some`, and
// exact otherwise. Both products are formed exactly, so the only rounding is that of the sum.
pub(crate) fn mul_add_mul_helper(
    x: Cow<'_, BigExponentFloat>,
    y: Cow<'_, BigExponentFloat>,
    z: Cow<'_, BigExponentFloat>,
    w: Cow<'_, BigExponentFloat>,
    negate: bool,
    prec_rm: Option<(u64, RoundingMode)>,
) -> (BigExponentFloat, Ordering) {
    if let Some((prec, _)) = prec_rm {
        assert_ne!(prec, 0);
    }
    if x.is_nan() || y.is_nan() || z.is_nan() || w.is_nan() {
        return (BigExponentFloat::NAN, Equal);
    }
    let xy = mul_helper(x, y, None).0;
    let zw = mul_helper(z, w, None).0;
    add_helper(Cow::Owned(xy), Cow::Owned(zw), negate, prec_rm)
}

crate_test_fn! {
    // x * y + z * w, or x * y - z * w if `negate` is true, rounded to `prec` bits with `rm`,
    // without `Float`'s kernels: the exact products are added by the generic implementation of
    // addition. Used to check the kernel path.
    mul_add_mul_prec_round_generic(
        x: &BigExponentFloat,
        y: &BigExponentFloat,
        z: &BigExponentFloat,
        w: &BigExponentFloat,
        negate: bool,
        prec: u64,
        rm: RoundingMode,
    ) -> (BigExponentFloat, Ordering) {
        add_prec_round_generic(&(x * y), &(z * w), negate, prec, rm)
    }
}

impl BigExponentFloat {
    /// Adds the products of two pairs of [`BigExponentFloat`]s, rounding the result to the
    /// specified precision and with the specified rounding mode. All four [`BigExponentFloat`]s are
    /// taken by value. An [`Ordering`] is also returned, indicating whether the rounded result is
    /// less than, equal to, or greater than the exact result. Although `NaN`s are not comparable to
    /// any [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The products are computed exactly, so only one rounding occurs.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,z,w,p,m) = xy+zw+\varepsilon.
    /// $$
    /// - If $xy+zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy+zw$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy+zw|\rfloor-p+1}$.
    /// - If $xy+zw$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |xy+zw|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,y,z,w,p,m)=\text{NaN}$ if any argument is NaN, if either product multiplies an
    ///   infinity by zero, or if $xy$ and $zw$ are infinities of opposite signs
    /// - $f(x,y,z,w,p,m)=\infty$ if $xy$ or $zw$ is $\infty$ and neither is $-\infty$
    /// - $f(x,y,z,w,p,m)=-\infty$ if $xy$ or $zw$ is $-\infty$ and neither is $\infty$
    /// - $f(x,y,z,w,p,m)=0$ if $xy=-zw$ and both are finite
    ///
    /// If you know you'll be using `Nearest`, consider using [`BigExponentFloat::mul_add_mul_prec`]
    /// instead. If you want the exact result, consider using [`MulAddMul::mul_add_mul`] instead.
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
    /// let (result, o) = BigExponentFloat::from(PI).mul_add_mul_prec_round(
    ///     BigExponentFloat::from(E),
    ///     BigExponentFloat::from(SQRT_2),
    ///     BigExponentFloat::from(LN_2),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x9.8");
    /// assert_eq!(o, Less);
    ///
    /// let (result, o) = BigExponentFloat::from(PI).mul_add_mul_prec_round(
    ///     BigExponentFloat::from(E),
    ///     BigExponentFloat::from(SQRT_2),
    ///     BigExponentFloat::from(LN_2),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{result:?}"), "0xa.0");
    /// assert_eq!(o, Greater);
    ///
    /// let (result, o) = BigExponentFloat::from(PI).mul_add_mul_prec_round(
    ///     BigExponentFloat::from(E),
    ///     BigExponentFloat::from(SQRT_2),
    ///     BigExponentFloat::from(LN_2),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x9.8");
    /// assert_eq!(o, Less);
    ///
    /// let (result, o) = BigExponentFloat::from(PI).mul_add_mul_prec_round(
    ///     BigExponentFloat::from(E),
    ///     BigExponentFloat::from(SQRT_2),
    ///     BigExponentFloat::from(LN_2),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x9.851e");
    /// assert_eq!(o, Less);
    ///
    /// let (result, o) = BigExponentFloat::from(PI).mul_add_mul_prec_round(
    ///     BigExponentFloat::from(E),
    ///     BigExponentFloat::from(SQRT_2),
    ///     BigExponentFloat::from(LN_2),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x9.851f");
    /// assert_eq!(o, Greater);
    ///
    /// let (result, o) = BigExponentFloat::from(PI).mul_add_mul_prec_round(
    ///     BigExponentFloat::from(E),
    ///     BigExponentFloat::from(SQRT_2),
    ///     BigExponentFloat::from(LN_2),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x9.851e");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn mul_add_mul_prec_round(
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
            false,
            Some((prec, rm)),
        )
    }

    /// Adds the products of two pairs of [`BigExponentFloat`]s, rounding the result to the
    /// specified precision and with the specified rounding mode. The first, second, and third
    /// [`BigExponentFloat`]s are taken by value and the fourth by reference. An [`Ordering`] is
    /// also returned, indicating whether the rounded result is less than, equal to, or greater than
    /// the exact result. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever
    /// this function returns a `NaN` it also returns `Equal`.
    ///
    /// The products are computed exactly, so only one rounding occurs.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,z,w,p,m) = xy+zw+\varepsilon.
    /// $$
    /// - If $xy+zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy+zw$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy+zw|\rfloor-p+1}$.
    /// - If $xy+zw$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |xy+zw|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,y,z,w,p,m)=\text{NaN}$ if any argument is NaN, if either product multiplies an
    ///   infinity by zero, or if $xy$ and $zw$ are infinities of opposite signs
    /// - $f(x,y,z,w,p,m)=\infty$ if $xy$ or $zw$ is $\infty$ and neither is $-\infty$
    /// - $f(x,y,z,w,p,m)=-\infty$ if $xy$ or $zw$ is $-\infty$ and neither is $\infty$
    /// - $f(x,y,z,w,p,m)=0$ if $xy=-zw$ and both are finite
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
    /// let (result, o) = BigExponentFloat::from(PI).mul_add_mul_prec_round_val_val_val_ref(
    ///     BigExponentFloat::from(E),
    ///     BigExponentFloat::from(SQRT_2),
    ///     &BigExponentFloat::from(LN_2),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x9.8");
    /// assert_eq!(o, Less);
    ///
    /// let (result, o) = BigExponentFloat::from(PI).mul_add_mul_prec_round_val_val_val_ref(
    ///     BigExponentFloat::from(E),
    ///     BigExponentFloat::from(SQRT_2),
    ///     &BigExponentFloat::from(LN_2),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{result:?}"), "0xa.0");
    /// assert_eq!(o, Greater);
    ///
    /// let (result, o) = BigExponentFloat::from(PI).mul_add_mul_prec_round_val_val_val_ref(
    ///     BigExponentFloat::from(E),
    ///     BigExponentFloat::from(SQRT_2),
    ///     &BigExponentFloat::from(LN_2),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x9.8");
    /// assert_eq!(o, Less);
    ///
    /// let (result, o) = BigExponentFloat::from(PI).mul_add_mul_prec_round_val_val_val_ref(
    ///     BigExponentFloat::from(E),
    ///     BigExponentFloat::from(SQRT_2),
    ///     &BigExponentFloat::from(LN_2),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x9.851e");
    /// assert_eq!(o, Less);
    ///
    /// let (result, o) = BigExponentFloat::from(PI).mul_add_mul_prec_round_val_val_val_ref(
    ///     BigExponentFloat::from(E),
    ///     BigExponentFloat::from(SQRT_2),
    ///     &BigExponentFloat::from(LN_2),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x9.851f");
    /// assert_eq!(o, Greater);
    ///
    /// let (result, o) = BigExponentFloat::from(PI).mul_add_mul_prec_round_val_val_val_ref(
    ///     BigExponentFloat::from(E),
    ///     BigExponentFloat::from(SQRT_2),
    ///     &BigExponentFloat::from(LN_2),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x9.851e");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn mul_add_mul_prec_round_val_val_val_ref(
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
            false,
            Some((prec, rm)),
        )
    }

    /// Adds the products of two pairs of [`BigExponentFloat`]s, rounding the result to the
    /// specified precision and with the specified rounding mode. The first, second, and fourth
    /// [`BigExponentFloat`]s are taken by value and the third by reference. An [`Ordering`] is also
    /// returned, indicating whether the rounded result is less than, equal to, or greater than the
    /// exact result. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// The products are computed exactly, so only one rounding occurs.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,z,w,p,m) = xy+zw+\varepsilon.
    /// $$
    /// - If $xy+zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy+zw$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy+zw|\rfloor-p+1}$.
    /// - If $xy+zw$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |xy+zw|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,y,z,w,p,m)=\text{NaN}$ if any argument is NaN, if either product multiplies an
    ///   infinity by zero, or if $xy$ and $zw$ are infinities of opposite signs
    /// - $f(x,y,z,w,p,m)=\infty$ if $xy$ or $zw$ is $\infty$ and neither is $-\infty$
    /// - $f(x,y,z,w,p,m)=-\infty$ if $xy$ or $zw$ is $-\infty$ and neither is $\infty$
    /// - $f(x,y,z,w,p,m)=0$ if $xy=-zw$ and both are finite
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
    /// let (result, o) = BigExponentFloat::from(PI).mul_add_mul_prec_round_val_val_ref_val(
    ///     BigExponentFloat::from(E),
    ///     &BigExponentFloat::from(SQRT_2),
    ///     BigExponentFloat::from(LN_2),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x9.8");
    /// assert_eq!(o, Less);
    ///
    /// let (result, o) = BigExponentFloat::from(PI).mul_add_mul_prec_round_val_val_ref_val(
    ///     BigExponentFloat::from(E),
    ///     &BigExponentFloat::from(SQRT_2),
    ///     BigExponentFloat::from(LN_2),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{result:?}"), "0xa.0");
    /// assert_eq!(o, Greater);
    ///
    /// let (result, o) = BigExponentFloat::from(PI).mul_add_mul_prec_round_val_val_ref_val(
    ///     BigExponentFloat::from(E),
    ///     &BigExponentFloat::from(SQRT_2),
    ///     BigExponentFloat::from(LN_2),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x9.8");
    /// assert_eq!(o, Less);
    ///
    /// let (result, o) = BigExponentFloat::from(PI).mul_add_mul_prec_round_val_val_ref_val(
    ///     BigExponentFloat::from(E),
    ///     &BigExponentFloat::from(SQRT_2),
    ///     BigExponentFloat::from(LN_2),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x9.851e");
    /// assert_eq!(o, Less);
    ///
    /// let (result, o) = BigExponentFloat::from(PI).mul_add_mul_prec_round_val_val_ref_val(
    ///     BigExponentFloat::from(E),
    ///     &BigExponentFloat::from(SQRT_2),
    ///     BigExponentFloat::from(LN_2),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x9.851f");
    /// assert_eq!(o, Greater);
    ///
    /// let (result, o) = BigExponentFloat::from(PI).mul_add_mul_prec_round_val_val_ref_val(
    ///     BigExponentFloat::from(E),
    ///     &BigExponentFloat::from(SQRT_2),
    ///     BigExponentFloat::from(LN_2),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x9.851e");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn mul_add_mul_prec_round_val_val_ref_val(
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
            false,
            Some((prec, rm)),
        )
    }

    /// Adds the products of two pairs of [`BigExponentFloat`]s, rounding the result to the
    /// specified precision and with the specified rounding mode. The first and second
    /// [`BigExponentFloat`]s are taken by value and the third and fourth by reference. An
    /// [`Ordering`] is also returned, indicating whether the rounded result is less than, equal to,
    /// or greater than the exact result. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The products are computed exactly, so only one rounding occurs.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,z,w,p,m) = xy+zw+\varepsilon.
    /// $$
    /// - If $xy+zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy+zw$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy+zw|\rfloor-p+1}$.
    /// - If $xy+zw$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |xy+zw|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,y,z,w,p,m)=\text{NaN}$ if any argument is NaN, if either product multiplies an
    ///   infinity by zero, or if $xy$ and $zw$ are infinities of opposite signs
    /// - $f(x,y,z,w,p,m)=\infty$ if $xy$ or $zw$ is $\infty$ and neither is $-\infty$
    /// - $f(x,y,z,w,p,m)=-\infty$ if $xy$ or $zw$ is $-\infty$ and neither is $\infty$
    /// - $f(x,y,z,w,p,m)=0$ if $xy=-zw$ and both are finite
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
    /// let (result, o) = BigExponentFloat::from(PI).mul_add_mul_prec_round_val_val_ref_ref(
    ///     BigExponentFloat::from(E),
    ///     &BigExponentFloat::from(SQRT_2),
    ///     &BigExponentFloat::from(LN_2),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x9.8");
    /// assert_eq!(o, Less);
    ///
    /// let (result, o) = BigExponentFloat::from(PI).mul_add_mul_prec_round_val_val_ref_ref(
    ///     BigExponentFloat::from(E),
    ///     &BigExponentFloat::from(SQRT_2),
    ///     &BigExponentFloat::from(LN_2),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{result:?}"), "0xa.0");
    /// assert_eq!(o, Greater);
    ///
    /// let (result, o) = BigExponentFloat::from(PI).mul_add_mul_prec_round_val_val_ref_ref(
    ///     BigExponentFloat::from(E),
    ///     &BigExponentFloat::from(SQRT_2),
    ///     &BigExponentFloat::from(LN_2),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x9.8");
    /// assert_eq!(o, Less);
    ///
    /// let (result, o) = BigExponentFloat::from(PI).mul_add_mul_prec_round_val_val_ref_ref(
    ///     BigExponentFloat::from(E),
    ///     &BigExponentFloat::from(SQRT_2),
    ///     &BigExponentFloat::from(LN_2),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x9.851e");
    /// assert_eq!(o, Less);
    ///
    /// let (result, o) = BigExponentFloat::from(PI).mul_add_mul_prec_round_val_val_ref_ref(
    ///     BigExponentFloat::from(E),
    ///     &BigExponentFloat::from(SQRT_2),
    ///     &BigExponentFloat::from(LN_2),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x9.851f");
    /// assert_eq!(o, Greater);
    ///
    /// let (result, o) = BigExponentFloat::from(PI).mul_add_mul_prec_round_val_val_ref_ref(
    ///     BigExponentFloat::from(E),
    ///     &BigExponentFloat::from(SQRT_2),
    ///     &BigExponentFloat::from(LN_2),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x9.851e");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn mul_add_mul_prec_round_val_val_ref_ref(
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
            false,
            Some((prec, rm)),
        )
    }

    /// Adds the products of two pairs of [`BigExponentFloat`]s, rounding the result to the
    /// specified precision and with the specified rounding mode. The first, third, and fourth
    /// [`BigExponentFloat`]s are taken by value and the second by reference. An [`Ordering`] is
    /// also returned, indicating whether the rounded result is less than, equal to, or greater than
    /// the exact result. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever
    /// this function returns a `NaN` it also returns `Equal`.
    ///
    /// The products are computed exactly, so only one rounding occurs.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,z,w,p,m) = xy+zw+\varepsilon.
    /// $$
    /// - If $xy+zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy+zw$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy+zw|\rfloor-p+1}$.
    /// - If $xy+zw$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |xy+zw|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,y,z,w,p,m)=\text{NaN}$ if any argument is NaN, if either product multiplies an
    ///   infinity by zero, or if $xy$ and $zw$ are infinities of opposite signs
    /// - $f(x,y,z,w,p,m)=\infty$ if $xy$ or $zw$ is $\infty$ and neither is $-\infty$
    /// - $f(x,y,z,w,p,m)=-\infty$ if $xy$ or $zw$ is $-\infty$ and neither is $\infty$
    /// - $f(x,y,z,w,p,m)=0$ if $xy=-zw$ and both are finite
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
    /// let (result, o) = BigExponentFloat::from(PI).mul_add_mul_prec_round_val_ref_val_val(
    ///     &BigExponentFloat::from(E),
    ///     BigExponentFloat::from(SQRT_2),
    ///     BigExponentFloat::from(LN_2),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x9.8");
    /// assert_eq!(o, Less);
    ///
    /// let (result, o) = BigExponentFloat::from(PI).mul_add_mul_prec_round_val_ref_val_val(
    ///     &BigExponentFloat::from(E),
    ///     BigExponentFloat::from(SQRT_2),
    ///     BigExponentFloat::from(LN_2),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{result:?}"), "0xa.0");
    /// assert_eq!(o, Greater);
    ///
    /// let (result, o) = BigExponentFloat::from(PI).mul_add_mul_prec_round_val_ref_val_val(
    ///     &BigExponentFloat::from(E),
    ///     BigExponentFloat::from(SQRT_2),
    ///     BigExponentFloat::from(LN_2),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x9.8");
    /// assert_eq!(o, Less);
    ///
    /// let (result, o) = BigExponentFloat::from(PI).mul_add_mul_prec_round_val_ref_val_val(
    ///     &BigExponentFloat::from(E),
    ///     BigExponentFloat::from(SQRT_2),
    ///     BigExponentFloat::from(LN_2),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x9.851e");
    /// assert_eq!(o, Less);
    ///
    /// let (result, o) = BigExponentFloat::from(PI).mul_add_mul_prec_round_val_ref_val_val(
    ///     &BigExponentFloat::from(E),
    ///     BigExponentFloat::from(SQRT_2),
    ///     BigExponentFloat::from(LN_2),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x9.851f");
    /// assert_eq!(o, Greater);
    ///
    /// let (result, o) = BigExponentFloat::from(PI).mul_add_mul_prec_round_val_ref_val_val(
    ///     &BigExponentFloat::from(E),
    ///     BigExponentFloat::from(SQRT_2),
    ///     BigExponentFloat::from(LN_2),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x9.851e");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn mul_add_mul_prec_round_val_ref_val_val(
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
            false,
            Some((prec, rm)),
        )
    }

    /// Adds the products of two pairs of [`BigExponentFloat`]s, rounding the result to the
    /// specified precision and with the specified rounding mode. The first and third
    /// [`BigExponentFloat`]s are taken by value and the second and fourth by reference. An
    /// [`Ordering`] is also returned, indicating whether the rounded result is less than, equal to,
    /// or greater than the exact result. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The products are computed exactly, so only one rounding occurs.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,z,w,p,m) = xy+zw+\varepsilon.
    /// $$
    /// - If $xy+zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy+zw$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy+zw|\rfloor-p+1}$.
    /// - If $xy+zw$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |xy+zw|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,y,z,w,p,m)=\text{NaN}$ if any argument is NaN, if either product multiplies an
    ///   infinity by zero, or if $xy$ and $zw$ are infinities of opposite signs
    /// - $f(x,y,z,w,p,m)=\infty$ if $xy$ or $zw$ is $\infty$ and neither is $-\infty$
    /// - $f(x,y,z,w,p,m)=-\infty$ if $xy$ or $zw$ is $-\infty$ and neither is $\infty$
    /// - $f(x,y,z,w,p,m)=0$ if $xy=-zw$ and both are finite
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
    /// let (result, o) = BigExponentFloat::from(PI).mul_add_mul_prec_round_val_ref_val_ref(
    ///     &BigExponentFloat::from(E),
    ///     BigExponentFloat::from(SQRT_2),
    ///     &BigExponentFloat::from(LN_2),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x9.8");
    /// assert_eq!(o, Less);
    ///
    /// let (result, o) = BigExponentFloat::from(PI).mul_add_mul_prec_round_val_ref_val_ref(
    ///     &BigExponentFloat::from(E),
    ///     BigExponentFloat::from(SQRT_2),
    ///     &BigExponentFloat::from(LN_2),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{result:?}"), "0xa.0");
    /// assert_eq!(o, Greater);
    ///
    /// let (result, o) = BigExponentFloat::from(PI).mul_add_mul_prec_round_val_ref_val_ref(
    ///     &BigExponentFloat::from(E),
    ///     BigExponentFloat::from(SQRT_2),
    ///     &BigExponentFloat::from(LN_2),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x9.8");
    /// assert_eq!(o, Less);
    ///
    /// let (result, o) = BigExponentFloat::from(PI).mul_add_mul_prec_round_val_ref_val_ref(
    ///     &BigExponentFloat::from(E),
    ///     BigExponentFloat::from(SQRT_2),
    ///     &BigExponentFloat::from(LN_2),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x9.851e");
    /// assert_eq!(o, Less);
    ///
    /// let (result, o) = BigExponentFloat::from(PI).mul_add_mul_prec_round_val_ref_val_ref(
    ///     &BigExponentFloat::from(E),
    ///     BigExponentFloat::from(SQRT_2),
    ///     &BigExponentFloat::from(LN_2),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x9.851f");
    /// assert_eq!(o, Greater);
    ///
    /// let (result, o) = BigExponentFloat::from(PI).mul_add_mul_prec_round_val_ref_val_ref(
    ///     &BigExponentFloat::from(E),
    ///     BigExponentFloat::from(SQRT_2),
    ///     &BigExponentFloat::from(LN_2),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x9.851e");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn mul_add_mul_prec_round_val_ref_val_ref(
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
            false,
            Some((prec, rm)),
        )
    }

    /// Adds the products of two pairs of [`BigExponentFloat`]s, rounding the result to the
    /// specified precision and with the specified rounding mode. The first and fourth
    /// [`BigExponentFloat`]s are taken by value and the second and third by reference. An
    /// [`Ordering`] is also returned, indicating whether the rounded result is less than, equal to,
    /// or greater than the exact result. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The products are computed exactly, so only one rounding occurs.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,z,w,p,m) = xy+zw+\varepsilon.
    /// $$
    /// - If $xy+zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy+zw$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy+zw|\rfloor-p+1}$.
    /// - If $xy+zw$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |xy+zw|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,y,z,w,p,m)=\text{NaN}$ if any argument is NaN, if either product multiplies an
    ///   infinity by zero, or if $xy$ and $zw$ are infinities of opposite signs
    /// - $f(x,y,z,w,p,m)=\infty$ if $xy$ or $zw$ is $\infty$ and neither is $-\infty$
    /// - $f(x,y,z,w,p,m)=-\infty$ if $xy$ or $zw$ is $-\infty$ and neither is $\infty$
    /// - $f(x,y,z,w,p,m)=0$ if $xy=-zw$ and both are finite
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
    /// let (result, o) = BigExponentFloat::from(PI).mul_add_mul_prec_round_val_ref_ref_val(
    ///     &BigExponentFloat::from(E),
    ///     &BigExponentFloat::from(SQRT_2),
    ///     BigExponentFloat::from(LN_2),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x9.8");
    /// assert_eq!(o, Less);
    ///
    /// let (result, o) = BigExponentFloat::from(PI).mul_add_mul_prec_round_val_ref_ref_val(
    ///     &BigExponentFloat::from(E),
    ///     &BigExponentFloat::from(SQRT_2),
    ///     BigExponentFloat::from(LN_2),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{result:?}"), "0xa.0");
    /// assert_eq!(o, Greater);
    ///
    /// let (result, o) = BigExponentFloat::from(PI).mul_add_mul_prec_round_val_ref_ref_val(
    ///     &BigExponentFloat::from(E),
    ///     &BigExponentFloat::from(SQRT_2),
    ///     BigExponentFloat::from(LN_2),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x9.8");
    /// assert_eq!(o, Less);
    ///
    /// let (result, o) = BigExponentFloat::from(PI).mul_add_mul_prec_round_val_ref_ref_val(
    ///     &BigExponentFloat::from(E),
    ///     &BigExponentFloat::from(SQRT_2),
    ///     BigExponentFloat::from(LN_2),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x9.851e");
    /// assert_eq!(o, Less);
    ///
    /// let (result, o) = BigExponentFloat::from(PI).mul_add_mul_prec_round_val_ref_ref_val(
    ///     &BigExponentFloat::from(E),
    ///     &BigExponentFloat::from(SQRT_2),
    ///     BigExponentFloat::from(LN_2),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x9.851f");
    /// assert_eq!(o, Greater);
    ///
    /// let (result, o) = BigExponentFloat::from(PI).mul_add_mul_prec_round_val_ref_ref_val(
    ///     &BigExponentFloat::from(E),
    ///     &BigExponentFloat::from(SQRT_2),
    ///     BigExponentFloat::from(LN_2),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x9.851e");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn mul_add_mul_prec_round_val_ref_ref_val(
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
            false,
            Some((prec, rm)),
        )
    }

    /// Adds the products of two pairs of [`BigExponentFloat`]s, rounding the result to the
    /// specified precision and with the specified rounding mode. The first [`BigExponentFloat`] is
    /// taken by value and the second, third, and fourth by reference. An [`Ordering`] is also
    /// returned, indicating whether the rounded result is less than, equal to, or greater than the
    /// exact result. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// The products are computed exactly, so only one rounding occurs.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,z,w,p,m) = xy+zw+\varepsilon.
    /// $$
    /// - If $xy+zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy+zw$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy+zw|\rfloor-p+1}$.
    /// - If $xy+zw$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |xy+zw|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,y,z,w,p,m)=\text{NaN}$ if any argument is NaN, if either product multiplies an
    ///   infinity by zero, or if $xy$ and $zw$ are infinities of opposite signs
    /// - $f(x,y,z,w,p,m)=\infty$ if $xy$ or $zw$ is $\infty$ and neither is $-\infty$
    /// - $f(x,y,z,w,p,m)=-\infty$ if $xy$ or $zw$ is $-\infty$ and neither is $\infty$
    /// - $f(x,y,z,w,p,m)=0$ if $xy=-zw$ and both are finite
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
    /// let (result, o) = BigExponentFloat::from(PI).mul_add_mul_prec_round_val_ref_ref_ref(
    ///     &BigExponentFloat::from(E),
    ///     &BigExponentFloat::from(SQRT_2),
    ///     &BigExponentFloat::from(LN_2),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x9.8");
    /// assert_eq!(o, Less);
    ///
    /// let (result, o) = BigExponentFloat::from(PI).mul_add_mul_prec_round_val_ref_ref_ref(
    ///     &BigExponentFloat::from(E),
    ///     &BigExponentFloat::from(SQRT_2),
    ///     &BigExponentFloat::from(LN_2),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{result:?}"), "0xa.0");
    /// assert_eq!(o, Greater);
    ///
    /// let (result, o) = BigExponentFloat::from(PI).mul_add_mul_prec_round_val_ref_ref_ref(
    ///     &BigExponentFloat::from(E),
    ///     &BigExponentFloat::from(SQRT_2),
    ///     &BigExponentFloat::from(LN_2),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x9.8");
    /// assert_eq!(o, Less);
    ///
    /// let (result, o) = BigExponentFloat::from(PI).mul_add_mul_prec_round_val_ref_ref_ref(
    ///     &BigExponentFloat::from(E),
    ///     &BigExponentFloat::from(SQRT_2),
    ///     &BigExponentFloat::from(LN_2),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x9.851e");
    /// assert_eq!(o, Less);
    ///
    /// let (result, o) = BigExponentFloat::from(PI).mul_add_mul_prec_round_val_ref_ref_ref(
    ///     &BigExponentFloat::from(E),
    ///     &BigExponentFloat::from(SQRT_2),
    ///     &BigExponentFloat::from(LN_2),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x9.851f");
    /// assert_eq!(o, Greater);
    ///
    /// let (result, o) = BigExponentFloat::from(PI).mul_add_mul_prec_round_val_ref_ref_ref(
    ///     &BigExponentFloat::from(E),
    ///     &BigExponentFloat::from(SQRT_2),
    ///     &BigExponentFloat::from(LN_2),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x9.851e");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn mul_add_mul_prec_round_val_ref_ref_ref(
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
            false,
            Some((prec, rm)),
        )
    }

    /// Adds the products of two pairs of [`BigExponentFloat`]s, rounding the result to the
    /// specified precision and with the specified rounding mode. All four [`BigExponentFloat`]s are
    /// taken by reference. An [`Ordering`] is also returned, indicating whether the rounded result
    /// is less than, equal to, or greater than the exact result. Although `NaN`s are not comparable
    /// to any [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The products are computed exactly, so only one rounding occurs.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,z,w,p,m) = xy+zw+\varepsilon.
    /// $$
    /// - If $xy+zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy+zw$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy+zw|\rfloor-p+1}$.
    /// - If $xy+zw$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |xy+zw|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,y,z,w,p,m)=\text{NaN}$ if any argument is NaN, if either product multiplies an
    ///   infinity by zero, or if $xy$ and $zw$ are infinities of opposite signs
    /// - $f(x,y,z,w,p,m)=\infty$ if $xy$ or $zw$ is $\infty$ and neither is $-\infty$
    /// - $f(x,y,z,w,p,m)=-\infty$ if $xy$ or $zw$ is $-\infty$ and neither is $\infty$
    /// - $f(x,y,z,w,p,m)=0$ if $xy=-zw$ and both are finite
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
    /// let (result, o) = (&BigExponentFloat::from(PI)).mul_add_mul_prec_round_ref_ref_ref_ref(
    ///     &BigExponentFloat::from(E),
    ///     &BigExponentFloat::from(SQRT_2),
    ///     &BigExponentFloat::from(LN_2),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x9.8");
    /// assert_eq!(o, Less);
    ///
    /// let (result, o) = (&BigExponentFloat::from(PI)).mul_add_mul_prec_round_ref_ref_ref_ref(
    ///     &BigExponentFloat::from(E),
    ///     &BigExponentFloat::from(SQRT_2),
    ///     &BigExponentFloat::from(LN_2),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{result:?}"), "0xa.0");
    /// assert_eq!(o, Greater);
    ///
    /// let (result, o) = (&BigExponentFloat::from(PI)).mul_add_mul_prec_round_ref_ref_ref_ref(
    ///     &BigExponentFloat::from(E),
    ///     &BigExponentFloat::from(SQRT_2),
    ///     &BigExponentFloat::from(LN_2),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x9.8");
    /// assert_eq!(o, Less);
    ///
    /// let (result, o) = (&BigExponentFloat::from(PI)).mul_add_mul_prec_round_ref_ref_ref_ref(
    ///     &BigExponentFloat::from(E),
    ///     &BigExponentFloat::from(SQRT_2),
    ///     &BigExponentFloat::from(LN_2),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x9.851e");
    /// assert_eq!(o, Less);
    ///
    /// let (result, o) = (&BigExponentFloat::from(PI)).mul_add_mul_prec_round_ref_ref_ref_ref(
    ///     &BigExponentFloat::from(E),
    ///     &BigExponentFloat::from(SQRT_2),
    ///     &BigExponentFloat::from(LN_2),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x9.851f");
    /// assert_eq!(o, Greater);
    ///
    /// let (result, o) = (&BigExponentFloat::from(PI)).mul_add_mul_prec_round_ref_ref_ref_ref(
    ///     &BigExponentFloat::from(E),
    ///     &BigExponentFloat::from(SQRT_2),
    ///     &BigExponentFloat::from(LN_2),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x9.851e");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn mul_add_mul_prec_round_ref_ref_ref_ref(
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
            false,
            Some((prec, rm)),
        )
    }

    /// Adds the product of two [`BigExponentFloat`]s to a [`BigExponentFloat`] multiplied by a
    /// [`BigExponentFloat`], in place, rounding the result to the specified precision and with the
    /// specified rounding mode. The [`BigExponentFloat`]s on the right-hand side are taken by
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
    /// x \gets xy+zw+\varepsilon.
    /// $$
    /// - If $xy+zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy+zw$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy+zw|\rfloor-p+1}$.
    /// - If $xy+zw$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |xy+zw|\rfloor-p}$.
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
    ///     x.mul_add_mul_prec_round_assign(
    ///         BigExponentFloat::from(E),
    ///         BigExponentFloat::from(SQRT_2),
    ///         BigExponentFloat::from(LN_2),
    ///         5,
    ///         Floor
    ///     ),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x9.8");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.mul_add_mul_prec_round_assign(
    ///         BigExponentFloat::from(E),
    ///         BigExponentFloat::from(SQRT_2),
    ///         BigExponentFloat::from(LN_2),
    ///         5,
    ///         Ceiling
    ///     ),
    ///     Greater
    /// );
    /// assert_eq!(format!("{x:?}"), "0xa.0");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.mul_add_mul_prec_round_assign(
    ///         BigExponentFloat::from(E),
    ///         BigExponentFloat::from(SQRT_2),
    ///         BigExponentFloat::from(LN_2),
    ///         5,
    ///         Nearest
    ///     ),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x9.8");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.mul_add_mul_prec_round_assign(
    ///         BigExponentFloat::from(E),
    ///         BigExponentFloat::from(SQRT_2),
    ///         BigExponentFloat::from(LN_2),
    ///         20,
    ///         Floor
    ///     ),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x9.851e");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.mul_add_mul_prec_round_assign(
    ///         BigExponentFloat::from(E),
    ///         BigExponentFloat::from(SQRT_2),
    ///         BigExponentFloat::from(LN_2),
    ///         20,
    ///         Ceiling
    ///     ),
    ///     Greater
    /// );
    /// assert_eq!(format!("{x:?}"), "0x9.851f");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.mul_add_mul_prec_round_assign(
    ///         BigExponentFloat::from(E),
    ///         BigExponentFloat::from(SQRT_2),
    ///         BigExponentFloat::from(LN_2),
    ///         20,
    ///         Nearest
    ///     ),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x9.851e");
    /// ```
    #[inline]
    pub fn mul_add_mul_prec_round_assign(
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
            false,
            Some((prec, rm)),
        );
        *self = result;
        o
    }

    /// Adds the product of two [`BigExponentFloat`]s to a [`BigExponentFloat`] multiplied by a
    /// [`BigExponentFloat`], in place, rounding the result to the specified precision and with the
    /// specified rounding mode. On the right-hand side, the first and second [`BigExponentFloat`]s
    /// are taken by value and the third by reference. An [`Ordering`] is returned, indicating
    /// whether the rounded result is less than, equal to, or greater than the exact result.
    /// Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function sets
    /// the [`BigExponentFloat`] to `NaN` it also returns `Equal`.
    ///
    /// The products are computed exactly, so only one rounding occurs.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets xy+zw+\varepsilon.
    /// $$
    /// - If $xy+zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy+zw$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy+zw|\rfloor-p+1}$.
    /// - If $xy+zw$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |xy+zw|\rfloor-p}$.
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
    ///     x.mul_add_mul_prec_round_assign_val_val_ref(
    ///         BigExponentFloat::from(E),
    ///         BigExponentFloat::from(SQRT_2),
    ///         &BigExponentFloat::from(LN_2),
    ///         5,
    ///         Floor
    ///     ),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x9.8");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.mul_add_mul_prec_round_assign_val_val_ref(
    ///         BigExponentFloat::from(E),
    ///         BigExponentFloat::from(SQRT_2),
    ///         &BigExponentFloat::from(LN_2),
    ///         5,
    ///         Ceiling
    ///     ),
    ///     Greater
    /// );
    /// assert_eq!(format!("{x:?}"), "0xa.0");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.mul_add_mul_prec_round_assign_val_val_ref(
    ///         BigExponentFloat::from(E),
    ///         BigExponentFloat::from(SQRT_2),
    ///         &BigExponentFloat::from(LN_2),
    ///         5,
    ///         Nearest
    ///     ),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x9.8");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.mul_add_mul_prec_round_assign_val_val_ref(
    ///         BigExponentFloat::from(E),
    ///         BigExponentFloat::from(SQRT_2),
    ///         &BigExponentFloat::from(LN_2),
    ///         20,
    ///         Floor
    ///     ),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x9.851e");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.mul_add_mul_prec_round_assign_val_val_ref(
    ///         BigExponentFloat::from(E),
    ///         BigExponentFloat::from(SQRT_2),
    ///         &BigExponentFloat::from(LN_2),
    ///         20,
    ///         Ceiling
    ///     ),
    ///     Greater
    /// );
    /// assert_eq!(format!("{x:?}"), "0x9.851f");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.mul_add_mul_prec_round_assign_val_val_ref(
    ///         BigExponentFloat::from(E),
    ///         BigExponentFloat::from(SQRT_2),
    ///         &BigExponentFloat::from(LN_2),
    ///         20,
    ///         Nearest
    ///     ),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x9.851e");
    /// ```
    #[inline]
    pub fn mul_add_mul_prec_round_assign_val_val_ref(
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
            false,
            Some((prec, rm)),
        );
        *self = result;
        o
    }

    /// Adds the product of two [`BigExponentFloat`]s to a [`BigExponentFloat`] multiplied by a
    /// [`BigExponentFloat`], in place, rounding the result to the specified precision and with the
    /// specified rounding mode. On the right-hand side, the first and third [`BigExponentFloat`]s
    /// are taken by value and the second by reference. An [`Ordering`] is returned, indicating
    /// whether the rounded result is less than, equal to, or greater than the exact result.
    /// Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function sets
    /// the [`BigExponentFloat`] to `NaN` it also returns `Equal`.
    ///
    /// The products are computed exactly, so only one rounding occurs.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets xy+zw+\varepsilon.
    /// $$
    /// - If $xy+zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy+zw$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy+zw|\rfloor-p+1}$.
    /// - If $xy+zw$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |xy+zw|\rfloor-p}$.
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
    ///     x.mul_add_mul_prec_round_assign_val_ref_val(
    ///         BigExponentFloat::from(E),
    ///         &BigExponentFloat::from(SQRT_2),
    ///         BigExponentFloat::from(LN_2),
    ///         5,
    ///         Floor
    ///     ),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x9.8");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.mul_add_mul_prec_round_assign_val_ref_val(
    ///         BigExponentFloat::from(E),
    ///         &BigExponentFloat::from(SQRT_2),
    ///         BigExponentFloat::from(LN_2),
    ///         5,
    ///         Ceiling
    ///     ),
    ///     Greater
    /// );
    /// assert_eq!(format!("{x:?}"), "0xa.0");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.mul_add_mul_prec_round_assign_val_ref_val(
    ///         BigExponentFloat::from(E),
    ///         &BigExponentFloat::from(SQRT_2),
    ///         BigExponentFloat::from(LN_2),
    ///         5,
    ///         Nearest
    ///     ),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x9.8");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.mul_add_mul_prec_round_assign_val_ref_val(
    ///         BigExponentFloat::from(E),
    ///         &BigExponentFloat::from(SQRT_2),
    ///         BigExponentFloat::from(LN_2),
    ///         20,
    ///         Floor
    ///     ),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x9.851e");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.mul_add_mul_prec_round_assign_val_ref_val(
    ///         BigExponentFloat::from(E),
    ///         &BigExponentFloat::from(SQRT_2),
    ///         BigExponentFloat::from(LN_2),
    ///         20,
    ///         Ceiling
    ///     ),
    ///     Greater
    /// );
    /// assert_eq!(format!("{x:?}"), "0x9.851f");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.mul_add_mul_prec_round_assign_val_ref_val(
    ///         BigExponentFloat::from(E),
    ///         &BigExponentFloat::from(SQRT_2),
    ///         BigExponentFloat::from(LN_2),
    ///         20,
    ///         Nearest
    ///     ),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x9.851e");
    /// ```
    #[inline]
    pub fn mul_add_mul_prec_round_assign_val_ref_val(
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
            false,
            Some((prec, rm)),
        );
        *self = result;
        o
    }

    /// Adds the product of two [`BigExponentFloat`]s to a [`BigExponentFloat`] multiplied by a
    /// [`BigExponentFloat`], in place, rounding the result to the specified precision and with the
    /// specified rounding mode. On the right-hand side, the first [`BigExponentFloat`] is taken by
    /// value and the second and third by reference. An [`Ordering`] is returned, indicating whether
    /// the rounded result is less than, equal to, or greater than the exact result. Although `NaN`s
    /// are not comparable to any [`BigExponentFloat`], whenever this function sets the
    /// [`BigExponentFloat`] to `NaN` it also returns `Equal`.
    ///
    /// The products are computed exactly, so only one rounding occurs.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets xy+zw+\varepsilon.
    /// $$
    /// - If $xy+zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy+zw$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy+zw|\rfloor-p+1}$.
    /// - If $xy+zw$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |xy+zw|\rfloor-p}$.
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
    ///     x.mul_add_mul_prec_round_assign_val_ref_ref(
    ///         BigExponentFloat::from(E),
    ///         &BigExponentFloat::from(SQRT_2),
    ///         &BigExponentFloat::from(LN_2),
    ///         5,
    ///         Floor
    ///     ),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x9.8");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.mul_add_mul_prec_round_assign_val_ref_ref(
    ///         BigExponentFloat::from(E),
    ///         &BigExponentFloat::from(SQRT_2),
    ///         &BigExponentFloat::from(LN_2),
    ///         5,
    ///         Ceiling
    ///     ),
    ///     Greater
    /// );
    /// assert_eq!(format!("{x:?}"), "0xa.0");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.mul_add_mul_prec_round_assign_val_ref_ref(
    ///         BigExponentFloat::from(E),
    ///         &BigExponentFloat::from(SQRT_2),
    ///         &BigExponentFloat::from(LN_2),
    ///         5,
    ///         Nearest
    ///     ),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x9.8");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.mul_add_mul_prec_round_assign_val_ref_ref(
    ///         BigExponentFloat::from(E),
    ///         &BigExponentFloat::from(SQRT_2),
    ///         &BigExponentFloat::from(LN_2),
    ///         20,
    ///         Floor
    ///     ),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x9.851e");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.mul_add_mul_prec_round_assign_val_ref_ref(
    ///         BigExponentFloat::from(E),
    ///         &BigExponentFloat::from(SQRT_2),
    ///         &BigExponentFloat::from(LN_2),
    ///         20,
    ///         Ceiling
    ///     ),
    ///     Greater
    /// );
    /// assert_eq!(format!("{x:?}"), "0x9.851f");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.mul_add_mul_prec_round_assign_val_ref_ref(
    ///         BigExponentFloat::from(E),
    ///         &BigExponentFloat::from(SQRT_2),
    ///         &BigExponentFloat::from(LN_2),
    ///         20,
    ///         Nearest
    ///     ),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x9.851e");
    /// ```
    #[inline]
    pub fn mul_add_mul_prec_round_assign_val_ref_ref(
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
            false,
            Some((prec, rm)),
        );
        *self = result;
        o
    }

    /// Adds the product of two [`BigExponentFloat`]s to a [`BigExponentFloat`] multiplied by a
    /// [`BigExponentFloat`], in place, rounding the result to the specified precision and with the
    /// specified rounding mode. On the right-hand side, the second and third [`BigExponentFloat`]s
    /// are taken by value and the first by reference. An [`Ordering`] is returned, indicating
    /// whether the rounded result is less than, equal to, or greater than the exact result.
    /// Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function sets
    /// the [`BigExponentFloat`] to `NaN` it also returns `Equal`.
    ///
    /// The products are computed exactly, so only one rounding occurs.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets xy+zw+\varepsilon.
    /// $$
    /// - If $xy+zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy+zw$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy+zw|\rfloor-p+1}$.
    /// - If $xy+zw$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |xy+zw|\rfloor-p}$.
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
    ///     x.mul_add_mul_prec_round_assign_ref_val_val(
    ///         &BigExponentFloat::from(E),
    ///         BigExponentFloat::from(SQRT_2),
    ///         BigExponentFloat::from(LN_2),
    ///         5,
    ///         Floor
    ///     ),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x9.8");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.mul_add_mul_prec_round_assign_ref_val_val(
    ///         &BigExponentFloat::from(E),
    ///         BigExponentFloat::from(SQRT_2),
    ///         BigExponentFloat::from(LN_2),
    ///         5,
    ///         Ceiling
    ///     ),
    ///     Greater
    /// );
    /// assert_eq!(format!("{x:?}"), "0xa.0");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.mul_add_mul_prec_round_assign_ref_val_val(
    ///         &BigExponentFloat::from(E),
    ///         BigExponentFloat::from(SQRT_2),
    ///         BigExponentFloat::from(LN_2),
    ///         5,
    ///         Nearest
    ///     ),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x9.8");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.mul_add_mul_prec_round_assign_ref_val_val(
    ///         &BigExponentFloat::from(E),
    ///         BigExponentFloat::from(SQRT_2),
    ///         BigExponentFloat::from(LN_2),
    ///         20,
    ///         Floor
    ///     ),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x9.851e");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.mul_add_mul_prec_round_assign_ref_val_val(
    ///         &BigExponentFloat::from(E),
    ///         BigExponentFloat::from(SQRT_2),
    ///         BigExponentFloat::from(LN_2),
    ///         20,
    ///         Ceiling
    ///     ),
    ///     Greater
    /// );
    /// assert_eq!(format!("{x:?}"), "0x9.851f");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.mul_add_mul_prec_round_assign_ref_val_val(
    ///         &BigExponentFloat::from(E),
    ///         BigExponentFloat::from(SQRT_2),
    ///         BigExponentFloat::from(LN_2),
    ///         20,
    ///         Nearest
    ///     ),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x9.851e");
    /// ```
    #[inline]
    pub fn mul_add_mul_prec_round_assign_ref_val_val(
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
            false,
            Some((prec, rm)),
        );
        *self = result;
        o
    }

    /// Adds the product of two [`BigExponentFloat`]s to a [`BigExponentFloat`] multiplied by a
    /// [`BigExponentFloat`], in place, rounding the result to the specified precision and with the
    /// specified rounding mode. On the right-hand side, the second [`BigExponentFloat`] is taken by
    /// value and the first and third by reference. An [`Ordering`] is returned, indicating whether
    /// the rounded result is less than, equal to, or greater than the exact result. Although `NaN`s
    /// are not comparable to any [`BigExponentFloat`], whenever this function sets the
    /// [`BigExponentFloat`] to `NaN` it also returns `Equal`.
    ///
    /// The products are computed exactly, so only one rounding occurs.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets xy+zw+\varepsilon.
    /// $$
    /// - If $xy+zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy+zw$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy+zw|\rfloor-p+1}$.
    /// - If $xy+zw$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |xy+zw|\rfloor-p}$.
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
    ///     x.mul_add_mul_prec_round_assign_ref_val_ref(
    ///         &BigExponentFloat::from(E),
    ///         BigExponentFloat::from(SQRT_2),
    ///         &BigExponentFloat::from(LN_2),
    ///         5,
    ///         Floor
    ///     ),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x9.8");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.mul_add_mul_prec_round_assign_ref_val_ref(
    ///         &BigExponentFloat::from(E),
    ///         BigExponentFloat::from(SQRT_2),
    ///         &BigExponentFloat::from(LN_2),
    ///         5,
    ///         Ceiling
    ///     ),
    ///     Greater
    /// );
    /// assert_eq!(format!("{x:?}"), "0xa.0");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.mul_add_mul_prec_round_assign_ref_val_ref(
    ///         &BigExponentFloat::from(E),
    ///         BigExponentFloat::from(SQRT_2),
    ///         &BigExponentFloat::from(LN_2),
    ///         5,
    ///         Nearest
    ///     ),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x9.8");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.mul_add_mul_prec_round_assign_ref_val_ref(
    ///         &BigExponentFloat::from(E),
    ///         BigExponentFloat::from(SQRT_2),
    ///         &BigExponentFloat::from(LN_2),
    ///         20,
    ///         Floor
    ///     ),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x9.851e");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.mul_add_mul_prec_round_assign_ref_val_ref(
    ///         &BigExponentFloat::from(E),
    ///         BigExponentFloat::from(SQRT_2),
    ///         &BigExponentFloat::from(LN_2),
    ///         20,
    ///         Ceiling
    ///     ),
    ///     Greater
    /// );
    /// assert_eq!(format!("{x:?}"), "0x9.851f");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.mul_add_mul_prec_round_assign_ref_val_ref(
    ///         &BigExponentFloat::from(E),
    ///         BigExponentFloat::from(SQRT_2),
    ///         &BigExponentFloat::from(LN_2),
    ///         20,
    ///         Nearest
    ///     ),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x9.851e");
    /// ```
    #[inline]
    pub fn mul_add_mul_prec_round_assign_ref_val_ref(
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
            false,
            Some((prec, rm)),
        );
        *self = result;
        o
    }

    /// Adds the product of two [`BigExponentFloat`]s to a [`BigExponentFloat`] multiplied by a
    /// [`BigExponentFloat`], in place, rounding the result to the specified precision and with the
    /// specified rounding mode. On the right-hand side, the third [`BigExponentFloat`] is taken by
    /// value and the first and second by reference. An [`Ordering`] is returned, indicating whether
    /// the rounded result is less than, equal to, or greater than the exact result. Although `NaN`s
    /// are not comparable to any [`BigExponentFloat`], whenever this function sets the
    /// [`BigExponentFloat`] to `NaN` it also returns `Equal`.
    ///
    /// The products are computed exactly, so only one rounding occurs.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets xy+zw+\varepsilon.
    /// $$
    /// - If $xy+zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy+zw$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy+zw|\rfloor-p+1}$.
    /// - If $xy+zw$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |xy+zw|\rfloor-p}$.
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
    ///     x.mul_add_mul_prec_round_assign_ref_ref_val(
    ///         &BigExponentFloat::from(E),
    ///         &BigExponentFloat::from(SQRT_2),
    ///         BigExponentFloat::from(LN_2),
    ///         5,
    ///         Floor
    ///     ),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x9.8");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.mul_add_mul_prec_round_assign_ref_ref_val(
    ///         &BigExponentFloat::from(E),
    ///         &BigExponentFloat::from(SQRT_2),
    ///         BigExponentFloat::from(LN_2),
    ///         5,
    ///         Ceiling
    ///     ),
    ///     Greater
    /// );
    /// assert_eq!(format!("{x:?}"), "0xa.0");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.mul_add_mul_prec_round_assign_ref_ref_val(
    ///         &BigExponentFloat::from(E),
    ///         &BigExponentFloat::from(SQRT_2),
    ///         BigExponentFloat::from(LN_2),
    ///         5,
    ///         Nearest
    ///     ),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x9.8");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.mul_add_mul_prec_round_assign_ref_ref_val(
    ///         &BigExponentFloat::from(E),
    ///         &BigExponentFloat::from(SQRT_2),
    ///         BigExponentFloat::from(LN_2),
    ///         20,
    ///         Floor
    ///     ),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x9.851e");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.mul_add_mul_prec_round_assign_ref_ref_val(
    ///         &BigExponentFloat::from(E),
    ///         &BigExponentFloat::from(SQRT_2),
    ///         BigExponentFloat::from(LN_2),
    ///         20,
    ///         Ceiling
    ///     ),
    ///     Greater
    /// );
    /// assert_eq!(format!("{x:?}"), "0x9.851f");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.mul_add_mul_prec_round_assign_ref_ref_val(
    ///         &BigExponentFloat::from(E),
    ///         &BigExponentFloat::from(SQRT_2),
    ///         BigExponentFloat::from(LN_2),
    ///         20,
    ///         Nearest
    ///     ),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x9.851e");
    /// ```
    #[inline]
    pub fn mul_add_mul_prec_round_assign_ref_ref_val(
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
            false,
            Some((prec, rm)),
        );
        *self = result;
        o
    }

    /// Adds the product of two [`BigExponentFloat`]s to a [`BigExponentFloat`] multiplied by a
    /// [`BigExponentFloat`], in place, rounding the result to the specified precision and with the
    /// specified rounding mode. The [`BigExponentFloat`]s on the right-hand side are taken by
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
    /// x \gets xy+zw+\varepsilon.
    /// $$
    /// - If $xy+zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy+zw$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy+zw|\rfloor-p+1}$.
    /// - If $xy+zw$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |xy+zw|\rfloor-p}$.
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
    ///     x.mul_add_mul_prec_round_assign_ref_ref_ref(
    ///         &BigExponentFloat::from(E),
    ///         &BigExponentFloat::from(SQRT_2),
    ///         &BigExponentFloat::from(LN_2),
    ///         5,
    ///         Floor
    ///     ),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x9.8");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.mul_add_mul_prec_round_assign_ref_ref_ref(
    ///         &BigExponentFloat::from(E),
    ///         &BigExponentFloat::from(SQRT_2),
    ///         &BigExponentFloat::from(LN_2),
    ///         5,
    ///         Ceiling
    ///     ),
    ///     Greater
    /// );
    /// assert_eq!(format!("{x:?}"), "0xa.0");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.mul_add_mul_prec_round_assign_ref_ref_ref(
    ///         &BigExponentFloat::from(E),
    ///         &BigExponentFloat::from(SQRT_2),
    ///         &BigExponentFloat::from(LN_2),
    ///         5,
    ///         Nearest
    ///     ),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x9.8");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.mul_add_mul_prec_round_assign_ref_ref_ref(
    ///         &BigExponentFloat::from(E),
    ///         &BigExponentFloat::from(SQRT_2),
    ///         &BigExponentFloat::from(LN_2),
    ///         20,
    ///         Floor
    ///     ),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x9.851e");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.mul_add_mul_prec_round_assign_ref_ref_ref(
    ///         &BigExponentFloat::from(E),
    ///         &BigExponentFloat::from(SQRT_2),
    ///         &BigExponentFloat::from(LN_2),
    ///         20,
    ///         Ceiling
    ///     ),
    ///     Greater
    /// );
    /// assert_eq!(format!("{x:?}"), "0x9.851f");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.mul_add_mul_prec_round_assign_ref_ref_ref(
    ///         &BigExponentFloat::from(E),
    ///         &BigExponentFloat::from(SQRT_2),
    ///         &BigExponentFloat::from(LN_2),
    ///         20,
    ///         Nearest
    ///     ),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x9.851e");
    /// ```
    #[inline]
    pub fn mul_add_mul_prec_round_assign_ref_ref_ref(
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
            false,
            Some((prec, rm)),
        );
        *self = result;
        o
    }

    /// Adds the products of two pairs of [`BigExponentFloat`]s, rounding the result to the nearest
    /// value of the specified precision. All four [`BigExponentFloat`]s are taken by value. An
    /// [`Ordering`] is also returned, indicating whether the rounded result is less than, equal to,
    /// or greater than the exact result. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The products are computed exactly, so only one rounding occurs.
    ///
    /// $$
    /// f(x,y,z,w,p) = xy+zw+\varepsilon.
    /// $$
    /// - If $xy+zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy+zw$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |xy+zw|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,y,z,w,p)=\text{NaN}$ if any argument is NaN, if either product multiplies an infinity
    ///   by zero, or if $xy$ and $zw$ are infinities of opposite signs
    /// - $f(x,y,z,w,p)=\infty$ if $xy$ or $zw$ is $\infty$ and neither is $-\infty$
    /// - $f(x,y,z,w,p)=-\infty$ if $xy$ or $zw$ is $-\infty$ and neither is $\infty$
    /// - $f(x,y,z,w,p)=0$ if $xy=-zw$ and both are finite
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`BigExponentFloat::mul_add_mul_prec_round`] instead. If you want the exact result, consider
    /// using [`MulAddMul::mul_add_mul`] instead.
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
    /// let (result, o) = BigExponentFloat::from(PI).mul_add_mul_prec(
    ///     BigExponentFloat::from(E),
    ///     BigExponentFloat::from(SQRT_2),
    ///     BigExponentFloat::from(LN_2),
    ///     5,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x9.8");
    /// assert_eq!(o, Less);
    ///
    /// let (result, o) = BigExponentFloat::from(PI).mul_add_mul_prec(
    ///     BigExponentFloat::from(E),
    ///     BigExponentFloat::from(SQRT_2),
    ///     BigExponentFloat::from(LN_2),
    ///     20,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x9.851e");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn mul_add_mul_prec(self, y: Self, z: Self, w: Self, prec: u64) -> (Self, Ordering) {
        mul_add_mul_helper(
            Cow::Owned(self),
            Cow::Owned(y),
            Cow::Owned(z),
            Cow::Owned(w),
            false,
            Some((prec, Nearest)),
        )
    }

    /// Adds the products of two pairs of [`BigExponentFloat`]s, rounding the result to the nearest
    /// value of the specified precision. The first, second, and third [`BigExponentFloat`]s are
    /// taken by value and the fourth by reference. An [`Ordering`] is also returned, indicating
    /// whether the rounded result is less than, equal to, or greater than the exact result.
    /// Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// The products are computed exactly, so only one rounding occurs.
    ///
    /// $$
    /// f(x,y,z,w,p) = xy+zw+\varepsilon.
    /// $$
    /// - If $xy+zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy+zw$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |xy+zw|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,y,z,w,p)=\text{NaN}$ if any argument is NaN, if either product multiplies an infinity
    ///   by zero, or if $xy$ and $zw$ are infinities of opposite signs
    /// - $f(x,y,z,w,p)=\infty$ if $xy$ or $zw$ is $\infty$ and neither is $-\infty$
    /// - $f(x,y,z,w,p)=-\infty$ if $xy$ or $zw$ is $-\infty$ and neither is $\infty$
    /// - $f(x,y,z,w,p)=0$ if $xy=-zw$ and both are finite
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
    /// let (result, o) = BigExponentFloat::from(PI).mul_add_mul_prec_val_val_val_ref(
    ///     BigExponentFloat::from(E),
    ///     BigExponentFloat::from(SQRT_2),
    ///     &BigExponentFloat::from(LN_2),
    ///     5,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x9.8");
    /// assert_eq!(o, Less);
    ///
    /// let (result, o) = BigExponentFloat::from(PI).mul_add_mul_prec_val_val_val_ref(
    ///     BigExponentFloat::from(E),
    ///     BigExponentFloat::from(SQRT_2),
    ///     &BigExponentFloat::from(LN_2),
    ///     20,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x9.851e");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn mul_add_mul_prec_val_val_val_ref(
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
            false,
            Some((prec, Nearest)),
        )
    }

    /// Adds the products of two pairs of [`BigExponentFloat`]s, rounding the result to the nearest
    /// value of the specified precision. The first, second, and fourth [`BigExponentFloat`]s are
    /// taken by value and the third by reference. An [`Ordering`] is also returned, indicating
    /// whether the rounded result is less than, equal to, or greater than the exact result.
    /// Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// The products are computed exactly, so only one rounding occurs.
    ///
    /// $$
    /// f(x,y,z,w,p) = xy+zw+\varepsilon.
    /// $$
    /// - If $xy+zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy+zw$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |xy+zw|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,y,z,w,p)=\text{NaN}$ if any argument is NaN, if either product multiplies an infinity
    ///   by zero, or if $xy$ and $zw$ are infinities of opposite signs
    /// - $f(x,y,z,w,p)=\infty$ if $xy$ or $zw$ is $\infty$ and neither is $-\infty$
    /// - $f(x,y,z,w,p)=-\infty$ if $xy$ or $zw$ is $-\infty$ and neither is $\infty$
    /// - $f(x,y,z,w,p)=0$ if $xy=-zw$ and both are finite
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
    /// let (result, o) = BigExponentFloat::from(PI).mul_add_mul_prec_val_val_ref_val(
    ///     BigExponentFloat::from(E),
    ///     &BigExponentFloat::from(SQRT_2),
    ///     BigExponentFloat::from(LN_2),
    ///     5,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x9.8");
    /// assert_eq!(o, Less);
    ///
    /// let (result, o) = BigExponentFloat::from(PI).mul_add_mul_prec_val_val_ref_val(
    ///     BigExponentFloat::from(E),
    ///     &BigExponentFloat::from(SQRT_2),
    ///     BigExponentFloat::from(LN_2),
    ///     20,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x9.851e");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn mul_add_mul_prec_val_val_ref_val(
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
            false,
            Some((prec, Nearest)),
        )
    }

    /// Adds the products of two pairs of [`BigExponentFloat`]s, rounding the result to the nearest
    /// value of the specified precision. The first and second [`BigExponentFloat`]s are taken by
    /// value and the third and fourth by reference. An [`Ordering`] is also returned, indicating
    /// whether the rounded result is less than, equal to, or greater than the exact result.
    /// Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// The products are computed exactly, so only one rounding occurs.
    ///
    /// $$
    /// f(x,y,z,w,p) = xy+zw+\varepsilon.
    /// $$
    /// - If $xy+zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy+zw$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |xy+zw|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,y,z,w,p)=\text{NaN}$ if any argument is NaN, if either product multiplies an infinity
    ///   by zero, or if $xy$ and $zw$ are infinities of opposite signs
    /// - $f(x,y,z,w,p)=\infty$ if $xy$ or $zw$ is $\infty$ and neither is $-\infty$
    /// - $f(x,y,z,w,p)=-\infty$ if $xy$ or $zw$ is $-\infty$ and neither is $\infty$
    /// - $f(x,y,z,w,p)=0$ if $xy=-zw$ and both are finite
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
    /// let (result, o) = BigExponentFloat::from(PI).mul_add_mul_prec_val_val_ref_ref(
    ///     BigExponentFloat::from(E),
    ///     &BigExponentFloat::from(SQRT_2),
    ///     &BigExponentFloat::from(LN_2),
    ///     5,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x9.8");
    /// assert_eq!(o, Less);
    ///
    /// let (result, o) = BigExponentFloat::from(PI).mul_add_mul_prec_val_val_ref_ref(
    ///     BigExponentFloat::from(E),
    ///     &BigExponentFloat::from(SQRT_2),
    ///     &BigExponentFloat::from(LN_2),
    ///     20,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x9.851e");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn mul_add_mul_prec_val_val_ref_ref(
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
            false,
            Some((prec, Nearest)),
        )
    }

    /// Adds the products of two pairs of [`BigExponentFloat`]s, rounding the result to the nearest
    /// value of the specified precision. The first, third, and fourth [`BigExponentFloat`]s are
    /// taken by value and the second by reference. An [`Ordering`] is also returned, indicating
    /// whether the rounded result is less than, equal to, or greater than the exact result.
    /// Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// The products are computed exactly, so only one rounding occurs.
    ///
    /// $$
    /// f(x,y,z,w,p) = xy+zw+\varepsilon.
    /// $$
    /// - If $xy+zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy+zw$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |xy+zw|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,y,z,w,p)=\text{NaN}$ if any argument is NaN, if either product multiplies an infinity
    ///   by zero, or if $xy$ and $zw$ are infinities of opposite signs
    /// - $f(x,y,z,w,p)=\infty$ if $xy$ or $zw$ is $\infty$ and neither is $-\infty$
    /// - $f(x,y,z,w,p)=-\infty$ if $xy$ or $zw$ is $-\infty$ and neither is $\infty$
    /// - $f(x,y,z,w,p)=0$ if $xy=-zw$ and both are finite
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
    /// let (result, o) = BigExponentFloat::from(PI).mul_add_mul_prec_val_ref_val_val(
    ///     &BigExponentFloat::from(E),
    ///     BigExponentFloat::from(SQRT_2),
    ///     BigExponentFloat::from(LN_2),
    ///     5,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x9.8");
    /// assert_eq!(o, Less);
    ///
    /// let (result, o) = BigExponentFloat::from(PI).mul_add_mul_prec_val_ref_val_val(
    ///     &BigExponentFloat::from(E),
    ///     BigExponentFloat::from(SQRT_2),
    ///     BigExponentFloat::from(LN_2),
    ///     20,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x9.851e");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn mul_add_mul_prec_val_ref_val_val(
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
            false,
            Some((prec, Nearest)),
        )
    }

    /// Adds the products of two pairs of [`BigExponentFloat`]s, rounding the result to the nearest
    /// value of the specified precision. The first and third [`BigExponentFloat`]s are taken by
    /// value and the second and fourth by reference. An [`Ordering`] is also returned, indicating
    /// whether the rounded result is less than, equal to, or greater than the exact result.
    /// Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// The products are computed exactly, so only one rounding occurs.
    ///
    /// $$
    /// f(x,y,z,w,p) = xy+zw+\varepsilon.
    /// $$
    /// - If $xy+zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy+zw$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |xy+zw|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,y,z,w,p)=\text{NaN}$ if any argument is NaN, if either product multiplies an infinity
    ///   by zero, or if $xy$ and $zw$ are infinities of opposite signs
    /// - $f(x,y,z,w,p)=\infty$ if $xy$ or $zw$ is $\infty$ and neither is $-\infty$
    /// - $f(x,y,z,w,p)=-\infty$ if $xy$ or $zw$ is $-\infty$ and neither is $\infty$
    /// - $f(x,y,z,w,p)=0$ if $xy=-zw$ and both are finite
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
    /// let (result, o) = BigExponentFloat::from(PI).mul_add_mul_prec_val_ref_val_ref(
    ///     &BigExponentFloat::from(E),
    ///     BigExponentFloat::from(SQRT_2),
    ///     &BigExponentFloat::from(LN_2),
    ///     5,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x9.8");
    /// assert_eq!(o, Less);
    ///
    /// let (result, o) = BigExponentFloat::from(PI).mul_add_mul_prec_val_ref_val_ref(
    ///     &BigExponentFloat::from(E),
    ///     BigExponentFloat::from(SQRT_2),
    ///     &BigExponentFloat::from(LN_2),
    ///     20,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x9.851e");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn mul_add_mul_prec_val_ref_val_ref(
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
            false,
            Some((prec, Nearest)),
        )
    }

    /// Adds the products of two pairs of [`BigExponentFloat`]s, rounding the result to the nearest
    /// value of the specified precision. The first and fourth [`BigExponentFloat`]s are taken by
    /// value and the second and third by reference. An [`Ordering`] is also returned, indicating
    /// whether the rounded result is less than, equal to, or greater than the exact result.
    /// Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// The products are computed exactly, so only one rounding occurs.
    ///
    /// $$
    /// f(x,y,z,w,p) = xy+zw+\varepsilon.
    /// $$
    /// - If $xy+zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy+zw$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |xy+zw|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,y,z,w,p)=\text{NaN}$ if any argument is NaN, if either product multiplies an infinity
    ///   by zero, or if $xy$ and $zw$ are infinities of opposite signs
    /// - $f(x,y,z,w,p)=\infty$ if $xy$ or $zw$ is $\infty$ and neither is $-\infty$
    /// - $f(x,y,z,w,p)=-\infty$ if $xy$ or $zw$ is $-\infty$ and neither is $\infty$
    /// - $f(x,y,z,w,p)=0$ if $xy=-zw$ and both are finite
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
    /// let (result, o) = BigExponentFloat::from(PI).mul_add_mul_prec_val_ref_ref_val(
    ///     &BigExponentFloat::from(E),
    ///     &BigExponentFloat::from(SQRT_2),
    ///     BigExponentFloat::from(LN_2),
    ///     5,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x9.8");
    /// assert_eq!(o, Less);
    ///
    /// let (result, o) = BigExponentFloat::from(PI).mul_add_mul_prec_val_ref_ref_val(
    ///     &BigExponentFloat::from(E),
    ///     &BigExponentFloat::from(SQRT_2),
    ///     BigExponentFloat::from(LN_2),
    ///     20,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x9.851e");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn mul_add_mul_prec_val_ref_ref_val(
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
            false,
            Some((prec, Nearest)),
        )
    }

    /// Adds the products of two pairs of [`BigExponentFloat`]s, rounding the result to the nearest
    /// value of the specified precision. The first [`BigExponentFloat`] is taken by value and the
    /// second, third, and fourth by reference. An [`Ordering`] is also returned, indicating whether
    /// the rounded result is less than, equal to, or greater than the exact result. Although `NaN`s
    /// are not comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it
    /// also returns `Equal`.
    ///
    /// The products are computed exactly, so only one rounding occurs.
    ///
    /// $$
    /// f(x,y,z,w,p) = xy+zw+\varepsilon.
    /// $$
    /// - If $xy+zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy+zw$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |xy+zw|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,y,z,w,p)=\text{NaN}$ if any argument is NaN, if either product multiplies an infinity
    ///   by zero, or if $xy$ and $zw$ are infinities of opposite signs
    /// - $f(x,y,z,w,p)=\infty$ if $xy$ or $zw$ is $\infty$ and neither is $-\infty$
    /// - $f(x,y,z,w,p)=-\infty$ if $xy$ or $zw$ is $-\infty$ and neither is $\infty$
    /// - $f(x,y,z,w,p)=0$ if $xy=-zw$ and both are finite
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
    /// let (result, o) = BigExponentFloat::from(PI).mul_add_mul_prec_val_ref_ref_ref(
    ///     &BigExponentFloat::from(E),
    ///     &BigExponentFloat::from(SQRT_2),
    ///     &BigExponentFloat::from(LN_2),
    ///     5,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x9.8");
    /// assert_eq!(o, Less);
    ///
    /// let (result, o) = BigExponentFloat::from(PI).mul_add_mul_prec_val_ref_ref_ref(
    ///     &BigExponentFloat::from(E),
    ///     &BigExponentFloat::from(SQRT_2),
    ///     &BigExponentFloat::from(LN_2),
    ///     20,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x9.851e");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn mul_add_mul_prec_val_ref_ref_ref(
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
            false,
            Some((prec, Nearest)),
        )
    }

    /// Adds the products of two pairs of [`BigExponentFloat`]s, rounding the result to the nearest
    /// value of the specified precision. All four [`BigExponentFloat`]s are taken by reference. An
    /// [`Ordering`] is also returned, indicating whether the rounded result is less than, equal to,
    /// or greater than the exact result. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The products are computed exactly, so only one rounding occurs.
    ///
    /// $$
    /// f(x,y,z,w,p) = xy+zw+\varepsilon.
    /// $$
    /// - If $xy+zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy+zw$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |xy+zw|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,y,z,w,p)=\text{NaN}$ if any argument is NaN, if either product multiplies an infinity
    ///   by zero, or if $xy$ and $zw$ are infinities of opposite signs
    /// - $f(x,y,z,w,p)=\infty$ if $xy$ or $zw$ is $\infty$ and neither is $-\infty$
    /// - $f(x,y,z,w,p)=-\infty$ if $xy$ or $zw$ is $-\infty$ and neither is $\infty$
    /// - $f(x,y,z,w,p)=0$ if $xy=-zw$ and both are finite
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
    /// let (result, o) = (&BigExponentFloat::from(PI)).mul_add_mul_prec_ref_ref_ref_ref(
    ///     &BigExponentFloat::from(E),
    ///     &BigExponentFloat::from(SQRT_2),
    ///     &BigExponentFloat::from(LN_2),
    ///     5,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x9.8");
    /// assert_eq!(o, Less);
    ///
    /// let (result, o) = (&BigExponentFloat::from(PI)).mul_add_mul_prec_ref_ref_ref_ref(
    ///     &BigExponentFloat::from(E),
    ///     &BigExponentFloat::from(SQRT_2),
    ///     &BigExponentFloat::from(LN_2),
    ///     20,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x9.851e");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn mul_add_mul_prec_ref_ref_ref_ref(
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
            false,
            Some((prec, Nearest)),
        )
    }

    /// Adds the product of two [`BigExponentFloat`]s to a [`BigExponentFloat`] multiplied by a
    /// [`BigExponentFloat`], in place, rounding the result to the nearest value of the specified
    /// precision. The [`BigExponentFloat`]s on the right-hand side are taken by value. An
    /// [`Ordering`] is returned, indicating whether the rounded result is less than, equal to, or
    /// greater than the exact result. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function sets the [`BigExponentFloat`] to `NaN` it also
    /// returns `Equal`.
    ///
    /// The products are computed exactly, so only one rounding occurs.
    ///
    /// $$
    /// x \gets xy+zw+\varepsilon.
    /// $$
    /// - If $xy+zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy+zw$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |xy+zw|\rfloor-p}$.
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
    ///     x.mul_add_mul_prec_assign(
    ///         BigExponentFloat::from(E),
    ///         BigExponentFloat::from(SQRT_2),
    ///         BigExponentFloat::from(LN_2),
    ///         5
    ///     ),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x9.8");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.mul_add_mul_prec_assign(
    ///         BigExponentFloat::from(E),
    ///         BigExponentFloat::from(SQRT_2),
    ///         BigExponentFloat::from(LN_2),
    ///         20
    ///     ),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x9.851e");
    /// ```
    #[inline]
    pub fn mul_add_mul_prec_assign(&mut self, y: Self, z: Self, w: Self, prec: u64) -> Ordering {
        let x = core::mem::replace(self, Self::ZERO);
        let (result, o) = mul_add_mul_helper(
            Cow::Owned(x),
            Cow::Owned(y),
            Cow::Owned(z),
            Cow::Owned(w),
            false,
            Some((prec, Nearest)),
        );
        *self = result;
        o
    }

    /// Adds the product of two [`BigExponentFloat`]s to a [`BigExponentFloat`] multiplied by a
    /// [`BigExponentFloat`], in place, rounding the result to the nearest value of the specified
    /// precision. On the right-hand side, the first and second [`BigExponentFloat`]s are taken by
    /// value and the third by reference. An [`Ordering`] is returned, indicating whether the
    /// rounded result is less than, equal to, or greater than the exact result. Although `NaN`s are
    /// not comparable to any [`BigExponentFloat`], whenever this function sets the
    /// [`BigExponentFloat`] to `NaN` it also returns `Equal`.
    ///
    /// The products are computed exactly, so only one rounding occurs.
    ///
    /// $$
    /// x \gets xy+zw+\varepsilon.
    /// $$
    /// - If $xy+zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy+zw$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |xy+zw|\rfloor-p}$.
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
    ///     x.mul_add_mul_prec_assign_val_val_ref(
    ///         BigExponentFloat::from(E),
    ///         BigExponentFloat::from(SQRT_2),
    ///         &BigExponentFloat::from(LN_2),
    ///         5
    ///     ),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x9.8");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.mul_add_mul_prec_assign_val_val_ref(
    ///         BigExponentFloat::from(E),
    ///         BigExponentFloat::from(SQRT_2),
    ///         &BigExponentFloat::from(LN_2),
    ///         20
    ///     ),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x9.851e");
    /// ```
    #[inline]
    pub fn mul_add_mul_prec_assign_val_val_ref(
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
            false,
            Some((prec, Nearest)),
        );
        *self = result;
        o
    }

    /// Adds the product of two [`BigExponentFloat`]s to a [`BigExponentFloat`] multiplied by a
    /// [`BigExponentFloat`], in place, rounding the result to the nearest value of the specified
    /// precision. On the right-hand side, the first and third [`BigExponentFloat`]s are taken by
    /// value and the second by reference. An [`Ordering`] is returned, indicating whether the
    /// rounded result is less than, equal to, or greater than the exact result. Although `NaN`s are
    /// not comparable to any [`BigExponentFloat`], whenever this function sets the
    /// [`BigExponentFloat`] to `NaN` it also returns `Equal`.
    ///
    /// The products are computed exactly, so only one rounding occurs.
    ///
    /// $$
    /// x \gets xy+zw+\varepsilon.
    /// $$
    /// - If $xy+zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy+zw$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |xy+zw|\rfloor-p}$.
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
    ///     x.mul_add_mul_prec_assign_val_ref_val(
    ///         BigExponentFloat::from(E),
    ///         &BigExponentFloat::from(SQRT_2),
    ///         BigExponentFloat::from(LN_2),
    ///         5
    ///     ),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x9.8");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.mul_add_mul_prec_assign_val_ref_val(
    ///         BigExponentFloat::from(E),
    ///         &BigExponentFloat::from(SQRT_2),
    ///         BigExponentFloat::from(LN_2),
    ///         20
    ///     ),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x9.851e");
    /// ```
    #[inline]
    pub fn mul_add_mul_prec_assign_val_ref_val(
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
            false,
            Some((prec, Nearest)),
        );
        *self = result;
        o
    }

    /// Adds the product of two [`BigExponentFloat`]s to a [`BigExponentFloat`] multiplied by a
    /// [`BigExponentFloat`], in place, rounding the result to the nearest value of the specified
    /// precision. On the right-hand side, the first [`BigExponentFloat`] is taken by value and the
    /// second and third by reference. An [`Ordering`] is returned, indicating whether the rounded
    /// result is less than, equal to, or greater than the exact result. Although `NaN`s are not
    /// comparable to any [`BigExponentFloat`], whenever this function sets the [`BigExponentFloat`]
    /// to `NaN` it also returns `Equal`.
    ///
    /// The products are computed exactly, so only one rounding occurs.
    ///
    /// $$
    /// x \gets xy+zw+\varepsilon.
    /// $$
    /// - If $xy+zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy+zw$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |xy+zw|\rfloor-p}$.
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
    ///     x.mul_add_mul_prec_assign_val_ref_ref(
    ///         BigExponentFloat::from(E),
    ///         &BigExponentFloat::from(SQRT_2),
    ///         &BigExponentFloat::from(LN_2),
    ///         5
    ///     ),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x9.8");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.mul_add_mul_prec_assign_val_ref_ref(
    ///         BigExponentFloat::from(E),
    ///         &BigExponentFloat::from(SQRT_2),
    ///         &BigExponentFloat::from(LN_2),
    ///         20
    ///     ),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x9.851e");
    /// ```
    #[inline]
    pub fn mul_add_mul_prec_assign_val_ref_ref(
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
            false,
            Some((prec, Nearest)),
        );
        *self = result;
        o
    }

    /// Adds the product of two [`BigExponentFloat`]s to a [`BigExponentFloat`] multiplied by a
    /// [`BigExponentFloat`], in place, rounding the result to the nearest value of the specified
    /// precision. On the right-hand side, the second and third [`BigExponentFloat`]s are taken by
    /// value and the first by reference. An [`Ordering`] is returned, indicating whether the
    /// rounded result is less than, equal to, or greater than the exact result. Although `NaN`s are
    /// not comparable to any [`BigExponentFloat`], whenever this function sets the
    /// [`BigExponentFloat`] to `NaN` it also returns `Equal`.
    ///
    /// The products are computed exactly, so only one rounding occurs.
    ///
    /// $$
    /// x \gets xy+zw+\varepsilon.
    /// $$
    /// - If $xy+zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy+zw$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |xy+zw|\rfloor-p}$.
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
    ///     x.mul_add_mul_prec_assign_ref_val_val(
    ///         &BigExponentFloat::from(E),
    ///         BigExponentFloat::from(SQRT_2),
    ///         BigExponentFloat::from(LN_2),
    ///         5
    ///     ),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x9.8");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.mul_add_mul_prec_assign_ref_val_val(
    ///         &BigExponentFloat::from(E),
    ///         BigExponentFloat::from(SQRT_2),
    ///         BigExponentFloat::from(LN_2),
    ///         20
    ///     ),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x9.851e");
    /// ```
    #[inline]
    pub fn mul_add_mul_prec_assign_ref_val_val(
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
            false,
            Some((prec, Nearest)),
        );
        *self = result;
        o
    }

    /// Adds the product of two [`BigExponentFloat`]s to a [`BigExponentFloat`] multiplied by a
    /// [`BigExponentFloat`], in place, rounding the result to the nearest value of the specified
    /// precision. On the right-hand side, the second [`BigExponentFloat`] is taken by value and the
    /// first and third by reference. An [`Ordering`] is returned, indicating whether the rounded
    /// result is less than, equal to, or greater than the exact result. Although `NaN`s are not
    /// comparable to any [`BigExponentFloat`], whenever this function sets the [`BigExponentFloat`]
    /// to `NaN` it also returns `Equal`.
    ///
    /// The products are computed exactly, so only one rounding occurs.
    ///
    /// $$
    /// x \gets xy+zw+\varepsilon.
    /// $$
    /// - If $xy+zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy+zw$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |xy+zw|\rfloor-p}$.
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
    ///     x.mul_add_mul_prec_assign_ref_val_ref(
    ///         &BigExponentFloat::from(E),
    ///         BigExponentFloat::from(SQRT_2),
    ///         &BigExponentFloat::from(LN_2),
    ///         5
    ///     ),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x9.8");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.mul_add_mul_prec_assign_ref_val_ref(
    ///         &BigExponentFloat::from(E),
    ///         BigExponentFloat::from(SQRT_2),
    ///         &BigExponentFloat::from(LN_2),
    ///         20
    ///     ),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x9.851e");
    /// ```
    #[inline]
    pub fn mul_add_mul_prec_assign_ref_val_ref(
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
            false,
            Some((prec, Nearest)),
        );
        *self = result;
        o
    }

    /// Adds the product of two [`BigExponentFloat`]s to a [`BigExponentFloat`] multiplied by a
    /// [`BigExponentFloat`], in place, rounding the result to the nearest value of the specified
    /// precision. On the right-hand side, the third [`BigExponentFloat`] is taken by value and the
    /// first and second by reference. An [`Ordering`] is returned, indicating whether the rounded
    /// result is less than, equal to, or greater than the exact result. Although `NaN`s are not
    /// comparable to any [`BigExponentFloat`], whenever this function sets the [`BigExponentFloat`]
    /// to `NaN` it also returns `Equal`.
    ///
    /// The products are computed exactly, so only one rounding occurs.
    ///
    /// $$
    /// x \gets xy+zw+\varepsilon.
    /// $$
    /// - If $xy+zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy+zw$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |xy+zw|\rfloor-p}$.
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
    ///     x.mul_add_mul_prec_assign_ref_ref_val(
    ///         &BigExponentFloat::from(E),
    ///         &BigExponentFloat::from(SQRT_2),
    ///         BigExponentFloat::from(LN_2),
    ///         5
    ///     ),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x9.8");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.mul_add_mul_prec_assign_ref_ref_val(
    ///         &BigExponentFloat::from(E),
    ///         &BigExponentFloat::from(SQRT_2),
    ///         BigExponentFloat::from(LN_2),
    ///         20
    ///     ),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x9.851e");
    /// ```
    #[inline]
    pub fn mul_add_mul_prec_assign_ref_ref_val(
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
            false,
            Some((prec, Nearest)),
        );
        *self = result;
        o
    }

    /// Adds the product of two [`BigExponentFloat`]s to a [`BigExponentFloat`] multiplied by a
    /// [`BigExponentFloat`], in place, rounding the result to the nearest value of the specified
    /// precision. The [`BigExponentFloat`]s on the right-hand side are taken by reference. An
    /// [`Ordering`] is returned, indicating whether the rounded result is less than, equal to, or
    /// greater than the exact result. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function sets the [`BigExponentFloat`] to `NaN` it also
    /// returns `Equal`.
    ///
    /// The products are computed exactly, so only one rounding occurs.
    ///
    /// $$
    /// x \gets xy+zw+\varepsilon.
    /// $$
    /// - If $xy+zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy+zw$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |xy+zw|\rfloor-p}$.
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
    ///     x.mul_add_mul_prec_assign_ref_ref_ref(
    ///         &BigExponentFloat::from(E),
    ///         &BigExponentFloat::from(SQRT_2),
    ///         &BigExponentFloat::from(LN_2),
    ///         5
    ///     ),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x9.8");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.mul_add_mul_prec_assign_ref_ref_ref(
    ///         &BigExponentFloat::from(E),
    ///         &BigExponentFloat::from(SQRT_2),
    ///         &BigExponentFloat::from(LN_2),
    ///         20
    ///     ),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x9.851e");
    /// ```
    #[inline]
    pub fn mul_add_mul_prec_assign_ref_ref_ref(
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
            false,
            Some((prec, Nearest)),
        );
        *self = result;
        o
    }
}

impl MulAddMul<Self, Self, Self> for BigExponentFloat {
    type Output = Self;

    /// Adds the products of two pairs of [`BigExponentFloat`]s. All four [`BigExponentFloat`]s are
    /// taken by value. The result is exact.
    ///
    /// $$
    /// f(x,y,z,w) = xy+zw.
    /// $$
    ///
    /// Special cases:
    /// - $f(x,y,z,w)=\text{NaN}$ if any argument is NaN, if either product multiplies an infinity
    ///   by zero, or if $xy$ and $zw$ are infinities of opposite signs
    /// - $f(x,y,z,w)=\infty$ if $xy$ or $zw$ is $\infty$ and neither is $-\infty$
    /// - $f(x,y,z,w)=-\infty$ if $xy$ or $zw$ is $-\infty$ and neither is $\infty$
    /// - $f(x,y,z,w)=0$ if $xy=-zw$ and both are finite
    ///
    /// When the two terms being added are finite and nonzero, the significand of the result spans
    /// the gap between their exponents. To bound its size, use
    /// [`BigExponentFloat::mul_add_mul_prec_round`] or [`BigExponentFloat::mul_add_mul_prec`]
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
    /// use malachite_base::num::arithmetic::traits::MulAddMul;
    /// use malachite_float::BigExponentFloat;
    ///
    /// assert_eq!(
    ///     format!(
    ///         "{:?}",
    ///         BigExponentFloat::from(1.5).mul_add_mul(
    ///             BigExponentFloat::from(0.25),
    ///             BigExponentFloat::from(3.0),
    ///             BigExponentFloat::from(-0.5)
    ///         )
    ///     ),
    ///     "-0x1.2"
    /// );
    /// ```
    #[inline]
    fn mul_add_mul(self, y: Self, z: Self, w: Self) -> Self {
        mul_add_mul_helper(
            Cow::Owned(self),
            Cow::Owned(y),
            Cow::Owned(z),
            Cow::Owned(w),
            false,
            None,
        )
        .0
    }
}

impl MulAddMul<Self, Self, &Self> for BigExponentFloat {
    type Output = Self;

    /// Adds the products of two pairs of [`BigExponentFloat`]s. The first, second, and third
    /// [`BigExponentFloat`]s are taken by value and the fourth by reference. The result is exact.
    ///
    /// $$
    /// f(x,y,z,w) = xy+zw.
    /// $$
    ///
    /// Special cases:
    /// - $f(x,y,z,w)=\text{NaN}$ if any argument is NaN, if either product multiplies an infinity
    ///   by zero, or if $xy$ and $zw$ are infinities of opposite signs
    /// - $f(x,y,z,w)=\infty$ if $xy$ or $zw$ is $\infty$ and neither is $-\infty$
    /// - $f(x,y,z,w)=-\infty$ if $xy$ or $zw$ is $-\infty$ and neither is $\infty$
    /// - $f(x,y,z,w)=0$ if $xy=-zw$ and both are finite
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
    /// use malachite_base::num::arithmetic::traits::MulAddMul;
    /// use malachite_float::BigExponentFloat;
    ///
    /// assert_eq!(
    ///     format!(
    ///         "{:?}",
    ///         BigExponentFloat::from(1.5).mul_add_mul(
    ///             BigExponentFloat::from(0.25),
    ///             BigExponentFloat::from(3.0),
    ///             &BigExponentFloat::from(-0.5)
    ///         )
    ///     ),
    ///     "-0x1.2"
    /// );
    /// ```
    #[inline]
    fn mul_add_mul(self, y: Self, z: Self, w: &Self) -> Self {
        mul_add_mul_helper(
            Cow::Owned(self),
            Cow::Owned(y),
            Cow::Owned(z),
            Cow::Borrowed(w),
            false,
            None,
        )
        .0
    }
}

impl MulAddMul<Self, &Self, Self> for BigExponentFloat {
    type Output = Self;

    /// Adds the products of two pairs of [`BigExponentFloat`]s. The first, second, and fourth
    /// [`BigExponentFloat`]s are taken by value and the third by reference. The result is exact.
    ///
    /// $$
    /// f(x,y,z,w) = xy+zw.
    /// $$
    ///
    /// Special cases:
    /// - $f(x,y,z,w)=\text{NaN}$ if any argument is NaN, if either product multiplies an infinity
    ///   by zero, or if $xy$ and $zw$ are infinities of opposite signs
    /// - $f(x,y,z,w)=\infty$ if $xy$ or $zw$ is $\infty$ and neither is $-\infty$
    /// - $f(x,y,z,w)=-\infty$ if $xy$ or $zw$ is $-\infty$ and neither is $\infty$
    /// - $f(x,y,z,w)=0$ if $xy=-zw$ and both are finite
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
    /// use malachite_base::num::arithmetic::traits::MulAddMul;
    /// use malachite_float::BigExponentFloat;
    ///
    /// assert_eq!(
    ///     format!(
    ///         "{:?}",
    ///         BigExponentFloat::from(1.5).mul_add_mul(
    ///             BigExponentFloat::from(0.25),
    ///             &BigExponentFloat::from(3.0),
    ///             BigExponentFloat::from(-0.5)
    ///         )
    ///     ),
    ///     "-0x1.2"
    /// );
    /// ```
    #[inline]
    fn mul_add_mul(self, y: Self, z: &Self, w: Self) -> Self {
        mul_add_mul_helper(
            Cow::Owned(self),
            Cow::Owned(y),
            Cow::Borrowed(z),
            Cow::Owned(w),
            false,
            None,
        )
        .0
    }
}

impl MulAddMul<Self, &Self, &Self> for BigExponentFloat {
    type Output = Self;

    /// Adds the products of two pairs of [`BigExponentFloat`]s. The first and second
    /// [`BigExponentFloat`]s are taken by value and the third and fourth by reference. The result
    /// is exact.
    ///
    /// $$
    /// f(x,y,z,w) = xy+zw.
    /// $$
    ///
    /// Special cases:
    /// - $f(x,y,z,w)=\text{NaN}$ if any argument is NaN, if either product multiplies an infinity
    ///   by zero, or if $xy$ and $zw$ are infinities of opposite signs
    /// - $f(x,y,z,w)=\infty$ if $xy$ or $zw$ is $\infty$ and neither is $-\infty$
    /// - $f(x,y,z,w)=-\infty$ if $xy$ or $zw$ is $-\infty$ and neither is $\infty$
    /// - $f(x,y,z,w)=0$ if $xy=-zw$ and both are finite
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
    /// use malachite_base::num::arithmetic::traits::MulAddMul;
    /// use malachite_float::BigExponentFloat;
    ///
    /// assert_eq!(
    ///     format!(
    ///         "{:?}",
    ///         BigExponentFloat::from(1.5).mul_add_mul(
    ///             BigExponentFloat::from(0.25),
    ///             &BigExponentFloat::from(3.0),
    ///             &BigExponentFloat::from(-0.5)
    ///         )
    ///     ),
    ///     "-0x1.2"
    /// );
    /// ```
    #[inline]
    fn mul_add_mul(self, y: Self, z: &Self, w: &Self) -> Self {
        mul_add_mul_helper(
            Cow::Owned(self),
            Cow::Owned(y),
            Cow::Borrowed(z),
            Cow::Borrowed(w),
            false,
            None,
        )
        .0
    }
}

impl MulAddMul<&Self, Self, Self> for BigExponentFloat {
    type Output = Self;

    /// Adds the products of two pairs of [`BigExponentFloat`]s. The first, third, and fourth
    /// [`BigExponentFloat`]s are taken by value and the second by reference. The result is exact.
    ///
    /// $$
    /// f(x,y,z,w) = xy+zw.
    /// $$
    ///
    /// Special cases:
    /// - $f(x,y,z,w)=\text{NaN}$ if any argument is NaN, if either product multiplies an infinity
    ///   by zero, or if $xy$ and $zw$ are infinities of opposite signs
    /// - $f(x,y,z,w)=\infty$ if $xy$ or $zw$ is $\infty$ and neither is $-\infty$
    /// - $f(x,y,z,w)=-\infty$ if $xy$ or $zw$ is $-\infty$ and neither is $\infty$
    /// - $f(x,y,z,w)=0$ if $xy=-zw$ and both are finite
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
    /// use malachite_base::num::arithmetic::traits::MulAddMul;
    /// use malachite_float::BigExponentFloat;
    ///
    /// assert_eq!(
    ///     format!(
    ///         "{:?}",
    ///         BigExponentFloat::from(1.5).mul_add_mul(
    ///             &BigExponentFloat::from(0.25),
    ///             BigExponentFloat::from(3.0),
    ///             BigExponentFloat::from(-0.5)
    ///         )
    ///     ),
    ///     "-0x1.2"
    /// );
    /// ```
    #[inline]
    fn mul_add_mul(self, y: &Self, z: Self, w: Self) -> Self {
        mul_add_mul_helper(
            Cow::Owned(self),
            Cow::Borrowed(y),
            Cow::Owned(z),
            Cow::Owned(w),
            false,
            None,
        )
        .0
    }
}

impl MulAddMul<&Self, Self, &Self> for BigExponentFloat {
    type Output = Self;

    /// Adds the products of two pairs of [`BigExponentFloat`]s. The first and third
    /// [`BigExponentFloat`]s are taken by value and the second and fourth by reference. The result
    /// is exact.
    ///
    /// $$
    /// f(x,y,z,w) = xy+zw.
    /// $$
    ///
    /// Special cases:
    /// - $f(x,y,z,w)=\text{NaN}$ if any argument is NaN, if either product multiplies an infinity
    ///   by zero, or if $xy$ and $zw$ are infinities of opposite signs
    /// - $f(x,y,z,w)=\infty$ if $xy$ or $zw$ is $\infty$ and neither is $-\infty$
    /// - $f(x,y,z,w)=-\infty$ if $xy$ or $zw$ is $-\infty$ and neither is $\infty$
    /// - $f(x,y,z,w)=0$ if $xy=-zw$ and both are finite
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
    /// use malachite_base::num::arithmetic::traits::MulAddMul;
    /// use malachite_float::BigExponentFloat;
    ///
    /// assert_eq!(
    ///     format!(
    ///         "{:?}",
    ///         BigExponentFloat::from(1.5).mul_add_mul(
    ///             &BigExponentFloat::from(0.25),
    ///             BigExponentFloat::from(3.0),
    ///             &BigExponentFloat::from(-0.5)
    ///         )
    ///     ),
    ///     "-0x1.2"
    /// );
    /// ```
    #[inline]
    fn mul_add_mul(self, y: &Self, z: Self, w: &Self) -> Self {
        mul_add_mul_helper(
            Cow::Owned(self),
            Cow::Borrowed(y),
            Cow::Owned(z),
            Cow::Borrowed(w),
            false,
            None,
        )
        .0
    }
}

impl MulAddMul<&Self, &Self, Self> for BigExponentFloat {
    type Output = Self;

    /// Adds the products of two pairs of [`BigExponentFloat`]s. The first and fourth
    /// [`BigExponentFloat`]s are taken by value and the second and third by reference. The result
    /// is exact.
    ///
    /// $$
    /// f(x,y,z,w) = xy+zw.
    /// $$
    ///
    /// Special cases:
    /// - $f(x,y,z,w)=\text{NaN}$ if any argument is NaN, if either product multiplies an infinity
    ///   by zero, or if $xy$ and $zw$ are infinities of opposite signs
    /// - $f(x,y,z,w)=\infty$ if $xy$ or $zw$ is $\infty$ and neither is $-\infty$
    /// - $f(x,y,z,w)=-\infty$ if $xy$ or $zw$ is $-\infty$ and neither is $\infty$
    /// - $f(x,y,z,w)=0$ if $xy=-zw$ and both are finite
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
    /// use malachite_base::num::arithmetic::traits::MulAddMul;
    /// use malachite_float::BigExponentFloat;
    ///
    /// assert_eq!(
    ///     format!(
    ///         "{:?}",
    ///         BigExponentFloat::from(1.5).mul_add_mul(
    ///             &BigExponentFloat::from(0.25),
    ///             &BigExponentFloat::from(3.0),
    ///             BigExponentFloat::from(-0.5)
    ///         )
    ///     ),
    ///     "-0x1.2"
    /// );
    /// ```
    #[inline]
    fn mul_add_mul(self, y: &Self, z: &Self, w: Self) -> Self {
        mul_add_mul_helper(
            Cow::Owned(self),
            Cow::Borrowed(y),
            Cow::Borrowed(z),
            Cow::Owned(w),
            false,
            None,
        )
        .0
    }
}

impl MulAddMul<&Self, &Self, &Self> for BigExponentFloat {
    type Output = Self;

    /// Adds the products of two pairs of [`BigExponentFloat`]s. The first [`BigExponentFloat`] is
    /// taken by value and the second, third, and fourth by reference. The result is exact.
    ///
    /// $$
    /// f(x,y,z,w) = xy+zw.
    /// $$
    ///
    /// Special cases:
    /// - $f(x,y,z,w)=\text{NaN}$ if any argument is NaN, if either product multiplies an infinity
    ///   by zero, or if $xy$ and $zw$ are infinities of opposite signs
    /// - $f(x,y,z,w)=\infty$ if $xy$ or $zw$ is $\infty$ and neither is $-\infty$
    /// - $f(x,y,z,w)=-\infty$ if $xy$ or $zw$ is $-\infty$ and neither is $\infty$
    /// - $f(x,y,z,w)=0$ if $xy=-zw$ and both are finite
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
    /// use malachite_base::num::arithmetic::traits::MulAddMul;
    /// use malachite_float::BigExponentFloat;
    ///
    /// assert_eq!(
    ///     format!(
    ///         "{:?}",
    ///         BigExponentFloat::from(1.5).mul_add_mul(
    ///             &BigExponentFloat::from(0.25),
    ///             &BigExponentFloat::from(3.0),
    ///             &BigExponentFloat::from(-0.5)
    ///         )
    ///     ),
    ///     "-0x1.2"
    /// );
    /// ```
    #[inline]
    fn mul_add_mul(self, y: &Self, z: &Self, w: &Self) -> Self {
        mul_add_mul_helper(
            Cow::Owned(self),
            Cow::Borrowed(y),
            Cow::Borrowed(z),
            Cow::Borrowed(w),
            false,
            None,
        )
        .0
    }
}

impl MulAddMul<&BigExponentFloat, &BigExponentFloat, &BigExponentFloat> for &BigExponentFloat {
    type Output = BigExponentFloat;

    /// Adds the products of two pairs of [`BigExponentFloat`]s. All four [`BigExponentFloat`]s are
    /// taken by reference. The result is exact.
    ///
    /// $$
    /// f(x,y,z,w) = xy+zw.
    /// $$
    ///
    /// Special cases:
    /// - $f(x,y,z,w)=\text{NaN}$ if any argument is NaN, if either product multiplies an infinity
    ///   by zero, or if $xy$ and $zw$ are infinities of opposite signs
    /// - $f(x,y,z,w)=\infty$ if $xy$ or $zw$ is $\infty$ and neither is $-\infty$
    /// - $f(x,y,z,w)=-\infty$ if $xy$ or $zw$ is $-\infty$ and neither is $\infty$
    /// - $f(x,y,z,w)=0$ if $xy=-zw$ and both are finite
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
    /// use malachite_base::num::arithmetic::traits::MulAddMul;
    /// use malachite_float::BigExponentFloat;
    ///
    /// assert_eq!(
    ///     format!(
    ///         "{:?}",
    ///         (&BigExponentFloat::from(1.5)).mul_add_mul(
    ///             &BigExponentFloat::from(0.25),
    ///             &BigExponentFloat::from(3.0),
    ///             &BigExponentFloat::from(-0.5)
    ///         )
    ///     ),
    ///     "-0x1.2"
    /// );
    /// ```
    #[inline]
    fn mul_add_mul(
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
            false,
            None,
        )
        .0
    }
}

impl MulAddMulAssign<Self, Self, Self> for BigExponentFloat {
    /// Adds the product of two [`BigExponentFloat`]s to a [`BigExponentFloat`] multiplied by a
    /// [`BigExponentFloat`], in place. The [`BigExponentFloat`]s on the right-hand side are taken
    /// by value. The result is exact.
    ///
    /// $$
    /// x \gets xy+zw.
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
    /// use malachite_base::num::arithmetic::traits::MulAddMulAssign;
    /// use malachite_float::BigExponentFloat;
    ///
    /// let mut x = BigExponentFloat::from(1.5);
    /// x.mul_add_mul_assign(
    ///     BigExponentFloat::from(0.25),
    ///     BigExponentFloat::from(3.0),
    ///     BigExponentFloat::from(-0.5),
    /// );
    /// assert_eq!(format!("{x:?}"), "-0x1.2");
    /// ```
    #[inline]
    fn mul_add_mul_assign(&mut self, y: Self, z: Self, w: Self) {
        let x = core::mem::replace(self, Self::ZERO);
        *self = mul_add_mul_helper(
            Cow::Owned(x),
            Cow::Owned(y),
            Cow::Owned(z),
            Cow::Owned(w),
            false,
            None,
        )
        .0;
    }
}

impl MulAddMulAssign<Self, Self, &Self> for BigExponentFloat {
    /// Adds the product of two [`BigExponentFloat`]s to a [`BigExponentFloat`] multiplied by a
    /// [`BigExponentFloat`], in place. On the right-hand side, the first and second
    /// [`BigExponentFloat`]s are taken by value and the third by reference. The result is exact.
    ///
    /// $$
    /// x \gets xy+zw.
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
    /// use malachite_base::num::arithmetic::traits::MulAddMulAssign;
    /// use malachite_float::BigExponentFloat;
    ///
    /// let mut x = BigExponentFloat::from(1.5);
    /// x.mul_add_mul_assign(
    ///     BigExponentFloat::from(0.25),
    ///     BigExponentFloat::from(3.0),
    ///     &BigExponentFloat::from(-0.5),
    /// );
    /// assert_eq!(format!("{x:?}"), "-0x1.2");
    /// ```
    #[inline]
    fn mul_add_mul_assign(&mut self, y: Self, z: Self, w: &Self) {
        let x = core::mem::replace(self, Self::ZERO);
        *self = mul_add_mul_helper(
            Cow::Owned(x),
            Cow::Owned(y),
            Cow::Owned(z),
            Cow::Borrowed(w),
            false,
            None,
        )
        .0;
    }
}

impl MulAddMulAssign<Self, &Self, Self> for BigExponentFloat {
    /// Adds the product of two [`BigExponentFloat`]s to a [`BigExponentFloat`] multiplied by a
    /// [`BigExponentFloat`], in place. On the right-hand side, the first and third
    /// [`BigExponentFloat`]s are taken by value and the second by reference. The result is exact.
    ///
    /// $$
    /// x \gets xy+zw.
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
    /// use malachite_base::num::arithmetic::traits::MulAddMulAssign;
    /// use malachite_float::BigExponentFloat;
    ///
    /// let mut x = BigExponentFloat::from(1.5);
    /// x.mul_add_mul_assign(
    ///     BigExponentFloat::from(0.25),
    ///     &BigExponentFloat::from(3.0),
    ///     BigExponentFloat::from(-0.5),
    /// );
    /// assert_eq!(format!("{x:?}"), "-0x1.2");
    /// ```
    #[inline]
    fn mul_add_mul_assign(&mut self, y: Self, z: &Self, w: Self) {
        let x = core::mem::replace(self, Self::ZERO);
        *self = mul_add_mul_helper(
            Cow::Owned(x),
            Cow::Owned(y),
            Cow::Borrowed(z),
            Cow::Owned(w),
            false,
            None,
        )
        .0;
    }
}

impl MulAddMulAssign<Self, &Self, &Self> for BigExponentFloat {
    /// Adds the product of two [`BigExponentFloat`]s to a [`BigExponentFloat`] multiplied by a
    /// [`BigExponentFloat`], in place. On the right-hand side, the first [`BigExponentFloat`] is
    /// taken by value and the second and third by reference. The result is exact.
    ///
    /// $$
    /// x \gets xy+zw.
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
    /// use malachite_base::num::arithmetic::traits::MulAddMulAssign;
    /// use malachite_float::BigExponentFloat;
    ///
    /// let mut x = BigExponentFloat::from(1.5);
    /// x.mul_add_mul_assign(
    ///     BigExponentFloat::from(0.25),
    ///     &BigExponentFloat::from(3.0),
    ///     &BigExponentFloat::from(-0.5),
    /// );
    /// assert_eq!(format!("{x:?}"), "-0x1.2");
    /// ```
    #[inline]
    fn mul_add_mul_assign(&mut self, y: Self, z: &Self, w: &Self) {
        let x = core::mem::replace(self, Self::ZERO);
        *self = mul_add_mul_helper(
            Cow::Owned(x),
            Cow::Owned(y),
            Cow::Borrowed(z),
            Cow::Borrowed(w),
            false,
            None,
        )
        .0;
    }
}

impl MulAddMulAssign<&Self, Self, Self> for BigExponentFloat {
    /// Adds the product of two [`BigExponentFloat`]s to a [`BigExponentFloat`] multiplied by a
    /// [`BigExponentFloat`], in place. On the right-hand side, the second and third
    /// [`BigExponentFloat`]s are taken by value and the first by reference. The result is exact.
    ///
    /// $$
    /// x \gets xy+zw.
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
    /// use malachite_base::num::arithmetic::traits::MulAddMulAssign;
    /// use malachite_float::BigExponentFloat;
    ///
    /// let mut x = BigExponentFloat::from(1.5);
    /// x.mul_add_mul_assign(
    ///     &BigExponentFloat::from(0.25),
    ///     BigExponentFloat::from(3.0),
    ///     BigExponentFloat::from(-0.5),
    /// );
    /// assert_eq!(format!("{x:?}"), "-0x1.2");
    /// ```
    #[inline]
    fn mul_add_mul_assign(&mut self, y: &Self, z: Self, w: Self) {
        let x = core::mem::replace(self, Self::ZERO);
        *self = mul_add_mul_helper(
            Cow::Owned(x),
            Cow::Borrowed(y),
            Cow::Owned(z),
            Cow::Owned(w),
            false,
            None,
        )
        .0;
    }
}

impl MulAddMulAssign<&Self, Self, &Self> for BigExponentFloat {
    /// Adds the product of two [`BigExponentFloat`]s to a [`BigExponentFloat`] multiplied by a
    /// [`BigExponentFloat`], in place. On the right-hand side, the second [`BigExponentFloat`] is
    /// taken by value and the first and third by reference. The result is exact.
    ///
    /// $$
    /// x \gets xy+zw.
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
    /// use malachite_base::num::arithmetic::traits::MulAddMulAssign;
    /// use malachite_float::BigExponentFloat;
    ///
    /// let mut x = BigExponentFloat::from(1.5);
    /// x.mul_add_mul_assign(
    ///     &BigExponentFloat::from(0.25),
    ///     BigExponentFloat::from(3.0),
    ///     &BigExponentFloat::from(-0.5),
    /// );
    /// assert_eq!(format!("{x:?}"), "-0x1.2");
    /// ```
    #[inline]
    fn mul_add_mul_assign(&mut self, y: &Self, z: Self, w: &Self) {
        let x = core::mem::replace(self, Self::ZERO);
        *self = mul_add_mul_helper(
            Cow::Owned(x),
            Cow::Borrowed(y),
            Cow::Owned(z),
            Cow::Borrowed(w),
            false,
            None,
        )
        .0;
    }
}

impl MulAddMulAssign<&Self, &Self, Self> for BigExponentFloat {
    /// Adds the product of two [`BigExponentFloat`]s to a [`BigExponentFloat`] multiplied by a
    /// [`BigExponentFloat`], in place. On the right-hand side, the third [`BigExponentFloat`] is
    /// taken by value and the first and second by reference. The result is exact.
    ///
    /// $$
    /// x \gets xy+zw.
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
    /// use malachite_base::num::arithmetic::traits::MulAddMulAssign;
    /// use malachite_float::BigExponentFloat;
    ///
    /// let mut x = BigExponentFloat::from(1.5);
    /// x.mul_add_mul_assign(
    ///     &BigExponentFloat::from(0.25),
    ///     &BigExponentFloat::from(3.0),
    ///     BigExponentFloat::from(-0.5),
    /// );
    /// assert_eq!(format!("{x:?}"), "-0x1.2");
    /// ```
    #[inline]
    fn mul_add_mul_assign(&mut self, y: &Self, z: &Self, w: Self) {
        let x = core::mem::replace(self, Self::ZERO);
        *self = mul_add_mul_helper(
            Cow::Owned(x),
            Cow::Borrowed(y),
            Cow::Borrowed(z),
            Cow::Owned(w),
            false,
            None,
        )
        .0;
    }
}

impl MulAddMulAssign<&Self, &Self, &Self> for BigExponentFloat {
    /// Adds the product of two [`BigExponentFloat`]s to a [`BigExponentFloat`] multiplied by a
    /// [`BigExponentFloat`], in place. The [`BigExponentFloat`]s on the right-hand side are taken
    /// by reference. The result is exact.
    ///
    /// $$
    /// x \gets xy+zw.
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
    /// use malachite_base::num::arithmetic::traits::MulAddMulAssign;
    /// use malachite_float::BigExponentFloat;
    ///
    /// let mut x = BigExponentFloat::from(1.5);
    /// x.mul_add_mul_assign(
    ///     &BigExponentFloat::from(0.25),
    ///     &BigExponentFloat::from(3.0),
    ///     &BigExponentFloat::from(-0.5),
    /// );
    /// assert_eq!(format!("{x:?}"), "-0x1.2");
    /// ```
    #[inline]
    fn mul_add_mul_assign(&mut self, y: &Self, z: &Self, w: &Self) {
        let x = core::mem::replace(self, Self::ZERO);
        *self = mul_add_mul_helper(
            Cow::Owned(x),
            Cow::Borrowed(y),
            Cow::Borrowed(z),
            Cow::Borrowed(w),
            false,
            None,
        )
        .0;
    }
}
