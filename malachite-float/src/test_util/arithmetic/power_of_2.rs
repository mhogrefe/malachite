// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Float;
use malachite_base::num::arithmetic::traits::PowerOf2;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_q::Rational;
use std::cmp::Ordering;

pub fn power_of_2_prec_round_naive(pow: i64, prec: u64, rm: RoundingMode) -> (Float, Ordering) {
    Float::from_rational_prec_round(Rational::power_of_2(pow), prec, rm)
}

pub fn power_of_2_prec_naive(pow: i64, prec: u64) -> (Float, Ordering) {
    Float::from_rational_prec_round(Rational::power_of_2(pow), prec, Nearest)
}

pub fn power_of_2_u64_naive(pow: u64) -> Float {
    Float::from_rational_prec_round(Rational::power_of_2(pow), 1, Nearest).0
}

pub fn power_of_2_i64_naive(pow: i64) -> Float {
    Float::from_rational_prec_round(Rational::power_of_2(pow), 1, Nearest).0
}
