// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::conversion::traits::ExactFrom;
use rug;
use rug::float::{Constant, Round};
use rug::ops::AssignRound;
use std::cmp::Ordering;

pub fn rug_catalans_constant_prec_round(prec: u64, rm: Round) -> (rug::Float, Ordering) {
    let mut g = rug::Float::with_val(u32::exact_from(prec), 0);
    let o = g.assign_round(Constant::Catalan, rm);
    (g, o)
}
