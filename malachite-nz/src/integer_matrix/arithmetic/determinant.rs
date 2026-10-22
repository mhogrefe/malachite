// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2010, 2011, 2018 Fredrik Johansson
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::integer_matrix::IntegerMatrix;
use crate::integer_matrix::arithmetic::fraction_free_lu_decomposition::fflu_determinant;
use malachite_base::num::arithmetic::traits::{AddMul, AddMulAssign, MulSubMul};
use malachite_base::num::basic::traits::One;

// The determinant of a square matrix of dimension at most 4, by cofactor expansion.
//
// This is equivalent to `fmpz_mat_det_cofactor` from `fmpz_mat/det_cofactor.c`, FLINT 3.6.0.
crate_test_fn! {
determinant_cofactor(a: &IntegerMatrix) -> Integer {
    assert_eq!(a.height, a.width, "the matrix must be square");
    let e = |i: usize, j: usize| &a.elements[i * a.width + j];
    let fmms = |a: &Integer, b: &Integer, c: &Integer, d: &Integer| a.mul_sub_mul(b, c, d);
    match a.height {
        0 => Integer::ONE,
        1 => e(0, 0).clone(),
        2 => fmms(e(0, 0), e(1, 1), e(0, 1), e(1, 0)),
        3 => {
            (fmms(e(1, 0), e(2, 1), e(1, 1), e(2, 0)) * e(0, 2))
                .add_mul(fmms(e(1, 2), e(2, 0), e(1, 0), e(2, 2)), e(0, 1))
                .add_mul(fmms(e(1, 1), e(2, 2), e(1, 2), e(2, 1)), e(0, 0))
        }
        4 => {
            let mut det =
                fmms(e(0, 3), e(1, 2), e(0, 2), e(1, 3)) * fmms(e(2, 1), e(3, 0), e(2, 0), e(3, 1));
            for (a, b) in [
                (
                    fmms(e(0, 1), e(1, 3), e(0, 3), e(1, 1)),
                    fmms(e(2, 2), e(3, 0), e(2, 0), e(3, 2)),
                ),
                (
                    fmms(e(0, 2), e(1, 1), e(0, 1), e(1, 2)),
                    fmms(e(2, 3), e(3, 0), e(2, 0), e(3, 3)),
                ),
                (
                    fmms(e(0, 3), e(1, 0), e(0, 0), e(1, 3)),
                    fmms(e(2, 2), e(3, 1), e(2, 1), e(3, 2)),
                ),
                (
                    fmms(e(0, 0), e(1, 2), e(0, 2), e(1, 0)),
                    fmms(e(2, 3), e(3, 1), e(2, 1), e(3, 3)),
                ),
                (
                    fmms(e(0, 1), e(1, 0), e(0, 0), e(1, 1)),
                    fmms(e(2, 3), e(3, 2), e(2, 2), e(3, 3)),
                ),
            ] {
                det.add_mul_assign(a, b);
            }
            det
        }
        _ => panic!("cofactor expansion is only implemented for dimensions up to 4"),
    }
}}

// The determinant of a square matrix, by Bareiss's fraction-free elimination: the last pivot of the
// fraction-free LU decomposition, negated if the row permutation is odd.
//
// This is equivalent to `fmpz_mat_det_bareiss` from `fmpz_mat/det_bareiss.c`, FLINT 3.6.0.
crate_test_fn! {
determinant_bareiss(a: &IntegerMatrix) -> Integer {
    assert_eq!(a.height, a.width, "the matrix must be square");
    let n = a.height;
    if n == 0 {
        return Integer::ONE;
    }
    fflu_determinant(&a.elements, n, true)
}}
