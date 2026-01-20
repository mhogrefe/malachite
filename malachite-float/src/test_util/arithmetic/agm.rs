// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Float;
use crate::InnerFloat::{Infinity, NaN, Zero};
use crate::basic::extended::{ExtendedFloat, agm_prec_round_normal_ref_ref_extended};
use crate::test_util::common::rug_float_significant_bits;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::rounding_modes::RoundingMode;
use rug::float::Round;
use rug::ops::AssignRound;
use std::cmp::Ordering::{self, *};
use std::cmp::max;

pub fn rug_agm_prec_round(
    x: &rug::Float,
    y: &rug::Float,
    prec: u64,
    rm: Round,
) -> (rug::Float, Ordering) {
    let mut agm = rug::Float::with_val(u32::exact_from(prec), 0);
    let o = agm.assign_round(x.agm_ref(y), rm);
    (agm, o)
}

#[inline]
pub fn rug_agm_round(x: &rug::Float, y: &rug::Float, rm: Round) -> (rug::Float, Ordering) {
    rug_agm_prec_round(
        x,
        y,
        max(rug_float_significant_bits(x), rug_float_significant_bits(y)),
        rm,
    )
}

#[inline]
pub fn rug_agm_prec(x: &rug::Float, y: &rug::Float, prec: u64) -> (rug::Float, Ordering) {
    rug_agm_prec_round(x, y, prec, Round::Nearest)
}

pub fn rug_agm(x: &rug::Float, y: &rug::Float) -> rug::Float {
    rug_agm_prec_round(
        x,
        y,
        max(rug_float_significant_bits(x), rug_float_significant_bits(y)),
        Round::Nearest,
    )
    .0
}

pub fn agm_prec_round_extended(
    x: Float,
    y: Float,
    prec: u64,
    rm: RoundingMode,
) -> (Float, Ordering) {
    assert_ne!(prec, 0);
    match (&x, &y) {
        (float_nan!(), _) | (_, float_nan!()) => (float_nan!(), Equal),
        (float_infinity!(), x) | (x, float_infinity!()) if *x > 0.0 => (float_infinity!(), Equal),
        (float_either_infinity!(), _) | (_, float_either_infinity!()) => (float_nan!(), Equal),
        (float_either_zero!(), _) | (_, float_either_zero!()) => (float_zero!(), Equal),
        _ => {
            let (x, o) = agm_prec_round_normal_ref_ref_extended(
                &ExtendedFloat::from(x),
                &ExtendedFloat::from(y),
                prec,
                rm,
            );
            x.into_float_helper(prec, rm, o)
        }
    }
}
