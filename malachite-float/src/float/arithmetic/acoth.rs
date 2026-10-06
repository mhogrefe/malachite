// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::InnerFloat::{Finite, Infinity, NaN, Zero};
use crate::float::arithmetic::acsch::{RECIPROCAL_SAFE_EXPONENT, via_exact_reciprocal};
use crate::float::arithmetic::asinh::round_with_error;
use crate::{Float, emulate_float_to_float_fn, emulate_rational_to_float_fn};
use core::cmp::Ordering::{self, *};
use malachite_base::fail_on_untested_path;
use malachite_base::num::arithmetic::traits::{
    Abs, Acoth, AcothAssign, CeilingLogBase2, IsPowerOf2, Ln, Ln1PlusX, Reciprocal,
};
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::{
    Infinity as InfinityTrait, NaN as NaNTrait, NegativeInfinity, NegativeZero, One, Two,
    Zero as ZeroTrait,
};
use malachite_base::num::conversion::traits::{ExactFrom, RoundingFrom};
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_nz::platform::Limb;
use malachite_q::Rational;

// Computes acoth(x) = atanh(1/x) = ln((x + 1)/(x - 1))/2 for a finite `Float` x with |x| > 1. MPFR
// has no acoth, so this is Malachite's own algorithm. The work is done on |x|, acoth being odd.
//
// Going through 1/x would be ill-conditioned near |x| = 1, where atanh has an infinite derivative,
// so instead acoth(|x|) = ln(1 + u)/2 with u = 2/d and d = |x| - 1, which is exact for |x| <= 2
// (Sterbenz) and is otherwise rounded down, so that it cannot overflow for an |x| near the largest
// finite `Float`, with a relative error below 2^(1-wp). Then u has a relative error below 3 *
// 2^-wp, which, since ln(1+u) >= u/(1+u), moves ln(1+u) by less than 3 ulps; its own rounding adds
// 1/2 ulp, and the halving is exact: under 2^2 ulps.
//
// When |x| is a power of 2, 1/x is exact, and acoth(x) = atanh(1/x) is taken from `atanh` by
// `via_exact_reciprocal`: acoth(2^k) lies just above 2^-k, an exactly representable value.
//
// If 2/d would overflow, which needs an x with a precision above 2^30, ln(|x| + 1) - ln(d) is used
// instead.
fn acoth_prec_round_normal_ref(x: &Float, prec: u64, rm: RoundingMode) -> (Float, Ordering) {
    assert_ne!(rm, Exact, "Inexact acoth");
    if let Some(result) = via_exact_reciprocal(x, prec, rm, Float::atanh_prec_round) {
        return result;
    }
    let negative = *x < 0u32;
    let x_abs = x.abs();
    let near_one = x_abs.get_exponent().unwrap() == 1; // 1 < |x| < 2
    // d = |x| - 1, exact for |x| < 2
    let exact_d = near_one.then(|| {
        let (d, o) = x_abs.sub_prec_ref_val(Float::ONE, x_abs.get_prec().unwrap());
        assert_eq!(o, Equal);
        d
    });
    let mut working_prec = prec + prec.ceiling_log_base_2() + 10;
    let mut increment = Limb::WIDTH;
    loop {
        let d_rounded;
        let d = if let Some(d) = &exact_d {
            d
        } else {
            d_rounded = x_abs
                .sub_prec_round_ref_val(Float::ONE, working_prec, Floor)
                .0;
            &d_rounded
        };
        let t = if i64::from(d.get_exponent().unwrap()) < RECIPROCAL_SAFE_EXPONENT {
            fail_on_untested_path("acoth_prec_round_normal_ref, 2/(|x| - 1) may overflow");
            // ln(|x| + 1) - ln(d)
            x_abs.add_prec_ref_val(Float::ONE, working_prec).0.ln() - d.ln_prec_ref(working_prec).0
        } else {
            // ln(1 + 2/d)
            Float::TWO.div_prec_val_ref(d, working_prec).0.ln_1_plus_x()
        } >> 1u32;
        if let Some(result) =
            round_with_error(if negative { -t } else { t }, working_prec, 2, prec, rm)
        {
            return result;
        }
        working_prec += increment;
        increment = working_prec >> 1;
    }
}

