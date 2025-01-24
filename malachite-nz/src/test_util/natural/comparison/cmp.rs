// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use malachite_base::num::logic::traits::SignificantBits;
use std::cmp::Ordering::{self, *};

pub fn natural_cmp_normalized_naive(x: &Natural, y: &Natural) -> Ordering {
    let x_bits = x.significant_bits();
    let y_bits = y.significant_bits();
    match x_bits.cmp(&y_bits) {
        Equal => x.cmp(y),
        Less => (x << (y_bits - x_bits)).cmp(y),
        Greater => x.cmp(&(y << (x_bits - y_bits))),
    }
}
