// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer_matrix::IntegerMatrix;
use crate::test_util::integer_matrix::determinant::rows;
use malachite_base::matrix::Matrix;
use malachite_base::num::conversion::traits::ExactFrom;

// The rank of a matrix, by fraction-free Gaussian elimination: a row below a pivot is replaced by a
// combination with the pivot row that cancels its entry in the pivot's column.
pub fn naive_rank(a: &IntegerMatrix) -> u64 {
    let mut m = rows(a);
    let (height, width) = (m.len(), usize::exact_from(a.width()));
    let mut rank = 0;
    for column in 0..width {
        let Some(p) = (rank..height).find(|&i| m[i][column] != 0u32) else {
            continue;
        };
        m.swap(p, rank);
        for i in rank + 1..height {
            if m[i][column] != 0u32 {
                let pivot_row = m[rank].clone();
                let (pivot, x) = (&pivot_row[column], m[i][column].clone());
                for (y, z) in m[i][column..].iter_mut().zip(&pivot_row[column..]) {
                    *y = &*y * pivot - z * &x;
                }
            }
        }
        rank += 1;
    }
    u64::exact_from(rank)
}
