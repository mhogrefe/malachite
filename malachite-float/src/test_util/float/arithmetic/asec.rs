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
use rug::float::Round;
use rug::ops::AssignRound;

// MPFR has no arcsecant, so the oracle is its arccosine of the reciprocal. The reciprocal is taken
// with the input's precision to spare, which absorbs the arccosine's amplification near |x| = 1: an
// error there is magnified by about the square root of the input's precision in bits.
pub fn rug_asec_prec_round(x: &rug::Float, prec: u64, rm: Round) -> (rug::Float, Ordering) {
    let mut t = rug::Float::with_val(
        u32::exact_from(prec + 128 + rug_float_significant_bits(x)),
        0,
    );
    t.assign_round(x.recip_ref(), Round::Nearest);
    let mut a = rug::Float::with_val(u32::exact_from(prec), 0);
    let o = a.assign_round(t.acos_ref(), rm);
    (a, o)
}

pub fn rug_asec_prec(x: &rug::Float, prec: u64) -> (rug::Float, Ordering) {
    rug_asec_prec_round(x, prec, Round::Nearest)
}

pub fn rug_asec(x: &rug::Float) -> rug::Float {
    rug_asec_prec_round(x, rug_float_significant_bits(x), Round::Nearest).0
}
