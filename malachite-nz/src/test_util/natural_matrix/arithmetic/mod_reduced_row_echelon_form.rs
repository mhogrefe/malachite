// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use crate::natural_matrix::NaturalMatrix;
use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::{ModInverse, ModMul, ModMulAssign, ModSubAssign};
use malachite_base::num::conversion::traits::ExactFrom;

/// The reduced row echelon form of `a` modulo the prime `m`, by Gauss-Jordan elimination, one
/// element at a time.
pub fn natural_matrix_mod_reduced_row_echelon_form_naive(
    a: &NaturalMatrix,
    m: &Natural,
) -> NaturalMatrix {
    let h = usize::exact_from(a.height());
    let w = usize::exact_from(a.width());
    let mut xs = a.elements.clone();
    let mut row = 0;
    for col in 0..w {
        let Some(p) = (row..h).find(|&i| xs[i * w + col] != 0u32) else {
            continue;
        };
        for j in 0..w {
            xs.swap(row * w + j, p * w + j);
        }
        let inverse = (&xs[row * w + col]).mod_inverse(m).unwrap();
        for j in 0..w {
            xs[row * w + j].mod_mul_assign(&inverse, m);
        }
        for i in 0..h {
            if i != row {
                let c = xs[i * w + col].clone();
                for j in 0..w {
                    let x = (&c).mod_mul(&xs[row * w + j], m);
                    xs[i * w + j].mod_sub_assign(x, m);
                }
            }
        }
        row += 1;
    }
    NaturalMatrix::from_owned_elements(a.height(), a.width(), xs)
}
