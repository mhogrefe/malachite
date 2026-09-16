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
use malachite_base::num::logic::traits::SignificantBits;
use malachite_q::Rational;
use rug::float::Round;
use rug::ops::AssignRound;

pub fn rug_asin_prec_round(x: &rug::Float, prec: u64, rm: Round) -> (rug::Float, Ordering) {
    let mut a = rug::Float::with_val(u32::exact_from(prec), 0);
    let o = a.assign_round(x.asin_ref(), rm);
    (a, o)
}

pub fn rug_asin_prec(x: &rug::Float, prec: u64) -> (rug::Float, Ordering) {
    rug_asin_prec_round(x, prec, Round::Nearest)
}

pub fn rug_asin(x: &rug::Float) -> rug::Float {
    rug_asin_prec_round(x, rug_float_significant_bits(x), Round::Nearest).0
}

// As for the other `Rational` oracles, the input carries its denominator's worth of extra bits, and
// the exponent term matters because the arcsine of a short dyadic sits close to the dyadic itself.
pub fn rug_asin_rational_prec_round(x: &Rational, prec: u64, rm: Round) -> (rug::Float, Ordering) {
    let exponent_bits = if *x == 0u32 {
        0
    } else {
        x.floor_log_base_2_abs().unsigned_abs()
    };
    let denominator_bits = x.denominator_ref().significant_bits();
    let rx = rug::Float::with_val(
        u32::exact_from(prec + 128 + (exponent_bits << 1) + denominator_bits),
        rug::Rational::exact_from(x),
    );
    rug_asin_prec_round(&rx, prec, rm)
}

pub fn rug_asin_rational_prec(x: &Rational, prec: u64) -> (rug::Float, Ordering) {
    rug_asin_rational_prec_round(x, prec, Round::Nearest)
}
