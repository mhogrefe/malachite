// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the GNU MPFR Library.
//
//      Copyright 1999-2026 Free Software Foundation, Inc.
//
//      Contributed by the Pascaline and Caramba projects, INRIA.
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::InnerFloat::{Finite, Infinity, NaN, Zero};
use crate::basic::extended::ExtendedFloat;
use crate::{
    Float, emulate_float_to_float_fn, emulate_rational_to_float_fn, float_either_zero,
    float_infinity, float_nan, float_negative_infinity, float_zero, floor_and_ceiling,
    significand_bits,
};
use core::cmp::Ordering::{self, *};
use core::mem::{swap, take};
use malachite_base::num::arithmetic::traits::{
    Abs, Agm, CeilingLogBase2, IsPowerOf2, Ln, LnAssign, Parity, Sign,
};
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::{NegativeInfinity, One, Zero as ZeroTrait};
use malachite_base::num::conversion::traits::{ExactFrom, RoundingFrom, SaturatingFrom};
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_nz::integer::Integer;
use malachite_nz::natural::arithmetic::float_extras::float_can_round;
use malachite_nz::platform::Limb;
use malachite_q::Rational;

// The computation of log(x) is done using the formula: if we want p bits of the result,
// ```
//                    pi
//      log(x) ~ ------------- - m log 2
//               2 AG(1,4 / s)
// ```
// where s = x 2^m > 2^(p/2).
//
// More precisely, if F(x) = int(1 / ln(1 - (1 - x ^ 2) * sin(t) ^ 2), t = 0..pi / 2), then for s >=
// 1.26 we have log(s) < F(4 / s) < log(s) * (1 + 4 / s ^ 2) from which we deduce pi / 2 / AG(1, 4 /
// s) * (1 - 4 / s ^ 2) < log(s) < pi / 2 / AG(1, 4 / s) so the relative error 4 / s ^ 2 is < 4 / 2
// ^ p i.e. 4 ulps.
//
// When `x` lies within a sliver of 1 -- `|x - 1|` within a few binades of the smallest positive
// `Float` -- returns `x - 1`, and otherwise `None`. For such `x`, `log(x) ~ x - 1` can fall below
// the smallest positive `Float`: the working subtractions in the log loops would flush to zero or
// clamp, and the rounding test could never certify a result, so the callers delegate to the
// `1_plus_x` functions, whose tiny-argument paths handle the underflow region correctly. Reaching
// the sliver requires an input precision of nearly 2^30 bits, so for every other `x` the guard
// costs only an exponent-and-precision test; the subtraction (performed only past that test) is
// exact, since `x` is within `(1/2, 2)`. Brackets of ln(1 + e) for an exact nonzero Rational e with
// |e| < 1/2, as exact Rationals, to a relative accuracy of about 2^-wprec. Uses the Mercator series
// ln(1 + e) = sum_{k>=1} (-1)^(k+1) e^k / k. For e > 0 the terms strictly alternate in sign and
// decrease in magnitude, so consecutive partial sums bracket the value; for e < 0 every term is
// negative, so a partial sum is an upper bound and the remainder after it is bounded in magnitude
// by |e|^(k+1) / ((k + 1)(1 - |e|)). Unlike the atanh form, this needs no `e / (2 + e)` division,
// which is the dominant cost when e is a sub-`MIN` sliver (a ~128-MB Rational).
pub(crate) fn ln_1_plus_rational_brackets(e: &Rational, wprec: u64) -> (Rational, Rational) {
    let negative = *e < 0u32;
    let mut pow = e.clone(); // e^k
    let mut s = e.clone(); // partial sum S_k
    let mut k = 1u64;
    loop {
        pow *= e; // e^(k+1)
        k += 1;
        let mut term = &pow / Rational::from(k); // (-1)^(k+1) e^k / k, up to sign
        if k.even() {
            term = -term;
        }
        let s_next = &s + &term; // S_{k+1}
        let (lo, hi) = if negative {
            // S_{k+1} is an upper bound; the remainder is bounded in magnitude by |e|^(k+2) / ((k +
            // 2)(1 - |e|)), and 1 - |e| = 1 + e for e < 0.
            let bound = -((&pow * e) / (Rational::from(k + 1) * (Rational::ONE + e))).abs();
            (&s_next + &bound, s_next.clone())
        } else if s < s_next {
            (s.clone(), s_next.clone())
        } else {
            (s_next.clone(), s.clone())
        };
        s = s_next;
        let width = &hi - &lo;
        if width == 0u32
            || width.floor_log_base_2_abs() < lo.floor_log_base_2_abs() - i64::exact_from(wprec) - 2
        {
            return (lo, hi);
        }
    }
}

pub(crate) enum SliverOfOne {
    // `x` is not within a sliver of 1; use the ordinary logarithm path.
    No,
    // `x = 1 + d` with `d` representable; compute the logarithm via the `1_plus_x` form.
    Representable(Float),
    // `x` is so close to 1 that its logarithm falls at or below the smallest positive `Float`. The
    // subtraction `x - 1` cannot be represented (it flushes to zero for `x > 1`, or clamps to the
    // minimum magnitude for `x < 1`), so the caller computes the underflowing result via the
    // exact-`Rational` logarithm of `x` (whose Rational-argument path has no exponent range).
    Underflow,
}

pub(crate) fn sliver_of_one(x: &Float) -> SliverOfOne {
    let e = i64::from(x.get_exponent().unwrap());
    if (e == 0 || e == 1)
        && x.get_prec().unwrap() >= u64::exact_from(-i64::from(Float::MIN_EXPONENT) - 8)
    {
        let (mut d, o) = x.sub_prec_round_ref_val(Float::ONE, x.get_prec().unwrap() + 1, Floor);
        if o != Equal {
            // `x - 1` fell below the smallest positive `Float`, so `ln(x) ~ x - 1` underflows.
            return SliverOfOne::Underflow;
        }
        if i64::from(d.get_exponent().unwrap()) <= i64::from(Float::MIN_EXPONENT) + 4 {
            // Shed the trailing zeros inherited from the subtraction's requested precision: d's
            // true significant span is small, and the inflated precision would defeat the
            // `1_plus_x` functions' round-near-x shortcut (a significand padded with ~2^30 trailing
            // zeros fails their rounding test).
            let sig = d.significand_ref().unwrap();
            let min_prec = significand_bits(sig) - sig.trailing_zeros().unwrap();
            let o = d.set_prec_round(min_prec, Floor);
            debug_assert_eq!(o, Equal);
            return SliverOfOne::Representable(d);
        }
    }
    SliverOfOne::No
}

// This is mpfr_log from log.c, MPFR 4.2.0.
fn ln_prec_round_normal_ref(x: &Float, prec: u64, rm: RoundingMode) -> (Float, Ordering) {
    if *x == 1u32 {
        return (Float::ZERO, Equal);
    }
    // ln(x) for x in a sliver of 1 can fall below the smallest positive Float; the 1-plus-x form
    // handles that underflow region.
    match sliver_of_one(x) {
        SliverOfOne::Representable(d) => return d.ln_1_plus_x_prec_round(prec, rm),
        SliverOfOne::Underflow => {
            return Float::ln_rational_prec_round(Rational::exact_from(x), prec, rm);
        }
        SliverOfOne::No => {}
    }
    assert_ne!(rm, Exact, "Inexact ln");
    let x_exp = i64::from(x.get_exponent().unwrap());
    // use initial precision about q + 2 * lg(q) + cte
    let mut working_prec = prec + (prec.ceiling_log_base_2() << 1) + 10;
    let mut increment = Limb::WIDTH;
    let mut previous_m = 0;
    let mut x = x.clone();
    loop {
        // Calculus of m (depends on p)
        let m = i64::exact_from((working_prec + 3) >> 1)
            .checked_sub(x_exp)
            .unwrap();
        x <<= m - previous_m;
        previous_m = m;
        assert!(x.is_normal());
        let tmp2 = Float::pi_prec(working_prec).0
            / (Float::ONE.agm(
                const { Float::const_from_unsigned(4) }
                    .div_prec_round_val_ref(&x, working_prec, Floor)
                    .0,
            ) << 1u32);
        let exp2 = tmp2.get_exponent();
        let tmp1 = tmp2
            - Float::ln_2_prec(working_prec)
                .0
                .mul_prec(Float::from(m), working_prec)
                .0;
        if let (Some(exp1), Some(exp2)) = (tmp1.get_exponent(), exp2) {
            let cancel = u64::saturating_from(exp2 - exp1);
            // we have 7 ulps of error from the above roundings, 4 ulps from the 4 / s ^ 2 second
            // order term, plus the canceled bits
            if float_can_round(
                tmp1.significand_ref().unwrap(),
                working_prec.saturating_sub(cancel).saturating_sub(4),
                prec,
                rm,
            ) {
                return Float::from_float_prec_round(tmp1, prec, rm);
            }
            working_prec += cancel + working_prec.ceiling_log_base_2();
        } else {
            working_prec += working_prec.ceiling_log_base_2();
        }
        working_prec += increment;
        increment = working_prec >> 1;
    }
}

