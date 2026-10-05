// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::test_util::common::rug_float_significant_bits;
use core::cmp::Ordering;
use malachite_base::num::arithmetic::traits::Abs;
use malachite_base::num::basic::traits::One;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_q::Rational;
use rug::float::Round;
use rug::ops::AssignRound;

pub fn rug_atanh_prec_round(x: &rug::Float, prec: u64, rm: Round) -> (rug::Float, Ordering) {
    let mut c = rug::Float::with_val(u32::exact_from(prec), 0);
    let o = c.assign_round(x.atanh_ref(), rm);
    (c, o)
}

pub fn rug_atanh_prec(x: &rug::Float, prec: u64) -> (rug::Float, Ordering) {
    rug_atanh_prec_round(x, prec, Round::Nearest)
}

pub fn rug_atanh_round(x: &rug::Float, rm: Round) -> (rug::Float, Ordering) {
    rug_atanh_prec_round(x, rug_float_significant_bits(x), rm)
}

pub fn rug_atanh(x: &rug::Float) -> rug::Float {
    rug_atanh_prec_round(x, rug_float_significant_bits(x), Round::Nearest).0
}

// The input is rounded with extra bits for three reasons: twice its exponent's worth for a tiny x,
// since atanh x lies within about x^2 (relatively) of x; as many as the magnitude of the exponent
// of 1 - |x| for an x near +/-1, since a relative error e in x then moves the result by about
// e/(2(1 - |x|) atanh |x|); and its denominator's bits, since a fraction close to a short dyadic
// can sit within about 2^-d (relatively) of a rounding boundary.
pub fn rug_atanh_rational_prec_round(x: &Rational, prec: u64, rm: Round) -> (rug::Float, Ordering) {
    let x_abs = x.abs();
    let exponent_bits = if *x == 0u32 {
        0
    } else {
        x.floor_log_base_2_abs().unsigned_abs() << 1
    };
    let near_one_bits = if x_abs >= 1u32 {
        0
    } else {
        (Rational::ONE - &x_abs)
            .floor_log_base_2_abs()
            .unsigned_abs()
    };
    let denominator_bits = x.denominator_ref().significant_bits();
    let rx = rug::Float::with_val(
        u32::exact_from(prec + 128 + exponent_bits + near_one_bits + denominator_bits),
        rug::Rational::exact_from(x),
    );
    let mut c = rug::Float::with_val(u32::exact_from(prec), 0);
    let o = c.assign_round(rx.atanh_ref(), rm);
    (c, o)
}

pub fn rug_atanh_rational_prec(x: &Rational, prec: u64) -> (rug::Float, Ordering) {
    rug_atanh_rational_prec_round(x, prec, Round::Nearest)
}
