// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2026 Fredrik Johansson
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::integer_matrix::IntegerMatrix;
use crate::integer_matrix::arithmetic::fraction_matrix::RightHandSide;
use crate::integer_matrix::arithmetic::mul::multi_mod::{OPTIMAL_MODULUS_MIN, next_prime};
use crate::integer_matrix::linear_algebra::solve::solve_nonsingular;
use crate::integer_matrix::linear_algebra::word_lu::WordLu;
use crate::natural::Natural;
use alloc::vec;
use alloc::vec::Vec;
use core::cmp::Ordering::*;
use malachite_base::num::arithmetic::traits::SubMulAssign;
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::comparison::traits::OrdProduct;

// The rows `rows` of `a` restricted to the columns `columns`, and the rows `rows` of `b`.
fn select(
    a: &IntegerMatrix,
    b: RightHandSide<'_>,
    rows: &[usize],
    columns: &[usize],
) -> (IntegerMatrix, IntegerMatrix) {
    let (w, k) = (a.width, b.width);
    let mut a_out = Vec::with_capacity(rows.len() * columns.len());
    let mut b_out = Vec::with_capacity(rows.len() * k);
    for &i in rows {
        a_out.extend(columns.iter().map(|&j| a.elements[i * w + j].clone()));
        b_out.extend_from_slice(&b.elements[i * k..(i + 1) * k]);
    }
    (
        IntegerMatrix {
            elements: a_out,
            height: rows.len(),
            width: columns.len(),
        },
        IntegerMatrix {
            elements: b_out,
            height: rows.len(),
            width: k,
        },
    )
}

// Returns whether the equation of row `i0` of `a` and `b` is a combination of the pivot equations,
// with left-hand side `pivot_rows` restricted to `pivots`, that makes its left-hand side vanish but
// not its right-hand side: a vector y with y^T A = 0 and y^T B != 0, which proves that AX = B has
// no solution (the Fredholm alternative).
fn is_inconsistency_certificate(
    a: &IntegerMatrix,
    b: RightHandSide<'_>,
    a_rank: &IntegerMatrix,
    perm: &[usize],
    pivots: &[usize],
    i0: usize,
) -> bool {
    let (w, k) = (a.width, b.width);
    let rank = pivots.len();
    let row = perm[i0];
    // Solve (A_pivot)^T z = (row i0 of A restricted to the pivot columns)^T.
    let mut transpose = Vec::with_capacity(rank * rank);
    for j in 0..rank {
        transpose.extend((0..rank).map(|i| a_rank.elements[i * rank + j].clone()));
    }
    let transpose = IntegerMatrix {
        elements: transpose,
        height: rank,
        width: rank,
    };
    let rhs = IntegerMatrix {
        elements: pivots.iter().map(|&j| a.elements[row * w + j].clone()).collect(),
        height: rank,
        width: 1,
    };
    let Some((z, z_den)) = solve_nonsingular(&transpose, &rhs) else {
        return false;
    };
    // y^T M on column j, where y = z_den e_{i0} - sum_k z_k e_{perm[k]}
    let combination = |m: &[Integer], width: usize, j: usize| {
        let mut acc = &z_den * &m[row * width + j];
        for (z_k, &p) in z.elements.iter().zip(perm) {
            acc.sub_mul_assign(z_k, &m[p * width + j]);
        }
        acc
    };
    (0..w).all(|j| combination(&a.elements, w, j) == 0u32)
        && (0..k).any(|j| combination(b.elements, k, j) != 0u32)
}

// Solves AX = dB for a matrix A of any shape, returning a solution X and d, or `None` if there is
// none. A is reduced modulo primes until the rank modulo a prime increases; then the system of the
// pivot rows and columns of the LU decomposition modulo that prime is solved exactly, and the
// solution is checked against the other rows. If the check fails, a certificate of inconsistency is
// tried. The system is found to have no solution once the product of the primes exceeds a bound for
// the determinants of the square submatrices of A, since then one of them gave the true rank. The
// denominator d is the one returned by `solve_nonsingular` for the pivot system.
//
// This is equivalent to `_fmpz_mat_can_solve_auto_den` and `_fmpz_mat_can_solve_generic` from
// `fmpz_mat/can_solve_multi_mod_den.c`, FLINT 3.7.0-dev, single-threaded and reducing A modulo one
// prime at a time, which gives the same results.
crate_test_fn! {
can_solve_auto_den<'a>(
    a: &IntegerMatrix,
    b: impl Into<RightHandSide<'a>>,
) -> Option<(IntegerMatrix, Integer)> {
    let b = b.into();
    assert_eq!(
        a.height, b.height,
        "the height of the matrix must equal the height of the right-hand side"
    );
    let (m, n, k) = (a.height, a.width, b.width);
    let zero = || IntegerMatrix {
        elements: vec![Integer::ZERO; n * k],
        height: n,
        width: k,
    };
    // X = 0 solves AX = 0 whatever the rank of A
    if m == 0 || k == 0 || b.is_zero() {
        return Some((zero(), Integer::ONE));
    }
    if n == 0 {
        return None;
    }
    // A prime is bad only if it divides some r by r minor, where r is the rank of A, so once the
    // product of the tested primes exceeds this bound, one of them gave the true rank.
    let det_bound = a.submatrix_determinant_bound();
    let mut tested = Natural::ONE;
    let mut p = OPTIMAL_MODULUS_MIN;
    // The verdict depends only on the rank, so exact work is redone only when the rank increases.
    let mut rank_done = 0;
    loop {
        p = next_prime(p);
        let lu = WordLu::new(a, p);
        let rank = lu.rank;
        if rank > rank_done {
            rank_done = rank;
            // square and nonsingular modulo p: every equation is a pivot equation
            if rank == m && rank == n {
                return solve_nonsingular(a, b);
            }
            let perm = &lu.perm;
            let pivots = &lu.pivots_nonpivots[..rank];
            let (a_rank, b_rank) = select(a, b, &perm[..rank], pivots);
            let (a_rest, b_rest) = select(a, b, &perm[rank..], pivots);
            if let Some((z, den)) = solve_nonsingular(&a_rank, &b_rank) {
                // The pivot equations hold by construction; check the others.
                let t = &a_rest * &z;
                let i0 = t
                    .elements
                    .chunks_exact(k)
                    .zip(b_rest.elements.chunks_exact(k))
                    .position(|(xs, ys)| {
                        xs.iter()
                            .zip(ys)
                            .any(|(x, y)| Integer::cmp_product(y, &den, x) != Equal)
                    });
                let Some(i0) = i0 else {
                    let mut x = zero();
                    for (&j, row) in pivots.iter().zip(z.elements.chunks_exact(k)) {
                        x.elements[j * k..(j + 1) * k].clone_from_slice(row);
                    }
                    return Some((x, den));
                };
                if is_inconsistency_certificate(a, b, &a_rank, perm, pivots, rank + i0) {
                    return None;
                }
                // Otherwise the rank modulo p was too small.
            }
        }
        tested *= Natural::from(p);
        if tested > det_bound {
            return None;
        }
    }
}}
