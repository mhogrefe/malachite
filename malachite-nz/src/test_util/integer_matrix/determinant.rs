// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::integer_matrix::IntegerMatrix;
use alloc::vec::Vec;
use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::{DivExactAssign, MulSubMulAssign};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::logic::traits::NotAssign;

// The rows of a matrix as `Vec`s, for elimination.
pub(crate) fn rows(a: &IntegerMatrix) -> Vec<Vec<Integer>> {
    (0..a.height())
        .map(|i| a.row_iter(i).cloned().collect())
        .collect()
}

// The determinant of a square matrix, by Bareiss's fraction-free elimination with row swaps. Every
// division is exact.
pub fn naive_determinant(a: &IntegerMatrix) -> Integer {
    assert!(a.is_square());
    let mut m = rows(a);
    let n = m.len();
    if n == 0 {
        return Integer::ONE;
    }
    let mut negate = false;
    let mut previous = Integer::ONE;
    for k in 0..n {
        let Some(p) = (k..n).find(|&i| m[i][k] != 0u32) else {
            return Integer::ZERO;
        };
        if p != k {
            m.swap(p, k);
            negate.not_assign();
        }
        let pivot_row = m[k].clone();
        for row in &mut m[k + 1..] {
            let x = row[k].clone();
            for (y, z) in row[k + 1..].iter_mut().zip(&pivot_row[k + 1..]) {
                y.mul_sub_mul_assign(&pivot_row[k], &x, z);
                y.div_exact_assign(&previous);
            }
        }
        previous = m[k][k].clone();
    }
    if negate { -previous } else { previous }
}
