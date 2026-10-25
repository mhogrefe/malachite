// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

// Contract checks for the generators of matrices with a given structure. The value tests pin down
// their outputs; these check what each one promises, such as the rank of every output modulo a
// prime, which a list of values does not show.

use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::ModIsReduced;
use malachite_base::random::EXAMPLE_SEED;
use malachite_base::test_util::unsigned_matrix::rank_mod::naive_rank_mod_prime;
use malachite_base::unsigned_matrix::UnsignedMatrix;
use malachite_base::unsigned_matrix::random::*;

// No two nonzero elements share a row or a column.
fn is_permuted_diagonal(m: &UnsignedMatrix<u8>) -> bool {
    let mut rows = Vec::new();
    let mut columns = Vec::new();
    for i in 0..m.height() {
        for (j, x) in m.row_iter(i).enumerate() {
            if *x != 0 {
                rows.push(i);
                columns.push(j);
            }
        }
    }
    let n = rows.len();
    rows.sort_unstable();
    rows.dedup();
    columns.sort_unstable();
    columns.dedup();
    rows.len() == n && columns.len() == n
}

#[test]
fn random_unsigned_matrices_with_permuted_diagonal_properties() {
    for (height, width, diagonal) in [
        (3, 4, vec![2, 3, 5]),
        (3, 3, vec![1, 4, 7]),
        (2, 5, vec![9]),
        (4, 4, vec![2, 0, 6, 1]),
        (0, 2, vec![]),
    ] {
        let diagonal: Vec<u8> = diagonal;
        let diagonal = diagonal.to_vec();
        let mut expected: Vec<_> = diagonal.iter().filter(|&x| *x != 0).copied().collect();
        expected.sort();
        for (m, _) in random_unsigned_matrices_with_permuted_diagonal(
            EXAMPLE_SEED,
            height,
            width,
            diagonal.clone(),
        )
        .take(100)
        {
            assert!(m.is_valid());
            assert_eq!((m.height(), m.width()), (height, width));
            assert!(is_permuted_diagonal(&m));
            let mut values: Vec<_> = m
                .elements_ref()
                .iter()
                .filter(|&x| *x != 0)
                .copied()
                .collect();
            values.sort();
            assert_eq!(values, expected);
        }
    }
}

#[test]
fn random_unsigned_matrices_with_rank_mod_properties() {
    // With a prime modulus, the rank is the requested one.
    for (height, width, rank, p) in [(3, 4, 2, 7), (2, 2, 2, 2), (4, 4, 0, 3), (5, 3, 3, 251)] {
        let p = u8::try_from(p).unwrap();
        for m in
            random_unsigned_matrices_with_rank_mod(EXAMPLE_SEED, height, width, rank, p).take(100)
        {
            assert!(m.is_valid());
            assert_eq!((m.height(), m.width()), (height, width));
            assert!(m.mod_is_reduced(&p));
            assert!(is_permuted_diagonal(&m));
            assert_eq!(
                m.elements_ref().iter().filter(|&x| *x != 0).count(),
                usize::try_from(rank).unwrap()
            );
            assert_eq!(naive_rank_mod_prime(&m, p), rank);
        }
    }
}

#[test]
fn random_unsigned_triangular_matrices_mod_properties() {
    for (height, width, unit, m) in
        [(3, 3, false, 7), (3, 3, true, 7), (2, 4, false, 2), (4, 2, true, 5), (0, 3, false, 1)]
    {
        let m = u8::try_from(m).unwrap();
        for (lower, xs) in [
            (
                true,
                random_unsigned_lower_triangular_matrices_mod(EXAMPLE_SEED, height, width, unit, m),
            ),
            (
                false,
                random_unsigned_upper_triangular_matrices_mod(EXAMPLE_SEED, height, width, unit, m),
            ),
        ] {
            for a in xs.take(100) {
                assert!(a.is_valid());
                assert_eq!((a.height(), a.width()), (height, width));
                assert!(a.mod_is_reduced(&m));
                if lower {
                    assert!(a.is_lower_triangular());
                } else {
                    assert!(a.is_upper_triangular());
                }
                for i in 0..height.min(width) {
                    let x = a.get(i, i).unwrap();
                    if unit {
                        assert_eq!(*x, 1);
                    } else {
                        assert_ne!(*x, 0);
                    }
                }
            }
        }
    }
}

#[test]
fn random_unsigned_matrices_from_elementary_operations_mod_properties() {
    // The operations keep the elements reduced and preserve the rank modulo a prime.
    let p = 7u8;
    for a in random_unsigned_matrices_from_elementary_operations_mod(
        EXAMPLE_SEED,
        random_unsigned_matrices_with_rank_mod(EXAMPLE_SEED.fork("rank"), 4, 5, 3, 7u8),
        20,
        p,
    )
    .take(100)
    {
        assert!(a.is_valid());
        assert_eq!((a.height(), a.width()), (4, 5));
        assert!(a.mod_is_reduced(&p));
        assert_eq!(naive_rank_mod_prime(&a, p), 3);
    }
    // A unit triangular matrix stays invertible modulo a prime.
    let p = 5u8;
    for a in random_unsigned_matrices_from_elementary_operations_mod(
        EXAMPLE_SEED,
        random_unsigned_lower_triangular_matrices_mod(EXAMPLE_SEED.fork("lower"), 4, 4, true, p),
        20,
        p,
    )
    .take(100)
    {
        assert_eq!(naive_rank_mod_prime(&a, p), 4);
    }
    // No operations leave the matrix unchanged.
    let xs = random_unsigned_matrices_with_rank_mod(EXAMPLE_SEED.fork("rank"), 3, 2, 2, 3u8);
    let ys = random_unsigned_matrices_with_rank_mod(EXAMPLE_SEED.fork("rank"), 3, 2, 2, 3u8);
    for (x, y) in random_unsigned_matrices_from_elementary_operations_mod(EXAMPLE_SEED, xs, 0, 3u8)
        .zip(ys)
        .take(100)
    {
        assert_eq!(x, y);
    }
}
