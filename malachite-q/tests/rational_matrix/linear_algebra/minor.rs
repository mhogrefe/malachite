// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::matrix::Matrix;
use malachite_base::num::linear_algebra::traits::{Determinant, Minor};
use malachite_q::rational_matrix::RationalMatrix;
use malachite_q::test_util::generators::rational_matrix_unsigned_unsigned_triple_gen_var_4;
use std::panic::catch_unwind;

#[test]
fn test_minor() {
    let test = |s, i, j, out| {
        let a = RationalMatrix::from_str(s).unwrap();
        let m = (&a).minor(i, j);
        assert_eq!(m.to_string(), out);
        assert_eq!(a.minor(i, j), m);
    };
    test("[1/2]", 0, 0, "1");
    test("[1/2, 1/3; 1/4, 1/5]", 0, 0, "1/5");
    test("[1/2, 1/3; 1/4, 1/5]", 0, 1, "1/4");
    test("[1/2, 1/3; 1/4, 1/5]", 1, 0, "1/3");
    test("[1/2, 1/3; 1/4, 1/5]", 1, 1, "1/2");
    test("[1/2, -2, 3; 4, 5/3, -6; 7, -8, 9]", 0, 0, "-33");
    test("[1/2, -2, 3; 4, 5/3, -6; 7, -8, 9]", 0, 1, "78");
    test("[1/2, -2, 3; 4, 5/3, -6; 7, -8, 9]", 0, 2, "-131/3");
    test("[1/2, -2, 3; 4, 5/3, -6; 7, -8, 9]", 1, 0, "6");
    test("[1, 2, 3; 2, 4, 6; 1/2, 0, 1]", 0, 0, "4");
    test("[1, 2, 3; 2, 4, 6; 1/2, 0, 1]", 0, 1, "-1");
    test("[1, 2, 3; 2, 4, 6; 1/2, 0, 1]", 0, 2, "-2");
    test("[1, 2, 3; 2, 4, 6; 1/2, 0, 1]", 1, 0, "2");
}

#[test]
fn minor_fail() {
    assert_panic!(
        RationalMatrix::from_str("[1, 2, 3; 4, 5, 6]")
            .unwrap()
            .minor(0, 0)
    );
    assert_panic!(
        RationalMatrix::from_str("[1, 2; 3, 4]")
            .unwrap()
            .minor(2, 0)
    );
    assert_panic!(
        RationalMatrix::from_str("[1, 2; 3, 4]")
            .unwrap()
            .minor(0, 2)
    );
}

#[test]
fn minor_properties() {
    rational_matrix_unsigned_unsigned_triple_gen_var_4().test_properties(|(a, i, j)| {
        let m = (&a).minor(i, j);
        assert_eq!(a.clone().minor(i, j), m);
        assert_eq!(a.without_row_and_column(i, j).determinant(), m);
        assert_eq!(a.transpose().minor(j, i), m);
    });
}