// This is mpfr_log from log.c, MPFR 4.2.0.
fn ln_prec_round_normal(mut x: Float, prec: u64, rm: RoundingMode) -> (Float, Ordering) {
    if x == 1u32 {
        return (Float::ZERO, Equal);
    }
    // ln(x) for x in a sliver of 1 can fall below the smallest positive Float; the 1-plus-x form
    // handles that underflow region.
    match sliver_of_one(&x) {
        SliverOfOne::Representable(d) => return d.ln_1_plus_x_prec_round(prec, rm),
        SliverOfOne::Underflow => {
            return Float::ln_rational_prec_round(Rational::exact_from(&x), prec, rm);
        }
        SliverOfOne::No => {}
    }
    assert_ne!(rm, Exact, "Inexact ln");
    let x_exp = i64::from(x.get_exponent().unwrap());
    // use initial precision about q + 2 * lg(q) + cte
    let mut working_prec = prec + (prec.ceiling_log_base_2() << 1) + 10;
    let mut increment = Limb::WIDTH;
    let mut previous_m = 0;
    loop {
        // Calculus of m (depends on p)
        let m = i64::exact_from((working_prec + 3) >> 1)
            .checked_sub(x_exp)
            .unwrap();
        x <<= m - previous_m;
        previous_m = m;
        assert!(x.is_normal());
        let tmp2 = Float::pi_prec(working_prec).0
            / (Float::ONE.agm(
                const { Float::const_from_unsigned(4) }
                    .div_prec_round_val_ref(&x, working_prec, Floor)
                    .0,
            ) << 1u32);
        let exp2 = tmp2.get_exponent();
        let tmp1 = tmp2
            - Float::ln_2_prec(working_prec)
                .0
                .mul_prec(Float::from(m), working_prec)
                .0;
        if let (Some(exp1), Some(exp2)) = (tmp1.get_exponent(), exp2) {
            let cancel = u64::saturating_from(exp2 - exp1);
            // we have 7 ulps of error from the above roundings, 4 ulps from the 4 / s ^ 2 second
            // order term, plus the canceled bits
            if float_can_round(
                tmp1.significand_ref().unwrap(),
                working_prec.saturating_sub(cancel).saturating_sub(4),
                prec,
                rm,
            ) {
                return Float::from_float_prec_round(tmp1, prec, rm);
            }
            working_prec += cancel + working_prec.ceiling_log_base_2();
        } else {
            working_prec += working_prec.ceiling_log_base_2();
        }
        working_prec += increment;
        increment = working_prec >> 1;
    }
}

pub(crate) fn ln_prec_round_normal_extended(
    x: ExtendedFloat,
    prec: u64,
    rm: RoundingMode,
) -> (Float, Ordering) {
    if x.exp == 1 && x.x.is_power_of_2() {
        return (Float::ZERO, Equal);
    }
    assert_ne!(rm, Exact, "Inexact ln");
    let x_exp = x.exp;
    // use initial precision about q + 2 * lg(q) + cte
    let mut working_prec = prec + (prec.ceiling_log_base_2() << 1) + 10;
    let mut increment = Limb::WIDTH;
    let mut m = i64::exact_from((working_prec + 3) >> 1)
        .checked_sub(x.exp)
        .unwrap();
    let mut previous_m = m;
    let mut x = Float::exact_from(x << m);
    let mut first = true;
    loop {
        if first {
            first = false;
        } else {
            // Calculus of m (depends on p)
            m = i64::exact_from((working_prec + 3) >> 1)
                .checked_sub(x_exp)
                .unwrap();
            x <<= m - previous_m;
            previous_m = m;
        }
        assert!(x.is_normal());
        let tmp2 = Float::pi_prec(working_prec).0
            / (Float::ONE.agm(
                const { Float::const_from_unsigned(4) }
                    .div_prec_round_val_ref(&x, working_prec, Floor)
                    .0,
            ) << 1u32);
        let exp2 = tmp2.get_exponent();
        let tmp1 = tmp2
            - Float::ln_2_prec(working_prec)
                .0
                .mul_prec(Float::from(m), working_prec)
                .0;
        if let (Some(exp1), Some(exp2)) = (tmp1.get_exponent(), exp2) {
            let cancel = u64::saturating_from(exp2 - exp1);
            // we have 7 ulps of error from the above roundings, 4 ulps from the 4 / s ^ 2 second
            // order term, plus the canceled bits
            if float_can_round(
                tmp1.significand_ref().unwrap(),
                working_prec.saturating_sub(cancel).saturating_sub(4),
                prec,
                rm,
            ) {
                return Float::from_float_prec_round(tmp1, prec, rm);
            }
            working_prec += cancel + working_prec.ceiling_log_base_2();
        } else {
            working_prec += working_prec.ceiling_log_base_2();
        }
        working_prec += increment;
        increment = working_prec >> 1;
    }
}

// Computes `ln(1 + eps)` for a nonzero `Rational` `eps` of tiny magnitude (the caller guards `|eps|
// < 2^(MIN_EXPONENT + 5)`), where the result can lie below the smallest positive `Float`: the
// bracketing in `ln_rational_helper` could never resolve such a value (its `Float` approximations
// of `1 + eps` collapse to 1). The Taylor series `ln(1 + eps) = eps - eps^2/2 + eps^3/3 - ...` is
// summed term by term in exact `Rational`s, bracketing the exact value between two rationals
// (consecutive partial sums for `eps > 0`, a partial sum and a remainder bound for `eps < 0`) until
// both ends round to the same `Float`; `from_rational_prec_round` performs the final, possibly
// underflowing, clamp. Termination: `ln(1 + eps)` is irrational, so some finite bracket eventually
// separates it from every representable point and tie. This mirrors `exp_rational_near_one`.
fn ln_rational_near_one(eps: &Rational, prec: u64, rm: RoundingMode) -> (Float, Ordering) {
    let negative = *eps < 0u32;
    let mut pow = eps.clone(); // eps^k
    let mut s = eps.clone(); // S_1
    let mut k = 1u64;
    loop {
        pow *= eps;
        k += 1;
        let mut term = &pow / Rational::from(k); // |term| when k even, term when k odd
        if k.even() {
            term = -term;
        }
        let s_next = &s + &term; // S_k
        let (lo, hi) = if negative {
            // Every term is negative, so the partial sums decrease toward ln(1 + eps), and the
            // remainder after S_k is bounded in magnitude by |eps|^(k+1) / ((k + 1) (1 - |eps|)).
            let bound = (&pow * eps) / (Rational::from(k + 1) * (Rational::ONE + eps.clone()));
            // pow * eps = eps^(k+1) is negative here (odd power of a negative number) when k is
            // even... its sign alternates; take the magnitude explicitly.
            let bound = -bound.abs();
            (&s_next + bound, s_next.clone())
        } else {
            // The terms alternate in sign with strictly decreasing magnitude, so ln(1 + eps) lies
            // between consecutive partial sums.
            if s < s_next {
                (s.clone(), s_next.clone())
            } else {
                (s_next.clone(), s.clone())
            }
        };
        s = s_next;
        let (f_lo, mut o_lo) = Float::from_rational_prec_round_ref(&lo, prec, rm);
        let (f_hi, mut o_hi) = Float::from_rational_prec_round_ref(&hi, prec, rm);
        // A bound that is exactly representable rounds with `Equal`; the exact value lies strictly
        // inside the bracket, so treat it as agreeing with the other bound.
        if o_lo == Equal {
            o_lo = o_hi;
        }
        if o_hi == Equal {
            o_hi = o_lo;
        }
        if o_lo == o_hi && f_lo == f_hi {
            return (f_lo, o_lo);
        }
    }
}

