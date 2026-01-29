// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use malachite_base::num::arithmetic::traits::{Abs, Ceiling, Floor};

pub fn mod_op_naive(x: Rational, y: Rational) -> Rational {
    let q = &x / &y;
    x - y * Rational::from(q.floor())
}

pub fn rem_naive(x: Rational, y: Rational) -> Rational {
    let q = &x / &y;
    let sign = (x >= 0u32) == (y >= 0u32);
    let z = y * Rational::from(q.abs().floor());
    if sign { x - z } else { x + z }
}

pub fn ceiling_mod_naive(x: Rational, y: Rational) -> Rational {
    let q = &x / &y;
    x - y * Rational::from(q.ceiling())
}
