// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Float;
use crate::big_exponent_float::BigExponentFloat;
use crate::big_exponent_float::InnerBigExponentFloat::{Finite, Infinity, NaN, Zero};
use crate::big_exponent_float::arithmetic::add::{
    add_helper, add_prec_round_generic, significand_prec,
};
use crate::big_exponent_float::arithmetic::mul_rational::{
    finite_to_float, float_rm, from_float_result,
};
use crate::big_exponent_float::arithmetic::sqrt::sqrt_prec_round_generic;
use alloc::borrow::Cow;
use core::cmp::Ordering::{self, *};
use core::cmp::max;
use malachite_base::num::arithmetic::traits::{Abs, Square};
use malachite_base::num::basic::traits::{
    Infinity as InfinityTrait, NaN as NaNTrait, One, Zero as ZeroTrait,
};
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;

// sqrt(x^2 + y^2), rounded to `prec` bits with `rm`.
pub(crate) fn hypot_helper(
    x: Cow<'_, BigExponentFloat>,
    y: Cow<'_, BigExponentFloat>,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    hypot_helper_with(x, y, prec, rm, true)
}

crate_test_fn! {
    // sqrt(x^2 + y^2) rounded to `prec` bits with `rm`, without `Float`'s kernels: the exact sum of
    // squares is formed and its square root rounded with the generic square root algorithm; or,
    // when one operand is far below the other, the generic addition described in
    // `hypot_helper_with` is used. Used to check and benchmark the kernel path.
    hypot_prec_round_generic(
        x: &BigExponentFloat,
        y: &BigExponentFloat,
        prec: u64,
        rm: RoundingMode,
    ) -> (BigExponentFloat, Ordering) {
        hypot_helper_with(Cow::Borrowed(x), Cow::Borrowed(y), prec, rm, false)
    }
}

// If y is so far below x that sqrt(x^2 + y^2) rounds like |x| + s for a short s, returns s. Both
// operands are finite and nonzero, and x's exponent is at least y's.
//
// Let x and y have exponents e and f, let p = max(prec(x), prec + 1), and let d = sqrt(x^2 + y^2) -
// |x|. Then 0 < d = y^2 / (sqrt(x^2 + y^2) + |x|) < y^2 / (2|x|) < 2^(2f - e). The rounding
// breakpoints at `prec` bits near |x|, including the `Nearest` midpoints, are multiples of 2^(e -
// p), as is |x| itself; so if d < 2^(e - p), the hypotenuse lies strictly between |x| and the next
// multiple, and rounds the same as any other such value, such as |x| + s for s = 2^(e - p - 1).
// This holds when 2f <= 2e - p.
fn far_below_offset(
    x: &BigExponentFloat,
    y: &BigExponentFloat,
    prec: u64,
) -> Option<BigExponentFloat> {
    let (
        BigExponentFloat(Finite {
            exponent: e,
            significand,
            ..
        }),
        BigExponentFloat(Finite { exponent: f, .. }),
    ) = (x, y)
    else {
        unreachable!()
    };
    let p = Integer::from(max(significand_prec(significand), prec + 1));
    if (f << 1u32) > (e << 1u32) - &p {
        None
    } else {
        Some(BigExponentFloat::from_parts(true, e - p, Natural::ONE))
    }
}

