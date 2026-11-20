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

// The value and the rounding direction are those of `mpfr_lgamma`; the `bool` is whether the sign
// that it reports is positive.
pub fn rug_ln_abs_gamma_prec_round(
    x: &rug::Float,
    prec: u64,
    rm: Round,
) -> (rug::Float, bool, Ordering) {
    let mut e = rug::Float::with_val(u32::exact_from(prec), 0);
    let mut sign = Ordering::Equal;
    let o = (&mut e, &mut sign).assign_round(x.ln_abs_gamma_ref(), rm);
    (e, sign != Ordering::Less, o)
}

pub fn rug_ln_abs_gamma_prec(x: &rug::Float, prec: u64) -> (rug::Float, bool, Ordering) {
    rug_ln_abs_gamma_prec_round(x, prec, Round::Nearest)
}

pub fn rug_ln_abs_gamma_round(x: &rug::Float, rm: Round) -> (rug::Float, bool, Ordering) {
    rug_ln_abs_gamma_prec_round(x, rug_float_significant_bits(x), rm)
}

pub fn rug_ln_abs_gamma(x: &rug::Float) -> (rug::Float, bool) {
    let (y, sign, _) =
        rug_ln_abs_gamma_prec_round(x, rug_float_significant_bits(x), Round::Nearest);
    (y, sign)
}

// Computes ln |Gamma(x)| and the sign of Gamma(x) for a Rational x, rounded to `prec` with mode
// `rm`. The Rational is first converted to a rug `Float` with `prec + 256` bits; see
// `rug_ln_gamma_rational_prec_round`. For a negative x, the result rounds the same as the exact one
// when x is also not too close to an integer.
pub fn rug_ln_abs_gamma_rational_prec_round(
    x: &Rational,
    prec: u64,
    rm: Round,
) -> (rug::Float, bool, Ordering) {
    let rx = rug::Float::with_val(u32::exact_from(prec + 256), rug::Rational::exact_from(x));
    rug_ln_abs_gamma_prec_round(&rx, prec, rm)
}
