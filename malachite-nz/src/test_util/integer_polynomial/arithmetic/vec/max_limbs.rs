// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;

// The largest number of limbs of the absolute value of any element of `xs`, found by converting
// each absolute value to a `Vec` of limbs.
pub fn vec_max_limbs_naive(xs: &[Integer]) -> u64 {
    xs.iter()
        .map(|x| u64::try_from(x.unsigned_abs_ref().to_limbs_asc().len()).unwrap())
        .max()
        .unwrap_or(0)
}
