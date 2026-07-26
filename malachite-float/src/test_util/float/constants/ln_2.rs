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

pub fn rug_ln_2_prec_round(prec: u64, rm: Round) -> (rug::Float, Ordering) {
    let mut ln_2 = rug::Float::with_val(u32::exact_from(prec), 0);
    let o = ln_2.assign_round(Constant::Log2, rm);
    (ln_2, o)
}
