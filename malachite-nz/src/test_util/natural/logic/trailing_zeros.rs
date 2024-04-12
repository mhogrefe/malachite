// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use malachite_base::num::conversion::traits::WrappingFrom;
use malachite_base::num::logic::traits::BitIterable;

pub fn natural_trailing_zeros_alt(n: &Natural) -> Option<u64> {
    if *n == 0 {
        None
    } else {
        Some(u64::wrapping_from(n.bits().take_while(|&b| !b).count()))
    }
}
