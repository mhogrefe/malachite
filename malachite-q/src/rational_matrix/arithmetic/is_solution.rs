// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2019 Fredrik Johansson
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::rational_matrix::RationalMatrix;
use core::cmp::Ordering;
use malachite_base::matrix::Matrix;
use malachite_base::num::comparison::traits::OrdProduct;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_nz::integer::Integer;
use malachite_nz::integer_matrix::IntegerMatrix;

// Whether `x` solves AX = B, for integer matrices A and B: the columns of X are cleared of
// denominators, giving an integer matrix X' with column denominators d_j, and AX' is compared with
// B times the d_j.
//
// This is equivalent to `_fmpq_mat_check_solution_fmpz_mat` from `fmpq_mat/can_solve_multi_mod.c`,
// FLINT 3.6.0.
crate_test_fn! {
is_solution(x: &RationalMatrix, a: &IntegerMatrix, b: &IntegerMatrix) -> bool {
    assert_eq!(a.width(), x.height(), "the width of A must equal the height of X");
    assert_eq!(a.height(), b.height(), "the heights of A and B must be equal");
    assert_eq!(x.width(), b.width(), "the widths of X and B must be equal");
    let (x_clear, denominators) = x.to_numerators_and_column_denominators();
    let ax = a * x_clear;
    let width = b.width();
    (0..b.height()).all(|i| {
        (0..width).all(|j| {
            let d = Integer::from(&denominators.elements[usize::exact_from(j)]);
            Integer::cmp_product(b.get(i, j).unwrap(), &d, ax.get(i, j).unwrap()) == Ordering::Equal
        })
    })
}}
