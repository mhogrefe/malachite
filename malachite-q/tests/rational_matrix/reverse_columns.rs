// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::matrix::Matrix;
use malachite_q::rational_matrix::RationalMatrix;
use malachite_q::test_util::generators::rational_matrix_gen;

#[test]
fn test_reverse_columns() {
    let test = |s, out| {
        let mut a = RationalMatrix::from_str(s).unwrap();
        a.reverse_columns();
        assert!(a.is_valid());
        assert_eq!(a.to_string(), out);
    };
    test("[]_{0,0}", "[]_{0,0}");
    test("[]_{0,3}", "[]_{0,3}");
    test("[]_{3,0}", "[]_{3,0}");
    test("[7]", "[7]");
    test("[1, 2, 3]", "[3, 2, 1]");
    test("[1; 2; 3]", "[1; 2; 3]");
    test(
        "[1/2, 2, 3; 4, 5/3, 6; 7, 8, -9]",
        "[3, 2, 1/2; 6, 5/3, 4; -9, 8, 7]",
    );
    test(
        "[1, -1/2, 3, 4; 5, 6, 7/8, 8]",
        "[4, 3, -1/2, 1; 8, 7/8, 6, 5]",
    );
    test("[1, 2; 3, 4]", "[2, 1; 4, 3]");
}

#[test]
fn reverse_columns_properties() {
    rational_matrix_gen().test_properties(|a| {
        let mut b = a.clone();
        b.reverse_columns();
        assert!(b.is_valid());
        assert_eq!((b.height(), b.width()), (a.height(), a.width()));
        for r in 0..a.height() {
            for c in 0..a.width() {
                assert_eq!(b.get(r, c), a.get(r, a.width() - 1 - c));
            }
        }

        let mut c = a.clone();
        let width = a.width();
        for j in 0..width >> 1 {
            c.swap_columns(j, width - 1 - j);
        }
        assert_eq!(c, b);

        let mut t = a.transpose();
        t.reverse_rows();
        assert_eq!(t.transpose(), b);

        // Reversing the rows and the columns reverses the elements.
        let mut d = b.clone();
        d.reverse_rows();
        let mut elements = a.to_elements();
        elements.reverse();
        assert_eq!(d.elements_ref(), elements);

        b.reverse_columns();
        assert_eq!(b, a);
    });
}
