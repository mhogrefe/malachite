// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the GNU MPFR Library.
//
//      Copyright 2001-2025 Free Software Foundation, Inc.
//
//      Contributed by the Pascaline and Caramba projects, INRIA.
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::InnerFloat::{Finite, Infinity, NaN, Zero};
use crate::arithmetic::exp::{exp_overflow, exp_rational_near_one, exp_underflow, one_neighbor};
use crate::arithmetic::round_near_x::float_round_near_x;
use crate::{Float, emulate_float_to_float_fn, emulate_rational_to_float_fn, floor_and_ceiling};
use core::cmp::Ordering::{self, *};
use malachite_base::num::arithmetic::traits::{CeilingLogBase2, PowerOf2, PowerOf2Assign, Sign};
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::{
    Infinity as InfinityTrait, NaN as NaNTrait, One, Zero as ZeroTrait,
};
use malachite_base::num::conversion::traits::{ExactFrom, IsInteger, RoundingFrom};
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_nz::integer::Integer;
use malachite_nz::natural::arithmetic::float_extras::float_can_round;
use malachite_nz::platform::{Limb, SignedLimb};
use malachite_q::Rational;

fn power_of_2_of_float_prec_round_normal_helper(
    xfrac: &Float,
    xint: i64,
    precy: u64,
    rm: RoundingMode,
) -> (Float, Ordering) {
    // For tiny xfrac, 2^xfrac is very close to 1 (above it if xfrac > 0, below if xfrac < 0), with
    // |2^xfrac - 1| < |xfrac| < 2^EXP(xfrac). Round it from 1 directly when possible: otherwise the
    // `exp` below would balloon its own working precision to ~ -EXP(xfrac) (up to ~2^30) just to
    // resolve the rounding of 1 + tiny. This is the `power_of_2_rational_near_one` fast path,
    // applied to the `Float` case.
    let ex = i64::from(xfrac.get_exponent().unwrap());
    if let Some((mut y, o)) = float_round_near_x(
        &Float::ONE,
        u64::exact_from(1 - ex),
        *xfrac > 0u32,
        precy,
        rm,
    ) {
        // Multiply by 2^xint. `y` is already rounded to `precy`, and `o` already compares it to the
        // exact 2^xfrac, so the shift helper is called directly with that ternary: it adjusts the
        // exponent, substituting the correct overflow or underflow result if the shift leaves the
        // valid exponent range.
        let o = y.shl_prec_round_assign_helper(xint, precy, rm, o);
        return (y, o);
    }
    let mut working_prec = precy + 5 + precy.ceiling_log_base_2();
    let mut increment = Limb::WIDTH;
    loop {
        let ln_2 = Float::ln_2_prec_round(working_prec, Up).0;
        let mut t = xfrac.mul_prec_round_ref_val(ln_2, working_prec, Up).0; // xfrac * ln(2)
        // Error estimate (cf. mpfr_exp2): the relative error of t (computed with two roundings) is
        // bounded so that exp(t) is correct to `err` bits.
        let err = u64::exact_from(
            i64::exact_from(working_prec) - (i64::from(t.get_exponent().unwrap()) + 2),
        );
        t.exp_prec_assign(working_prec); // exp(xfrac * ln(2))
        if float_can_round(t.significand_ref().unwrap(), err, precy, rm) {
            // Round to `precy` and multiply by 2^xint. MPFR performs the multiplication in an
            // extended exponent range and applies the range reduction in mpfr_check_range;
            // `shl_prec_round` provides the same overflow and underflow handling here. In
            // particular, when `Nearest` rounds 2^xfrac down to exactly 1/2 and xint = MIN_EXPONENT
            // - 1, the shifted value is the midpoint between 0 and the smallest positive Float, but
            // the rounding's ternary shows that the exact value lies above the midpoint, so the
            // result rounds up to that smallest value rather than underflowing to zero.
            return t.shl_prec_round(xint, precy, rm);
        }
        working_prec += increment;
        increment = working_prec >> 1;
    }
}

// This is mpfr_exp2 from exp2.c, MPFR 4.2.2, where the input is finite and nonzero and the float is
// taken by reference.
fn power_of_2_of_float_prec_round_normal(
    x: &Float,
    precy: u64,
    rm: RoundingMode,
) -> (Float, Ordering) {
    // 2^x overflows once x >= MAX_EXPONENT, and underflows once x <= MIN_EXPONENT - 2 (the smallest
    // representable positive value is 2^(MIN_EXPONENT - 1)).
    if *x >= const { Float::const_from_signed(Float::MAX_EXPONENT as SignedLimb) } {
        return exp_overflow(precy, rm);
    }
    if *x <= const { Float::const_from_signed((Float::MIN_EXPONENT as SignedLimb) - 2) } {
        return exp_underflow(precy, rm);
    }
    // We now know that MIN_EXPONENT - 2 < x < MAX_EXPONENT, so the integer part fits in an i64.
    let xint = i64::exact_from(&Integer::rounding_from(x, Down).0); // trunc(x), toward zero
    // If x is an integer, 2^x is a power of 2, hence exact.
    if x.is_integer() {
        return Float::power_of_2_prec_round(xint, precy, rm);
    }
    // 2^x for a non-integer Float is transcendental, hence never exactly representable.
    assert_ne!(rm, Exact, "Inexact power_of_2_of_float");
    // 2^x = 2^xint * 2^xfrac, where xfrac = x - xint and |xfrac| < 1. We compute 2^xfrac =
    // exp(xfrac * ln(2)) and then multiply by 2^xint by shifting the result's exponent.
    let p = x.get_prec().unwrap();
    if xint == 0 {
        power_of_2_of_float_prec_round_normal_helper(x, 0, precy, rm)
    } else {
        // x - xint is exact: the difference has fewer significant bits than x.
        let xint_f = Float::from_integer_prec(Integer::from(xint), p).0;
        let xfrac = x.sub_prec_round_ref_val(xint_f, p, Floor).0;
        power_of_2_of_float_prec_round_normal_helper(&xfrac, xint, precy, rm)
    }
}

