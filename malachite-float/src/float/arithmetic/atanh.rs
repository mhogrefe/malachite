// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the GNU MPFR Library.
//
//      Copyright 2001-2026 Free Software Foundation, Inc.
//
//      Contributed by the Pascaline and Caramba projects, INRIA.
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::InnerFloat::{Finite, Infinity, NaN, Zero};
use crate::float::arithmetic::asinh::round_with_error;
use crate::float::arithmetic::round_near_x::small_input_shortcut;
use crate::{Float, emulate_float_to_float_fn};
use core::cmp::Ordering::{self, *};
use core::cmp::max;
use malachite_base::fail_on_untested_path;
use malachite_base::num::arithmetic::traits::{
    Abs, Atanh, AtanhAssign, CeilingLogBase2, IsPowerOf2,
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

// This is mpfr_atanh_small from atanh.c, MPFR 4.2.2. It returns an approximation y of atanh(x), for
// 0 < x <= 1/2, at precision p, together with k such that the error is bounded by 2^k ulp(y). MPFR
// uses faithful rounding throughout; `Nearest` is used instead, which only improves the bounds.
//
// In the following, theta represents a value with |theta| <= 2^(1-p) (might be a different value
// each time).
fn atanh_small(x: &Float, p: u64) -> (Float, u64) {
    // t = x * (1 + theta)
    let mut t = Float::from_float_prec_round_ref(x, p, Nearest).0;
    // exact
    let mut y = t.clone();
    // x2 = x^2 * (1 + theta)
    let x2 = x.square_prec_round_ref(p, Nearest).0;
    let p_i64 = i64::exact_from(p);
    let mut i = 3u64;
    // i as a `Float`, kept at 64 bits so that every odd i stays exact
    let mut i_float = Float::from_unsigned_prec(3u32, 64).0;
    loop {
        // t = x^i * (1 + theta)^i
        t.mul_prec_round_assign_ref(&x2, p, Nearest);
        // u = x^i/i * (1 + theta)^(i+1)
        let u = t.div_prec_round_ref_ref(&i_float, p, Nearest).0;
        if u == 0u32 {
            // MPFR's extended exponent range keeps u from underflowing. Here a u that underflows to
            // zero is negligible too; this needs x^i to underflow before |u| < ulp(y), and so a
            // working precision above 2^30.
            fail_on_untested_path("atanh_small, u underflows");
            break;
        }
        // |u| < ulp(y)
        if i64::from(u.get_exponent().unwrap()) <= i64::from(y.get_exponent().unwrap()) - p_i64 {
            break;
        }
        // error <= ulp(y)
        y.add_prec_round_assign(u, p, Nearest);
        i += 2;
        i_float.add_prec_assign(Float::TWO, 64);
    }
    // See algorithms.tex: the total error is bounded by (i+7)/2 ulp(y).
    let err = (i + 8) >> 1; // ceil((i+7)/2)
    let k = err.ceiling_log_base_2();
    // if k + 2 < p, since k = ceil(log2(err)), we have err <= 2^k <= 2^(p-3), thus i+7 <= 2*err <=
    // 2^(p-2), thus (i+7)*epsilon <= 1/2, which implies our assumption (i+1)*epsilon <= 1/2.
    assert!(k + 2 < p);
    (y, k)
}

// This is mpfr_atanh from atanh.c, MPFR 4.2.2, where the input is finite, nonzero, and less than 1
// in absolute value. atanh = ln((1+x)/(1-x)) / 2, except when x is very small, in which case atanh
// = x + tiny error, and when x is small, where the Taylor expansion is used directly.
//
// MPFR computes (1+x)/(1-x) in an extended exponent range. Here it overflows once 1 - x is below
// about 2^(1 - MAX_EXPONENT), which needs an x with a precision above 2^30; for such an x the
// logarithm is instead computed as ln(1+x) - ln(1-x).
fn atanh_prec_round_normal_ref(xt: &Float, prec: u64, rm: RoundingMode) -> (Float, Ordering) {
    assert_ne!(rm, Exact, "Inexact atanh");
    let exp_xt = i64::from(xt.get_exponent().unwrap());
    // atanh(x) = x + x^3/3 + ... so the error is < 2^(3*EXP(x)-1)
    if let Some(result) = small_input_shortcut(xt, -(exp_xt << 1), 1, true, prec, rm) {
        return result;
    }
    let negative = *xt < 0u32;
    let x = xt.abs();
    // Compute initial precision
    let nt = max(x.get_prec().unwrap(), prec);
    let mut working_prec = nt + nt.ceiling_log_base_2() + 4;
    let mut increment = Limb::WIDTH;
    // small case: assuming the AGM algorithm used by ln uses log2(p) steps for a precision of p
    // bits, try the special variant whenever EXP(x) <= -p/log2(p). The +1 avoids a division by 0
    // when prec = 1. This implies EXP(x) <= -1, thus x < 1/2.
    let k = 1 + prec.ceiling_log_base_2();
    let small = exp_xt <= -1 - i64::exact_from(prec / k);
    loop {
        let (t, err) = if small {
            let (t, k) = atanh_small(&x, working_prec);
            (t, i64::exact_from(k))
        } else {
            // (1-x) with x = |xt|
            let te = Float::ONE
                .sub_prec_round_val_ref(&x, working_prec, Ceiling)
                .0;
            // (1+x)
            let t = x.add_prec_round_ref_val(Float::ONE, working_prec, Floor).0;
            let t = if i64::from(te.get_exponent().unwrap()) < 2 - Float::MAX_EXPONENT_I64 {
                fail_on_untested_path("atanh_prec_round_normal_ref, (1+x)/(1-x) may overflow");
                // ln(1+x) - ln(1-x)
                t.ln_prec_round(working_prec, Nearest)
                    .0
                    .sub_prec_round(
                        te.ln_prec_round(working_prec, Nearest).0,
                        working_prec,
                        Nearest,
                    )
                    .0
            } else {
                t
                    // (1+x)/(1-x)
                    .div_prec_round(te, working_prec, Nearest)
                    .0
                    // ln((1+x)/(1-x))
                    .ln_prec_round(working_prec, Nearest)
                    .0
            };
            // ln((1+x)/(1-x)) / 2
            let t = t >> 1u32;
            // error estimate: see algorithms.tex
            let err = max(4 - i64::from(t.get_exponent().unwrap()), 0) + 1;
            (t, err)
        };
        // MPFR also stops on a zero t, which cannot occur here: |x| is at least about
        // 2^(-prec/log2(prec)) outside the small case, and atanh_small returns at least x.
        if let Some(result) =
            round_with_error(if negative { -t } else { t }, working_prec, err, prec, rm)
        {
            return result;
        }
        // reactualisation of the precision
        working_prec += increment;
        increment = working_prec >> 1;
    }
}

impl Float {
    /// Computes $\operatorname{atanh} x$, the inverse hyperbolic tangent of a [`Float`], rounding
    /// the result to the specified precision and with the specified rounding mode. The [`Float`] is
    /// taken by value. An [`Ordering`] is also returned, indicating whether the rounded inverse
    /// hyperbolic tangent is less than, equal to, or greater than the exact inverse hyperbolic
    /// tangent. Although `NaN`s are not comparable to any [`Float`], whenever this function returns
    /// a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \operatorname{atanh} x+\varepsilon.
    /// $$
    /// - If $\operatorname{atanh} x$ is infinite, zero, or NaN, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $\operatorname{atanh} x$ is finite and nonzero, and $m$ is not `Nearest`, then
    ///   $|\varepsilon| < 2^{\lfloor\log_2 |\operatorname{atanh} x|\rfloor-p+1}$.
    /// - If $\operatorname{atanh} x$ is finite and nonzero, and $m$ is `Nearest`, then
    ///   $|\varepsilon| \leq 2^{\lfloor\log_2 |\operatorname{atanh} x|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=\text{NaN}$
    /// - $f(\pm\infty,p,m)=\text{NaN}$
    /// - $f(\pm0.0,p,m)=\pm0.0$
    /// - $f(\pm1,p,m)=\pm\infty$
    /// - $f(x,p,m)=\text{NaN}$ if $|x|>1$
    ///
    /// The result never overflows or underflows: for a nonzero $x$ with $|x|<1$, $|x| \leq
    /// |\operatorname{atanh} x| \leq \frac{q+1}{2}\ln 2$, where $q$ is the precision of $x$.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::atanh_prec`] instead. If you
    /// know that your target precision is the precision of the input, consider using
    /// [`Float::atanh_round`] instead. If both of these things are true, consider using
    /// [`Float::atanh`] instead.
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
    /// since the inverse hyperbolic tangent of such a [`Float`] is never exactly representable, or
    /// if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = (Float::one_prec(100) >> 1u32).atanh_prec_round(5, Floor);
    /// assert_eq!(c.to_string(), "0.531");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = (Float::one_prec(100) >> 1u32).atanh_prec_round(5, Ceiling);
    /// assert_eq!(c.to_string(), "0.562");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = (Float::one_prec(100) >> 1u32).atanh_prec_round(5, Nearest);
    /// assert_eq!(c.to_string(), "0.562");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = (Float::one_prec(100) >> 1u32).atanh_prec_round(20, Floor);
    /// assert_eq!(c.to_string(), "0.54930592");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = (Float::one_prec(100) >> 1u32).atanh_prec_round(20, Ceiling);
    /// assert_eq!(c.to_string(), "0.54930687");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = (Float::one_prec(100) >> 1u32).atanh_prec_round(20, Nearest);
    /// assert_eq!(c.to_string(), "0.54930592");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn atanh_prec_round(self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        self.atanh_prec_round_ref(prec, rm)
    }

    /// Computes $\operatorname{atanh} x$, the inverse hyperbolic tangent of a [`Float`], rounding
    /// the result to the specified precision and with the specified rounding mode. The [`Float`] is
    /// taken by reference. An [`Ordering`] is also returned, indicating whether the rounded inverse
    /// hyperbolic tangent is less than, equal to, or greater than the exact inverse hyperbolic
    /// tangent. Although `NaN`s are not comparable to any [`Float`], whenever this function returns
    /// a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \operatorname{atanh} x+\varepsilon.
    /// $$
    /// - If $\operatorname{atanh} x$ is infinite, zero, or NaN, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $\operatorname{atanh} x$ is finite and nonzero, and $m$ is not `Nearest`, then
    ///   $|\varepsilon| < 2^{\lfloor\log_2 |\operatorname{atanh} x|\rfloor-p+1}$.
    /// - If $\operatorname{atanh} x$ is finite and nonzero, and $m$ is `Nearest`, then
    ///   $|\varepsilon| \leq 2^{\lfloor\log_2 |\operatorname{atanh} x|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=\text{NaN}$
    /// - $f(\pm\infty,p,m)=\text{NaN}$
    /// - $f(\pm0.0,p,m)=\pm0.0$
    /// - $f(\pm1,p,m)=\pm\infty$
    /// - $f(x,p,m)=\text{NaN}$ if $|x|>1$
    ///
    /// The result never overflows or underflows: for a nonzero $x$ with $|x|<1$, $|x| \leq
    /// |\operatorname{atanh} x| \leq \frac{q+1}{2}\ln 2$, where $q$ is the precision of $x$.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::atanh_prec_ref`] instead. If
    /// you know that your target precision is the precision of the input, consider using
    /// [`Float::atanh_round_ref`] instead. If both of these things are true, consider using
    /// `(&Float).atanh()` instead.
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
    /// since the inverse hyperbolic tangent of such a [`Float`] is never exactly representable, or
    /// if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = (Float::one_prec(100) >> 1u32).atanh_prec_round_ref(5, Floor);
    /// assert_eq!(c.to_string(), "0.531");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = (Float::one_prec(100) >> 1u32).atanh_prec_round_ref(5, Ceiling);
    /// assert_eq!(c.to_string(), "0.562");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = (Float::one_prec(100) >> 1u32).atanh_prec_round_ref(5, Nearest);
    /// assert_eq!(c.to_string(), "0.562");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = (Float::one_prec(100) >> 1u32).atanh_prec_round_ref(20, Floor);
    /// assert_eq!(c.to_string(), "0.54930592");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = (Float::one_prec(100) >> 1u32).atanh_prec_round_ref(20, Ceiling);
    /// assert_eq!(c.to_string(), "0.54930687");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = (Float::one_prec(100) >> 1u32).atanh_prec_round_ref(20, Nearest);
    /// assert_eq!(c.to_string(), "0.54930592");
    /// assert_eq!(o, Less);
    /// ```
    pub fn atanh_prec_round_ref(&self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        assert_ne!(prec, 0);
        match &self.0 {
            // atanh(NaN) = NaN, and atanh(+/-Inf) = NaN since tanh gives a result between -1 and 1
            NaN | Infinity { .. } => (Self::NAN, Equal),
            // atanh(+/-0) = +/-0
            Zero { sign } => (
                if *sign {
                    Self::ZERO
                } else {
                    Self::NEGATIVE_ZERO
                },
                Equal,
            ),
            // atanh(x) = NaN as soon as |x| > 1, and atanh(+/-1) = +/-Inf
            Finite {
                sign,
                exponent,
                significand,
                ..
            } if *exponent > 0 => {
                if *exponent == 1 && significand.is_power_of_2() {
                    (
                        if *sign {
                            Self::INFINITY
                        } else {
                            Self::NEGATIVE_INFINITY
                        },
                        Equal,
                    )
                } else {
                    (Self::NAN, Equal)
                }
            }
            Finite { .. } => atanh_prec_round_normal_ref(self, prec, rm),
        }
    }

    /// Computes $\operatorname{atanh} x$, the inverse hyperbolic tangent of a [`Float`], rounding
    /// the result to the nearest value of the specified precision. The [`Float`] is taken by value.
    /// An [`Ordering`] is also returned, indicating whether the rounded inverse hyperbolic tangent
    /// is less than, equal to, or greater than the exact inverse hyperbolic tangent. Although
    /// `NaN`s are not comparable to any [`Float`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// If the inverse hyperbolic tangent is equidistant from two [`Float`]s with the specified
    /// precision, the [`Float`] with fewer 1s in its binary expansion is chosen. See
    /// [`RoundingMode`] for a description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = \operatorname{atanh} x+\varepsilon.
    /// $$
    /// - If $\operatorname{atanh} x$ is infinite, zero, or NaN, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $\operatorname{atanh} x$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2
    ///   |\operatorname{atanh} x|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p)=\text{NaN}$
    /// - $f(\pm\infty,p)=\text{NaN}$
    /// - $f(\pm0.0,p)=\pm0.0$
    /// - $f(\pm1,p)=\pm\infty$
    /// - $f(x,p)=\text{NaN}$ if $|x|>1$
    ///
    /// The result never overflows or underflows: for a nonzero $x$ with $|x|<1$, $|x| \leq
    /// |\operatorname{atanh} x| \leq \frac{q+1}{2}\ln 2$, where $q$ is the precision of $x$.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::atanh_prec_round`] instead. If you know that your target precision is the precision
    /// of the input, consider using [`Float::atanh`] instead.
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
    /// let (c, o) = (Float::one_prec(100) >> 1u32).atanh_prec(5);
    /// assert_eq!(c.to_string(), "0.562");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = (Float::one_prec(100) >> 1u32).atanh_prec(20);
    /// assert_eq!(c.to_string(), "0.54930592");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn atanh_prec(self, prec: u64) -> (Self, Ordering) {
        self.atanh_prec_round(prec, Nearest)
    }

    /// Computes $\operatorname{atanh} x$, the inverse hyperbolic tangent of a [`Float`], rounding
    /// the result to the nearest value of the specified precision. The [`Float`] is taken by
    /// reference. An [`Ordering`] is also returned, indicating whether the rounded inverse
    /// hyperbolic tangent is less than, equal to, or greater than the exact inverse hyperbolic
    /// tangent. Although `NaN`s are not comparable to any [`Float`], whenever this function returns
    /// a `NaN` it also returns `Equal`.
    ///
    /// If the inverse hyperbolic tangent is equidistant from two [`Float`]s with the specified
    /// precision, the [`Float`] with fewer 1s in its binary expansion is chosen. See
    /// [`RoundingMode`] for a description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = \operatorname{atanh} x+\varepsilon.
    /// $$
    /// - If $\operatorname{atanh} x$ is infinite, zero, or NaN, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $\operatorname{atanh} x$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2
    ///   |\operatorname{atanh} x|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p)=\text{NaN}$
    /// - $f(\pm\infty,p)=\text{NaN}$
    /// - $f(\pm0.0,p)=\pm0.0$
    /// - $f(\pm1,p)=\pm\infty$
    /// - $f(x,p)=\text{NaN}$ if $|x|>1$
    ///
    /// The result never overflows or underflows: for a nonzero $x$ with $|x|<1$, $|x| \leq
    /// |\operatorname{atanh} x| \leq \frac{q+1}{2}\ln 2$, where $q$ is the precision of $x$.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::atanh_prec_round_ref`] instead. If you know that your target precision is the
    /// precision of the input, consider using `(&Float).atanh()` instead.
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
    /// let (c, o) = (Float::one_prec(100) >> 1u32).atanh_prec_ref(5);
    /// assert_eq!(c.to_string(), "0.562");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = (Float::one_prec(100) >> 1u32).atanh_prec_ref(20);
    /// assert_eq!(c.to_string(), "0.54930592");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn atanh_prec_ref(&self, prec: u64) -> (Self, Ordering) {
        self.atanh_prec_round_ref(prec, Nearest)
    }

    /// Computes $\operatorname{atanh} x$, the inverse hyperbolic tangent of a [`Float`], rounding
    /// the result with the specified rounding mode. The [`Float`] is taken by value. An
    /// [`Ordering`] is also returned, indicating whether the rounded inverse hyperbolic tangent is
    /// less than, equal to, or greater than the exact inverse hyperbolic tangent. Although `NaN`s
    /// are not comparable to any [`Float`], whenever this function returns a `NaN` it also returns
    /// `Equal`.
    ///
    /// The precision of the output is the precision of the input. See [`RoundingMode`] for a
    /// description of the possible rounding modes.
    ///
    /// $$
    /// f(x,m) = \operatorname{atanh} x+\varepsilon.
    /// $$
    /// - If $\operatorname{atanh} x$ is infinite, zero, or NaN, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $\operatorname{atanh} x$ is finite and nonzero, and $m$ is not `Nearest`, then
    ///   $|\varepsilon| < 2^{\lfloor\log_2 |\operatorname{atanh} x|\rfloor-p+1}$, where $p$ is the
    ///   precision of the input.
    /// - If $\operatorname{atanh} x$ is finite and nonzero, and $m$ is `Nearest`, then
    ///   $|\varepsilon| \leq 2^{\lfloor\log_2 |\operatorname{atanh} x|\rfloor-p}$, where $p$ is the
    ///   precision of the input.
    ///
    /// If the output has a precision, it is the precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN},m)=\text{NaN}$
    /// - $f(\pm\infty,m)=\text{NaN}$
    /// - $f(\pm0.0,m)=\pm0.0$
    /// - $f(\pm1,m)=\pm\infty$
    /// - $f(x,m)=\text{NaN}$ if $|x|>1$
    ///
    /// The result never overflows or underflows: for a nonzero $x$ with $|x|<1$, $|x| \leq
    /// |\operatorname{atanh} x| \leq \frac{q+1}{2}\ln 2$, where $q$ is the precision of $x$.
    ///
    /// If you want to specify an output precision, consider using [`Float::atanh_prec_round`]
    /// instead. If you know you'll be using the `Nearest` rounding mode, consider using
    /// [`Float::atanh`] instead.
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
    /// since the inverse hyperbolic tangent of such a [`Float`] is never exactly representable.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = (Float::one_prec(100) >> 1u32).atanh_round(Floor);
    /// assert_eq!(c.to_string(), "0.54930614433405484569762261846113");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = (Float::one_prec(100) >> 1u32).atanh_round(Ceiling);
    /// assert_eq!(c.to_string(), "0.54930614433405484569762261846192");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = (Float::one_prec(100) >> 1u32).atanh_round(Nearest);
    /// assert_eq!(c.to_string(), "0.54930614433405484569762261846113");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn atanh_round(self, rm: RoundingMode) -> (Self, Ordering) {
        let prec = self.significant_bits();
        self.atanh_prec_round(prec, rm)
    }

    /// Computes $\operatorname{atanh} x$, the inverse hyperbolic tangent of a [`Float`], rounding
    /// the result with the specified rounding mode. The [`Float`] is taken by reference. An
    /// [`Ordering`] is also returned, indicating whether the rounded inverse hyperbolic tangent is
    /// less than, equal to, or greater than the exact inverse hyperbolic tangent. Although `NaN`s
    /// are not comparable to any [`Float`], whenever this function returns a `NaN` it also returns
    /// `Equal`.
    ///
    /// The precision of the output is the precision of the input. See [`RoundingMode`] for a
    /// description of the possible rounding modes.
    ///
    /// $$
    /// f(x,m) = \operatorname{atanh} x+\varepsilon.
    /// $$
    /// - If $\operatorname{atanh} x$ is infinite, zero, or NaN, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $\operatorname{atanh} x$ is finite and nonzero, and $m$ is not `Nearest`, then
    ///   $|\varepsilon| < 2^{\lfloor\log_2 |\operatorname{atanh} x|\rfloor-p+1}$, where $p$ is the
    ///   precision of the input.
    /// - If $\operatorname{atanh} x$ is finite and nonzero, and $m$ is `Nearest`, then
    ///   $|\varepsilon| \leq 2^{\lfloor\log_2 |\operatorname{atanh} x|\rfloor-p}$, where $p$ is the
    ///   precision of the input.
    ///
    /// If the output has a precision, it is the precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN},m)=\text{NaN}$
    /// - $f(\pm\infty,m)=\text{NaN}$
    /// - $f(\pm0.0,m)=\pm0.0$
    /// - $f(\pm1,m)=\pm\infty$
    /// - $f(x,m)=\text{NaN}$ if $|x|>1$
    ///
    /// The result never overflows or underflows: for a nonzero $x$ with $|x|<1$, $|x| \leq
    /// |\operatorname{atanh} x| \leq \frac{q+1}{2}\ln 2$, where $q$ is the precision of $x$.
    ///
    /// If you want to specify an output precision, consider using [`Float::atanh_prec_round_ref`]
    /// instead. If you know you'll be using the `Nearest` rounding mode, consider using
    /// `(&Float).atanh()` instead.
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
    /// since the inverse hyperbolic tangent of such a [`Float`] is never exactly representable.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = (Float::one_prec(100) >> 1u32).atanh_round_ref(Floor);
    /// assert_eq!(c.to_string(), "0.54930614433405484569762261846113");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = (Float::one_prec(100) >> 1u32).atanh_round_ref(Ceiling);
    /// assert_eq!(c.to_string(), "0.54930614433405484569762261846192");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = (Float::one_prec(100) >> 1u32).atanh_round_ref(Nearest);
    /// assert_eq!(c.to_string(), "0.54930614433405484569762261846113");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn atanh_round_ref(&self, rm: RoundingMode) -> (Self, Ordering) {
        self.atanh_prec_round_ref(self.significant_bits(), rm)
    }

    /// Computes $\operatorname{atanh} x$, the inverse hyperbolic tangent of a [`Float`], rounding
    /// the result to the specified precision and with the specified rounding mode. The [`Float`] is
    /// replaced by the result, and an [`Ordering`] is returned, indicating whether the rounded
    /// inverse hyperbolic tangent is less than, equal to, or greater than the exact inverse
    /// hyperbolic tangent. Although `NaN`s are not comparable to any [`Float`], whenever this
    /// function sets a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets \operatorname{atanh} x+\varepsilon.
    /// $$
    /// - If $\operatorname{atanh} x$ is infinite, zero, or NaN, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $\operatorname{atanh} x$ is finite and nonzero, and $m$ is not `Nearest`, then
    ///   $|\varepsilon| < 2^{\lfloor\log_2 |\operatorname{atanh} x|\rfloor-p+1}$.
    /// - If $\operatorname{atanh} x$ is finite and nonzero, and $m$ is `Nearest`, then
    ///   $|\varepsilon| \leq 2^{\lfloor\log_2 |\operatorname{atanh} x|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// See the [`Float::atanh_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::atanh_prec_assign`] instead.
    /// If you know that your target precision is the precision of the input, consider using
    /// [`Float::atanh_round_assign`] instead. If both of these things are true, consider using
    /// [`Float::atanh_assign`] instead.
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
    /// since the inverse hyperbolic tangent of such a [`Float`] is never exactly representable, or
    /// if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::one_prec(100) >> 1u32;
    /// assert_eq!(x.atanh_prec_round_assign(5, Floor), Less);
    /// assert_eq!(x.to_string(), "0.531");
    ///
    /// let mut x = Float::one_prec(100) >> 1u32;
    /// assert_eq!(x.atanh_prec_round_assign(5, Ceiling), Greater);
    /// assert_eq!(x.to_string(), "0.562");
    ///
    /// let mut x = Float::one_prec(100) >> 1u32;
    /// assert_eq!(x.atanh_prec_round_assign(5, Nearest), Greater);
    /// assert_eq!(x.to_string(), "0.562");
    ///
    /// let mut x = Float::one_prec(100) >> 1u32;
    /// assert_eq!(x.atanh_prec_round_assign(20, Floor), Less);
    /// assert_eq!(x.to_string(), "0.54930592");
    ///
    /// let mut x = Float::one_prec(100) >> 1u32;
    /// assert_eq!(x.atanh_prec_round_assign(20, Ceiling), Greater);
    /// assert_eq!(x.to_string(), "0.54930687");
    ///
    /// let mut x = Float::one_prec(100) >> 1u32;
    /// assert_eq!(x.atanh_prec_round_assign(20, Nearest), Less);
    /// assert_eq!(x.to_string(), "0.54930592");
    /// ```
    #[inline]
    pub fn atanh_prec_round_assign(&mut self, prec: u64, rm: RoundingMode) -> Ordering {
        let o;
        (*self, o) = self.atanh_prec_round_ref(prec, rm);
        o
    }

    /// Computes $\operatorname{atanh} x$, the inverse hyperbolic tangent of a [`Float`], rounding
    /// the result to the nearest value of the specified precision. The [`Float`] is replaced by the
    /// result, and an [`Ordering`] is returned, indicating whether the rounded inverse hyperbolic
    /// tangent is less than, equal to, or greater than the exact inverse hyperbolic tangent.
    /// Although `NaN`s are not comparable to any [`Float`], whenever this function sets a `NaN` it
    /// also returns `Equal`.
    ///
    /// If the inverse hyperbolic tangent is equidistant from two [`Float`]s with the specified
    /// precision, the [`Float`] with fewer 1s in its binary expansion is chosen. See
    /// [`RoundingMode`] for a description of the `Nearest` rounding mode.
    ///
    /// $$
    /// x \gets \operatorname{atanh} x+\varepsilon.
    /// $$
    /// - If $\operatorname{atanh} x$ is infinite, zero, or NaN, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $\operatorname{atanh} x$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2
    ///   |\operatorname{atanh} x|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// See the [`Float::atanh_prec`] documentation for information on special cases, overflow, and
    /// underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::atanh_prec_round_assign`] instead. If you know that your target precision is the
    /// precision of the input, consider using [`Float::atanh_assign`] instead.
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
    /// let mut x = Float::one_prec(100) >> 1u32;
    /// assert_eq!(x.atanh_prec_assign(5), Greater);
    /// assert_eq!(x.to_string(), "0.562");
    ///
    /// let mut x = Float::one_prec(100) >> 1u32;
    /// assert_eq!(x.atanh_prec_assign(20), Less);
    /// assert_eq!(x.to_string(), "0.54930592");
    /// ```
    #[inline]
    pub fn atanh_prec_assign(&mut self, prec: u64) -> Ordering {
        self.atanh_prec_round_assign(prec, Nearest)
    }

    /// Computes $\operatorname{atanh} x$, the inverse hyperbolic tangent of a [`Float`], rounding
    /// the result with the specified rounding mode. The [`Float`] is replaced by the result, and an
    /// [`Ordering`] is returned, indicating whether the rounded inverse hyperbolic tangent is less
    /// than, equal to, or greater than the exact inverse hyperbolic tangent. Although `NaN`s are
    /// not comparable to any [`Float`], whenever this function sets a `NaN` it also returns
    /// `Equal`.
    ///
    /// The precision of the output is the precision of the input. See [`RoundingMode`] for a
    /// description of the possible rounding modes.
    ///
    /// $$
    /// x \gets \operatorname{atanh} x+\varepsilon.
    /// $$
    /// - If $\operatorname{atanh} x$ is infinite, zero, or NaN, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $\operatorname{atanh} x$ is finite and nonzero, and $m$ is not `Nearest`, then
    ///   $|\varepsilon| < 2^{\lfloor\log_2 |\operatorname{atanh} x|\rfloor-p+1}$, where $p$ is the
    ///   precision of the input.
    /// - If $\operatorname{atanh} x$ is finite and nonzero, and $m$ is `Nearest`, then
    ///   $|\varepsilon| \leq 2^{\lfloor\log_2 |\operatorname{atanh} x|\rfloor-p}$, where $p$ is the
    ///   precision of the input.
    ///
    /// If the output has a precision, it is the precision of the input.
    ///
    /// See the [`Float::atanh_round`] documentation for information on special cases, overflow, and
    /// underflow.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::atanh_prec_round_assign`] instead. If you know you'll be using the `Nearest`
    /// rounding mode, consider using [`Float::atanh_assign`] instead.
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
    /// since the inverse hyperbolic tangent of such a [`Float`] is never exactly representable.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::one_prec(100) >> 1u32;
    /// assert_eq!(x.atanh_round_assign(Floor), Less);
    /// assert_eq!(x.to_string(), "0.54930614433405484569762261846113");
    ///
    /// let mut x = Float::one_prec(100) >> 1u32;
    /// assert_eq!(x.atanh_round_assign(Ceiling), Greater);
    /// assert_eq!(x.to_string(), "0.54930614433405484569762261846192");
    ///
    /// let mut x = Float::one_prec(100) >> 1u32;
    /// assert_eq!(x.atanh_round_assign(Nearest), Less);
    /// assert_eq!(x.to_string(), "0.54930614433405484569762261846113");
    /// ```
    #[inline]
    pub fn atanh_round_assign(&mut self, rm: RoundingMode) -> Ordering {
        let prec = self.significant_bits();
        self.atanh_prec_round_assign(prec, rm)
    }
}

