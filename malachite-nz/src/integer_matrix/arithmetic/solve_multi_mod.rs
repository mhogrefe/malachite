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
use crate::integer_matrix::arithmetic::fraction_matrix::*;
use crate::integer_matrix::arithmetic::mul::multi_mod::next_prime;
use crate::integer_matrix::arithmetic::solve_bounds::solve_bounds_helper;
use crate::integer_matrix::arithmetic::solve_dixon::{find_good_prime, reconstruction_bound};
use crate::integer_matrix::arithmetic::word_lu::WordLu;
use crate::natural::Natural;
use crate::natural_matrix::NaturalMatrix;
use alloc::vec;
use alloc::vec::Vec;
use malachite_base::num::arithmetic::traits::Crt;

// Solves AX = B for a square matrix A with the multimodular algorithm, returning the elements of X,
// or `None` if A is singular. The system is solved modulo primes for which A is invertible, and the
// solutions are combined by the Chinese remainder theorem until rational reconstruction recovers X;
// every few primes, the reconstruction is attempted and checked, so that the algorithm may stop
// early.
//
// This is equivalent to `fmpq_mat_solve_fmpz_mat_multi_mod` from `fmpq_mat/solve_multi_mod.c`,
// FLINT 3.6.0, except that it panics if the dimensions are incompatible.
#[doc(hidden)]
pub fn solve_multi_mod<'a>(
    a: &IntegerMatrix,
    b: impl Into<RightHandSide<'a>>,
) -> Option<Vec<Fraction>> {
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
    let (x, mut p) = find_good_prime(a, &den_bound, |lu| lu.solve_square(&b.mod_word(lu.p)))?;
    let bound = reconstruction_bound(num_bound, den_bound);
    let mut product = Natural::from(p);
    let mut x = NaturalMatrix::from(x);
    // The number of primes used, and the number at which the next check is made.
    let mut i = 1;
    let mut next_i = 1;
    while product <= bound {
        if i == next_i {
            next_i = next_check_iteration(i);
            if let Some(xs) = reconstruct_fractions(x.elements.iter(), &product)
                && is_solution(&xs, n, k, a, b)
            {
                return Some(xs);
            }
        }
        i += 1;
        let x_mod = loop {
            p = next_prime(p);
            let lu = WordLu::new(a, p);
            if lu.rank == n {
                break lu.solve_square(&b.mod_word(p));
            }
        };
        x = x.crt(&product, &x_mod, p).unwrap();
        product *= Natural::from(p);
    }
    Some(reconstruct_fractions(x.elements.iter(), &product).unwrap())
}

// Solves AX = dB for a square matrix A with the multimodular algorithm, returning X and the
// positive d, the least common denominator of the solution, or `None` if A is singular.
//
// This is equivalent to `fmpz_mat_solve_multi_mod_den` from `fmpz_mat/solve_multi_mod_den.c`, FLINT
// 3.6.0.
crate_test_fn! {
solve_multi_mod_den<'a>(
    a: &IntegerMatrix,
    b: impl Into<RightHandSide<'a>>,
) -> Option<(IntegerMatrix, Integer)> {
    let b = b.into();
    let x = solve_multi_mod(a, b)?;
    let (x, den) = fractions_to_numerators_and_denominator(&x, a.width, b.width);
    Some((x, Integer::from(den)))
}}
