// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use num::BigUint;

pub fn num_partial_eq_unsigned<T>(x: &BigUint, u: T) -> bool
where
    BigUint: From<T>,
{
    *x == BigUint::from(u)
}