impl Atanh for Float {
    type Output = Self;

    /// Computes $\operatorname{atanh} x$, the inverse hyperbolic tangent of a [`Float`], taking it
    /// by value.
    ///
    /// If the output has a precision, it is the precision of the input. If the inverse hyperbolic
    /// tangent is equidistant from two [`Float`]s with the specified precision, the [`Float`] with
    /// fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of the
    /// `Nearest` rounding mode.
    ///
    /// $$
    /// f(x) = \operatorname{atanh} x+\varepsilon.
    /// $$
    /// - If $\operatorname{atanh} x$ is infinite, zero, or NaN, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $\operatorname{atanh} x$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2
    ///   |\operatorname{atanh} x|\rfloor-p}$, where $p$ is the precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN})=\text{NaN}$
    /// - $f(\pm\infty)=\text{NaN}$
    /// - $f(\pm0.0)=\pm0.0$
    /// - $f(\pm1)=\pm\infty$
    /// - $f(x)=\text{NaN}$ if $|x|>1$
    ///
    /// See the [`Float::atanh_round`] documentation for information on overflow and underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::atanh_round`] instead. If you want to specify the output precision, consider using
    /// [`Float::atanh_prec`]. If you want both of these things, consider using
    /// [`Float::atanh_prec_round`].
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
    /// use malachite_base::num::arithmetic::traits::Atanh;
    /// use malachite_base::num::basic::traits::*;
    /// use malachite_float::Float;
    ///
    /// assert!(Float::NAN.atanh().is_nan());
    /// assert!(Float::INFINITY.atanh().is_nan());
    /// assert!(Float::NEGATIVE_INFINITY.atanh().is_nan());
    /// assert_eq!(Float::ZERO.atanh().to_string(), "0.0");
    /// assert_eq!(Float::NEGATIVE_ZERO.atanh().to_string(), "-0.0");
    /// assert_eq!(Float::ONE.atanh().to_string(), "Infinity");
    /// assert_eq!(Float::NEGATIVE_ONE.atanh().to_string(), "-Infinity");
    /// assert!(Float::TWO.atanh().is_nan());
    /// assert_eq!((Float::one_prec(100) >> 1u32).atanh().to_string(), "0.54930614433405484569762261846113");
    /// assert_eq!((-(Float::from_unsigned_prec(3u32, 100).0 >> 2u32)).atanh().to_string(), "-0.97295507452765665255267637172144");
    /// ```
    #[inline]
    fn atanh(self) -> Self {
        let prec = self.significant_bits();
        self.atanh_prec_round(prec, Nearest).0
    }
}

