// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2011, 2019 Fredrik Johansson
//
//      Copyright © 2019, 2020 William Hart
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::integer_matrix::IntegerMatrix;
use crate::integer_matrix::arithmetic::fraction_matrix::*;
use crate::integer_matrix::arithmetic::mul::multi_mod::{OPTIMAL_MODULUS_MIN, next_prime};
use crate::integer_matrix::arithmetic::word_lu::can_solve_inner;
use crate::natural::Natural;
use crate::natural_matrix::NaturalMatrix;
use alloc::vec;
use alloc::vec::Vec;
use core::cmp::Ordering::{self, *};
use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::Crt;
use malachite_base::num::basic::traits::One;
use malachite_base::num::conversion::traits::ExactFrom;

// Compares the row permutation and pivot columns of a solve modulo a prime with the best ones so
// far: `Less` if they have earlier pivots or row swaps, `Greater` if they have later ones, and
// `Equal` if they are the same. A pivot column of -1 is unset.
//
// This is equivalent to `_permpiv_cmp` from `fmpq_mat/can_solve_multi_mod.c`, FLINT 3.6.0, which
// returns 1, -1, and 0 for `Less`, `Greater`, and `Equal`.
fn compare_permutations_and_pivots(
    perm: &[usize],
    best_perm: &[usize],
    pivots: &[isize],
    best_pivots: &[isize],
) -> Ordering {
    for (((&p, &best_p), &pivot), &best_pivot) in
        perm.iter().zip(best_perm).zip(pivots).zip(best_pivots)
    {
        if (p < best_p && pivot <= best_pivot) || (p == best_p && pivot < best_pivot && pivot != -1)
        {
            return Less;
        } else if p > best_p || pivot > best_pivot {
            return Greater;
        }
    }
    Equal
}

// Solves AX = B with the multimodular algorithm, for a matrix A of any shape, returning the
// elements of a solution X, or `None` if there is none. The system is solved modulo primes, keeping
// only the solutions whose rank, row permutation, and pivots are the best seen, and the solutions
// are combined by the Chinese remainder theorem until rational reconstruction recovers a solution
// that checks. The system is found to have no solution once the product of the primes modulo which
// it has none exceeds a bound for the determinants of the square submatrices of A.
//
// This is equivalent to `fmpq_mat_can_solve_fmpz_mat_multi_mod` from
// `fmpq_mat/can_solve_multi_mod.c`, FLINT 3.6.0, except that it panics if the dimensions are
// incompatible.
#[doc(hidden)]
pub fn can_solve_multi_mod<'a>(
    a: &IntegerMatrix,
    b: impl Into<RightHandSide<'a>>,
) -> Option<Vec<Fraction>> {
    let b = b.into();
    assert_eq!(
        a.height, b.height,
        "the height of the matrix must equal the height of the right-hand side"
    );
    let (h, w, k) = (a.height, a.width, b.width);
    // FLINT does not special-case k == 0, but then every prime gives the zero solution.
    if h == 0 || k == 0 {
        return Some(vec![Fraction::ZERO; w * k]);
    }
    if w == 0 {
        return if b.is_zero() { Some(Vec::new()) } else { None };
    }
    let det_bound = a.submatrix_determinant_bound();
    let mut product = Natural::ONE;
    let mut bad_product = Natural::ONE;
    // The current and best row permutations and pivot columns.
    let mut perm: Vec<usize> = (0..h).collect();
    let mut best_perm = vec![0; h];
    let mut pivot_buffer = vec![-1; h << 1];
    let (pivots, best_pivots) = pivot_buffer.split_at_mut(h);
    let mut best_rank = None;
    let mut x = NaturalMatrix::zero(u64::exact_from(w), u64::exact_from(k));
    let mut p = OPTIMAL_MODULUS_MIN;
    // The number of primes used, and the number at which the next check is made.
    let mut i = 0;
    let mut next_i = 1;
    loop {
        if i == next_i {
            next_i = next_check_iteration(i);
            if let Some(xs) = reconstruct_fractions(x.elements.iter(), &product)
                && is_solution(&xs, w, k, a, b)
            {
                return Some(xs);
            }
        }
        i += 1;
        let x_mod = loop {
            p = next_prime(p);
            let (rank, x_mod) = can_solve_inner(a, b, p, &mut perm, pivots);
            let Some(x_mod) = x_mod else {
                bad_product *= Natural::from(p);
                if bad_product > det_bound {
                    return None;
                }
                continue;
            };
            let ordering = compare_permutations_and_pivots(&perm, &best_perm, pivots, best_pivots);
            if best_rank == Some(rank) && ordering == Equal {
                // Everything is the same: accept.
                break x_mod;
            }
            if best_rank.is_some_and(|r| rank < r) {
                // The rank is too low: reject.
                continue;
            }
            if best_rank.is_none_or(|r| rank > r) || ordering == Less {
                // The rank increased, or the pivots or row swaps are earlier: restart.
                best_perm.copy_from_slice(&perm);
                best_pivots.copy_from_slice(pivots);
                best_rank = Some(rank);
                product = Natural::from(p);
                x = NaturalMatrix::from(x_mod);
            }
            // Otherwise, the pivots or row swaps are worse: reject.
        };
        x = x.crt(&product, &x_mod, p).unwrap();
        product *= Natural::from(p);
    }
}

// Solves AX = dB with the multimodular algorithm, for a matrix A of any shape, returning a solution
// X and the positive d, the least common denominator of the solution, or `None` if there is none.
//
// This is equivalent to `fmpz_mat_can_solve_multi_mod_den` from
// `fmpz_mat/can_solve_multi_mod_den.c`, FLINT 3.6.0, except that it panics if the dimensions are
// incompatible.
crate_test_fn! {
can_solve_multi_mod_den<'a>(
    a: &IntegerMatrix,
    b: impl Into<RightHandSide<'a>>,
) -> Option<(IntegerMatrix, Integer)> {
    let b = b.into();
    let x = can_solve_multi_mod(a, b)?;
    let (x, den) = fractions_to_numerators_and_denominator(&x, a.width, b.width);
    Some((x, Integer::from(den)))
}}
