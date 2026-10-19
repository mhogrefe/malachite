// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::matrix::Matrix;
use malachite_base::test_util::generators::unsigned_matrix_gen;
use malachite_base::unsigned_matrix::UnsignedMatrix;

#[test]
fn test_reverse_rows() {
    let test = |s, out| {
        let mut a = UnsignedMatrix::<u64>::from_str(s).unwrap();
        a.reverse_rows();
        assert!(a.is_valid());
        assert_eq!(a.to_string(), out);
    };
    test("[]_{0,0}", "[]_{0,0}");
    test("[]_{0,3}", "[]_{0,3}");
    test("[]_{3,0}", "[]_{3,0}");
    test("[7]", "[7]");
    test("[1, 2, 3]", "[1, 2, 3]");
    test("[1; 2; 3]", "[3; 2; 1]");
    test("[1, 2, 3; 4, 5, 6; 7, 8, 9]", "[7, 8, 9; 4, 5, 6; 1, 2, 3]");
    test("[1, 2, 3, 4; 5, 6, 7, 8]", "[5, 6, 7, 8; 1, 2, 3, 4]");
    test("[1, 2; 3, 4]", "[3, 4; 1, 2]");
}

#[test]
fn reverse_rows_properties() {
    unsigned_matrix_gen().test_properties(|a| {
        let mut b = a.clone();
        b.reverse_rows();
        assert!(b.is_valid());
        assert_eq!((b.height(), b.width()), (a.height(), a.width()));
        for r in 0..a.height() {
            for c in 0..a.width() {
                assert_eq!(b.get(r, c), a.get(a.height() - 1 - r, c));
            }
        }

        let mut c = a.clone();
        let height = a.height();
        for i in 0..height >> 1 {
            c.swap_rows(i, height - 1 - i);
        }
        assert_eq!(c, b);

        let mut t = a.transpose();
        t.reverse_columns();
        assert_eq!(t.transpose(), b);

        // Reversing the rows and the columns reverses the elements.
        let mut d = b.clone();
        d.reverse_columns();
        let mut elements = a.elements.clone();
        elements.reverse();
        assert_eq!(d.elements, elements);

        b.reverse_rows();
        assert_eq!(b, a);
    });
}
