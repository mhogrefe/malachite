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
use crate::big_exponent_float::arithmetic::add::significand_prec;
use crate::big_exponent_float::arithmetic::mul_rational::{
    finite_to_float, from_float_result, shift_finite,
};
use alloc::borrow::Cow;
use core::cmp::{
    Ordering::{self, *},
    max,
};
use malachite_base::num::arithmetic::traits::{CeilingLogBase2, PowerOf2};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::{
    Infinity as InfinityTrait, NaN as NaNTrait, One, Zero as ZeroTrait,
};
use malachite_base::num::conversion::traits::{ExactFrom, RoundingFrom};
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_nz::natural::arithmetic::float::round::float_can_round;
use malachite_nz::platform::Limb;
use malachite_q::Rational;

// If |x| < 2^29, then e^x, 2^x, 10^x / 2, and their counterparts minus 1 lie strictly between
// 2^(-(2^30 - 2)) and 2^(2^30 - 2), well within `Float`'s exponent range; so these functions can
// use `Float`'s functions for every argument whose exponent is at most `SAFE_EXPONENT`, or at most
// `SAFE_EXPONENT - 1` for 10^x and 10^x - 1. For example, e^(2^29) = 2^(2^29 log_2 e) < 2^(7.75 *
// 10^8), and 10^(2^28) = 2^(2^28 log_2 10) < 2^(8.92 * 10^8), while 2^30 > 1.07 * 10^9.
pub(crate) const SAFE_EXPONENT: i32 = 29;

// `SAFE_EXPONENT` as an `i64`.
pub(crate) const SAFE_EXPONENT_I64: i64 = SAFE_EXPONENT as i64;

// The smallest exponent of an argument for which the functions minus 1 use `Float`'s functions. The
// result is then at least half of |x| in absolute value, so its exponent is at least
// `Float::MIN_EXPONENT + 1`, and is never taken for an underflow.
pub(crate) const MIN_SAFE_MINUS_1_EXPONENT: i32 = Float::MIN_EXPONENT + 2;

// Whether `x`, an approximation with an error of at most 2^(EXP(x) - err) (where x lies in
// [2^(EXP(x) - 1), 2^EXP(x)) in absolute value), determines the rounding of the exact value to
// `prec` bits with `rm`, together with the `Ordering`. This is MPFR's `MPFR_CAN_ROUND`, applied to
// x's significand padded with zero limbs to `wp` bits: `float_can_round` only examines the
// significand's own limbs, and a `BigExponentFloat`'s significand has no trailing zero limbs.
pub(crate) fn finite_can_round(
    x: &BigExponentFloat,
    err: u64,
    wp: u64,
    prec: u64,
    rm: RoundingMode,
) -> bool {
    let BigExponentFloat(Finite { significand, .. }) = x else {
        return false;
    };
    let limbs = wp.div_ceil(Limb::WIDTH);
    let len = significand.limb_count();
    if len >= limbs {
        float_can_round(significand, err, prec, rm)
    } else {
        float_can_round(
            &(significand << ((limbs - len) << Limb::LOG_WIDTH)),
            err,
            prec,
            rm,
        )
    }
}

// Ziv's strategy: `approx(wp)` returns an approximation of a nonzero real number, computed with
// working precision `wp`, together with an `err` such that the approximation's error is at most
// 2^(EXP - err), where EXP is the approximation's exponent. The working precision grows until the
// approximation determines the correctly rounded result and its `Ordering`. The real number must
// not be exactly representable with `prec` bits, nor be a midpoint between two such numbers, or the
// loop never ends.
pub(crate) fn ziv_loop<F: FnMut(u64) -> (BigExponentFloat, u64)>(
    prec: u64,
    rm: RoundingMode,
    mut approx: F,
) -> (BigExponentFloat, Ordering) {
    let mut wp = prec + prec.significant_bits() + 10;
    let mut increment = Limb::WIDTH;
    loop {
        let (y, err) = approx(wp);
        if finite_can_round(&y, err, wp, prec, rm) {
            return y.round_to_prec_round(prec, rm);
        }
        wp += increment;
        increment = wp >> 1;
    }
}

