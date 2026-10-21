// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::{
    IsPowerOf2, ModIsReduced, ModKroneckerProduct, ModKroneckerProductAssign,
    ModPowerOf2KroneckerProduct,
};
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::test_util::generators::{
    unsigned_matrix_unsigned_matrix_unsigned_triple_gen_var_6,
    unsigned_matrix_unsigned_pair_gen_var_5,
};
use malachite_base::test_util::unsigned_matrix::arithmetic::mod_kronecker_product::*;
use malachite_base::unsigned_matrix::UnsignedMatrix;

#[test]
fn test_mod_kronecker_product() {
    let test = |s, t, m, out| {
        let a = UnsignedMatrix::<u8>::from_str(s).unwrap();
        let b = UnsignedMatrix::<u8>::from_str(t).unwrap();
        let c = (&a).mod_kronecker_product(&b, m);
        assert!(c.is_valid());
        assert_eq!(c.to_string(), out);
        assert_eq!(a.clone().mod_kronecker_product(b.clone(), m), c);
        assert_eq!(a.clone().mod_kronecker_product(&b, m), c);
        assert_eq!((&a).mod_kronecker_product(b.clone(), m), c);
        let mut d = a.clone();
        d.mod_kronecker_product_assign(b.clone(), m);
        assert!(d.is_valid());
        assert_eq!(d, c);
        let mut d = a.clone();
        d.mod_kronecker_product_assign(&b, m);
        assert_eq!(d, c);
        assert_eq!(unsigned_matrix_mod_kronecker_product_naive(&a, &b, m), c);
    };
    test("[]_{0,0}", "[]_{0,0}", 3, "[]_{0,0}");
    test("[]_{2,0}", "[1, 2]", 3, "[]_{2,0}");
    test("[1, 2]", "[]_{0,3}", 3, "[]_{0,6}");
    test("[0, 0]", "[0; 0]", 1, "[0, 0; 0, 0]");
    test("[3]", "[1, 2; 3, 4]", 5, "[3, 1; 4, 2]");
    test(
        "[1, 2; 3, 4]",
        "[0, 5; 6, 7]",
        9,
        "[0, 5, 0, 1; 6, 7, 3, 5; 0, 6, 0, 2; 0, 3, 6, 1]",
    );
    test("[254, 2]", "[3; 128]", 255, "[252, 6; 127, 1]");
    test("[1, 1]", "[1; 1]", 2, "[1, 1; 1, 1]");
    test(
        "[200, 100; 50, 25]",
        "[7, 128]",
        251,
        "[145, 249, 198, 250; 99, 125, 175, 188]",
    );
}

#[test]
#[should_panic]
fn mod_kronecker_product_fail_1() {
    // The modulus is zero.
    let a = UnsignedMatrix::<u8>::from_str("[]_{0,0}").unwrap();
    let _ = (&a).mod_kronecker_product(&a, 0);
}

#[test]
#[should_panic]
fn mod_kronecker_product_fail_2() {
    // The first matrix is not reduced.
    let a = UnsignedMatrix::<u8>::from_str("[7]").unwrap();
    let b = UnsignedMatrix::<u8>::from_str("[1]").unwrap();
    let _ = a.mod_kronecker_product(b, 7);
}

#[test]
#[should_panic]
fn mod_kronecker_product_fail_3() {
    // The second matrix is not reduced.
    let a = UnsignedMatrix::<u8>::from_str("[1]").unwrap();
    let b = UnsignedMatrix::<u8>::from_str("[7]").unwrap();
    let _ = a.mod_kronecker_product(b, 7);
}

#[test]
#[should_panic]
fn mod_kronecker_product_assign_fail() {
    let mut a = UnsignedMatrix::<u8>::from_str("[1, 2]").unwrap();
    a.mod_kronecker_product_assign(UnsignedMatrix::<u8>::from_str("[4]").unwrap(), 3);
}

#[test]
fn mod_kronecker_product_properties() {
    unsigned_matrix_unsigned_matrix_unsigned_triple_gen_var_6().test_properties(|(a, b, m)| {
        let c = (&a).mod_kronecker_product(&b, m);
        assert!(c.is_valid());
        assert_eq!(a.clone().mod_kronecker_product(b.clone(), m), c);
        assert_eq!(a.clone().mod_kronecker_product(&b, m), c);
        assert_eq!((&a).mod_kronecker_product(b.clone(), m), c);
        let mut d = a.clone();
        d.mod_kronecker_product_assign(b.clone(), m);
        assert!(d.is_valid());
        assert_eq!(d, c);
        let mut d = a.clone();
        d.mod_kronecker_product_assign(&b, m);
        assert_eq!(d, c);
        assert_eq!(unsigned_matrix_mod_kronecker_product_naive(&a, &b, m), c);
        assert!(c.mod_is_reduced(&m));
        assert_eq!(
            (c.height(), c.width()),
            (a.height() * b.height(), a.width() * b.width())
        );
        // The transpose of the product is the product of the transposes.
        assert_eq!(
            a.transpose().mod_kronecker_product(b.transpose(), m),
            c.transpose()
        );
        // When m is a power of 2, the product agrees with the product modulo a power of 2.
        if m.is_power_of_2() {
            assert_eq!(
                (&a).mod_power_of_2_kronecker_product(&b, m.significant_bits() - 1),
                c
            );
        }
    });

    unsigned_matrix_unsigned_pair_gen_var_5().test_properties(|(a, m)| {
        let (h, w) = (a.height(), a.width());
        // The 1 by 1 identity matrix, reduced, is an identity on either side, unless m is 1.
        let one = UnsignedMatrix::<u64>::identity(1) % m;
        if m != 1 {
            assert_eq!(one.clone().mod_kronecker_product(&a, m), a);
            assert_eq!((&a).mod_kronecker_product(&one, m), a);
        }
        // A zero matrix gives a zero matrix.
        assert_eq!(
            (&a).mod_kronecker_product(UnsignedMatrix::<u64>::zero(2, 3), m),
            UnsignedMatrix::<u64>::zero(h * 2, w * 3)
        );
    });
}
