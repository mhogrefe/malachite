// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use core::cmp::Ordering;
use malachite_base::num::arithmetic::traits::ShrRound;
use malachite_base::rounding_modes::RoundingMode;

// The exact value is represented by `x - sign(o) / 4`, which lies strictly between `x` and its
// neighbor on the side `o` indicates, and is close enough to `x` that no rounding boundary falls
// between them. Scaled by 4 it is the integer `4x - sign(o)`, so the result is an ordinary
// rounded shift of that integer by two more bits.
pub fn integer_shr_round_with_ordering_naive(
    x: &Integer,
    bits: u64,
    rm: RoundingMode,
    o: Ordering,
) -> (Integer, Ordering) {
    ((x << 2u64) - Integer::from(o as i8)).shr_round(bits + 2, rm)
}
