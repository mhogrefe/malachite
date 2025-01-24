// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Float;
use malachite_base::num::factorization::primes::prime_indicator_sequence;
use malachite_base::rounding_modes::RoundingMode;
use std::cmp::Ordering;

pub fn prime_constant_prec_round_naive(prec: u64, rm: RoundingMode) -> (Float, Ordering) {
    Float::non_dyadic_from_bits_prec_round(prime_indicator_sequence(), prec, rm)
}
