// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use alloc::vec::Vec;
use core::cmp::max;
use malachite_base::num::basic::traits::Zero;

// Computes `xs + c * ys` element by element, treating each vector as zero past its end, with none
// of the special cases for ±1. Nothing is shared with the in-place implementation. As there, the
// result is as long as the longer vector, except that when `c` is zero it is `xs` unchanged.
pub fn integers_add_mul_scalar_naive(xs: &[Integer], ys: &[Integer], c: &Integer) -> Vec<Integer> {
    if *c == 0u32 {
        return xs.to_vec();
    }
    (0..max(xs.len(), ys.len()))
        .map(|i| xs.get(i).unwrap_or(&Integer::ZERO) + c * ys.get(i).unwrap_or(&Integer::ZERO))
        .collect()
}
