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
use malachite_nz::integer_matrix::IntegerMatrix;
use malachite_nz::test_util::generators::{integer_matrix_gen, integer_matrix_pair_gen};
use malachite_nz::test_util::integer_matrix::arithmetic::kronecker_product::*;

#[test]
fn test_kronecker_product() {
    let test = |s, t, out| {
        let a = IntegerMatrix::from_str(s).unwrap();
        let b = IntegerMatrix::from_str(t).unwrap();
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
        assert_eq!(integer_matrix_kronecker_product_naive(&a, &b), c);
    };
    test("[]_{0,0}", "[]_{0,0}", "[]_{0,0}");
    test("[]_{2,0}", "[1, -2]", "[]_{2,0}");
    test("[1, -2]", "[]_{0,3}", "[]_{0,6}");
    test("[-3]", "[1, 2; 3, 4]", "[-3, -6; -9, -12]");
    test(
        "[1, -2; 3, 4]",
        "[0, 5; -6, 7]",
        "[0, 5, 0, -10; -6, 7, 12, -14; 0, 15, 0, 20; -18, 21, -24, 28]",
    );
    test("[1, 2, 3]", "[-1; 1]", "[-1, -2, -3; 1, 2, 3]");
    test(
        "[0, 1; -1, 0]",
        "[1, 2, 3; 4, 5, 6]",
        "[0, 0, 0, 1, 2, 3; 0, 0, 0, 4, 5, 6; -1, -2, -3, 0, 0, 0; -4, -5, -6, 0, 0, 0]",
    );
    test(
        "[-18446744073709551615, 2]",
        "[18446744073709551617; -3]",
        "[-340282366920938463463374607431768211455, 36893488147419103234; 55340232221128654845, -6\
        ]",
    );
}

#[test]
fn kronecker_product_properties() {
    integer_matrix_pair_gen().test_properties(|(a, b)| {
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
        assert_eq!(integer_matrix_kronecker_product_naive(&a, &b), c);
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

    integer_matrix_gen().test_properties(|a| {
        let (h, w) = (a.height(), a.width());
        // The 1 by 1 identity matrix is an identity on either side.
        assert_eq!(IntegerMatrix::identity(1).kronecker_product(&a), a);
        assert_eq!((&a).kronecker_product(IntegerMatrix::identity(1)), a);
        // A zero matrix gives a zero matrix.
        assert_eq!(
            (&a).kronecker_product(IntegerMatrix::zero(2, 3)),
            IntegerMatrix::zero(h * 2, w * 3)
        );
        assert_eq!(
            IntegerMatrix::zero(2, 3).kronecker_product(&a),
            IntegerMatrix::zero(h * 2, w * 3)
        );
    });
}
