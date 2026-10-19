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
use malachite_nz::test_util::generators::natural_matrix_unsigned_unsigned_triple_gen_var_3;

#[test]
fn test_swap_rows() {
    let test = |s, i, j, out| {
        let mut a = NaturalMatrix::from_str(s).unwrap();
        a.swap_rows(i, j);
        assert!(a.is_valid());
        assert_eq!(a.to_string(), out);
    };
    test(
        "[1, 2, 3; 4, 5, 6; 7, 8, 9]",
        0,
        2,
        "[7, 8, 9; 4, 5, 6; 1, 2, 3]",
    );
    test(
        "[1, 2, 3; 4, 5, 6; 7, 8, 9]",
        2,
        0,
        "[7, 8, 9; 4, 5, 6; 1, 2, 3]",
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
        "[4, 5, 6; 1, 2, 3; 7, 8, 9]",
    );
    test("[1, 2, 3, 4; 5, 6, 7, 8]", 0, 1, "[5, 6, 7, 8; 1, 2, 3, 4]");
    test("[1, 2, 3, 4; 5, 6, 7, 8]", 1, 1, "[1, 2, 3, 4; 5, 6, 7, 8]");
    test("[7]", 0, 0, "[7]");
    test("[]_{3,0}", 0, 2, "[]_{3,0}");
    test("[1; 2; 3]", 0, 2, "[3; 2; 1]");
}

#[test]
#[should_panic]
fn swap_rows_fail_1() {
    let mut a = NaturalMatrix::from_str("[]_{0,0}").unwrap();
    a.swap_rows(0, 0);
}

#[test]
#[should_panic]
fn swap_rows_fail_2() {
    let mut a = NaturalMatrix::from_str("[1, 2, 3; 4, 5, 6; 7, 8, 9]").unwrap();
    a.swap_rows(3, 0);
}

#[test]
#[should_panic]
fn swap_rows_fail_3() {
    let mut a = NaturalMatrix::from_str("[1, 2, 3; 4, 5, 6; 7, 8, 9]").unwrap();
    a.swap_rows(0, 3);
}

#[test]
#[should_panic]
fn swap_rows_fail_4() {
    let mut a = NaturalMatrix::from_str("[1, 2, 3; 4, 5, 6; 7, 8, 9]").unwrap();
    a.swap_rows(0, u64::MAX);
}

#[test]
#[should_panic]
fn swap_rows_fail_5() {
    let mut a = NaturalMatrix::from_str("[]_{0,3}").unwrap();
    a.swap_rows(0, 0);
}

#[test]
fn swap_rows_properties() {
    natural_matrix_unsigned_unsigned_triple_gen_var_3().test_properties(|(a, i, j)| {
        let mut b = a.clone();
        b.swap_rows(i, j);
        assert!(b.is_valid());
        assert_eq!((b.height(), b.width()), (a.height(), a.width()));
        for r in 0..a.height() {
            let source = if r == i {
                j
            } else if r == j {
                i
            } else {
                r
            };
            for c in 0..a.width() {
                assert_eq!(b.get(r, c), a.get(source, c));
            }
        }

        let mut c = a.clone();
        c.swap_rows(j, i);
        assert_eq!(c, b);

        c.swap_rows(i, j);
        assert_eq!(c, a);

        let mut t = a.transpose();
        t.swap_columns(i, j);
        assert_eq!(t.transpose(), b);

        if i == j {
            assert_eq!(b, a);
        }
    });
}
