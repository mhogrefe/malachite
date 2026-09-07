// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Float;
use crate::float::arithmetic::cos::cos_basic;
use crate::float::arithmetic::sin::sin_basic;
use crate::float::arithmetic::sin_cos::{sin_cos_basic, sin_cos_fast};
use crate::test_util::common::rug_float_significant_bits;
use core::cmp::Ordering;
use malachite_base::num::arithmetic::traits::CeilingLogBase2;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode;
use malachite_q::Rational;
use rug::float::Round;
use rug::ops::AssignRound;

pub fn rug_sin_cos_prec_round(
    x: &rug::Float,
    prec: u64,
    rm: Round,
) -> (rug::Float, rug::Float, Ordering, Ordering) {
    let mut s = rug::Float::with_val(u32::exact_from(prec), 0);
    let mut c = rug::Float::with_val(u32::exact_from(prec), 0);
    let (o_s, o_c) = (&mut s, &mut c).assign_round(x.sin_cos_ref(), rm);
    (s, c, o_s, o_c)
}

pub fn rug_sin_cos_prec(x: &rug::Float, prec: u64) -> (rug::Float, rug::Float, Ordering, Ordering) {
    rug_sin_cos_prec_round(x, prec, Round::Nearest)
}

pub fn rug_sin_cos_round(
    x: &rug::Float,
    rm: Round,
) -> (rug::Float, rug::Float, Ordering, Ordering) {
    rug_sin_cos_prec_round(x, rug_float_significant_bits(x), rm)
}

pub fn rug_sin_cos(x: &rug::Float) -> (rug::Float, rug::Float) {
    let (s, c, _, _) = rug_sin_cos_prec_round(x, rug_float_significant_bits(x), Round::Nearest);
    (s, c)
}

// As for the sine oracle, the input carries its denominator's worth of extra bits.
pub fn rug_sin_cos_rational_prec_round(
    x: &Rational,
    prec: u64,
    rm: Round,
) -> (rug::Float, rug::Float, Ordering, Ordering) {
    let exponent_bits = if *x == 0u32 {
        0
    } else {
        u64::try_from(x.floor_log_base_2_abs()).unwrap_or(0)
    };
    let denominator_bits = x.denominator_ref().significant_bits();
    let rx = rug::Float::with_val(
        u32::exact_from(prec + 128 + exponent_bits + denominator_bits),
        rug::Rational::exact_from(x),
    );
    let mut s = rug::Float::with_val(u32::exact_from(prec), 0);
    let mut c = rug::Float::with_val(u32::exact_from(prec), 0);
    let (o_s, o_c) = (&mut s, &mut c).assign_round(rx.sin_cos_ref(), rm);
    (s, c, o_s, o_c)
}

pub fn rug_sin_cos_rational_prec(
    x: &Rational,
    prec: u64,
) -> (rug::Float, rug::Float, Ordering, Ordering) {
    rug_sin_cos_rational_prec_round(x, prec, Round::Nearest)
}

// The sine and cosine in `u`ths of a turn, which MPFR has no combined function for: `sin_u` and
// `cos_u` separately. `u` must fit in a `u32`, the type rug takes.
pub fn rug_sin_cos_with_period_prec_round(
    x: &rug::Float,
    u: u64,
    prec: u64,
    rm: Round,
) -> (rug::Float, rug::Float, Ordering, Ordering) {
    let u = u32::exact_from(u);
    let mut s = rug::Float::with_val(u32::exact_from(prec), 0);
    let o_s = s.assign_round(x.sin_u_ref(u), rm);
    let mut c = rug::Float::with_val(u32::exact_from(prec), 0);
    let o_c = c.assign_round(x.cos_u_ref(u), rm);
    (s, c, o_s, o_c)
}

pub fn rug_sin_cos_with_period_prec(
    x: &rug::Float,
    u: u64,
    prec: u64,
) -> (rug::Float, rug::Float, Ordering, Ordering) {
    rug_sin_cos_with_period_prec_round(x, u, prec, Round::Nearest)
}

