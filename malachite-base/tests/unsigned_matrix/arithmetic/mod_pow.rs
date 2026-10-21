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
    IsPowerOf2, ModIsReduced, ModMul, ModPow, ModPowAssign, ModPowerOf2Pow, ModSquare,
};
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::test_util::generators::{
    unsigned_matrix_unsigned_pair_gen_var_8, unsigned_matrix_unsigned_unsigned_triple_gen_var_9,
};
use malachite_base::test_util::unsigned_matrix::arithmetic::mod_pow::*;
use malachite_base::unsigned_matrix::UnsignedMatrix;

#[test]
fn test_mod_pow() {
    fn test<T: PrimitiveUnsigned>(s: &str, exp: u64, m: T, out: &str) {
        let a = UnsignedMatrix::<T>::from_str(s).unwrap();
        let b = a.clone().mod_pow(exp, m);
        assert!(b.is_valid());
        assert_eq!(b.to_string(), out);
        assert_eq!((&a).mod_pow(exp, m), b);
        let mut c = a.clone();
        c.mod_pow_assign(exp, m);
        assert!(c.is_valid());
        assert_eq!(c, b);
        assert_eq!(unsigned_matrix_mod_pow_naive(&a, exp, m), b);
    }
    let test_u8 = test::<u8>;
    test_u8("[]_{0,0}", 0, 3, "[]_{0,0}");
    test_u8("[]_{0,0}", 5, 3, "[]_{0,0}");
    test_u8("[0]", 0, 1, "[0]");
    test_u8("[5]", 0, 7, "[1]");
    test_u8("[5]", 1, 7, "[5]");
    test_u8("[5]", 6, 7, "[1]");
    test_u8("[1, 1; 1, 0]", 10, 100, "[89, 55; 55, 34]");
    test_u8("[1, 1; 1, 0]", 12, 255, "[233, 144; 144, 89]");
    test_u8("[1, 2; 3, 4]", 2, 9, "[7, 1; 6, 4]");
    test_u8("[1, 2; 3, 4]", 3, 9, "[1, 0; 0, 1]");
    test_u8(
        "[2, 0, 1; 1, 3, 0; 0, 1, 1]",
        4,
        7,
        "[3, 4, 2; 3, 6, 4; 4, 6, 1]",
    );
    test_u8("[0, 1; 0, 0]", 2, 5, "[0, 0; 0, 0]");
    test_u8("[0, 1; 0, 0]", 100, 5, "[0, 0; 0, 0]");
    test_u8("[2, 0; 0, 3]", 6, 6, "[4, 0; 0, 3]");
    test_u8("[1, 1; 1, 0]", 200, 251, "[45, 224; 224, 72]");
    let test_u64 = test::<u64>;
    test_u64(
        "[18446744073709551556, 9223372036854775808; 5, 18446744073709551555]",
        5,
        18446744073709551557,
        "[13835058055282007538, 16140901064496600415; 4611686018427513788, 9223372036854594469]",
    );
    test_u64(
        "[1, 1; 1, 0]",
        1000,
        18446744073709551615,
        "[1965850429854679336, 6604850919967101945; 6604850919967101945, 13807743583597129006]",
    );
    test_u64(
        "[18446744073709551614, 1, 7; 3, 9223372036854775808, 0; 0, 1, 1099511627776]",
        17,
        18446744073709551615,
        "[8427596703847044866, 6473944048393747566, 9145951559589380395; 10397708756782753014, 136\
        26451082238421373, 9024123388398489684; 6559658790831084702, 7221643057315389781, 11751976\
        640274636154]",
    );
}

#[test]
#[should_panic]
fn mod_pow_fail_1() {
    // The modulus is zero.
    let _ = UnsignedMatrix::<u8>::from_str("[]_{0,0}")
        .unwrap()
        .mod_pow(2, 0);
}

#[test]
#[should_panic]
fn mod_pow_fail_2() {
    // The matrix is not square.
    let _ = UnsignedMatrix::<u8>::from_str("[1, 2]")
        .unwrap()
        .mod_pow(2, 3);
}

#[test]
#[should_panic]
fn mod_pow_fail_3() {
    // The matrix is not reduced, and the exponent is 1.
    let _ = UnsignedMatrix::<u8>::from_str("[7]").unwrap().mod_pow(1, 7);
}

#[test]
#[should_panic]
fn mod_pow_ref_fail() {
    // The matrix is not reduced, and the exponent is 0.
    let _ = (&UnsignedMatrix::<u8>::from_str("[7]").unwrap()).mod_pow(0, 7);
}

#[test]
#[should_panic]
fn mod_pow_assign_fail() {
    // The matrix is not square, and the exponent is 1.
    let mut a = UnsignedMatrix::<u8>::from_str("[1; 2]").unwrap();
    a.mod_pow_assign(1, 3);
}

#[test]
fn mod_pow_properties() {
    unsigned_matrix_unsigned_unsigned_triple_gen_var_9().test_properties(|(a, exp, m)| {
        let b = (&a).mod_pow(exp, m);
        assert!(b.is_valid());
        assert_eq!((b.height(), b.width()), (a.height(), a.width()));
        assert!(b.mod_is_reduced(&m));
        assert_eq!(a.clone().mod_pow(exp, m), b);
        let mut c = a.clone();
        c.mod_pow_assign(exp, m);
        assert_eq!(c, b);
        assert_eq!(unsigned_matrix_mod_pow_naive(&a, exp, m), b);
        // The power of the transpose is the transpose of the power.
        assert_eq!(a.transpose().mod_pow(exp, m), b.transpose());
        if let Some(exp_plus_1) = exp.checked_add(1) {
            // One more factor multiplies the power by the matrix, on either side.
            let c = (&a).mod_pow(exp_plus_1, m);
            assert_eq!((&b).mod_mul(&a, m), c);
            assert_eq!((&a).mod_mul(&b, m), c);
        }
        // When m is a power of 2, the power agrees with the power modulo a power of 2.
        if m.is_power_of_2() {
            assert_eq!(a.mod_power_of_2_pow(exp, m.significant_bits() - 1), b);
        }
    });

    unsigned_matrix_unsigned_pair_gen_var_8().test_properties(|(a, m)| {
        assert_eq!((&a).mod_pow(0, m), UnsignedMatrix::identity(a.height()) % m);
        assert_eq!((&a).mod_pow(1, m), a);
        assert_eq!((&a).mod_pow(2, m), (&a).mod_square(m));
    });
}
