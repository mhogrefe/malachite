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
    ModPowerOf2, ModPowerOf2IsReduced, ModPowerOf2KroneckerProduct,
    ModPowerOf2KroneckerProductAssign,
};
use malachite_base::test_util::generators::{
    unsigned_matrix_unsigned_matrix_unsigned_triple_gen_var_5,
    unsigned_matrix_unsigned_pair_gen_var_4,
};
use malachite_base::test_util::unsigned_matrix::arithmetic::mod_power_of_2_kronecker_product::*;
use malachite_base::unsigned_matrix::UnsignedMatrix;

#[test]
fn test_mod_power_of_2_kronecker_product() {
    let test = |s, t, pow, out| {
        let a = UnsignedMatrix::<u8>::from_str(s).unwrap();
        let b = UnsignedMatrix::<u8>::from_str(t).unwrap();
        let c = (&a).mod_power_of_2_kronecker_product(&b, pow);
        assert!(c.is_valid());
        assert_eq!(c.to_string(), out);
        assert_eq!(
            a.clone().mod_power_of_2_kronecker_product(b.clone(), pow),
            c
        );
        assert_eq!(a.clone().mod_power_of_2_kronecker_product(&b, pow), c);
        assert_eq!((&a).mod_power_of_2_kronecker_product(b.clone(), pow), c);
        let mut d = a.clone();
        d.mod_power_of_2_kronecker_product_assign(b.clone(), pow);
        assert!(d.is_valid());
        assert_eq!(d, c);
        let mut d = a.clone();
        d.mod_power_of_2_kronecker_product_assign(&b, pow);
        assert_eq!(d, c);
        assert_eq!(
            unsigned_matrix_mod_power_of_2_kronecker_product_naive(&a, &b, pow),
            c
        );
    };
    test("[]_{0,0}", "[]_{0,0}", 3, "[]_{0,0}");
    test("[]_{2,0}", "[1, 2]", 3, "[]_{2,0}");
    test("[1, 2]", "[]_{0,3}", 3, "[]_{0,6}");
    test("[0, 0]", "[0; 0]", 0, "[0, 0; 0, 0]");
    test("[3]", "[1, 2; 3, 4]", 3, "[3, 6; 1, 4]");
    test(
        "[1, 2; 3, 4]",
        "[0, 5; 6, 7]",
        4,
        "[0, 5, 0, 10; 6, 7, 12, 14; 0, 15, 0, 4; 2, 5, 8, 12]",
    );
    test("[255, 2]", "[3; 129]", 8, "[253, 6; 127, 2]");
    test("[1, 1]", "[1; 1]", 1, "[1, 1; 1, 1]");
    test(
        "[200, 100; 50, 25]",
        "[7, 128]",
        8,
        "[120, 0, 188, 0; 94, 0, 175, 128]",
    );
}

#[test]
#[should_panic]
fn mod_power_of_2_kronecker_product_fail_1() {
    // The first matrix is not reduced.
    let a = UnsignedMatrix::<u8>::from_str("[8]").unwrap();
    let b = UnsignedMatrix::<u8>::from_str("[1]").unwrap();
    let _ = a.mod_power_of_2_kronecker_product(b, 3);
}

#[test]
#[should_panic]
fn mod_power_of_2_kronecker_product_fail_2() {
    // The second matrix is not reduced.
    let a = UnsignedMatrix::<u8>::from_str("[1]").unwrap();
    let b = UnsignedMatrix::<u8>::from_str("[8]").unwrap();
    let _ = a.mod_power_of_2_kronecker_product(b, 3);
}

#[test]
#[should_panic]
fn mod_power_of_2_kronecker_product_fail_3() {
    // `pow` is greater than the width of the type.
    let a = UnsignedMatrix::<u8>::from_str("[1]").unwrap();
    let _ = (&a).mod_power_of_2_kronecker_product(&a, 9);
}

#[test]
#[should_panic]
fn mod_power_of_2_kronecker_product_assign_fail() {
    let mut a = UnsignedMatrix::<u8>::from_str("[1, 2]").unwrap();
    a.mod_power_of_2_kronecker_product_assign(UnsignedMatrix::<u8>::from_str("[4]").unwrap(), 2);
}

#[test]
fn mod_power_of_2_kronecker_product_properties() {
    unsigned_matrix_unsigned_matrix_unsigned_triple_gen_var_5().test_properties(|(a, b, pow)| {
        let c = (&a).mod_power_of_2_kronecker_product(&b, pow);
        assert!(c.is_valid());
        assert!(c.mod_power_of_2_is_reduced(pow));
        assert_eq!(
            (c.height(), c.width()),
            (a.height() * b.height(), a.width() * b.width())
        );
        assert_eq!(
            a.clone().mod_power_of_2_kronecker_product(b.clone(), pow),
            c
        );
        assert_eq!(a.clone().mod_power_of_2_kronecker_product(&b, pow), c);
        assert_eq!((&a).mod_power_of_2_kronecker_product(b.clone(), pow), c);
        let mut d = a.clone();
        d.mod_power_of_2_kronecker_product_assign(b.clone(), pow);
        assert!(d.is_valid());
        assert_eq!(d, c);
        let mut d = a.clone();
        d.mod_power_of_2_kronecker_product_assign(&b, pow);
        assert_eq!(d, c);
        assert_eq!(
            unsigned_matrix_mod_power_of_2_kronecker_product_naive(&a, &b, pow),
            c
        );
        // The transpose of the product is the product of the transposes.
        assert_eq!(
            a.transpose()
                .mod_power_of_2_kronecker_product(b.transpose(), pow),
            c.transpose()
        );
    });

    unsigned_matrix_unsigned_pair_gen_var_4().test_properties(|(a, pow)| {
        let (h, w) = (a.height(), a.width());
        // The 1 by 1 identity matrix, reduced, is an identity on either side, unless 2^k is 1.
        let one = UnsignedMatrix::<u64>::identity(1).mod_power_of_2(pow);
        if pow != 0 {
            assert_eq!(one.clone().mod_power_of_2_kronecker_product(&a, pow), a);
            assert_eq!((&a).mod_power_of_2_kronecker_product(&one, pow), a);
        }
        // A zero matrix gives a zero matrix.
        assert_eq!(
            (&a).mod_power_of_2_kronecker_product(UnsignedMatrix::<u64>::zero(2, 3), pow),
            UnsignedMatrix::<u64>::zero(h * 2, w * 3)
        );
    });
}
