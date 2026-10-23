// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2022 William Hart
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer_matrix::IntegerMatrix;
use crate::integer_matrix::arithmetic::determinant_bound::determinant_bound_helper;
use crate::integer_matrix::arithmetic::fraction_matrix::*;
use crate::integer_matrix::arithmetic::mul::multi_mod::{OPTIMAL_MODULUS_MIN, next_prime};
use crate::integer_matrix::arithmetic::solve_dixon::solve_dixon;
use crate::integer_matrix::arithmetic::word_lu::WordLu;
use crate::natural::Natural;
use alloc::vec;
use alloc::vec::Vec;
use malachite_base::num::basic::traits::One;

// Solves AX = B by Dixon's p-adic lifting, for a matrix A of any shape, returning the elements of a
// solution X, or `None` if there is none. For each prime p, the rows and columns of A that hold the
// pivots of its LU decomposition modulo p form a square matrix that is nonsingular over the
// integers; the corresponding subsystem is solved by Dixon's lifting, its solution is extended by
// zeros, and the result is checked against the whole system. The system is found to have no
// solution once the product of the primes tried exceeds a bound for the determinant of A.
//
// This is equivalent to `fmpq_mat_can_solve_fmpz_mat_dixon` from `fmpq_mat/can_solve_dixon.c`,
// FLINT 3.6.0, except that it panics if the dimensions are incompatible, and that it handles the
// systems whose matrix or right-hand side has no elements, which FLINT leaves to
// `fmpq_mat_can_solve_dixon`.
#[doc(hidden)]
pub fn can_solve_dixon<'a>(
    a: &IntegerMatrix,
    b: impl Into<RightHandSide<'a>>,
) -> Option<Vec<Fraction>> {
    let b = b.into();
    assert_eq!(
        a.height, b.height,
        "the height of the matrix must equal the height of the right-hand side"
    );
    let (w, k) = (a.width, b.width);
    if a.height == 0 || k == 0 {
        return Some(vec![Fraction::ZERO; w * k]);
    }
    if w == 0 {
        return if b.is_zero() { Some(Vec::new()) } else { None };
    }
    let det_bound = determinant_bound_helper(a, true);
    let mut tested = Natural::ONE;
    let mut p = OPTIMAL_MODULUS_MIN;
    loop {
        p = next_prime(p);
        let lu = WordLu::new(a, p);
        let rank = lu.rank;
        let pivots = &lu.pivots_nonpivots[..rank];
        // The pivot rows and columns of A, and the pivot rows of B.
        let mut a_rank = Vec::with_capacity(rank * rank);
        let mut b_rank = Vec::with_capacity(rank * k);
        for &i in &lu.perm[..rank] {
            a_rank.extend(pivots.iter().map(|&j| a.elements[i * w + j].clone()));
            b_rank.extend_from_slice(&b.elements[i * k..(i + 1) * k]);
        }
        let a_rank = IntegerMatrix {
            elements: a_rank,
            height: rank,
            width: rank,
        };
        let b_rank = IntegerMatrix {
            elements: b_rank,
            height: rank,
            width: k,
        };
        if let Some(x_rank) = solve_dixon(&a_rank, &b_rank) {
            let mut x = vec![Fraction::ZERO; w * k];
            for (row, &j) in x_rank.chunks_exact(k).zip(pivots) {
                x[j * k..(j + 1) * k].clone_from_slice(row);
            }
            if is_solution(&x, w, k, a, b) {
                return Some(x);
            }
        }
        tested *= Natural::from(p);
        if tested > det_bound {
            return None;
        }
    }
}