fn ln_rational_helper(x: &Rational, prec: u64, rm: RoundingMode) -> (Float, Ordering) {
    let mut working_prec = prec + 10;
    let mut increment = Limb::WIDTH;
    loop {
        let (x_lo, x_o) = Float::from_rational_prec_round_ref(x, working_prec, Floor);
        if x_o == Equal {
            return ln_prec_round_normal(x_lo, prec, rm);
        }
        let (x_lo, x_hi) = floor_and_ceiling((x_lo, x_o));
        let (ln_lo, mut o_lo) = ln_prec_round_normal(x_lo, prec, rm);
        let (ln_hi, mut o_hi) = ln_prec_round_normal(x_hi, prec, rm);
        if o_lo == Equal {
            o_lo = o_hi;
        }
        if o_hi == Equal {
            o_hi = o_lo;
        }
        if o_lo == o_hi && ln_lo == ln_hi {
            return (ln_lo, o_lo);
        }
        working_prec += increment;
        increment = working_prec >> 1;
    }
}

fn ln_rational_helper_extended(x: &Rational, prec: u64, rm: RoundingMode) -> (Float, Ordering) {
    let mut working_prec = prec + 10;
    let mut increment = Limb::WIDTH;
    loop {
        let (x_lo, x_o) = ExtendedFloat::from_rational_prec_round_ref(x, working_prec, Floor);
        if x_o == Equal {
            return ln_prec_round_normal_extended(x_lo, prec, rm);
        }
        let (x_lo, x_hi) = crate::basic::extended::floor_and_ceiling((x_lo, x_o));
        let (ln_lo, mut o_lo) = ln_prec_round_normal_extended(x_lo, prec, rm);
        let (ln_hi, mut o_hi) = ln_prec_round_normal_extended(x_hi, prec, rm);
        if o_lo == Equal {
            o_lo = o_hi;
        }
        if o_hi == Equal {
            o_hi = o_lo;
        }
        if o_lo == o_hi && ln_lo == ln_hi {
            return (ln_lo, o_lo);
        }
        working_prec += increment;
        increment = working_prec >> 1;
    }
}

// This is the recursive function S from log_ui.c, MPFR 4.2.2. It performs the binary splitting of
// the Taylor series of log(1 + x) for x = p/2^k, over the terms n1..n2: the sum is T[0]/(B[0] *
// 2^q). `p`, `b`, and `t` are per-recursion-depth scratch stacks (indexed by depth); `p_val` is odd
// or zero.
#[allow(clippy::too_many_arguments)]
fn log_ui_s(
    p: &mut [Integer],
    b: &mut [Integer],
    t: &mut [Integer],
    q: &mut u64,
    n1: u64,
    n2: u64,
    p_val: i64,
    k: u64,
    need_p: bool,
) {
    if n2 == n1 + 1 {
        p[0] = Integer::from(if n1 == 1 { p_val } else { -p_val });
        *q = k;
        b[0] = Integer::from(n1);
        // T = B * Q * S where S = P / (B * Q), thus T = P
        t[0] = p[0].clone();
    } else {
        // m = floor((n1 + n2) / 2)
        let m = (n1 >> 1) + (n2 >> 1) + (n1 & n2 & 1);
        log_ui_s(p, b, t, q, n1, m, p_val, k, true);
        let mut q1 = 0;
        log_ui_s(
            &mut p[1..],
            &mut b[1..],
            &mut t[1..],
            &mut q1,
            m,
            n2,
            p_val,
            k,
            need_p,
        );
        // T[0] <- T[0] * B[1] * 2^q1 + P[0] * B[0] * T[1]
        let new_t1 = &t[1] * &p[0] * &b[0];
        t[1] = new_t1;
        let new_t0 = ((&t[0] * &b[1]) << q1) + &t[1];
        t[0] = new_t0;
        if need_p {
            let new_p0 = &p[0] * &p[1];
            p[0] = new_p0;
        }
        *q += q1;
        let new_b0 = &b[0] * &b[1];
        b[0] = new_b0;
    }
}

// This is mpfr_log_ui from log_ui.c, MPFR 4.2.2, for n >= 3.
fn ln_unsigned_prec_round_normal(n: u64, prec: u64, rm: RoundingMode) -> (Float, Ordering) {
    assert_ne!(rm, Exact, "Inexact ln");
    // Argument reduction: compute k such that 2/3 < n/2^k < 4/3, i.e., 2^(k+1) < 3n < 2^(k+2). So k
    // = sizeinbase(3n, 2) - 2.
    let three_n = 3u128 * u128::from(n);
    let k = u64::from(128 - three_n.leading_zeros() - 2);
    // The reduced argument is (n - 2^k)/2^k. p = n - 2^k satisfies |p| < 2^k/3 < n/2 <= i64::MAX,
    // so it fits in an i64.
    let mut p = i64::exact_from(i128::from(n) - (1i128 << k));
    let mut kk = k;
    if p != 0 {
        // replace p/2^kk by (p/2)/2^(kk-1) so that p is odd
        while p.even() {
            p >>= 1;
            kk -= 1;
        }
    }
    let mut w = prec + prec.ceiling_log_base_2() + 10;
    loop {
        // We need at most w/log2(2^kk/|p|) = w/(kk - log2|p|) terms for an accuracy of w bits.
        let abs_p = p.unsigned_abs();
        let n_terms = if abs_p == 0 {
            2
        } else {
            let log2_abs_p = if abs_p == 1 {
                0
            } else {
                abs_p.ceiling_log_base_2()
            };
            w.div_ceil(kk - log2_abs_p).max(2)
        };
        // The binary-splitting integers T[0] and B[0] * 2^q0 have about n_terms * (log2(n_terms) +
        // kk) bits; if that exceeds the Float exponent range, converting them to Floats would
        // overflow to Infinity. In that extreme-precision regime, fall back to the
        // arithmetic-geometric-mean logarithm, which is correct at any precision and produces the
        // same correctly-rounded result.
        let integer_bits =
            n_terms.saturating_mul(n_terms.ceiling_log_base_2().saturating_add(kk));
        if integer_bits.saturating_add(64) >= u64::exact_from(Float::MAX_EXPONENT) {
            return Float::from(n).ln_prec_round(prec, rm);
        }
        let lg_n = usize::exact_from(n_terms.ceiling_log_base_2() + 1);
        let mut p_arr = vec![Integer::ZERO; lg_n];
        let mut b_arr = vec![Integer::ZERO; lg_n];
        let mut t_arr = vec![Integer::ZERO; lg_n];
        let mut q0 = 0;
        log_ui_s(
            &mut p_arr, &mut b_arr, &mut t_arr, &mut q0, 1, n_terms, p, kk, false,
        );
        // t = T[0] / (B[0] * 2^q0) = log(n/2^k) approximately
        let t_num = Float::from_integer_prec(take(&mut t_arr[0]), w).0;
        let t_den = Float::from_integer_prec(take(&mut b_arr[0]), w).0 << q0;
        let mut t = t_num.div_prec(t_den, w).0;
        // argument reconstruction: add k * log(2)
        let ln_2_k = Float::ln_2_prec(w).0.mul_prec(Float::from(k), w).0;
        t.add_prec_assign(ln_2_k, w);
        // The maximal error is at most k + 6 ulps.
        let err = (k + 6).ceiling_log_base_2() + 1;
        if float_can_round(
            t.significand_ref().unwrap(),
            w.saturating_sub(err),
            prec,
            rm,
        ) {
            return Float::from_float_prec_round(t, prec, rm);
        }
        w += w >> 1;
    }
}

