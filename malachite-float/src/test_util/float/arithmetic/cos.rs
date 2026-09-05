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
use malachite_q::Rational;
use rug::float::Round;
use rug::ops::AssignRound;

pub fn rug_cos_prec_round(x: &rug::Float, prec: u64, rm: Round) -> (rug::Float, Ordering) {
    let mut e = rug::Float::with_val(u32::exact_from(prec), 0);
    let o = e.assign_round(x.cos_ref(), rm);
    (e, o)
}

pub fn rug_cos_prec(x: &rug::Float, prec: u64) -> (rug::Float, Ordering) {
    rug_cos_prec_round(x, prec, Round::Nearest)
}

// Computes cos(x) for a Rational x, rounded to `prec` with mode `rm`. The Rational is first
// converted to a rug `Float` with `prec + 128` bits; since the finite cos range has |x| < 2^30,
// that is enough extra precision that the result rounds the same as the exact cos(x) for all
// property test inputs (it would only differ if cos(x) were within ~2^-98 of a rounding boundary).
pub fn rug_cos_round(x: &rug::Float, rm: Round) -> (rug::Float, Ordering) {
    rug_cos_prec_round(x, rug_float_significant_bits(x), rm)
}

pub fn rug_cos(x: &rug::Float) -> rug::Float {
    rug_cos_prec_round(x, rug_float_significant_bits(x), Round::Nearest).0
}

pub fn rug_cos_rational_prec_round(x: &Rational, prec: u64, rm: Round) -> (rug::Float, Ordering) {
    // The input must be accurate to well below 2^-prec in absolute terms, which for a large x means
    // carrying its exponent's worth of extra bits.
    let exponent_bits = if *x == 0u32 {
        0
    } else {
        u64::try_from(x.floor_log_base_2_abs()).unwrap_or(0)
    };
    let rx = rug::Float::with_val(
        u32::exact_from(prec + 128 + exponent_bits),
        rug::Rational::exact_from(x),
    );
    let mut c = rug::Float::with_val(u32::exact_from(prec), 0);
    let o = c.assign_round(rx.cos_ref(), rm);
    (c, o)
}

pub fn rug_cos_rational_prec(x: &Rational, prec: u64) -> (rug::Float, Ordering) {
    rug_cos_rational_prec_round(x, prec, Round::Nearest)
}

// `u` must fit in a `u32`, the type rug takes.
pub fn rug_cos_with_period_prec_round(
    x: &rug::Float,
    u: u64,
    prec: u64,
    rm: Round,
) -> (rug::Float, Ordering) {
    let mut c = rug::Float::with_val(u32::exact_from(prec), 0);
    let o = c.assign_round(x.cos_u_ref(u32::exact_from(u)), rm);
    (c, o)
}

pub fn rug_cos_with_period_prec(x: &rug::Float, u: u64, prec: u64) -> (rug::Float, Ordering) {
    rug_cos_with_period_prec_round(x, u, prec, Round::Nearest)
}