// Computes `2 ^ x` for a nonzero `Rational` `x` with MPFR-style exponent `exp_x = floor(log2|x|) +
// 1 <= MIN_EXPONENT`, so `|x| < 2^MIN_EXPONENT` and `x` is too small to be a normal `Float` (the
// squeeze in `power_of_2_rational_helper` cannot bracket it). Then `2 ^ x` is extremely close to 1:
// `0 < |2^x - 1| < |x| < 2^exp_x = 2^(EXP(1) - (1 - exp_x))`, above 1 if `x > 0` and below it if `x
// < 0`.
//
// As a fast path, `float_round_near_x` rounds `2 ^ x` from 1 alone (no evaluation of `2 ^ x`)
// whenever `prec < -exp_x`. Otherwise we compute it: `2 ^ x = exp(x * ln(2))`, so bracketing
// `ln(2)` between two `Rational`s and applying `exp_rational_near_one` to each product brackets `2
// ^ x`. The key point is that the needed `ln(2)` precision is only about `prec - (-exp_x)` bits,
// not `prec`: `x` is so tiny that the bracket `x * (ln_2_hi - ln_2_lo)` shrinks far faster than the
// result's ulp. So `ln_2_prec_round` is called at a modest precision, never near the `~2^30`
// ceiling where it would overflow.
fn power_of_2_rational_near_one(
    x: &Rational,
    exp_x: i64,
    prec: u64,
    rm: RoundingMode,
) -> (Float, Ordering) {
    let above = x.sign() == Greater;
    let err = u64::exact_from(1 - exp_x);
    if let Some(result) = float_round_near_x(&Float::ONE, err, above, prec, rm) {
        return result;
    }
    // prec >= -exp_x. ln(2) needs roughly `prec - (-exp_x)` bits to separate the two products at
    // the target precision; start a little above that and let the Ziv loop grow it.
    let mut working_prec = (prec - u64::exact_from(-exp_x)) + Limb::WIDTH;
    let mut increment = Limb::WIDTH;
    loop {
        // ln_2_lo <= ln(2) <= ln_2_hi, as exact Rationals, from a single ln(2) computation.
        let (ln_2_lo, ln_2_hi) = floor_and_ceiling(Float::ln_2_prec_round(working_prec, Floor));
        let ln_2_lo = Rational::exact_from(&ln_2_lo);
        let ln_2_hi = Rational::exact_from(&ln_2_hi);
        // x * ln(2) lies between x * ln_2_lo and x * ln_2_hi, and exp is increasing, so 2 ^ x lies
        // between exp of these two products.
        let (lo, o_lo) = exp_rational_near_one(&(x * ln_2_lo), prec, rm);
        let (hi, o_hi) = exp_rational_near_one(&(x * ln_2_hi), prec, rm);
        if o_lo == o_hi && lo == hi {
            return (lo, o_lo);
        }
        working_prec += increment;
        increment = working_prec >> 1;
    }
}

// Computes `2 ^ x` for a non-integer `Rational` `x`, rounded to precision `prec` with rounding mode
// `rm`. (Integer `x`, including 0, is handled by the caller, where `2 ^ x` is an exact power of 2.)
// `2 ^ x` for a non-integer `x` is transcendental, hence never exactly representable, so `rm` must
// not be `Exact`.
fn power_of_2_rational_helper(x: &Rational, prec: u64, rm: RoundingMode) -> (Float, Ordering) {
    assert_ne!(rm, Exact, "Inexact power_of_2");
    let positive = x.sign() == Greater;
    let exp_x = x.floor_log_base_2_abs() + 1; // the MPFR-style exponent of x
    // |x| is too large to be a finite Float, so 2^x overflows (x > 0) or underflows (x < 0).
    // Smaller x that still overflow/underflow are caught by `power_of_2_of_float_prec_round_normal`
    // in the loop below.
    if exp_x >= const { Float::MAX_EXPONENT as i64 } {
        return if positive {
            exp_overflow(prec, rm)
        } else {
            exp_underflow(prec, rm)
        };
    }
    // x is too small to be represented as a normal Float (|x| < 2^MIN_EXPONENT). The squeeze below
    // cannot bracket it, so round 2^x directly from 1 instead.
    if exp_x <= const { Float::MIN_EXPONENT as i64 } {
        return power_of_2_rational_near_one(x, exp_x, prec, rm);
    }
    // Tiny x: if |x| < 2^(-prec) then 2^x is within half an ulp of 1, so it rounds to 1 (or, for
    // directed rounding away from 1, to the neighbor of 1). This mirrors the tiny-x fast path of
    // exp.
    if -exp_x > i64::exact_from(prec) {
        return match (positive, rm) {
            (false, Down | Floor) => (one_neighbor(prec, false), Less), // 1 - ulp
            (true, Up | Ceiling) => (one_neighbor(prec, true), Greater), // 1 + ulp
            (true, _) => (Float::one_prec(prec), Less),
            (false, _) => (Float::one_prec(prec), Greater),
        };
    }
    // General case: bracket x between the Floats x_lo <= x <= x_hi, raise 2 to both, and increase
    // the working precision until the two bounds round to the same result. 2^x is monotonic, so
    // once the bounds agree the exact 2^x (which lies between them) rounds the same way.
    let mut working_prec = prec + 10;
    let mut increment = Limb::WIDTH;
    loop {
        let (x_lo, x_o) = Float::from_rational_prec_round_ref(x, working_prec, Floor);
        if x_o == Equal {
            // x (a non-integer dyadic rational) is exactly representable at `working_prec`, so 2^x
            // is simply 2^x_lo, computed by `power_of_2_of_float_prec_round_normal`.
            return power_of_2_of_float_prec_round_normal(&x_lo, prec, rm);
        }
        let (x_lo, x_hi) = floor_and_ceiling((x_lo, x_o));
        let (e_lo, o_lo) = power_of_2_of_float_prec_round_normal(&x_lo, prec, rm);
        let (e_hi, o_hi) = power_of_2_of_float_prec_round_normal(&x_hi, prec, rm);
        if o_lo == o_hi && e_lo == e_hi {
            return (e_lo, o_lo);
        }
        working_prec += increment;
        increment = working_prec >> 1;
    }
}

