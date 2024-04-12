// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use num::BigUint;
use std::cmp::Ordering;

pub fn num_partial_cmp_unsigned<T>(x: &BigUint, u: T) -> Option<Ordering>
where
    BigUint: From<T>,
{
    x.partial_cmp(&BigUint::from(u))
}