impl Float {
    /// Computes $\operatorname{acoth} x$, the inverse hyperbolic cotangent of a [`Float`], rounding
    /// the result to the specified precision and with the specified rounding mode. The [`Float`] is
    /// taken by value. An [`Ordering`] is also returned, indicating whether the rounded inverse
    /// hyperbolic cotangent is less than, equal to, or greater than the exact inverse hyperbolic
    /// cotangent. Although `NaN`s are not comparable to any [`Float`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \operatorname{acoth} x+\varepsilon.
    /// $$
    /// - If $\operatorname{acoth} x$ is infinite, zero, or NaN, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $\operatorname{acoth} x$ is finite and nonzero, and $m$ is not `Nearest`, then
    ///   $|\varepsilon| < 2^{\lfloor\log_2 |\operatorname{acoth} x|\rfloor-p+1}$.
    /// - If $\operatorname{acoth} x$ is finite and nonzero, and $m$ is `Nearest`, then
    ///   $|\varepsilon| \leq 2^{\lfloor\log_2 |\operatorname{acoth} x|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=\text{NaN}$
    /// - $f(\infty,p,m)=0.0$
    /// - $f(-\infty,p,m)=-0.0$
    /// - $f(\pm1,p,m)=\pm\infty$
    /// - $f(x,p,m)=\text{NaN}$ if $|x|<1$, including $\pm0.0$
    ///
    /// The result never overflows or underflows: for a finite $x$ with $|x|>1$ and precision $q$,
    /// $1/|x| < |\operatorname{acoth} x| \leq \frac{1}{2}\ln(1 + 2^q)$, and every finite [`Float`]
    /// is below $2^{2^{30}-1}$.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::acoth_prec`] instead. If you
    /// know that your target precision is the precision of the input, consider using
    /// [`Float::acoth_round`] instead. If both of these things are true, consider using
    /// [`Float::acoth`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec,
    /// self.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` and `self` is finite, nonzero, and less than 1 in absolute value,
    /// since the inverse hyperbolic cotangent of such a [`Float`] is never exactly representable,
    /// or if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = (Float::one_prec(100) << 1u32).acoth_prec_round(5, Floor);
    /// assert_eq!(c.to_string(), "0.531");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = (Float::one_prec(100) << 1u32).acoth_prec_round(5, Ceiling);
    /// assert_eq!(c.to_string(), "0.562");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = (Float::one_prec(100) << 1u32).acoth_prec_round(5, Nearest);
    /// assert_eq!(c.to_string(), "0.562");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = (Float::one_prec(100) << 1u32).acoth_prec_round(20, Floor);
    /// assert_eq!(c.to_string(), "0.54930592");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = (Float::one_prec(100) << 1u32).acoth_prec_round(20, Ceiling);
    /// assert_eq!(c.to_string(), "0.54930687");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = (Float::one_prec(100) << 1u32).acoth_prec_round(20, Nearest);
    /// assert_eq!(c.to_string(), "0.54930592");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn acoth_prec_round(self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        self.acoth_prec_round_ref(prec, rm)
    }

    /// Computes $\operatorname{acoth} x$, the inverse hyperbolic cotangent of a [`Float`], rounding
    /// the result to the specified precision and with the specified rounding mode. The [`Float`] is
    /// taken by reference. An [`Ordering`] is also returned, indicating whether the rounded inverse
    /// hyperbolic cotangent is less than, equal to, or greater than the exact inverse hyperbolic
    /// cotangent. Although `NaN`s are not comparable to any [`Float`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \operatorname{acoth} x+\varepsilon.
    /// $$
    /// - If $\operatorname{acoth} x$ is infinite, zero, or NaN, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $\operatorname{acoth} x$ is finite and nonzero, and $m$ is not `Nearest`, then
    ///   $|\varepsilon| < 2^{\lfloor\log_2 |\operatorname{acoth} x|\rfloor-p+1}$.
    /// - If $\operatorname{acoth} x$ is finite and nonzero, and $m$ is `Nearest`, then
    ///   $|\varepsilon| \leq 2^{\lfloor\log_2 |\operatorname{acoth} x|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=\text{NaN}$
    /// - $f(\infty,p,m)=0.0$
    /// - $f(-\infty,p,m)=-0.0$
    /// - $f(\pm1,p,m)=\pm\infty$
    /// - $f(x,p,m)=\text{NaN}$ if $|x|<1$, including $\pm0.0$
    ///
    /// The result never overflows or underflows: for a finite $x$ with $|x|>1$ and precision $q$,
    /// $1/|x| < |\operatorname{acoth} x| \leq \frac{1}{2}\ln(1 + 2^q)$, and every finite [`Float`]
    /// is below $2^{2^{30}-1}$.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::acoth_prec_ref`] instead. If
    /// you know that your target precision is the precision of the input, consider using
    /// [`Float::acoth_round_ref`] instead. If both of these things are true, consider using
    /// `(&Float).acoth()` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec,
    /// self.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` and `self` is finite, nonzero, and less than 1 in absolute value,
    /// since the inverse hyperbolic cotangent of such a [`Float`] is never exactly representable,
    /// or if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = (Float::one_prec(100) << 1u32).acoth_prec_round_ref(5, Floor);
    /// assert_eq!(c.to_string(), "0.531");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = (Float::one_prec(100) << 1u32).acoth_prec_round_ref(5, Ceiling);
    /// assert_eq!(c.to_string(), "0.562");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = (Float::one_prec(100) << 1u32).acoth_prec_round_ref(5, Nearest);
    /// assert_eq!(c.to_string(), "0.562");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = (Float::one_prec(100) << 1u32).acoth_prec_round_ref(20, Floor);
    /// assert_eq!(c.to_string(), "0.54930592");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = (Float::one_prec(100) << 1u32).acoth_prec_round_ref(20, Ceiling);
    /// assert_eq!(c.to_string(), "0.54930687");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = (Float::one_prec(100) << 1u32).acoth_prec_round_ref(20, Nearest);
    /// assert_eq!(c.to_string(), "0.54930592");
    /// assert_eq!(o, Less);
    /// ```
    pub fn acoth_prec_round_ref(&self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        assert_ne!(prec, 0);
        match &self.0 {
            // acoth(NaN) = NaN, and acoth(±0) = NaN since |0| < 1
            NaN | Zero { .. } => (Self::NAN, Equal),
            // acoth(±inf) = ±0
            Infinity { sign } => (
                if *sign {
                    Self::ZERO
                } else {
                    Self::NEGATIVE_ZERO
                },
                Equal,
            ),
            // acoth(x) = NaN for |x| < 1, and acoth(±1) = ±inf
            Finite {
                sign,
                exponent,
                significand,
                ..
            } => {
                if *exponent <= 0 {
                    (Self::NAN, Equal)
                } else if *exponent == 1 && significand.is_power_of_2() {
                    (
                        if *sign {
                            Self::INFINITY
                        } else {
                            Self::NEGATIVE_INFINITY
                        },
                        Equal,
                    )
                } else {
                    acoth_prec_round_normal_ref(self, prec, rm)
                }
            }
        }
    }

    /// Computes $\operatorname{acoth} x$, the inverse hyperbolic cotangent of a [`Float`], rounding
    /// the result to the nearest value of the specified precision. The [`Float`] is taken by value.
    /// An [`Ordering`] is also returned, indicating whether the rounded inverse hyperbolic
    /// cotangent is less than, equal to, or greater than the exact inverse hyperbolic cotangent.
    /// Although `NaN`s are not comparable to any [`Float`], whenever this function returns a `NaN`
    /// it also returns `Equal`.
    ///
    /// If the inverse hyperbolic cotangent is equidistant from two [`Float`]s with the specified
    /// precision, the [`Float`] with fewer 1s in its binary expansion is chosen. See
    /// [`RoundingMode`] for a description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = \operatorname{acoth} x+\varepsilon.
    /// $$
    /// - If $\operatorname{acoth} x$ is infinite, zero, or NaN, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $\operatorname{acoth} x$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2
    ///   |\operatorname{acoth} x|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p)=\text{NaN}$
    /// - $f(\infty,p)=0.0$
    /// - $f(-\infty,p)=-0.0$
    /// - $f(\pm1,p)=\pm\infty$
    /// - $f(x,p)=\text{NaN}$ if $|x|<1$, including $\pm0.0$
    ///
    /// The result never overflows or underflows: for a finite $x$ with $|x|>1$ and precision $q$,
    /// $1/|x| < |\operatorname{acoth} x| \leq \frac{1}{2}\ln(1 + 2^q)$, and every finite [`Float`]
    /// is below $2^{2^{30}-1}$.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::acoth_prec_round`] instead. If you know that your target precision is the precision
    /// of the input, consider using [`Float::acoth`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec,
    /// self.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = (Float::one_prec(100) << 1u32).acoth_prec(5);
    /// assert_eq!(c.to_string(), "0.562");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = (Float::one_prec(100) << 1u32).acoth_prec(20);
    /// assert_eq!(c.to_string(), "0.54930592");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn acoth_prec(self, prec: u64) -> (Self, Ordering) {
        self.acoth_prec_round(prec, Nearest)
    }

    /// Computes $\operatorname{acoth} x$, the inverse hyperbolic cotangent of a [`Float`], rounding
    /// the result to the nearest value of the specified precision. The [`Float`] is taken by
    /// reference. An [`Ordering`] is also returned, indicating whether the rounded inverse
    /// hyperbolic cotangent is less than, equal to, or greater than the exact inverse hyperbolic
    /// cotangent. Although `NaN`s are not comparable to any [`Float`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// If the inverse hyperbolic cotangent is equidistant from two [`Float`]s with the specified
    /// precision, the [`Float`] with fewer 1s in its binary expansion is chosen. See
    /// [`RoundingMode`] for a description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = \operatorname{acoth} x+\varepsilon.
    /// $$
    /// - If $\operatorname{acoth} x$ is infinite, zero, or NaN, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $\operatorname{acoth} x$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2
    ///   |\operatorname{acoth} x|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p)=\text{NaN}$
    /// - $f(\infty,p)=0.0$
    /// - $f(-\infty,p)=-0.0$
    /// - $f(\pm1,p)=\pm\infty$
    /// - $f(x,p)=\text{NaN}$ if $|x|<1$, including $\pm0.0$
    ///
    /// The result never overflows or underflows: for a finite $x$ with $|x|>1$ and precision $q$,
    /// $1/|x| < |\operatorname{acoth} x| \leq \frac{1}{2}\ln(1 + 2^q)$, and every finite [`Float`]
    /// is below $2^{2^{30}-1}$.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::acoth_prec_round_ref`] instead. If you know that your target precision is the
    /// precision of the input, consider using `(&Float).acoth()` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec,
    /// self.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = (Float::one_prec(100) << 1u32).acoth_prec_ref(5);
    /// assert_eq!(c.to_string(), "0.562");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = (Float::one_prec(100) << 1u32).acoth_prec_ref(20);
    /// assert_eq!(c.to_string(), "0.54930592");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn acoth_prec_ref(&self, prec: u64) -> (Self, Ordering) {
        self.acoth_prec_round_ref(prec, Nearest)
    }

    /// Computes $\operatorname{acoth} x$, the inverse hyperbolic cotangent of a [`Float`], rounding
    /// the result with the specified rounding mode. The [`Float`] is taken by value. An
    /// [`Ordering`] is also returned, indicating whether the rounded inverse hyperbolic cotangent
    /// is less than, equal to, or greater than the exact inverse hyperbolic cotangent. Although
    /// `NaN`s are not comparable to any [`Float`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// The precision of the output is the precision of the input. See [`RoundingMode`] for a
    /// description of the possible rounding modes.
    ///
    /// $$
    /// f(x,m) = \operatorname{acoth} x+\varepsilon.
    /// $$
    /// - If $\operatorname{acoth} x$ is infinite, zero, or NaN, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $\operatorname{acoth} x$ is finite and nonzero, and $m$ is not `Nearest`, then
    ///   $|\varepsilon| < 2^{\lfloor\log_2 |\operatorname{acoth} x|\rfloor-p+1}$, where $p$ is the
    ///   precision of the input.
    /// - If $\operatorname{acoth} x$ is finite and nonzero, and $m$ is `Nearest`, then
    ///   $|\varepsilon| \leq 2^{\lfloor\log_2 |\operatorname{acoth} x|\rfloor-p}$, where $p$ is the
    ///   precision of the input.
    ///
    /// If the output has a precision, it is the precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN},m)=\text{NaN}$
    /// - $f(\infty,m)=0.0$
    /// - $f(-\infty,m)=-0.0$
    /// - $f(\pm1,m)=\pm\infty$
    /// - $f(x,m)=\text{NaN}$ if $|x|<1$, including $\pm0.0$
    ///
    /// The result never overflows or underflows: for a finite $x$ with $|x|>1$ and precision $q$,
    /// $1/|x| < |\operatorname{acoth} x| \leq \frac{1}{2}\ln(1 + 2^q)$, and every finite [`Float`]
    /// is below $2^{2^{30}-1}$.
    ///
    /// If you want to specify an output precision, consider using [`Float::acoth_prec_round`]
    /// instead. If you know you'll be using the `Nearest` rounding mode, consider using
    /// [`Float::acoth`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` and `self` is finite, nonzero, and less than 1 in absolute value,
    /// since the inverse hyperbolic cotangent of such a [`Float`] is never exactly representable.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = (Float::one_prec(100) << 1u32).acoth_round(Floor);
    /// assert_eq!(c.to_string(), "0.54930614433405484569762261846113");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = (Float::one_prec(100) << 1u32).acoth_round(Ceiling);
    /// assert_eq!(c.to_string(), "0.54930614433405484569762261846192");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = (Float::one_prec(100) << 1u32).acoth_round(Nearest);
    /// assert_eq!(c.to_string(), "0.54930614433405484569762261846113");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn acoth_round(self, rm: RoundingMode) -> (Self, Ordering) {
        let prec = self.significant_bits();
        self.acoth_prec_round(prec, rm)
    }

    /// Computes $\operatorname{acoth} x$, the inverse hyperbolic cotangent of a [`Float`], rounding
    /// the result with the specified rounding mode. The [`Float`] is taken by reference. An
    /// [`Ordering`] is also returned, indicating whether the rounded inverse hyperbolic cotangent
    /// is less than, equal to, or greater than the exact inverse hyperbolic cotangent. Although
    /// `NaN`s are not comparable to any [`Float`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// The precision of the output is the precision of the input. See [`RoundingMode`] for a
    /// description of the possible rounding modes.
    ///
    /// $$
    /// f(x,m) = \operatorname{acoth} x+\varepsilon.
    /// $$
    /// - If $\operatorname{acoth} x$ is infinite, zero, or NaN, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $\operatorname{acoth} x$ is finite and nonzero, and $m$ is not `Nearest`, then
    ///   $|\varepsilon| < 2^{\lfloor\log_2 |\operatorname{acoth} x|\rfloor-p+1}$, where $p$ is the
    ///   precision of the input.
    /// - If $\operatorname{acoth} x$ is finite and nonzero, and $m$ is `Nearest`, then
    ///   $|\varepsilon| \leq 2^{\lfloor\log_2 |\operatorname{acoth} x|\rfloor-p}$, where $p$ is the
    ///   precision of the input.
    ///
    /// If the output has a precision, it is the precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN},m)=\text{NaN}$
    /// - $f(\infty,m)=0.0$
    /// - $f(-\infty,m)=-0.0$
    /// - $f(\pm1,m)=\pm\infty$
    /// - $f(x,m)=\text{NaN}$ if $|x|<1$, including $\pm0.0$
    ///
    /// The result never overflows or underflows: for a finite $x$ with $|x|>1$ and precision $q$,
    /// $1/|x| < |\operatorname{acoth} x| \leq \frac{1}{2}\ln(1 + 2^q)$, and every finite [`Float`]
    /// is below $2^{2^{30}-1}$.
    ///
    /// If you want to specify an output precision, consider using [`Float::acoth_prec_round_ref`]
    /// instead. If you know you'll be using the `Nearest` rounding mode, consider using
    /// `(&Float).acoth()` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` and `self` is finite, nonzero, and less than 1 in absolute value,
    /// since the inverse hyperbolic cotangent of such a [`Float`] is never exactly representable.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = (Float::one_prec(100) << 1u32).acoth_round_ref(Floor);
    /// assert_eq!(c.to_string(), "0.54930614433405484569762261846113");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = (Float::one_prec(100) << 1u32).acoth_round_ref(Ceiling);
    /// assert_eq!(c.to_string(), "0.54930614433405484569762261846192");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = (Float::one_prec(100) << 1u32).acoth_round_ref(Nearest);
    /// assert_eq!(c.to_string(), "0.54930614433405484569762261846113");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn acoth_round_ref(&self, rm: RoundingMode) -> (Self, Ordering) {
        self.acoth_prec_round_ref(self.significant_bits(), rm)
    }

    /// Computes $\operatorname{acoth} x$, the inverse hyperbolic cotangent of a [`Float`], rounding
    /// the result to the specified precision and with the specified rounding mode. The [`Float`] is
    /// replaced by the result, and an [`Ordering`] is returned, indicating whether the rounded
    /// inverse hyperbolic cotangent is less than, equal to, or greater than the exact inverse
    /// hyperbolic cotangent. Although `NaN`s are not comparable to any [`Float`], whenever this
    /// function sets a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets \operatorname{acoth} x+\varepsilon.
    /// $$
    /// - If $\operatorname{acoth} x$ is infinite, zero, or NaN, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $\operatorname{acoth} x$ is finite and nonzero, and $m$ is not `Nearest`, then
    ///   $|\varepsilon| < 2^{\lfloor\log_2 |\operatorname{acoth} x|\rfloor-p+1}$.
    /// - If $\operatorname{acoth} x$ is finite and nonzero, and $m$ is `Nearest`, then
    ///   $|\varepsilon| \leq 2^{\lfloor\log_2 |\operatorname{acoth} x|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// See the [`Float::acoth_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::acoth_prec_assign`] instead.
    /// If you know that your target precision is the precision of the input, consider using
    /// [`Float::acoth_round_assign`] instead. If both of these things are true, consider using
    /// [`Float::acoth_assign`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec,
    /// self.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` and `self` is finite, nonzero, and less than 1 in absolute value,
    /// since the inverse hyperbolic cotangent of such a [`Float`] is never exactly representable,
    /// or if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::one_prec(100) << 1u32;
    /// assert_eq!(x.acoth_prec_round_assign(5, Floor), Less);
    /// assert_eq!(x.to_string(), "0.531");
    ///
    /// let mut x = Float::one_prec(100) << 1u32;
    /// assert_eq!(x.acoth_prec_round_assign(5, Ceiling), Greater);
    /// assert_eq!(x.to_string(), "0.562");
    ///
    /// let mut x = Float::one_prec(100) << 1u32;
    /// assert_eq!(x.acoth_prec_round_assign(5, Nearest), Greater);
    /// assert_eq!(x.to_string(), "0.562");
    ///
    /// let mut x = Float::one_prec(100) << 1u32;
    /// assert_eq!(x.acoth_prec_round_assign(20, Floor), Less);
    /// assert_eq!(x.to_string(), "0.54930592");
    ///
    /// let mut x = Float::one_prec(100) << 1u32;
    /// assert_eq!(x.acoth_prec_round_assign(20, Ceiling), Greater);
    /// assert_eq!(x.to_string(), "0.54930687");
    ///
    /// let mut x = Float::one_prec(100) << 1u32;
    /// assert_eq!(x.acoth_prec_round_assign(20, Nearest), Less);
    /// assert_eq!(x.to_string(), "0.54930592");
    /// ```
    #[inline]
    pub fn acoth_prec_round_assign(&mut self, prec: u64, rm: RoundingMode) -> Ordering {
        let o;
        (*self, o) = self.acoth_prec_round_ref(prec, rm);
        o
    }

    /// Computes $\operatorname{acoth} x$, the inverse hyperbolic cotangent of a [`Float`], rounding
    /// the result to the nearest value of the specified precision. The [`Float`] is replaced by the
    /// result, and an [`Ordering`] is returned, indicating whether the rounded inverse hyperbolic
    /// cotangent is less than, equal to, or greater than the exact inverse hyperbolic cotangent.
    /// Although `NaN`s are not comparable to any [`Float`], whenever this function sets a `NaN` it
    /// also returns `Equal`.
    ///
    /// If the inverse hyperbolic cotangent is equidistant from two [`Float`]s with the specified
    /// precision, the [`Float`] with fewer 1s in its binary expansion is chosen. See
    /// [`RoundingMode`] for a description of the `Nearest` rounding mode.
    ///
    /// $$
    /// x \gets \operatorname{acoth} x+\varepsilon.
    /// $$
    /// - If $\operatorname{acoth} x$ is infinite, zero, or NaN, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $\operatorname{acoth} x$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2
    ///   |\operatorname{acoth} x|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// See the [`Float::acoth_prec`] documentation for information on special cases, overflow, and
    /// underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::acoth_prec_round_assign`] instead. If you know that your target precision is the
    /// precision of the input, consider using [`Float::acoth_assign`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec,
    /// self.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::one_prec(100) << 1u32;
    /// assert_eq!(x.acoth_prec_assign(5), Greater);
    /// assert_eq!(x.to_string(), "0.562");
    ///
    /// let mut x = Float::one_prec(100) << 1u32;
    /// assert_eq!(x.acoth_prec_assign(20), Less);
    /// assert_eq!(x.to_string(), "0.54930592");
    /// ```
    #[inline]
    pub fn acoth_prec_assign(&mut self, prec: u64) -> Ordering {
        self.acoth_prec_round_assign(prec, Nearest)
    }

    /// Computes $\operatorname{acoth} x$, the inverse hyperbolic cotangent of a [`Float`], rounding
    /// the result with the specified rounding mode. The [`Float`] is replaced by the result, and an
    /// [`Ordering`] is returned, indicating whether the rounded inverse hyperbolic cotangent is
    /// less than, equal to, or greater than the exact inverse hyperbolic cotangent. Although `NaN`s
    /// are not comparable to any [`Float`], whenever this function sets a `NaN` it also returns
    /// `Equal`.
    ///
    /// The precision of the output is the precision of the input. See [`RoundingMode`] for a
    /// description of the possible rounding modes.
    ///
    /// $$
    /// x \gets \operatorname{acoth} x+\varepsilon.
    /// $$
    /// - If $\operatorname{acoth} x$ is infinite, zero, or NaN, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $\operatorname{acoth} x$ is finite and nonzero, and $m$ is not `Nearest`, then
    ///   $|\varepsilon| < 2^{\lfloor\log_2 |\operatorname{acoth} x|\rfloor-p+1}$, where $p$ is the
    ///   precision of the input.
    /// - If $\operatorname{acoth} x$ is finite and nonzero, and $m$ is `Nearest`, then
    ///   $|\varepsilon| \leq 2^{\lfloor\log_2 |\operatorname{acoth} x|\rfloor-p}$, where $p$ is the
    ///   precision of the input.
    ///
    /// If the output has a precision, it is the precision of the input.
    ///
    /// See the [`Float::acoth_round`] documentation for information on special cases, overflow, and
    /// underflow.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::acoth_prec_round_assign`] instead. If you know you'll be using the `Nearest`
    /// rounding mode, consider using [`Float::acoth_assign`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` and `self` is finite, nonzero, and less than 1 in absolute value,
    /// since the inverse hyperbolic cotangent of such a [`Float`] is never exactly representable.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::one_prec(100) << 1u32;
    /// assert_eq!(x.acoth_round_assign(Floor), Less);
    /// assert_eq!(x.to_string(), "0.54930614433405484569762261846113");
    ///
    /// let mut x = Float::one_prec(100) << 1u32;
    /// assert_eq!(x.acoth_round_assign(Ceiling), Greater);
    /// assert_eq!(x.to_string(), "0.54930614433405484569762261846192");
    ///
    /// let mut x = Float::one_prec(100) << 1u32;
    /// assert_eq!(x.acoth_round_assign(Nearest), Less);
    /// assert_eq!(x.to_string(), "0.54930614433405484569762261846113");
    /// ```
    #[inline]
    pub fn acoth_round_assign(&mut self, rm: RoundingMode) -> Ordering {
        let prec = self.significant_bits();
        self.acoth_prec_round_assign(prec, rm)
    }
}