impl Float {
    #[allow(clippy::needless_pass_by_value)]
    /// Computes $2^x$, where $x$ is a [`Float`], rounding the result to the specified precision and
    /// with the specified rounding mode. The [`Float`] is taken by value. An [`Ordering`] is also
    /// returned, indicating whether the rounded power is less than, equal to, or greater than the
    /// exact power. Although `NaN`s are not comparable to any [`Float`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = 2^x+\varepsilon.
    /// $$
    /// - If $2^x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $2^x$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 2^x\rfloor-p+1}$.
    /// - If $2^x$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 2^x\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=\text{NaN}$
    /// - $f(\infty,p,m)=\infty$
    /// - $f(-\infty,p,m)=0.0$
    /// - $f(\pm0.0,p,m)=1.0$
    ///
    /// Overflow and underflow:
    /// - If $f(x,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$ is
    ///   returned instead.
    /// - If $f(x,p,m)<2^{-2^{30}}$ and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $f(x,p,m)<2^{-2^{30}}$ and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned instead.
    /// - If $f(x,p,m)\leq2^{-2^{30}-1}$ and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,p,m)<2^{-2^{30}}$ and $m$ is `Nearest`, $2^{-2^{30}}$ is returned
    ///   instead.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::power_of_2_of_float_prec`]
    /// instead. If you know that your target precision is the precision of the input, consider
    /// using [`Float::power_of_2_of_float_round`] instead. If both of these things are true,
    /// consider using the [`PowerOf2`] implementation instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with the given precision.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (p, o) = Float::power_of_2_of_float_prec_round(Float::from(1.5), 5, Floor);
    /// assert_eq!(p.to_string(), "2.8");
    /// assert_eq!(o, Less);
    ///
    /// let (p, o) = Float::power_of_2_of_float_prec_round(Float::from(1.5), 5, Ceiling);
    /// assert_eq!(p.to_string(), "2.9");
    /// assert_eq!(o, Greater);
    ///
    /// let (p, o) = Float::power_of_2_of_float_prec_round(Float::from(1.5), 5, Nearest);
    /// assert_eq!(p.to_string(), "2.9");
    /// assert_eq!(o, Greater);
    ///
    /// let (p, o) = Float::power_of_2_of_float_prec_round(Float::from(1.5), 20, Floor);
    /// assert_eq!(p.to_string(), "2.828426");
    /// assert_eq!(o, Less);
    ///
    /// let (p, o) = Float::power_of_2_of_float_prec_round(Float::from(1.5), 20, Ceiling);
    /// assert_eq!(p.to_string(), "2.82843");
    /// assert_eq!(o, Greater);
    ///
    /// let (p, o) = Float::power_of_2_of_float_prec_round(Float::from(1.5), 20, Nearest);
    /// assert_eq!(p.to_string(), "2.828426");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn power_of_2_of_float_prec_round(
        pow: Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        Self::power_of_2_of_float_prec_round_ref(&pow, prec, rm)
    }

    /// Computes $2^x$, where $x$ is a [`Float`], rounding the result to the specified precision and
    /// with the specified rounding mode. The [`Float`] is taken by reference. An [`Ordering`] is
    /// also returned, indicating whether the rounded power is less than, equal to, or greater than
    /// the exact power. Although `NaN`s are not comparable to any [`Float`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = 2^x+\varepsilon.
    /// $$
    /// - If $2^x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $2^x$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 2^x\rfloor-p+1}$.
    /// - If $2^x$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 2^x\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=\text{NaN}$
    /// - $f(\infty,p,m)=\infty$
    /// - $f(-\infty,p,m)=0.0$
    /// - $f(\pm0.0,p,m)=1.0$
    ///
    /// Overflow and underflow:
    /// - If $f(x,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$ is
    ///   returned instead.
    /// - If $f(x,p,m)<2^{-2^{30}}$ and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $f(x,p,m)<2^{-2^{30}}$ and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned instead.
    /// - If $f(x,p,m)\leq2^{-2^{30}-1}$ and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,p,m)<2^{-2^{30}}$ and $m$ is `Nearest`, $2^{-2^{30}}$ is returned
    ///   instead.
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`Float::power_of_2_of_float_prec_ref`] instead. If you know that your target precision is
    /// the precision of the input, consider using [`Float::power_of_2_of_float_round_ref`] instead.
    /// If both of these things are true, consider using the [`PowerOf2`] implementation instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with the given precision.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(1.5);
    ///
    /// let (p, o) = Float::power_of_2_of_float_prec_round_ref(&x, 5, Floor);
    /// assert_eq!(p.to_string(), "2.8");
    /// assert_eq!(o, Less);
    ///
    /// let (p, o) = Float::power_of_2_of_float_prec_round_ref(&x, 5, Ceiling);
    /// assert_eq!(p.to_string(), "2.9");
    /// assert_eq!(o, Greater);
    ///
    /// let (p, o) = Float::power_of_2_of_float_prec_round_ref(&x, 5, Nearest);
    /// assert_eq!(p.to_string(), "2.9");
    /// assert_eq!(o, Greater);
    ///
    /// let (p, o) = Float::power_of_2_of_float_prec_round_ref(&x, 20, Floor);
    /// assert_eq!(p.to_string(), "2.828426");
    /// assert_eq!(o, Less);
    ///
    /// let (p, o) = Float::power_of_2_of_float_prec_round_ref(&x, 20, Ceiling);
    /// assert_eq!(p.to_string(), "2.82843");
    /// assert_eq!(o, Greater);
    ///
    /// let (p, o) = Float::power_of_2_of_float_prec_round_ref(&x, 20, Nearest);
    /// assert_eq!(p.to_string(), "2.828426");
    /// assert_eq!(o, Less);
    /// ```
    pub fn power_of_2_of_float_prec_round_ref(
        pow: &Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        assert_ne!(prec, 0);
        match &pow.0 {
            NaN => (Self::NAN, Equal),
            // 2^(+inf) = +inf; 2^(-inf) = +0
            Infinity { sign } => {
                if *sign {
                    (Self::INFINITY, Equal)
                } else {
                    (Self::ZERO, Equal)
                }
            }
            // 2^(+0) = 2^(-0) = 1
            Zero { .. } => (Self::one_prec(prec), Equal),
            Finite { .. } => power_of_2_of_float_prec_round_normal(pow, prec, rm),
        }
    }

    #[allow(clippy::needless_pass_by_value)]
    /// Computes $2^x$, where $x$ is a [`Float`], rounding the result to the nearest value of the
    /// specified precision. The [`Float`] is taken by value. An [`Ordering`] is also returned,
    /// indicating whether the rounded power is less than, equal to, or greater than the exact
    /// power. Although `NaN`s are not comparable to any [`Float`], whenever this function returns a
    /// `NaN` it also returns `Equal`.
    ///
    /// If the power is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = 2^x+\varepsilon.
    /// $$
    /// - If $2^x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $2^x$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2 2^x\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p)=\text{NaN}$
    /// - $f(\infty,p)=\infty$
    /// - $f(-\infty,p)=0.0$
    /// - $f(\pm0.0,p)=1.0$
    ///
    /// Overflow and underflow:
    /// - If $f(x,p)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::power_of_2_of_float_prec_round`] instead. If you know that your target precision is
    /// the precision of the input, consider using the [`PowerOf2`] implementation instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (p, o) = Float::power_of_2_of_float_prec(Float::from(1.5), 5);
    /// assert_eq!(p.to_string(), "2.9");
    /// assert_eq!(o, Greater);
    ///
    /// let (p, o) = Float::power_of_2_of_float_prec(Float::from(1.5), 20);
    /// assert_eq!(p.to_string(), "2.828426");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn power_of_2_of_float_prec(pow: Self, prec: u64) -> (Self, Ordering) {
        Self::power_of_2_of_float_prec_round_ref(&pow, prec, Nearest)
    }

    /// Computes $2^x$, where $x$ is a [`Float`], rounding the result to the nearest value of the
    /// specified precision. The [`Float`] is taken by reference. An [`Ordering`] is also returned,
    /// indicating whether the rounded power is less than, equal to, or greater than the exact
    /// power. Although `NaN`s are not comparable to any [`Float`], whenever this function returns a
    /// `NaN` it also returns `Equal`.
    ///
    /// If the power is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = 2^x+\varepsilon.
    /// $$
    /// - If $2^x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $2^x$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2 2^x\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p)=\text{NaN}$
    /// - $f(\infty,p)=\infty$
    /// - $f(-\infty,p)=0.0$
    /// - $f(\pm0.0,p)=1.0$
    ///
    /// Overflow and underflow:
    /// - If $f(x,p)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::power_of_2_of_float_prec_round_ref`] instead. If you know that your target
    /// precision is the precision of the input, consider using the [`PowerOf2`] implementation
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(1.5);
    ///
    /// let (p, o) = Float::power_of_2_of_float_prec_ref(&x, 5);
    /// assert_eq!(p.to_string(), "2.9");
    /// assert_eq!(o, Greater);
    ///
    /// let (p, o) = Float::power_of_2_of_float_prec_ref(&x, 20);
    /// assert_eq!(p.to_string(), "2.828426");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn power_of_2_of_float_prec_ref(pow: &Self, prec: u64) -> (Self, Ordering) {
        Self::power_of_2_of_float_prec_round_ref(pow, prec, Nearest)
    }

    #[allow(clippy::needless_pass_by_value)]
    /// Computes $2^x$, where $x$ is a [`Float`], rounding the result with the specified rounding
    /// mode. The [`Float`] is taken by value. An [`Ordering`] is also returned, indicating whether
    /// the rounded power is less than, equal to, or greater than the exact power. Although `NaN`s
    /// are not comparable to any [`Float`], whenever this function returns a `NaN` it also returns
    /// `Equal`.
    ///
    /// The precision of the output is the precision of the input. See [`RoundingMode`] for a
    /// description of the possible rounding modes.
    ///
    /// $$
    /// f(x,m) = 2^x+\varepsilon.
    /// $$
    /// - If $2^x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $2^x$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 2^x\rfloor-p+1}$, where $p$ is the precision of the input.
    /// - If $2^x$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 2^x\rfloor-p}$, where $p$ is the precision of the input.
    ///
    /// If the output has a precision, it is the precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN},m)=\text{NaN}$
    /// - $f(\infty,m)=\infty$
    /// - $f(-\infty,m)=0.0$
    /// - $f(\pm0.0,m)=1.0$
    ///
    /// See the [`Float::power_of_2_of_float_prec_round`] documentation for information on overflow
    /// and underflow.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::power_of_2_of_float_prec_round`] instead. If you know you'll be using the `Nearest`
    /// rounding mode, consider using the [`PowerOf2`] implementation instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the result cannot be represented exactly with the input
    /// precision.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (p, o) =
    ///     Float::power_of_2_of_float_round(Float::from_unsigned_prec(3u32, 100).0 >> 1u32, Floor);
    /// assert_eq!(p.to_string(), "2.828427124746190097603377448418");
    /// assert_eq!(o, Less);
    ///
    /// let (p, o) = Float::power_of_2_of_float_round(
    ///     Float::from_unsigned_prec(3u32, 100).0 >> 1u32,
    ///     Ceiling,
    /// );
    /// assert_eq!(p.to_string(), "2.828427124746190097603377448422");
    /// assert_eq!(o, Greater);
    ///
    /// let (p, o) = Float::power_of_2_of_float_round(
    ///     Float::from_unsigned_prec(3u32, 100).0 >> 1u32,
    ///     Nearest,
    /// );
    /// assert_eq!(p.to_string(), "2.828427124746190097603377448418");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn power_of_2_of_float_round(pow: Self, rm: RoundingMode) -> (Self, Ordering) {
        let prec = pow.significant_bits();
        Self::power_of_2_of_float_prec_round_ref(&pow, prec, rm)
    }

    /// Computes $2^x$, where $x$ is a [`Float`], rounding the result with the specified rounding
    /// mode. The [`Float`] is taken by reference. An [`Ordering`] is also returned, indicating
    /// whether the rounded power is less than, equal to, or greater than the exact power. Although
    /// `NaN`s are not comparable to any [`Float`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// The precision of the output is the precision of the input. See [`RoundingMode`] for a
    /// description of the possible rounding modes.
    ///
    /// $$
    /// f(x,m) = 2^x+\varepsilon.
    /// $$
    /// - If $2^x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $2^x$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 2^x\rfloor-p+1}$, where $p$ is the precision of the input.
    /// - If $2^x$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 2^x\rfloor-p}$, where $p$ is the precision of the input.
    ///
    /// If the output has a precision, it is the precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN},m)=\text{NaN}$
    /// - $f(\infty,m)=\infty$
    /// - $f(-\infty,m)=0.0$
    /// - $f(\pm0.0,m)=1.0$
    ///
    /// See the [`Float::power_of_2_of_float_prec_round`] documentation for information on overflow
    /// and underflow.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::power_of_2_of_float_prec_round_ref`] instead. If you know you'll be using the
    /// `Nearest` rounding mode, consider using the [`PowerOf2`] implementation instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the result cannot be represented exactly with the input
    /// precision.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from_unsigned_prec(3u32, 100).0 >> 1u32;
    ///
    /// let (p, o) = Float::power_of_2_of_float_round_ref(&x, Floor);
    /// assert_eq!(p.to_string(), "2.828427124746190097603377448418");
    /// assert_eq!(o, Less);
    ///
    /// let (p, o) = Float::power_of_2_of_float_round_ref(&x, Ceiling);
    /// assert_eq!(p.to_string(), "2.828427124746190097603377448422");
    /// assert_eq!(o, Greater);
    ///
    /// let (p, o) = Float::power_of_2_of_float_round_ref(&x, Nearest);
    /// assert_eq!(p.to_string(), "2.828427124746190097603377448418");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn power_of_2_of_float_round_ref(pow: &Self, rm: RoundingMode) -> (Self, Ordering) {
        let prec = pow.significant_bits();
        Self::power_of_2_of_float_prec_round_ref(pow, prec, rm)
    }

    /// Computes $2^x$, where $x$ is a [`Float`], in place, rounding the result to the specified
    /// precision and with the specified rounding mode. An [`Ordering`] is returned, indicating
    /// whether the rounded power is less than, equal to, or greater than the exact power. Although
    /// `NaN`s are not comparable to any [`Float`], whenever this function sets the [`Float`] to
    /// `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets 2^x+\varepsilon.
    /// $$
    /// - If $2^x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $2^x$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 2^x\rfloor-p+1}$.
    /// - If $2^x$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 2^x\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// See the [`Float::power_of_2_of_float_prec_round`] documentation for information on special
    /// cases, overflow, and underflow.
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`Float::power_of_2_of_float_prec_assign`] instead. If you know that your target precision
    /// is the precision of the input, consider using [`Float::power_of_2_of_float_round_assign`]
    /// instead. If both of these things are true, consider using the [`PowerOf2Assign`]
    /// implementation instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with the given precision.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from(1.5);
    /// assert_eq!(x.power_of_2_of_float_prec_round_assign(5, Floor), Less);
    /// assert_eq!(x.to_string(), "2.8");
    ///
    /// let mut x = Float::from(1.5);
    /// assert_eq!(x.power_of_2_of_float_prec_round_assign(5, Ceiling), Greater);
    /// assert_eq!(x.to_string(), "2.9");
    ///
    /// let mut x = Float::from(1.5);
    /// assert_eq!(x.power_of_2_of_float_prec_round_assign(5, Nearest), Greater);
    /// assert_eq!(x.to_string(), "2.9");
    ///
    /// let mut x = Float::from(1.5);
    /// assert_eq!(x.power_of_2_of_float_prec_round_assign(20, Floor), Less);
    /// assert_eq!(x.to_string(), "2.828426");
    ///
    /// let mut x = Float::from(1.5);
    /// assert_eq!(
    ///     x.power_of_2_of_float_prec_round_assign(20, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "2.82843");
    ///
    /// let mut x = Float::from(1.5);
    /// assert_eq!(x.power_of_2_of_float_prec_round_assign(20, Nearest), Less);
    /// assert_eq!(x.to_string(), "2.828426");
    /// ```
    #[inline]
    pub fn power_of_2_of_float_prec_round_assign(
        &mut self,
        prec: u64,
        rm: RoundingMode,
    ) -> Ordering {
        let (result, o) = Self::power_of_2_of_float_prec_round_ref(self, prec, rm);
        *self = result;
        o
    }

    /// Computes $2^x$, where $x$ is a [`Float`], in place, rounding the result to the nearest value
    /// of the specified precision. An [`Ordering`] is returned, indicating whether the rounded
    /// power is less than, equal to, or greater than the exact power. Although `NaN`s are not
    /// comparable to any [`Float`], whenever this function sets the [`Float`] to `NaN` it also
    /// returns `Equal`.
    ///
    /// If the power is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// x \gets 2^x+\varepsilon.
    /// $$
    /// - If $2^x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $2^x$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2 2^x\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// See the [`Float::power_of_2_of_float_prec`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::power_of_2_of_float_prec_round_assign`] instead. If you know that your target
    /// precision is the precision of the input, consider using the [`PowerOf2Assign`]
    /// implementation instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from(1.5);
    /// assert_eq!(x.power_of_2_of_float_prec_assign(5), Greater);
    /// assert_eq!(x.to_string(), "2.9");
    ///
    /// let mut x = Float::from(1.5);
    /// assert_eq!(x.power_of_2_of_float_prec_assign(20), Less);
    /// assert_eq!(x.to_string(), "2.828426");
    /// ```
    #[inline]
    pub fn power_of_2_of_float_prec_assign(&mut self, prec: u64) -> Ordering {
        self.power_of_2_of_float_prec_round_assign(prec, Nearest)
    }

    /// Computes $2^x$, where $x$ is a [`Float`], in place, rounding the result with the specified
    /// rounding mode. An [`Ordering`] is returned, indicating whether the rounded power is less
    /// than, equal to, or greater than the exact power. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function sets the [`Float`] to `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the precision of the input. See [`RoundingMode`] for a
    /// description of the possible rounding modes.
    ///
    /// $$
    /// x \gets 2^x+\varepsilon.
    /// $$
    /// - If $2^x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $2^x$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 2^x\rfloor-p+1}$, where $p$ is the precision of the input.
    /// - If $2^x$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 2^x\rfloor-p}$, where $p$ is the precision of the input.
    ///
    /// If the output has a precision, it is the precision of the input.
    ///
    /// See the [`Float::power_of_2_of_float_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::power_of_2_of_float_prec_round_assign`] instead. If you know you'll be using the
    /// `Nearest` rounding mode, consider using the [`PowerOf2Assign`] implementation instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the result cannot be represented exactly with the input
    /// precision.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from_unsigned_prec(3u32, 100).0 >> 1u32;
    /// assert_eq!(x.power_of_2_of_float_round_assign(Floor), Less);
    /// assert_eq!(x.to_string(), "2.828427124746190097603377448418");
    ///
    /// let mut x = Float::from_unsigned_prec(3u32, 100).0 >> 1u32;
    /// assert_eq!(x.power_of_2_of_float_round_assign(Ceiling), Greater);
    /// assert_eq!(x.to_string(), "2.828427124746190097603377448422");
    ///
    /// let mut x = Float::from_unsigned_prec(3u32, 100).0 >> 1u32;
    /// assert_eq!(x.power_of_2_of_float_round_assign(Nearest), Less);
    /// assert_eq!(x.to_string(), "2.828427124746190097603377448418");
    /// ```
    #[inline]
    pub fn power_of_2_of_float_round_assign(&mut self, rm: RoundingMode) -> Ordering {
        let prec = self.significant_bits();
        self.power_of_2_of_float_prec_round_assign(prec, rm)
    }

    #[allow(clippy::needless_pass_by_value)]
    /// Computes $2^x$, where $x$ is a [`Rational`], rounding the result to the specified precision
    /// and with the specified rounding mode and returning the result as a [`Float`]. The
    /// [`Rational`] is taken by value. An [`Ordering`] is also returned, indicating whether the
    /// rounded power is less than, equal to, or greater than the exact power.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = 2^x+\varepsilon.
    /// $$
    /// - If $m$ is not `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2 2^x\rfloor-p+1}$.
    /// - If $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2 2^x\rfloor-p}$.
    ///
    /// These bounds do not apply when the result overflows or underflows; see below.
    ///
    /// The output has precision `prec`.
    ///
    /// Special cases:
    /// - $f(0,p,m)=1$.
    ///
    /// Overflow and underflow:
    /// - If $f(x,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$ is
    ///   returned instead.
    /// - If $f(x,p,m)<2^{-2^{30}}$ and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $f(x,p,m)<2^{-2^{30}}$ and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned instead.
    /// - If $f(x,p,m)\leq2^{-2^{30}-1}$ and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,p,m)<2^{-2^{30}}$ and $m$ is `Nearest`, $2^{-2^{30}}$ is returned
    ///   instead.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::power_of_2_rational_prec`]
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with the given precision (which is the case whenever $x$ is not an integer).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (p, o) =
    ///     Float::power_of_2_rational_prec_round(Rational::from_unsigneds(3u8, 5), 5, Floor);
    /// assert_eq!(p.to_string(), "1.5");
    /// assert_eq!(o, Less);
    ///
    /// let (p, o) =
    ///     Float::power_of_2_rational_prec_round(Rational::from_unsigneds(3u8, 5), 5, Ceiling);
    /// assert_eq!(p.to_string(), "1.56");
    /// assert_eq!(o, Greater);
    ///
    /// let (p, o) =
    ///     Float::power_of_2_rational_prec_round(Rational::from_unsigneds(3u8, 5), 20, Floor);
    /// assert_eq!(p.to_string(), "1.515717");
    /// assert_eq!(o, Less);
    ///
    /// let (p, o) =
    ///     Float::power_of_2_rational_prec_round(Rational::from_unsigneds(3u8, 5), 20, Ceiling);
    /// assert_eq!(p.to_string(), "1.515718");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn power_of_2_rational_prec_round(
        x: Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        Self::power_of_2_rational_prec_round_ref(&x, prec, rm)
    }

    /// Computes $2^x$, where $x$ is a [`Rational`], rounding the result to the specified precision
    /// and with the specified rounding mode and returning the result as a [`Float`]. The
    /// [`Rational`] is taken by reference. An [`Ordering`] is also returned, indicating whether the
    /// rounded power is less than, equal to, or greater than the exact power.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = 2^x+\varepsilon.
    /// $$
    /// - If $m$ is not `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2 2^x\rfloor-p+1}$.
    /// - If $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2 2^x\rfloor-p}$.
    ///
    /// These bounds do not apply when the result overflows or underflows; see below.
    ///
    /// The output has precision `prec`.
    ///
    /// Special cases:
    /// - $f(0,p,m)=1$.
    ///
    /// Overflow and underflow:
    /// - If $f(x,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$ is
    ///   returned instead.
    /// - If $f(x,p,m)<2^{-2^{30}}$ and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $f(x,p,m)<2^{-2^{30}}$ and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned instead.
    /// - If $f(x,p,m)\leq2^{-2^{30}-1}$ and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,p,m)<2^{-2^{30}}$ and $m$ is `Nearest`, $2^{-2^{30}}$ is returned
    ///   instead.
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`Float::power_of_2_rational_prec_ref`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with the given precision (which is the case whenever $x$ is not an integer).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (p, o) =
    ///     Float::power_of_2_rational_prec_round_ref(&Rational::from_unsigneds(3u8, 5), 5, Floor);
    /// assert_eq!(p.to_string(), "1.5");
    /// assert_eq!(o, Less);
    ///
    /// let (p, o) = Float::power_of_2_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(3u8, 5),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(p.to_string(), "1.56");
    /// assert_eq!(o, Greater);
    ///
    /// let (p, o) =
    ///     Float::power_of_2_rational_prec_round_ref(&Rational::from_unsigneds(3u8, 5), 20, Floor);
    /// assert_eq!(p.to_string(), "1.515717");
    /// assert_eq!(o, Less);
    ///
    /// let (p, o) = Float::power_of_2_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(3u8, 5),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(p.to_string(), "1.515718");
    /// assert_eq!(o, Greater);
    /// ```
    pub fn power_of_2_rational_prec_round_ref(
        x: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        assert_ne!(prec, 0);
        // If x is an integer, 2^x is exactly a power of 2 (this includes 2^0 = 1). Handle it
        // directly: the Ziv loop in the helper never converges on an exactly-representable result.
        if let Ok(n) = Integer::try_from(x) {
            return if let Ok(pow) = i64::try_from(&n) {
                // `power_of_2_prec_round` handles its own overflow and underflow.
                Self::power_of_2_prec_round(pow, prec, rm)
            } else if x.sign() == Greater {
                // x is too large to fit in an i64, so 2^x overflows.
                exp_overflow(prec, rm)
            } else {
                exp_underflow(prec, rm)
            };
        }
        power_of_2_rational_helper(x, prec, rm)
    }

    #[allow(clippy::needless_pass_by_value)]
    /// Computes $2^x$, where $x$ is a [`Rational`], rounding the result to the nearest value of the
    /// specified precision and returning the result as a [`Float`]. The [`Rational`] is taken by
    /// value. An [`Ordering`] is also returned, indicating whether the rounded power is less than,
    /// equal to, or greater than the exact power.
    ///
    /// If the power is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = 2^x+\varepsilon,
    /// $$
    /// where $|\varepsilon| \leq 2^{\lfloor\log_2 2^x\rfloor-p}$ (unless the result overflows or
    /// underflows; see below).
    ///
    /// The output has precision `prec`.
    ///
    /// Special cases:
    /// - $f(0,p)=1$.
    ///
    /// Overflow and underflow:
    /// - If $f(x,p)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::power_of_2_rational_prec_round`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (p, o) = Float::power_of_2_rational_prec(Rational::from_unsigneds(3u8, 5), 5);
    /// assert_eq!(p.to_string(), "1.5");
    /// assert_eq!(o, Less);
    ///
    /// let (p, o) = Float::power_of_2_rational_prec(Rational::from_unsigneds(3u8, 5), 20);
    /// assert_eq!(p.to_string(), "1.515717");
    /// assert_eq!(o, Less);
    ///
    /// let (p, o) = Float::power_of_2_rational_prec(Rational::from(0), 10);
    /// assert_eq!(p.to_string(), "1.0");
    /// assert_eq!(o, Equal);
    /// ```
    #[inline]
    pub fn power_of_2_rational_prec(x: Rational, prec: u64) -> (Self, Ordering) {
        Self::power_of_2_rational_prec_round_ref(&x, prec, Nearest)
    }

    /// Computes $2^x$, where $x$ is a [`Rational`], rounding the result to the nearest value of the
    /// specified precision and returning the result as a [`Float`]. The [`Rational`] is taken by
    /// reference. An [`Ordering`] is also returned, indicating whether the rounded power is less
    /// than, equal to, or greater than the exact power.
    ///
    /// If the power is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = 2^x+\varepsilon,
    /// $$
    /// where $|\varepsilon| \leq 2^{\lfloor\log_2 2^x\rfloor-p}$ (unless the result overflows or
    /// underflows; see below).
    ///
    /// The output has precision `prec`.
    ///
    /// Special cases:
    /// - $f(0,p)=1$.
    ///
    /// Overflow and underflow:
    /// - If $f(x,p)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::power_of_2_rational_prec_round_ref`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (p, o) = Float::power_of_2_rational_prec_ref(&Rational::from_unsigneds(3u8, 5), 5);
    /// assert_eq!(p.to_string(), "1.5");
    /// assert_eq!(o, Less);
    ///
    /// let (p, o) = Float::power_of_2_rational_prec_ref(&Rational::from_unsigneds(3u8, 5), 20);
    /// assert_eq!(p.to_string(), "1.515717");
    /// assert_eq!(o, Less);
    ///
    /// let (p, o) = Float::power_of_2_rational_prec_ref(&Rational::from(0), 10);
    /// assert_eq!(p.to_string(), "1.0");
    /// assert_eq!(o, Equal);
    /// ```
    #[inline]
    pub fn power_of_2_rational_prec_ref(x: &Rational, prec: u64) -> (Self, Ordering) {
        Self::power_of_2_rational_prec_round_ref(x, prec, Nearest)
    }
}