// v + s if `up`, and v - s otherwise, rounded to `prec` bits with `rm`, for a finite nonzero v and
// any s with 0 < s < 2^(e - max(p, q) - 2), where e is v's exponent (|v| lies in [2^(e - 1), 2^e))
// and q its precision. The rounding breakpoints for `prec` bits (the representable values and the
// midpoints between them) nearest to v on either side are at least 2^(e - max(p + 2, q)) away from
// it, so v ± s lies strictly between v and the next breakpoint, and every such s gives the same
// result and `Ordering`. The stand-in s = 2^(e - max(p, q) - 3) is used.
pub(crate) fn round_nudged(
    v: &BigExponentFloat,
    up: bool,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let BigExponentFloat(Finite {
        exponent,
        significand,
        ..
    }) = v
    else {
        unreachable!()
    };
    let gap = max(prec, significand_prec(significand)) + 2;
    let s = BigExponentFloat::from_parts(true, exponent - Integer::from(gap), Natural::ONE);
    if up {
        v.add_prec_round_ref_val(s, prec, rm)
    } else {
        v.sub_prec_round_ref_val(s, prec, rm)
    }
}

// 2^n for an integer n.
pub(crate) fn power_of_2_integer(n: Integer) -> BigExponentFloat {
    BigExponentFloat::from_parts(true, n + Integer::ONE, Natural::ONE)
}

crate_test_fn! {
    // An approximation S of e^t - 1 for a nonzero t with |t| <= 1/2, computed by summing the Taylor
    // series t + t^2/2! + t^3/3! + ... with `wp`-bit arithmetic, and the number of bits lost: the
    // error is at most 2^(EXP(S) - wp + loss).
    //
    // With u = 2^(-wp), the kth term is computed with 2(k - 1) roundings, so its relative error is
    // at most 2.02ku; summed over all terms this is at most 2.02u|t|e^|t| <= 3.4u|t|. Each partial
    // sum is at most |t|e^|t| <= 1.65|t| in absolute value, so the K - 1 additions contribute at
    // most 1.65(K - 1)u|t|. The summation stops once a term is below 2^(EXP(t) - wp) <= 2u|t|, and
    // then the remaining terms add up to less than 2.02u|t|. The total is below (1.65K + 3.8)u|t|,
    // and since |e^t - 1| >= 0.78|t| for |t| <= 1/2, |t| < 1.3 * 2^EXP(S); so the error is below
    // (2.2K + 5) * 2^(EXP(S) - wp) <= (4K + 8) * 2^(EXP(S) - wp).
    exp_x_minus_1_series(t: &BigExponentFloat, wp: u64) -> (BigExponentFloat, u64) {
        let threshold = t.get_exponent().unwrap() - Integer::from(wp);
        let mut sum = t.clone();
        let mut term = t.clone();
        let mut k = 1u64;
        loop {
            k += 1;
            term = term
                .mul_prec_val_ref(t, wp)
                .0
                .div_prec(BigExponentFloat::from(k), wp)
                .0;
            sum.add_prec_assign_ref(&term, wp);
            if *term.get_exponent().unwrap() <= threshold {
                return (sum, ((k << 2) + 8).ceiling_log_base_2());
            }
        }
    }
}

// An approximation of e^t for a nonzero t with |t| <= 1/2, computed as 1 + S with S from
// `exp_x_minus_1_series`, and an `err` as in `ziv_loop`. S has an error of at most 2^(EXP(S) - wp +
// loss), and EXP(S) is at most the exponent of 1 + S, so adding 1 and rounding brings the error to
// at most 2^(EXP - wp + loss + 1).
pub(crate) fn exp_small_approx(t: &BigExponentFloat, wp: u64) -> (BigExponentFloat, u64) {
    let (s, loss) = exp_x_minus_1_series(t, wp);
    (BigExponentFloat::ONE.add_prec(s, wp).0, wp - loss - 1)
}

