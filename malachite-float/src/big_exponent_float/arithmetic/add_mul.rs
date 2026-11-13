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
use malachite_base::num::arithmetic::traits::{AddMul, AddMulAssign};
use malachite_base::num::basic::traits::{NaN, Zero};
use malachite_base::rounding_modes::RoundingMode::{self, *};

// x + y * z, or x - y * z if `negate` is true, rounded with `prec_rm` if it is `Some`, and exact
// otherwise. The product is formed exactly, so the only rounding is that of the sum; this is how
// FLINT's `arf_addmul` and `arf_submul` work.
pub(crate) fn add_mul_helper(
    x: Cow<'_, BigExponentFloat>,
    y: Cow<'_, BigExponentFloat>,
    z: Cow<'_, BigExponentFloat>,
    negate: bool,
    prec_rm: Option<(u64, RoundingMode)>,
) -> (BigExponentFloat, Ordering) {
    if let Some((prec, _)) = prec_rm {
        assert_ne!(prec, 0);
    }
    if x.is_nan() {
        return (BigExponentFloat::NAN, Equal);
    }
    let product = mul_helper(y, z, None).0;
    add_helper(x, Cow::Owned(product), negate, prec_rm)
}

crate_test_fn! {
    // x + y * z, or x - y * z if `negate` is true, rounded to `prec` bits with `rm`, without
    // `Float`'s kernels: the exact product is added to x by the generic implementation of addition.
    // Used to check the kernel path.
    add_mul_prec_round_generic(
        x: &BigExponentFloat,
        y: &BigExponentFloat,
        z: &BigExponentFloat,
        negate: bool,
        prec: u64,
        rm: RoundingMode,
    ) -> (BigExponentFloat, Ordering) {
        add_prec_round_generic(x, &(y * z), negate, prec, rm)
    }
}

