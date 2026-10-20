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
    ModPowerOf2, ModPowerOf2Mul, ModPowerOf2Square, ModPowerOf2SquareAssign, Square,
};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::unsigned_matrix::UnsignedMatrix;
use malachite_nz::natural_matrix::NaturalMatrix;
use malachite_nz::platform::Limb;
use malachite_nz::test_util::generators::natural_matrix_unsigned_pair_gen_var_6;
use malachite_nz::test_util::natural_matrix::arithmetic::mod_power_of_2_square::*;

#[test]
fn test_mod_power_of_2_square() {
    let test = |s, pow, out| {
        let a = NaturalMatrix::from_str(s).unwrap();
        let b = a.clone().mod_power_of_2_square(pow);
        assert!(b.is_valid());
        assert_eq!(b.to_string(), out);
        assert_eq!((&a).mod_power_of_2_square(pow), b);
        let mut c = a.clone();
        c.mod_power_of_2_square_assign(pow);
        assert!(c.is_valid());
        assert_eq!(c, b);
        assert_eq!(natural_matrix_mod_power_of_2_square_naive(&a, pow), b);
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
        "[18446744073709551615, 9223372036854775808; 5, 18446744073709551616]",
        66,
        "[9223372036854775809, 64563604257983430656; 36893488147419103227, 46116860184273879040]",
    );
    test(
        "[1267650600228229401496703205375, 3; 633825300114114700748351602688, 18446744073709551616]\
        ",
        100,
        "[633825300114114700748351602689, 55340232221128654845; 633825300114114700748351602688, 633\
        825300114114700748351602688]",
    );
    test(
        "[170141183460469231731687303715884105728, 1, 7; 3, 170141183460469231731687303715884105739\
        , 85070591730234615865843651857942052864; 1, 2, 3]",
        128,
        "[10, 25, 255211775190703847597530955573826158613; 85070591730234615865843651857942052897, \
        170141183460469231731687303715884105852, 170141183460469231731687303715884105749; 170141183\
        460469231731687303715884105737, 29, 170141183460469231731687303715884105744]",
    );
}

#[test]
#[should_panic]
fn mod_power_of_2_square_fail_1() {
    // The matrix is not square.
    let _ = NaturalMatrix::from_str("[1, 2]")
        .unwrap()
        .mod_power_of_2_square(3);
}

#[test]
#[should_panic]
fn mod_power_of_2_square_fail_2() {
    // The matrix is not reduced.
    let _ = NaturalMatrix::from_str("[8]")
        .unwrap()
        .mod_power_of_2_square(3);
}

#[test]
#[should_panic]
fn mod_power_of_2_square_ref_fail() {
    let _ = (&NaturalMatrix::from_str("[]_{0,2}").unwrap()).mod_power_of_2_square(3);
}

#[test]
#[should_panic]
fn mod_power_of_2_square_assign_fail() {
    let mut a = NaturalMatrix::from_str("[1; 2]").unwrap();
    a.mod_power_of_2_square_assign(3);
}

#[test]
fn mod_power_of_2_square_properties() {
    natural_matrix_unsigned_pair_gen_var_6().test_properties(|(a, pow)| {
        let b = (&a).mod_power_of_2_square(pow);
        assert!(b.is_valid());
        assert_eq!((b.height(), b.width()), (a.height(), a.width()));
        assert_eq!(a.clone().mod_power_of_2_square(pow), b);
        let mut c = a.clone();
        c.mod_power_of_2_square_assign(pow);
        assert_eq!(c, b);
        assert_eq!(natural_matrix_mod_power_of_2_square_naive(&a, pow), b);
        assert_eq!((&a).mod_power_of_2_mul(&a, pow), b);
        // The square of the transpose is the transpose of the square.
        assert_eq!(a.transpose().mod_power_of_2_square(pow), b.transpose());
        // The square modulo 2^k is the exact square, reduced.
        assert_eq!((&a).square().mod_power_of_2(pow), b);
        // When 2^k fits in a limb, the square agrees with that of an `UnsignedMatrix`.
        if pow <= Limb::WIDTH {
            let a_limbs = UnsignedMatrix::<Limb>::try_from(&a).unwrap();
            assert_eq!(NaturalMatrix::from(a_limbs.mod_power_of_2_square(pow)), b);
        }
    });
}
