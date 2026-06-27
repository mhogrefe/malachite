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

pub fn rug_exp_x_minus_1_prec_round(
    x: &rug::Float,
    prec: u64,
    rm: Round,
) -> (rug::Float, Ordering) {
    let mut exp_x_minus_1 = rug::Float::with_val(u32::exact_from(prec), 0);
    let o = exp_x_minus_1.assign_round(x.exp_m1_ref(), rm);
    (exp_x_minus_1, o)
}

// Computes exp(x) - 1 for a Rational x, rounded to `prec` with mode `rm`. The Rational is first
// converted to a rug `Float`. Since expm1(x) ~ x for small x, the result tracks x's magnitude, so a
// large-denominator x with |x| just above a power of 2 rounds on a knife edge: the conversion must
// resolve x exactly relative to that boundary, which takes about as many bits as x's numerator and
// denominator. A fixed `prec + 128` guard mis-rounds such inputs, so scale the guard with x's bit
// length. (This is still not valid for |x| < 2^MIN_EXPONENT, where the rug conversion underflows to
// 0.)
pub fn rug_exp_x_minus_1_rational_prec_round(
    x: &Rational,
    prec: u64,
    rm: Round,
) -> (rug::Float, Ordering) {
    let guard =
        prec + 128 + x.numerator_ref().significant_bits() + x.denominator_ref().significant_bits();
    let rx = rug::Float::with_val(u32::exact_from(guard), rug::Rational::exact_from(x));
    let mut e = rug::Float::with_val(u32::exact_from(prec), 0);
    let o = e.assign_round(rx.exp_m1_ref(), rm);
    (e, o)
}

pub fn rug_exp_x_minus_1_rational_prec(x: &Rational, prec: u64) -> (rug::Float, Ordering) {
    rug_exp_x_minus_1_rational_prec_round(x, prec, Round::Nearest)
}

pub fn rug_exp_x_minus_1_prec(x: &rug::Float, prec: u64) -> (rug::Float, Ordering) {
    rug_exp_x_minus_1_prec_round(x, prec, Round::Nearest)
}

pub fn rug_exp_x_minus_1_round(x: &rug::Float, rm: Round) -> (rug::Float, Ordering) {
    rug_exp_x_minus_1_prec_round(x, rug_float_significant_bits(x), rm)
}

pub fn rug_exp_x_minus_1(x: &rug::Float) -> rug::Float {
    rug_exp_x_minus_1_prec_round(x, rug_float_significant_bits(x), Round::Nearest).0
}