impl Float {
    /// Computes the natural logarithm of a [`Float`], rounding the result to the specified
    /// precision and with the specified rounding mode. The [`Float`] is taken by value. An
    /// [`Ordering`] is also returned, indicating whether the rounded logarithm is less than, equal
    /// to, or greater than the exact logarithm. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The logarithm of any nonzero negative number is `NaN`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \ln{x}+\varepsilon.
    /// $$
    /// - If $\ln{x}$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $\ln{x}$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 \|ln{x}|\rfloor-p+1}$.
    /// - If $\ln{x}$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 \|ln{x}|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=\text{NaN}$
    /// - $f(\infty,p,m)=\infty$
    /// - $f(-\infty,p,m)=\text{NaN}$
    /// - $f(\pm0.0,p,m)=-\infty$
    ///
    /// Neither overflow nor underflow is possible.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::ln_prec`] instead. If you
    /// know that your target precision is the precision of the input, consider using
    /// [`Float::ln_round`] instead. If both of these things are true, consider using [`Float::ln`]
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n (\log n)^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the result cannot be represented exactly with the given
    /// precision.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (ln, o) = Float::from_unsigned_prec(10u32, 100)
    ///     .0
    ///     .ln_prec_round(5, Floor);
    /// assert_eq!(ln.to_string(), "2.2");
    /// assert_eq!(o, Less);
    ///
    /// let (ln, o) = Float::from_unsigned_prec(10u32, 100)
    ///     .0
    ///     .ln_prec_round(5, Ceiling);
    /// assert_eq!(ln.to_string(), "2.4");
    /// assert_eq!(o, Greater);
    ///
    /// let (ln, o) = Float::from_unsigned_prec(10u32, 100)
    ///     .0
    ///     .ln_prec_round(5, Nearest);
    /// assert_eq!(ln.to_string(), "2.2");
    /// assert_eq!(o, Less);
    ///
    /// let (ln, o) = Float::from_unsigned_prec(10u32, 100)
    ///     .0
    ///     .ln_prec_round(20, Floor);
    /// assert_eq!(ln.to_string(), "2.302582");
    /// assert_eq!(o, Less);
    ///
    /// let (ln, o) = Float::from_unsigned_prec(10u32, 100)
    ///     .0
    ///     .ln_prec_round(20, Ceiling);
    /// assert_eq!(ln.to_string(), "2.302586");
    /// assert_eq!(o, Greater);
    ///
    /// let (ln, o) = Float::from_unsigned_prec(10u32, 100)
    ///     .0
    ///     .ln_prec_round(20, Nearest);
    /// assert_eq!(ln.to_string(), "2.302586");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn ln_prec_round(self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        assert_ne!(prec, 0);
        match self {
            Self(NaN | Infinity { sign: false } | Finite { sign: false, .. }) => {
                (float_nan!(), Equal)
            }
            float_either_zero!() => (float_negative_infinity!(), Equal),
            float_infinity!() => (float_infinity!(), Equal),
            _ => ln_prec_round_normal(self, prec, rm),
        }
    }

    /// Computes the natural logarithm of a [`Float`], rounding the result to the specified
    /// precision and with the specified rounding mode. The [`Float`] is taken by reference. An
    /// [`Ordering`] is also returned, indicating whether the rounded logarithm is less than, equal
    /// to, or greater than the exact logarithm. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The logarithm of any nonzero negative number is `NaN`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \ln{x}+\varepsilon.
    /// $$
    /// - If $\ln{x}$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $\ln{x}$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 \|ln{x}|\rfloor-p+1}$.
    /// - If $\ln{x}$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 \|ln{x}|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=\text{NaN}$
    /// - $f(\infty,p,m)=\infty$
    /// - $f(-\infty,p,m)=\text{NaN}$
    /// - $f(\pm0.0,p,m)=-\infty$
    ///
    /// Neither overflow nor underflow is possible.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::ln_prec_ref`] instead. If you
    /// know that your target precision is the precision of the input, consider using
    /// [`Float::ln_round_ref`] instead. If both of these things are true, consider using
    /// `(&Float).ln()`instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n (\log n)^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the result cannot be represented exactly with the given
    /// precision.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (ln, o) = Float::from_unsigned_prec(10u32, 100)
    ///     .0
    ///     .ln_prec_round_ref(5, Floor);
    /// assert_eq!(ln.to_string(), "2.2");
    /// assert_eq!(o, Less);
    ///
    /// let (ln, o) = Float::from_unsigned_prec(10u32, 100)
    ///     .0
    ///     .ln_prec_round_ref(5, Ceiling);
    /// assert_eq!(ln.to_string(), "2.4");
    /// assert_eq!(o, Greater);
    ///
    /// let (ln, o) = Float::from_unsigned_prec(10u32, 100)
    ///     .0
    ///     .ln_prec_round_ref(5, Nearest);
    /// assert_eq!(ln.to_string(), "2.2");
    /// assert_eq!(o, Less);
    ///
    /// let (ln, o) = Float::from_unsigned_prec(10u32, 100)
    ///     .0
    ///     .ln_prec_round_ref(20, Floor);
    /// assert_eq!(ln.to_string(), "2.302582");
    /// assert_eq!(o, Less);
    ///
    /// let (ln, o) = Float::from_unsigned_prec(10u32, 100)
    ///     .0
    ///     .ln_prec_round_ref(20, Ceiling);
    /// assert_eq!(ln.to_string(), "2.302586");
    /// assert_eq!(o, Greater);
    ///
    /// let (ln, o) = Float::from_unsigned_prec(10u32, 100)
    ///     .0
    ///     .ln_prec_round_ref(20, Nearest);
    /// assert_eq!(ln.to_string(), "2.302586");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn ln_prec_round_ref(&self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        assert_ne!(prec, 0);
        match self {
            Self(NaN | Infinity { sign: false } | Finite { sign: false, .. }) => {
                (float_nan!(), Equal)
            }
            float_either_zero!() => (float_negative_infinity!(), Equal),
            float_infinity!() => (float_infinity!(), Equal),
            _ => ln_prec_round_normal_ref(self, prec, rm),
        }
    }

    /// Computes the natural logarithm of a [`Float`], rounding the result to the nearest value of
    /// the specified precision. The [`Float`] is taken by value. An [`Ordering`] is also returned,
    /// indicating whether the rounded logarithm is less than, equal to, or greater than the exact
    /// logarithm. Although `NaN`s are not comparable to any [`Float`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// The logarithm of any nonzero negative number is `NaN`.
    ///
    /// If the logarithm is equidistant from two [`Float`]s with the specified precision, the
    /// [`Float`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a
    /// description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = \ln{x}+\varepsilon.
    /// $$
    /// - If $\ln{x}$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $\ln{x}$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2
    ///   \ln{x}\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=\text{NaN}$
    /// - $f(\infty,p,m)=\infty$
    /// - $f(-\infty,p,m)=\text{NaN}$
    /// - $f(\pm0.0,p,m)=-\infty$
    ///
    /// Neither overflow nor underflow is possible.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::ln_prec_round`] instead. If you know that your target precision is the precision of
    /// the input, consider using [`Float::ln`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n (\log n)^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (ln, o) = Float::from_unsigned_prec(10u32, 100).0.ln_prec(5);
    /// assert_eq!(ln.to_string(), "2.2");
    /// assert_eq!(o, Less);
    ///
    /// let (ln, o) = Float::from_unsigned_prec(10u32, 100).0.ln_prec(20);
    /// assert_eq!(ln.to_string(), "2.302586");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn ln_prec(self, prec: u64) -> (Self, Ordering) {
        self.ln_prec_round(prec, Nearest)
    }

    /// Computes the natural logarithm of a [`Float`], rounding the result to the nearest value of
    /// the specified precision. The [`Float`] is taken by reference. An [`Ordering`] is also
    /// returned, indicating whether the rounded logarithm is less than, equal to, or greater than
    /// the exact logarithm. Although `NaN`s are not comparable to any [`Float`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// The logarithm of any nonzero negative number is `NaN`.
    ///
    /// If the logarithm is equidistant from two [`Float`]s with the specified precision, the
    /// [`Float`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a
    /// description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = \ln{x}+\varepsilon.
    /// $$
    /// - If $\ln{x}$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $\ln{x}$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2
    ///   \ln{x}\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p)=\text{NaN}$
    /// - $f(\infty,p)=\infty$
    /// - $f(-\infty,p)=\text{NaN}$
    /// - $f(\pm0.0,p)=-\infty$
    ///
    /// Neither overflow nor underflow is possible.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::ln_prec_round_ref`] instead. If you know that your target precision is the
    /// precision of the input, consider using `(&Float).ln()` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n (\log n)^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (ln, o) = Float::from_unsigned_prec(10u32, 100).0.ln_prec_ref(5);
    /// assert_eq!(ln.to_string(), "2.2");
    /// assert_eq!(o, Less);
    ///
    /// let (ln, o) = Float::from_unsigned_prec(10u32, 100).0.ln_prec_ref(20);
    /// assert_eq!(ln.to_string(), "2.302586");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn ln_prec_ref(&self, prec: u64) -> (Self, Ordering) {
        self.ln_prec_round_ref(prec, Nearest)
    }

    /// Computes the natural logarithm of a [`Float`], rounding the result with the specified
    /// rounding mode. The [`Float`] is taken by value. An [`Ordering`] is also returned, indicating
    /// whether the rounded logarithm is less than, equal to, or greater than the exact logarithm.
    /// Although `NaN`s are not comparable to any [`Float`], whenever this function returns a `NaN`
    /// it also returns `Equal`.
    ///
    /// The logarithm of any nonzero negative number is `NaN`.
    ///
    /// The precision of the output is the precision of the input. See [`RoundingMode`] for a
    /// description of the possible rounding modes.
    ///
    /// $$
    /// f(x,m) = \ln{x}+\varepsilon.
    /// $$
    /// - If $\ln{x}$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $\ln{x}$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 \|ln{x}|\rfloor-p+1}$, where $p$ is the precision of the input.
    /// - If $\ln{x}$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 \|ln{x}|\rfloor-p}$, where $p$ is the precision of the input.
    ///
    /// If the output has a precision, it is the precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN},m)=\text{NaN}$
    /// - $f(\infty,m)=\infty$
    /// - $f(-\infty,m)=\text{NaN}$
    /// - $f(\pm0.0,m)=-\infty$
    ///
    /// Neither overflow nor underflow is possible.
    ///
    /// If you want to specify an output precision, consider using [`Float::ln_prec_round`] instead.
    /// If you know you'll be using the `Nearest` rounding mode, consider using [`Float::ln`]
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n (\log n)^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.get_prec()`.
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
    /// let (ln, o) = Float::from_unsigned_prec(10u32, 100).0.ln_round(Floor);
    /// assert_eq!(ln.to_string(), "2.302585092994045684017991454684");
    /// assert_eq!(o, Less);
    ///
    /// let (ln, o) = Float::from_unsigned_prec(10u32, 100).0.ln_round(Ceiling);
    /// assert_eq!(ln.to_string(), "2.302585092994045684017991454687");
    /// assert_eq!(o, Greater);
    ///
    /// let (ln, o) = Float::from_unsigned_prec(10u32, 100).0.ln_round(Nearest);
    /// assert_eq!(ln.to_string(), "2.302585092994045684017991454684");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn ln_round(self, rm: RoundingMode) -> (Self, Ordering) {
        let prec = self.significant_bits();
        self.ln_prec_round(prec, rm)
    }

    /// Computes the natural logarithm of a [`Float`], rounding the result with the specified
    /// rounding mode. The [`Float`] is taken by reference. An [`Ordering`] is also returned,
    /// indicating whether the rounded logarithm is less than, equal to, or greater than the exact
    /// logarithm. Although `NaN`s are not comparable to any [`Float`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// The logarithm of any nonzero negative number is `NaN`.
    ///
    /// The precision of the output is the precision of the input. See [`RoundingMode`] for a
    /// description of the possible rounding modes.
    ///
    /// $$
    /// f(x,m) = \ln{x}+\varepsilon.
    /// $$
    /// - If $\ln{x}$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $\ln{x}$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 \|ln{x}|\rfloor-p+1}$, where $p$ is the precision of the input.
    /// - If $\ln{x}$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 \|ln{x}|\rfloor-p}$, where $p$ is the precision of the input.
    ///
    /// If the output has a precision, it is the precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN},m)=\text{NaN}$
    /// - $f(\infty,m)=\infty$
    /// - $f(-\infty,m)=\text{NaN}$
    /// - $f(\pm0.0,m)=-\infty$
    ///
    /// Neither overflow nor underflow is possible.
    ///
    /// If you want to specify an output precision, consider using [`Float::ln_prec_round_ref`]
    /// instead. If you know you'll be using the `Nearest` rounding mode, consider using
    /// `(&Float).ln()` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n (\log n)^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.get_prec()`.
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
    /// let (ln, o) = Float::from_unsigned_prec(10u32, 100).0.ln_round_ref(Floor);
    /// assert_eq!(ln.to_string(), "2.302585092994045684017991454684");
    /// assert_eq!(o, Less);
    ///
    /// let (ln, o) = Float::from_unsigned_prec(10u32, 100)
    ///     .0
    ///     .ln_round_ref(Ceiling);
    /// assert_eq!(ln.to_string(), "2.302585092994045684017991454687");
    /// assert_eq!(o, Greater);
    ///
    /// let (ln, o) = Float::from_unsigned_prec(10u32, 100)
    ///     .0
    ///     .ln_round_ref(Nearest);
    /// assert_eq!(ln.to_string(), "2.302585092994045684017991454684");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn ln_round_ref(&self, rm: RoundingMode) -> (Self, Ordering) {
        let prec = self.significant_bits();
        self.ln_prec_round_ref(prec, rm)
    }

    /// Computes the natural logarithm of a [`Float`] in place, rounding the result to the specified
    /// precision and with the specified rounding mode. An [`Ordering`] is returned, indicating
    /// whether the rounded logarithm is less than, equal to, or greater than the exact logarithm.
    /// Although `NaN`s are not comparable to any [`Float`], whenever this function sets the
    /// [`Float`] to `NaN` it also returns `Equal`.
    ///
    /// The logarithm of any nonzero negative number is `NaN`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets \ln{x}+\varepsilon.
    /// $$
    /// - If $\ln{x}$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $\ln{x}$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy|\rfloor-p+1}$.
    /// - If $\ln{x}$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 \|ln{x}|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// See the [`Float::ln_prec_round`] documentation for information on special cases, overflow,
    /// and underflow.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::ln_prec_assign`] instead. If
    /// you know that your target precision is the precision of the input, consider using
    /// [`Float::ln_round_assign`] instead. If both of these things are true, consider using
    /// [`Float::ln_assign`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n (\log n)^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the result cannot be represented exactly with the given
    /// precision.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from_unsigned_prec(10u32, 100).0;
    /// assert_eq!(x.ln_prec_round_assign(5, Floor), Less);
    /// assert_eq!(x.to_string(), "2.2");
    ///
    /// let mut x = Float::from_unsigned_prec(10u32, 100).0;
    /// assert_eq!(x.ln_prec_round_assign(5, Ceiling), Greater);
    /// assert_eq!(x.to_string(), "2.4");
    ///
    /// let mut x = Float::from_unsigned_prec(10u32, 100).0;
    /// assert_eq!(x.ln_prec_round_assign(5, Nearest), Less);
    /// assert_eq!(x.to_string(), "2.2");
    ///
    /// let mut x = Float::from_unsigned_prec(10u32, 100).0;
    /// assert_eq!(x.ln_prec_round_assign(20, Floor), Less);
    /// assert_eq!(x.to_string(), "2.302582");
    ///
    /// let mut x = Float::from_unsigned_prec(10u32, 100).0;
    /// assert_eq!(x.ln_prec_round_assign(20, Ceiling), Greater);
    /// assert_eq!(x.to_string(), "2.302586");
    ///
    /// let mut x = Float::from_unsigned_prec(10u32, 100).0;
    /// assert_eq!(x.ln_prec_round_assign(20, Nearest), Greater);
    /// assert_eq!(x.to_string(), "2.302586");
    /// ```
    #[inline]
    pub fn ln_prec_round_assign(&mut self, prec: u64, rm: RoundingMode) -> Ordering {
        let mut x = Self::ZERO;
        swap(self, &mut x);
        let o;
        (*self, o) = x.ln_prec_round(prec, rm);
        o
    }

    /// Computes the natural logarithm of a [`Float`] in place, rounding the result to the nearest
    /// value of the specified precision. An [`Ordering`] is returned, indicating whether the
    /// rounded logarithm is less than, equal to, or greater than the exact logarithm. Although
    /// `NaN`s are not comparable to any [`Float`], whenever this function sets the [`Float`] to
    /// `NaN` it also returns `Equal`.
    ///
    /// The logarithm of any nonzero negative number is `NaN`.
    ///
    /// If the logarithm is equidistant from two [`Float`]s with the specified precision, the
    /// [`Float`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a
    /// description of the `Nearest` rounding mode.
    ///
    /// $$
    /// x \gets \ln{x}+\varepsilon.
    /// $$
    /// - If $\ln{x}$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $\ln{x}$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2
    ///   \ln{x}\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// See the [`Float::ln_prec`] documentation for information on special cases, overflow, and
    /// underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::ln_prec_round_assign`] instead. If you know that your target precision is the
    /// precision of the input, consider using [`Float::ln`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n (\log n)^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from_unsigned_prec(10u32, 100).0;
    /// assert_eq!(x.ln_prec_assign(5), Less);
    /// assert_eq!(x.to_string(), "2.2");
    ///
    /// let mut x = Float::from_unsigned_prec(10u32, 100).0;
    /// assert_eq!(x.ln_prec_assign(20), Greater);
    /// assert_eq!(x.to_string(), "2.302586");
    /// ```
    #[inline]
    pub fn ln_prec_assign(&mut self, prec: u64) -> Ordering {
        self.ln_prec_round_assign(prec, Nearest)
    }

    /// Computes the natural logarithm of a [`Float`] in place, rounding the result with the
    /// specified rounding mode. An [`Ordering`] is returned, indicating whether the rounded
    /// logarithm is less than, equal to, or greater than the exact logarithm. Although `NaN`s are
    /// not comparable to any [`Float`], whenever this function sets the [`Float`] to `NaN` it also
    /// returns `Equal`.
    ///
    /// The logarithm of any nonzero negative number is `NaN`.
    ///
    /// The precision of the output is the precision of the input. See [`RoundingMode`] for a
    /// description of the possible rounding modes.
    ///
    /// $$
    /// x \gets \ln{x}+\varepsilon.
    /// $$
    /// - If $\ln{x}$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $\ln{x}$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 \|ln{x}|\rfloor-p+1}$, where $p$ is the maximum precision of the inputs.
    /// - If $\ln{x}$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 \|ln{x}|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// If the output has a precision, it is the precision of the input.
    ///
    /// See the [`Float::ln_round`] documentation for information on special cases, overflow, and
    /// underflow.
    ///
    /// If you want to specify an output precision, consider using [`Float::ln_prec_round_assign`]
    /// instead. If you know you'll be using the `Nearest` rounding mode, consider using
    /// [`Float::ln_assign`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n (\log n)^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.get_prec()`.
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
    /// let mut x = Float::from_unsigned_prec(10u32, 100).0;
    /// assert_eq!(x.ln_round_assign(Floor), Less);
    /// assert_eq!(x.to_string(), "2.302585092994045684017991454684");
    ///
    /// let mut x = Float::from_unsigned_prec(10u32, 100).0;
    /// assert_eq!(x.ln_round_assign(Ceiling), Greater);
    /// assert_eq!(x.to_string(), "2.302585092994045684017991454687");
    ///
    /// let mut x = Float::from_unsigned_prec(10u32, 100).0;
    /// assert_eq!(x.ln_round_assign(Nearest), Less);
    /// assert_eq!(x.to_string(), "2.302585092994045684017991454684");
    /// ```
    #[inline]
    pub fn ln_round_assign(&mut self, rm: RoundingMode) -> Ordering {
        let prec = self.significant_bits();
        self.ln_prec_round_assign(prec, rm)
    }

    /// Computes the natural logarithm of a [`Rational`], rounding the result to the specified
    /// precision and with the specified rounding mode and returning the result as a [`Float`]. The
    /// [`Rational`] is taken by value. An [`Ordering`] is also returned, indicating whether the
    /// rounded logarithm is less than, equal to, or greater than the exact logarithm. Although
    /// `NaN`s are not comparable to any [`Float`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// The logarithm of any nonzero negative number is `NaN`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \ln{x}+\varepsilon.
    /// $$
    /// - If $\ln{x}$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $\ln{x}$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\ln{x}|\rfloor-p+1}$.
    /// - If $\ln{x}$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\ln{x}|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(0.0,p,m)=-\infty$
    ///
    /// Neither overflow nor underflow is possible.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::ln_rational_prec`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n (\log n)^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the result cannot be represented exactly with the given
    /// precision.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (ln, o) = Float::ln_rational_prec_round(Rational::from_unsigneds(3u8, 5), 5, Floor);
    /// assert_eq!(ln.to_string(), "-0.53");
    /// assert_eq!(o, Less);
    ///
    /// let (ln, o) = Float::ln_rational_prec_round(Rational::from_unsigneds(3u8, 5), 5, Ceiling);
    /// assert_eq!(ln.to_string(), "-0.5");
    /// assert_eq!(o, Greater);
    ///
    /// let (ln, o) = Float::ln_rational_prec_round(Rational::from_unsigneds(3u8, 5), 5, Nearest);
    /// assert_eq!(ln.to_string(), "-0.5");
    /// assert_eq!(o, Greater);
    ///
    /// let (ln, o) = Float::ln_rational_prec_round(Rational::from_unsigneds(3u8, 5), 20, Floor);
    /// assert_eq!(ln.to_string(), "-0.510826");
    /// assert_eq!(o, Less);
    ///
    /// let (ln, o) = Float::ln_rational_prec_round(Rational::from_unsigneds(3u8, 5), 20, Ceiling);
    /// assert_eq!(ln.to_string(), "-0.510825");
    /// assert_eq!(o, Greater);
    ///
    /// let (ln, o) = Float::ln_rational_prec_round(Rational::from_unsigneds(3u8, 5), 20, Nearest);
    /// assert_eq!(ln.to_string(), "-0.510825");
    /// assert_eq!(o, Greater);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn ln_rational_prec_round(x: Rational, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        Self::ln_rational_prec_round_ref(&x, prec, rm)
    }

    /// Computes the natural logarithm of a [`Rational`], rounding the result to the specified
    /// precision and with the specified rounding mode and returning the result as a [`Float`]. The
    /// [`Rational`] is taken by reference. An [`Ordering`] is also returned, indicating whether the
    /// rounded logarithm is less than, equal to, or greater than the exact logarithm. Although
    /// `NaN`s are not comparable to any [`Float`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// The logarithm of any nonzero negative number is `NaN`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \ln{x}+\varepsilon.
    /// $$
    /// - If $\ln{x}$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $\ln{x}$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\ln{x}|\rfloor-p+1}$.
    /// - If $\ln{x}$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\ln{x}|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(0.0,p,m)=-\infty$
    ///
    /// Neither overflow nor underflow is possible.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::ln_rational_prec_ref`]
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n (\log n)^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the result cannot be represented exactly with the given
    /// precision.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (ln, o) =
    ///     Float::ln_rational_prec_round_ref(&Rational::from_unsigneds(3u8, 5), 5, Floor);
    /// assert_eq!(ln.to_string(), "-0.53");
    /// assert_eq!(o, Less);
    ///
    /// let (ln, o) =
    ///     Float::ln_rational_prec_round_ref(&Rational::from_unsigneds(3u8, 5), 5, Ceiling);
    /// assert_eq!(ln.to_string(), "-0.5");
    /// assert_eq!(o, Greater);
    ///
    /// let (ln, o) =
    ///     Float::ln_rational_prec_round_ref(&Rational::from_unsigneds(3u8, 5), 5, Nearest);
    /// assert_eq!(ln.to_string(), "-0.5");
    /// assert_eq!(o, Greater);
    ///
    /// let (ln, o) =
    ///     Float::ln_rational_prec_round_ref(&Rational::from_unsigneds(3u8, 5), 20, Floor);
    /// assert_eq!(ln.to_string(), "-0.510826");
    /// assert_eq!(o, Less);
    ///
    /// let (ln, o) =
    ///     Float::ln_rational_prec_round_ref(&Rational::from_unsigneds(3u8, 5), 20, Ceiling);
    /// assert_eq!(ln.to_string(), "-0.510825");
    /// assert_eq!(o, Greater);
    ///
    /// let (ln, o) =
    ///     Float::ln_rational_prec_round_ref(&Rational::from_unsigneds(3u8, 5), 20, Nearest);
    /// assert_eq!(ln.to_string(), "-0.510825");
    /// assert_eq!(o, Greater);
    /// ```
    pub fn ln_rational_prec_round_ref(
        x: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        assert_ne!(prec, 0);
        match x.sign() {
            Equal => return (float_negative_infinity!(), Equal),
            Less => return (float_nan!(), Equal),
            Greater => {}
        }
        if *x == 1u32 {
            return (float_zero!(), Equal);
        }
        assert_ne!(rm, Exact, "Inexact ln");
        // x within a sliver of 1: ln(x) ~ x - 1 may fall below the smallest positive Float, which
        // the helpers below could never resolve.
        let eps = x - Rational::ONE;
        if eps.floor_log_base_2_abs() <= i64::from(Self::MIN_EXPONENT) + 4 {
            return ln_rational_near_one(&eps, prec, rm);
        }
        let x_exp = i32::saturating_from(x.floor_log_base_2_abs()).saturating_add(1);
        if x_exp >= Self::MAX_EXPONENT - 1 || x_exp <= Self::MIN_EXPONENT + 1 {
            ln_rational_helper_extended(x, prec, rm)
        } else {
            ln_rational_helper(x, prec, rm)
        }
    }

    /// Computes the natural logarithm of a [`Rational`], rounding the result to the nearest value
    /// of the specified precision and returning the result as a [`Float`]. The [`Rational`] is
    /// taken by value. An [`Ordering`] is also returned, indicating whether the rounded logarithm
    /// is less than, equal to, or greater than the exact logarithm. Although `NaN`s are not
    /// comparable to any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The logarithm of any nonzero negative number is `NaN`.
    ///
    /// If the logarithm is equidistant from two [`Float`]s with the specified precision, the
    /// [`Float`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a
    /// description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = \ln{x}+\varepsilon.
    /// $$
    /// - If $\ln{x}$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $\ln{x}$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2
    ///   |\ln{x}|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(0.0,p)=-\infty$
    ///
    /// Neither overflow nor underflow is possible.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::ln_rational_prec_round`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n (\log n)^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (ln, o) = Float::ln_rational_prec(Rational::from_unsigneds(3u8, 5), 5);
    /// assert_eq!(ln.to_string(), "-0.5");
    /// assert_eq!(o, Greater);
    ///
    /// let (ln, o) = Float::ln_rational_prec(Rational::from_unsigneds(3u8, 5), 20);
    /// assert_eq!(ln.to_string(), "-0.510825");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn ln_rational_prec(x: Rational, prec: u64) -> (Self, Ordering) {
        Self::ln_rational_prec_round(x, prec, Nearest)
    }

    /// Computes the natural logarithm of a [`Rational`], rounding the result to the nearest value
    /// of the specified precision and returning the result as a [`Float`]. The [`Rational`] is
    /// taken by reference. An [`Ordering`] is also returned, indicating whether the rounded
    /// logarithm is less than, equal to, or greater than the exact logarithm. Although `NaN`s are
    /// not comparable to any [`Float`], whenever this function returns a `NaN` it also returns
    /// `Equal`.
    ///
    /// The logarithm of any nonzero negative number is `NaN`.
    ///
    /// If the logarithm is equidistant from two [`Float`]s with the specified precision, the
    /// [`Float`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a
    /// description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = \ln{x}+\varepsilon.
    /// $$
    /// - If $\ln{x}$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $\ln{x}$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2
    ///   |\ln{x}|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(0.0,p)=-\infty$
    ///
    /// Neither overflow nor underflow is possible.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::ln_rational_prec_round_ref`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n (\log n)^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (ln, o) = Float::ln_rational_prec_ref(&Rational::from_unsigneds(3u8, 5), 5);
    /// assert_eq!(ln.to_string(), "-0.5");
    /// assert_eq!(o, Greater);
    ///
    /// let (ln, o) = Float::ln_rational_prec_ref(&Rational::from_unsigneds(3u8, 5), 20);
    /// assert_eq!(ln.to_string(), "-0.510825");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn ln_rational_prec_ref(x: &Rational, prec: u64) -> (Self, Ordering) {
        Self::ln_rational_prec_round_ref(x, prec, Nearest)
    }

    /// Computes the natural logarithm of an unsigned integer, returning a [`Float`]. The result is
    /// rounded to the specified precision and with the specified rounding mode. An [`Ordering`] is
    /// also returned, indicating whether the rounded logarithm is less than, equal to, or greater
    /// than the exact logarithm.
    ///
    /// This is typically faster than converting the integer to a [`Float`] and taking its
    /// logarithm, as it uses binary splitting of the Taylor series of the logarithm rather than the
    /// arithmetic-geometric mean iteration.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(n,p,m) = \ln{n}+\varepsilon.
    /// $$
    /// - If $\ln{n}$ is infinite or zero, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $\ln{n}$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 (\ln{n})\rfloor-p+1}$.
    /// - If $\ln{n}$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 (\ln{n})\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(0,p,m)=-\infty$
    /// - $f(1,p,m)=0.0$
    ///
    /// Neither overflow nor underflow is possible.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::ln_unsigned_prec`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n (\log n)^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the logarithm is irrational (that is,
    /// whenever $n \geq 2$).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (ln, o) = Float::ln_unsigned_prec_round(10, 100, Floor);
    /// assert_eq!(ln.to_string(), "2.302585092994045684017991454684");
    /// assert_eq!(o, Less);
    ///
    /// let (ln, o) = Float::ln_unsigned_prec_round(10, 100, Ceiling);
    /// assert_eq!(ln.to_string(), "2.302585092994045684017991454687");
    /// assert_eq!(o, Greater);
    ///
    /// let (ln, o) = Float::ln_unsigned_prec_round(1, 10, Exact);
    /// assert_eq!(ln.to_string(), "0.0");
    /// assert_eq!(o, Equal);
    /// ```
    ///
    /// This is `mpfr_log_ui` from `log_ui.c`, MPFR 4.2.2.
    pub fn ln_unsigned_prec_round(n: u64, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        assert_ne!(prec, 0);
        match n {
            // log(0) is an exact -Infinity
            0 => (Self::NEGATIVE_INFINITY, Equal),
            // log(1) = 0, the only "normal" case where the result is exact
            1 => (Self::ZERO, Equal),
            2 => Self::ln_2_prec_round(prec, rm),
            _ => ln_unsigned_prec_round_normal(n, prec, rm),
        }
    }

    /// Computes the natural logarithm of an unsigned integer, returning a [`Float`]. The result is
    /// rounded to the specified precision and to the nearest value. An [`Ordering`] is also
    /// returned, indicating whether the rounded logarithm is less than, equal to, or greater than
    /// the exact logarithm.
    ///
    /// This is typically faster than converting the integer to a [`Float`] and taking its
    /// logarithm, as it uses binary splitting of the Taylor series of the logarithm rather than the
    /// arithmetic-geometric mean iteration.
    ///
    /// If the logarithm is equidistant from two [`Float`]s with the specified precision, the
    /// [`Float`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a
    /// description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(n,p) = \ln{n}+\varepsilon.
    /// $$
    /// - If $\ln{n}$ is infinite or zero, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $\ln{n}$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   (\ln{n})\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(0,p)=-\infty$
    /// - $f(1,p)=0.0$
    ///
    /// Neither overflow nor underflow is possible.
    ///
    /// If you want to specify a rounding mode as well, consider using
    /// [`Float::ln_unsigned_prec_round`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n (\log n)^2)$
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
    /// let (ln, o) = Float::ln_unsigned_prec(10, 100);
    /// assert_eq!(ln.to_string(), "2.302585092994045684017991454684");
    /// assert_eq!(o, Less);
    ///
    /// let (ln, o) = Float::ln_unsigned_prec(0, 10);
    /// assert_eq!(ln.to_string(), "-Infinity");
    /// assert_eq!(o, Equal);
    /// ```
    #[inline]
    pub fn ln_unsigned_prec(n: u64, prec: u64) -> (Self, Ordering) {
        Self::ln_unsigned_prec_round(n, prec, Nearest)
    }
}

