// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use alloc::vec::Vec;

// Multiplies every element of `xs` by `c`, with none of the special cases for 0 and ±1.
pub fn integers_mul_scalar_naive(xs: &[Integer], c: &Integer) -> Vec<Integer> {
    xs.iter().map(|x| x * c).collect()
}
