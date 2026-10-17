// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::matrix::Matrix;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_q::rational_matrix::RationalMatrix;
use malachite_q::test_util::generators::{rational_matrix_gen, rational_matrix_gen_var_1};

// The square matrix with the elements above the diagonal copied onto the ones below it.
fn symmetrized(m: &RationalMatrix) -> RationalMatrix {
    let mut n = m.clone();
    let size = usize::exact_from(m.height());
    for i in 0..size {
        for j in 0..i {
            n[(i, j)] = m[(j, i)].clone();
        }
    }
    n
}

#[test]
fn test_is_symmetric() {
    let test = |s, out| {
        assert_eq!(RationalMatrix::from_str(s).unwrap().is_symmetric(), out);
    };
    test("[]_{0,0}", true);
    test("[]_{3,0}", false);
    test("[]_{0,2}", false);
    test("[0]", true);
    test("[5]", true);
    test("[0, 0; 0, 0]", true);
    test("[1, 2; 2, 1]", true);
    test("[1, 2; 3, 1]", false);
    test("[0, 2; 2, 0]", true);
    test("[1, 0; 0, 2]", true);
    test("[1, 2; 0, 3]", false);
    test("[1, 0; 2, 3]", false);
    test("[1, 2, 3; 0, 4, 5]", false);
    test("[1, 0, 0; 2, 3, 0]", false);
    test("[1, 0, 0; 0, 2, 0]", false);
    test("[1; 0; 0]", false);
    test("[0; 1]", false);
    test("[1, 2, 3; 2, 4, 5; 3, 5, 6]", true);
    test("[0, 2; -2, 0]", false);
    test("[0, -2; 2, 0]", false);
    test("[1, 2; -2, 0]", false);
    test("[0, 1, -2; -1, 0, 3; 2, -3, 0]", false);
    test("[0, 1/2; -1/2, 0]", false);
    test("[1/2, 1/3; 1/3, 1/2]", true);
}

#[test]
fn is_symmetric_properties() {
    rational_matrix_gen().test_properties(|m| {
        let symmetric = m.is_symmetric();
        assert_eq!(symmetric, m.is_square() && m.transpose() == m);
        assert_eq!(m.transpose().is_symmetric(), symmetric);
        if m.is_square() && m.is_diagonal() {
            assert!(symmetric);
        }
    });

    rational_matrix_gen_var_1().test_properties(|m| {
        let s = symmetrized(&m);
        assert!(s.is_symmetric());
        assert_eq!(s == m, m.is_symmetric());
    });
}
