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

/// Solves $AX = B$ modulo $m$, where $A$ is lower triangular, by substitution, one element at a
/// time.
pub fn natural_matrix_mod_solve_lower_triangular_naive(
    a: &NaturalMatrix,
    b: &NaturalMatrix,
    m: &Natural,
    unit_diagonal: bool,
) -> NaturalMatrix {
    let n = a.height();
    let k = b.width();
    let mut x = b.clone();
    if *m == 1u32 {
        return x;
    }
    for j in 0..k {
        for i in 0..n {
            let mut s = b.get(i, j).unwrap().clone();
            for l in 0..i {
                let p = a.get(i, l).unwrap().mod_mul(x.get(l, j).unwrap(), m);
                s.mod_sub_assign(p, m);
            }
            if !unit_diagonal {
                s.mod_mul_assign(a.get(i, i).unwrap().mod_inverse(m).unwrap(), m);
            }
            *x.get_mut(i, j).unwrap() = s;
        }
    }
    x
}