impl PowerOf2<Self> for Float {
    /// Computes $2^x$, where $x$ is a [`Float`], taking it by value.
    ///
    /// If the output has a precision, it is the precision of the input. If the power is equidistant
    /// from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s in its binary
    /// expansion is chosen. See [`RoundingMode`] for a description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x) = 2^x+\varepsilon.
    /// $$
    /// - If $2^x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $2^x$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2 2^x\rfloor-p}$,
    ///   where $p$ is the precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN})=\text{NaN}$
    /// - $f(\infty)=\infty$
    /// - $f(-\infty)=0.0$
    /// - $f(\pm0.0)=1.0$
    ///
    /// See the [`Float::power_of_2_of_float_round`] documentation for information on overflow and
    /// underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::power_of_2_of_float_round`] instead. If you want to specify the output precision,
    /// consider using [`Float::power_of_2_of_float_prec`]. If you want both of these things,
    /// consider using [`Float::power_of_2_of_float_prec_round`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::PowerOf2;
    /// use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity, Zero};
    /// use malachite_float::Float;
    ///
    /// assert!(Float::power_of_2(Float::NAN).is_nan());
    /// assert_eq!(Float::power_of_2(Float::INFINITY), Float::INFINITY);
    /// assert_eq!(Float::power_of_2(Float::NEGATIVE_INFINITY), Float::ZERO);
    /// assert_eq!(
    ///     Float::power_of_2(Float::from_unsigned_prec(3u32, 100).0 >> 1u32).to_string(),
    ///     "2.828427124746190097603377448418"
    /// );
    /// ```
    #[inline]
    fn power_of_2(pow: Self) -> Self {
        Self::power_of_2_of_float_round(pow, Nearest).0
    }
}

