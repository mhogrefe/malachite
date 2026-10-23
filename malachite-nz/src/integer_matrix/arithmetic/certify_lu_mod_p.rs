// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2010-2012, 2026 Fredrik Johansson
//
//      Copyright © 2014 Alex J. Best
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::integer_matrix::IntegerMatrix;
use crate::integer_matrix::arithmetic::is_in_reduced_row_echelon_form_with_rank::*;
use crate::integer_matrix::arithmetic::solve::solve_nonsingular;
use alloc::vec;
use alloc::vec::Vec;
use malachite_base::num::basic::traits::Zero;

// Given the rank `rank`, the row permutation `perm`, and the pivot and non-pivot columns
// `pivots_nonpivots` of an LU decomposition of `a` modulo a prime, tries to certify that the rank
// of `a` over the integers is also `rank`. The square submatrix of `a` made of its first `rank`
// permuted rows and its pivot columns must be nonsingular. Returns the first `rank` rows E of the
// fraction-free reduced row echelon form of `a`, with its denominator, if the certification
// succeeds, and `None` otherwise.
//
// This is equivalent to `fmpz_mat_rref_upper_certify_lu_mod_p` from `fmpz_mat/rref_mul.c`, FLINT
// 3.6.0, except that it panics where FLINT throws an exception.
crate_test_fn! {
rref_upper_certify_lu_mod_p(
    a: &IntegerMatrix,
    rank: usize,
    perm: &[usize],
    pivots_nonpivots: &[usize],
) -> Option<(IntegerMatrix, Integer)> {
    let (m, n) = (a.height, a.width);
    let entry = |i: usize, j: usize| &a.elements[i * n + j];
    let (pivots, nonpivots) = pivots_nonpivots.split_at(rank);
    // B is made of the pivot rows and pivot columns, and C of the pivot rows and non-pivot columns.
    let mut b = Vec::with_capacity(rank * rank);
    let mut c = Vec::with_capacity(rank * (n - rank));
    for &i in &perm[..rank] {
        b.extend(pivots.iter().map(|&j| entry(i, j).clone()));
        c.extend(nonpivots.iter().map(|&j| entry(i, j).clone()));
    }
    let b = IntegerMatrix {
        elements: b,
        height: rank,
        width: rank,
    };
    let c = IntegerMatrix {
        elements: c,
        height: rank,
        width: n - rank,
    };
    // Solve B E2 = den C. B is invertible modulo the prime, hence over the integers.
    let (e2, den) = solve_nonsingular(&b, &c).expect("Singular input matrix for solve");
    // Assemble the scaled rref rows into E: den on the pivot "diagonal" and E2 in the non-pivot
    // columns. The remaining pivot-column elements are zero.
    let mut e = vec![Integer::ZERO; rank * n];
    for (i, &pivot) in pivots.iter().enumerate() {
        e[i * n + pivot] = den.clone();
        for (j, &nonpivot) in nonpivots.iter().enumerate() {
            e[i * n + nonpivot] = e2.elements[i * (n - rank) + j].clone();
        }
    }
    let e = IntegerMatrix {
        elements: e,
        height: rank,
        width: n,
    };
    if !is_in_reduced_row_echelon_form_with_rank(&e, &den, rank) {
        return None;
    }
    // D is a basis of the nullspace of E, as an n by n - rank matrix.
    let k = n - rank;
    let mut d = vec![Integer::ZERO; n * k];
    for (j, &nonpivot) in nonpivots.iter().enumerate() {
        d[nonpivot * k + j] = den.clone();
        for (i, &pivot) in pivots.iter().enumerate() {
            d[pivot * k + j] = -&e.elements[i * n + nonpivot];
        }
    }
    let d = IntegerMatrix {
        elements: d,
        height: n,
        width: k,
    };
    // F is made of the non-pivot rows of A. The rank is certified if and only if FD = 0, that is,
    // every remaining row lies in the span of the pivot rows.
    let mut f = Vec::with_capacity((m - rank) * n);
    for &i in &perm[rank..] {
        f.extend_from_slice(&a.elements[i * n..(i + 1) * n]);
    }
    let f = IntegerMatrix {
        elements: f,
        height: m - rank,
        width: n,
    };
    if (f * d).elements.iter().all(|x| *x == 0u32) {
        Some((e, den))
    } else {
        None
    }
}}

// Whether the rank of `a` over the integers is certified to be `rank`, given the rank, permutation,
// and pivot and non-pivot columns of an LU decomposition of `a` modulo a prime.
//
// This is equivalent to `fmpz_mat_rank_certify_lu_mod_p` from `fmpz_mat/rref_mul.c`, FLINT 3.6.0.
crate_test_fn! {
rank_certify_lu_mod_p(
    a: &IntegerMatrix,
    rank: usize,
    perm: &[usize],
    pivots_nonpivots: &[usize],
) -> bool {
    rref_upper_certify_lu_mod_p(a, rank, perm, pivots_nonpivots).is_some()
}}
