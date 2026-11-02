// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::test_util::common::rug_float_significant_bits;
use core::cmp::{Ordering, max};
use malachite_base::num::conversion::traits::ExactFrom;
use rug::float::Round;
use rug::ops::AssignRound;

pub fn rug_atan2_prec_round(
    y: &rug::Float,
    x: &rug::Float,
    prec: u64,
    rm: Round,
) -> (rug::Float, Ordering) {
    let mut a = rug::Float::with_val(u32::exact_from(prec), 0);
    let o = a.assign_round(y.atan2_ref(x), rm);
    (a, o)
}

pub fn rug_atan2_prec(y: &rug::Float, x: &rug::Float, prec: u64) -> (rug::Float, Ordering) {
    rug_atan2_prec_round(y, x, prec, Round::Nearest)
}

pub fn rug_atan2(y: &rug::Float, x: &rug::Float) -> rug::Float {
    rug_atan2_prec_round(
        y,
        x,
        max(rug_float_significant_bits(y), rug_float_significant_bits(x)),
        Round::Nearest,
    )
    .0
}

pub fn rug_atan2_with_period_prec_round(
    y: &rug::Float,
    x: &rug::Float,
    u: u64,
    prec: u64,
    rm: Round,
) -> (rug::Float, Ordering) {
    let mut a = rug::Float::with_val(u32::exact_from(prec), 0);
    let o = a.assign_round(y.atan2_u_ref(x, u32::exact_from(u)), rm);
    (a, o)
}

pub fn rug_atan2_with_period_prec(
    y: &rug::Float,
    x: &rug::Float,
    u: u64,
    prec: u64,
) -> (rug::Float, Ordering) {
    rug_atan2_with_period_prec_round(y, x, u, prec, Round::Nearest)
}
