// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::cmp::Ordering;
use rug::float::Round;

pub fn rug_square_round(mut x: rug::Float, rm: Round) -> (rug::Float, Ordering) {
    let o = x.square_round(rm);
    (x, o)
}

pub fn rug_square(x: rug::Float) -> rug::Float {
    x.square()
}
