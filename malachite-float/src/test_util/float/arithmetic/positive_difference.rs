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
use std::cmp::{Ordering, max};

pub fn rug_positive_difference_prec_round(
    x: &rug::Float,
    y: &rug::Float,
    prec: u64,
    rm: Round,
) -> (rug::Float, Ordering) {
    let mut diff = rug::Float::with_val(u32::exact_from(prec), 0);
    let o = diff.assign_round(x.positive_diff_ref(y), rm);
    (diff, o)
}

#[inline]
pub fn rug_positive_difference_round(
    x: &rug::Float,
    y: &rug::Float,
    rm: Round,
) -> (rug::Float, Ordering) {
    rug_positive_difference_prec_round(
        x,
        y,
        max(rug_float_significant_bits(x), rug_float_significant_bits(y)),
        rm,
    )
}

#[inline]
pub fn rug_positive_difference_prec(
    x: &rug::Float,
    y: &rug::Float,
    prec: u64,
) -> (rug::Float, Ordering) {
    rug_positive_difference_prec_round(x, y, prec, Round::Nearest)
}

pub fn rug_positive_difference(x: &rug::Float, y: &rug::Float) -> rug::Float {
    rug_positive_difference_prec_round(
        x,
        y,
        max(rug_float_significant_bits(x), rug_float_significant_bits(y)),
        Round::Nearest,
    )
    .0
}
