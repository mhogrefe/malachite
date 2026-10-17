// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::NegAssign;
use malachite_nz::test_util::generators::integer_matrix_gen;
use malachite_q::rational_matrix::RationalMatrix;
use malachite_q::test_util::generators::{rational_matrix_gen, rational_matrix_gen_var_1};

#[test]
fn test_neg() {
    let test = |s, out| {
        let a = RationalMatrix::from_str(s).unwrap();
        assert_eq!((-a.clone()).to_string(), out);
        assert_eq!((-&a).to_string(), out);
        let mut b = a;
        b.neg_assign();
        assert!(b.is_valid());
        assert_eq!(b.to_string(), out);
    };
    test("[]_{0,0}", "[]_{0,0}");
    test("[]_{3,0}", "[]_{3,0}");
    test("[0]", "[0]");
    test("[0, 0; 0, 0]", "[0, 0; 0, 0]");
    test("[1]", "[-1]");
    test("[-1]", "[1]");
    test("[1/2, -3; 2/3, 0]", "[-1/2, 3; -2/3, 0]");
    test("[1/2, -3, 2/3]", "[-1/2, 3, -2/3]");
    test(
        "[5/7; 0; -1000000000000000000000/3]",
        "[-5/7; 0; 1000000000000000000000/3]",
    );
}

#[test]
fn neg_properties() {
    rational_matrix_gen().test_properties(|a| {
        let b = -&a;
        assert!(b.is_valid());
        assert_eq!(-a.clone(), b);
        let mut c = a.clone();
        c.neg_assign();
        assert_eq!(c, b);

        // Negating twice gives the matrix back, and only a zero matrix is its own negation.
        assert_eq!(-&b, a);
        assert_eq!(b == a, a.is_zero());
        assert_eq!((b.height(), b.width()), (a.height(), a.width()));
        for (x, y) in a.elements.iter().zip(&b.elements) {
            assert_eq!(*y, -x);
        }
        // Negation commutes with transposition, and preserves the shape predicates.
        assert_eq!(-a.transpose(), b.transpose());
        assert_eq!(b.is_symmetric(), a.is_symmetric());
        assert_eq!(b.is_skew_symmetric(), a.is_skew_symmetric());
        assert_eq!(b.is_diagonal(), a.is_diagonal());
        assert_eq!(b.is_upper_triangular(), a.is_upper_triangular());
        assert_eq!(b.is_lower_triangular(), a.is_lower_triangular());
    });

    rational_matrix_gen_var_1().test_properties(|a| {
        // A square matrix is skew-symmetric iff its transpose is its negation.
        assert_eq!(a.is_skew_symmetric(), a.transpose() == -&a);
    });

    integer_matrix_gen().test_properties(|a| {
        // Negating a `IntegerMatrix`'s conversion negates every element.
        let b = -RationalMatrix::from(a.clone());
        assert!(b.elements.iter().zip(&a.elements).all(|(x, y)| -x == *y));
    });
}
