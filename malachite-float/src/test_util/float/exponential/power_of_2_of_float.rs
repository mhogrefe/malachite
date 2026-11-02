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
use malachite_q::Rational;
use rug::float::Round;
use rug::ops::AssignRound;

pub fn rug_power_of_2_of_float_prec_round(
    x: &rug::Float,
    prec: u64,
    rm: Round,
) -> (rug::Float, Ordering) {
    let mut e = rug::Float::with_val(u32::exact_from(prec), 0);
    let o = e.assign_round(x.exp2_ref(), rm);
    (e, o)
}

pub fn rug_power_of_2_of_float_prec(x: &rug::Float, prec: u64) -> (rug::Float, Ordering) {
    rug_power_of_2_of_float_prec_round(x, prec, Round::Nearest)
}

pub fn rug_power_of_2_of_float_round(x: &rug::Float, rm: Round) -> (rug::Float, Ordering) {
    rug_power_of_2_of_float_prec_round(x, rug_float_significant_bits(x), rm)
}

pub fn rug_power_of_2_of_float(x: &rug::Float) -> rug::Float {
    rug_power_of_2_of_float_prec_round(x, rug_float_significant_bits(x), Round::Nearest).0
}

// Approximates 2^x for a Rational x by converting x to a rug::Float with `prec + 128` bits and
// raising 2 to it. That is enough extra precision that the result rounds the same as the exact 2^x
// for all property test inputs (it would only differ if 2^x were within ~2^-98 of a rounding
// boundary).
pub fn rug_power_of_2_rational_prec_round(
    x: &Rational,
    prec: u64,
    rm: Round,
) -> (rug::Float, Ordering) {
    let rx = rug::Float::with_val(u32::exact_from(prec + 128), rug::Rational::exact_from(x));
    let mut e = rug::Float::with_val(u32::exact_from(prec), 0);
    let o = e.assign_round(rx.exp2_ref(), rm);
    (e, o)
}

pub fn rug_power_of_2_rational_prec(x: &Rational, prec: u64) -> (rug::Float, Ordering) {
    rug_power_of_2_rational_prec_round(x, prec, Round::Nearest)
}
