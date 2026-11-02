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

/// Solves $AX = B$ modulo $m$, where $A$ is upper triangular, by substitution, one element at a
/// time.
pub fn unsigned_matrix_mod_solve_upper_triangular_naive<T: PrimitiveUnsigned>(
    a: &UnsignedMatrix<T>,
    b: &UnsignedMatrix<T>,
    m: T,
    unit_diagonal: bool,
) -> UnsignedMatrix<T> {
    let n = a.height();
    let k = b.width();
    let mut x: Vec<T> = b.elements.clone();
    if m == T::ONE {
        return b.clone();
    }
    for j in 0..k {
        for i in (0..n).rev() {
            let mut s = *b.get(i, j).unwrap();
            for l in i + 1..n {
                let p = a
                    .get(i, l)
                    .unwrap()
                    .mod_mul(x[usize::exact_from(l * k + j)], m);
                s.mod_sub_assign(p, m);
            }
            if !unit_diagonal {
                s.mod_mul_assign(a.get(i, i).unwrap().mod_inverse(m).unwrap(), m);
            }
            x[usize::exact_from(i * k + j)] = s;
        }
    }
    UnsignedMatrix::from_owned_elements(n, k, x)
}
