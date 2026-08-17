// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Float;
use crate::test_util::common::rug_float_significant_bits;
use crate::test_util::float::arithmetic::add_mul::{
    add_mul_prec_round_naive_helper, add_mul_rational_prec_round_naive_helper,
};
use malachite_base::max;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::rounding_modes::RoundingMode;
use malachite_q::Rational;
use rug::float::Round;
use rug::ops::AssignRound;
use std::cmp::Ordering;

#[inline]
pub fn sub_mul_prec_round_naive(
    x: &Float,
    y: &Float,
    z: &Float,
    prec: u64,
    rm: RoundingMode,
) -> (Float, Ordering) {
    add_mul_prec_round_naive_helper(x, y, z, true, prec, rm)
}

#[inline]
pub fn sub_mul_rational_prec_round_naive(
    x: &Float,
    y: &Float,
    z: &Rational,
    prec: u64,
    rm: RoundingMode,
) -> (Float, Ordering) {
    add_mul_rational_prec_round_naive_helper(x, y, z, true, prec, rm)
}

// x - y * z is x + (-y) * z; negating the multiplicand rather than the whole fms preserves the
// sign-of-zero rules, which negating an fms result does not.
pub fn rug_sub_mul_prec_round(
    x: &rug::Float,
    y: &rug::Float,
    z: &rug::Float,
    prec: u64,
    rm: Round,
) -> (rug::Float, Ordering) {
    let mut diff = rug::Float::with_val(u32::exact_from(prec), 0);
    let neg_y = -y.clone();
    let o = diff.assign_round(neg_y.mul_add_ref(z, x), rm);
    (diff, o)
}

#[inline]
pub fn rug_sub_mul_prec(
    x: &rug::Float,
    y: &rug::Float,
    z: &rug::Float,
    prec: u64,
) -> (rug::Float, Ordering) {
    rug_sub_mul_prec_round(x, y, z, prec, Round::Nearest)
}

#[inline]
pub fn rug_sub_mul_round(
    x: &rug::Float,
    y: &rug::Float,
    z: &rug::Float,
    rm: Round,
) -> (rug::Float, Ordering) {
    rug_sub_mul_prec_round(
        x,
        y,
        z,
        max!(
            rug_float_significant_bits(x),
            rug_float_significant_bits(y),
            rug_float_significant_bits(z)
        ),
        rm,
    )
}

pub fn rug_sub_mul(x: &rug::Float, y: &rug::Float, z: &rug::Float) -> rug::Float {
    rug_sub_mul_round(x, y, z, Round::Nearest).0
}