impl Ln for Float {
    type Output = Self;

    /// Computes the natural logarithm of a [`Float`], taking it by value.
    ///
    /// If the output has a precision, it is the precision of the input. If the logarithm is
    /// equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s in
    /// its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// The logarithm of any nonzero negative number is `NaN`.
    ///
    /// $$
    /// f(x) = \ln{x}+\varepsilon.
    /// $$
    /// - If $\ln{x}$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $\ln{x}$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2
    ///   \ln{x}\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// Special cases:
    /// - $f(\text{NaN})=\text{NaN}$
    /// - $f(\infty)=\infty$
    /// - $f(-\infty)=\text{NaN}$
    /// - $f(\pm0.0)=-\infty$
    ///
    /// Neither overflow nor underflow is possible.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using [`Float::ln_prec`]
    /// instead. If you want to specify the output precision, consider using [`Float::ln_round`]. If
    /// you want both of these things, consider using [`Float::ln_prec_round`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n (\log n)^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.get_prec()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Ln;
    /// use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity};
    /// use malachite_float::Float;
    ///
    /// assert!(Float::NAN.ln().is_nan());
    /// assert_eq!(Float::INFINITY.ln(), Float::INFINITY);
    /// assert!(Float::NEGATIVE_INFINITY.ln().is_nan());
    /// assert_eq!(
    ///     Float::from_unsigned_prec(10u32, 100).0.ln().to_string(),
    ///     "2.302585092994045684017991454684"
    /// );
    /// assert!(Float::from_signed_prec(-10, 100).0.ln().is_nan());
    /// ```
    #[inline]
    fn ln(self) -> Self {
        let prec = self.significant_bits();
        self.ln_prec_round(prec, Nearest).0
    }
}

