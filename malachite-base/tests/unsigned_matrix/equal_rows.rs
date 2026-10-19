// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::matrix::Matrix;
use malachite_base::test_util::generators::unsigned_matrix_unsigned_unsigned_triple_gen_var_6;
use malachite_base::unsigned_matrix::UnsignedMatrix;

#[test]
fn test_equal_rows() {
    let test = |s, i, j, out| {
        assert_eq!(
            UnsignedMatrix::<u64>::from_str(s).unwrap().equal_rows(i, j),
            out
        );
    };
    test("[1, 7, 1; 3, 4, 3; 1, 7, 1]", 0, 2, true);
    test("[1, 7, 1; 3, 4, 3; 1, 7, 1]", 2, 0, true);
    test("[1, 7, 1; 3, 4, 3; 1, 7, 1]", 0, 1, false);
    test("[1, 7, 1; 3, 4, 3; 1, 7, 1]", 1, 1, true);
    test("[1, 7; 1, 7]", 0, 1, true);
    test("[1, 7; 7, 1]", 0, 1, false);
    test("[]_{3,0}", 0, 2, true);
    test("[5]", 0, 0, true);
}

#[test]
#[should_panic]
fn equal_rows_fail_1() {
    let _ = UnsignedMatrix::<u64>::from_str("[]_{0,0}")
        .unwrap()
        .equal_rows(0, 0);
}

#[test]
#[should_panic]
fn equal_rows_fail_2() {
    let _ = UnsignedMatrix::<u64>::from_str("[1, 7, 1; 3, 4, 3; 1, 7, 1]")
        .unwrap()
        .equal_rows(3, 0);
}

#[test]
#[should_panic]
fn equal_rows_fail_3() {
    let _ = UnsignedMatrix::<u64>::from_str("[1, 7, 1; 3, 4, 3; 1, 7, 1]")
        .unwrap()
        .equal_rows(0, 3);
}

#[test]
#[should_panic]
fn equal_rows_fail_4() {
    let _ = UnsignedMatrix::<u64>::from_str("[1, 7, 1; 3, 4, 3; 1, 7, 1]")
        .unwrap()
        .equal_rows(0, u64::MAX);
}

#[test]
#[should_panic]
fn equal_rows_fail_5() {
    let _ = UnsignedMatrix::<u64>::from_str("[]_{0,3}")
        .unwrap()
        .equal_rows(0, 0);
}

#[test]
fn equal_rows_properties() {
    unsigned_matrix_unsigned_unsigned_triple_gen_var_6().test_properties(|(mut a, i, j)| {
        let equal = a.equal_rows(i, j);
        assert_eq!(a.equal_rows(j, i), equal);
        assert_eq!(a.row(i) == a.row(j), equal);
        assert_eq!(a.transpose().equal_columns(i, j), equal);
        assert!(a.equal_rows(i, i));
        assert!(a.equal_rows(j, j));
        a.swap_rows(i, j);
        assert_eq!(a.equal_rows(i, j), equal);
        let v = a.row(i);
        a.set_row(j, &v);
        assert!(a.equal_rows(i, j));
    });
}