impl Atanh for &Float {
    type Output = Float;

    /// Computes $\operatorname{atanh} x$, the inverse hyperbolic tangent of a [`Float`], taking it
    /// by reference.
    ///
    /// If the output has a precision, it is the precision of the input. If the inverse hyperbolic
    /// tangent is equidistant from two [`Float`]s with the specified precision, the [`Float`] with
    /// fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of the
    /// `Nearest` rounding mode.
    ///
    /// $$
    /// f(x) = \operatorname{atanh} x+\varepsilon.
    /// $$
    /// - If $\operatorname{atanh} x$ is infinite, zero, or NaN, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $\operatorname{atanh} x$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2
    ///   |\operatorname{atanh} x|\rfloor-p}$, where $p$ is the precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN})=\text{NaN}$
    /// - $f(\pm\infty)=\text{NaN}$
    /// - $f(\pm0.0)=\pm0.0$
    /// - $f(\pm1)=\pm\infty$
    /// - $f(x)=\text{NaN}$ if $|x|>1$
    ///
    /// See the [`Float::atanh_round`] documentation for information on overflow and underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::atanh_round_ref`] instead. If you want to specify the output precision, consider
    /// using [`Float::atanh_prec_ref`]. If you want both of these things, consider using
    /// [`Float::atanh_prec_round_ref`].
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
    /// use malachite_base::num::arithmetic::traits::Atanh;
    /// use malachite_base::num::basic::traits::*;
    /// use malachite_float::Float;
    ///
    /// assert!((&Float::NAN).atanh().is_nan());
    /// assert!((&Float::INFINITY).atanh().is_nan());
    /// assert!((&Float::NEGATIVE_INFINITY).atanh().is_nan());
    /// assert_eq!((&Float::ZERO).atanh().to_string(), "0.0");
    /// assert_eq!((&Float::NEGATIVE_ZERO).atanh().to_string(), "-0.0");
    /// assert_eq!((&Float::ONE).atanh().to_string(), "Infinity");
    /// assert_eq!((&Float::NEGATIVE_ONE).atanh().to_string(), "-Infinity");
    /// assert!((&Float::TWO).atanh().is_nan());
    /// assert_eq!((&(Float::one_prec(100) >> 1u32)).atanh().to_string(), "0.54930614433405484569762261846113");
    /// assert_eq!((&-(Float::from_unsigned_prec(3u32, 100).0 >> 2u32)).atanh().to_string(), "-0.97295507452765665255267637172144");
    /// ```
    #[inline]
    fn atanh(self) -> Float {
        self.atanh_prec_round_ref(self.significant_bits(), Nearest)
            .0
    }
}

