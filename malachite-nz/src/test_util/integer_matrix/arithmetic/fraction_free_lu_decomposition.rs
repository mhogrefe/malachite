// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::integer_matrix::IntegerMatrix;
use crate::integer_matrix::arithmetic::fraction_free_lu_decomposition::*;
use alloc::vec::Vec;
use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::{DivExact, MulSubMul};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::conversion::traits::ExactFrom;

/// The fraction-free LU decomposition of `a`, as `FractionFreeLuDecomposition` gives it, by
/// textbook Bareiss elimination on a copy of the whole matrix, one element at a time: at each step,
/// the first row with a nonzero element in the next column is swapped into place, and every element
/// below and to the right of the pivot becomes $(x p - l u) / d$, where $p$ is the pivot, $l$ and
/// $u$ are the elements in its column and row, and $d$ is the previous pivot.
pub fn integer_matrix_fraction_free_lu_decomposition_naive(
    a: &IntegerMatrix,
) -> (Vec<u64>, IntegerMatrix, IntegerMatrix, Integer) {
    let h = usize::exact_from(a.height());
    let w = usize::exact_from(a.width());
    let mut xs = a.elements.clone();
    let mut perm: Vec<usize> = (0..h).collect();
    let mut pivots = Vec::new();
    let mut previous = Integer::ONE;
    let mut row = 0;
    for col in 0..w {
        if row == h {
            break;
        }
        let Some(p) = (row..h).find(|&i| xs[i * w + col] != 0u32) else {
            continue;
        };
        for j in 0..w {
            xs.swap(row * w + j, p * w + j);
        }
        perm.swap(row, p);
        for i in row + 1..h {
            for j in col + 1..w {
                let x = (&xs[i * w + j]).mul_sub_mul(
                    &xs[row * w + col],
                    &xs[i * w + col],
                    &xs[row * w + j],
                );
                xs[i * w + j] = x.div_exact(&previous);
            }
        }
        previous = xs[row * w + col].clone();
        pivots.push(col);
        row += 1;
    }
    let rank = pivots.len();
    let mut l = Vec::with_capacity(h * rank);
    for i in 0..h {
        for (k, &p) in pivots.iter().enumerate() {
            l.push(if i >= k {
                xs[i * w + p].clone()
            } else {
                Integer::ZERO
            });
        }
    }
    let mut u = Vec::with_capacity(rank * w);
    for (k, &p) in pivots.iter().enumerate() {
        for j in 0..w {
            u.push(if j >= p {
                xs[k * w + j].clone()
            } else {
                Integer::ZERO
            });
        }
    }
    (
        perm.into_iter().map(u64::exact_from).collect(),
        IntegerMatrix::from_owned_elements(a.height(), u64::exact_from(rank), l),
        IntegerMatrix::from_owned_elements(u64::exact_from(rank), a.width(), u),
        previous,
    )
}

/// The fraction-free LU decomposition of `a`, as `FractionFreeLuDecomposition` gives it, computed
/// with `Integer` arithmetic throughout, without the path for small elements.
pub fn integer_matrix_fraction_free_lu_decomposition_general(
    a: &IntegerMatrix,
) -> (Vec<u64>, IntegerMatrix, IntegerMatrix, Integer) {
    fraction_free_lu_decomposition_helper(a.clone(), false)
}