// The argument of an exponential function whose result is outside `Float`'s exponent range: a
// `BigExponentFloat`, used exactly, or a `Rational`, which is rounded as precisely as needed.
pub(crate) enum ExpArgument<'a> {
    BigExponentFloat(&'a BigExponentFloat),
    Rational(&'a Rational),
}

impl ExpArgument<'_> {
    // The argument x, with an absolute error of at most 2^(e - prec - 1) if |x| < 2^e.
    fn approx(&self, prec: u64) -> Cow<'_, BigExponentFloat> {
        match self {
            ExpArgument::BigExponentFloat(x) => Cow::Borrowed(x),
            ExpArgument::Rational(x) => {
                Cow::Owned(BigExponentFloat::from_rational_prec_ref(x, prec).0)
            }
        }
    }
}

// e^x, or e^x - 1 if `minus_one`, rounded to `prec` bits with `rm`, for an x with 2^29 <= |x| <
// 2^e. `rm` is not `Exact`: these values are irrational.
//
// With n = round(x / log 2) and r = x - n log 2, e^x = 2^n e^r with |r| <= (log 2) / 2 (or a little
// more, since n is computed from approximations). log 2 is computed with lp = wp + e + 10 bits, so
// its error is at most 2^(-lp - 1); |n| < 2^(e + 1), so n log 2 has an error of at most 2^(-wp -
// 10). With the error of x's approximation (for a `Rational` x) and the rounding of r to wp + 12
// bits, r has an error below 2^(-wp - 9), and so does e^r, relatively; rounding e^r to wp bits then
// leaves an error of at most 2^(EXP - wp). Subtracting 1 (when `minus_one`) adds half an ulp, and
// relative errors are not magnified: for x >= 2^29 the result is about e^x, and for x <= -2^29 it
// is about -1, with the error of e^x far below an ulp.
pub(crate) fn exp_huge(
    x: &ExpArgument<'_>,
    e: u64,
    prec: u64,
    rm: RoundingMode,
    minus_one: bool,
) -> (BigExponentFloat, Ordering) {
    ziv_loop(prec, rm, |wp| {
        let lp = wp + e + 10;
        let ln_2 = BigExponentFloat::from(Float::ln_2_prec(lp).0);
        let x = x.approx(lp + 2);
        let quotient = x.div_prec_ref_ref(&ln_2, e + 8).0;
        let n = Integer::rounding_from(&quotient, Nearest).0;
        let r = x
            .sub_prec_ref_val(BigExponentFloat::from(n.clone()) * &ln_2, wp + 12)
            .0;
        let (y, err) = match &r {
            BigExponentFloat(Finite { exponent, .. }) => {
                if *exponent < -Integer::from(wp + 10) {
                    // e^r is within 2^(-wp - 8) of 1. This approximation never passes the rounding
                    // test, but the next working precision brings r into range.
                    (BigExponentFloat::ONE, wp)
                } else if let Ok(r_exponent) = i32::try_from(exponent)
                    && r_exponent >= Float::MIN_EXPONENT
                {
                    let y = finite_to_float(Cow::Borrowed(&r), r_exponent)
                        .exp_prec(wp)
                        .0;
                    (BigExponentFloat::from(y), wp - 1)
                } else {
                    let (y, err) = exp_small_approx(&r, wp);
                    (y, err - 1)
                }
            }
            BigExponentFloat(Zero) => (BigExponentFloat::ONE, wp),
            _ => unreachable!(),
        };
        let y = shift_finite(y, &n);
        if minus_one {
            (y.sub_prec(BigExponentFloat::ONE, wp).0, err - 2)
        } else {
            (y, err)
        }
    })
}

// Whether 2^(e - 1) >= prec + 3, which for a negative x with exponent e means that e^x, 2^x, and
// 10^x are all less than 2^(-prec - 3), so that their counterparts minus 1 are -1 plus a positive
// amount small enough for `round_nudged`.
pub(crate) fn far_below_minus_one(e: &Integer, prec: u64) -> bool {
    match u64::try_from(&(e - Integer::ONE)) {
        Ok(k) => k >= u64::WIDTH || u64::power_of_2(k) >= prec + 3,
        Err(_) => false,
    }
}

