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
use malachite_q::test_util::generators::rational_matrix_integer_matrix_pair_gen_var_1;
use malachite_q::test_util::rational_matrix::arithmetic::mul_integer_matrix::*;

#[test]
fn test_mul_integer_matrix() {
    let test = |s, t, out| {
        let a = RationalMatrix::from_str(s).unwrap();
        let b = IntegerMatrix::from_str(t).unwrap();
        let c = &a * &b;
        assert!(c.is_valid());
        assert_eq!(c.to_string(), out);
        assert_eq!(a.clone() * b.clone(), c);
        assert_eq!(a.clone() * &b, c);
        assert_eq!(&a * b.clone(), c);
        let mut d = a.clone();
        d *= b.clone();
        assert!(d.is_valid());
        assert_eq!(d, c);
        let mut d = a.clone();
        d *= &b;
        assert_eq!(d, c);
        assert_eq!(rational_matrix_mul_integer_matrix_naive(&a, &b), c);
    };
    test("[]_{0,0}", "[]_{0,0}", "[]_{0,0}");
    test("[]_{0,3}", "[1, 2; 0, 1; 2, 0]", "[]_{0,2}");
    test("[]_{2,0}", "[]_{0,3}", "[0, 0, 0; 0, 0, 0]");
    test("[1/2, 2, 3]", "[]_{3,0}", "[]_{1,0}");
    test("[1/2]", "[4]", "[2]");
    test("[-3/4]", "[0]", "[0]");
    test(
        "[1, -2, 3; 4, 5, -6]",
        "[1, 0; 2, -1; 0, 3]",
        "[-3, 11; 14, -23]",
    );
    test("[1, 2; 1/2, 1/3]", "[6, 1; 3, 7]", "[12, 15; 4, 17/6]");
    test(
        "[1/2, 1, 0; 0, 1/3, 2]",
        "[2, 1; -1, 0; 1, 3]",
        "[0, 1/2; 5/3, 6]",
    );
    test("[1/6, 1/3; 1/2, 1/4]", "[3, 3; 3, 2]", "[3/2, 7/6; 9/4, 2]");
    test(
        "[1000000000000/3, 1/1000000000000; -7, 22/7]",
        "[3, 1000000000000; 1000000000000, -7]",
        "[1000000000001, 999999999999999999999999999999999979/3000000000000; 21999999999853/7, -70\
        00000000022]",
    );
}

#[test]
#[should_panic]
fn mul_integer_matrix_fail() {
    let a = RationalMatrix::from_str("[1, 2]").unwrap();
    let b = IntegerMatrix::from_str("[1, 2]").unwrap();
    let _ = &a * &b;
}

#[test]
#[should_panic]
fn mul_assign_integer_matrix_fail() {
    let mut a = RationalMatrix::from_str("[1/2; 2]").unwrap();
    a *= IntegerMatrix::from_str("[1; 2]").unwrap();
}

#[test]
fn mul_integer_matrix_properties() {
    rational_matrix_integer_matrix_pair_gen_var_1().test_properties(|(a, b)| {
        let c = &a * &b;
        assert!(c.is_valid());
        assert_eq!((c.height(), c.width()), (a.height(), b.width()));
        assert_eq!(a.clone() * b.clone(), c);
        assert_eq!(a.clone() * &b, c);
        assert_eq!(&a * b.clone(), c);
        let mut d = a.clone();
        d *= b.clone();
        assert!(d.is_valid());
        assert_eq!(d, c);
        let mut d = a.clone();
        d *= &b;
        assert_eq!(d, c);
        assert_eq!(rational_matrix_mul_integer_matrix_naive(&a, &b), c);
        // The product agrees with the product of `RationalMatrix`s.
        assert_eq!(&a * &RationalMatrix::from(b.clone()), c);
        // The transpose of a product is the product of the transposes in the other order.
        assert_eq!(b.transpose() * a.transpose(), c.transpose());
    });

    integer_matrix_pair_gen_var_4().test_properties(|(a, b)| {
        // The product of integer matrices agrees with the product of `IntegerMatrix`s.
        assert_eq!(
            RationalMatrix::from(a.clone()) * &b,
            RationalMatrix::from(&a * &b)
        );
    });
}
