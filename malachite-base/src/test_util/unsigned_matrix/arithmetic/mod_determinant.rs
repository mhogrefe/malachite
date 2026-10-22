// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::matrix::Matrix;
use crate::num::arithmetic::traits::{ExtendedGcd, Parity};
use crate::num::basic::signeds::PrimitiveSigned;
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::num::conversion::traits::{ExactFrom, WrappingFrom};
use crate::num::logic::traits::NotAssign;
use crate::unsigned_matrix::UnsignedMatrix;
use crate::unsigned_matrix::arithmetic::mod_determinant::{charpoly_berkowitz, det_howell, det_lu};
use alloc::vec;
use alloc::vec::Vec;

/// The determinant of the square matrix `a` modulo `m`, which may be composite, from the
/// definition: the signed sum over all permutations. The time taken is proportional to n!, so this
/// is only practical for small matrices.
pub fn unsigned_matrix_mod_determinant_naive<T: PrimitiveUnsigned>(
    a: &UnsignedMatrix<T>,
    m: T,
) -> T {
    let n = usize::exact_from(a.height());
    if m == T::ONE {
        return T::ZERO;
    }
    // Heap's algorithm, tracking the sign of each permutation.
    let mut perm: Vec<usize> = (0..n).collect();
    let mut c = vec![0; n];
    let mut odd = false;
    let term = |perm: &[usize], odd: bool| {
        let mut p = T::ONE;
        for (i, &j) in perm.iter().enumerate() {
            p.mod_mul_assign(a.elements[i * n + j], m);
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
/// computed with Berkowitz's algorithm.
pub fn unsigned_matrix_mod_determinant_berkowitz<T: PrimitiveUnsigned>(
    a: &UnsignedMatrix<T>,
    m: T,
) -> T {
    let n = usize::exact_from(a.height());
    let mut cp = vec![T::ZERO; n + 1];
    charpoly_berkowitz(&mut cp, &a.elements, n, m);
    if n.odd() { cp[0].mod_neg(m) } else { cp[0] }
}

/// The determinant of the square matrix `a` modulo the prime `m`, from its LU decomposition.
pub fn unsigned_matrix_mod_determinant_lu<T: PrimitiveUnsigned>(a: &UnsignedMatrix<T>, m: T) -> T {
    det_lu(a.clone(), m)
}

/// The determinant of the square matrix `a` modulo `m`, which may be composite, by Euclidean row
/// operations.
pub fn unsigned_matrix_mod_determinant_howell<
    U: PrimitiveUnsigned + ExtendedGcd<U, Gcd = U, Cofactor = S> + WrappingFrom<S>,
    S: PrimitiveSigned + WrappingFrom<U>,
>(
    a: &UnsignedMatrix<U>,
    m: U,
) -> U {
    let mut xs = a.elements.clone();
    det_howell::<U, S>(&mut xs, usize::exact_from(a.height()), m)
}
