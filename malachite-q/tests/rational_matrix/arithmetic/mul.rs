// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::matrix::Matrix;
use malachite_nz::test_util::generators::integer_matrix_pair_gen_var_4;
use malachite_q::rational_matrix::RationalMatrix;
use malachite_q::rational_matrix::arithmetic::mul::mul_by_clearing_denominators;
use malachite_q::test_util::generators::{rational_matrix_gen, rational_matrix_pair_gen_var_4};
use malachite_q::test_util::rational_matrix::arithmetic::mul::*;

#[test]
fn test_mul() {
    let test = |s, t, out| {
        let a = RationalMatrix::from_str(s).unwrap();
        let b = RationalMatrix::from_str(t).unwrap();
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
        assert_eq!(rational_matrix_mul_naive(&a, &b), c);
        assert_eq!(rational_matrix_mul_matwise(&a, &b), c);
        assert_eq!(mul_by_clearing_denominators(&a, &b), c);
    };
    test("[]_{0,0}", "[]_{0,0}", "[]_{0,0}");
    test("[]_{0,3}", "[1/2, 2; 3, -1/3; 0, 1]", "[]_{0,2}");
    test("[]_{2,0}", "[]_{0,3}", "[0, 0, 0; 0, 0, 0]");
    test("[1/2, 2, 3]", "[]_{3,0}", "[]_{1,0}");
    test("[1/2]", "[2/3]", "[1/3]");
    test("[-3/4]", "[0]", "[0]");
    test(
        "[1, -2, 3; 4, 5, -6]",
        "[1, 0; 2, -1; 0, 3]",
        "[-3, 11; 14, -23]",
    );
    test(
        "[1, 2; 1/2, 1/3]",
        "[1/5, 1; 3, 1/7]",
        "[31/5, 9/7; 11/10, 23/42]",
    );
    test("[1/2, 1/2]", "[1; 1]", "[1]");
    test(
        "[1/6, 1/3; 1/2, 1/4]",
        "[3, 3/2; 3/2, 2]",
        "[1, 11/12; 15/8, 5/4]",
    );
    test("[1/2, -1/2; 1/3, 2/3]", "[1, 1; 1, 1]", "[0, 0; 1, 1]");
    test("[0, 0; 1/2, 1/3]", "[1/5, 0; 3, 0]", "[0, 0; 11/10, 0]");
    test("[1/2, 1/3, 1/4]", "[2; 3; 4]", "[3]");
    test(
        "[2; 3; 4]",
        "[1/2, 1/3, 1/4]",
        "[1, 2/3, 1/2; 3/2, 1, 3/4; 2, 4/3, 1]",
    );
    test(
        "[1000000000000/3, 1/1000000000000; -7, 22/7]",
        "[3/1000000000000, 5; 1000000000000, -7/22]",
        "[2, 109999999999999999999999979/66000000000000; 21999999999999999999999853/7000000000000, \
        -36]",
    );
    test(
        "[340282366920938463463374607431768211455/18446744073709551616, 1/340282366920938463463374\
        607431768211457]",
        "[18446744073709551616; 340282366920938463463374607431768211457]",
        "[340282366920938463463374607431768211456]",
    );
}

#[test]
#[should_panic]
fn mul_fail() {
    let a = RationalMatrix::from_str("[1/2, 2]").unwrap();
    let _ = &a * &a;
}

#[test]
#[should_panic]
fn mul_assign_fail() {
    let mut a = RationalMatrix::from_str("[1/2; 2]").unwrap();
    a *= RationalMatrix::from_str("[1/2; 2]").unwrap();
}

#[test]
fn mul_properties() {
    rational_matrix_pair_gen_var_4().test_properties(|(a, b)| {
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
        assert_eq!(rational_matrix_mul_naive(&a, &b), c);
        assert_eq!(rational_matrix_mul_matwise(&a, &b), c);
        assert_eq!(mul_by_clearing_denominators(&a, &b), c);
        // The transpose of a product is the product of the transposes in the other order.
        assert_eq!(b.transpose() * a.transpose(), c.transpose());
        // The product is linear in each factor.
        assert_eq!(&(&a + &a) * &b, &c + &c);
        assert_eq!(&a * &(&b + &b), &c + &c);
        assert_eq!(&(-&a) * &b, -&c);
    });

    rational_matrix_gen().test_properties(|a| {
        let (h, w) = (a.height(), a.width());
        // Multiplying by the identity matrix leaves a matrix unchanged.
        assert_eq!(&RationalMatrix::identity(h) * &a, a);
        assert_eq!(&a * &RationalMatrix::identity(w), a);
        // Zero matrices give zero products.
        assert_eq!(&a * &RationalMatrix::zero(w, 3), RationalMatrix::zero(h, 3));
        assert_eq!(&RationalMatrix::zero(3, h) * &a, RationalMatrix::zero(3, w));
    });

    integer_matrix_pair_gen_var_4().test_properties(|(a, b)| {
        // The product agrees with the product of the conversions.
        assert_eq!(
            RationalMatrix::from(a.clone()) * RationalMatrix::from(b.clone()),
            RationalMatrix::from(&a * &b)
        );
    });
}
