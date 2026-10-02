// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::test_util::unsigned_polynomial::arithmetic::mod_mul::mod_mul_naive;
use alloc::vec::Vec;

// The middle product of `xs` and `ys` modulo $m$: the `k` coefficients of their schoolbook product
// from coefficient `xs.len() - 1` on, where `ys` has length `xs.len() + k - 1`.
pub fn mod_mul_middle_naive<T: PrimitiveUnsigned>(xs: &[T], ys: &[T], m: T) -> Vec<T> {
    let n = xs.len();
    let k = ys.len() + 1 - n;
    mod_mul_naive(xs, ys, m)[n - 1..n - 1 + k].to_vec()
}
