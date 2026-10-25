// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::integer_matrix::IntegerMatrix;
use malachite_base::matrix::Matrix;
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::conversion::traits::ExactFrom;

// The sum, over the ways of assigning distinct unused columns to the rows `row..`, of the products
// of the chosen elements.
fn permanent_naive_helper(
    xs: &[Integer],
    n: usize,
    row: usize,
    used: &mut [bool],
    product: &Integer,
) -> Integer {
    if row == n {
        return product.clone();
    }
    let mut sum = Integer::ZERO;
    for j in 0..n {
        if !used[j] {
            used[j] = true;
            sum += permanent_naive_helper(xs, n, row + 1, used, &(product * &xs[row * n + j]));
            used[j] = false;
        }
    }
    sum
}

// The permanent of a square matrix as the sum, over all permutations, of the products of the
// elements they select, in time $O(n \cdot n!)$.
pub fn integer_matrix_permanent_naive(a: &IntegerMatrix) -> Integer {
    assert!(a.is_square(), "the matrix must be square");
    let n = usize::exact_from(a.height());
    permanent_naive_helper(a.elements_ref(), n, 0, &mut vec![false; n], &Integer::ONE)
}