impl BigExponentFloat {
    /// Adds a [`BigExponentFloat`] and the product of two other [`BigExponentFloat`]s, rounding the
    /// result to the specified precision and with the specified rounding mode. All three
    /// [`BigExponentFloat`]s are taken by value. An [`Ordering`] is also returned, indicating
    /// whether the rounded result is less than, equal to, or greater than the exact result.
    /// Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// The product is computed exactly, so only one rounding occurs.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,z,p,m) = x+yz+\varepsilon.
    /// $$
    /// - If $x+yz$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x+yz$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x+yz|\rfloor-p+1}$.
    /// - If $x+yz$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x+yz|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,y,z,p,m)=\text{NaN}$ if any of $x$, $y$, and $z$ is NaN, if one of $y$ and $z$ is
    ///   infinite and the other is zero, or if $x$ and $yz$ are infinities of opposite signs
    /// - $f(x,y,z,p,m)=\infty$ if $x$ or $yz$ is $\infty$ and neither is $-\infty$
    /// - $f(x,y,z,p,m)=-\infty$ if $x$ or $yz$ is $-\infty$ and neither is $\infty$
    /// - $f(x,y,z,p,m)=0$ if $x=-yz$ and both are finite
    ///
    /// If you know you'll be using `Nearest`, consider using [`BigExponentFloat::add_mul_prec`]
    /// instead. If you want the exact result, consider using [`AddMul::add_mul`] instead.
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
    /// This is FLINT's `arf_addmul`, with $x$ as the accumulator.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (result, o) = BigExponentFloat::from(PI).add_mul_prec_round(
    ///     BigExponentFloat::from(E),
    ///     BigExponentFloat::from(SQRT_2),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x6.c");
    /// assert_eq!(o, Less);
    ///
    /// let (result, o) = BigExponentFloat::from(PI).add_mul_prec_round(
    ///     BigExponentFloat::from(E),
    ///     BigExponentFloat::from(SQRT_2),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x7.0");
    /// assert_eq!(o, Greater);
    ///
    /// let (result, o) = BigExponentFloat::from(PI).add_mul_prec_round(
    ///     BigExponentFloat::from(E),
    ///     BigExponentFloat::from(SQRT_2),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x7.0");
    /// assert_eq!(o, Greater);
    ///
    /// let (result, o) = BigExponentFloat::from(PI).add_mul_prec_round(
    ///     BigExponentFloat::from(E),
    ///     BigExponentFloat::from(SQRT_2),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x6.fc5e8");
    /// assert_eq!(o, Less);
    ///
    /// let (result, o) = BigExponentFloat::from(PI).add_mul_prec_round(
    ///     BigExponentFloat::from(E),
    ///     BigExponentFloat::from(SQRT_2),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x6.fc5f");
    /// assert_eq!(o, Greater);
    ///
    /// let (result, o) = BigExponentFloat::from(PI).add_mul_prec_round(
    ///     BigExponentFloat::from(E),
    ///     BigExponentFloat::from(SQRT_2),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x6.fc5f");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn add_mul_prec_round(
        self,
        y: Self,
        z: Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        add_mul_helper(
            Cow::Owned(self),
            Cow::Owned(y),
            Cow::Owned(z),
            false,
            Some((prec, rm)),
        )
    }

    /// Adds a [`BigExponentFloat`] and the product of two other [`BigExponentFloat`]s, rounding the
    /// result to the specified precision and with the specified rounding mode. The first and second
    /// [`BigExponentFloat`]s are taken by value and the third by reference. An [`Ordering`] is also
    /// returned, indicating whether the rounded result is less than, equal to, or greater than the
    /// exact result. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// The product is computed exactly, so only one rounding occurs.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,z,p,m) = x+yz+\varepsilon.
    /// $$
    /// - If $x+yz$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x+yz$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x+yz|\rfloor-p+1}$.
    /// - If $x+yz$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x+yz|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,y,z,p,m)=\text{NaN}$ if any of $x$, $y$, and $z$ is NaN, if one of $y$ and $z$ is
    ///   infinite and the other is zero, or if $x$ and $yz$ are infinities of opposite signs
    /// - $f(x,y,z,p,m)=\infty$ if $x$ or $yz$ is $\infty$ and neither is $-\infty$
    /// - $f(x,y,z,p,m)=-\infty$ if $x$ or $yz$ is $-\infty$ and neither is $\infty$
    /// - $f(x,y,z,p,m)=0$ if $x=-yz$ and both are finite
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
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (result, o) = BigExponentFloat::from(PI).add_mul_prec_round_val_val_ref(
    ///     BigExponentFloat::from(E),
    ///     &BigExponentFloat::from(SQRT_2),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x6.c");
    /// assert_eq!(o, Less);
    ///
    /// let (result, o) = BigExponentFloat::from(PI).add_mul_prec_round_val_val_ref(
    ///     BigExponentFloat::from(E),
    ///     &BigExponentFloat::from(SQRT_2),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x7.0");
    /// assert_eq!(o, Greater);
    ///
    /// let (result, o) = BigExponentFloat::from(PI).add_mul_prec_round_val_val_ref(
    ///     BigExponentFloat::from(E),
    ///     &BigExponentFloat::from(SQRT_2),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x7.0");
    /// assert_eq!(o, Greater);
    ///
    /// let (result, o) = BigExponentFloat::from(PI).add_mul_prec_round_val_val_ref(
    ///     BigExponentFloat::from(E),
    ///     &BigExponentFloat::from(SQRT_2),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x6.fc5e8");
    /// assert_eq!(o, Less);
    ///
    /// let (result, o) = BigExponentFloat::from(PI).add_mul_prec_round_val_val_ref(
    ///     BigExponentFloat::from(E),
    ///     &BigExponentFloat::from(SQRT_2),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x6.fc5f");
    /// assert_eq!(o, Greater);
    ///
    /// let (result, o) = BigExponentFloat::from(PI).add_mul_prec_round_val_val_ref(
    ///     BigExponentFloat::from(E),
    ///     &BigExponentFloat::from(SQRT_2),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x6.fc5f");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn add_mul_prec_round_val_val_ref(
        self,
        y: Self,
        z: &Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        add_mul_helper(
            Cow::Owned(self),
            Cow::Owned(y),
            Cow::Borrowed(z),
            false,
            Some((prec, rm)),
        )
    }

    /// Adds a [`BigExponentFloat`] and the product of two other [`BigExponentFloat`]s, rounding the
    /// result to the specified precision and with the specified rounding mode. The first and third
    /// [`BigExponentFloat`]s are taken by value and the second by reference. An [`Ordering`] is
    /// also returned, indicating whether the rounded result is less than, equal to, or greater than
    /// the exact result. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever
    /// this function returns a `NaN` it also returns `Equal`.
    ///
    /// The product is computed exactly, so only one rounding occurs.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,z,p,m) = x+yz+\varepsilon.
    /// $$
    /// - If $x+yz$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x+yz$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x+yz|\rfloor-p+1}$.
    /// - If $x+yz$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x+yz|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,y,z,p,m)=\text{NaN}$ if any of $x$, $y$, and $z$ is NaN, if one of $y$ and $z$ is
    ///   infinite and the other is zero, or if $x$ and $yz$ are infinities of opposite signs
    /// - $f(x,y,z,p,m)=\infty$ if $x$ or $yz$ is $\infty$ and neither is $-\infty$
    /// - $f(x,y,z,p,m)=-\infty$ if $x$ or $yz$ is $-\infty$ and neither is $\infty$
    /// - $f(x,y,z,p,m)=0$ if $x=-yz$ and both are finite
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
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (result, o) = BigExponentFloat::from(PI).add_mul_prec_round_val_ref_val(
    ///     &BigExponentFloat::from(E),
    ///     BigExponentFloat::from(SQRT_2),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x6.c");
    /// assert_eq!(o, Less);
    ///
    /// let (result, o) = BigExponentFloat::from(PI).add_mul_prec_round_val_ref_val(
    ///     &BigExponentFloat::from(E),
    ///     BigExponentFloat::from(SQRT_2),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x7.0");
    /// assert_eq!(o, Greater);
    ///
    /// let (result, o) = BigExponentFloat::from(PI).add_mul_prec_round_val_ref_val(
    ///     &BigExponentFloat::from(E),
    ///     BigExponentFloat::from(SQRT_2),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x7.0");
    /// assert_eq!(o, Greater);
    ///
    /// let (result, o) = BigExponentFloat::from(PI).add_mul_prec_round_val_ref_val(
    ///     &BigExponentFloat::from(E),
    ///     BigExponentFloat::from(SQRT_2),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x6.fc5e8");
    /// assert_eq!(o, Less);
    ///
    /// let (result, o) = BigExponentFloat::from(PI).add_mul_prec_round_val_ref_val(
    ///     &BigExponentFloat::from(E),
    ///     BigExponentFloat::from(SQRT_2),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x6.fc5f");
    /// assert_eq!(o, Greater);
    ///
    /// let (result, o) = BigExponentFloat::from(PI).add_mul_prec_round_val_ref_val(
    ///     &BigExponentFloat::from(E),
    ///     BigExponentFloat::from(SQRT_2),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x6.fc5f");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn add_mul_prec_round_val_ref_val(
        self,
        y: &Self,
        z: Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        add_mul_helper(
            Cow::Owned(self),
            Cow::Borrowed(y),
            Cow::Owned(z),
            false,
            Some((prec, rm)),
        )
    }

    /// Adds a [`BigExponentFloat`] and the product of two other [`BigExponentFloat`]s, rounding the
    /// result to the specified precision and with the specified rounding mode. The first
    /// [`BigExponentFloat`] is taken by value and the second and third by reference. An
    /// [`Ordering`] is also returned, indicating whether the rounded result is less than, equal to,
    /// or greater than the exact result. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The product is computed exactly, so only one rounding occurs.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,z,p,m) = x+yz+\varepsilon.
    /// $$
    /// - If $x+yz$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x+yz$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x+yz|\rfloor-p+1}$.
    /// - If $x+yz$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x+yz|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,y,z,p,m)=\text{NaN}$ if any of $x$, $y$, and $z$ is NaN, if one of $y$ and $z$ is
    ///   infinite and the other is zero, or if $x$ and $yz$ are infinities of opposite signs
    /// - $f(x,y,z,p,m)=\infty$ if $x$ or $yz$ is $\infty$ and neither is $-\infty$
    /// - $f(x,y,z,p,m)=-\infty$ if $x$ or $yz$ is $-\infty$ and neither is $\infty$
    /// - $f(x,y,z,p,m)=0$ if $x=-yz$ and both are finite
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
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (result, o) = BigExponentFloat::from(PI).add_mul_prec_round_val_ref_ref(
    ///     &BigExponentFloat::from(E),
    ///     &BigExponentFloat::from(SQRT_2),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x6.c");
    /// assert_eq!(o, Less);
    ///
    /// let (result, o) = BigExponentFloat::from(PI).add_mul_prec_round_val_ref_ref(
    ///     &BigExponentFloat::from(E),
    ///     &BigExponentFloat::from(SQRT_2),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x7.0");
    /// assert_eq!(o, Greater);
    ///
    /// let (result, o) = BigExponentFloat::from(PI).add_mul_prec_round_val_ref_ref(
    ///     &BigExponentFloat::from(E),
    ///     &BigExponentFloat::from(SQRT_2),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x7.0");
    /// assert_eq!(o, Greater);
    ///
    /// let (result, o) = BigExponentFloat::from(PI).add_mul_prec_round_val_ref_ref(
    ///     &BigExponentFloat::from(E),
    ///     &BigExponentFloat::from(SQRT_2),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x6.fc5e8");
    /// assert_eq!(o, Less);
    ///
    /// let (result, o) = BigExponentFloat::from(PI).add_mul_prec_round_val_ref_ref(
    ///     &BigExponentFloat::from(E),
    ///     &BigExponentFloat::from(SQRT_2),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x6.fc5f");
    /// assert_eq!(o, Greater);
    ///
    /// let (result, o) = BigExponentFloat::from(PI).add_mul_prec_round_val_ref_ref(
    ///     &BigExponentFloat::from(E),
    ///     &BigExponentFloat::from(SQRT_2),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x6.fc5f");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn add_mul_prec_round_val_ref_ref(
        self,
        y: &Self,
        z: &Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        add_mul_helper(
            Cow::Owned(self),
            Cow::Borrowed(y),
            Cow::Borrowed(z),
            false,
            Some((prec, rm)),
        )
    }

    /// Adds a [`BigExponentFloat`] and the product of two other [`BigExponentFloat`]s, rounding the
    /// result to the specified precision and with the specified rounding mode. The second and third
    /// [`BigExponentFloat`]s are taken by value and the first by reference. An [`Ordering`] is also
    /// returned, indicating whether the rounded result is less than, equal to, or greater than the
    /// exact result. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// The product is computed exactly, so only one rounding occurs.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,z,p,m) = x+yz+\varepsilon.
    /// $$
    /// - If $x+yz$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x+yz$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x+yz|\rfloor-p+1}$.
    /// - If $x+yz$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x+yz|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,y,z,p,m)=\text{NaN}$ if any of $x$, $y$, and $z$ is NaN, if one of $y$ and $z$ is
    ///   infinite and the other is zero, or if $x$ and $yz$ are infinities of opposite signs
    /// - $f(x,y,z,p,m)=\infty$ if $x$ or $yz$ is $\infty$ and neither is $-\infty$
    /// - $f(x,y,z,p,m)=-\infty$ if $x$ or $yz$ is $-\infty$ and neither is $\infty$
    /// - $f(x,y,z,p,m)=0$ if $x=-yz$ and both are finite
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
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (result, o) = (&BigExponentFloat::from(PI)).add_mul_prec_round_ref_val_val(
    ///     BigExponentFloat::from(E),
    ///     BigExponentFloat::from(SQRT_2),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x6.c");
    /// assert_eq!(o, Less);
    ///
    /// let (result, o) = (&BigExponentFloat::from(PI)).add_mul_prec_round_ref_val_val(
    ///     BigExponentFloat::from(E),
    ///     BigExponentFloat::from(SQRT_2),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x7.0");
    /// assert_eq!(o, Greater);
    ///
    /// let (result, o) = (&BigExponentFloat::from(PI)).add_mul_prec_round_ref_val_val(
    ///     BigExponentFloat::from(E),
    ///     BigExponentFloat::from(SQRT_2),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x7.0");
    /// assert_eq!(o, Greater);
    ///
    /// let (result, o) = (&BigExponentFloat::from(PI)).add_mul_prec_round_ref_val_val(
    ///     BigExponentFloat::from(E),
    ///     BigExponentFloat::from(SQRT_2),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x6.fc5e8");
    /// assert_eq!(o, Less);
    ///
    /// let (result, o) = (&BigExponentFloat::from(PI)).add_mul_prec_round_ref_val_val(
    ///     BigExponentFloat::from(E),
    ///     BigExponentFloat::from(SQRT_2),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x6.fc5f");
    /// assert_eq!(o, Greater);
    ///
    /// let (result, o) = (&BigExponentFloat::from(PI)).add_mul_prec_round_ref_val_val(
    ///     BigExponentFloat::from(E),
    ///     BigExponentFloat::from(SQRT_2),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x6.fc5f");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn add_mul_prec_round_ref_val_val(
        &self,
        y: Self,
        z: Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        add_mul_helper(
            Cow::Borrowed(self),
            Cow::Owned(y),
            Cow::Owned(z),
            false,
            Some((prec, rm)),
        )
    }

    /// Adds a [`BigExponentFloat`] and the product of two other [`BigExponentFloat`]s, rounding the
    /// result to the specified precision and with the specified rounding mode. The second
    /// [`BigExponentFloat`] is taken by value and the first and third by reference. An [`Ordering`]
    /// is also returned, indicating whether the rounded result is less than, equal to, or greater
    /// than the exact result. Although `NaN`s are not comparable to any [`BigExponentFloat`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The product is computed exactly, so only one rounding occurs.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,z,p,m) = x+yz+\varepsilon.
    /// $$
    /// - If $x+yz$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x+yz$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x+yz|\rfloor-p+1}$.
    /// - If $x+yz$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x+yz|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,y,z,p,m)=\text{NaN}$ if any of $x$, $y$, and $z$ is NaN, if one of $y$ and $z$ is
    ///   infinite and the other is zero, or if $x$ and $yz$ are infinities of opposite signs
    /// - $f(x,y,z,p,m)=\infty$ if $x$ or $yz$ is $\infty$ and neither is $-\infty$
    /// - $f(x,y,z,p,m)=-\infty$ if $x$ or $yz$ is $-\infty$ and neither is $\infty$
    /// - $f(x,y,z,p,m)=0$ if $x=-yz$ and both are finite
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
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (result, o) = (&BigExponentFloat::from(PI)).add_mul_prec_round_ref_val_ref(
    ///     BigExponentFloat::from(E),
    ///     &BigExponentFloat::from(SQRT_2),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x6.c");
    /// assert_eq!(o, Less);
    ///
    /// let (result, o) = (&BigExponentFloat::from(PI)).add_mul_prec_round_ref_val_ref(
    ///     BigExponentFloat::from(E),
    ///     &BigExponentFloat::from(SQRT_2),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x7.0");
    /// assert_eq!(o, Greater);
    ///
    /// let (result, o) = (&BigExponentFloat::from(PI)).add_mul_prec_round_ref_val_ref(
    ///     BigExponentFloat::from(E),
    ///     &BigExponentFloat::from(SQRT_2),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x7.0");
    /// assert_eq!(o, Greater);
    ///
    /// let (result, o) = (&BigExponentFloat::from(PI)).add_mul_prec_round_ref_val_ref(
    ///     BigExponentFloat::from(E),
    ///     &BigExponentFloat::from(SQRT_2),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x6.fc5e8");
    /// assert_eq!(o, Less);
    ///
    /// let (result, o) = (&BigExponentFloat::from(PI)).add_mul_prec_round_ref_val_ref(
    ///     BigExponentFloat::from(E),
    ///     &BigExponentFloat::from(SQRT_2),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x6.fc5f");
    /// assert_eq!(o, Greater);
    ///
    /// let (result, o) = (&BigExponentFloat::from(PI)).add_mul_prec_round_ref_val_ref(
    ///     BigExponentFloat::from(E),
    ///     &BigExponentFloat::from(SQRT_2),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x6.fc5f");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn add_mul_prec_round_ref_val_ref(
        &self,
        y: Self,
        z: &Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        add_mul_helper(
            Cow::Borrowed(self),
            Cow::Owned(y),
            Cow::Borrowed(z),
            false,
            Some((prec, rm)),
        )
    }

    /// Adds a [`BigExponentFloat`] and the product of two other [`BigExponentFloat`]s, rounding the
    /// result to the specified precision and with the specified rounding mode. The third
    /// [`BigExponentFloat`] is taken by value and the first and second by reference. An
    /// [`Ordering`] is also returned, indicating whether the rounded result is less than, equal to,
    /// or greater than the exact result. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The product is computed exactly, so only one rounding occurs.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,z,p,m) = x+yz+\varepsilon.
    /// $$
    /// - If $x+yz$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x+yz$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x+yz|\rfloor-p+1}$.
    /// - If $x+yz$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x+yz|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,y,z,p,m)=\text{NaN}$ if any of $x$, $y$, and $z$ is NaN, if one of $y$ and $z$ is
    ///   infinite and the other is zero, or if $x$ and $yz$ are infinities of opposite signs
    /// - $f(x,y,z,p,m)=\infty$ if $x$ or $yz$ is $\infty$ and neither is $-\infty$
    /// - $f(x,y,z,p,m)=-\infty$ if $x$ or $yz$ is $-\infty$ and neither is $\infty$
    /// - $f(x,y,z,p,m)=0$ if $x=-yz$ and both are finite
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
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (result, o) = (&BigExponentFloat::from(PI)).add_mul_prec_round_ref_ref_val(
    ///     &BigExponentFloat::from(E),
    ///     BigExponentFloat::from(SQRT_2),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x6.c");
    /// assert_eq!(o, Less);
    ///
    /// let (result, o) = (&BigExponentFloat::from(PI)).add_mul_prec_round_ref_ref_val(
    ///     &BigExponentFloat::from(E),
    ///     BigExponentFloat::from(SQRT_2),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x7.0");
    /// assert_eq!(o, Greater);
    ///
    /// let (result, o) = (&BigExponentFloat::from(PI)).add_mul_prec_round_ref_ref_val(
    ///     &BigExponentFloat::from(E),
    ///     BigExponentFloat::from(SQRT_2),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x7.0");
    /// assert_eq!(o, Greater);
    ///
    /// let (result, o) = (&BigExponentFloat::from(PI)).add_mul_prec_round_ref_ref_val(
    ///     &BigExponentFloat::from(E),
    ///     BigExponentFloat::from(SQRT_2),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x6.fc5e8");
    /// assert_eq!(o, Less);
    ///
    /// let (result, o) = (&BigExponentFloat::from(PI)).add_mul_prec_round_ref_ref_val(
    ///     &BigExponentFloat::from(E),
    ///     BigExponentFloat::from(SQRT_2),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x6.fc5f");
    /// assert_eq!(o, Greater);
    ///
    /// let (result, o) = (&BigExponentFloat::from(PI)).add_mul_prec_round_ref_ref_val(
    ///     &BigExponentFloat::from(E),
    ///     BigExponentFloat::from(SQRT_2),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x6.fc5f");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn add_mul_prec_round_ref_ref_val(
        &self,
        y: &Self,
        z: Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        add_mul_helper(
            Cow::Borrowed(self),
            Cow::Borrowed(y),
            Cow::Owned(z),
            false,
            Some((prec, rm)),
        )
    }

    /// Adds a [`BigExponentFloat`] and the product of two other [`BigExponentFloat`]s, rounding the
    /// result to the specified precision and with the specified rounding mode. All three
    /// [`BigExponentFloat`]s are taken by reference. An [`Ordering`] is also returned, indicating
    /// whether the rounded result is less than, equal to, or greater than the exact result.
    /// Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// The product is computed exactly, so only one rounding occurs.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,z,p,m) = x+yz+\varepsilon.
    /// $$
    /// - If $x+yz$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x+yz$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x+yz|\rfloor-p+1}$.
    /// - If $x+yz$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x+yz|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,y,z,p,m)=\text{NaN}$ if any of $x$, $y$, and $z$ is NaN, if one of $y$ and $z$ is
    ///   infinite and the other is zero, or if $x$ and $yz$ are infinities of opposite signs
    /// - $f(x,y,z,p,m)=\infty$ if $x$ or $yz$ is $\infty$ and neither is $-\infty$
    /// - $f(x,y,z,p,m)=-\infty$ if $x$ or $yz$ is $-\infty$ and neither is $\infty$
    /// - $f(x,y,z,p,m)=0$ if $x=-yz$ and both are finite
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
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (result, o) = (&BigExponentFloat::from(PI)).add_mul_prec_round_ref_ref_ref(
    ///     &BigExponentFloat::from(E),
    ///     &BigExponentFloat::from(SQRT_2),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x6.c");
    /// assert_eq!(o, Less);
    ///
    /// let (result, o) = (&BigExponentFloat::from(PI)).add_mul_prec_round_ref_ref_ref(
    ///     &BigExponentFloat::from(E),
    ///     &BigExponentFloat::from(SQRT_2),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x7.0");
    /// assert_eq!(o, Greater);
    ///
    /// let (result, o) = (&BigExponentFloat::from(PI)).add_mul_prec_round_ref_ref_ref(
    ///     &BigExponentFloat::from(E),
    ///     &BigExponentFloat::from(SQRT_2),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x7.0");
    /// assert_eq!(o, Greater);
    ///
    /// let (result, o) = (&BigExponentFloat::from(PI)).add_mul_prec_round_ref_ref_ref(
    ///     &BigExponentFloat::from(E),
    ///     &BigExponentFloat::from(SQRT_2),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x6.fc5e8");
    /// assert_eq!(o, Less);
    ///
    /// let (result, o) = (&BigExponentFloat::from(PI)).add_mul_prec_round_ref_ref_ref(
    ///     &BigExponentFloat::from(E),
    ///     &BigExponentFloat::from(SQRT_2),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x6.fc5f");
    /// assert_eq!(o, Greater);
    ///
    /// let (result, o) = (&BigExponentFloat::from(PI)).add_mul_prec_round_ref_ref_ref(
    ///     &BigExponentFloat::from(E),
    ///     &BigExponentFloat::from(SQRT_2),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x6.fc5f");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn add_mul_prec_round_ref_ref_ref(
        &self,
        y: &Self,
        z: &Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        add_mul_helper(
            Cow::Borrowed(self),
            Cow::Borrowed(y),
            Cow::Borrowed(z),
            false,
            Some((prec, rm)),
        )
    }

    /// Adds the product of two [`BigExponentFloat`]s to a [`BigExponentFloat`] in place, rounding
    /// the result to the specified precision and with the specified rounding mode. The
    /// [`BigExponentFloat`]s on the right-hand side are taken by value. An [`Ordering`] is
    /// returned, indicating whether the rounded result is less than, equal to, or greater than the
    /// exact result. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this
    /// function sets the [`BigExponentFloat`] to `NaN` it also returns `Equal`.
    ///
    /// The product is computed exactly, so only one rounding occurs.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets x+yz+\varepsilon.
    /// $$
    /// - If $x+yz$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x+yz$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x+yz|\rfloor-p+1}$.
    /// - If $x+yz$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x+yz|\rfloor-p}$.
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
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.add_mul_prec_round_assign(
    ///         BigExponentFloat::from(E),
    ///         BigExponentFloat::from(SQRT_2),
    ///         5,
    ///         Floor
    ///     ),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x6.c");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.add_mul_prec_round_assign(
    ///         BigExponentFloat::from(E),
    ///         BigExponentFloat::from(SQRT_2),
    ///         5,
    ///         Ceiling
    ///     ),
    ///     Greater
    /// );
    /// assert_eq!(format!("{x:?}"), "0x7.0");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.add_mul_prec_round_assign(
    ///         BigExponentFloat::from(E),
    ///         BigExponentFloat::from(SQRT_2),
    ///         5,
    ///         Nearest
    ///     ),
    ///     Greater
    /// );
    /// assert_eq!(format!("{x:?}"), "0x7.0");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.add_mul_prec_round_assign(
    ///         BigExponentFloat::from(E),
    ///         BigExponentFloat::from(SQRT_2),
    ///         20,
    ///         Floor
    ///     ),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x6.fc5e8");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.add_mul_prec_round_assign(
    ///         BigExponentFloat::from(E),
    ///         BigExponentFloat::from(SQRT_2),
    ///         20,
    ///         Ceiling
    ///     ),
    ///     Greater
    /// );
    /// assert_eq!(format!("{x:?}"), "0x6.fc5f");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.add_mul_prec_round_assign(
    ///         BigExponentFloat::from(E),
    ///         BigExponentFloat::from(SQRT_2),
    ///         20,
    ///         Nearest
    ///     ),
    ///     Greater
    /// );
    /// assert_eq!(format!("{x:?}"), "0x6.fc5f");
    /// ```
    #[inline]
    pub fn add_mul_prec_round_assign(
        &mut self,
        y: Self,
        z: Self,
        prec: u64,
        rm: RoundingMode,
    ) -> Ordering {
        let x = core::mem::replace(self, Self::ZERO);
        let (result, o) = add_mul_helper(
            Cow::Owned(x),
            Cow::Owned(y),
            Cow::Owned(z),
            false,
            Some((prec, rm)),
        );
        *self = result;
        o
    }

    /// Adds the product of two [`BigExponentFloat`]s to a [`BigExponentFloat`] in place, rounding
    /// the result to the specified precision and with the specified rounding mode. On the
    /// right-hand side, the first [`BigExponentFloat`] is taken by value and the second by
    /// reference. An [`Ordering`] is returned, indicating whether the rounded result is less than,
    /// equal to, or greater than the exact result. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function sets the [`BigExponentFloat`] to `NaN` it also
    /// returns `Equal`.
    ///
    /// The product is computed exactly, so only one rounding occurs.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets x+yz+\varepsilon.
    /// $$
    /// - If $x+yz$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x+yz$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x+yz|\rfloor-p+1}$.
    /// - If $x+yz$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x+yz|\rfloor-p}$.
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
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.add_mul_prec_round_assign_val_ref(
    ///         BigExponentFloat::from(E),
    ///         &BigExponentFloat::from(SQRT_2),
    ///         5,
    ///         Floor
    ///     ),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x6.c");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.add_mul_prec_round_assign_val_ref(
    ///         BigExponentFloat::from(E),
    ///         &BigExponentFloat::from(SQRT_2),
    ///         5,
    ///         Ceiling
    ///     ),
    ///     Greater
    /// );
    /// assert_eq!(format!("{x:?}"), "0x7.0");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.add_mul_prec_round_assign_val_ref(
    ///         BigExponentFloat::from(E),
    ///         &BigExponentFloat::from(SQRT_2),
    ///         5,
    ///         Nearest
    ///     ),
    ///     Greater
    /// );
    /// assert_eq!(format!("{x:?}"), "0x7.0");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.add_mul_prec_round_assign_val_ref(
    ///         BigExponentFloat::from(E),
    ///         &BigExponentFloat::from(SQRT_2),
    ///         20,
    ///         Floor
    ///     ),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x6.fc5e8");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.add_mul_prec_round_assign_val_ref(
    ///         BigExponentFloat::from(E),
    ///         &BigExponentFloat::from(SQRT_2),
    ///         20,
    ///         Ceiling
    ///     ),
    ///     Greater
    /// );
    /// assert_eq!(format!("{x:?}"), "0x6.fc5f");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.add_mul_prec_round_assign_val_ref(
    ///         BigExponentFloat::from(E),
    ///         &BigExponentFloat::from(SQRT_2),
    ///         20,
    ///         Nearest
    ///     ),
    ///     Greater
    /// );
    /// assert_eq!(format!("{x:?}"), "0x6.fc5f");
    /// ```
    #[inline]
    pub fn add_mul_prec_round_assign_val_ref(
        &mut self,
        y: Self,
        z: &Self,
        prec: u64,
        rm: RoundingMode,
    ) -> Ordering {
        let x = core::mem::replace(self, Self::ZERO);
        let (result, o) = add_mul_helper(
            Cow::Owned(x),
            Cow::Owned(y),
            Cow::Borrowed(z),
            false,
            Some((prec, rm)),
        );
        *self = result;
        o
    }

    /// Adds the product of two [`BigExponentFloat`]s to a [`BigExponentFloat`] in place, rounding
    /// the result to the specified precision and with the specified rounding mode. On the
    /// right-hand side, the second [`BigExponentFloat`] is taken by value and the first by
    /// reference. An [`Ordering`] is returned, indicating whether the rounded result is less than,
    /// equal to, or greater than the exact result. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function sets the [`BigExponentFloat`] to `NaN` it also
    /// returns `Equal`.
    ///
    /// The product is computed exactly, so only one rounding occurs.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets x+yz+\varepsilon.
    /// $$
    /// - If $x+yz$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x+yz$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x+yz|\rfloor-p+1}$.
    /// - If $x+yz$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x+yz|\rfloor-p}$.
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
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.add_mul_prec_round_assign_ref_val(
    ///         &BigExponentFloat::from(E),
    ///         BigExponentFloat::from(SQRT_2),
    ///         5,
    ///         Floor
    ///     ),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x6.c");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.add_mul_prec_round_assign_ref_val(
    ///         &BigExponentFloat::from(E),
    ///         BigExponentFloat::from(SQRT_2),
    ///         5,
    ///         Ceiling
    ///     ),
    ///     Greater
    /// );
    /// assert_eq!(format!("{x:?}"), "0x7.0");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.add_mul_prec_round_assign_ref_val(
    ///         &BigExponentFloat::from(E),
    ///         BigExponentFloat::from(SQRT_2),
    ///         5,
    ///         Nearest
    ///     ),
    ///     Greater
    /// );
    /// assert_eq!(format!("{x:?}"), "0x7.0");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.add_mul_prec_round_assign_ref_val(
    ///         &BigExponentFloat::from(E),
    ///         BigExponentFloat::from(SQRT_2),
    ///         20,
    ///         Floor
    ///     ),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x6.fc5e8");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.add_mul_prec_round_assign_ref_val(
    ///         &BigExponentFloat::from(E),
    ///         BigExponentFloat::from(SQRT_2),
    ///         20,
    ///         Ceiling
    ///     ),
    ///     Greater
    /// );
    /// assert_eq!(format!("{x:?}"), "0x6.fc5f");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.add_mul_prec_round_assign_ref_val(
    ///         &BigExponentFloat::from(E),
    ///         BigExponentFloat::from(SQRT_2),
    ///         20,
    ///         Nearest
    ///     ),
    ///     Greater
    /// );
    /// assert_eq!(format!("{x:?}"), "0x6.fc5f");
    /// ```
    #[inline]
    pub fn add_mul_prec_round_assign_ref_val(
        &mut self,
        y: &Self,
        z: Self,
        prec: u64,
        rm: RoundingMode,
    ) -> Ordering {
        let x = core::mem::replace(self, Self::ZERO);
        let (result, o) = add_mul_helper(
            Cow::Owned(x),
            Cow::Borrowed(y),
            Cow::Owned(z),
            false,
            Some((prec, rm)),
        );
        *self = result;
        o
    }

    /// Adds the product of two [`BigExponentFloat`]s to a [`BigExponentFloat`] in place, rounding
    /// the result to the specified precision and with the specified rounding mode. The
    /// [`BigExponentFloat`]s on the right-hand side are taken by reference. An [`Ordering`] is
    /// returned, indicating whether the rounded result is less than, equal to, or greater than the
    /// exact result. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this
    /// function sets the [`BigExponentFloat`] to `NaN` it also returns `Equal`.
    ///
    /// The product is computed exactly, so only one rounding occurs.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets x+yz+\varepsilon.
    /// $$
    /// - If $x+yz$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x+yz$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x+yz|\rfloor-p+1}$.
    /// - If $x+yz$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x+yz|\rfloor-p}$.
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
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.add_mul_prec_round_assign_ref_ref(
    ///         &BigExponentFloat::from(E),
    ///         &BigExponentFloat::from(SQRT_2),
    ///         5,
    ///         Floor
    ///     ),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x6.c");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.add_mul_prec_round_assign_ref_ref(
    ///         &BigExponentFloat::from(E),
    ///         &BigExponentFloat::from(SQRT_2),
    ///         5,
    ///         Ceiling
    ///     ),
    ///     Greater
    /// );
    /// assert_eq!(format!("{x:?}"), "0x7.0");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.add_mul_prec_round_assign_ref_ref(
    ///         &BigExponentFloat::from(E),
    ///         &BigExponentFloat::from(SQRT_2),
    ///         5,
    ///         Nearest
    ///     ),
    ///     Greater
    /// );
    /// assert_eq!(format!("{x:?}"), "0x7.0");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.add_mul_prec_round_assign_ref_ref(
    ///         &BigExponentFloat::from(E),
    ///         &BigExponentFloat::from(SQRT_2),
    ///         20,
    ///         Floor
    ///     ),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x6.fc5e8");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.add_mul_prec_round_assign_ref_ref(
    ///         &BigExponentFloat::from(E),
    ///         &BigExponentFloat::from(SQRT_2),
    ///         20,
    ///         Ceiling
    ///     ),
    ///     Greater
    /// );
    /// assert_eq!(format!("{x:?}"), "0x6.fc5f");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.add_mul_prec_round_assign_ref_ref(
    ///         &BigExponentFloat::from(E),
    ///         &BigExponentFloat::from(SQRT_2),
    ///         20,
    ///         Nearest
    ///     ),
    ///     Greater
    /// );
    /// assert_eq!(format!("{x:?}"), "0x6.fc5f");
    /// ```
    #[inline]
    pub fn add_mul_prec_round_assign_ref_ref(
        &mut self,
        y: &Self,
        z: &Self,
        prec: u64,
        rm: RoundingMode,
    ) -> Ordering {
        let x = core::mem::replace(self, Self::ZERO);
        let (result, o) = add_mul_helper(
            Cow::Owned(x),
            Cow::Borrowed(y),
            Cow::Borrowed(z),
            false,
            Some((prec, rm)),
        );
        *self = result;
        o
    }

    /// Adds a [`BigExponentFloat`] and the product of two other [`BigExponentFloat`]s, rounding the
    /// result to the nearest value of the specified precision. All three [`BigExponentFloat`]s are
    /// taken by value. An [`Ordering`] is also returned, indicating whether the rounded result is
    /// less than, equal to, or greater than the exact result. Although `NaN`s are not comparable to
    /// any [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The product is computed exactly, so only one rounding occurs.
    ///
    /// $$
    /// f(x,y,z,p) = x+yz+\varepsilon.
    /// $$
    /// - If $x+yz$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x+yz$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x+yz|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,y,z,p)=\text{NaN}$ if any of $x$, $y$, and $z$ is NaN, if one of $y$ and $z$ is
    ///   infinite and the other is zero, or if $x$ and $yz$ are infinities of opposite signs
    /// - $f(x,y,z,p)=\infty$ if $x$ or $yz$ is $\infty$ and neither is $-\infty$
    /// - $f(x,y,z,p)=-\infty$ if $x$ or $yz$ is $-\infty$ and neither is $\infty$
    /// - $f(x,y,z,p)=0$ if $x=-yz$ and both are finite
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`BigExponentFloat::add_mul_prec_round`] instead. If you want the exact result, consider
    /// using [`AddMul::add_mul`] instead.
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
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (result, o) = BigExponentFloat::from(PI).add_mul_prec(
    ///     BigExponentFloat::from(E),
    ///     BigExponentFloat::from(SQRT_2),
    ///     5,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x7.0");
    /// assert_eq!(o, Greater);
    ///
    /// let (result, o) = BigExponentFloat::from(PI).add_mul_prec(
    ///     BigExponentFloat::from(E),
    ///     BigExponentFloat::from(SQRT_2),
    ///     20,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x6.fc5f");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn add_mul_prec(self, y: Self, z: Self, prec: u64) -> (Self, Ordering) {
        add_mul_helper(
            Cow::Owned(self),
            Cow::Owned(y),
            Cow::Owned(z),
            false,
            Some((prec, Nearest)),
        )
    }

    /// Adds a [`BigExponentFloat`] and the product of two other [`BigExponentFloat`]s, rounding the
    /// result to the nearest value of the specified precision. The first and second
    /// [`BigExponentFloat`]s are taken by value and the third by reference. An [`Ordering`] is also
    /// returned, indicating whether the rounded result is less than, equal to, or greater than the
    /// exact result. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// The product is computed exactly, so only one rounding occurs.
    ///
    /// $$
    /// f(x,y,z,p) = x+yz+\varepsilon.
    /// $$
    /// - If $x+yz$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x+yz$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x+yz|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,y,z,p)=\text{NaN}$ if any of $x$, $y$, and $z$ is NaN, if one of $y$ and $z$ is
    ///   infinite and the other is zero, or if $x$ and $yz$ are infinities of opposite signs
    /// - $f(x,y,z,p)=\infty$ if $x$ or $yz$ is $\infty$ and neither is $-\infty$
    /// - $f(x,y,z,p)=-\infty$ if $x$ or $yz$ is $-\infty$ and neither is $\infty$
    /// - $f(x,y,z,p)=0$ if $x=-yz$ and both are finite
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
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (result, o) = BigExponentFloat::from(PI).add_mul_prec_val_val_ref(
    ///     BigExponentFloat::from(E),
    ///     &BigExponentFloat::from(SQRT_2),
    ///     5,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x7.0");
    /// assert_eq!(o, Greater);
    ///
    /// let (result, o) = BigExponentFloat::from(PI).add_mul_prec_val_val_ref(
    ///     BigExponentFloat::from(E),
    ///     &BigExponentFloat::from(SQRT_2),
    ///     20,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x6.fc5f");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn add_mul_prec_val_val_ref(self, y: Self, z: &Self, prec: u64) -> (Self, Ordering) {
        add_mul_helper(
            Cow::Owned(self),
            Cow::Owned(y),
            Cow::Borrowed(z),
            false,
            Some((prec, Nearest)),
        )
    }

    /// Adds a [`BigExponentFloat`] and the product of two other [`BigExponentFloat`]s, rounding the
    /// result to the nearest value of the specified precision. The first and third
    /// [`BigExponentFloat`]s are taken by value and the second by reference. An [`Ordering`] is
    /// also returned, indicating whether the rounded result is less than, equal to, or greater than
    /// the exact result. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever
    /// this function returns a `NaN` it also returns `Equal`.
    ///
    /// The product is computed exactly, so only one rounding occurs.
    ///
    /// $$
    /// f(x,y,z,p) = x+yz+\varepsilon.
    /// $$
    /// - If $x+yz$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x+yz$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x+yz|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,y,z,p)=\text{NaN}$ if any of $x$, $y$, and $z$ is NaN, if one of $y$ and $z$ is
    ///   infinite and the other is zero, or if $x$ and $yz$ are infinities of opposite signs
    /// - $f(x,y,z,p)=\infty$ if $x$ or $yz$ is $\infty$ and neither is $-\infty$
    /// - $f(x,y,z,p)=-\infty$ if $x$ or $yz$ is $-\infty$ and neither is $\infty$
    /// - $f(x,y,z,p)=0$ if $x=-yz$ and both are finite
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
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (result, o) = BigExponentFloat::from(PI).add_mul_prec_val_ref_val(
    ///     &BigExponentFloat::from(E),
    ///     BigExponentFloat::from(SQRT_2),
    ///     5,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x7.0");
    /// assert_eq!(o, Greater);
    ///
    /// let (result, o) = BigExponentFloat::from(PI).add_mul_prec_val_ref_val(
    ///     &BigExponentFloat::from(E),
    ///     BigExponentFloat::from(SQRT_2),
    ///     20,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x6.fc5f");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn add_mul_prec_val_ref_val(self, y: &Self, z: Self, prec: u64) -> (Self, Ordering) {
        add_mul_helper(
            Cow::Owned(self),
            Cow::Borrowed(y),
            Cow::Owned(z),
            false,
            Some((prec, Nearest)),
        )
    }

    /// Adds a [`BigExponentFloat`] and the product of two other [`BigExponentFloat`]s, rounding the
    /// result to the nearest value of the specified precision. The first [`BigExponentFloat`] is
    /// taken by value and the second and third by reference. An [`Ordering`] is also returned,
    /// indicating whether the rounded result is less than, equal to, or greater than the exact
    /// result. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// The product is computed exactly, so only one rounding occurs.
    ///
    /// $$
    /// f(x,y,z,p) = x+yz+\varepsilon.
    /// $$
    /// - If $x+yz$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x+yz$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x+yz|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,y,z,p)=\text{NaN}$ if any of $x$, $y$, and $z$ is NaN, if one of $y$ and $z$ is
    ///   infinite and the other is zero, or if $x$ and $yz$ are infinities of opposite signs
    /// - $f(x,y,z,p)=\infty$ if $x$ or $yz$ is $\infty$ and neither is $-\infty$
    /// - $f(x,y,z,p)=-\infty$ if $x$ or $yz$ is $-\infty$ and neither is $\infty$
    /// - $f(x,y,z,p)=0$ if $x=-yz$ and both are finite
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
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (result, o) = BigExponentFloat::from(PI).add_mul_prec_val_ref_ref(
    ///     &BigExponentFloat::from(E),
    ///     &BigExponentFloat::from(SQRT_2),
    ///     5,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x7.0");
    /// assert_eq!(o, Greater);
    ///
    /// let (result, o) = BigExponentFloat::from(PI).add_mul_prec_val_ref_ref(
    ///     &BigExponentFloat::from(E),
    ///     &BigExponentFloat::from(SQRT_2),
    ///     20,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x6.fc5f");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn add_mul_prec_val_ref_ref(self, y: &Self, z: &Self, prec: u64) -> (Self, Ordering) {
        add_mul_helper(
            Cow::Owned(self),
            Cow::Borrowed(y),
            Cow::Borrowed(z),
            false,
            Some((prec, Nearest)),
        )
    }

    /// Adds a [`BigExponentFloat`] and the product of two other [`BigExponentFloat`]s, rounding the
    /// result to the nearest value of the specified precision. The second and third
    /// [`BigExponentFloat`]s are taken by value and the first by reference. An [`Ordering`] is also
    /// returned, indicating whether the rounded result is less than, equal to, or greater than the
    /// exact result. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// The product is computed exactly, so only one rounding occurs.
    ///
    /// $$
    /// f(x,y,z,p) = x+yz+\varepsilon.
    /// $$
    /// - If $x+yz$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x+yz$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x+yz|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,y,z,p)=\text{NaN}$ if any of $x$, $y$, and $z$ is NaN, if one of $y$ and $z$ is
    ///   infinite and the other is zero, or if $x$ and $yz$ are infinities of opposite signs
    /// - $f(x,y,z,p)=\infty$ if $x$ or $yz$ is $\infty$ and neither is $-\infty$
    /// - $f(x,y,z,p)=-\infty$ if $x$ or $yz$ is $-\infty$ and neither is $\infty$
    /// - $f(x,y,z,p)=0$ if $x=-yz$ and both are finite
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
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (result, o) = (&BigExponentFloat::from(PI)).add_mul_prec_ref_val_val(
    ///     BigExponentFloat::from(E),
    ///     BigExponentFloat::from(SQRT_2),
    ///     5,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x7.0");
    /// assert_eq!(o, Greater);
    ///
    /// let (result, o) = (&BigExponentFloat::from(PI)).add_mul_prec_ref_val_val(
    ///     BigExponentFloat::from(E),
    ///     BigExponentFloat::from(SQRT_2),
    ///     20,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x6.fc5f");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn add_mul_prec_ref_val_val(&self, y: Self, z: Self, prec: u64) -> (Self, Ordering) {
        add_mul_helper(
            Cow::Borrowed(self),
            Cow::Owned(y),
            Cow::Owned(z),
            false,
            Some((prec, Nearest)),
        )
    }

    /// Adds a [`BigExponentFloat`] and the product of two other [`BigExponentFloat`]s, rounding the
    /// result to the nearest value of the specified precision. The second [`BigExponentFloat`] is
    /// taken by value and the first and third by reference. An [`Ordering`] is also returned,
    /// indicating whether the rounded result is less than, equal to, or greater than the exact
    /// result. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// The product is computed exactly, so only one rounding occurs.
    ///
    /// $$
    /// f(x,y,z,p) = x+yz+\varepsilon.
    /// $$
    /// - If $x+yz$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x+yz$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x+yz|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,y,z,p)=\text{NaN}$ if any of $x$, $y$, and $z$ is NaN, if one of $y$ and $z$ is
    ///   infinite and the other is zero, or if $x$ and $yz$ are infinities of opposite signs
    /// - $f(x,y,z,p)=\infty$ if $x$ or $yz$ is $\infty$ and neither is $-\infty$
    /// - $f(x,y,z,p)=-\infty$ if $x$ or $yz$ is $-\infty$ and neither is $\infty$
    /// - $f(x,y,z,p)=0$ if $x=-yz$ and both are finite
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
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (result, o) = (&BigExponentFloat::from(PI)).add_mul_prec_ref_val_ref(
    ///     BigExponentFloat::from(E),
    ///     &BigExponentFloat::from(SQRT_2),
    ///     5,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x7.0");
    /// assert_eq!(o, Greater);
    ///
    /// let (result, o) = (&BigExponentFloat::from(PI)).add_mul_prec_ref_val_ref(
    ///     BigExponentFloat::from(E),
    ///     &BigExponentFloat::from(SQRT_2),
    ///     20,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x6.fc5f");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn add_mul_prec_ref_val_ref(&self, y: Self, z: &Self, prec: u64) -> (Self, Ordering) {
        add_mul_helper(
            Cow::Borrowed(self),
            Cow::Owned(y),
            Cow::Borrowed(z),
            false,
            Some((prec, Nearest)),
        )
    }

    /// Adds a [`BigExponentFloat`] and the product of two other [`BigExponentFloat`]s, rounding the
    /// result to the nearest value of the specified precision. The third [`BigExponentFloat`] is
    /// taken by value and the first and second by reference. An [`Ordering`] is also returned,
    /// indicating whether the rounded result is less than, equal to, or greater than the exact
    /// result. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// The product is computed exactly, so only one rounding occurs.
    ///
    /// $$
    /// f(x,y,z,p) = x+yz+\varepsilon.
    /// $$
    /// - If $x+yz$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x+yz$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x+yz|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,y,z,p)=\text{NaN}$ if any of $x$, $y$, and $z$ is NaN, if one of $y$ and $z$ is
    ///   infinite and the other is zero, or if $x$ and $yz$ are infinities of opposite signs
    /// - $f(x,y,z,p)=\infty$ if $x$ or $yz$ is $\infty$ and neither is $-\infty$
    /// - $f(x,y,z,p)=-\infty$ if $x$ or $yz$ is $-\infty$ and neither is $\infty$
    /// - $f(x,y,z,p)=0$ if $x=-yz$ and both are finite
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
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (result, o) = (&BigExponentFloat::from(PI)).add_mul_prec_ref_ref_val(
    ///     &BigExponentFloat::from(E),
    ///     BigExponentFloat::from(SQRT_2),
    ///     5,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x7.0");
    /// assert_eq!(o, Greater);
    ///
    /// let (result, o) = (&BigExponentFloat::from(PI)).add_mul_prec_ref_ref_val(
    ///     &BigExponentFloat::from(E),
    ///     BigExponentFloat::from(SQRT_2),
    ///     20,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x6.fc5f");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn add_mul_prec_ref_ref_val(&self, y: &Self, z: Self, prec: u64) -> (Self, Ordering) {
        add_mul_helper(
            Cow::Borrowed(self),
            Cow::Borrowed(y),
            Cow::Owned(z),
            false,
            Some((prec, Nearest)),
        )
    }

    /// Adds a [`BigExponentFloat`] and the product of two other [`BigExponentFloat`]s, rounding the
    /// result to the nearest value of the specified precision. All three [`BigExponentFloat`]s are
    /// taken by reference. An [`Ordering`] is also returned, indicating whether the rounded result
    /// is less than, equal to, or greater than the exact result. Although `NaN`s are not comparable
    /// to any [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The product is computed exactly, so only one rounding occurs.
    ///
    /// $$
    /// f(x,y,z,p) = x+yz+\varepsilon.
    /// $$
    /// - If $x+yz$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x+yz$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x+yz|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,y,z,p)=\text{NaN}$ if any of $x$, $y$, and $z$ is NaN, if one of $y$ and $z$ is
    ///   infinite and the other is zero, or if $x$ and $yz$ are infinities of opposite signs
    /// - $f(x,y,z,p)=\infty$ if $x$ or $yz$ is $\infty$ and neither is $-\infty$
    /// - $f(x,y,z,p)=-\infty$ if $x$ or $yz$ is $-\infty$ and neither is $\infty$
    /// - $f(x,y,z,p)=0$ if $x=-yz$ and both are finite
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
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (result, o) = (&BigExponentFloat::from(PI)).add_mul_prec_ref_ref_ref(
    ///     &BigExponentFloat::from(E),
    ///     &BigExponentFloat::from(SQRT_2),
    ///     5,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x7.0");
    /// assert_eq!(o, Greater);
    ///
    /// let (result, o) = (&BigExponentFloat::from(PI)).add_mul_prec_ref_ref_ref(
    ///     &BigExponentFloat::from(E),
    ///     &BigExponentFloat::from(SQRT_2),
    ///     20,
    /// );
    /// assert_eq!(format!("{result:?}"), "0x6.fc5f");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn add_mul_prec_ref_ref_ref(&self, y: &Self, z: &Self, prec: u64) -> (Self, Ordering) {
        add_mul_helper(
            Cow::Borrowed(self),
            Cow::Borrowed(y),
            Cow::Borrowed(z),
            false,
            Some((prec, Nearest)),
        )
    }

    /// Adds the product of two [`BigExponentFloat`]s to a [`BigExponentFloat`] in place, rounding
    /// the result to the nearest value of the specified precision. The [`BigExponentFloat`]s on the
    /// right-hand side are taken by value. An [`Ordering`] is returned, indicating whether the
    /// rounded result is less than, equal to, or greater than the exact result. Although `NaN`s are
    /// not comparable to any [`BigExponentFloat`], whenever this function sets the
    /// [`BigExponentFloat`] to `NaN` it also returns `Equal`.
    ///
    /// The product is computed exactly, so only one rounding occurs.
    ///
    /// $$
    /// x \gets x+yz+\varepsilon.
    /// $$
    /// - If $x+yz$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x+yz$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x+yz|\rfloor-p}$.
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
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.add_mul_prec_assign(BigExponentFloat::from(E), BigExponentFloat::from(SQRT_2), 5),
    ///     Greater
    /// );
    /// assert_eq!(format!("{x:?}"), "0x7.0");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.add_mul_prec_assign(
    ///         BigExponentFloat::from(E),
    ///         BigExponentFloat::from(SQRT_2),
    ///         20
    ///     ),
    ///     Greater
    /// );
    /// assert_eq!(format!("{x:?}"), "0x6.fc5f");
    /// ```
    #[inline]
    pub fn add_mul_prec_assign(&mut self, y: Self, z: Self, prec: u64) -> Ordering {
        let x = core::mem::replace(self, Self::ZERO);
        let (result, o) = add_mul_helper(
            Cow::Owned(x),
            Cow::Owned(y),
            Cow::Owned(z),
            false,
            Some((prec, Nearest)),
        );
        *self = result;
        o
    }

    /// Adds the product of two [`BigExponentFloat`]s to a [`BigExponentFloat`] in place, rounding
    /// the result to the nearest value of the specified precision. On the right-hand side, the
    /// first [`BigExponentFloat`] is taken by value and the second by reference. An [`Ordering`] is
    /// returned, indicating whether the rounded result is less than, equal to, or greater than the
    /// exact result. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this
    /// function sets the [`BigExponentFloat`] to `NaN` it also returns `Equal`.
    ///
    /// The product is computed exactly, so only one rounding occurs.
    ///
    /// $$
    /// x \gets x+yz+\varepsilon.
    /// $$
    /// - If $x+yz$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x+yz$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x+yz|\rfloor-p}$.
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
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.add_mul_prec_assign_val_ref(
    ///         BigExponentFloat::from(E),
    ///         &BigExponentFloat::from(SQRT_2),
    ///         5
    ///     ),
    ///     Greater
    /// );
    /// assert_eq!(format!("{x:?}"), "0x7.0");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.add_mul_prec_assign_val_ref(
    ///         BigExponentFloat::from(E),
    ///         &BigExponentFloat::from(SQRT_2),
    ///         20
    ///     ),
    ///     Greater
    /// );
    /// assert_eq!(format!("{x:?}"), "0x6.fc5f");
    /// ```
    #[inline]
    pub fn add_mul_prec_assign_val_ref(&mut self, y: Self, z: &Self, prec: u64) -> Ordering {
        let x = core::mem::replace(self, Self::ZERO);
        let (result, o) = add_mul_helper(
            Cow::Owned(x),
            Cow::Owned(y),
            Cow::Borrowed(z),
            false,
            Some((prec, Nearest)),
        );
        *self = result;
        o
    }

    /// Adds the product of two [`BigExponentFloat`]s to a [`BigExponentFloat`] in place, rounding
    /// the result to the nearest value of the specified precision. On the right-hand side, the
    /// second [`BigExponentFloat`] is taken by value and the first by reference. An [`Ordering`] is
    /// returned, indicating whether the rounded result is less than, equal to, or greater than the
    /// exact result. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this
    /// function sets the [`BigExponentFloat`] to `NaN` it also returns `Equal`.
    ///
    /// The product is computed exactly, so only one rounding occurs.
    ///
    /// $$
    /// x \gets x+yz+\varepsilon.
    /// $$
    /// - If $x+yz$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x+yz$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x+yz|\rfloor-p}$.
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
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.add_mul_prec_assign_ref_val(
    ///         &BigExponentFloat::from(E),
    ///         BigExponentFloat::from(SQRT_2),
    ///         5
    ///     ),
    ///     Greater
    /// );
    /// assert_eq!(format!("{x:?}"), "0x7.0");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.add_mul_prec_assign_ref_val(
    ///         &BigExponentFloat::from(E),
    ///         BigExponentFloat::from(SQRT_2),
    ///         20
    ///     ),
    ///     Greater
    /// );
    /// assert_eq!(format!("{x:?}"), "0x6.fc5f");
    /// ```
    #[inline]
    pub fn add_mul_prec_assign_ref_val(&mut self, y: &Self, z: Self, prec: u64) -> Ordering {
        let x = core::mem::replace(self, Self::ZERO);
        let (result, o) = add_mul_helper(
            Cow::Owned(x),
            Cow::Borrowed(y),
            Cow::Owned(z),
            false,
            Some((prec, Nearest)),
        );
        *self = result;
        o
    }

    /// Adds the product of two [`BigExponentFloat`]s to a [`BigExponentFloat`] in place, rounding
    /// the result to the nearest value of the specified precision. The [`BigExponentFloat`]s on the
    /// right-hand side are taken by reference. An [`Ordering`] is returned, indicating whether the
    /// rounded result is less than, equal to, or greater than the exact result. Although `NaN`s are
    /// not comparable to any [`BigExponentFloat`], whenever this function sets the
    /// [`BigExponentFloat`] to `NaN` it also returns `Equal`.
    ///
    /// The product is computed exactly, so only one rounding occurs.
    ///
    /// $$
    /// x \gets x+yz+\varepsilon.
    /// $$
    /// - If $x+yz$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x+yz$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x+yz|\rfloor-p}$.
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
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.add_mul_prec_assign_ref_ref(
    ///         &BigExponentFloat::from(E),
    ///         &BigExponentFloat::from(SQRT_2),
    ///         5
    ///     ),
    ///     Greater
    /// );
    /// assert_eq!(format!("{x:?}"), "0x7.0");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.add_mul_prec_assign_ref_ref(
    ///         &BigExponentFloat::from(E),
    ///         &BigExponentFloat::from(SQRT_2),
    ///         20
    ///     ),
    ///     Greater
    /// );
    /// assert_eq!(format!("{x:?}"), "0x6.fc5f");
    /// ```
    #[inline]
    pub fn add_mul_prec_assign_ref_ref(&mut self, y: &Self, z: &Self, prec: u64) -> Ordering {
        let x = core::mem::replace(self, Self::ZERO);
        let (result, o) = add_mul_helper(
            Cow::Owned(x),
            Cow::Borrowed(y),
            Cow::Borrowed(z),
            false,
            Some((prec, Nearest)),
        );
        *self = result;
        o
    }
}

impl AddMul<Self, Self> for BigExponentFloat {
    type Output = Self;

    /// Adds a [`BigExponentFloat`] and the product of two other [`BigExponentFloat`]s. All three
    /// [`BigExponentFloat`]s are taken by value. The result is exact.
    ///
    /// $$
    /// f(x,y,z) = x+yz.
    /// $$
    ///
    /// Special cases:
    /// - $f(x,y,z)=\text{NaN}$ if any of $x$, $y$, and $z$ is NaN, if one of $y$ and $z$ is
    ///   infinite and the other is zero, or if $x$ and $yz$ are infinities of opposite signs
    /// - $f(x,y,z)=\infty$ if $x$ or $yz$ is $\infty$ and neither is $-\infty$
    /// - $f(x,y,z)=-\infty$ if $x$ or $yz$ is $-\infty$ and neither is $\infty$
    /// - $f(x,y,z)=0$ if $x=-yz$ and both are finite
    ///
    /// When the two terms being added are finite and nonzero, the significand of the result spans
    /// the gap between their exponents. To bound its size, use
    /// [`BigExponentFloat::add_mul_prec_round`] or [`BigExponentFloat::add_mul_prec`] instead.
    ///
    /// This is FLINT's `arf_addmul` with precision `ARF_PREC_EXACT`, with $x$ as the accumulator.
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
    /// use malachite_base::num::arithmetic::traits::AddMul;
    /// use malachite_float::BigExponentFloat;
    ///
    /// assert_eq!(
    ///     format!(
    ///         "{:?}",
    ///         BigExponentFloat::from(1.5)
    ///             .add_mul(BigExponentFloat::from(0.25), BigExponentFloat::from(3.0))
    ///     ),
    ///     "0x2.4"
    /// );
    /// ```
    #[inline]
    fn add_mul(self, y: Self, z: Self) -> Self {
        add_mul_helper(Cow::Owned(self), Cow::Owned(y), Cow::Owned(z), false, None).0
    }
}

impl AddMul<Self, &Self> for BigExponentFloat {
    type Output = Self;

    /// Adds a [`BigExponentFloat`] and the product of two other [`BigExponentFloat`]s. The first
    /// and second [`BigExponentFloat`]s are taken by value and the third by reference. The result
    /// is exact.
    ///
    /// $$
    /// f(x,y,z) = x+yz.
    /// $$
    ///
    /// Special cases:
    /// - $f(x,y,z)=\text{NaN}$ if any of $x$, $y$, and $z$ is NaN, if one of $y$ and $z$ is
    ///   infinite and the other is zero, or if $x$ and $yz$ are infinities of opposite signs
    /// - $f(x,y,z)=\infty$ if $x$ or $yz$ is $\infty$ and neither is $-\infty$
    /// - $f(x,y,z)=-\infty$ if $x$ or $yz$ is $-\infty$ and neither is $\infty$
    /// - $f(x,y,z)=0$ if $x=-yz$ and both are finite
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
    /// use malachite_base::num::arithmetic::traits::AddMul;
    /// use malachite_float::BigExponentFloat;
    ///
    /// assert_eq!(
    ///     format!(
    ///         "{:?}",
    ///         BigExponentFloat::from(1.5)
    ///             .add_mul(BigExponentFloat::from(0.25), &BigExponentFloat::from(3.0))
    ///     ),
    ///     "0x2.4"
    /// );
    /// ```
    #[inline]
    fn add_mul(self, y: Self, z: &Self) -> Self {
        add_mul_helper(
            Cow::Owned(self),
            Cow::Owned(y),
            Cow::Borrowed(z),
            false,
            None,
        )
        .0
    }
}

impl AddMul<&Self, Self> for BigExponentFloat {
    type Output = Self;

    /// Adds a [`BigExponentFloat`] and the product of two other [`BigExponentFloat`]s. The first
    /// and third [`BigExponentFloat`]s are taken by value and the second by reference. The result
    /// is exact.
    ///
    /// $$
    /// f(x,y,z) = x+yz.
    /// $$
    ///
    /// Special cases:
    /// - $f(x,y,z)=\text{NaN}$ if any of $x$, $y$, and $z$ is NaN, if one of $y$ and $z$ is
    ///   infinite and the other is zero, or if $x$ and $yz$ are infinities of opposite signs
    /// - $f(x,y,z)=\infty$ if $x$ or $yz$ is $\infty$ and neither is $-\infty$
    /// - $f(x,y,z)=-\infty$ if $x$ or $yz$ is $-\infty$ and neither is $\infty$
    /// - $f(x,y,z)=0$ if $x=-yz$ and both are finite
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
    /// use malachite_base::num::arithmetic::traits::AddMul;
    /// use malachite_float::BigExponentFloat;
    ///
    /// assert_eq!(
    ///     format!(
    ///         "{:?}",
    ///         BigExponentFloat::from(1.5)
    ///             .add_mul(&BigExponentFloat::from(0.25), BigExponentFloat::from(3.0))
    ///     ),
    ///     "0x2.4"
    /// );
    /// ```
    #[inline]
    fn add_mul(self, y: &Self, z: Self) -> Self {
        add_mul_helper(
            Cow::Owned(self),
            Cow::Borrowed(y),
            Cow::Owned(z),
            false,
            None,
        )
        .0
    }
}

impl AddMul<&Self, &Self> for BigExponentFloat {
    type Output = Self;

    /// Adds a [`BigExponentFloat`] and the product of two other [`BigExponentFloat`]s. The first
    /// [`BigExponentFloat`] is taken by value and the second and third by reference. The result is
    /// exact.
    ///
    /// $$
    /// f(x,y,z) = x+yz.
    /// $$
    ///
    /// Special cases:
    /// - $f(x,y,z)=\text{NaN}$ if any of $x$, $y$, and $z$ is NaN, if one of $y$ and $z$ is
    ///   infinite and the other is zero, or if $x$ and $yz$ are infinities of opposite signs
    /// - $f(x,y,z)=\infty$ if $x$ or $yz$ is $\infty$ and neither is $-\infty$
    /// - $f(x,y,z)=-\infty$ if $x$ or $yz$ is $-\infty$ and neither is $\infty$
    /// - $f(x,y,z)=0$ if $x=-yz$ and both are finite
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
    /// use malachite_base::num::arithmetic::traits::AddMul;
    /// use malachite_float::BigExponentFloat;
    ///
    /// assert_eq!(
    ///     format!(
    ///         "{:?}",
    ///         BigExponentFloat::from(1.5)
    ///             .add_mul(&BigExponentFloat::from(0.25), &BigExponentFloat::from(3.0))
    ///     ),
    ///     "0x2.4"
    /// );
    /// ```
    #[inline]
    fn add_mul(self, y: &Self, z: &Self) -> Self {
        add_mul_helper(
            Cow::Owned(self),
            Cow::Borrowed(y),
            Cow::Borrowed(z),
            false,
            None,
        )
        .0
    }
}

impl AddMul<BigExponentFloat, BigExponentFloat> for &BigExponentFloat {
    type Output = BigExponentFloat;

    /// Adds a [`BigExponentFloat`] and the product of two other [`BigExponentFloat`]s. The second
    /// and third [`BigExponentFloat`]s are taken by value and the first by reference. The result is
    /// exact.
    ///
    /// $$
    /// f(x,y,z) = x+yz.
    /// $$
    ///
    /// Special cases:
    /// - $f(x,y,z)=\text{NaN}$ if any of $x$, $y$, and $z$ is NaN, if one of $y$ and $z$ is
    ///   infinite and the other is zero, or if $x$ and $yz$ are infinities of opposite signs
    /// - $f(x,y,z)=\infty$ if $x$ or $yz$ is $\infty$ and neither is $-\infty$
    /// - $f(x,y,z)=-\infty$ if $x$ or $yz$ is $-\infty$ and neither is $\infty$
    /// - $f(x,y,z)=0$ if $x=-yz$ and both are finite
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
    /// use malachite_base::num::arithmetic::traits::AddMul;
    /// use malachite_float::BigExponentFloat;
    ///
    /// assert_eq!(
    ///     format!(
    ///         "{:?}",
    ///         (&BigExponentFloat::from(1.5))
    ///             .add_mul(BigExponentFloat::from(0.25), BigExponentFloat::from(3.0))
    ///     ),
    ///     "0x2.4"
    /// );
    /// ```
    #[inline]
    fn add_mul(self, y: BigExponentFloat, z: BigExponentFloat) -> BigExponentFloat {
        add_mul_helper(
            Cow::Borrowed(self),
            Cow::Owned(y),
            Cow::Owned(z),
            false,
            None,
        )
        .0
    }
}

impl AddMul<BigExponentFloat, &BigExponentFloat> for &BigExponentFloat {
    type Output = BigExponentFloat;

    /// Adds a [`BigExponentFloat`] and the product of two other [`BigExponentFloat`]s. The second
    /// [`BigExponentFloat`] is taken by value and the first and third by reference. The result is
    /// exact.
    ///
    /// $$
    /// f(x,y,z) = x+yz.
    /// $$
    ///
    /// Special cases:
    /// - $f(x,y,z)=\text{NaN}$ if any of $x$, $y$, and $z$ is NaN, if one of $y$ and $z$ is
    ///   infinite and the other is zero, or if $x$ and $yz$ are infinities of opposite signs
    /// - $f(x,y,z)=\infty$ if $x$ or $yz$ is $\infty$ and neither is $-\infty$
    /// - $f(x,y,z)=-\infty$ if $x$ or $yz$ is $-\infty$ and neither is $\infty$
    /// - $f(x,y,z)=0$ if $x=-yz$ and both are finite
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
    /// use malachite_base::num::arithmetic::traits::AddMul;
    /// use malachite_float::BigExponentFloat;
    ///
    /// assert_eq!(
    ///     format!(
    ///         "{:?}",
    ///         (&BigExponentFloat::from(1.5))
    ///             .add_mul(BigExponentFloat::from(0.25), &BigExponentFloat::from(3.0))
    ///     ),
    ///     "0x2.4"
    /// );
    /// ```
    #[inline]
    fn add_mul(self, y: BigExponentFloat, z: &BigExponentFloat) -> BigExponentFloat {
        add_mul_helper(
            Cow::Borrowed(self),
            Cow::Owned(y),
            Cow::Borrowed(z),
            false,
            None,
        )
        .0
    }
}

impl AddMul<&BigExponentFloat, BigExponentFloat> for &BigExponentFloat {
    type Output = BigExponentFloat;

    /// Adds a [`BigExponentFloat`] and the product of two other [`BigExponentFloat`]s. The third
    /// [`BigExponentFloat`] is taken by value and the first and second by reference. The result is
    /// exact.
    ///
    /// $$
    /// f(x,y,z) = x+yz.
    /// $$
    ///
    /// Special cases:
    /// - $f(x,y,z)=\text{NaN}$ if any of $x$, $y$, and $z$ is NaN, if one of $y$ and $z$ is
    ///   infinite and the other is zero, or if $x$ and $yz$ are infinities of opposite signs
    /// - $f(x,y,z)=\infty$ if $x$ or $yz$ is $\infty$ and neither is $-\infty$
    /// - $f(x,y,z)=-\infty$ if $x$ or $yz$ is $-\infty$ and neither is $\infty$
    /// - $f(x,y,z)=0$ if $x=-yz$ and both are finite
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
    /// use malachite_base::num::arithmetic::traits::AddMul;
    /// use malachite_float::BigExponentFloat;
    ///
    /// assert_eq!(
    ///     format!(
    ///         "{:?}",
    ///         (&BigExponentFloat::from(1.5))
    ///             .add_mul(&BigExponentFloat::from(0.25), BigExponentFloat::from(3.0))
    ///     ),
    ///     "0x2.4"
    /// );
    /// ```
    #[inline]
    fn add_mul(self, y: &BigExponentFloat, z: BigExponentFloat) -> BigExponentFloat {
        add_mul_helper(
            Cow::Borrowed(self),
            Cow::Borrowed(y),
            Cow::Owned(z),
            false,
            None,
        )
        .0
    }
}

impl AddMul<&BigExponentFloat, &BigExponentFloat> for &BigExponentFloat {
    type Output = BigExponentFloat;

    /// Adds a [`BigExponentFloat`] and the product of two other [`BigExponentFloat`]s. All three
    /// [`BigExponentFloat`]s are taken by reference. The result is exact.
    ///
    /// $$
    /// f(x,y,z) = x+yz.
    /// $$
    ///
    /// Special cases:
    /// - $f(x,y,z)=\text{NaN}$ if any of $x$, $y$, and $z$ is NaN, if one of $y$ and $z$ is
    ///   infinite and the other is zero, or if $x$ and $yz$ are infinities of opposite signs
    /// - $f(x,y,z)=\infty$ if $x$ or $yz$ is $\infty$ and neither is $-\infty$
    /// - $f(x,y,z)=-\infty$ if $x$ or $yz$ is $-\infty$ and neither is $\infty$
    /// - $f(x,y,z)=0$ if $x=-yz$ and both are finite
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
    /// use malachite_base::num::arithmetic::traits::AddMul;
    /// use malachite_float::BigExponentFloat;
    ///
    /// assert_eq!(
    ///     format!(
    ///         "{:?}",
    ///         (&BigExponentFloat::from(1.5))
    ///             .add_mul(&BigExponentFloat::from(0.25), &BigExponentFloat::from(3.0))
    ///     ),
    ///     "0x2.4"
    /// );
    /// ```
    #[inline]
    fn add_mul(self, y: &BigExponentFloat, z: &BigExponentFloat) -> BigExponentFloat {
        add_mul_helper(
            Cow::Borrowed(self),
            Cow::Borrowed(y),
            Cow::Borrowed(z),
            false,
            None,
        )
        .0
    }
}

impl AddMulAssign<Self, Self> for BigExponentFloat {
    /// Adds the product of two [`BigExponentFloat`]s to a [`BigExponentFloat`] in place. The
    /// [`BigExponentFloat`]s on the right-hand side are taken by value. The result is exact.
    ///
    /// $$
    /// x \gets x+yz.
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
    /// use malachite_base::num::arithmetic::traits::AddMulAssign;
    /// use malachite_float::BigExponentFloat;
    ///
    /// let mut x = BigExponentFloat::from(1.5);
    /// x.add_mul_assign(BigExponentFloat::from(0.25), BigExponentFloat::from(3.0));
    /// assert_eq!(format!("{x:?}"), "0x2.4");
    /// ```
    #[inline]
    fn add_mul_assign(&mut self, y: Self, z: Self) {
        let x = core::mem::replace(self, Self::ZERO);
        *self = add_mul_helper(Cow::Owned(x), Cow::Owned(y), Cow::Owned(z), false, None).0;
    }
}

impl AddMulAssign<Self, &Self> for BigExponentFloat {
    /// Adds the product of two [`BigExponentFloat`]s to a [`BigExponentFloat`] in place. On the
    /// right-hand side, the first [`BigExponentFloat`] is taken by value and the second by
    /// reference. The result is exact.
    ///
    /// $$
    /// x \gets x+yz.
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
    /// use malachite_base::num::arithmetic::traits::AddMulAssign;
    /// use malachite_float::BigExponentFloat;
    ///
    /// let mut x = BigExponentFloat::from(1.5);
    /// x.add_mul_assign(BigExponentFloat::from(0.25), &BigExponentFloat::from(3.0));
    /// assert_eq!(format!("{x:?}"), "0x2.4");
    /// ```
    #[inline]
    fn add_mul_assign(&mut self, y: Self, z: &Self) {
        let x = core::mem::replace(self, Self::ZERO);
        *self = add_mul_helper(Cow::Owned(x), Cow::Owned(y), Cow::Borrowed(z), false, None).0;
    }
}

impl AddMulAssign<&Self, Self> for BigExponentFloat {
    /// Adds the product of two [`BigExponentFloat`]s to a [`BigExponentFloat`] in place. On the
    /// right-hand side, the second [`BigExponentFloat`] is taken by value and the first by
    /// reference. The result is exact.
    ///
    /// $$
    /// x \gets x+yz.
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
    /// use malachite_base::num::arithmetic::traits::AddMulAssign;
    /// use malachite_float::BigExponentFloat;
    ///
    /// let mut x = BigExponentFloat::from(1.5);
    /// x.add_mul_assign(&BigExponentFloat::from(0.25), BigExponentFloat::from(3.0));
    /// assert_eq!(format!("{x:?}"), "0x2.4");
    /// ```
    #[inline]
    fn add_mul_assign(&mut self, y: &Self, z: Self) {
        let x = core::mem::replace(self, Self::ZERO);
        *self = add_mul_helper(Cow::Owned(x), Cow::Borrowed(y), Cow::Owned(z), false, None).0;
    }
}

impl AddMulAssign<&Self, &Self> for BigExponentFloat {
    /// Adds the product of two [`BigExponentFloat`]s to a [`BigExponentFloat`] in place. The
    /// [`BigExponentFloat`]s on the right-hand side are taken by reference. The result is exact.
    ///
    /// $$
    /// x \gets x+yz.
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
    /// use malachite_base::num::arithmetic::traits::AddMulAssign;
    /// use malachite_float::BigExponentFloat;
    ///
    /// let mut x = BigExponentFloat::from(1.5);
    /// x.add_mul_assign(&BigExponentFloat::from(0.25), &BigExponentFloat::from(3.0));
    /// assert_eq!(format!("{x:?}"), "0x2.4");
    /// ```
    #[inline]
    fn add_mul_assign(&mut self, y: &Self, z: &Self) {
        let x = core::mem::replace(self, Self::ZERO);
        *self = add_mul_helper(
            Cow::Owned(x),
            Cow::Borrowed(y),
            Cow::Borrowed(z),
            false,
            None,
        )
        .0;
    }
}
