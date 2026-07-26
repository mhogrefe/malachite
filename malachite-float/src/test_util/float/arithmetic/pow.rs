// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::test_util::common::rug_float_significant_bits;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_q::Rational;
use rug::float::{Round, Special};
use rug::ops::AssignRound;
use rug::ops::Pow;
use std::cmp::Ordering::{self, *};
use std::cmp::max;

pub fn rug_pow_prec_round(
    x: &rug::Float,
    y: &rug::Float,
    prec: u64,
    rm: Round,
) -> (rug::Float, Ordering) {
    let mut power = rug::Float::with_val(u32::exact_from(prec), 0);
    let o = power.assign_round(Pow::pow(x, y), rm);
    (power, o)
}

#[inline]
pub fn rug_pow_round(x: &rug::Float, y: &rug::Float, rm: Round) -> (rug::Float, Ordering) {
    rug_pow_prec_round(
        x,
        y,
        max(rug_float_significant_bits(x), rug_float_significant_bits(y)),
        rm,
    )
}

#[inline]
pub fn rug_pow_prec(x: &rug::Float, y: &rug::Float, prec: u64) -> (rug::Float, Ordering) {
    rug_pow_prec_round(x, y, prec, Round::Nearest)
}

pub fn rug_pow(x: &rug::Float, y: &rug::Float) -> rug::Float {
    rug_pow_prec_round(
        x,
        y,
        max(rug_float_significant_bits(x), rug_float_significant_bits(y)),
        Round::Nearest,
    )
    .0
}

pub fn rug_pow_integer_prec_round(
    x: &rug::Float,
    y: &rug::Integer,
    prec: u64,
    rm: Round,
) -> (rug::Float, Ordering) {
    let mut power = rug::Float::with_val(u32::exact_from(prec), 0);
    let o = power.assign_round(Pow::pow(x, y), rm);
    (power, o)
}

#[inline]
pub fn rug_pow_integer_round(
    x: &rug::Float,
    y: &rug::Integer,
    rm: Round,
) -> (rug::Float, Ordering) {
    rug_pow_integer_prec_round(x, y, rug_float_significant_bits(x), rm)
}

#[inline]
pub fn rug_pow_integer_prec(x: &rug::Float, y: &rug::Integer, prec: u64) -> (rug::Float, Ordering) {
    rug_pow_integer_prec_round(x, y, prec, Round::Nearest)
}

pub fn rug_pow_integer(x: &rug::Float, y: &rug::Integer) -> rug::Float {
    rug_pow_integer_prec_round(x, y, rug_float_significant_bits(x), Round::Nearest).0
}

// rug has no direct binding to `mpfr_pow_ui`, so these oracles for `x^n` (a `u64` n) use
// `mpfr_pow_z` via a `rug::Integer`; it is correctly rounded and so gives the same result.
pub fn rug_pow_u_prec_round(
    x: &rug::Float,
    n: u64,
    prec: u64,
    rm: Round,
) -> (rug::Float, Ordering) {
    rug_pow_integer_prec_round(x, &rug::Integer::from(n), prec, rm)
}

#[inline]
pub fn rug_pow_u_round(x: &rug::Float, n: u64, rm: Round) -> (rug::Float, Ordering) {
    rug_pow_u_prec_round(x, n, rug_float_significant_bits(x), rm)
}

#[inline]
pub fn rug_pow_u_prec(x: &rug::Float, n: u64, prec: u64) -> (rug::Float, Ordering) {
    rug_pow_u_prec_round(x, n, prec, Round::Nearest)
}

pub fn rug_pow_u(x: &rug::Float, n: u64) -> rug::Float {
    rug_pow_u_prec_round(x, n, rug_float_significant_bits(x), Round::Nearest).0
}

// As with `rug_pow_u`, these oracles for `x^n` (an `i64` n) use `mpfr_pow_z` via a `rug::Integer`;
// it is correctly rounded and so gives the same result as `mpfr_pow_si`.
pub fn rug_pow_s_prec_round(
    x: &rug::Float,
    n: i64,
    prec: u64,
    rm: Round,
) -> (rug::Float, Ordering) {
    rug_pow_integer_prec_round(x, &rug::Integer::from(n), prec, rm)
}

#[inline]
pub fn rug_pow_s_round(x: &rug::Float, n: i64, rm: Round) -> (rug::Float, Ordering) {
    rug_pow_s_prec_round(x, n, rug_float_significant_bits(x), rm)
}

#[inline]
pub fn rug_pow_s_prec(x: &rug::Float, n: i64, prec: u64) -> (rug::Float, Ordering) {
    rug_pow_s_prec_round(x, n, prec, Round::Nearest)
}

pub fn rug_pow_s(x: &rug::Float, n: i64) -> rug::Float {
    rug_pow_s_prec_round(x, n, rug_float_significant_bits(x), Round::Nearest).0
}

