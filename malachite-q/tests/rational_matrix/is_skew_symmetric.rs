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
use malachite_q::test_util::generators::{rational_matrix_gen, rational_matrix_gen_var_1};

// The negative of a matrix, element by element.
fn negative(m: &RationalMatrix) -> RationalMatrix {
    RationalMatrix::from_owned_elements(
        m.height(),
        m.width(),
        m.elements.iter().map(|x| -x).collect(),
    )
}

// The square matrix with the negatives of the elements above the diagonal copied onto the ones
// below it, and zeros on the diagonal.
fn skew_symmetrized(m: &RationalMatrix) -> RationalMatrix {
    let mut n = m.clone();
    let size = usize::exact_from(m.height());
    for i in 0..size {
        n[(i, i)] = Rational::ZERO;
        for j in 0..i {
            n[(i, j)] = -&m[(j, i)];
        }
    }
    n
}

#[test]
fn test_is_skew_symmetric() {
    let test = |s, out| {
        assert_eq!(
            RationalMatrix::from_str(s).unwrap().is_skew_symmetric(),
            out
        );
    };
    test("[]_{0,0}", true);
    test("[]_{3,0}", false);
    test("[]_{0,2}", false);
    test("[0]", true);
    test("[5]", false);
    test("[0, 0; 0, 0]", true);
    test("[1, 2; 2, 1]", false);
    test("[1, 2; 3, 1]", false);
    test("[0, 2; 2, 0]", false);
    test("[1, 0; 0, 2]", false);
    test("[1, 2; 0, 3]", false);
    test("[1, 0; 2, 3]", false);
    test("[1, 2, 3; 0, 4, 5]", false);
    test("[1, 0, 0; 2, 3, 0]", false);
    test("[1, 0, 0; 0, 2, 0]", false);
    test("[1; 0; 0]", false);
    test("[0; 1]", false);
    test("[1, 2, 3; 2, 4, 5; 3, 5, 6]", false);
    test("[0, 2; -2, 0]", true);
    test("[0, -2; 2, 0]", true);
    test("[1, 2; -2, 0]", false);
    test("[0, 1, -2; -1, 0, 3; 2, -3, 0]", true);
    test("[0, 1/2; -1/2, 0]", true);
    test("[1/2, 1/3; 1/3, 1/2]", false);
}

#[test]
fn is_skew_symmetric_properties() {
    rational_matrix_gen().test_properties(|m| {
        let skew = m.is_skew_symmetric();
        assert_eq!(skew, m.is_square() && m.transpose() == negative(&m));
        assert_eq!(m.transpose().is_skew_symmetric(), skew);
        assert_eq!(negative(&m).is_skew_symmetric(), skew);
        // Only the square zero matrices are both symmetric and skew-symmetric.
        assert_eq!(skew && m.is_symmetric(), m.is_square() && m.is_zero());
    });

    rational_matrix_gen_var_1().test_properties(|m| {
        let s = skew_symmetrized(&m);
        assert!(s.is_skew_symmetric());
        assert_eq!(s == m, m.is_skew_symmetric());
    });
}
