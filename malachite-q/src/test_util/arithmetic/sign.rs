// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use num::bigint::Sign;
use num::BigRational;
use std::cmp::Ordering::{self, *};

pub fn num_sign(x: &BigRational) -> Ordering {
    match x.numer().sign() {
        Sign::NoSign => Equal,
        Sign::Plus => Greater,
        Sign::Minus => Less,
    }
}
