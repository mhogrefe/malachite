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
use malachite_nz::integer_matrix::IntegerMatrix;
use malachite_nz::natural_matrix::NaturalMatrix;
use malachite_nz::test_util::generators::natural_matrix_unsigned_unsigned_triple_gen_var_6;
use std::panic::catch_unwind;

#[test]
fn test_minor() {
    let test = |s, i, j, out| {
        let a = NaturalMatrix::from_str(s).unwrap();
        let m = (&a).minor(i, j);
        assert_eq!(m.to_string(), out);
        assert_eq!(a.minor(i, j), m);
    };
    test("[5]", 0, 0, "1");
    test("[1, 2; 3, 4]", 0, 0, "4");
    test("[1, 2; 3, 4]", 0, 1, "3");
    test("[1, 2; 3, 4]", 1, 0, "2");
    test("[1, 2; 3, 4]", 1, 1, "1");
    test("[1, 2, 3; 4, 5, 6; 7, 8, 10]", 0, 0, "2");
    test("[1, 2, 3; 4, 5, 6; 7, 8, 10]", 0, 1, "-2");
    test("[1, 2, 3; 4, 5, 6; 7, 8, 10]", 0, 2, "-3");
    test("[1, 2, 3; 4, 5, 6; 7, 8, 10]", 1, 0, "-4");
    test(
        "[2, 0, 1, 3; 3, 1, 0, 2; 0, 4, 2, 1; 1, 1, 1, 1]",
        0,
        0,
        "5",
    );
    test(
        "[2, 0, 1, 3; 3, 1, 0, 2; 0, 4, 2, 1; 1, 1, 1, 1]",
        0,
        1,
        "-1",
    );
    test(
        "[2, 0, 1, 3; 3, 1, 0, 2; 0, 4, 2, 1; 1, 1, 1, 1]",
        0,
        2,
        "2",
    );
    test(
        "[2, 0, 1, 3; 3, 1, 0, 2; 0, 4, 2, 1; 1, 1, 1, 1]",
        0,
        3,
        "8",
    );
    test("[1, 2, 3; 2, 4, 6; 1, 0, 1]", 0, 0, "4");
    test("[1, 2, 3; 2, 4, 6; 1, 0, 1]", 0, 1, "-4");
    test("[1, 2, 3; 2, 4, 6; 1, 0, 1]", 0, 2, "-4");
    test("[1, 2, 3; 2, 4, 6; 1, 0, 1]", 1, 0, "2");
}

#[test]
fn minor_fail() {
    assert_panic!(
        NaturalMatrix::from_str("[1, 2, 3; 4, 5, 6]")
            .unwrap()
            .minor(0, 0)
    );
    assert_panic!(NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap().minor(2, 0));
    assert_panic!(NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap().minor(0, 2));
}

#[test]
fn minor_properties() {
    natural_matrix_unsigned_unsigned_triple_gen_var_6().test_properties(|(a, i, j)| {
        let m = (&a).minor(i, j);
        assert_eq!(a.clone().minor(i, j), m);
        assert_eq!(a.without_row_and_column(i, j).determinant(), m);
        assert_eq!(a.transpose().minor(j, i), m);
        assert_eq!(IntegerMatrix::from(&a).minor(i, j), m);
    });
}
