// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use crate::natural_matrix::NaturalMatrix;
use alloc::vec;
use alloc::vec::Vec;
use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::{ModInverse, ModMul, ModMulAssign, ModSubAssign};
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::ExactFrom;

/// Solves `a` X = `b` modulo the prime `m` by reducing the augmented matrix to reduced row echelon
/// form, one element at a time. Returns `None` if there is no solution; otherwise, the solution
/// whose elements in the rows of the non-pivot columns of `a` are zero.
pub fn natural_matrix_mod_solve_naive(
    a: &NaturalMatrix,
    b: &NaturalMatrix,
    m: &Natural,
) -> Option<NaturalMatrix> {
    let h = usize::exact_from(a.height());
    let w = usize::exact_from(a.width());
    let k = usize::exact_from(b.width());
    let width = w + k;
    // The augmented matrix [a | b].
    let mut xs: Vec<Natural> = Vec::with_capacity(h * width);
    for i in 0..h {
        xs.extend(a.row_iter(u64::exact_from(i)).cloned());
        xs.extend(b.row_iter(u64::exact_from(i)).cloned());
    }
    let mut pivots = Vec::new();
    let mut row = 0;
    for col in 0..w {
        let Some(p) = (row..h).find(|&i| xs[i * width + col] != 0u32) else {
            continue;
        };
        for j in 0..width {
            xs.swap(row * width + j, p * width + j);
        }
        let inverse = (&xs[row * width + col]).mod_inverse(m).unwrap();
        for j in 0..width {
            xs[row * width + j].mod_mul_assign(&inverse, m);
        }
        for i in 0..h {
            if i != row {
                let c = xs[i * width + col].clone();
                for j in 0..width {
                    let x = (&c).mod_mul(&xs[row * width + j], m);
                    xs[i * width + j].mod_sub_assign(x, m);
                }
            }
        }
        pivots.push(col);
        row += 1;
    }
    if (row..h).any(|i| {
        xs[i * width + w..(i + 1) * width]
            .iter()
            .any(|x| *x != 0u32)
    }) {
        return None;
    }
    let mut x = vec![Natural::ZERO; w * k];
    for (i, &p) in pivots.iter().enumerate() {
        x[p * k..(p + 1) * k].clone_from_slice(&xs[i * width + w..(i + 1) * width]);
    }
    Some(NaturalMatrix::from_owned_elements(
        u64::exact_from(w),
        u64::exact_from(k),
        x,
    ))
}