impl Float {
    /// Computes $\operatorname{acoth} x$, the inverse hyperbolic cotangent of a [`Rational`],
    /// rounding the result to the specified precision and with the specified rounding mode and
    /// returning the result as a [`Float`]. The [`Rational`] is taken by value. An [`Ordering`] is
    /// also returned, indicating whether the rounded inverse hyperbolic cotangent is less than,
    /// equal to, or greater than the exact inverse hyperbolic cotangent. Although `NaN`s are not
    /// comparable to any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \operatorname{acoth} x+\varepsilon.
    /// $$
    /// - If $\operatorname{acoth} x$ is infinite, zero, or NaN, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $\operatorname{acoth} x$ is finite and nonzero, and $m$ is not `Nearest`, then
    ///   $|\varepsilon| < 2^{\lfloor\log_2 |\operatorname{acoth} x|\rfloor-p+1}$.
    /// - If $\operatorname{acoth} x$ is finite and nonzero, and $m$ is `Nearest`, then
    ///   $|\varepsilon| \leq 2^{\lfloor\log_2 |\operatorname{acoth} x|\rfloor-p}$.
    ///
    /// These bounds do not apply when the result underflows; see below.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\pm1,p,m)=\pm\infty$
    /// - $f(x,p,m)=\text{NaN}$ if $|x|<1$
    ///
    /// Overflow and underflow:
    /// - The result never overflows: for $|x|>1$ with denominator $d$, $|x| - 1 \geq 1/d$, so
    ///   $|\operatorname{acoth} x| \leq \frac{1}{2}\ln(1 + 2d)$.
    /// - If $0<f(x,p,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,p,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,p,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,p,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,p,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,p,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,p,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,p,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is
    ///   returned instead.
    ///
    /// Underflow requires $|x| > 2^{2^{30}}$, since $|\operatorname{acoth} x| > 1/|x|$.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::acoth_rational_prec`]
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^2 \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `x.significant_bits()`: the logarithm is computed at a working precision of about $n$, and
    /// the input is handled with `Rational` arithmetic.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with the given precision (which is the case for every $x$ with $|x|>1$).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = Float::acoth_rational_prec_round(Rational::from(3u32), 5, Floor);
    /// assert_eq!(c.to_string(), "0.344");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::acoth_rational_prec_round(Rational::from(3u32), 5, Ceiling);
    /// assert_eq!(c.to_string(), "0.359");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = Float::acoth_rational_prec_round(Rational::from(3u32), 20, Floor);
    /// assert_eq!(c.to_string(), "0.34657335");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::acoth_rational_prec_round(Rational::from(3u32), 20, Ceiling);
    /// assert_eq!(c.to_string(), "0.34657383");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    #[allow(clippy::needless_pass_by_value)]
    pub fn acoth_rational_prec_round(x: Rational, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        Self::acoth_rational_prec_round_ref(&x, prec, rm)
    }

