// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::natural::Natural;
use crate::test_util::integer::arithmetic::shr_round_with_ordering::*;
use core::cmp::Ordering;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::rounding_modes::RoundingMode::{self, *};

// For a `Natural`, `Down` and `Up` are `Floor` and `Ceiling`, even when the exact value lies just
// below zero, where the result would be negative and the function panics.
pub fn natural_shr_round_with_ordering_naive(
    x: &Natural,
    bits: u64,
    rm: RoundingMode,
    o: Ordering,
) -> (Natural, Ordering) {
    let rm = match rm {
        Down => Floor,
        Up => Ceiling,
        rm => rm,
    };
    let (q, o) = integer_shr_round_with_ordering_naive(&Integer::from(x), bits, rm, o);
    (Natural::exact_from(q), o)
}