// e^x, rounded to `prec` bits with `rm`.
pub(crate) fn exp_helper(
    x: Cow<'_, BigExponentFloat>,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    assert_ne!(prec, 0);
    let (sign, exponent) = match &*x {
        BigExponentFloat(NaN) => return (BigExponentFloat::NAN, Equal),
        BigExponentFloat(Infinity { sign: true }) => return (BigExponentFloat::INFINITY, Equal),
        BigExponentFloat(Infinity { sign: false }) => return (BigExponentFloat::ZERO, Equal),
        BigExponentFloat(Zero) => return (BigExponentFloat::ONE, Equal),
        BigExponentFloat(Finite { sign, exponent, .. }) => (*sign, exponent),
    };
    assert_ne!(rm, Exact, "Inexact BigExponentFloat exp");
    // If |x| < 2^(-prec - 2), e^x is 1 plus or minus less than 2^(-prec - 1).
    if *exponent <= -Integer::from(prec + 2) {
        return round_nudged(&BigExponentFloat::ONE, sign, prec, rm);
    }
    if let Ok(e) = i32::try_from(exponent)
        && (Float::MIN_EXPONENT..=SAFE_EXPONENT).contains(&e)
    {
        return from_float_result(finite_to_float(x, e).exp_prec_round(prec, rm), None).unwrap();
    }
    if *exponent < 0u32 {
        // x is below `Float`'s exponent range, and prec is huge.
        return ziv_loop(prec, rm, |wp| exp_small_approx(&x, wp));
    }
    let e = u64::exact_from(exponent);
    exp_huge(&ExpArgument::BigExponentFloat(&x), e, prec, rm, false)
}

// e^x for a `Rational` x, rounded to `prec` bits with `rm`.
pub(crate) fn exp_rational_helper(
    x: &Rational,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    assert_ne!(prec, 0);
    if *x == 0u32 {
        return (BigExponentFloat::ONE, Equal);
    }
    assert_ne!(rm, Exact, "Inexact BigExponentFloat exp");
    let e = x.floor_log_base_2_abs() + 1;
    if e <= SAFE_EXPONENT_I64 {
        from_float_result(Float::exp_rational_prec_round_ref(x, prec, rm), None).unwrap()
    } else {
        exp_huge(
            &ExpArgument::Rational(x),
            u64::exact_from(e),
            prec,
            rm,
            false,
        )
    }
}