impl Ln for &Float {
    type Output = Float;

    /// Computes the natural logarithm of a [`Float`], taking it by reference.
    ///
    /// If the output has a precision, it is the precision of the input. If the logarithm is
    /// equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s in
    /// its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// The logarithm of any nonzero negative number is `NaN`.
    ///
    /// $$
    /// f(x) = \ln{x}+\varepsilon.
    /// $$
    /// - If $\ln{x}$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $\ln{x}$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2
    ///   \ln{x}\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// Special cases:
    /// - $f(\text{NaN})=\text{NaN}$
    /// - $f(\infty)=\infty$
    /// - $f(-\infty)=\text{NaN}$
    /// - $f(\pm0.0)=-\infty$
    ///
    /// Neither overflow nor underflow is possible.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::ln_prec_ref`] instead. If you want to specify the output precision, consider using
    /// [`Float::ln_round_ref`]. If you want both of these things, consider using
    /// [`Float::ln_prec_round_ref`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n (\log n)^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.get_prec()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Ln;
    /// use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity};
    /// use malachite_float::Float;
    ///
    /// assert!((&Float::NAN).ln().is_nan());
    /// assert_eq!((&Float::INFINITY).ln(), Float::INFINITY);
    /// assert!((&Float::NEGATIVE_INFINITY).ln().is_nan());
    /// assert_eq!(
    ///     (&Float::from_unsigned_prec(10u32, 100).0).ln().to_string(),
    ///     "2.302585092994045684017991454684"
    /// );
    /// assert!((&Float::from_signed_prec(-10, 100).0).ln().is_nan());
    /// ```
    #[inline]
    fn ln(self) -> Float {
        let prec = self.significant_bits();
        self.ln_prec_round_ref(prec, Nearest).0
    }
}

