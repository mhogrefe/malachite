// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

// Contract checks for the generators of matrices with a given structure. The value tests pin down
// their outputs; these check what each one promises, such as the rank or the determinant of every
// output, which a list of values does not show.

use core::str::FromStr;
use malachite_base::matrix::Matrix;
use malachite_base::num::basic::traits::One;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::random::EXAMPLE_SEED;
use malachite_nz::integer::Integer;
use malachite_nz::integer_matrix::IntegerMatrix;
use malachite_nz::integer_matrix::random::{
    random_integer_matrices_from_elementary_operations, random_integer_matrices_with_determinant,
    random_integer_matrices_with_dimensions, random_integer_matrices_with_permuted_diagonal,
    random_integer_matrices_with_rank,
};
use malachite_nz::test_util::integer_matrix::determinant::naive_determinant;
use malachite_nz::test_util::integer_matrix::rank::naive_rank;

// The positions and values of the nonzero elements, in row-major order.
fn nonzero_elements(m: &IntegerMatrix) -> Vec<(u64, u64, Integer)> {
    let mut out = Vec::new();
    for i in 0..m.height() {
        for (j, x) in m.row_iter(i).enumerate() {
            if *x != 0u32 {
                out.push((i, u64::try_from(j).unwrap(), x.clone()));
            }
        }
    }
    out
}

// No two nonzero elements share a row or a column.
fn is_permuted_diagonal(m: &IntegerMatrix) -> bool {
    let nonzero = nonzero_elements(m);
    let mut rows: Vec<u64> = nonzero.iter().map(|t| t.0).collect();
    let mut columns: Vec<u64> = nonzero.iter().map(|t| t.1).collect();
    rows.sort_unstable();
    rows.dedup();
    columns.sort_unstable();
    columns.dedup();
    rows.len() == nonzero.len() && columns.len() == nonzero.len()
}

#[test]
fn random_integer_matrices_with_permuted_diagonal_properties() {
    for (height, width, diagonal) in [
        (3, 4, vec!["2", "-3", "5"]),
        (3, 3, vec!["-1", "4", "7"]),
        (2, 5, vec!["9"]),
        (4, 4, vec!["2", "0", "-6", "1"]),
        (0, 2, vec![]),
    ] {
        let diagonal: Vec<Integer> = diagonal
            .into_iter()
            .map(|x| Integer::from_str(x).unwrap())
            .collect();
        let mut expected: Vec<Integer> = diagonal.iter().filter(|&x| *x != 0u32).cloned().collect();
        expected.sort();
        for (m, odd) in random_integer_matrices_with_permuted_diagonal(
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
            let mut values: Vec<Integer> = nonzero_elements(&m).into_iter().map(|t| t.2).collect();
            values.sort();
            assert_eq!(values, expected);
            if height == width && u64::try_from(diagonal.len()).unwrap() == height {
                let product: Integer = diagonal.iter().product();
                assert_eq!(naive_determinant(&m), if odd { -product } else { product });
            }
        }
    }
}

#[test]
fn random_integer_matrices_with_rank_properties() {
    for (height, width, rank, bits) in [(3, 4, 2, 3), (2, 2, 2, 1), (4, 4, 0, 5), (5, 3, 3, 70)] {
        for m in
            random_integer_matrices_with_rank(EXAMPLE_SEED, height, width, rank, bits).take(100)
        {
            assert!(m.is_valid());
            assert_eq!((m.height(), m.width()), (height, width));
            assert!(is_permuted_diagonal(&m));
            let nonzero = nonzero_elements(&m);
            assert_eq!(u64::try_from(nonzero.len()).unwrap(), rank);
            assert!(nonzero.iter().all(|t| t.2.significant_bits() == bits));
            assert_eq!(naive_rank(&m), rank);
        }
    }
}

#[test]
fn random_integer_matrices_with_determinant_properties() {
    for (size, determinant) in [
        (3, "-60"),
        (2, "1"),
        (4, "0"),
        (1, "13"),
        (0, "1"),
        (4, "-1024"),
        (3, "18446744073709551615"),
        (5, "-30030"),
    ] {
        let determinant = Integer::from_str(determinant).unwrap();
        for m in
            random_integer_matrices_with_determinant(EXAMPLE_SEED, size, &determinant).take(100)
        {
            assert!(m.is_valid());
            assert_eq!((m.height(), m.width()), (size, size));
            assert!(is_permuted_diagonal(&m));
            assert_eq!(naive_determinant(&m), determinant);
        }
    }
}

#[test]
fn random_integer_matrices_from_elementary_operations_properties() {
    // The operations preserve the rank.
    for m in random_integer_matrices_from_elementary_operations(
        EXAMPLE_SEED,
        random_integer_matrices_with_rank(EXAMPLE_SEED.fork("rank"), 4, 5, 3, 4),
        20,
    )
    .take(100)
    {
        assert!(m.is_valid());
        assert_eq!((m.height(), m.width()), (4, 5));
        assert_eq!(naive_rank(&m), 3);
    }
    // They preserve the determinant.
    let determinant = Integer::from_str("-210").unwrap();
    for m in random_integer_matrices_from_elementary_operations(
        EXAMPLE_SEED,
        random_integer_matrices_with_determinant(EXAMPLE_SEED.fork("determinant"), 4, &determinant),
        20,
    )
    .take(100)
    {
        assert_eq!(naive_determinant(&m), determinant);
    }
    // No operations, or a matrix with no elements, leave the matrix unchanged.
    let xs = random_integer_matrices_with_dimensions(EXAMPLE_SEED.fork("dimensions"), 3, 2, 4, 1);
    let ys = random_integer_matrices_with_dimensions(EXAMPLE_SEED.fork("dimensions"), 3, 2, 4, 1);
    for (x, y) in random_integer_matrices_from_elementary_operations(EXAMPLE_SEED, xs, 0)
        .zip(ys)
        .take(100)
    {
        assert_eq!(x, y);
    }
    for m in random_integer_matrices_from_elementary_operations(
        EXAMPLE_SEED,
        random_integer_matrices_with_dimensions(EXAMPLE_SEED.fork("dimensions"), 0, 3, 4, 1),
        5,
    )
    .take(10)
    {
        assert_eq!(m, IntegerMatrix::zero(0, 3));
    }
    // The identity stays unimodular.
    for m in random_integer_matrices_from_elementary_operations(
        EXAMPLE_SEED,
        core::iter::repeat(IntegerMatrix::identity(3)),
        30,
    )
    .take(100)
    {
        assert_eq!(naive_determinant(&m), Integer::ONE);
    }
}
