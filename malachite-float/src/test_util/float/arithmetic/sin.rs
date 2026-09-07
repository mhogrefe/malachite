// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::test_util::common::rug_float_significant_bits;
use core::cmp::Ordering;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_q::Rational;
use rug::float::Round;
use rug::ops::AssignRound;

pub fn rug_sin_prec_round(x: &rug::Float, prec: u64, rm: Round) -> (rug::Float, Ordering) {
    let mut s = rug::Float::with_val(u32::exact_from(prec), 0);
    let o = s.assign_round(x.sin_ref(), rm);
    (s, o)
}

pub fn rug_sin_prec(x: &rug::Float, prec: u64) -> (rug::Float, Ordering) {
    rug_sin_prec_round(x, prec, Round::Nearest)
}

pub fn rug_sin_round(x: &rug::Float, rm: Round) -> (rug::Float, Ordering) {
    rug_sin_prec_round(x, rug_float_significant_bits(x), rm)
}

pub fn rug_sin(x: &rug::Float) -> rug::Float {
    rug_sin_prec_round(x, rug_float_significant_bits(x), Round::Nearest).0
}

// Computes sin(x) for a Rational x, rounded to `prec` with mode `rm`. The Rational is first
// converted to a rug `Float` with `prec + 128` bits plus its exponent's and its denominator's worth
// of bits. Since sin(x) is close to x for small x, an input that is very close to a short dyadic
// (as a/b can be, to within about 1/b relative) must be carried precisely enough to stay on the
// right side of it, or the ternary value comes out wrong; the cosine oracle needs no such care.
pub fn rug_sin_rational_prec_round(x: &Rational, prec: u64, rm: Round) -> (rug::Float, Ordering) {
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
    let o = s.assign_round(rx.sin_ref(), rm);
    (s, o)
}

pub fn rug_sin_rational_prec(x: &Rational, prec: u64) -> (rug::Float, Ordering) {
    rug_sin_rational_prec_round(x, prec, Round::Nearest)
}

// `u` must fit in a `u32`, the type rug takes.
pub fn rug_sin_with_period_prec_round(
    x: &rug::Float,
    u: u64,
    prec: u64,
    rm: Round,
) -> (rug::Float, Ordering) {
    let mut s = rug::Float::with_val(u32::exact_from(prec), 0);
    let o = s.assign_round(x.sin_u_ref(u32::exact_from(u)), rm);
    (s, o)
}

pub fn rug_sin_with_period_prec(x: &rug::Float, u: u64, prec: u64) -> (rug::Float, Ordering) {
    rug_sin_with_period_prec_round(x, u, prec, Round::Nearest)
}

// As for `rug_sin_rational_prec_round`, the input carries its denominator's worth of extra bits,
// since the sine of a fraction of a turn close to a short dyadic depends on that closeness.
pub fn rug_sin_with_period_rational_prec_round(
    x: &Rational,
    u: u64,
    prec: u64,
    rm: Round,
) -> (rug::Float, Ordering) {
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
    let o = s.assign_round(rx.sin_u_ref(u32::exact_from(u)), rm);
    (s, o)
}

pub fn rug_sin_with_period_rational_prec(
    x: &Rational,
    u: u64,
    prec: u64,
) -> (rug::Float, Ordering) {
    rug_sin_with_period_rational_prec_round(x, u, prec, Round::Nearest)
}

pub fn rug_sin_pi_prec_round(x: &rug::Float, prec: u64, rm: Round) -> (rug::Float, Ordering) {
    let mut s = rug::Float::with_val(u32::exact_from(prec), 0);
    let o = s.assign_round(x.sin_pi_ref(), rm);
    (s, o)
}

pub fn rug_sin_pi_rational_prec_round(
    x: &Rational,
    prec: u64,
    rm: Round,
) -> (rug::Float, Ordering) {
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
    let o = s.assign_round(rx.sin_pi_ref(), rm);
    (s, o)
}