impl PowerOf2<&Self> for Float {
    /// Computes $2^x$, where $x$ is a [`Float`], taking it by reference.
    ///
    /// If the output has a precision, it is the precision of the input. If the power is equidistant
    /// from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s in its binary
    /// expansion is chosen. See [`RoundingMode`] for a description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x) = 2^x+\varepsilon.
    /// $$
    /// - If $2^x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $2^x$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2 2^x\rfloor-p}$,
    ///   where $p$ is the precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN})=\text{NaN}$
    /// - $f(\infty)=\infty$
    /// - $f(-\infty)=0.0$
    /// - $f(\pm0.0)=1.0$
    ///
    /// See the [`Float::power_of_2_of_float_round`] documentation for information on overflow and
    /// underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::power_of_2_of_float_round_ref`] instead. If you want to specify the output
    /// precision, consider using [`Float::power_of_2_of_float_prec_ref`]. If you want both of these
    /// things, consider using [`Float::power_of_2_of_float_prec_round_ref`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::PowerOf2;
    /// use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity, Zero};
    /// use malachite_float::Float;
    ///
    /// assert!(Float::power_of_2(&Float::NAN).is_nan());
    /// assert_eq!(Float::power_of_2(&Float::INFINITY), Float::INFINITY);
    /// assert_eq!(Float::power_of_2(&Float::NEGATIVE_INFINITY), Float::ZERO);
    /// assert_eq!(
    ///     Float::power_of_2(&(Float::from_unsigned_prec(3u32, 100).0 >> 1u32)).to_string(),
    ///     "2.828427124746190097603377448418"
    /// );
    /// ```
    #[inline]
    fn power_of_2(pow: &Self) -> Self {
        Self::power_of_2_of_float_round_ref(pow, Nearest).0
    }
}

