// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::matrix::Matrix;
use malachite_nz::natural_matrix::NaturalMatrix;
use malachite_nz::test_util::generators::natural_matrix_unsigned_unsigned_triple_gen_var_4;

#[test]
fn test_equal_columns() {
    let test = |s, i, j, out| {
        assert_eq!(NaturalMatrix::from_str(s).unwrap().equal_columns(i, j), out);
    };
    test("[1, 7, 1; 3, 4, 3; 1, 7, 1]", 0, 2, true);
    test("[1, 7, 1; 3, 4, 3; 1, 7, 1]", 2, 0, true);
    test("[1, 7, 1; 3, 4, 3; 1, 7, 1]", 0, 1, false);
    test("[1, 7, 1; 3, 4, 3; 1, 7, 1]", 1, 1, true);
    test("[1, 1; 7, 7]", 0, 1, true);
    test("[1, 7; 7, 1]", 0, 1, false);
    test("[]_{0,3}", 0, 2, true);
    test("[5]", 0, 0, true);
}

#[test]
#[should_panic]
fn equal_columns_fail_1() {
    let _ = NaturalMatrix::from_str("[]_{0,0}")
        .unwrap()
        .equal_columns(0, 0);
}

#[test]
#[should_panic]
fn equal_columns_fail_2() {
    let _ = NaturalMatrix::from_str("[1, 7, 1; 3, 4, 3; 1, 7, 1]")
        .unwrap()
        .equal_columns(3, 0);
}

#[test]
#[should_panic]
fn equal_columns_fail_3() {
    let _ = NaturalMatrix::from_str("[1, 7, 1; 3, 4, 3; 1, 7, 1]")
        .unwrap()
        .equal_columns(0, 3);
}

#[test]
#[should_panic]
fn equal_columns_fail_4() {
    let _ = NaturalMatrix::from_str("[1, 7, 1; 3, 4, 3; 1, 7, 1]")
        .unwrap()
        .equal_columns(0, u64::MAX);
}

#[test]
#[should_panic]
fn equal_columns_fail_5() {
    let _ = NaturalMatrix::from_str("[]_{3,0}")
        .unwrap()
        .equal_columns(0, 0);
}

#[test]
fn equal_columns_properties() {
    natural_matrix_unsigned_unsigned_triple_gen_var_4().test_properties(|(mut a, i, j)| {
        let equal = a.equal_columns(i, j);
        assert_eq!(a.equal_columns(j, i), equal);
        assert_eq!(a.column(i) == a.column(j), equal);
        assert_eq!(a.transpose().equal_rows(i, j), equal);
        assert!(a.equal_columns(i, i));
        assert!(a.equal_columns(j, j));
        a.swap_columns(i, j);
        assert_eq!(a.equal_columns(i, j), equal);
        let v = a.column(i);
        a.set_column(j, &v);
        assert!(a.equal_columns(i, j));
    });
}
