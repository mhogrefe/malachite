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

pub fn rug_sin_cos_prec_round(
    x: &rug::Float,
    prec: u64,
    rm: Round,
) -> (rug::Float, rug::Float, Ordering, Ordering) {
    let mut s = rug::Float::with_val(u32::exact_from(prec), 0);
    let mut c = rug::Float::with_val(u32::exact_from(prec), 0);
    let (o_s, o_c) = (&mut s, &mut c).assign_round(x.sin_cos_ref(), rm);
    (s, c, o_s, o_c)
}

pub fn rug_sin_cos_prec(x: &rug::Float, prec: u64) -> (rug::Float, rug::Float, Ordering, Ordering) {
    rug_sin_cos_prec_round(x, prec, Round::Nearest)
}

pub fn rug_sin_cos_round(
    x: &rug::Float,
    rm: Round,
) -> (rug::Float, rug::Float, Ordering, Ordering) {
    rug_sin_cos_prec_round(x, rug_float_significant_bits(x), rm)
}

pub fn rug_sin_cos(x: &rug::Float) -> (rug::Float, rug::Float) {
    let (s, c, _, _) = rug_sin_cos_prec_round(x, rug_float_significant_bits(x), Round::Nearest);
    (s, c)
}

// As for the sine oracle, the input carries its denominator's worth of extra bits.
pub fn rug_sin_cos_rational_prec_round(
    x: &Rational,
    prec: u64,
    rm: Round,
) -> (rug::Float, rug::Float, Ordering, Ordering) {
    let exponent_bits = if *x == 0u32 {
        0
    } else {
        u64::try_from(x.floor_log_base_2_abs()).unwrap_or(0)
    };
    let denominator_bits = x.denominator_ref().significant_bits();
    let rx = rug::Float::with_val(
        u32::exact_from(prec + 128 + exponent_bits + denominator_bits),
        rug::Rational::exact_from(x),
    );
    let mut s = rug::Float::with_val(u32::exact_from(prec), 0);
    let mut c = rug::Float::with_val(u32::exact_from(prec), 0);
    let (o_s, o_c) = (&mut s, &mut c).assign_round(rx.sin_cos_ref(), rm);
    (s, c, o_s, o_c)
}

pub fn rug_sin_cos_rational_prec(
    x: &Rational,
    prec: u64,
) -> (rug::Float, rug::Float, Ordering, Ordering) {
    rug_sin_cos_rational_prec_round(x, prec, Round::Nearest)
}