impl BigExponentFloat {
    /// Computes $e^x$, the exponential of a [`BigExponentFloat`], rounding the result to the
    /// specified precision and with the specified rounding mode. The [`BigExponentFloat`] is taken
    /// by value. An [`Ordering`] is also returned, indicating whether the rounded exponential is
    /// less than, equal to, or greater than the exact exponential. Although `NaN`s are not
    /// comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = e^x+\varepsilon.
    /// $$
    /// - If $e^x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $e^x$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 e^x\rfloor-p+1}$.
    /// - If $e^x$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 e^x\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=\text{NaN}$
    /// - $f(\infty,p,m)=\infty$
    /// - $f(-\infty,p,m)=0$
    /// - $f(0,p,m)=1$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::exp_prec_round), this function never
    /// overflows or underflows.
    ///
    /// If you know you'll be using `Nearest`, consider using [`BigExponentFloat::exp_prec`]
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{3/2} \log n \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(prec, self.significant_bits())`,
    /// and $m$ is the number of significant bits of the integer part of $x$.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with `prec` bits (which is the case for every finite nonzero input).
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (exp, o) = BigExponentFloat::from(PI).exp_prec_round(5, Floor);
    /// assert_eq!(exp.to_string(), "23.0");
    /// assert_eq!(o, Less);
    ///
    /// let (exp, o) = BigExponentFloat::from(PI).exp_prec_round(5, Ceiling);
    /// assert_eq!(exp.to_string(), "24.0");
    /// assert_eq!(o, Greater);
    ///
    /// let (exp, o) = BigExponentFloat::from(PI).exp_prec_round(5, Nearest);
    /// assert_eq!(exp.to_string(), "23.0");
    /// assert_eq!(o, Less);
    ///
    /// let (exp, o) = BigExponentFloat::from(PI).exp_prec_round(20, Floor);
    /// assert_eq!(exp.to_string(), "23.14069");
    /// assert_eq!(o, Less);
    ///
    /// let (exp, o) = BigExponentFloat::from(PI).exp_prec_round(20, Ceiling);
    /// assert_eq!(exp.to_string(), "23.140717");
    /// assert_eq!(o, Greater);
    ///
    /// let (exp, o) = BigExponentFloat::from(PI).exp_prec_round(20, Nearest);
    /// assert_eq!(exp.to_string(), "23.14069");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn exp_prec_round(self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        exp_helper(Cow::Owned(self), prec, rm)
    }

    /// Computes $e^x$, the exponential of a [`BigExponentFloat`], rounding the result to the
    /// specified precision and with the specified rounding mode. The [`BigExponentFloat`] is taken
    /// by reference. An [`Ordering`] is also returned, indicating whether the rounded exponential
    /// is less than, equal to, or greater than the exact exponential. Although `NaN`s are not
    /// comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = e^x+\varepsilon.
    /// $$
    /// - If $e^x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $e^x$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 e^x\rfloor-p+1}$.
    /// - If $e^x$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 e^x\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=\text{NaN}$
    /// - $f(\infty,p,m)=\infty$
    /// - $f(-\infty,p,m)=0$
    /// - $f(0,p,m)=1$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::exp_prec_round), this function never
    /// overflows or underflows.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{3/2} \log n \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(prec, self.significant_bits())`,
    /// and $m$ is the number of significant bits of the integer part of $x$.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with `prec` bits (which is the case for every finite nonzero input).
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (exp, o) = (&BigExponentFloat::from(PI)).exp_prec_round_ref(5, Floor);
    /// assert_eq!(exp.to_string(), "23.0");
    /// assert_eq!(o, Less);
    ///
    /// let (exp, o) = (&BigExponentFloat::from(PI)).exp_prec_round_ref(5, Ceiling);
    /// assert_eq!(exp.to_string(), "24.0");
    /// assert_eq!(o, Greater);
    ///
    /// let (exp, o) = (&BigExponentFloat::from(PI)).exp_prec_round_ref(5, Nearest);
    /// assert_eq!(exp.to_string(), "23.0");
    /// assert_eq!(o, Less);
    ///
    /// let (exp, o) = (&BigExponentFloat::from(PI)).exp_prec_round_ref(20, Floor);
    /// assert_eq!(exp.to_string(), "23.14069");
    /// assert_eq!(o, Less);
    ///
    /// let (exp, o) = (&BigExponentFloat::from(PI)).exp_prec_round_ref(20, Ceiling);
    /// assert_eq!(exp.to_string(), "23.140717");
    /// assert_eq!(o, Greater);
    ///
    /// let (exp, o) = (&BigExponentFloat::from(PI)).exp_prec_round_ref(20, Nearest);
    /// assert_eq!(exp.to_string(), "23.14069");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn exp_prec_round_ref(&self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        exp_helper(Cow::Borrowed(self), prec, rm)
    }

    /// Computes $e^x$, the exponential of a [`BigExponentFloat`], in place, rounding the result to
    /// the specified precision and with the specified rounding mode. An [`Ordering`] is returned,
    /// indicating whether the rounded exponential is less than, equal to, or greater than the exact
    /// exponential. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this
    /// function sets the [`BigExponentFloat`] to `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets e^x+\varepsilon.
    /// $$
    /// - If $e^x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $e^x$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 e^x\rfloor-p+1}$.
    /// - If $e^x$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 e^x\rfloor-p}$.
    ///
    /// Unlike [its `Float` counterpart](crate::Float::exp_prec_round), this function never
    /// overflows or underflows.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{3/2} \log n \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(prec, self.significant_bits())`,
    /// and $m$ is the number of significant bits of the integer part of $x$.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with `prec` bits (which is the case for every finite nonzero input).
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(x.exp_prec_round_assign(5, Floor), Less);
    /// assert_eq!(x.to_string(), "23.0");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(x.exp_prec_round_assign(5, Ceiling), Greater);
    /// assert_eq!(x.to_string(), "24.0");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(x.exp_prec_round_assign(5, Nearest), Less);
    /// assert_eq!(x.to_string(), "23.0");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(x.exp_prec_round_assign(20, Floor), Less);
    /// assert_eq!(x.to_string(), "23.14069");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(x.exp_prec_round_assign(20, Ceiling), Greater);
    /// assert_eq!(x.to_string(), "23.140717");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(x.exp_prec_round_assign(20, Nearest), Less);
    /// assert_eq!(x.to_string(), "23.14069");
    /// ```
    #[inline]
    pub fn exp_prec_round_assign(&mut self, prec: u64, rm: RoundingMode) -> Ordering {
        let x = core::mem::replace(self, Self::ZERO);
        let (result, o) = exp_helper(Cow::Owned(x), prec, rm);
        *self = result;
        o
    }

    /// Computes $e^x$, the exponential of a [`BigExponentFloat`], rounding the result to the
    /// nearest value of the specified precision. The [`BigExponentFloat`] is taken by value. An
    /// [`Ordering`] is also returned, indicating whether the rounded exponential is less than,
    /// equal to, or greater than the exact exponential. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,p) = e^x+\varepsilon.
    /// $$
    /// - If $e^x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $e^x$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2 e^x\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p)=\text{NaN}$
    /// - $f(\infty,p)=\infty$
    /// - $f(-\infty,p)=0$
    /// - $f(0,p)=1$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::exp_prec_round), this function never
    /// overflows or underflows.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`BigExponentFloat::exp_prec_round`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{3/2} \log n \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(prec, self.significant_bits())`,
    /// and $m$ is the number of significant bits of the integer part of $x$.
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
    /// let (exp, o) = BigExponentFloat::from(PI).exp_prec(5);
    /// assert_eq!(exp.to_string(), "23.0");
    /// assert_eq!(o, Less);
    ///
    /// let (exp, o) = BigExponentFloat::from(PI).exp_prec(20);
    /// assert_eq!(exp.to_string(), "23.14069");
    /// assert_eq!(o, Less);
    ///
    /// let (exp, o) = BigExponentFloat::from(1.0e10).exp_prec(10);
    /// assert_eq!(exp.to_string(), "1.0782e4342944819");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn exp_prec(self, prec: u64) -> (Self, Ordering) {
        exp_helper(Cow::Owned(self), prec, Nearest)
    }

    /// Computes $e^x$, the exponential of a [`BigExponentFloat`], rounding the result to the
    /// nearest value of the specified precision. The [`BigExponentFloat`] is taken by reference. An
    /// [`Ordering`] is also returned, indicating whether the rounded exponential is less than,
    /// equal to, or greater than the exact exponential. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,p) = e^x+\varepsilon.
    /// $$
    /// - If $e^x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $e^x$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2 e^x\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p)=\text{NaN}$
    /// - $f(\infty,p)=\infty$
    /// - $f(-\infty,p)=0$
    /// - $f(0,p)=1$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::exp_prec_round), this function never
    /// overflows or underflows.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{3/2} \log n \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(prec, self.significant_bits())`,
    /// and $m$ is the number of significant bits of the integer part of $x$.
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
    /// let (exp, o) = (&BigExponentFloat::from(PI)).exp_prec_ref(5);
    /// assert_eq!(exp.to_string(), "23.0");
    /// assert_eq!(o, Less);
    ///
    /// let (exp, o) = (&BigExponentFloat::from(PI)).exp_prec_ref(20);
    /// assert_eq!(exp.to_string(), "23.14069");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn exp_prec_ref(&self, prec: u64) -> (Self, Ordering) {
        exp_helper(Cow::Borrowed(self), prec, Nearest)
    }

    /// Computes $e^x$, the exponential of a [`BigExponentFloat`], in place, rounding the result to
    /// the nearest value of the specified precision. An [`Ordering`] is returned, indicating
    /// whether the rounded exponential is less than, equal to, or greater than the exact
    /// exponential. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this
    /// function sets the [`BigExponentFloat`] to `NaN` it also returns `Equal`.
    ///
    /// $$
    /// x \gets e^x+\varepsilon.
    /// $$
    /// - If $e^x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $e^x$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2 e^x\rfloor-p}$.
    ///
    /// Unlike [its `Float` counterpart](crate::Float::exp_prec_round), this function never
    /// overflows or underflows.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{3/2} \log n \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(prec, self.significant_bits())`,
    /// and $m$ is the number of significant bits of the integer part of $x$.
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
    /// assert_eq!(x.exp_prec_assign(5), Less);
    /// assert_eq!(x.to_string(), "23.0");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(x.exp_prec_assign(20), Less);
    /// assert_eq!(x.to_string(), "23.14069");
    /// ```
    #[inline]
    pub fn exp_prec_assign(&mut self, prec: u64) -> Ordering {
        let x = core::mem::replace(self, Self::ZERO);
        let (result, o) = exp_helper(Cow::Owned(x), prec, Nearest);
        *self = result;
        o
    }

    #[allow(clippy::needless_pass_by_value)]
    /// Computes $e^x$, the exponential of a [`Rational`], rounding the result to the specified
    /// precision and with the specified rounding mode and returning the result as a
    /// [`BigExponentFloat`]. The [`Rational`] is taken by value. An [`Ordering`] is also returned,
    /// indicating whether the rounded exponential is less than, equal to, or greater than the exact
    /// exponential.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = e^x+\varepsilon.
    /// $$
    /// - If $e^x$ is zero, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $e^x$ is nonzero, and $m$ is not `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2
    ///   e^x\rfloor-p+1}$.
    /// - If $e^x$ is nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   e^x\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(0,p,m)=1$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::exp_rational_prec_round), this function never
    /// overflows or underflows.
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`BigExponentFloat::exp_rational_prec`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{3/2} \log n \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(prec, x.significant_bits())`, and
    /// $m$ is the number of significant bits of the integer part of $x$.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with `prec` bits (which is the case for every nonzero input).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (exp, o) =
    ///     BigExponentFloat::exp_rational_prec_round(Rational::from_unsigneds(3u8, 5), 5, Floor);
    /// assert_eq!(exp.to_string(), "1.81");
    /// assert_eq!(o, Less);
    ///
    /// let (exp, o) =
    ///     BigExponentFloat::exp_rational_prec_round(Rational::from_unsigneds(3u8, 5), 5, Ceiling);
    /// assert_eq!(exp.to_string(), "1.88");
    /// assert_eq!(o, Greater);
    ///
    /// let (exp, o) =
    ///     BigExponentFloat::exp_rational_prec_round(Rational::from_unsigneds(3u8, 5), 5, Nearest);
    /// assert_eq!(exp.to_string(), "1.81");
    /// assert_eq!(o, Less);
    ///
    /// let (exp, o) =
    ///     BigExponentFloat::exp_rational_prec_round(Rational::from_unsigneds(3u8, 5), 20, Floor);
    /// assert_eq!(exp.to_string(), "1.8221188");
    /// assert_eq!(o, Less);
    ///
    /// let (exp, o) = BigExponentFloat::exp_rational_prec_round(
    ///     Rational::from_unsigneds(3u8, 5),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(exp.to_string(), "1.822121");
    /// assert_eq!(o, Greater);
    ///
    /// let (exp, o) = BigExponentFloat::exp_rational_prec_round(
    ///     Rational::from_unsigneds(3u8, 5),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(exp.to_string(), "1.8221188");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn exp_rational_prec_round(x: Rational, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        exp_rational_helper(&x, prec, rm)
    }

    /// Computes $e^x$, the exponential of a [`Rational`], rounding the result to the specified
    /// precision and with the specified rounding mode and returning the result as a
    /// [`BigExponentFloat`]. The [`Rational`] is taken by reference. An [`Ordering`] is also
    /// returned, indicating whether the rounded exponential is less than, equal to, or greater than
    /// the exact exponential.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = e^x+\varepsilon.
    /// $$
    /// - If $e^x$ is zero, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $e^x$ is nonzero, and $m$ is not `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2
    ///   e^x\rfloor-p+1}$.
    /// - If $e^x$ is nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   e^x\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(0,p,m)=1$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::exp_rational_prec_round), this function never
    /// overflows or underflows.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{3/2} \log n \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(prec, x.significant_bits())`, and
    /// $m$ is the number of significant bits of the integer part of $x$.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with `prec` bits (which is the case for every nonzero input).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (exp, o) = BigExponentFloat::exp_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(3u8, 5),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(exp.to_string(), "1.81");
    /// assert_eq!(o, Less);
    ///
    /// let (exp, o) = BigExponentFloat::exp_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(3u8, 5),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(exp.to_string(), "1.88");
    /// assert_eq!(o, Greater);
    ///
    /// let (exp, o) = BigExponentFloat::exp_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(3u8, 5),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(exp.to_string(), "1.81");
    /// assert_eq!(o, Less);
    ///
    /// let (exp, o) = BigExponentFloat::exp_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(3u8, 5),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(exp.to_string(), "1.8221188");
    /// assert_eq!(o, Less);
    ///
    /// let (exp, o) = BigExponentFloat::exp_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(3u8, 5),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(exp.to_string(), "1.822121");
    /// assert_eq!(o, Greater);
    ///
    /// let (exp, o) = BigExponentFloat::exp_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(3u8, 5),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(exp.to_string(), "1.8221188");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn exp_rational_prec_round_ref(
        x: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        exp_rational_helper(x, prec, rm)
    }

    #[allow(clippy::needless_pass_by_value)]
    /// Computes $e^x$, the exponential of a [`Rational`], rounding the result to the nearest value
    /// of the specified precision and returning the result as a [`BigExponentFloat`]. The
    /// [`Rational`] is taken by value. An [`Ordering`] is also returned, indicating whether the
    /// rounded exponential is less than, equal to, or greater than the exact exponential.
    ///
    /// $$
    /// f(x,p) = e^x+\varepsilon.
    /// $$
    /// - If $e^x$ is zero, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $e^x$ is nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2 e^x\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(0,p)=1$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::exp_rational_prec_round), this function never
    /// overflows or underflows.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`BigExponentFloat::exp_rational_prec_round`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{3/2} \log n \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(prec, x.significant_bits())`, and
    /// $m$ is the number of significant bits of the integer part of $x$.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (exp, o) = BigExponentFloat::exp_rational_prec(Rational::from_unsigneds(3u8, 5), 5);
    /// assert_eq!(exp.to_string(), "1.81");
    /// assert_eq!(o, Less);
    ///
    /// let (exp, o) = BigExponentFloat::exp_rational_prec(Rational::from_unsigneds(3u8, 5), 20);
    /// assert_eq!(exp.to_string(), "1.8221188");
    /// assert_eq!(o, Less);
    ///
    /// let (exp, o) =
    ///     BigExponentFloat::exp_rational_prec(Rational::from_unsigneds(30000000001u64, 3), 10);
    /// assert_eq!(exp.to_string(), "1.504e4342944819");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn exp_rational_prec(x: Rational, prec: u64) -> (Self, Ordering) {
        exp_rational_helper(&x, prec, Nearest)
    }

    /// Computes $e^x$, the exponential of a [`Rational`], rounding the result to the nearest value
    /// of the specified precision and returning the result as a [`BigExponentFloat`]. The
    /// [`Rational`] is taken by reference. An [`Ordering`] is also returned, indicating whether the
    /// rounded exponential is less than, equal to, or greater than the exact exponential.
    ///
    /// $$
    /// f(x,p) = e^x+\varepsilon.
    /// $$
    /// - If $e^x$ is zero, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $e^x$ is nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2 e^x\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(0,p)=1$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::exp_rational_prec_round), this function never
    /// overflows or underflows.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{3/2} \log n \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(prec, x.significant_bits())`, and
    /// $m$ is the number of significant bits of the integer part of $x$.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (exp, o) =
    ///     BigExponentFloat::exp_rational_prec_ref(&Rational::from_unsigneds(3u8, 5), 5);
    /// assert_eq!(exp.to_string(), "1.81");
    /// assert_eq!(o, Less);
    ///
    /// let (exp, o) =
    ///     BigExponentFloat::exp_rational_prec_ref(&Rational::from_unsigneds(3u8, 5), 20);
    /// assert_eq!(exp.to_string(), "1.8221188");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn exp_rational_prec_ref(x: &Rational, prec: u64) -> (Self, Ordering) {
        exp_rational_helper(x, prec, Nearest)
    }
}