impl PowerOf2Assign for Float {
    /// Computes $2^x$, where $x$ is a [`Float`], in place.
    ///
    /// If the output has a precision, it is the precision of the input. If the power is equidistant
    /// from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s in its binary
    /// expansion is chosen. See [`RoundingMode`] for a description of the `Nearest` rounding mode.
    ///
    /// $$
    /// x \gets 2^x+\varepsilon.
    /// $$
    /// - If $2^x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $2^x$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2 2^x\rfloor-p}$,
    ///   where $p$ is the precision of the input.
    ///
    /// See the [`Float::power_of_2_of_float_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::power_of_2_of_float_round_assign`] instead. If you want to specify the output
    /// precision, consider using [`Float::power_of_2_of_float_prec_assign`]. If you want both of
    /// these things, consider using [`Float::power_of_2_of_float_prec_round_assign`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::PowerOf2Assign;
    /// use malachite_float::Float;
    ///
    /// let mut x = Float::from_unsigned_prec(3u32, 100).0 >> 1u32;
    /// x.power_of_2_assign();
    /// assert_eq!(x.to_string(), "2.828427124746190097603377448418");
    /// ```
    #[inline]
    fn power_of_2_assign(&mut self) {
        self.power_of_2_of_float_round_assign(Nearest);
    }
}