    /// Computes $\operatorname{acoth} x$, the inverse hyperbolic cotangent of a [`Rational`],
    /// rounding the result to the specified precision and with the specified rounding mode and
    /// returning the result as a [`Float`]. The [`Rational`] is taken by reference. An [`Ordering`]
    /// is also returned, indicating whether the rounded inverse hyperbolic cotangent is less than,
    /// equal to, or greater than the exact inverse hyperbolic cotangent. Although `NaN`s are not
    /// comparable to any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \operatorname{acoth} x+\varepsilon.
    /// $$
    /// - If $\operatorname{acoth} x$ is infinite, zero, or NaN, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $\operatorname{acoth} x$ is finite and nonzero, and $m$ is not `Nearest`, then
    ///   $|\varepsilon| < 2^{\lfloor\log_2 |\operatorname{acoth} x|\rfloor-p+1}$.
    /// - If $\operatorname{acoth} x$ is finite and nonzero, and $m$ is `Nearest`, then
    ///   $|\varepsilon| \leq 2^{\lfloor\log_2 |\operatorname{acoth} x|\rfloor-p}$.
    ///
    /// These bounds do not apply when the result underflows; see below.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\pm1,p,m)=\pm\infty$
    /// - $f(x,p,m)=\text{NaN}$ if $|x|<1$
    ///
    /// Overflow and underflow:
    /// - The result never overflows: for $|x|>1$ with denominator $d$, $|x| - 1 \geq 1/d$, so
    ///   $|\operatorname{acoth} x| \leq \frac{1}{2}\ln(1 + 2d)$.
    /// - If $0<f(x,p,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,p,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,p,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,p,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,p,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,p,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,p,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,p,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is
    ///   returned instead.
    ///
    /// Underflow requires $|x| > 2^{2^{30}}$, since $|\operatorname{acoth} x| > 1/|x|$.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::acoth_rational_prec_ref`]
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^2 \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `x.significant_bits()`: the logarithm is computed at a working precision of about $n$, and
    /// the input is handled with `Rational` arithmetic.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with the given precision (which is the case for every $x$ with $|x|>1$).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = Float::acoth_rational_prec_round_ref(&Rational::from(3u32), 5, Floor);
    /// assert_eq!(c.to_string(), "0.344");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::acoth_rational_prec_round_ref(&Rational::from(3u32), 5, Ceiling);
    /// assert_eq!(c.to_string(), "0.359");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = Float::acoth_rational_prec_round_ref(&Rational::from(3u32), 20, Floor);
    /// assert_eq!(c.to_string(), "0.34657335");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::acoth_rational_prec_round_ref(&Rational::from(3u32), 20, Ceiling);
    /// assert_eq!(c.to_string(), "0.34657383");
    /// assert_eq!(o, Greater);
    /// ```
    pub fn acoth_rational_prec_round_ref(
        x: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        assert_ne!(prec, 0);
        if *x == 0u32 {
            // acoth(0) = NaN, since |0| < 1
            return (Self::NAN, Equal);
        }
        // acoth(x) = atanh(1/x), and the reciprocal of a `Rational` is exact
        Self::atanh_rational_prec_round(x.reciprocal(), prec, rm)
    }

