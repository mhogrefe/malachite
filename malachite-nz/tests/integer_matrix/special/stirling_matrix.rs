// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::{Parity, UnsignedAbs};
use malachite_base::num::number_theory::traits::{
    StirlingMatrixOfTheFirstKind, StirlingMatrixOfTheSecondKind, StirlingNumberOfTheFirstKind,
    UnsignedStirlingMatrixOfTheFirstKind,
};
use malachite_base::polynomial::Polynomial;
use malachite_base::polynomial::special::traits::FallingFactorialPolynomialTruncated;
use malachite_base::test_util::generators::unsigned_pair_gen_var_28;
use malachite_nz::integer::Integer;
use malachite_nz::integer_matrix::IntegerMatrix;
use malachite_nz::integer_polynomial::IntegerPolynomial;
use malachite_nz::natural_matrix::NaturalMatrix;

#[test]
fn test_stirling_matrix_of_the_first_kind() {
    let test = |height, width, out: &str| {
        let m = IntegerMatrix::stirling_matrix_of_the_first_kind(height, width);
        assert!(m.is_valid());
        assert_eq!(m.to_string(), out);
    };
    // (expected values computed independently with python)
    // - empty matrices
    test(0, 0, "[]_{0,0}");
    test(0, 3, "[]_{0,3}");
    test(3, 0, "[]_{3,0}");
    test(1, 1, "[1]");
    test(1, 4, "[1, 0, 0, 0]");
    // - square
    test(4, 4, "[1, 0, 0, 0; 0, 1, 0, 0; 0, -1, 1, 0; 0, 2, -3, 1]");
    test(
        6,
        6,
        "[1, 0, 0, 0, 0, 0; 0, 1, 0, 0, 0, 0; 0, -1, 1, 0, 0, 0; 0, 2, -3, 1, 0, 0; \
        0, -6, 11, -6, 1, 0; 0, 24, -50, 35, -10, 1]",
    );
    // - taller than wide
    test(5, 3, "[1, 0, 0; 0, 1, 0; 0, -1, 1; 0, 2, -3; 0, -6, 11]");
    test(
        8,
        2,
        "[1, 0; 0, 1; 0, -1; 0, 2; 0, -6; 0, 24; 0, -120; 0, 720]",
    );
    // - wider than tall
    test(3, 5, "[1, 0, 0, 0, 0; 0, 1, 0, 0, 0; 0, -1, 1, 0, 0]");
}

#[test]
fn stirling_matrix_of_the_first_kind_properties() {
    unsigned_pair_gen_var_28::<u8, u8>().test_properties(|(height, width)| {
        let (height, width) = (u64::from(height), u64::from(width));
        let m = IntegerMatrix::stirling_matrix_of_the_first_kind(height, width);
        assert!(m.is_valid());
        assert_eq!(m.height(), height);
        assert_eq!(m.width(), width);
        assert!(m.is_lower_triangular());
        let unsigned = NaturalMatrix::unsigned_stirling_matrix_of_the_first_kind(height, width);
        for n in 0..height {
            // Row n holds the first `width` coefficients of the falling factorial polynomial.
            let falling = IntegerPolynomial::falling_factorial_polynomial_truncated(n, width);
            for k in 0..width {
                let (i, j) = (usize::try_from(n).unwrap(), usize::try_from(k).unwrap());
                let x = &m[(i, j)];
                assert_eq!(*x, *falling.coefficient(k));
                assert_eq!(*x, Integer::stirling_number_of_the_first_kind(n, k));
                // the entries are the unsigned ones, with signs (-1)^(n + k)
                assert_eq!(x.unsigned_abs(), unsigned[(i, j)]);
                if *x != 0u32 {
                    assert_eq!(*x > 0u32, n.even() == k.even());
                }
            }
        }
    });
}

#[test]
fn test_stirling_matrices_inverse() {
    // The square matrices of the first and second kinds are inverses of each other. Every size up
    // to 60 is checked; the products of larger ones take long to compute.
    for n in 0..=60 {
        let first = IntegerMatrix::stirling_matrix_of_the_first_kind(n, n);
        let second = IntegerMatrix::from(NaturalMatrix::stirling_matrix_of_the_second_kind(n, n));
        assert_eq!(&first * &second, IntegerMatrix::identity(n));
        assert_eq!(&second * &first, IntegerMatrix::identity(n));
    }
}