impl LnAssign for Float {
    /// Computes the natural logarithm of a [`Float`] in place.
    ///
    /// If the output has a precision, it is the precision of the input. If the logarithm is
    /// equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s in
    /// its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// The logarithm of any nonzero negative number is `NaN`.
    ///
    /// $$
    /// x\gets = \ln{x}+\varepsilon.
    /// $$
    /// - If $\ln{x}$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $\ln{x}$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2
    ///   \ln{x}\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// See the [`Float::ln`] documentation for information on special cases, overflow, and
    /// underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::ln_prec_assign`] instead. If you want to specify the output precision, consider
    /// using [`Float::ln_round_assign`]. If you want both of these things, consider using
    /// [`Float::ln_prec_round_assign`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n (\log n)^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.get_prec()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::LnAssign;
    /// use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity};
    /// use malachite_float::Float;
    ///
    /// let mut x = Float::NAN;
    /// x.ln_assign();
    /// assert!(x.is_nan());
    ///
    /// let mut x = Float::INFINITY;
    /// x.ln_assign();
    /// assert_eq!(x, Float::INFINITY);
    ///
    /// let mut x = Float::NEGATIVE_INFINITY;
    /// x.ln_assign();
    /// assert!(x.is_nan());
    ///
    /// let mut x = Float::from_unsigned_prec(10u32, 100).0;
    /// x.ln_assign();
    /// assert_eq!(x.to_string(), "2.302585092994045684017991454684");
    ///
    /// let mut x = Float::from_signed_prec(-10, 100).0;
    /// x.ln_assign();
    /// assert!(x.is_nan());
    /// ```
    #[inline]
    fn ln_assign(&mut self) {
        let prec = self.significant_bits();
        self.ln_prec_round_assign(prec, Nearest);
    }
}

