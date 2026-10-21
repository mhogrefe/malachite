// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::{KroneckerProduct, KroneckerProductAssign};
use malachite_nz::test_util::generators::integer_matrix_pair_gen;
use malachite_q::rational_matrix::RationalMatrix;
use malachite_q::test_util::generators::{rational_matrix_gen, rational_matrix_pair_gen};
use malachite_q::test_util::rational_matrix::arithmetic::kronecker_product::*;

#[test]
fn test_kronecker_product() {
    let test = |s, t, out| {
        let a = RationalMatrix::from_str(s).unwrap();
        let b = RationalMatrix::from_str(t).unwrap();
        let c = (&a).kronecker_product(&b);
        assert!(c.is_valid());
        assert_eq!(c.to_string(), out);
        assert_eq!(a.clone().kronecker_product(b.clone()), c);
        assert_eq!(a.clone().kronecker_product(&b), c);
        assert_eq!((&a).kronecker_product(b.clone()), c);
        let mut d = a.clone();
        d.kronecker_product_assign(b.clone());
        assert!(d.is_valid());
        assert_eq!(d, c);
        let mut d = a.clone();
        d.kronecker_product_assign(&b);
        assert_eq!(d, c);
        assert_eq!(rational_matrix_kronecker_product_naive(&a, &b), c);
    };
    test("[]_{0,0}", "[]_{0,0}", "[]_{0,0}");
    test("[]_{2,0}", "[1/2, -2]", "[]_{2,0}");
    test("[1/2, -2]", "[]_{0,3}", "[]_{0,6}");
    test("[-2/3]", "[1, 1/2; 3, 3/4]", "[-2/3, -1/3; -2, -1/2]");
    test(
        "[1/2, -2; 3, 1/4]",
        "[0, 2/3; -6, 1]",
        "[0, 1/3, 0, -4/3; -3, 1/2, 12, -2; 0, 2, 0, 1/6; -18, 3, -3/2, 1/4]",
    );
    test("[1/2, 1/3]", "[3; -2]", "[3/2, 1; -1, -2/3]");
    test("[1/2; 1/3]", "[2, 3]", "[1, 3/2; 2/3, 1]");
    test(
        "[1000000000000/3, -1/7]",
        "[3/1000000000000; 7]",
        "[1, -3/7000000000000; 7000000000000/3, -1]",
    );
}

#[test]
fn kronecker_product_properties() {
    rational_matrix_pair_gen().test_properties(|(a, b)| {
        let c = (&a).kronecker_product(&b);
        assert!(c.is_valid());
        assert_eq!(
            (c.height(), c.width()),
            (a.height() * b.height(), a.width() * b.width())
        );
        assert_eq!(a.clone().kronecker_product(b.clone()), c);
        assert_eq!(a.clone().kronecker_product(&b), c);
        assert_eq!((&a).kronecker_product(b.clone()), c);
        let mut d = a.clone();
        d.kronecker_product_assign(b.clone());
        assert!(d.is_valid());
        assert_eq!(d, c);
        let mut d = a.clone();
        d.kronecker_product_assign(&b);
        assert_eq!(d, c);
        assert_eq!(rational_matrix_kronecker_product_naive(&a, &b), c);
        // The transpose of the product is the product of the transposes.
        assert_eq!(
            a.transpose().kronecker_product(b.transpose()),
            c.transpose()
        );
        // The product is linear in each factor.
        assert_eq!((&a + &a).kronecker_product(&b), &c + &c);
        assert_eq!((&a).kronecker_product(&b + &b), &c + &c);
        assert_eq!((-&a).kronecker_product(&b), -&c);
    });

    rational_matrix_gen().test_properties(|a| {
        let (h, w) = (a.height(), a.width());
        // The 1 by 1 identity matrix is an identity on either side.
        assert_eq!(RationalMatrix::identity(1).kronecker_product(&a), a);
        assert_eq!((&a).kronecker_product(RationalMatrix::identity(1)), a);
        // A zero matrix gives a zero matrix.
        assert_eq!(
            (&a).kronecker_product(RationalMatrix::zero(2, 3)),
            RationalMatrix::zero(h * 2, w * 3)
        );
        assert_eq!(
            RationalMatrix::zero(2, 3).kronecker_product(&a),
            RationalMatrix::zero(h * 2, w * 3)
        );
    });

    integer_matrix_pair_gen().test_properties(|(a, b)| {
        // The product agrees with the product of `IntegerMatrix`s.
        assert_eq!(
            RationalMatrix::from(a.clone()).kronecker_product(RationalMatrix::from(b.clone())),
            RationalMatrix::from(a.kronecker_product(b))
        );
    });
}
