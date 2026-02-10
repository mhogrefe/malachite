// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Float;
use crate::InnerFloat::{Finite, Infinity, NaN, Zero};
use crate::arithmetic::ln::ln_prec_round_normal_extended;
use crate::basic::extended::ExtendedFloat;
use crate::test_util::common::rug_float_significant_bits;
use core::cmp::Ordering::{self, *};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::rounding_modes::RoundingMode;
use rug::float::Round;
use rug::ops::AssignRound;

pub fn rug_ln_prec_round(x: &rug::Float, prec: u64, rm: Round) -> (rug::Float, Ordering) {
    let mut ln = rug::Float::with_val(u32::exact_from(prec), 0);
    let o = ln.assign_round(x.ln_ref(), rm);
    (ln, o)
}

pub fn rug_ln_prec(x: &rug::Float, prec: u64) -> (rug::Float, Ordering) {
    rug_ln_prec_round(x, prec, Round::Nearest)
}

pub fn rug_ln_round(x: &rug::Float, rm: Round) -> (rug::Float, Ordering) {
    rug_ln_prec_round(x, rug_float_significant_bits(x), rm)
}

pub fn rug_ln(x: &rug::Float) -> rug::Float {
    rug_ln_prec_round(x, rug_float_significant_bits(x), Round::Nearest).0
}

pub fn ln_prec_round_extended(x: Float, prec: u64, rm: RoundingMode) -> (Float, Ordering) {
    assert_ne!(prec, 0);
    match x {
        Float(NaN | Infinity { sign: false } | Finite { sign: false, .. }) => (float_nan!(), Equal),
        float_either_zero!() => (float_negative_infinity!(), Equal),
        float_infinity!() => (float_infinity!(), Equal),
        _ => ln_prec_round_normal_extended(ExtendedFloat::from(x), prec, rm),
    }
}
