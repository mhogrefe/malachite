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
    IsPowerOf2, ModIsReduced, ModMul, ModPowerOf2Square, ModSquare, ModSquareAssign, Square,
};
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::test_util::generators::unsigned_matrix_unsigned_pair_gen_var_8;
use malachite_nz::natural::Natural;
use malachite_nz::natural_matrix::NaturalMatrix;
use malachite_nz::test_util::generators::natural_matrix_natural_pair_gen_var_3;
use malachite_nz::test_util::natural_matrix::arithmetic::mod_square::*;

#[test]
fn test_mod_square() {
    let test = |s, m, out| {
        let a = NaturalMatrix::from_str(s).unwrap();
        let m = Natural::from_str(m).unwrap();
        let b = a.clone().mod_square(m.clone());
        assert!(b.is_valid());
        assert_eq!(b.to_string(), out);
        assert_eq!(a.clone().mod_square(&m), b);
        assert_eq!((&a).mod_square(m.clone()), b);
        assert_eq!((&a).mod_square(&m), b);
        let mut c = a.clone();
        c.mod_square_assign(m.clone());
        assert!(c.is_valid());
        assert_eq!(c, b);
        let mut c = a.clone();
        c.mod_square_assign(&m);
        assert_eq!(c, b);
        assert_eq!(natural_matrix_mod_square_naive(&a, &m), b);
    };
    test("[]_{0,0}", "3", "[]_{0,0}");
    test("[0, 0; 0, 0]", "1", "[0, 0; 0, 0]");
    test("[5]", "7", "[4]");
    test("[1, 2; 3, 4]", "9", "[7, 1; 6, 4]");
    test(
        "[2, 0, 1; 1, 3, 0; 0, 1, 1]",
        "5",
        "[4, 1, 3; 0, 4, 1; 1, 4, 1]",
    );
    test("[254, 253; 252, 251]", "255", "[7, 10; 15, 22]");
    test(
        "[18446744073709551614, 9223372036854775808; 5, 18446744073709551613]",
        "18446744073709551615",
        "[9223372036854775811, 9223372036854775806; 18446744073709551600, 9223372036854775814]",
    );
    test(
        "[18446744073709551615, 9223372036854775808; 5, 18446744073709551613]",
        "18446744073709551616",
        "[9223372036854775809, 0; 18446744073709551596, 9223372036854775817]",
    );
    test(
        "[18446744073709551621, 3, 10000000000000000000; 1, 36893488147419103232, 0; 7, 8, 3689348\
        8147419103244]",
        "36893488147419103245",
        "[23883139815726120949, 24659767778871345089, 11893488147419103245; 18446744073709551608, \
        172, 10000000000000000000; 18446744073709551613, 36893488147419103154, 3310651185258089675\
        6]",
    );
    test(
        "[100000000000000000000000000000, 3; 7, 100000000000000000000000000001]",
        "1000000000000000000000000000000",
        "[21, 600000000000000000000000000003; 400000000000000000000000000007, 20000000000000000000\
        0000000022]",
    );
    test(
        "[170141183460469231731687303715884105728, 1; 3, 170141183460469231731687303715884105739]",
        "340282366920938463463374607431768211457",
        "[255211775190703847597530955573826158596, 10; 30, 255211775190703847597530955573826158706\
        ]",
    );
}

#[test]
#[should_panic]
fn mod_square_fail_1() {
    // The modulus is zero.
    let _ = NaturalMatrix::from_str("[]_{0,0}")
        .unwrap()
        .mod_square(Natural::ZERO);
}

#[test]
#[should_panic]
fn mod_square_fail_2() {
    // The matrix is not square.
    let _ = NaturalMatrix::from_str("[1, 2]")
        .unwrap()
        .mod_square(Natural::from(3u32));
}

#[test]
#[should_panic]
fn mod_square_fail_3() {
    // The matrix is not reduced.
    let _ = NaturalMatrix::from_str("[7]")
        .unwrap()
        .mod_square(Natural::from(7u32));
}

#[test]
#[should_panic]
fn mod_square_ref_fail() {
    // The matrix is not reduced, and the modulus does not fit in a limb.
    let _ = (&NaturalMatrix::from_str("[1000000000000000000000000000000]").unwrap())
        .mod_square(&Natural::from_str("1000000000000000000000000000000").unwrap());
}

#[test]
#[should_panic]
fn mod_square_assign_fail() {
    let mut a = NaturalMatrix::from_str("[1; 2]").unwrap();
    a.mod_square_assign(Natural::from(3u32));
}

#[test]
fn mod_square_properties() {
    natural_matrix_natural_pair_gen_var_3().test_properties(|(a, m)| {
        let b = (&a).mod_square(&m);
        assert!(b.is_valid());
        assert_eq!((b.height(), b.width()), (a.height(), a.width()));
        assert!(b.mod_is_reduced(&m));
        assert_eq!(a.clone().mod_square(m.clone()), b);
        assert_eq!(a.clone().mod_square(&m), b);
        assert_eq!((&a).mod_square(m.clone()), b);
        let mut c = a.clone();
        c.mod_square_assign(m.clone());
        assert_eq!(c, b);
        let mut c = a.clone();
        c.mod_square_assign(&m);
        assert_eq!(c, b);
        assert_eq!(natural_matrix_mod_square_naive(&a, &m), b);
        assert_eq!((&a).mod_mul(&a, &m), b);
        // The square modulo m is the exact square, reduced.
        assert_eq!((&a).square() % &m, b);
        // The square of the transpose is the transpose of the square.
        assert_eq!(a.transpose().mod_square(&m), b.transpose());
        // When m is a power of 2, the square agrees with the square modulo a power of 2.
        if m.is_power_of_2() {
            assert_eq!(a.mod_power_of_2_square(m.significant_bits() - 1), b);
        }
    });

    unsigned_matrix_unsigned_pair_gen_var_8().test_properties(|(a, m)| {
        // The `u64` elements square as their `Natural` counterparts do.
        assert_eq!(
            NaturalMatrix::from(a.clone()).mod_square(Natural::from(m)),
            NaturalMatrix::from(a.mod_square(m))
        );
    });
}
