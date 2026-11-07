// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::matrix::Matrix;
use malachite_base::num::number_theory::traits::{
    StirlingMatrixOfTheSecondKind, StirlingNumberOfTheSecondKind,
    UnsignedStirlingMatrixOfTheFirstKind, UnsignedStirlingNumberOfTheFirstKind,
};
use malachite_base::polynomial::Polynomial;
use malachite_base::polynomial::special::traits::{
    RisingFactorialPolynomialTruncated, TouchardPolynomialTruncated,
};
use malachite_base::test_util::generators::unsigned_pair_gen_var_28;
use malachite_nz::natural::Natural;
use malachite_nz::natural_matrix::NaturalMatrix;
use malachite_nz::natural_polynomial::NaturalPolynomial;

#[test]
fn test_unsigned_stirling_matrix_of_the_first_kind() {
    let test = |height, width, out: &str| {
        let m = NaturalMatrix::unsigned_stirling_matrix_of_the_first_kind(height, width);
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
    test(4, 4, "[1, 0, 0, 0; 0, 1, 0, 0; 0, 1, 1, 0; 0, 2, 3, 1]");
    test(
        6,
        6,
        "[1, 0, 0, 0, 0, 0; 0, 1, 0, 0, 0, 0; 0, 1, 1, 0, 0, 0; 0, 2, 3, 1, 0, 0; \
        0, 6, 11, 6, 1, 0; 0, 24, 50, 35, 10, 1]",
    );
    // - taller than wide
    test(5, 3, "[1, 0, 0; 0, 1, 0; 0, 1, 1; 0, 2, 3; 0, 6, 11]");
    test(
        8,
        2,
        "[1, 0; 0, 1; 0, 1; 0, 2; 0, 6; 0, 24; 0, 120; 0, 720]",
    );
    // - wider than tall
    test(3, 5, "[1, 0, 0, 0, 0; 0, 1, 0, 0, 0; 0, 1, 1, 0, 0]");
}

#[test]
fn test_stirling_matrix_of_the_second_kind() {
    let test = |height, width, out: &str| {
        let m = NaturalMatrix::stirling_matrix_of_the_second_kind(height, width);
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
    test(4, 4, "[1, 0, 0, 0; 0, 1, 0, 0; 0, 1, 1, 0; 0, 1, 3, 1]");
    test(
        6,
        6,
        "[1, 0, 0, 0, 0, 0; 0, 1, 0, 0, 0, 0; 0, 1, 1, 0, 0, 0; 0, 1, 3, 1, 0, 0; \
        0, 1, 7, 6, 1, 0; 0, 1, 15, 25, 10, 1]",
    );
    // - taller than wide
    test(5, 3, "[1, 0, 0; 0, 1, 0; 0, 1, 1; 0, 1, 3; 0, 1, 7]");
    test(8, 2, "[1, 0; 0, 1; 0, 1; 0, 1; 0, 1; 0, 1; 0, 1; 0, 1]");
    // - wider than tall
    test(3, 5, "[1, 0, 0, 0, 0; 0, 1, 0, 0, 0; 0, 1, 1, 0, 0]");
}

#[test]
fn stirling_matrix_properties() {
    unsigned_pair_gen_var_28::<u8, u8>().test_properties(|(height, width)| {
        let (height, width) = (u64::from(height), u64::from(width));
        let first = NaturalMatrix::unsigned_stirling_matrix_of_the_first_kind(height, width);
        let second = NaturalMatrix::stirling_matrix_of_the_second_kind(height, width);
        for m in [&first, &second] {
            assert!(m.is_valid());
            assert_eq!(m.height(), height);
            assert_eq!(m.width(), width);
            assert!(m.is_lower_triangular());
        }
        for n in 0..height {
            // Row n holds the first `width` coefficients of the rising factorial and Touchard
            // polynomials, which are the Stirling numbers of the first and second kinds.
            let rising = NaturalPolynomial::rising_factorial_polynomial_truncated(n, width);
            let touchard = NaturalPolynomial::touchard_polynomial_truncated(n, width);
            for k in 0..width {
                let (i, j) = (usize::try_from(n).unwrap(), usize::try_from(k).unwrap());
                assert_eq!(first[(i, j)], *rising.coefficient(k));
                assert_eq!(second[(i, j)], *touchard.coefficient(k));
                assert_eq!(
                    first[(i, j)],
                    Natural::unsigned_stirling_number_of_the_first_kind(n, k)
                );
                assert_eq!(
                    second[(i, j)],
                    Natural::stirling_number_of_the_second_kind(n, k)
                );
            }
        }
        // A smaller matrix is the top-left block of a larger one.
        let larger = NaturalMatrix::stirling_matrix_of_the_second_kind(height + 1, width + 2);
        for i in 0..usize::try_from(height).unwrap() {
            for j in 0..usize::try_from(width).unwrap() {
                assert_eq!(second[(i, j)], larger[(i, j)]);
            }
        }
    });
}
