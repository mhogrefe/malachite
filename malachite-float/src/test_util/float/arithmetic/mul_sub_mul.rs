// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Float;
use crate::test_util::float::arithmetic::mul_add_mul::{
    mul_add_mul_prec_round_naive_helper, mul_add_mul_rational_prec_round_naive_helper,
};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::rounding_modes::RoundingMode;
use malachite_q::Rational;
use rug::float::Round;
use rug::ops::AssignRound;
use std::cmp::Ordering;

#[inline]
pub fn mul_sub_mul_rational_prec_round_naive(
    x: &Float,
    y: &Float,
    z: &Float,
    w: &Rational,
    prec: u64,
    rm: RoundingMode,
) -> (Float, Ordering) {
    mul_add_mul_rational_prec_round_naive_helper(x, y, z, w, true, prec, rm)
}

#[inline]
pub fn mul_sub_mul_prec_round_naive(
    a: &Float,
    b: &Float,
    c: &Float,
    d: &Float,
    prec: u64,
    rm: RoundingMode,
) -> (Float, Ordering) {
    mul_add_mul_prec_round_naive_helper(a, b, c, d, true, prec, rm)
}

pub fn rug_mul_sub_mul_prec_round(
    a: &rug::Float,
    b: &rug::Float,
    c: &rug::Float,
    d: &rug::Float,
    prec: u64,
    rm: Round,
) -> (rug::Float, Ordering) {
    let mut diff = rug::Float::with_val(u32::exact_from(prec), 0);
    let o = diff.assign_round(a.mul_sub_mul_ref(b, c, d), rm);
    (diff, o)
}
