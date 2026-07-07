// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::test_util::common::rug_float_significant_bits;
use malachite_base::num::conversion::traits::ExactFrom;
use rug::float::Round;
use rug::ops::AssignRound;
use rug::ops::Pow;
use std::cmp::Ordering;
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
