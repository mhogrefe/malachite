// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Float;
use crate::arithmetic::reciprocal_sqrt::generic_reciprocal_sqrt_rational_ref;
use crate::test_util::common::rug_float_significant_bits;
use core::cmp::Ordering::{self, *};
use malachite_base::num::arithmetic::traits::{CheckedSqrt, Reciprocal};
use malachite_base::num::basic::traits::{Infinity, NaN};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::rounding_modes::RoundingMode;
use malachite_q::Rational;
use rug::float::Round;
use rug::ops::AssignRound;

pub fn rug_reciprocal_sqrt_prec_round(
    x: &rug::Float,
    prec: u64,
    rm: Round,
) -> (rug::Float, Ordering) {
    let mut sum = rug::Float::with_val(u32::exact_from(prec), 0);
    let o = sum.assign_round(x.recip_sqrt_ref(), rm);
    (sum, o)
}

pub fn rug_reciprocal_sqrt_prec(x: &rug::Float, prec: u64) -> (rug::Float, Ordering) {
    rug_reciprocal_sqrt_prec_round(x, prec, Round::Nearest)
}

pub fn rug_reciprocal_sqrt_round(x: &rug::Float, rm: Round) -> (rug::Float, Ordering) {
    rug_reciprocal_sqrt_prec_round(x, rug_float_significant_bits(x), rm)
}

pub fn rug_reciprocal_sqrt(x: &rug::Float) -> rug::Float {
    rug_reciprocal_sqrt_prec_round(x, rug_float_significant_bits(x), Round::Nearest).0
}

pub fn reciprocal_sqrt_rational_prec_round_generic(
    x: &Rational,
    prec: u64,
    rm: RoundingMode,
) -> (Float, Ordering) {
    if *x == 0u32 {
        return (Float::INFINITY, Equal);
    } else if *x < 0u32 {
        return (Float::NAN, Equal);
    }
    if let Some(sqrt) = x.checked_sqrt() {
        return Float::from_rational_prec_round(sqrt.reciprocal(), prec, rm);
    }
    generic_reciprocal_sqrt_rational_ref(x, prec, rm)
}