// Oracle for k^q (a u64 k and Rational q) by bracketing exp(q ln k): compute ln(k), t = q ln(k),
// and exp(t) with directed rounding at an escalating working precision, round both bracket ends to
// `prec`, and return once the ends agree on the value and the ternary. The caller must ensure the
// true value is irrational (k >= 2 and k not a perfect b-th power of q's denominator), since the
// brackets can never resolve an exactly-representable result. Overflow and underflow are resolved
// explicitly: rug's default exponent range matches `Float`'s, so once the lower bracket pins the
// value above the largest finite prec-bit value the overflowed result is fully determined, and in
// the underflow region the `Nearest` tie at 2^(-2^30-1) -- which lies below rug's own exponent
// range -- is decided in t-space against a bracketed multiple of ln 2 (the sides always separate,
// since equality would make k^q dyadic).
pub fn rug_unsigned_pow_rational_prec_round(
    k: u64,
    q: &Rational,
    prec: u64,
    rm: Round,
) -> (rug::Float, Ordering) {
    let rug_q = rug::Rational::from(q);
    let q_positive = *q > 0u32;
    let target_prec = u32::exact_from(prec);
    // the largest finite prec-bit value
    let mut max_finite = rug::Float::with_val(target_prec, Special::Infinity);
    max_finite.next_down();
    // the smallest positive value, 2^(-2^30)
    let mut min_pos = rug::Float::with_val(target_prec, 0);
    min_pos.next_up();
    let mut working_prec = (prec << 1) + 128 + (q.significant_bits() << 1);
    loop {
        let wp = u32::exact_from(working_prec);
        // 0 < b_lo <= ln(k) <= b_hi
        let k_float = rug::Float::with_val(wp, k);
        let mut b_lo = rug::Float::with_val(wp, 0);
        b_lo.assign_round(k_float.ln_ref(), Round::Down);
        let mut b_hi = rug::Float::with_val(wp, 0);
        b_hi.assign_round(k_float.ln_ref(), Round::Up);
        // t_lo <= q ln(k) <= t_hi; which end of the ln bracket extremizes each end depends on the
        // sign of q
        let mut t_lo = rug::Float::with_val(wp, 0);
        t_lo.assign_round(if q_positive { &b_lo } else { &b_hi } * &rug_q, Round::Down);
        let mut t_hi = rug::Float::with_val(wp, 0);
        t_hi.assign_round(if q_positive { &b_hi } else { &b_lo } * &rug_q, Round::Up);
        // exp is increasing, so v_lo <= k^q <= v_hi
        let mut v_lo = rug::Float::with_val(wp, 0);
        v_lo.assign_round(t_lo.exp_ref(), Round::Down);
        let mut v_hi = rug::Float::with_val(wp, 0);
        v_hi.assign_round(t_hi.exp_ref(), Round::Up);
        let mut p_lo = rug::Float::with_val(target_prec, 0);
        let mut o_lo = p_lo.assign_round(&v_lo, rm);
        if v_hi.is_infinite() {
            // Overflow: exp saturates at the working precision itself, so the upper bracket can
            // never tighten. Once the lower bracket alone pins the true value at or above the
            // largest finite prec-bit value, the result is determined: up modes and `Nearest` give
            // infinity, and down modes the largest finite value (with a forced `Less`, since the
            // irrational true value lies strictly above it).
            if p_lo.is_infinite() {
                return (p_lo, Greater);
            }
            if p_lo == max_finite {
                return (p_lo, Less);
            }
        } else if v_lo.is_zero() && v_hi <= min_pos {
            // Underflow: 0 < k^q < 2^(-2^30) (irrationality rules out equality with the smallest
            // positive value).
            match rm {
                Round::Zero | Round::Down => {
                    return (rug::Float::with_val(target_prec, 0), Less);
                }
                Round::Up | Round::AwayZero => {
                    return (min_pos.clone(), Greater);
                }
                Round::Nearest => {
                    // k^q < 2^(-2^30-1) iff t < -(2^30+1) ln 2
                    const TIE_EXPONENT: i32 = -(1 << 30) - 1;
                    let two = rug::Float::with_val(wp, 2u32);
                    let mut c_lo = rug::Float::with_val(wp, 0);
                    c_lo.assign_round(two.ln_ref(), Round::Up);
                    c_lo *= TIE_EXPONENT;
                    let mut c_hi = rug::Float::with_val(wp, 0);
                    c_hi.assign_round(two.ln_ref(), Round::Down);
                    c_hi *= TIE_EXPONENT;
                    if t_hi < c_lo {
                        return (rug::Float::with_val(target_prec, 0), Less);
                    }
                    if t_lo > c_hi {
                        return (min_pos.clone(), Greater);
                    }
                }
                _ => unreachable!(),
            }
        } else {
            let mut p_hi = rug::Float::with_val(target_prec, 0);
            let mut o_hi = p_hi.assign_round(&v_hi, rm);
            // A bracket end that lands exactly on a representable value rounds with `Equal`; the
            // true value lies strictly between the ends, so the exact end adopts the other's
            // ternary.
            if o_lo == Equal {
                o_lo = o_hi;
            }
            if o_hi == Equal {
                o_hi = o_lo;
            }
            if p_lo == p_hi && o_lo == o_hi {
                return (p_lo, o_lo);
            }
        }
        working_prec += working_prec >> 1;
    }
}
