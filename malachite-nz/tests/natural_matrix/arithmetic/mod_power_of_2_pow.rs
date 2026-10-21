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
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::unsigned_matrix::UnsignedMatrix;
use malachite_nz::natural_matrix::NaturalMatrix;
use malachite_nz::platform::Limb;
use malachite_nz::test_util::generators::{
    natural_matrix_unsigned_pair_gen_var_6, natural_matrix_unsigned_unsigned_triple_gen_var_5,
};
use malachite_nz::test_util::natural_matrix::arithmetic::mod_power_of_2_pow::*;

#[test]
fn test_mod_power_of_2_pow() {
    let test = |s, exp: u64, pow, out| {
        let a = NaturalMatrix::from_str(s).unwrap();
        let b = a.clone().mod_power_of_2_pow(exp, pow);
        assert!(b.is_valid());
        assert_eq!(b.to_string(), out);
        assert_eq!((&a).mod_power_of_2_pow(exp, pow), b);
        let mut c = a.clone();
        c.mod_power_of_2_pow_assign(exp, pow);
        assert!(c.is_valid());
        assert_eq!(c, b);
        assert_eq!(natural_matrix_mod_power_of_2_pow_naive(&a, exp, pow), b);
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
        100,
        "[1182588270542693272899077927483, 457535229876603549268635748066; 457535229876603549268635\
        748066, 725053040666089723630442179417]",
    );
    test(
        "[18446744073709551615, 9223372036854775808; 5, 18446744073709551616]",
        1000,
        66,
        "[27670116110564327425, 64563604257983430656; 18446744073709551611, 46116860184273879040]",
    );
    test(
        "[170141183460469231731687303715884105728, 1, 7; 3, 170141183460469231731687303715884105739\
        , 85070591730234615865843651857942052864; 1, 2, 3]",
        12345,
        128,
        "[198530513580268406164216123272347747164, 30753978262482851668582846834329775797, 24717946\
        2502412295519486925709011481835; 255636178296520311263484361481621164759, 19895883497636252\
        7027053364478054058383, 173524653436167969468663045084510239908; 25503100765686183665408916\
        3127031528821, 313336261739970813197720907873859576006, 24463557542574403865227700847453617\
        943]",
    );
}

#[test]
#[should_panic]
fn mod_power_of_2_pow_fail_1() {
    // The matrix is not square.
    let _ = NaturalMatrix::from_str("[1, 2]")
        .unwrap()
        .mod_power_of_2_pow(2, 3);
}

#[test]
#[should_panic]
fn mod_power_of_2_pow_fail_2() {
    // The matrix is not reduced, even though the exponent is 1.
    let _ = NaturalMatrix::from_str("[8]")
        .unwrap()
        .mod_power_of_2_pow(1, 3);
}

#[test]
#[should_panic]
fn mod_power_of_2_pow_ref_fail() {
    let _ = (&NaturalMatrix::from_str("[]_{0,2}").unwrap()).mod_power_of_2_pow(0, 3);
}

#[test]
#[should_panic]
fn mod_power_of_2_pow_assign_fail() {
    let mut a = NaturalMatrix::from_str("[1; 2]").unwrap();
    a.mod_power_of_2_pow_assign(1, 3);
}

#[test]
fn mod_power_of_2_pow_properties() {
    natural_matrix_unsigned_unsigned_triple_gen_var_5().test_properties(|(a, exp, pow)| {
        let b = (&a).mod_power_of_2_pow(exp, pow);
        assert!(b.is_valid());
        assert_eq!((b.height(), b.width()), (a.height(), a.width()));
        assert_eq!(a.clone().mod_power_of_2_pow(exp, pow), b);
        let mut c = a.clone();
        c.mod_power_of_2_pow_assign(exp, pow);
        assert_eq!(c, b);
        assert_eq!(natural_matrix_mod_power_of_2_pow_naive(&a, exp, pow), b);
        // The power of the transpose is the transpose of the power.
        assert_eq!(a.transpose().mod_power_of_2_pow(exp, pow), b.transpose());
        if let Some(exp_plus_1) = exp.checked_add(1) {
            // One more factor multiplies the power by the matrix, on either side.
            let c = (&a).mod_power_of_2_pow(exp_plus_1, pow);
            assert_eq!((&b).mod_power_of_2_mul(&a, pow), c);
            assert_eq!((&a).mod_power_of_2_mul(&b, pow), c);
        }
        // When 2^k fits in a limb, the power agrees with that of an `UnsignedMatrix`.
        if pow <= Limb::WIDTH {
            let a_limbs = UnsignedMatrix::<Limb>::try_from(&a).unwrap();
            assert_eq!(NaturalMatrix::from(a_limbs.mod_power_of_2_pow(exp, pow)), b);
        }
    });

    natural_matrix_unsigned_pair_gen_var_6().test_properties(|(a, pow)| {
        assert_eq!(
            (&a).mod_power_of_2_pow(0, pow),
            NaturalMatrix::identity(a.height()).mod_power_of_2(pow)
        );
        assert_eq!((&a).mod_power_of_2_pow(1, pow), a);
        assert_eq!(
            (&a).mod_power_of_2_pow(2, pow),
            (&a).mod_power_of_2_square(pow)
        );
    });
}
