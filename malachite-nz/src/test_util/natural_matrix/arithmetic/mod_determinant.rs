// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use crate::natural_matrix::NaturalMatrix;
use crate::natural_matrix::arithmetic::mod_determinant::{det_berkowitz, det_lu};
use alloc::vec;
use alloc::vec::Vec;
use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::{ModAddAssign, ModMulAssign, ModNeg, Parity};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::NotAssign;

/// The determinant of the square matrix `a` modulo `m`, which may be composite, from the
/// definition: the signed sum over all permutations. The time taken is proportional to n!, so this
/// is only practical for small matrices.
pub fn natural_matrix_mod_determinant_naive(a: &NaturalMatrix, m: &Natural) -> Natural {
    let n = usize::exact_from(a.height());
    if *m == 1u32 {
        return Natural::ZERO;
    }
    // Heap's algorithm, tracking the sign of each permutation.
    let mut perm: Vec<usize> = (0..n).collect();
    let mut c = vec![0; n];
    let mut odd = false;
    let term = |perm: &[usize], odd: bool| {
        let mut p = Natural::ONE;
        for (i, &j) in perm.iter().enumerate() {
            p.mod_mul_assign(&a.elements[i * n + j], m);
        }
        if odd { p.mod_neg(m) } else { p }
    };
    let mut det = term(&perm, odd);
    let mut i = 1;
    while i < n {
        if c[i] < i {
            if i.even() {
                perm.swap(0, i);
            } else {
                perm.swap(c[i], i);
            }
            odd.not_assign();
            det.mod_add_assign(term(&perm, odd), m);
            c[i] += 1;
            i = 1;
        } else {
            c[i] = 0;
            i += 1;
        }
    }
    det
}

/// The determinant of the square matrix `a` modulo `m`, from its characteristic polynomial,
/// computed with Berkowitz's algorithm. The large-modulus algorithm is used even when `m` fits in a
/// limb.
pub fn natural_matrix_mod_determinant_berkowitz(a: &NaturalMatrix, m: &Natural) -> Natural {
    det_berkowitz(&a.elements, usize::exact_from(a.height()), m)
}

/// The determinant of the square matrix `a` modulo `m`, from its LU decomposition, or `None` if a
/// pivot is not invertible. The large-modulus algorithm is used even when `m` fits in a limb.
pub fn natural_matrix_mod_determinant_lu(a: &NaturalMatrix, m: &Natural) -> Option<Natural> {
    det_lu(a.clone(), m)
}