/// Computes $2^x$, where $x$ is a primitive float, returning the result as a primitive float of the
/// same type. Using this function is more accurate than using `x.exp2()` or the `exp2` function
/// provided by `libm`.
///
/// $$
/// f(x) = 2^x+\varepsilon.
/// $$
/// - If $2^x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
/// - If $2^x$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2 2^x\rfloor-p}$, where
///   $p$ is the precision of the output (typically 24 if `T` is a [`f32`] and 53 if `T` is a
///   [`f64`], but less if the output is subnormal).
///
/// Special cases:
/// - $f(\text{NaN})=\text{NaN}$
/// - $f(\infty)=\infty$
/// - $f(-\infty)=0.0$
/// - $f(\pm0.0)=1.0$
///
/// Overflow and underflow are possible: a large positive `x` gives $\infty$, and a large negative
/// `x` gives `0.0`.
///
/// # Worst-case complexity
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use malachite_base::num::basic::traits::NegativeInfinity;
/// use malachite_base::num::float::NiceFloat;
/// use malachite_float::arithmetic::power_of_2_of_float::primitive_float_power_of_2;
///
/// assert!(primitive_float_power_of_2(f32::NAN).is_nan());
/// assert_eq!(
///     NiceFloat(primitive_float_power_of_2(f32::INFINITY)),
///     NiceFloat(f32::INFINITY)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_power_of_2(f32::NEGATIVE_INFINITY)),
///     NiceFloat(0.0)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_power_of_2(0.0f32)),
///     NiceFloat(1.0)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_power_of_2(1.0f32)),
///     NiceFloat(2.0)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_power_of_2(0.5f32)),
///     NiceFloat(1.4142135)
/// );
/// ```
#[inline]
#[allow(clippy::type_repetition_in_bounds)]
pub fn primitive_float_power_of_2<T: PrimitiveFloat>(x: T) -> T
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    emulate_float_to_float_fn(Float::power_of_2_of_float_prec, x)
}