    /// Computes $\operatorname{acoth} x$, the inverse hyperbolic cotangent of a [`Rational`],
    /// rounding the result to the nearest value of the specified precision and returning the result
    /// as a [`Float`]. The [`Rational`] is taken by value. An [`Ordering`] is also returned,
    /// indicating whether the rounded inverse hyperbolic cotangent is less than, equal to, or
    /// greater than the exact inverse hyperbolic cotangent. Although `NaN`s are not comparable to
    /// any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// If the inverse hyperbolic cotangent is equidistant from two [`Float`]s with the specified
    /// precision, the [`Float`] with fewer 1s in its binary expansion is chosen. See
    /// [`RoundingMode`] for a description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = \operatorname{acoth} x+\varepsilon,
    /// $$
    /// where, if $\operatorname{acoth} x$ is finite and nonzero, $|\varepsilon| \leq
    /// 2^{\lfloor\log_2 |\operatorname{acoth} x|\rfloor-p}$ (unless the result underflows; see
    /// below).
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\pm1,p)=\pm\infty$
    /// - $f(x,p)=\text{NaN}$ if $|x|<1$
    ///
    /// Overflow and underflow:
    /// - The result never overflows: for $|x|>1$ with denominator $d$, $|x| - 1 \geq 1/d$, so
    ///   $|\operatorname{acoth} x| \leq \frac{1}{2}\ln(1 + 2d)$.
    /// - If $0<f(x,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// Underflow requires $|x| > 2^{2^{30}}$, since $|\operatorname{acoth} x| > 1/|x|$.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::acoth_rational_prec_round`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^2 \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `x.significant_bits()`: the logarithm is computed at a working precision of about $n$, and
    /// the input is handled with `Rational` arithmetic.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{One, OneHalf};
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = Float::acoth_rational_prec(Rational::from(3u32), 5);
    /// assert_eq!(c.to_string(), "0.344");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::acoth_rational_prec(Rational::from(3u32), 20);
    /// assert_eq!(c.to_string(), "0.34657335");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::acoth_rational_prec(Rational::ONE, 10);
    /// assert_eq!(c.to_string(), "Infinity");
    /// assert_eq!(o, Equal);
    ///
    /// let (c, o) = Float::acoth_rational_prec(Rational::ONE_HALF, 10);
    /// assert!(c.is_nan());
    /// assert_eq!(o, Equal);
    /// ```
    #[inline]
    #[allow(clippy::needless_pass_by_value)]
    pub fn acoth_rational_prec(x: Rational, prec: u64) -> (Self, Ordering) {
        Self::acoth_rational_prec_round_ref(&x, prec, Nearest)
    }

    /// Computes $\operatorname{acoth} x$, the inverse hyperbolic cotangent of a [`Rational`],
    /// rounding the result to the nearest value of the specified precision and returning the result
    /// as a [`Float`]. The [`Rational`] is taken by reference. An [`Ordering`] is also returned,
    /// indicating whether the rounded inverse hyperbolic cotangent is less than, equal to, or
    /// greater than the exact inverse hyperbolic cotangent. Although `NaN`s are not comparable to
    /// any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// If the inverse hyperbolic cotangent is equidistant from two [`Float`]s with the specified
    /// precision, the [`Float`] with fewer 1s in its binary expansion is chosen. See
    /// [`RoundingMode`] for a description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = \operatorname{acoth} x+\varepsilon,
    /// $$
    /// where, if $\operatorname{acoth} x$ is finite and nonzero, $|\varepsilon| \leq
    /// 2^{\lfloor\log_2 |\operatorname{acoth} x|\rfloor-p}$ (unless the result underflows; see
    /// below).
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\pm1,p)=\pm\infty$
    /// - $f(x,p)=\text{NaN}$ if $|x|<1$
    ///
    /// Overflow and underflow:
    /// - The result never overflows: for $|x|>1$ with denominator $d$, $|x| - 1 \geq 1/d$, so
    ///   $|\operatorname{acoth} x| \leq \frac{1}{2}\ln(1 + 2d)$.
    /// - If $0<f(x,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// Underflow requires $|x| > 2^{2^{30}}$, since $|\operatorname{acoth} x| > 1/|x|$.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::acoth_rational_prec_round_ref`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^2 \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `x.significant_bits()`: the logarithm is computed at a working precision of about $n$, and
    /// the input is handled with `Rational` arithmetic.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{One, OneHalf};
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = Float::acoth_rational_prec_ref(&Rational::from(3u32), 5);
    /// assert_eq!(c.to_string(), "0.344");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::acoth_rational_prec_ref(&Rational::from(3u32), 20);
    /// assert_eq!(c.to_string(), "0.34657335");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::acoth_rational_prec_ref(&Rational::ONE, 10);
    /// assert_eq!(c.to_string(), "Infinity");
    /// assert_eq!(o, Equal);
    ///
    /// let (c, o) = Float::acoth_rational_prec_ref(&Rational::ONE_HALF, 10);
    /// assert!(c.is_nan());
    /// assert_eq!(o, Equal);
    /// ```
    #[inline]
    pub fn acoth_rational_prec_ref(x: &Rational, prec: u64) -> (Self, Ordering) {
        Self::acoth_rational_prec_round_ref(x, prec, Nearest)
    }
}

