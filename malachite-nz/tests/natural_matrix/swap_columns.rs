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
fn test_swap_columns() {
    let test = |s, i, j, out| {
        let mut a = NaturalMatrix::from_str(s).unwrap();
        a.swap_columns(i, j);
        assert!(a.is_valid());
        assert_eq!(a.to_string(), out);
    };
    test(
        "[1, 2, 3; 4, 5, 6; 7, 8, 9]",
        0,
        2,
        "[3, 2, 1; 6, 5, 4; 9, 8, 7]",
    );
    test(
        "[1, 2, 3; 4, 5, 6; 7, 8, 9]",
        2,
        0,
        "[3, 2, 1; 6, 5, 4; 9, 8, 7]",
    );
    test(
        "[1, 2, 3; 4, 5, 6; 7, 8, 9]",
        1,
        1,
        "[1, 2, 3; 4, 5, 6; 7, 8, 9]",
    );
    test(
        "[1, 2, 3; 4, 5, 6; 7, 8, 9]",
        0,
        1,
        "[2, 1, 3; 5, 4, 6; 8, 7, 9]",
    );
    test("[1, 2, 3, 4; 5, 6, 7, 8]", 1, 3, "[1, 4, 3, 2; 5, 8, 7, 6]");
    test("[1, 2, 3, 4; 5, 6, 7, 8]", 2, 2, "[1, 2, 3, 4; 5, 6, 7, 8]");
    test("[7]", 0, 0, "[7]");
    test("[]_{0,3}", 0, 2, "[]_{0,3}");
    test("[1, 2, 3]", 0, 2, "[3, 2, 1]");
}

#[test]
#[should_panic]
fn swap_columns_fail_1() {
    let mut a = NaturalMatrix::from_str("[]_{0,0}").unwrap();
    a.swap_columns(0, 0);
}

#[test]
#[should_panic]
fn swap_columns_fail_2() {
    let mut a = NaturalMatrix::from_str("[1, 2, 3; 4, 5, 6; 7, 8, 9]").unwrap();
    a.swap_columns(3, 0);
}

#[test]
#[should_panic]
fn swap_columns_fail_3() {
    let mut a = NaturalMatrix::from_str("[1, 2, 3; 4, 5, 6; 7, 8, 9]").unwrap();
    a.swap_columns(0, 3);
}

#[test]
#[should_panic]
fn swap_columns_fail_4() {
    let mut a = NaturalMatrix::from_str("[1, 2, 3; 4, 5, 6; 7, 8, 9]").unwrap();
    a.swap_columns(0, u64::MAX);
}

#[test]
#[should_panic]
fn swap_columns_fail_5() {
    let mut a = NaturalMatrix::from_str("[]_{3,0}").unwrap();
    a.swap_columns(0, 0);
}

#[test]
fn swap_columns_properties() {
    natural_matrix_unsigned_unsigned_triple_gen_var_4().test_properties(|(a, i, j)| {
        let mut b = a.clone();
        b.swap_columns(i, j);
        assert!(b.is_valid());
        assert_eq!((b.height(), b.width()), (a.height(), a.width()));
        for c in 0..a.width() {
            let source = if c == i {
                j
            } else if c == j {
                i
            } else {
                c
            };
            for r in 0..a.height() {
                assert_eq!(b.get(r, c), a.get(r, source));
            }
        }

        let mut c = a.clone();
        c.swap_columns(j, i);
        assert_eq!(c, b);

        c.swap_columns(i, j);
        assert_eq!(c, a);

        let mut t = a.transpose();
        t.swap_rows(i, j);
        assert_eq!(t.transpose(), b);

        if i == j {
            assert_eq!(b, a);
        }
    });
}