/// Computes the natural logarithm of a primitive float. Using this function is more accurate than
/// using the default `log` function or the one provided by `libm`.
///
/// The reciprocal logarithm of any nonzero negative number is `NaN`.
///
/// $$
/// f(x) = \ln x+\varepsilon.
/// $$
/// - If $\ln x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
/// - If $\ln x$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2 \ln x\rfloor-p}$,
///   where $p$ is precision of the output (typically 24 if `T` is a [`f32`] and 53 if `T` is a
///   [`f64`], but less if the output is subnormal).
///
/// Special cases:
/// - $f(\text{NaN})=\text{NaN}$
/// - $f(\infty)=\infty$
/// - $f(-\infty)=\text{NaN}$
/// - $f(\pm0.0)=-\infty$
///
/// Neither overflow nor underflow is possible.
///
/// # Worst-case complexity
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use malachite_base::num::basic::traits::NegativeInfinity;
/// use malachite_base::num::float::NiceFloat;
/// use malachite_float::arithmetic::ln::primitive_float_ln;
///
/// assert!(primitive_float_ln(f32::NAN).is_nan());
/// assert_eq!(
///     NiceFloat(primitive_float_ln(f32::INFINITY)),
///     NiceFloat(f32::INFINITY)
/// );
/// assert!(primitive_float_ln(f32::NEGATIVE_INFINITY).is_nan());
/// assert_eq!(NiceFloat(primitive_float_ln(10.0f32)), NiceFloat(2.3025851));
/// assert!(primitive_float_ln(-10.0f32).is_nan());
/// ```
#[inline]
#[allow(clippy::type_repetition_in_bounds)]
pub fn primitive_float_ln<T: PrimitiveFloat>(x: T) -> T
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    emulate_float_to_float_fn(Float::ln_prec, x)
}

/// Computes the natural logarithm of a [`Rational`], returning a primitive float result.
///
/// If the logarithm is equidistant from two primitive floats, the primitive float with fewer 1s in
/// its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest` rounding
/// mode.
///
/// The logarithm of any negative number is `NaN`.
///
/// $$
/// f(x) = \ln{x}+\varepsilon.
/// $$
/// - If $\ln{x}$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
/// - If $\ln{x}$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2 |\ln{x}|\rfloor-p}$,
///   where $p$ is precision of the output (typically 24 if `T` is a [`f32`] and 53 if `T` is a
///   [`f64`], but less if the output is subnormal).
///
/// Special cases:
/// - $f(0)=-\infty$
///
/// Neither overflow nor underflow is possible.
///
/// # Worst-case complexity
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use malachite_base::num::basic::traits::{NegativeInfinity, Zero};
/// use malachite_base::num::float::NiceFloat;
/// use malachite_float::arithmetic::ln::primitive_float_ln_rational;
/// use malachite_q::Rational;
///
/// assert_eq!(
///     NiceFloat(primitive_float_ln_rational::<f64>(&Rational::ZERO)),
///     NiceFloat(f64::NEGATIVE_INFINITY)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_ln_rational::<f64>(
///         &Rational::from_unsigneds(1u8, 3)
///     )),
///     NiceFloat(-1.0986122886681098)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_ln_rational::<f64>(&Rational::from(10000))),
///     NiceFloat(9.210340371976184)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_ln_rational::<f64>(&Rational::from(-10000))),
///     NiceFloat(f64::NAN)
/// );
/// ```
#[inline]
#[allow(clippy::type_repetition_in_bounds)]
pub fn primitive_float_ln_rational<T: PrimitiveFloat>(x: &Rational) -> T
where
    Float: PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    emulate_rational_to_float_fn(Float::ln_rational_prec_ref, x)
}