// The special cases are those of `Float`'s `hypot`: if either operand is infinite, the result is
// infinity, even if the other is NaN.
//
// If one operand is far below the other, the hypotenuse is |x| + s rounded, for the larger operand
// x and the s from `far_below_offset`. When x is negative, x - s = -(|x| + s) is rounded in the
// opposite direction and negated.
//
// Otherwise, with `use_kernels`, both operands are scaled by the same power of 2, so that the
// larger exponent is 0, and the hypotenuse is computed with `Float`'s `hypot_prec_round` and scaled
// back. The scaled result is between 2^(-1) and 2, far from overflow and underflow. Without
// `use_kernels`, or if the scaled smaller operand is outside `Float`'s exponent range (possible
// only if x or `prec` has billions of bits), the exact sum of squares is formed and its square root
// rounded with the generic algorithm.
fn hypot_helper_with(
    x: Cow<'_, BigExponentFloat>,
    y: Cow<'_, BigExponentFloat>,
    prec: u64,
    rm: RoundingMode,
    use_kernels: bool,
) -> (BigExponentFloat, Ordering) {
    assert_ne!(prec, 0);
    match (&*x, &*y) {
        (BigExponentFloat(Infinity { .. }), _) | (_, BigExponentFloat(Infinity { .. })) => {
            return (BigExponentFloat::INFINITY, Equal);
        }
        (BigExponentFloat(NaN), _) | (_, BigExponentFloat(NaN)) => {
            return (BigExponentFloat::NAN, Equal);
        }
        (BigExponentFloat(Zero), _) => return y.into_owned().abs().round_to_prec_round(prec, rm),
        (_, BigExponentFloat(Zero)) => return x.into_owned().abs().round_to_prec_round(prec, rm),
        _ => {}
    }
    // Order the operands so that x's exponent is at least y's.
    let (x, y) = if x.get_exponent() >= y.get_exponent() {
        (x, y)
    } else {
        (y, x)
    };
    if let Some(s) = far_below_offset(&x, &y, prec) {
        let negative = x.is_sign_negative();
        let (s, rm) = if negative { (s.negate(), -rm) } else { (s, rm) };
        let (z, o) = if use_kernels {
            add_helper(x, Cow::Owned(s), false, Some((prec, rm)))
        } else {
            add_prec_round_generic(&x, &s, false, prec, rm)
        };
        return if negative {
            (z.negate(), o.reverse())
        } else {
            (z, o)
        };
    }
    if !use_kernels {
        return hypot_exact_generic(&x, &y, prec, rm);
    }
    let e = x.get_exponent().unwrap().clone();
    let y_exponent = y.get_exponent().unwrap() - &e;
    let y_exponent = match i32::try_from(&y_exponent) {
        Ok(y_exponent) if y_exponent >= Float::MIN_EXPONENT => y_exponent,
        _ => return hypot_exact_generic(&x, &y, prec, rm),
    };
    let (z, o) = from_float_result(
        finite_to_float(x, 0).hypot_prec_round(finite_to_float(y, y_exponent), prec, float_rm(rm)),
        Some(&e),
    )
    .unwrap();
    assert!(rm != Exact || o == Equal, "Inexact BigExponentFloat hypot");
    (z, o)
}

// sqrt(x^2 + y^2) for finite nonzero x and y, from the exact sum of squares.
fn hypot_exact_generic(
    x: &BigExponentFloat,
    y: &BigExponentFloat,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    sqrt_prec_round_generic(&(x.square() + y.square()), prec, rm)
}

