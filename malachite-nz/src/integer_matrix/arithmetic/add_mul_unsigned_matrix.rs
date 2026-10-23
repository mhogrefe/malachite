// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2011 Fredrik Johansson
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::integer_matrix::IntegerMatrix;
use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::AddMulAssign;
use malachite_base::unsigned_matrix::UnsignedMatrix;

// Adds `c` times the matrix of words `a` to `b`, which must have the same shape.
//
// This is equivalent to `fmpz_mat_scalar_addmul_nmod_mat_fmpz` from `fmpz_mat/scalar.c`, FLINT
// 3.6.0, except that it panics if the shapes differ, which FLINT does not check.
crate_test_fn! {
add_mul_unsigned_matrix_assign(b: &mut IntegerMatrix, a: &UnsignedMatrix<u64>, c: &Integer) {
    assert_eq!(b.height(), a.height(), "the matrices must have the same height");
    assert_eq!(b.width(), a.width(), "the matrices must have the same width");
    for (x, &y) in b.elements.iter_mut().zip(&a.elements) {
        x.add_mul_assign(c, Integer::from(y));
    }
}}
