// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::integer_matrix::IntegerMatrix;
use crate::integer_matrix::arithmetic::fraction_free_lu_decomposition::*;
use crate::integer_matrix::arithmetic::solve::{
    solve_fflu_precomp, solve_fflu_precomp_with_options,
};
use crate::natural_matrix::arithmetic::mod_determinant::permutation_is_odd;
use crate::test_util::integer_matrix::arithmetic::determinant::*;
use alloc::vec;
use alloc::vec::Vec;
use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::{NegAssign, Parity};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::conversion::traits::ExactFrom;

/// The solution $(X, d)$ of $AX = dB$ with $X = \operatorname{adj}(A)B$ and $d = \det A$, for a
/// square `a`, or `None` if `a` is singular. As in FLINT, if `a` or `b` has no elements, the result
/// is a zero matrix and 1, even if `a` is singular. The adjugate is computed from minors, each by
/// `integer_matrix_determinant_naive`.
pub fn integer_matrix_solve_naive(
    a: &IntegerMatrix,
    b: &IntegerMatrix,
) -> Option<(IntegerMatrix, Integer)> {
    let n = a.height();
    assert_eq!(a.width(), n);
    assert_eq!(b.height(), n);
    // As in FLINT, a system with no unknowns or no right-hand sides is solved with denominator 1.
    if n == 0 || b.width() == 0 {
        return Some((
            IntegerMatrix::from_owned_elements(
                n,
                b.width(),
                vec![Integer::ZERO; usize::exact_from(n * b.width())],
            ),
            Integer::ONE,
        ));
    }
    let det = integer_matrix_determinant_naive(a);
    if det == 0u32 {
        return None;
    }
    // adj(A)[i][j] = (-1)^(i + j) det(A with row j and column i removed).
    let minor = |row: u64, col: u64| {
        let elements: Vec<Integer> = (0..n)
            .filter(|&i| i != row)
            .flat_map(|i| {
                a.row_iter(i)
                    .enumerate()
                    .filter(|&(j, _)| u64::exact_from(j) != col)
                    .map(|(_, x)| x.clone())
                    .collect::<Vec<_>>()
            })
            .collect();
        integer_matrix_determinant_naive(&IntegerMatrix::from_owned_elements(
            n - 1,
            n - 1,
            elements,
        ))
    };
    let mut adjugate = Vec::new();
    for i in 0..n {
        for j in 0..n {
            let m = minor(j, i);
            adjugate.push(if (i + j).even() { m } else { -m });
        }
    }
    let adjugate = IntegerMatrix::from_owned_elements(n, n, adjugate);
    let x = adjugate * b;
    Some((x, det))
}

/// The result of `solve_fflu_precomp` for the fraction-free LU decomposition of `a`, without the
/// rank check, and the right-hand side `b`: a `width`-by-`b.width()` matrix, or `None` if a
/// division was not exact.
pub fn integer_matrix_solve_fflu_precomp_from_matrix(
    a: &IntegerMatrix,
    b: &IntegerMatrix,
) -> Option<IntegerMatrix> {
    let height = usize::exact_from(a.height());
    let width = usize::exact_from(a.width());
    let mut lu = a.elements.clone();
    let mut perm: Vec<usize> = (0..height).collect();
    fflu_in_place_with_options(&mut lu, height, width, &mut perm, false, true);
    let k = usize::exact_from(b.width());
    solve_fflu_precomp(&perm, &lu, height, width, &b.elements, k)
        .map(|x| IntegerMatrix::from_owned_elements(a.width(), b.width(), x))
}

/// `integer_matrix_solve_fflu_precomp_from_matrix`, with `Integer` arithmetic throughout, without
/// the path for small elements.
pub fn integer_matrix_solve_fflu_precomp_from_matrix_general(
    a: &IntegerMatrix,
    b: &IntegerMatrix,
) -> Option<IntegerMatrix> {
    let height = usize::exact_from(a.height());
    let width = usize::exact_from(a.width());
    let mut lu = a.elements.clone();
    let mut perm: Vec<usize> = (0..height).collect();
    fflu_in_place_with_options(&mut lu, height, width, &mut perm, false, true);
    let k = usize::exact_from(b.width());
    solve_fflu_precomp_with_options(&perm, &lu, height, width, &b.elements, k, false)
        .map(|x| IntegerMatrix::from_owned_elements(a.width(), b.width(), x))
}

/// The solution of `solve_fflu`, computed with `Integer` arithmetic throughout, without the paths
/// for small elements in the decomposition and the substitutions.
pub fn integer_matrix_solve_fflu_general(
    a: &IntegerMatrix,
    b: &IntegerMatrix,
) -> Option<(IntegerMatrix, Integer)> {
    let n = usize::exact_from(a.height());
    let k = usize::exact_from(b.width());
    if n == 0 || k == 0 {
        return Some((
            IntegerMatrix::from_owned_elements(a.height(), b.width(), vec![Integer::ZERO; n * k]),
            Integer::ONE,
        ));
    }
    let mut lu = a.elements.clone();
    let mut perm: Vec<usize> = (0..n).collect();
    let (rank, mut den) = fflu_in_place_with_options(&mut lu, n, n, &mut perm, true, false);
    if rank != n {
        return None;
    }
    let mut x = solve_fflu_precomp_with_options(&perm, &lu, n, n, &b.elements, k, false).unwrap();
    if permutation_is_odd(&perm) {
        den.neg_assign();
        for y in &mut x {
            y.neg_assign();
        }
    }
    Some((
        IntegerMatrix::from_owned_elements(a.height(), b.width(), x),
        den,
    ))
}
