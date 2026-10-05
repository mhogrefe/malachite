// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::test_util::common::rug_float_significant_bits;
use core::cmp::Ordering;
use malachite_base::num::basic::traits::One;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_q::Rational;
use rug::float::Round;
use rug::ops::AssignRound;

pub fn rug_acosh_prec_round(x: &rug::Float, prec: u64, rm: Round) -> (rug::Float, Ordering) {
    let mut c = rug::Float::with_val(u32::exact_from(prec), 0);
    let o = c.assign_round(x.acosh_ref(), rm);
    (c, o)
}

pub fn rug_acosh_prec(x: &rug::Float, prec: u64) -> (rug::Float, Ordering) {
    rug_acosh_prec_round(x, prec, Round::Nearest)
}

pub fn rug_acosh_round(x: &rug::Float, rm: Round) -> (rug::Float, Ordering) {
    rug_acosh_prec_round(x, rug_float_significant_bits(x), rm)
}

pub fn rug_acosh(x: &rug::Float) -> rug::Float {
    rug_acosh_prec_round(x, rug_float_significant_bits(x), Round::Nearest).0
}

// The input is rounded with as many extra bits as the magnitude of the exponent of x - 1, since
// acosh(1 + t) is about sqrt(2t), so that a relative error e in x moves the result by a relative
// error of about e/(2t); and with its denominator's bits, since a fraction close to a short dyadic
// can sit within about 2^-d (relatively) of a rounding boundary.
pub fn rug_acosh_rational_prec_round(x: &Rational, prec: u64, rm: Round) -> (rug::Float, Ordering) {
    let t = x - Rational::ONE;
    let exponent_bits = if t <= 0u32 {
        0
    } else {
        t.floor_log_base_2_abs().unsigned_abs()
    };
    let denominator_bits = x.denominator_ref().significant_bits();
    let rx = rug::Float::with_val(
        u32::exact_from(prec + 128 + exponent_bits + denominator_bits),
        rug::Rational::exact_from(x),
    );
    let mut c = rug::Float::with_val(u32::exact_from(prec), 0);
    let o = c.assign_round(rx.acosh_ref(), rm);
    (c, o)
}

pub fn rug_acosh_rational_prec(x: &Rational, prec: u64) -> (rug::Float, Ordering) {
    rug_acosh_rational_prec_round(x, prec, Round::Nearest)
}
