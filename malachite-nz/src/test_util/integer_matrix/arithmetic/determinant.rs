// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::integer_matrix::IntegerMatrix;
use crate::integer_matrix::arithmetic::fraction_free_lu_decomposition::fflu_determinant;
use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::{DivExact, MulSubMul, NegAssign};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::NotAssign;

/// The determinant of a square `a`, by Bareiss's fraction-free elimination, one element at a time,
/// swapping in the first row with a nonzero element when a pivot is zero.
pub fn integer_matrix_determinant_naive(a: &IntegerMatrix) -> Integer {
    let n = usize::exact_from(a.height());
    assert_eq!(a.height(), a.width());
    let mut xs = a.elements.clone();
    let mut previous = Integer::ONE;
    let mut negate = false;
    for k in 0..n {
        let Some(p) = (k..n).find(|&i| xs[i * n + k] != 0u32) else {
            return Integer::ZERO;
        };
        if p != k {
            for j in 0..n {
                xs.swap(k * n + j, p * n + j);
            }
            negate.not_assign();
        }
        for i in k + 1..n {
            for j in k + 1..n {
                let x =
                    (&xs[i * n + j]).mul_sub_mul(&xs[k * n + k], &xs[i * n + k], &xs[k * n + j]);
                xs[i * n + j] = x.div_exact(&previous);
            }
        }
        previous = xs[k * n + k].clone();
    }
    let mut det = if n == 0 { Integer::ONE } else { previous };
    if negate {
        det.neg_assign();
    }
    det
}

/// The determinant of a square `a` by Bareiss's algorithm, as `determinant_bareiss` computes it,
/// but with `Integer` arithmetic throughout, without the path for small elements.
pub fn integer_matrix_determinant_bareiss_general(a: &IntegerMatrix) -> Integer {
    let n = usize::exact_from(a.height());
    assert_eq!(a.height(), a.width());
    if n == 0 {
        return Integer::ONE;
    }
    fflu_determinant(&a.elements, n, false)
}
