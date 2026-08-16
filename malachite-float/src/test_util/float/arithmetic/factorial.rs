// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::conversion::traits::ExactFrom;
use rug::float::Round;
use rug::ops::AssignRound;
use std::cmp::Ordering;

pub fn rug_factorial_prec_round(n: u32, prec: u64, rm: Round) -> (rug::Float, Ordering) {
    let mut f = rug::Float::with_val(u32::exact_from(prec), 0);
    let o = f.assign_round(rug::Float::factorial(n), rm);
    (f, o)
}

#[inline]
pub fn rug_factorial_prec(n: u32, prec: u64) -> (rug::Float, Ordering) {
    rug_factorial_prec_round(n, prec, Round::Nearest)
}