impl Acoth for Float {
    type Output = Self;

    /// Computes $\operatorname{acoth} x$, the inverse hyperbolic cotangent of a [`Float`], taking
    /// it by value.
    ///
    /// If the output has a precision, it is the precision of the input. If the inverse hyperbolic
    /// cotangent is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x) = \operatorname{acoth} x+\varepsilon.
    /// $$
    /// - If $\operatorname{acoth} x$ is infinite, zero, or NaN, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $\operatorname{acoth} x$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2
    ///   |\operatorname{acoth} x|\rfloor-p}$, where $p$ is the precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN})=\text{NaN}$
    /// - $f(\infty)=0.0$
    /// - $f(-\infty)=-0.0$
    /// - $f(\pm1)=\pm\infty$
    /// - $f(x)=\text{NaN}$ if $|x|<1$, including $\pm0.0$
    ///
    /// See the [`Float::acoth_round`] documentation for information on overflow and underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::acoth_round`] instead. If you want to specify the output precision, consider using
    /// [`Float::acoth_prec`]. If you want both of these things, consider using
    /// [`Float::acoth_prec_round`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Acoth;
    /// use malachite_base::num::basic::traits::*;
    /// use malachite_float::Float;
    ///
    /// assert!(Float::NAN.acoth().is_nan());
    /// assert_eq!(Float::INFINITY.acoth().to_string(), "0.0");
    /// assert_eq!(Float::NEGATIVE_INFINITY.acoth().to_string(), "-0.0");
    /// assert!(Float::ZERO.acoth().is_nan());
    /// assert_eq!(Float::ONE.acoth().to_string(), "Infinity");
    /// assert_eq!(Float::NEGATIVE_ONE.acoth().to_string(), "-Infinity");
    /// assert_eq!(
    ///     (Float::one_prec(100) << 1u32).acoth().to_string(),
    ///     "0.54930614433405484569762261846113"
    /// );
    /// assert_eq!(
    ///     (Float::from_unsigned_prec(3u32, 100).0).acoth().to_string(),
    ///     "0.34657359027997265470861606072899"
    /// );
    /// assert_eq!(
    ///     (-(Float::one_prec(100) << 1u32)).acoth().to_string(),
    ///     "-0.54930614433405484569762261846113"
    /// );
    /// ```
    #[inline]
    fn acoth(self) -> Self {
        let prec = self.significant_bits();
        self.acoth_prec_round(prec, Nearest).0
    }
}

