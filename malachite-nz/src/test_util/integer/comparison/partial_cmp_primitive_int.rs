// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use num::BigInt;
use std::cmp::Ordering;

pub fn num_partial_cmp_primitive<T>(x: &BigInt, u: T) -> Option<Ordering>
where
    BigInt: From<T>,
{
    x.partial_cmp(&BigInt::from(u))
}
