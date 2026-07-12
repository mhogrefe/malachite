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

pub fn rug_power_of_10_x_minus_1_prec_round(
    x: &rug::Float,
    prec: u64,
    rm: Round,
) -> (rug::Float, Ordering) {
    let mut power_of_10_x_minus_1 = rug::Float::with_val(u32::exact_from(prec), 0);
    let o = power_of_10_x_minus_1.assign_round(x.exp10_m1_ref(), rm);
    (power_of_10_x_minus_1, o)
}

pub fn rug_power_of_10_x_minus_1_prec(x: &rug::Float, prec: u64) -> (rug::Float, Ordering) {
    rug_power_of_10_x_minus_1_prec_round(x, prec, Round::Nearest)
}

pub fn rug_power_of_10_x_minus_1_round(x: &rug::Float, rm: Round) -> (rug::Float, Ordering) {
    rug_power_of_10_x_minus_1_prec_round(x, rug_float_significant_bits(x), rm)
}

pub fn rug_power_of_10_x_minus_1(x: &rug::Float) -> rug::Float {
    rug_power_of_10_x_minus_1_prec_round(x, rug_float_significant_bits(x), Round::Nearest).0
}

// Computes 10^x - 1 for a Rational x, rounded to `prec` with mode `rm`. The Rational is first
// converted to a rug `Float`. Since 10^x - 1 ~ x ln(2) for small x, the result tracks x's
// magnitude, so a large-denominator x rounds on a knife edge: the conversion must resolve x
// exactly, which takes about as many bits as x's numerator and denominator; the guard precision
// scales accordingly. (This is still not valid for |x| < 2^MIN_EXPONENT, where the rug conversion
// underflows to 0.)
pub fn rug_power_of_10_x_minus_1_rational_prec_round(
    x: &Rational,
    prec: u64,
    rm: Round,
) -> (rug::Float, Ordering) {
    let guard =
        prec + 128 + x.numerator_ref().significant_bits() + x.denominator_ref().significant_bits();
    let rx = rug::Float::with_val(u32::exact_from(guard), rug::Rational::exact_from(x));
    let mut p = rug::Float::with_val(u32::exact_from(prec), 0);
    let o = p.assign_round(rx.exp10_m1_ref(), rm);
    (p, o)
}

pub fn rug_power_of_10_x_minus_1_rational_prec(x: &Rational, prec: u64) -> (rug::Float, Ordering) {
    rug_power_of_10_x_minus_1_rational_prec_round(x, prec, Round::Nearest)
}