impl Acoth for &Float {
    type Output = Float;

    /// Computes $\operatorname{acoth} x$, the inverse hyperbolic cotangent of a [`Float`], taking
    /// it by reference.
    ///
    /// If the output has a precision, it is the precision of the input. If the inverse hyperbolic
    /// cotangent is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x) = \operatorname{acoth} x+\varepsilon.
    /// $$
    /// - If $\operatorname{acoth} x$ is infinite, zero, or NaN, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $\operatorname{acoth} x$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2
    ///   |\operatorname{acoth} x|\rfloor-p}$, where $p$ is the precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN})=\text{NaN}$
    /// - $f(\infty)=0.0$
    /// - $f(-\infty)=-0.0$
    /// - $f(\pm1)=\pm\infty$
    /// - $f(x)=\text{NaN}$ if $|x|<1$, including $\pm0.0$
    ///
    /// See the [`Float::acoth_round`] documentation for information on overflow and underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::acoth_round_ref`] instead. If you want to specify the output precision, consider
    /// using [`Float::acoth_prec_ref`]. If you want both of these things, consider using
    /// [`Float::acoth_prec_round_ref`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Acoth;
    /// use malachite_base::num::basic::traits::*;
    /// use malachite_float::Float;
    ///
    /// assert!((&Float::NAN).acoth().is_nan());
    /// assert_eq!((&Float::INFINITY).acoth().to_string(), "0.0");
    /// assert_eq!((&Float::NEGATIVE_INFINITY).acoth().to_string(), "-0.0");
    /// assert!((&Float::ZERO).acoth().is_nan());
    /// assert_eq!((&Float::ONE).acoth().to_string(), "Infinity");
    /// assert_eq!((&Float::NEGATIVE_ONE).acoth().to_string(), "-Infinity");
    /// assert_eq!(
    ///     (&(Float::one_prec(100) << 1u32)).acoth().to_string(),
    ///     "0.54930614433405484569762261846113"
    /// );
    /// assert_eq!(
    ///     (&Float::from_unsigned_prec(3u32, 100).0)
    ///         .acoth()
    ///         .to_string(),
    ///     "0.34657359027997265470861606072899"
    /// );
    /// assert_eq!(
    ///     (&(-(Float::one_prec(100) << 1u32))).acoth().to_string(),
    ///     "-0.54930614433405484569762261846113"
    /// );
    /// ```
    #[inline]
    fn acoth(self) -> Float {
        self.acoth_prec_round_ref(self.significant_bits(), Nearest)
            .0
    }
}

