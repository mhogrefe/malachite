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

pub fn rug_gamma_prec_round(x: &rug::Float, prec: u64, rm: Round) -> (rug::Float, Ordering) {
    let mut e = rug::Float::with_val(u32::exact_from(prec), 0);
    let o = e.assign_round(x.gamma_ref(), rm);
    (e, o)
}

pub fn rug_gamma_prec(x: &rug::Float, prec: u64) -> (rug::Float, Ordering) {
    rug_gamma_prec_round(x, prec, Round::Nearest)
}

pub fn rug_gamma_round(x: &rug::Float, rm: Round) -> (rug::Float, Ordering) {
    rug_gamma_prec_round(x, rug_float_significant_bits(x), rm)
}

pub fn rug_gamma(x: &rug::Float) -> rug::Float {
    rug_gamma_prec_round(x, rug_float_significant_bits(x), Round::Nearest).0
}

// Computes Gamma(x) for a Rational x, rounded to `prec` with mode `rm`. The Rational is first
// converted to a rug `Float` with `prec + 256` bits. The conversion has a relative error of at most
// 2^-(prec + 256), which the gamma function magnifies by about |x psi(x)|: next to a pole at a
// nonpositive integer other than 0, by about |x| divided by the distance to the pole. So the result
// rounds the same as the exact Gamma(x) when x is not too large and not too close to a negative
// integer (it would only differ if Gamma(x) were very close to a rounding boundary).
pub fn rug_gamma_rational_prec_round(x: &Rational, prec: u64, rm: Round) -> (rug::Float, Ordering) {
    let rx = rug::Float::with_val(u32::exact_from(prec + 256), rug::Rational::exact_from(x));
    let mut e = rug::Float::with_val(u32::exact_from(prec), 0);
    let o = e.assign_round(rx.gamma_ref(), rm);
    (e, o)
}