// As for the sine oracle, the input carries its denominator's worth of extra bits.
pub fn rug_sin_cos_with_period_rational_prec_round(
    x: &Rational,
    u: u64,
    prec: u64,
    rm: Round,
) -> (rug::Float, rug::Float, Ordering, Ordering) {
    let exponent_bits = if *x == 0u32 {
        0
    } else {
        u64::try_from(x.floor_log_base_2_abs()).unwrap_or(0)
    };
    let denominator_bits = x.denominator_ref().significant_bits();
    let rx = rug::Float::with_val(
        u32::exact_from(prec + 128 + exponent_bits + denominator_bits),
        rug::Rational::exact_from(x),
    );
    rug_sin_cos_with_period_prec_round(&rx, u, prec, rm)
}

pub fn rug_sin_cos_with_period_rational_prec(
    x: &Rational,
    u: u64,
    prec: u64,
) -> (rug::Float, rug::Float, Ordering, Ordering) {
    rug_sin_cos_with_period_rational_prec_round(x, u, prec, Round::Nearest)
}

pub fn rug_sin_cos_pi_prec_round(
    x: &rug::Float,
    prec: u64,
    rm: Round,
) -> (rug::Float, rug::Float, Ordering, Ordering) {
    let mut s = rug::Float::with_val(u32::exact_from(prec), 0);
    let o_s = s.assign_round(x.sin_pi_ref(), rm);
    let mut c = rug::Float::with_val(u32::exact_from(prec), 0);
    let o_c = c.assign_round(x.cos_pi_ref(), rm);
    (s, c, o_s, o_c)
}

// As for the sine oracle, the input carries its denominator's worth of extra bits.
pub fn rug_sin_cos_pi_rational_prec_round(
    x: &Rational,
    prec: u64,
    rm: Round,
) -> (rug::Float, rug::Float, Ordering, Ordering) {
    let exponent_bits = if *x == 0u32 {
        0
    } else {
        u64::try_from(x.floor_log_base_2_abs()).unwrap_or(0)
    };
    let denominator_bits = x.denominator_ref().significant_bits();
    let rx = rug::Float::with_val(
        u32::exact_from(prec + 128 + exponent_bits + denominator_bits),
        rug::Rational::exact_from(x),
    );
    rug_sin_cos_pi_prec_round(&rx, prec, rm)
}

// The basic and fast tiers of `sin`, `cos`, and `sin_cos`, callable at any precision regardless of
// `SINCOS_THRESHOLD`, for the threshold tuner. `x` must be finite, nonzero, and of exponent at
// least 0 (so that the small-input shortcuts, which precede the tiers, do not apply).

pub fn sin_basic_for_tuning(x: &Float, prec: u64, rm: RoundingMode) -> Float {
    let exp_x = i64::from(x.get_exponent().unwrap());
    assert!(exp_x >= 0);
    sin_basic(x, exp_x, -(exp_x << 1), prec, rm).0
}

pub fn sin_fast_for_tuning(x: &Float, prec: u64, rm: RoundingMode) -> Float {
    sin_cos_fast(x, prec, rm, true, false).0.unwrap().0
}

pub fn cos_basic_for_tuning(x: &Float, prec: u64, rm: RoundingMode) -> Float {
    let exp_x = i64::from(x.get_exponent().unwrap());
    assert!(exp_x >= 0);
    cos_basic(x, exp_x, prec, rm).0
}

pub fn cos_fast_for_tuning(x: &Float, prec: u64, rm: RoundingMode) -> Float {
    sin_cos_fast(x, prec, rm, false, true).1.unwrap().0
}

pub fn sin_cos_basic_for_tuning(x: &Float, prec: u64, rm: RoundingMode) -> (Float, Float) {
    let exp_x = i64::from(x.get_exponent().unwrap());
    assert!(exp_x >= 0);
    // the initial working precision of `sin_cos_prec_round_normal_ref` for such an x
    let m = prec + prec.ceiling_log_base_2() + 13;
    let (s, c, _, _) = sin_cos_basic(x, exp_x, m, prec, rm);
    (s, c)
}

pub fn sin_cos_fast_for_tuning(x: &Float, prec: u64, rm: RoundingMode) -> (Float, Float) {
    let (s, c) = sin_cos_fast(x, prec, rm, true, true);
    (s.unwrap().0, c.unwrap().0)
}
