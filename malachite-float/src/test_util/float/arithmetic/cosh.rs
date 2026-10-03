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

pub fn rug_cosh_prec_round(x: &rug::Float, prec: u64, rm: Round) -> (rug::Float, Ordering) {
    let mut c = rug::Float::with_val(u32::exact_from(prec), 0);
    let o = c.assign_round(x.cosh_ref(), rm);
    (c, o)
}

pub fn rug_cosh_prec(x: &rug::Float, prec: u64) -> (rug::Float, Ordering) {
    rug_cosh_prec_round(x, prec, Round::Nearest)
}

pub fn rug_cosh_round(x: &rug::Float, rm: Round) -> (rug::Float, Ordering) {
    rug_cosh_prec_round(x, rug_float_significant_bits(x), rm)
}

pub fn rug_cosh(x: &rug::Float) -> rug::Float {
    rug_cosh_prec_round(x, rug_float_significant_bits(x), Round::Nearest).0
}

// The input must be accurate to well below 2^-prec relative to cosh(x), which for a large x means
// carrying its exponent's worth of extra bits.
pub fn rug_cosh_rational_prec_round(x: &Rational, prec: u64, rm: Round) -> (rug::Float, Ordering) {
    let exponent_bits = if *x == 0u32 {
        0
    } else {
        x.floor_log_base_2_abs().unsigned_abs()
    };
    let rx = rug::Float::with_val(
        u32::exact_from(prec + 128 + exponent_bits),
        rug::Rational::exact_from(x),
    );
    let mut c = rug::Float::with_val(u32::exact_from(prec), 0);
    let o = c.assign_round(rx.cosh_ref(), rm);
    (c, o)
}

pub fn rug_cosh_rational_prec(x: &Rational, prec: u64) -> (rug::Float, Ordering) {
    rug_cosh_rational_prec_round(x, prec, Round::Nearest)
}
