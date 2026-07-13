// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::conversion::traits::ExactFrom;
use rug;
use rug::float::Round;
use rug::ops::AssignRound;
use std::cmp::Ordering;

pub fn rug_cbrt_2_prec_round(prec: u64, rm: Round) -> (rug::Float, Ordering) {
    let mut cbrt = rug::Float::with_val(u32::exact_from(prec), 0);
    let o = cbrt.assign_round(rug::Float::with_val(1, 2).cbrt_ref(), rm);
    (cbrt, o)
}
