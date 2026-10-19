// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::matrix::Matrix;
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::num::conversion::traits::ExactFrom;
use crate::unsigned_matrix::UnsignedMatrix;
use alloc::vec::Vec;

// The rank of a matrix over the integers modulo a prime `p`, by fraction-free Gaussian elimination
// modulo `p`. Every pivot is nonzero, hence invertible, so the combinations preserve the row space.
pub fn naive_rank_mod_prime<T: PrimitiveUnsigned>(a: &UnsignedMatrix<T>, p: T) -> u64 {
    let mut m: Vec<Vec<T>> = (0..a.height())
        .map(|i| a.row_iter(i).copied().collect())
        .collect();
    let (height, width) = (m.len(), usize::exact_from(a.width()));
    let mut rank = 0;
    for column in 0..width {
        let Some(q) = (rank..height).find(|&i| m[i][column] != T::ZERO) else {
            continue;
        };
        m.swap(q, rank);
        for i in rank + 1..height {
            if m[i][column] != T::ZERO {
                let pivot_row = m[rank].clone();
                let (pivot, x) = (pivot_row[column], m[i][column]);
                for (y, &z) in m[i][column..].iter_mut().zip(&pivot_row[column..]) {
                    *y = y.mod_mul(pivot, p).mod_sub(z.mod_mul(x, p), p);
                }
            }
        }
        rank += 1;
    }
    u64::exact_from(rank)
}
