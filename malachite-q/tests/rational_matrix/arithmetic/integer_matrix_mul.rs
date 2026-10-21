// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::matrix::Matrix;
use malachite_nz::integer_matrix::IntegerMatrix;
use malachite_nz::test_util::generators::integer_matrix_pair_gen_var_4;
use malachite_q::rational_matrix::RationalMatrix;
use malachite_q::test_util::generators::integer_matrix_rational_matrix_pair_gen_var_1;
use malachite_q::test_util::rational_matrix::arithmetic::integer_matrix_mul::*;

#[test]
fn test_integer_matrix_mul() {
    let test = |s, t, out| {
        let a = IntegerMatrix::from_str(s).unwrap();
        let b = RationalMatrix::from_str(t).unwrap();
        let c = &a * &b;
        assert!(c.is_valid());
        assert_eq!(c.to_string(), out);
        assert_eq!(a.clone() * b.clone(), c);
        assert_eq!(a.clone() * &b, c);
        assert_eq!(&a * b.clone(), c);
        assert_eq!(integer_matrix_mul_rational_matrix_naive(&a, &b), c);
    };
    test("[]_{0,0}", "[]_{0,0}", "[]_{0,0}");
    test("[]_{0,3}", "[1/2, 2; 3, -1/3; 0, 1]", "[]_{0,2}");
    test("[]_{2,0}", "[]_{0,3}", "[0, 0, 0; 0, 0, 0]");
    test("[1, 2, 3]", "[]_{3,0}", "[]_{1,0}");
    test("[4]", "[1/2]", "[2]");
    test("[0]", "[-3/4]", "[0]");
    test(
        "[1, -2, 3; 4, 5, -6]",
        "[1, 0; 2, -1; 0, 3]",
        "[-3, 11; 14, -23]",
    );
    test("[6, 3; 1, 7]", "[1, 1/2; 2, 1/3]", "[12, 4; 15, 17/6]");
    test(
        "[1, -2; 3, 4]",
        "[1/2, 1/3; 1/4, 1/5]",
        "[0, -1/15; 5/2, 9/5]",
    );
    test("[2, 0, 1]", "[1/2; 1/3; 1/4]", "[5/4]");
    test(
        "[3, 1000000000000; 1000000000000, -7]",
        "[1000000000000/3, 1/1000000000000; -7, 22/7]",
        "[-6000000000000, 22000000000000000000000021/7000000000000; 1000000000000000000000147/3, -\
        21]",
    );
}

#[test]
#[should_panic]
fn integer_matrix_mul_fail() {
    let a = IntegerMatrix::from_str("[1, 2]").unwrap();
    let b = RationalMatrix::from_str("[1, 2]").unwrap();
    let _ = &a * &b;
}

#[test]
fn integer_matrix_mul_properties() {
    integer_matrix_rational_matrix_pair_gen_var_1().test_properties(|(a, b)| {
        let c = &a * &b;
        assert!(c.is_valid());
        assert_eq!((c.height(), c.width()), (a.height(), b.width()));
        assert_eq!(a.clone() * b.clone(), c);
        assert_eq!(a.clone() * &b, c);
        assert_eq!(&a * b.clone(), c);
        assert_eq!(integer_matrix_mul_rational_matrix_naive(&a, &b), c);
        // The product agrees with the product of `RationalMatrix`s.
        assert_eq!(&RationalMatrix::from(a.clone()) * &b, c);
        // The transpose of a product is the product of the transposes in the other order.
        assert_eq!(b.transpose() * a.transpose(), c.transpose());
    });

    integer_matrix_pair_gen_var_4().test_properties(|(a, b)| {
        // The product of integer matrices agrees with the product of `IntegerMatrix`s.
        assert_eq!(
            &a * RationalMatrix::from(b.clone()),
            RationalMatrix::from(&a * &b)
        );
    });
}
