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
use crate::integer_matrix::arithmetic::fraction_matrix::fractions_to_numerators_and_denominator;
use crate::integer_matrix::arithmetic::solve_dixon::solve_dixon;
use crate::natural::Natural;
use malachite_base::num::arithmetic::traits::Parity;
use malachite_base::num::basic::traits::{NegativeOne, One, Zero};

// A divisor of the determinant of a square matrix A, likely to be a large one: the least common
// denominator of the solution x of Ax = b, where b is the vector (-1, 1, -1, 1, ...), found by
// Dixon's p-adic lifting. Returns zero if A is singular.
//
// This is equivalent to `fmpz_mat_det_divisor` from `fmpz_mat/det_divisor.c`, FLINT 3.6.0, which
// finds the same denominator by lifting the solution modulo a power of p with
// `fmpz_mat_solve_dixon` and reconstructing it; the solution is unique, so the denominator is the
// same.
crate_test_fn! {
determinant_divisor(a: &IntegerMatrix) -> Natural {
    assert_eq!(a.height, a.width, "the matrix must be square");
    let n = a.height;
    let b = IntegerMatrix {
        elements: (0..n)
            .map(|i| if i.even() { Integer::NEGATIVE_ONE } else { Integer::ONE })
            .collect(),
        height: n,
        width: 1,
    };
    solve_dixon(a, &b).map_or(Natural::ZERO, |x| {
        fractions_to_numerators_and_denominator(&x, n, 1).1
    })
}}
