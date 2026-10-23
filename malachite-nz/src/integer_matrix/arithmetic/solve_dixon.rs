// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2011, 2019, 2026 Fredrik Johansson
//
//      Copyright © 2019 William Hart
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::integer_matrix::IntegerMatrix;
use crate::integer_matrix::arithmetic::add_mul_unsigned_matrix::add_mul_unsigned_matrix_assign;
use crate::integer_matrix::arithmetic::certify_lu_mod_p::rank_certify_lu_mod_p;
use crate::integer_matrix::arithmetic::fraction_matrix::*;
use crate::integer_matrix::arithmetic::solve_bounds::solve_bounds_helper;
use crate::integer_matrix::arithmetic::mul::multi_mod::{OPTIMAL_MODULUS_MIN, next_prime};
use crate::integer_matrix::arithmetic::word_lu::WordLu;
use crate::natural::Natural;
use crate::platform::Limb;
use alloc::vec;
use alloc::vec::Vec;
use malachite_base::num::arithmetic::traits::{DivExactAssign, Mod, ModMul, Square};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::unsigned_matrix::UnsignedMatrix;

// Finds a prime p modulo which the square matrix `a` is invertible, returning `full_rank` applied
// to the LU decomposition of `a` modulo p, and p; or `None` if `a` is found to be singular: either
// once the product of the primes tried exceeds `det_bound`, a bound for the absolute value of its
// determinant, or once a rank deficiency modulo a prime is certified to hold over the integers. The
// primes tried are those just above $2^{W-5}$, where $W$ is the limb width.
//
// This is equivalent to `fmpz_mat_find_good_prime_and_invert` from `fmpz_mat/solve_dixon.c`, FLINT
// 3.6.0, when `full_rank` inverts the matrix, and to `fmpz_mat_find_good_prime_and_solve` from
// `fmpq_mat/solve_multi_mod.c` when it solves a system.
pub(crate) fn find_good_prime<T>(
    a: &IntegerMatrix,
    det_bound: &Natural,
    full_rank: impl Fn(&WordLu) -> T,
) -> Option<(T, Limb)> {
    let n = a.height;
    let mut tested = Natural::ONE;
    let mut p = OPTIMAL_MODULUS_MIN;
    loop {
        p = next_prime(p);
        let lu = WordLu::new(a, p);
        if lu.rank == n {
            return Some((full_rank(&lu), p));
        }
        // Enough primes to certify the rank deficiency.
        tested *= Natural::from(p);
        if tested > *det_bound {
            return None;
        }
        // The matrix is rank-deficient modulo p; try to certify that it is rank-deficient over the
        // integers, hence singular.
        if rank_certify_lu_mod_p(a, lu.rank, &lu.perm, &lu.pivots_nonpivots) {
            return None;
        }
    }
}

// The bound that the modulus must exceed for a solution with numerators at most `num_bound` and
// denominators at most `den_bound` in absolute value to be recovered by rational reconstruction:
// twice the square of the larger bound.
pub(crate) fn reconstruction_bound(num_bound: Natural, den_bound: Natural) -> Natural {
    (if num_bound < den_bound {
        den_bound
    } else {
        num_bound
    })
    .square()
        << 1u32
}

// Solves AX = B by Dixon's p-adic lifting, given the inverse `a_inverse` of A modulo the prime `p`
// and bounds for the numerators and denominators of the solution, returning the elements of X.
// Every few iterations, the solution modulo the current power of p is reconstructed and checked, so
// that the lifting may stop early.
//
// This is equivalent to `_fmpq_mat_solve_dixon` from `fmpq_mat/solve_dixon.c`, FLINT 3.6.0, which
// computes the products Ay directly. FLINT's `_fmpz_mat_solve_dixon` instead computes them modulo
// several word-sized primes and reconstructs them by the Chinese remainder theorem; with
// Malachite's matrix multiplication, that was found to be about twice as slow.
fn solve_dixon_with_inverse(
    a: &IntegerMatrix,
    b: RightHandSide<'_>,
    a_inverse: &UnsignedMatrix<Limb>,
    p: Limb,
    (num_bound, den_bound): (Natural, Natural),
) -> Vec<Fraction> {
    let (n, k) = (a.height, b.width);
    let bound = reconstruction_bound(num_bound, den_bound);
    let p_natural = Natural::from(p);
    let p_integer = Integer::from(p);
    let mut x = IntegerMatrix {
        elements: vec![Integer::ZERO; n * k],
        height: n,
        width: k,
    };
    let mut d = b.to_matrix();
    let mut p_power = Natural::ONE;
    // The current iteration, and the iteration of the next check.
    let mut i = 1;
    let mut next_i = 1;
    loop {
        // y = A^(-1) d mod p
        let y = a_inverse.mod_mul((&d).mod_op(p), p);
        // x = x + y p^i, which is A^(-1) B mod p^(i + 1)
        add_mul_unsigned_matrix_assign(&mut x, &y, &Integer::from(&p_power));
        p_power *= &p_natural;
        if p_power > bound {
            break;
        }
        if i == next_i {
            next_i = next_check_iteration(i);
            if let Some(xs) =
                reconstruct_fractions(x.elements.iter().map(Integer::unsigned_abs_ref), &p_power)
                && is_solution(&xs, n, k, a, b)
            {
                return xs;
            }
        }
        i += 1;
        // d = (d - Ay) / p
        d -= a * IntegerMatrix::from(y);
        d.div_exact_assign(&p_integer);
    }
    reconstruct_fractions(x.elements.iter().map(Integer::unsigned_abs_ref), &p_power).unwrap()
}

// Solves AX = B for a square matrix A by Dixon's p-adic lifting, returning the elements of X, or
// `None` if A is singular.
//
// This is equivalent to `fmpq_mat_solve_fmpz_mat_dixon` from `fmpq_mat/solve_dixon.c`, FLINT 3.6.0,
// except that it panics if the dimensions are incompatible.
crate_test_fn! {
solve_dixon<'a>(a: &IntegerMatrix, b: impl Into<RightHandSide<'a>>) -> Option<Vec<Fraction>> {
    let b = b.into();
    assert_eq!(a.height, a.width, "the matrix must be square");
    assert_eq!(
        a.height, b.height,
        "the height of the matrix must equal the height of the right-hand side"
    );
    let (n, k) = (a.height, b.width);
    if n == 0 || k == 0 {
        return Some(vec![Fraction::ZERO; n * k]);
    }
    let (num_bound, den_bound) = solve_bounds_helper(a, b);
    let (a_inverse, p) = find_good_prime(a, &den_bound, WordLu::inverse)?;
    Some(solve_dixon_with_inverse(
        a,
        b,
        &a_inverse,
        p,
        (num_bound, den_bound),
    ))
}}

// Solves AX = dB for a square matrix A by Dixon's p-adic lifting, returning X and the positive d,
// the least common denominator of the solution, or `None` if A is singular.
//
// This is equivalent to `fmpz_mat_solve_dixon_den` from `fmpz_mat/solve_dixon_den.c`, FLINT 3.6.0.
crate_test_fn! {
solve_dixon_den<'a>(
    a: &IntegerMatrix,
    b: impl Into<RightHandSide<'a>>,
) -> Option<(IntegerMatrix, Integer)> {
    let b = b.into();
    let x = solve_dixon(a, b)?;
    let (x, den) = fractions_to_numerators_and_denominator(&x, a.width, b.width);
    Some((x, Integer::from(den)))
}}
