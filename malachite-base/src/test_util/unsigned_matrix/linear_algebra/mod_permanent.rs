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
use alloc::vec;

// The sum modulo m, over the ways of assigning distinct unused columns to the rows `row..`, of the
// products of the chosen elements.
fn permanent_naive_helper<T: PrimitiveUnsigned>(
    xs: &[T],
    n: usize,
    row: usize,
    used: &mut [bool],
    product: T,
    m: T,
) -> T {
    if row == n {
        return product;
    }
    let mut sum = T::ZERO;
    for j in 0..n {
        if !used[j] {
            used[j] = true;
            let p = product.mod_mul(xs[row * n + j], m);
            sum.mod_add_assign(permanent_naive_helper(xs, n, row + 1, used, p, m), m);
            used[j] = false;
        }
    }
    sum
}

// The permanent modulo m of a square matrix as the sum, over all permutations, of the products of
// the elements they select, in time $O(n \cdot n!)$.
pub fn unsigned_matrix_mod_permanent_naive<T: PrimitiveUnsigned>(a: &UnsignedMatrix<T>, m: T) -> T {
    assert!(a.is_square(), "the matrix must be square");
    let n = usize::exact_from(a.height());
    permanent_naive_helper(a.elements_ref(), n, 0, &mut vec![false; n], T::ONE % m, m)
}
