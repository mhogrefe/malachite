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
    IsPowerOf2, ModIsReduced, ModMul, ModPowerOf2Square, ModSquare, ModSquareAssign,
};
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::test_util::generators::unsigned_matrix_unsigned_pair_gen_var_8;
use malachite_base::test_util::unsigned_matrix::arithmetic::mod_square::*;
use malachite_base::unsigned_matrix::UnsignedMatrix;

#[test]
fn test_mod_square() {
    fn test<T: PrimitiveUnsigned>(s: &str, m: T, out: &str) {
        let a = UnsignedMatrix::<T>::from_str(s).unwrap();
        let b = a.clone().mod_square(m);
        assert!(b.is_valid());
        assert_eq!(b.to_string(), out);
        assert_eq!((&a).mod_square(m), b);
        let mut c = a.clone();
        c.mod_square_assign(m);
        assert!(c.is_valid());
        assert_eq!(c, b);
        assert_eq!(unsigned_matrix_mod_square_naive(&a, m), b);
    }
    let test_u8 = test::<u8>;
    test_u8("[]_{0,0}", 3, "[]_{0,0}");
    test_u8("[0]", 1, "[0]");
    test_u8("[5]", 7, "[4]");
    test_u8("[1, 2; 3, 4]", 9, "[7, 1; 6, 4]");
    test_u8("[1, 2; 3, 4]", 255, "[7, 10; 15, 22]");
    test_u8(
        "[2, 0, 1; 1, 3, 0; 0, 1, 1]",
        5,
        "[4, 1, 3; 0, 4, 1; 1, 4, 1]",
    );
    test_u8("[254, 253; 252, 251]", 255, "[7, 10; 15, 22]");
    test_u8("[1, 1; 1, 0]", 2, "[0, 1; 1, 1]");
    test_u8(
        "[127, 1, 64; 3, 100, 2; 50, 60, 70]",
        128,
        "[4, 99, 66; 13, 11, 20; 46, 10, 28]",
    );
    let test_u64 = test::<u64>;
    test_u64(
        "[18446744073709551556, 9223372036854775808; 5, 18446744073709551555]",
        18446744073709551557,
        "[9223372036854775927, 9223372036854775690; 18446744073709551542, 9223372036854775930]",
    );
    test_u64(
        "[18446744073709551614, 1, 18446744073709551613; 7, 18446744073709551614, 4294967296; 0, 1\
        099511627776, 3]",
        18446744073709551615,
        "[8, 18446741874686296061, 4294967292; 18446744073709551601, 264, 8589934578; 769658139443\
        2, 2199023255552, 265]",
    );
}

#[test]
#[should_panic]
fn mod_square_fail_1() {
    // The modulus is zero.
    let _ = UnsignedMatrix::<u8>::from_str("[]_{0,0}")
        .unwrap()
        .mod_square(0);
}

#[test]
#[should_panic]
fn mod_square_fail_2() {
    // The matrix is not square.
    let _ = UnsignedMatrix::<u8>::from_str("[1, 2]")
        .unwrap()
        .mod_square(3);
}

#[test]
#[should_panic]
fn mod_square_fail_3() {
    // The matrix is not reduced.
    let _ = UnsignedMatrix::<u8>::from_str("[7]").unwrap().mod_square(7);
}

#[test]
#[should_panic]
fn mod_square_ref_fail() {
    let _ = (&UnsignedMatrix::<u8>::from_str("[]_{0,2}").unwrap()).mod_square(3);
}

#[test]
#[should_panic]
fn mod_square_assign_fail() {
    let mut a = UnsignedMatrix::<u8>::from_str("[1; 2]").unwrap();
    a.mod_square_assign(3);
}

#[test]
fn mod_square_properties() {
    unsigned_matrix_unsigned_pair_gen_var_8().test_properties(|(a, m)| {
        let b = (&a).mod_square(m);
        assert!(b.is_valid());
        assert_eq!((b.height(), b.width()), (a.height(), a.width()));
        assert!(b.mod_is_reduced(&m));
        assert_eq!(a.clone().mod_square(m), b);
        let mut c = a.clone();
        c.mod_square_assign(m);
        assert_eq!(c, b);
        assert_eq!(unsigned_matrix_mod_square_naive(&a, m), b);
        assert_eq!((&a).mod_mul(&a, m), b);
        // The square of the transpose is the transpose of the square.
        assert_eq!(a.transpose().mod_square(m), b.transpose());
        // When m is a power of 2, the square agrees with the square modulo a power of 2.
        if m.is_power_of_2() {
            assert_eq!(a.mod_power_of_2_square(m.significant_bits() - 1), b);
        }
    });
}