impl AtanhAssign for Float {
    /// Computes $\operatorname{atanh} x$, the inverse hyperbolic tangent of a [`Float`], in place.
    ///
    /// If the output has a precision, it is the precision of the input. If the inverse hyperbolic
    /// tangent is equidistant from two [`Float`]s with the specified precision, the [`Float`] with
    /// fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of the
    /// `Nearest` rounding mode.
    ///
    /// $$
    /// x \gets \operatorname{atanh} x+\varepsilon.
    /// $$
    /// - If $\operatorname{atanh} x$ is infinite, zero, or NaN, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $\operatorname{atanh} x$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2
    ///   |\operatorname{atanh} x|\rfloor-p}$, where $p$ is the precision of the input.
    ///
    /// See the [`Float::atanh`] documentation for information on special cases, overflow, and
    /// underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::atanh_round_assign`] instead. If you want to specify the output precision, consider
    /// using [`Float::atanh_prec_assign`]. If you want both of these things, consider using
    /// [`Float::atanh_prec_round_assign`].
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
    /// use malachite_base::num::arithmetic::traits::AtanhAssign;
    /// use malachite_base::num::basic::traits::*;
    /// use malachite_float::Float;
    ///
    /// let mut x = Float::NAN;
    /// x.atanh_assign();
    /// assert!(x.is_nan());
    ///
    /// let mut x = Float::INFINITY;
    /// x.atanh_assign();
    /// assert!(x.is_nan());
    ///
    /// let mut x = Float::NEGATIVE_INFINITY;
    /// x.atanh_assign();
    /// assert!(x.is_nan());
    ///
    /// let mut x = Float::ZERO;
    /// x.atanh_assign();
    /// assert_eq!(x.to_string(), "0.0");
    ///
    /// let mut x = Float::NEGATIVE_ZERO;
    /// x.atanh_assign();
    /// assert_eq!(x.to_string(), "-0.0");
    ///
    /// let mut x = Float::ONE;
    /// x.atanh_assign();
    /// assert_eq!(x.to_string(), "Infinity");
    ///
    /// let mut x = Float::NEGATIVE_ONE;
    /// x.atanh_assign();
    /// assert_eq!(x.to_string(), "-Infinity");
    ///
    /// let mut x = Float::TWO;
    /// x.atanh_assign();
    /// assert!(x.is_nan());
    ///
    /// let mut x = Float::one_prec(100) >> 1u32;
    /// x.atanh_assign();
    /// assert_eq!(x.to_string(), "0.54930614433405484569762261846113");
    ///
    /// let mut x = -(Float::from_unsigned_prec(3u32, 100).0 >> 2u32);
    /// x.atanh_assign();
    /// assert_eq!(x.to_string(), "-0.97295507452765665255267637172144");
    /// ```
    #[inline]
    fn atanh_assign(&mut self) {
        let prec = self.significant_bits();
        self.atanh_prec_round_assign(prec, Nearest);
    }
}

