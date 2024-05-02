// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::{BitIterable, CountOnes};

pub fn natural_count_ones_alt_1(n: &Natural) -> u64 {
    u64::exact_from(n.bits().filter(|&b| b).count())
}

pub fn natural_count_ones_alt_2(n: &Natural) -> u64 {
    n.limbs().map(CountOnes::count_ones).sum()
}