impl BigExponentFloat {
    /// Computes the hypotenuse of two [`BigExponentFloat`]s, $\sqrt{x^2+y^2}$, rounding the result
    /// to the specified precision and with the specified rounding mode. Both [`BigExponentFloat`]s
    /// are taken by value. An [`Ordering`] is also returned, indicating whether the rounded
    /// hypotenuse is less than, equal to, or greater than the exact hypotenuse. Although `NaN`s are
    /// not comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,p,m) = \sqrt{x^2+y^2}+\varepsilon.
    /// $$
    /// - If $\sqrt{x^2+y^2}$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed
    ///   to be 0.
    /// - If $\sqrt{x^2+y^2}$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 \sqrt{x^2+y^2}\rfloor-p+1}$.
    /// - If $\sqrt{x^2+y^2}$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 \sqrt{x^2+y^2}\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\pm\infty,y,p,m)=f(x,\pm\infty,p,m)=\infty$, even when the other argument is `NaN`
    /// - $f(\text{NaN},y,p,m)=f(x,\text{NaN},p,m)=\text{NaN}$ if the other argument is not infinite
    /// - $f(0,0,p,m)=0$
    ///
    /// If you know you'll be using `Nearest`, consider using [`BigExponentFloat::hypot_prec`]
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O((n + m) \log (n + m) \log\log (n + m))$
    ///
    /// $M(n, m) = O(n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `max(self.significant_bits(), other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the hypotenuse cannot be represented
    /// exactly with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (hypot, o) =
    ///     BigExponentFloat::from(PI).hypot_prec_round(BigExponentFloat::from(E), 5, Floor);
    /// assert_eq!(format!("{hypot:?}"), "0x4.0");
    /// assert_eq!(o, Less);
    ///
    /// let (hypot, o) =
    ///     BigExponentFloat::from(PI).hypot_prec_round(BigExponentFloat::from(E), 5, Ceiling);
    /// assert_eq!(format!("{hypot:?}"), "0x4.4");
    /// assert_eq!(o, Greater);
    ///
    /// let (hypot, o) =
    ///     BigExponentFloat::from(PI).hypot_prec_round(BigExponentFloat::from(E), 5, Nearest);
    /// assert_eq!(format!("{hypot:?}"), "0x4.4");
    /// assert_eq!(o, Greater);
    ///
    /// let (hypot, o) =
    ///     BigExponentFloat::from(PI).hypot_prec_round(BigExponentFloat::from(E), 20, Floor);
    /// assert_eq!(format!("{hypot:?}"), "0x4.27838");
    /// assert_eq!(o, Less);
    ///
    /// let (hypot, o) =
    ///     BigExponentFloat::from(PI).hypot_prec_round(BigExponentFloat::from(E), 20, Ceiling);
    /// assert_eq!(format!("{hypot:?}"), "0x4.2784");
    /// assert_eq!(o, Greater);
    ///
    /// let (hypot, o) =
    ///     BigExponentFloat::from(PI).hypot_prec_round(BigExponentFloat::from(E), 20, Nearest);
    /// assert_eq!(format!("{hypot:?}"), "0x4.2784");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn hypot_prec_round(self, other: Self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        hypot_helper(Cow::Owned(self), Cow::Owned(other), prec, rm)
    }

    /// Computes the hypotenuse of two [`BigExponentFloat`]s, $\sqrt{x^2+y^2}$, rounding the result
    /// to the specified precision and with the specified rounding mode. The first
    /// [`BigExponentFloat`] is taken by value and the second by reference. An [`Ordering`] is also
    /// returned, indicating whether the rounded hypotenuse is less than, equal to, or greater than
    /// the exact hypotenuse. Although `NaN`s are not comparable to any [`BigExponentFloat`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,p,m) = \sqrt{x^2+y^2}+\varepsilon.
    /// $$
    /// - If $\sqrt{x^2+y^2}$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed
    ///   to be 0.
    /// - If $\sqrt{x^2+y^2}$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 \sqrt{x^2+y^2}\rfloor-p+1}$.
    /// - If $\sqrt{x^2+y^2}$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 \sqrt{x^2+y^2}\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\pm\infty,y,p,m)=f(x,\pm\infty,p,m)=\infty$, even when the other argument is `NaN`
    /// - $f(\text{NaN},y,p,m)=f(x,\text{NaN},p,m)=\text{NaN}$ if the other argument is not infinite
    /// - $f(0,0,p,m)=0$
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O((n + m) \log (n + m) \log\log (n + m))$
    ///
    /// $M(n, m) = O(n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `max(self.significant_bits(), other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the hypotenuse cannot be represented
    /// exactly with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (hypot, o) = BigExponentFloat::from(PI).hypot_prec_round_val_ref(
    ///     &BigExponentFloat::from(E),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{hypot:?}"), "0x4.0");
    /// assert_eq!(o, Less);
    ///
    /// let (hypot, o) = BigExponentFloat::from(PI).hypot_prec_round_val_ref(
    ///     &BigExponentFloat::from(E),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{hypot:?}"), "0x4.4");
    /// assert_eq!(o, Greater);
    ///
    /// let (hypot, o) = BigExponentFloat::from(PI).hypot_prec_round_val_ref(
    ///     &BigExponentFloat::from(E),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{hypot:?}"), "0x4.4");
    /// assert_eq!(o, Greater);
    ///
    /// let (hypot, o) = BigExponentFloat::from(PI).hypot_prec_round_val_ref(
    ///     &BigExponentFloat::from(E),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{hypot:?}"), "0x4.27838");
    /// assert_eq!(o, Less);
    ///
    /// let (hypot, o) = BigExponentFloat::from(PI).hypot_prec_round_val_ref(
    ///     &BigExponentFloat::from(E),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{hypot:?}"), "0x4.2784");
    /// assert_eq!(o, Greater);
    ///
    /// let (hypot, o) = BigExponentFloat::from(PI).hypot_prec_round_val_ref(
    ///     &BigExponentFloat::from(E),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{hypot:?}"), "0x4.2784");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn hypot_prec_round_val_ref(
        self,
        other: &Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        hypot_helper(Cow::Owned(self), Cow::Borrowed(other), prec, rm)
    }

    /// Computes the hypotenuse of two [`BigExponentFloat`]s, $\sqrt{x^2+y^2}$, rounding the result
    /// to the specified precision and with the specified rounding mode. The first
    /// [`BigExponentFloat`] is taken by reference and the second by value. An [`Ordering`] is also
    /// returned, indicating whether the rounded hypotenuse is less than, equal to, or greater than
    /// the exact hypotenuse. Although `NaN`s are not comparable to any [`BigExponentFloat`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,p,m) = \sqrt{x^2+y^2}+\varepsilon.
    /// $$
    /// - If $\sqrt{x^2+y^2}$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed
    ///   to be 0.
    /// - If $\sqrt{x^2+y^2}$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 \sqrt{x^2+y^2}\rfloor-p+1}$.
    /// - If $\sqrt{x^2+y^2}$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 \sqrt{x^2+y^2}\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\pm\infty,y,p,m)=f(x,\pm\infty,p,m)=\infty$, even when the other argument is `NaN`
    /// - $f(\text{NaN},y,p,m)=f(x,\text{NaN},p,m)=\text{NaN}$ if the other argument is not infinite
    /// - $f(0,0,p,m)=0$
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O((n + m) \log (n + m) \log\log (n + m))$
    ///
    /// $M(n, m) = O(n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `max(self.significant_bits(), other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the hypotenuse cannot be represented
    /// exactly with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (hypot, o) = (&BigExponentFloat::from(PI)).hypot_prec_round_ref_val(
    ///     BigExponentFloat::from(E),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{hypot:?}"), "0x4.0");
    /// assert_eq!(o, Less);
    ///
    /// let (hypot, o) = (&BigExponentFloat::from(PI)).hypot_prec_round_ref_val(
    ///     BigExponentFloat::from(E),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{hypot:?}"), "0x4.4");
    /// assert_eq!(o, Greater);
    ///
    /// let (hypot, o) = (&BigExponentFloat::from(PI)).hypot_prec_round_ref_val(
    ///     BigExponentFloat::from(E),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{hypot:?}"), "0x4.4");
    /// assert_eq!(o, Greater);
    ///
    /// let (hypot, o) = (&BigExponentFloat::from(PI)).hypot_prec_round_ref_val(
    ///     BigExponentFloat::from(E),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{hypot:?}"), "0x4.27838");
    /// assert_eq!(o, Less);
    ///
    /// let (hypot, o) = (&BigExponentFloat::from(PI)).hypot_prec_round_ref_val(
    ///     BigExponentFloat::from(E),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{hypot:?}"), "0x4.2784");
    /// assert_eq!(o, Greater);
    ///
    /// let (hypot, o) = (&BigExponentFloat::from(PI)).hypot_prec_round_ref_val(
    ///     BigExponentFloat::from(E),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{hypot:?}"), "0x4.2784");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn hypot_prec_round_ref_val(
        &self,
        other: Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        hypot_helper(Cow::Borrowed(self), Cow::Owned(other), prec, rm)
    }

    /// Computes the hypotenuse of two [`BigExponentFloat`]s, $\sqrt{x^2+y^2}$, rounding the result
    /// to the specified precision and with the specified rounding mode. Both [`BigExponentFloat`]s
    /// are taken by reference. An [`Ordering`] is also returned, indicating whether the rounded
    /// hypotenuse is less than, equal to, or greater than the exact hypotenuse. Although `NaN`s are
    /// not comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,p,m) = \sqrt{x^2+y^2}+\varepsilon.
    /// $$
    /// - If $\sqrt{x^2+y^2}$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed
    ///   to be 0.
    /// - If $\sqrt{x^2+y^2}$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 \sqrt{x^2+y^2}\rfloor-p+1}$.
    /// - If $\sqrt{x^2+y^2}$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 \sqrt{x^2+y^2}\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\pm\infty,y,p,m)=f(x,\pm\infty,p,m)=\infty$, even when the other argument is `NaN`
    /// - $f(\text{NaN},y,p,m)=f(x,\text{NaN},p,m)=\text{NaN}$ if the other argument is not infinite
    /// - $f(0,0,p,m)=0$
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O((n + m) \log (n + m) \log\log (n + m))$
    ///
    /// $M(n, m) = O(n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `max(self.significant_bits(), other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the hypotenuse cannot be represented
    /// exactly with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (hypot, o) = (&BigExponentFloat::from(PI)).hypot_prec_round_ref_ref(
    ///     &BigExponentFloat::from(E),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{hypot:?}"), "0x4.0");
    /// assert_eq!(o, Less);
    ///
    /// let (hypot, o) = (&BigExponentFloat::from(PI)).hypot_prec_round_ref_ref(
    ///     &BigExponentFloat::from(E),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{hypot:?}"), "0x4.4");
    /// assert_eq!(o, Greater);
    ///
    /// let (hypot, o) = (&BigExponentFloat::from(PI)).hypot_prec_round_ref_ref(
    ///     &BigExponentFloat::from(E),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{hypot:?}"), "0x4.4");
    /// assert_eq!(o, Greater);
    ///
    /// let (hypot, o) = (&BigExponentFloat::from(PI)).hypot_prec_round_ref_ref(
    ///     &BigExponentFloat::from(E),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{hypot:?}"), "0x4.27838");
    /// assert_eq!(o, Less);
    ///
    /// let (hypot, o) = (&BigExponentFloat::from(PI)).hypot_prec_round_ref_ref(
    ///     &BigExponentFloat::from(E),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{hypot:?}"), "0x4.2784");
    /// assert_eq!(o, Greater);
    ///
    /// let (hypot, o) = (&BigExponentFloat::from(PI)).hypot_prec_round_ref_ref(
    ///     &BigExponentFloat::from(E),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{hypot:?}"), "0x4.2784");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn hypot_prec_round_ref_ref(
        &self,
        other: &Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        hypot_helper(Cow::Borrowed(self), Cow::Borrowed(other), prec, rm)
    }

    /// Replaces a [`BigExponentFloat`] $x$ with the hypotenuse $\sqrt{x^2+y^2}$ of $x$ and another
    /// [`BigExponentFloat`] $y$, rounding the result to the specified precision and with the
    /// specified rounding mode. The [`BigExponentFloat`] on the right-hand side is taken by value.
    /// An [`Ordering`] is returned, indicating whether the rounded hypotenuse is less than, equal
    /// to, or greater than the exact hypotenuse. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function sets the [`BigExponentFloat`] to `NaN` it also
    /// returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets \sqrt{x^2+y^2}+\varepsilon.
    /// $$
    /// - If $\sqrt{x^2+y^2}$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed
    ///   to be 0.
    /// - If $\sqrt{x^2+y^2}$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 \sqrt{x^2+y^2}\rfloor-p+1}$.
    /// - If $\sqrt{x^2+y^2}$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 \sqrt{x^2+y^2}\rfloor-p}$.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O((n + m) \log (n + m) \log\log (n + m))$
    ///
    /// $M(n, m) = O(n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `max(self.significant_bits(), other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the hypotenuse cannot be represented
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
    ///     x.hypot_prec_round_assign(BigExponentFloat::from(E), 5, Floor),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x4.0");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.hypot_prec_round_assign(BigExponentFloat::from(E), 5, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(format!("{x:?}"), "0x4.4");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.hypot_prec_round_assign(BigExponentFloat::from(E), 5, Nearest),
    ///     Greater
    /// );
    /// assert_eq!(format!("{x:?}"), "0x4.4");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.hypot_prec_round_assign(BigExponentFloat::from(E), 20, Floor),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x4.27838");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.hypot_prec_round_assign(BigExponentFloat::from(E), 20, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(format!("{x:?}"), "0x4.2784");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.hypot_prec_round_assign(BigExponentFloat::from(E), 20, Nearest),
    ///     Greater
    /// );
    /// assert_eq!(format!("{x:?}"), "0x4.2784");
    /// ```
    #[inline]
    pub fn hypot_prec_round_assign(
        &mut self,
        other: Self,
        prec: u64,
        rm: RoundingMode,
    ) -> Ordering {
        let x = core::mem::replace(self, Self::ZERO);
        let (result, o) = hypot_helper(Cow::Owned(x), Cow::Owned(other), prec, rm);
        *self = result;
        o
    }

    /// Replaces a [`BigExponentFloat`] $x$ with the hypotenuse $\sqrt{x^2+y^2}$ of $x$ and another
    /// [`BigExponentFloat`] $y$, rounding the result to the specified precision and with the
    /// specified rounding mode. The [`BigExponentFloat`] on the right-hand side is taken by
    /// reference. An [`Ordering`] is returned, indicating whether the rounded hypotenuse is less
    /// than, equal to, or greater than the exact hypotenuse. Although `NaN`s are not comparable to
    /// any [`BigExponentFloat`], whenever this function sets the [`BigExponentFloat`] to `NaN` it
    /// also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets \sqrt{x^2+y^2}+\varepsilon.
    /// $$
    /// - If $\sqrt{x^2+y^2}$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed
    ///   to be 0.
    /// - If $\sqrt{x^2+y^2}$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 \sqrt{x^2+y^2}\rfloor-p+1}$.
    /// - If $\sqrt{x^2+y^2}$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 \sqrt{x^2+y^2}\rfloor-p}$.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O((n + m) \log (n + m) \log\log (n + m))$
    ///
    /// $M(n, m) = O(n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `max(self.significant_bits(), other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the hypotenuse cannot be represented
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
    ///     x.hypot_prec_round_assign_ref(&BigExponentFloat::from(E), 5, Floor),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x4.0");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.hypot_prec_round_assign_ref(&BigExponentFloat::from(E), 5, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(format!("{x:?}"), "0x4.4");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.hypot_prec_round_assign_ref(&BigExponentFloat::from(E), 5, Nearest),
    ///     Greater
    /// );
    /// assert_eq!(format!("{x:?}"), "0x4.4");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.hypot_prec_round_assign_ref(&BigExponentFloat::from(E), 20, Floor),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x4.27838");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.hypot_prec_round_assign_ref(&BigExponentFloat::from(E), 20, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(format!("{x:?}"), "0x4.2784");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.hypot_prec_round_assign_ref(&BigExponentFloat::from(E), 20, Nearest),
    ///     Greater
    /// );
    /// assert_eq!(format!("{x:?}"), "0x4.2784");
    /// ```
    #[inline]
    pub fn hypot_prec_round_assign_ref(
        &mut self,
        other: &Self,
        prec: u64,
        rm: RoundingMode,
    ) -> Ordering {
        let x = core::mem::replace(self, Self::ZERO);
        let (result, o) = hypot_helper(Cow::Owned(x), Cow::Borrowed(other), prec, rm);
        *self = result;
        o
    }

    /// Computes the hypotenuse of two [`BigExponentFloat`]s, $\sqrt{x^2+y^2}$, rounding the result
    /// to the nearest value of the specified precision. Both [`BigExponentFloat`]s are taken by
    /// value. An [`Ordering`] is also returned, indicating whether the rounded hypotenuse is less
    /// than, equal to, or greater than the exact hypotenuse. Although `NaN`s are not comparable to
    /// any [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,y,p) = \sqrt{x^2+y^2}+\varepsilon.
    /// $$
    /// - If $\sqrt{x^2+y^2}$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed
    ///   to be 0.
    /// - If $\sqrt{x^2+y^2}$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   \sqrt{x^2+y^2}\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\pm\infty,y,p)=f(x,\pm\infty,p)=\infty$, even when the other argument is `NaN`
    /// - $f(\text{NaN},y,p)=f(x,\text{NaN},p)=\text{NaN}$ if the other argument is not infinite
    /// - $f(0,0,p)=0$
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`BigExponentFloat::hypot_prec_round`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O((n + m) \log (n + m) \log\log (n + m))$
    ///
    /// $M(n, m) = O(n + m)$
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
    /// let (hypot, o) = BigExponentFloat::from(PI).hypot_prec(BigExponentFloat::from(E), 5);
    /// assert_eq!(format!("{hypot:?}"), "0x4.4");
    /// assert_eq!(o, Greater);
    ///
    /// let (hypot, o) = BigExponentFloat::from(PI).hypot_prec(BigExponentFloat::from(E), 20);
    /// assert_eq!(format!("{hypot:?}"), "0x4.2784");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn hypot_prec(self, other: Self, prec: u64) -> (Self, Ordering) {
        hypot_helper(Cow::Owned(self), Cow::Owned(other), prec, Nearest)
    }

    /// Computes the hypotenuse of two [`BigExponentFloat`]s, $\sqrt{x^2+y^2}$, rounding the result
    /// to the nearest value of the specified precision. The first [`BigExponentFloat`] is taken by
    /// value and the second by reference. An [`Ordering`] is also returned, indicating whether the
    /// rounded hypotenuse is less than, equal to, or greater than the exact hypotenuse. Although
    /// `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function returns a
    /// `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,y,p) = \sqrt{x^2+y^2}+\varepsilon.
    /// $$
    /// - If $\sqrt{x^2+y^2}$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed
    ///   to be 0.
    /// - If $\sqrt{x^2+y^2}$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   \sqrt{x^2+y^2}\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\pm\infty,y,p)=f(x,\pm\infty,p)=\infty$, even when the other argument is `NaN`
    /// - $f(\text{NaN},y,p)=f(x,\text{NaN},p)=\text{NaN}$ if the other argument is not infinite
    /// - $f(0,0,p)=0$
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O((n + m) \log (n + m) \log\log (n + m))$
    ///
    /// $M(n, m) = O(n + m)$
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
    /// let (hypot, o) =
    ///     BigExponentFloat::from(PI).hypot_prec_val_ref(&BigExponentFloat::from(E), 5);
    /// assert_eq!(format!("{hypot:?}"), "0x4.4");
    /// assert_eq!(o, Greater);
    ///
    /// let (hypot, o) =
    ///     BigExponentFloat::from(PI).hypot_prec_val_ref(&BigExponentFloat::from(E), 20);
    /// assert_eq!(format!("{hypot:?}"), "0x4.2784");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn hypot_prec_val_ref(self, other: &Self, prec: u64) -> (Self, Ordering) {
        hypot_helper(Cow::Owned(self), Cow::Borrowed(other), prec, Nearest)
    }

    /// Computes the hypotenuse of two [`BigExponentFloat`]s, $\sqrt{x^2+y^2}$, rounding the result
    /// to the nearest value of the specified precision. The first [`BigExponentFloat`] is taken by
    /// reference and the second by value. An [`Ordering`] is also returned, indicating whether the
    /// rounded hypotenuse is less than, equal to, or greater than the exact hypotenuse. Although
    /// `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function returns a
    /// `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,y,p) = \sqrt{x^2+y^2}+\varepsilon.
    /// $$
    /// - If $\sqrt{x^2+y^2}$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed
    ///   to be 0.
    /// - If $\sqrt{x^2+y^2}$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   \sqrt{x^2+y^2}\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\pm\infty,y,p)=f(x,\pm\infty,p)=\infty$, even when the other argument is `NaN`
    /// - $f(\text{NaN},y,p)=f(x,\text{NaN},p)=\text{NaN}$ if the other argument is not infinite
    /// - $f(0,0,p)=0$
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O((n + m) \log (n + m) \log\log (n + m))$
    ///
    /// $M(n, m) = O(n + m)$
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
    /// let (hypot, o) =
    ///     (&BigExponentFloat::from(PI)).hypot_prec_ref_val(BigExponentFloat::from(E), 5);
    /// assert_eq!(format!("{hypot:?}"), "0x4.4");
    /// assert_eq!(o, Greater);
    ///
    /// let (hypot, o) =
    ///     (&BigExponentFloat::from(PI)).hypot_prec_ref_val(BigExponentFloat::from(E), 20);
    /// assert_eq!(format!("{hypot:?}"), "0x4.2784");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn hypot_prec_ref_val(&self, other: Self, prec: u64) -> (Self, Ordering) {
        hypot_helper(Cow::Borrowed(self), Cow::Owned(other), prec, Nearest)
    }

    /// Computes the hypotenuse of two [`BigExponentFloat`]s, $\sqrt{x^2+y^2}$, rounding the result
    /// to the nearest value of the specified precision. Both [`BigExponentFloat`]s are taken by
    /// reference. An [`Ordering`] is also returned, indicating whether the rounded hypotenuse is
    /// less than, equal to, or greater than the exact hypotenuse. Although `NaN`s are not
    /// comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// $$
    /// f(x,y,p) = \sqrt{x^2+y^2}+\varepsilon.
    /// $$
    /// - If $\sqrt{x^2+y^2}$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed
    ///   to be 0.
    /// - If $\sqrt{x^2+y^2}$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   \sqrt{x^2+y^2}\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\pm\infty,y,p)=f(x,\pm\infty,p)=\infty$, even when the other argument is `NaN`
    /// - $f(\text{NaN},y,p)=f(x,\text{NaN},p)=\text{NaN}$ if the other argument is not infinite
    /// - $f(0,0,p)=0$
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O((n + m) \log (n + m) \log\log (n + m))$
    ///
    /// $M(n, m) = O(n + m)$
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
    /// let (hypot, o) =
    ///     (&BigExponentFloat::from(PI)).hypot_prec_ref_ref(&BigExponentFloat::from(E), 5);
    /// assert_eq!(format!("{hypot:?}"), "0x4.4");
    /// assert_eq!(o, Greater);
    ///
    /// let (hypot, o) =
    ///     (&BigExponentFloat::from(PI)).hypot_prec_ref_ref(&BigExponentFloat::from(E), 20);
    /// assert_eq!(format!("{hypot:?}"), "0x4.2784");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn hypot_prec_ref_ref(&self, other: &Self, prec: u64) -> (Self, Ordering) {
        hypot_helper(Cow::Borrowed(self), Cow::Borrowed(other), prec, Nearest)
    }

    /// Replaces a [`BigExponentFloat`] $x$ with the hypotenuse $\sqrt{x^2+y^2}$ of $x$ and another
    /// [`BigExponentFloat`] $y$, rounding the result to the nearest value of the specified
    /// precision. The [`BigExponentFloat`] on the right-hand side is taken by value. An
    /// [`Ordering`] is returned, indicating whether the rounded hypotenuse is less than, equal to,
    /// or greater than the exact hypotenuse. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function sets the [`BigExponentFloat`] to `NaN` it also
    /// returns `Equal`.
    ///
    /// $$
    /// x \gets \sqrt{x^2+y^2}+\varepsilon.
    /// $$
    /// - If $\sqrt{x^2+y^2}$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed
    ///   to be 0.
    /// - If $\sqrt{x^2+y^2}$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   \sqrt{x^2+y^2}\rfloor-p}$.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O((n + m) \log (n + m) \log\log (n + m))$
    ///
    /// $M(n, m) = O(n + m)$
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
    /// assert_eq!(x.hypot_prec_assign(BigExponentFloat::from(E), 5), Greater);
    /// assert_eq!(format!("{x:?}"), "0x4.4");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(x.hypot_prec_assign(BigExponentFloat::from(E), 20), Greater);
    /// assert_eq!(format!("{x:?}"), "0x4.2784");
    /// ```
    #[inline]
    pub fn hypot_prec_assign(&mut self, other: Self, prec: u64) -> Ordering {
        let x = core::mem::replace(self, Self::ZERO);
        let (result, o) = hypot_helper(Cow::Owned(x), Cow::Owned(other), prec, Nearest);
        *self = result;
        o
    }

    /// Replaces a [`BigExponentFloat`] $x$ with the hypotenuse $\sqrt{x^2+y^2}$ of $x$ and another
    /// [`BigExponentFloat`] $y$, rounding the result to the nearest value of the specified
    /// precision. The [`BigExponentFloat`] on the right-hand side is taken by reference. An
    /// [`Ordering`] is returned, indicating whether the rounded hypotenuse is less than, equal to,
    /// or greater than the exact hypotenuse. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function sets the [`BigExponentFloat`] to `NaN` it also
    /// returns `Equal`.
    ///
    /// $$
    /// x \gets \sqrt{x^2+y^2}+\varepsilon.
    /// $$
    /// - If $\sqrt{x^2+y^2}$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed
    ///   to be 0.
    /// - If $\sqrt{x^2+y^2}$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   \sqrt{x^2+y^2}\rfloor-p}$.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O((n + m) \log (n + m) \log\log (n + m))$
    ///
    /// $M(n, m) = O(n + m)$
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
    /// assert_eq!(
    ///     x.hypot_prec_assign_ref(&BigExponentFloat::from(E), 5),
    ///     Greater
    /// );
    /// assert_eq!(format!("{x:?}"), "0x4.4");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.hypot_prec_assign_ref(&BigExponentFloat::from(E), 20),
    ///     Greater
    /// );
    /// assert_eq!(format!("{x:?}"), "0x4.2784");
    /// ```
    #[inline]
    pub fn hypot_prec_assign_ref(&mut self, other: &Self, prec: u64) -> Ordering {
        let x = core::mem::replace(self, Self::ZERO);
        let (result, o) = hypot_helper(Cow::Owned(x), Cow::Borrowed(other), prec, Nearest);
        *self = result;
        o
    }
}
