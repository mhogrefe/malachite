// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::matrix::Matrix;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_q::Rational;
use malachite_q::rational_matrix::RationalMatrix;
use malachite_q::test_util::generators::rational_matrix_gen;

// The matrix with the elements in row `i` and column `j` kept where `i == j`, and the others
// replaced with zero.
fn diagonal_part(m: &RationalMatrix) -> RationalMatrix {
    let mut xs = m.to_elements();
    let width = usize::exact_from(m.width());
    for (k, x) in xs.iter_mut().enumerate() {
        let (i, j) = (k / width, k % width);
        if !(i == j) {
            *x = Rational::ZERO;
        }
    }
    RationalMatrix::from_owned_elements(m.height(), m.width(), xs)
}

#[test]
fn test_is_diagonal() {
    let test = |s, out| {
        assert_eq!(RationalMatrix::from_str(s).unwrap().is_diagonal(), out);
    };
    test("[]_{0,0}", true);
    test("[]_{3,0}", true);
    test("[]_{0,2}", true);
    test("[0]", true);
    test("[5]", true);
    test("[0, 0; 0, 0]", true);
    test("[1, 2; 2, 1]", false);
    test("[1, 2; 3, 1]", false);
    test("[0, 2; 2, 0]", false);
    test("[1, 0; 0, 2]", true);
    test("[1, 2; 0, 3]", false);
    test("[1, 0; 2, 3]", false);
    test("[1, 2, 3; 0, 4, 5]", false);
    test("[1, 0, 0; 2, 3, 0]", false);
    test("[1, 0, 0; 0, 2, 0]", true);
    test("[1; 0; 0]", true);
    test("[0; 1]", false);
    test("[1, 2, 3; 2, 4, 5; 3, 5, 6]", false);
    test("[0, 2; -2, 0]", false);
    test("[0, -2; 2, 0]", false);
    test("[1, 2; -2, 0]", false);
    test("[0, 1, -2; -1, 0, 3; 2, -3, 0]", false);
    test("[0, 1/2; -1/2, 0]", false);
    test("[1/2, 1/3; 1/3, 1/2]", false);
}

#[test]
fn is_diagonal_properties() {
    rational_matrix_gen().test_properties(|m| {
        let diagonal = m.is_diagonal();
        assert_eq!(diagonal, m.is_upper_triangular() && m.is_lower_triangular());
        assert_eq!(m.transpose().is_diagonal(), diagonal);
        let d = diagonal_part(&m);
        assert!(d.is_diagonal());
        assert_eq!(d == m, diagonal);
    });
}
