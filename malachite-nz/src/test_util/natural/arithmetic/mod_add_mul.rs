// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;

// Computes the sum exactly and reduces it once.
pub fn mod_add_mul_naive(x: &Natural, y: &Natural, z: &Natural, m: &Natural) -> Natural {
    (x + y * z) % m
}
