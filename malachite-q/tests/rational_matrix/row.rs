// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_q::rational_matrix::RationalMatrix;
use malachite_q::test_util::generators::{
    rational_matrix_gen, rational_matrix_unsigned_pair_gen_var_1,
};

#[test]
fn test_row() {
    let test = |s, i: usize, out| {
        let m = RationalMatrix::from_str(s).unwrap();
        let v = m.row(i);
        assert_eq!(v.to_string(), out);
    };
    test("[]_{3,0}", 0, "()");
    test("[]_{3,0}", 1, "()");
    test("[]_{3,0}", 2, "()");
    test("[5]", 0, "(5)");
    test("[1, 2, 3; 4, 5, 6]", 0, "(1, 2, 3)");
    test("[1, 2, 3; 4, 5, 6]", 1, "(4, 5, 6)");
    test("[1, 2; 3, 4; 5, 6]", 0, "(1, 2)");
    test("[1, 2; 3, 4; 5, 6]", 1, "(3, 4)");
    test("[1, 2; 3, 4; 5, 6]", 2, "(5, 6)");
    test("[1/2, -2; 3, 4/3]", 0, "(1/2, -2)");
    test("[1/2, -2; 3, 4/3]", 1, "(3, 4/3)");
}

#[test]
#[should_panic]
fn row_fail_1() {
    let m = RationalMatrix::from_str("[]_{0,0}").unwrap();
    let _ = m.row(0);
}

#[test]
#[should_panic]
fn row_fail_2() {
    let m = RationalMatrix::from_str("[1, 2, 3; 4, 5, 6]").unwrap();
    let _ = m.row(2);
}

#[test]
#[should_panic]
fn row_fail_3() {
    let m = RationalMatrix::from_str("[]_{0,4}").unwrap();
    let _ = m.row(0);
}

#[test]
fn row_properties() {
    rational_matrix_unsigned_pair_gen_var_1().test_properties(|(m, i)| {
        let v = m.row(i);
        assert_eq!(v.elements.len(), m.width());
        assert!(v.elements.iter().eq(m.row_iter(i)));
        assert_eq!(m.transpose().column(i), v);
    });

    rational_matrix_gen().test_properties(|m| {
        // The rows rebuild the matrix, unless it has no rows and so loses its width.
        if m.height() != 0 {
            let rows = (0..m.height()).map(|i| m.row(i)).collect::<Vec<_>>();
            assert_eq!(RationalMatrix::from_rows(&rows), m);
        }
    });
}