/// Computes $\operatorname{atanh} x$, the inverse hyperbolic tangent of a primitive float. Using
/// this function is more accurate than using the default `atanh` function or the one provided by
/// `libm`.
///
/// $$
/// f(x) = \operatorname{atanh} x+\varepsilon.
/// $$
/// - If $\operatorname{atanh} x$ is infinite, zero, or NaN, $\varepsilon$ may be ignored or assumed
///   to be 0.
/// - If $\operatorname{atanh} x$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2
///   |\operatorname{atanh} x|\rfloor-p}$, where $p$ is the precision of the output (24 if `T` is a
///   [`f32`] and 53 if `T` is a [`f64`]).
///
/// Special cases:
/// - $f(\text{NaN})=\text{NaN}$
/// - $f(\pm\infty)=\text{NaN}$
/// - $f(\pm0.0)=\pm0.0$
/// - $f(\pm1)=\pm\infty$
/// - $f(x)=\text{NaN}$ if $|x|>1$
///
/// Overflow is not possible. The result is subnormal only when $x$ is, and then it is $x$ itself,
/// since $|\operatorname{atanh} x - x| < |x|^3/2$.
///
/// # Worst-case complexity
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use malachite_base::num::basic::traits::NegativeInfinity;
/// use malachite_base::num::float::NiceFloat;
/// use malachite_float::float::arithmetic::atanh::primitive_float_atanh;
///
/// assert!(primitive_float_atanh(f32::NAN).is_nan());
/// assert!(primitive_float_atanh(f32::INFINITY).is_nan());
/// assert!(primitive_float_atanh(f32::NEGATIVE_INFINITY).is_nan());
/// assert_eq!(NiceFloat(primitive_float_atanh(0.0f32)), NiceFloat(0.0));
/// assert_eq!(NiceFloat(primitive_float_atanh(-0.0f32)), NiceFloat(-0.0));
/// assert_eq!(NiceFloat(primitive_float_atanh(1.0f32)), NiceFloat(f32::INFINITY));
/// assert!(primitive_float_atanh(2.0f32).is_nan());
/// assert_eq!(NiceFloat(primitive_float_atanh(0.5f32)), NiceFloat(0.54930615));
/// assert_eq!(NiceFloat(primitive_float_atanh(0.5f64)), NiceFloat(0.5493061443340549));
/// assert_eq!(NiceFloat(primitive_float_atanh(-0.75f64)), NiceFloat(-0.9729550745276566));
/// ```
#[inline]
#[allow(clippy::type_repetition_in_bounds)]
pub fn primitive_float_atanh<T: PrimitiveFloat>(x: T) -> T
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    emulate_float_to_float_fn(Float::atanh_prec, x)
}