impl AcothAssign for Float {
    /// Computes $\operatorname{acoth} x$, the inverse hyperbolic cotangent of a [`Float`], in
    /// place.
    ///
    /// If the output has a precision, it is the precision of the input. If the inverse hyperbolic
    /// cotangent is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// x \gets \operatorname{acoth} x+\varepsilon.
    /// $$
    /// - If $\operatorname{acoth} x$ is infinite, zero, or NaN, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $\operatorname{acoth} x$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2
    ///   |\operatorname{acoth} x|\rfloor-p}$, where $p$ is the precision of the input.
    ///
    /// See the [`Float::acoth`] documentation for information on special cases, overflow, and
    /// underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::acoth_round_assign`] instead. If you want to specify the output precision, consider
    /// using [`Float::acoth_prec_assign`]. If you want both of these things, consider using
    /// [`Float::acoth_prec_round_assign`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::AcothAssign;
    /// use malachite_base::num::basic::traits::*;
    /// use malachite_float::Float;
    ///
    /// let mut x = Float::NAN;
    /// x.acoth_assign();
    /// assert!(x.is_nan());
    ///
    /// let mut x = Float::INFINITY;
    /// x.acoth_assign();
    /// assert_eq!(x.to_string(), "0.0");
    ///
    /// let mut x = Float::ZERO;
    /// x.acoth_assign();
    /// assert!(x.is_nan());
    ///
    /// let mut x = Float::ONE;
    /// x.acoth_assign();
    /// assert_eq!(x.to_string(), "Infinity");
    ///
    /// let mut x = Float::one_prec(100) << 1u32;
    /// x.acoth_assign();
    /// assert_eq!(x.to_string(), "0.54930614433405484569762261846113");
    ///
    /// let mut x = Float::from_unsigned_prec(3u32, 100).0;
    /// x.acoth_assign();
    /// assert_eq!(x.to_string(), "0.34657359027997265470861606072899");
    /// ```
    #[inline]
    fn acoth_assign(&mut self) {
        let prec = self.significant_bits();
        self.acoth_prec_round_assign(prec, Nearest);
    }
}

/// Computes $\operatorname{acoth} x$, the inverse hyperbolic cotangent of a primitive float. Using
/// this function is more accurate than using the default `acoth` function or the one provided by
/// `libm`.
///
/// $$
/// f(x) = \operatorname{acoth} x+\varepsilon.
/// $$
/// - If $\operatorname{acoth} x$ is infinite, zero, or NaN, $\varepsilon$ may be ignored or assumed
///   to be 0.
/// - If $\operatorname{acoth} x$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2
///   |\operatorname{acoth} x|\rfloor-p}$, where $p$ is the precision of the output (24 if `T` is a
///   [`f32`] and 53 if `T` is a [`f64`]).
///
/// Special cases:
/// - $f(\text{NaN})=\text{NaN}$
/// - $f(\infty)=0.0$
/// - $f(-\infty)=-0.0$
/// - $f(\pm1)=\pm\infty$
/// - $f(x)=\text{NaN}$ if $|x|<1$, including $\pm0.0$
///
/// Overflow is not possible. The result is subnormal only when $x$ is, and then it is $x$ itself,
/// since $|\operatorname{acoth} x - x| < |x|^3/2$.
///
/// # Worst-case complexity
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use malachite_base::num::basic::traits::NegativeInfinity;
/// use malachite_base::num::float::NiceFloat;
/// use malachite_float::float::arithmetic::acoth::primitive_float_acoth;
///
/// assert!(primitive_float_acoth(f32::NAN).is_nan());
/// assert_eq!(
///     NiceFloat(primitive_float_acoth(f32::INFINITY)),
///     NiceFloat(0.0)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_acoth(f32::NEGATIVE_INFINITY)),
///     NiceFloat(-0.0)
/// );
/// assert!(primitive_float_acoth(0.0f32).is_nan());
/// assert_eq!(
///     NiceFloat(primitive_float_acoth(1.0f32)),
///     NiceFloat(f32::INFINITY)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_acoth(-1.0f32)),
///     NiceFloat(f32::NEGATIVE_INFINITY)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_acoth(2.0f32)),
///     NiceFloat(0.54930615)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_acoth(2.0f64)),
///     NiceFloat(0.5493061443340549)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_acoth(-1.5f64)),
///     NiceFloat(-0.8047189562170501)
/// );
/// ```
#[inline]
#[allow(clippy::type_repetition_in_bounds)]
pub fn primitive_float_acoth<T: PrimitiveFloat>(x: T) -> T
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    emulate_float_to_float_fn(Float::acoth_prec, x)
}

/// Computes $\operatorname{acoth} x$, the inverse hyperbolic cotangent of a [`Rational`], returning
/// the result as a primitive float. The result is correctly rounded.
///
/// $$
/// f(x) = \operatorname{acoth} x+\varepsilon.
/// $$
/// - If $\operatorname{acoth} x$ is infinite, zero, or NaN, $\varepsilon$ may be ignored or assumed
///   to be 0.
/// - If $\operatorname{acoth} x$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2
///   |\operatorname{acoth} x|\rfloor-p}$, where $p$ is the precision of the output (typically 24 if
///   `T` is a [`f32`] and 53 if `T` is a [`f64`], but less if the output is subnormal).
///
/// Special cases:
/// - $f(\pm1)=\pm\infty$
/// - $f(x)=\text{NaN}$ if $|x|<1$
///
/// Overflow is not possible. Underflow is: an `x` of large enough magnitude gives `0.0` or `-0.0`.
///
/// # Worst-case complexity
/// $T(m) = O(m (\log m)^2 \log\log m)$
///
/// $M(m) = O(m \log m)$
///
/// where $T$ is time, $M$ is additional memory, and $m$ is `x.significant_bits()`.
///
/// # Examples
/// ```
/// use malachite_base::num::basic::traits::{One, Zero};
/// use malachite_base::num::float::NiceFloat;
/// use malachite_float::float::arithmetic::acoth::primitive_float_acoth_rational;
/// use malachite_q::Rational;
///
/// assert!(primitive_float_acoth_rational::<f64>(&Rational::ZERO).is_nan());
/// assert_eq!(
///     NiceFloat(primitive_float_acoth_rational::<f64>(&Rational::ONE)),
///     NiceFloat(f64::INFINITY)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_acoth_rational::<f64>(&Rational::from(3u32))),
///     NiceFloat(0.34657359027997264)
/// );
/// ```
#[inline]
#[allow(clippy::type_repetition_in_bounds)]
pub fn primitive_float_acoth_rational<T: PrimitiveFloat>(x: &Rational) -> T
where
    Float: PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    emulate_rational_to_float_fn(Float::acoth_rational_prec_ref, x)
}
