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
    ModPowerOf2, ModPowerOf2Mul, ModPowerOf2Pow, ModPowerOf2PowAssign, ModPowerOf2Square,
};
use malachite_base::test_util::generators::{
    unsigned_matrix_unsigned_pair_gen_var_7, unsigned_matrix_unsigned_unsigned_triple_gen_var_8,
};
use malachite_base::test_util::unsigned_matrix::arithmetic::mod_power_of_2_pow::*;
use malachite_base::unsigned_matrix::UnsignedMatrix;

#[test]
fn test_mod_power_of_2_pow() {
    let test = |s, exp: u64, pow, out| {
        let a = UnsignedMatrix::<u8>::from_str(s).unwrap();
        let b = a.clone().mod_power_of_2_pow(exp, pow);
        assert!(b.is_valid());
        assert_eq!(b.to_string(), out);
        assert_eq!((&a).mod_power_of_2_pow(exp, pow), b);
        let mut c = a.clone();
        c.mod_power_of_2_pow_assign(exp, pow);
        assert!(c.is_valid());
        assert_eq!(c, b);
        assert_eq!(unsigned_matrix_mod_power_of_2_pow_naive(&a, exp, pow), b);
    };
    test("[]_{0,0}", 3, 2, "[]_{0,0}");
    test("[3]", 5, 4, "[3]");
    test("[1, 1; 1, 0]", 10, 6, "[25, 55; 55, 34]");
    test(
        "[2, 0, 1; 1, 3, 0; 0, 1, 1]",
        4,
        5,
        "[24, 25, 16; 2, 26, 25; 25, 9, 8]",
    );
    test("[1, 2; 3, 4]", 0, 3, "[1, 0; 0, 1]");
    test("[0, 0; 0, 0]", 0, 0, "[0, 0; 0, 0]");
    test("[0, 1; 0, 0]", 5, 3, "[0, 0; 0, 0]");
    test(
        "[1, 1; 1, 0]",
        18446744073709551615,
        8,
        "[59, 226; 226, 89]",
    );
    test("[255, 1; 2, 254]", 1000, 8, "[11, 245; 234, 22]");
    test("[1, 2; 3, 4]", 1, 3, "[1, 2; 3, 4]");
}

#[test]
#[should_panic]
fn mod_power_of_2_pow_fail_1() {
    // The matrix is not square.
    let _ = UnsignedMatrix::<u8>::from_str("[1, 2]")
        .unwrap()
        .mod_power_of_2_pow(2, 3);
}

#[test]
#[should_panic]
fn mod_power_of_2_pow_fail_2() {
    // The matrix is not reduced, even though the exponent is 1.
    let _ = UnsignedMatrix::<u8>::from_str("[8]")
        .unwrap()
        .mod_power_of_2_pow(1, 3);
}

#[test]
#[should_panic]
fn mod_power_of_2_pow_fail_3() {
    // `pow` is greater than the width of the type.
    let _ = UnsignedMatrix::<u8>::from_str("[1]")
        .unwrap()
        .mod_power_of_2_pow(2, 9);
}

#[test]
#[should_panic]
fn mod_power_of_2_pow_ref_fail() {
    let _ = (&UnsignedMatrix::<u8>::from_str("[]_{0,2}").unwrap()).mod_power_of_2_pow(0, 3);
}

#[test]
#[should_panic]
fn mod_power_of_2_pow_assign_fail() {
    let mut a = UnsignedMatrix::<u8>::from_str("[1; 2]").unwrap();
    a.mod_power_of_2_pow_assign(1, 3);
}

#[test]
fn mod_power_of_2_pow_properties() {
    unsigned_matrix_unsigned_unsigned_triple_gen_var_8().test_properties(|(a, exp, pow)| {
        let b = (&a).mod_power_of_2_pow(exp, pow);
        assert!(b.is_valid());
        assert_eq!((b.height(), b.width()), (a.height(), a.width()));
        assert_eq!(a.clone().mod_power_of_2_pow(exp, pow), b);
        let mut c = a.clone();
        c.mod_power_of_2_pow_assign(exp, pow);
        assert_eq!(c, b);
        assert_eq!(unsigned_matrix_mod_power_of_2_pow_naive(&a, exp, pow), b);
        // The power of the transpose is the transpose of the power.
        assert_eq!(a.transpose().mod_power_of_2_pow(exp, pow), b.transpose());
        if let Some(exp_plus_1) = exp.checked_add(1) {
            // One more factor multiplies the power by the matrix, on either side.
            let c = (&a).mod_power_of_2_pow(exp_plus_1, pow);
            assert_eq!((&b).mod_power_of_2_mul(&a, pow), c);
            assert_eq!((&a).mod_power_of_2_mul(&b, pow), c);
        }
    });

    unsigned_matrix_unsigned_pair_gen_var_7().test_properties(|(a, pow)| {
        assert_eq!(
            (&a).mod_power_of_2_pow(0, pow),
            UnsignedMatrix::identity(a.height()).mod_power_of_2(pow)
        );
        assert_eq!((&a).mod_power_of_2_pow(1, pow), a);
        assert_eq!(
            (&a).mod_power_of_2_pow(2, pow),
            (&a).mod_power_of_2_square(pow)
        );
    });
}
