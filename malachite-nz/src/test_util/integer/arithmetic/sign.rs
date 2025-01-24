// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use num::bigint::Sign;
use num::BigInt;
use std::cmp::Ordering::{self, *};

pub fn num_sign(x: &BigInt) -> Ordering {
    match x.sign() {
        Sign::NoSign => Equal,
        Sign::Plus => Greater,
        Sign::Minus => Less,
    }
}
