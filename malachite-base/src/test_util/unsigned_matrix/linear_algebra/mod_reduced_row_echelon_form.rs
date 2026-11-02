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

/// The reduced row echelon form of `a` modulo the prime `m`, by Gauss-Jordan elimination, one
/// element at a time.
pub fn unsigned_matrix_mod_reduced_row_echelon_form_naive<T: PrimitiveUnsigned>(
    a: &UnsignedMatrix<T>,
    m: T,
) -> UnsignedMatrix<T> {
    let h = usize::exact_from(a.height());
    let w = usize::exact_from(a.width());
    let mut xs: Vec<T> = (0..a.height())
        .flat_map(|i| a.row_iter(i).copied().collect::<Vec<_>>())
        .collect();
    let mut row = 0;
    for col in 0..w {
        let Some(p) = (row..h).find(|&i| xs[i * w + col] != T::ZERO) else {
            continue;
        };
        for j in 0..w {
            xs.swap(row * w + j, p * w + j);
        }
        let inverse = xs[row * w + col].mod_inverse(m).unwrap();
        for j in 0..w {
            xs[row * w + j].mod_mul_assign(inverse, m);
        }
        for i in 0..h {
            if i != row {
                let c = xs[i * w + col];
                for j in 0..w {
                    let x = c.mod_mul(xs[row * w + j], m);
                    xs[i * w + j].mod_sub_assign(x, m);
                }
            }
        }
        row += 1;
    }
    UnsignedMatrix::from_owned_elements(a.height(), a.width(), xs)
}