/// Computes $2^x$, where $x$ is a [`Rational`], returning the result as a primitive float.
///
/// $$
/// f(x) = 2^x+\varepsilon.
/// $$
/// - If $2^x$ is infinite or zero, $\varepsilon$ may be ignored or assumed to be 0.
/// - If $2^x$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2 2^x\rfloor-p}$, where
///   $p$ is the precision of the output (typically 24 if `T` is a [`f32`] and 53 if `T` is a
///   [`f64`], but less if the output is subnormal).
///
/// Special cases:
/// - $f(0)=1$
///
/// Overflow and underflow are possible: a large positive `x` gives $\infty$, and a large negative
/// `x` gives `0.0`.
///
/// # Worst-case complexity
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use malachite_base::num::basic::traits::Zero;
/// use malachite_base::num::float::NiceFloat;
/// use malachite_float::arithmetic::power_of_2_of_float::primitive_float_power_of_2_rational;
/// use malachite_q::Rational;
///
/// assert_eq!(
///     NiceFloat(primitive_float_power_of_2_rational::<f64>(&Rational::ZERO)),
///     NiceFloat(1.0)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_power_of_2_rational::<f64>(
///         &Rational::from_unsigneds(1u8, 3)
///     )),
///     NiceFloat(1.2599210498948732)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_power_of_2_rational::<f64>(&Rational::from(
///         10000
///     ))),
///     NiceFloat(f64::INFINITY)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_power_of_2_rational::<f64>(&Rational::from(
///         -10000
///     ))),
///     NiceFloat(0.0)
/// );
/// ```
#[inline]
#[allow(clippy::type_repetition_in_bounds)]
pub fn primitive_float_power_of_2_rational<T: PrimitiveFloat>(x: &Rational) -> T
where
    Float: PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    emulate_rational_to_float_fn(Float::power_of_2_rational_prec_ref, x)
}
