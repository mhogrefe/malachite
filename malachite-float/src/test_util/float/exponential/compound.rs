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
use std::cmp::Ordering;

pub fn rug_compound_prec_round(
    x: &rug::Float,
    n: i32,
    prec: u64,
    rm: Round,
) -> (rug::Float, Ordering) {
    let mut compound = rug::Float::with_val(u32::exact_from(prec), 0);
    let o = compound.assign_round(x.compound_i_ref(n), rm);
    (compound, o)
}

#[inline]
pub fn rug_compound_round(x: &rug::Float, n: i32, rm: Round) -> (rug::Float, Ordering) {
    rug_compound_prec_round(x, n, rug_float_significant_bits(x), rm)
}

#[inline]
pub fn rug_compound_prec(x: &rug::Float, n: i32, prec: u64) -> (rug::Float, Ordering) {
    rug_compound_prec_round(x, n, prec, Round::Nearest)
}

pub fn rug_compound(x: &rug::Float, n: i32) -> rug::Float {
    rug_compound_prec_round(x, n, rug_float_significant_bits(x), Round::Nearest).0
}
