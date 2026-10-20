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
    ModPowerOf2Mul, ModPowerOf2Square, ModPowerOf2SquareAssign,
};
use malachite_base::test_util::generators::unsigned_matrix_unsigned_pair_gen_var_7;
use malachite_base::test_util::unsigned_matrix::arithmetic::mod_power_of_2_square::*;
use malachite_base::unsigned_matrix::UnsignedMatrix;

#[test]
fn test_mod_power_of_2_square() {
    let test = |s, pow, out| {
        let a = UnsignedMatrix::<u8>::from_str(s).unwrap();
        let b = a.clone().mod_power_of_2_square(pow);
        assert!(b.is_valid());
        assert_eq!(b.to_string(), out);
        assert_eq!((&a).mod_power_of_2_square(pow), b);
        let mut c = a.clone();
        c.mod_power_of_2_square_assign(pow);
        assert!(c.is_valid());
        assert_eq!(c, b);
        assert_eq!(unsigned_matrix_mod_power_of_2_square_naive(&a, pow), b);
    };
    test("[]_{0,0}", 3, "[]_{0,0}");
    test("[5]", 3, "[1]");
    test("[1, 2; 3, 4]", 4, "[7, 10; 15, 6]");
    test("[1, 2; 3, 4]", 8, "[7, 10; 15, 22]");
    test(
        "[2, 0, 1; 1, 3, 0; 0, 1, 1]",
        3,
        "[4, 1, 3; 5, 1, 1; 1, 4, 1]",
    );
    test("[255, 254; 253, 252]", 8, "[7, 10; 15, 22]");
    test("[1, 1; 1, 0]", 1, "[0, 1; 1, 1]");
    test("[0, 0; 0, 0]", 0, "[0, 0; 0, 0]");
    test(
        "[127, 1, 64; 3, 100, 2; 50, 60, 70]",
        7,
        "[4, 99, 66; 13, 11, 20; 46, 10, 28]",
    );
}

#[test]
#[should_panic]
fn mod_power_of_2_square_fail_1() {
    // The matrix is not square.
    let _ = UnsignedMatrix::<u8>::from_str("[1, 2]")
        .unwrap()
        .mod_power_of_2_square(3);
}

#[test]
#[should_panic]
fn mod_power_of_2_square_fail_2() {
    // The matrix is not reduced.
    let _ = UnsignedMatrix::<u8>::from_str("[8]")
        .unwrap()
        .mod_power_of_2_square(3);
}

#[test]
#[should_panic]
fn mod_power_of_2_square_fail_3() {
    // `pow` is greater than the width of the type.
    let a = UnsignedMatrix::<u8>::from_str("[1]").unwrap();
    let _ = a.mod_power_of_2_square(9);
}

#[test]
#[should_panic]
fn mod_power_of_2_square_ref_fail() {
    let _ = (&UnsignedMatrix::<u8>::from_str("[]_{0,2}").unwrap()).mod_power_of_2_square(3);
}

#[test]
#[should_panic]
fn mod_power_of_2_square_assign_fail() {
    let mut a = UnsignedMatrix::<u8>::from_str("[1; 2]").unwrap();
    a.mod_power_of_2_square_assign(3);
}

#[test]
fn mod_power_of_2_square_properties() {
    unsigned_matrix_unsigned_pair_gen_var_7().test_properties(|(a, pow)| {
        let b = (&a).mod_power_of_2_square(pow);
        assert!(b.is_valid());
        assert_eq!((b.height(), b.width()), (a.height(), a.width()));
        assert_eq!(a.clone().mod_power_of_2_square(pow), b);
        let mut c = a.clone();
        c.mod_power_of_2_square_assign(pow);
        assert_eq!(c, b);
        assert_eq!(unsigned_matrix_mod_power_of_2_square_naive(&a, pow), b);
        assert_eq!((&a).mod_power_of_2_mul(&a, pow), b);
        // The square of the transpose is the transpose of the square.
        assert_eq!(a.transpose().mod_power_of_2_square(pow), b.transpose());
    });
}
