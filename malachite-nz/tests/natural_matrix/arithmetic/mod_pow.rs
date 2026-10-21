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
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::test_util::generators::unsigned_matrix_unsigned_unsigned_triple_gen_var_9;
use malachite_nz::natural::Natural;
use malachite_nz::natural_matrix::NaturalMatrix;
use malachite_nz::test_util::generators::{
    natural_matrix_natural_pair_gen_var_3, natural_matrix_unsigned_natural_triple_gen_var_2,
};
use malachite_nz::test_util::natural_matrix::arithmetic::mod_pow::*;

#[test]
fn test_mod_pow() {
    let test = |s, exp, m, out| {
        let a = NaturalMatrix::from_str(s).unwrap();
        let m = Natural::from_str(m).unwrap();
        let b = a.clone().mod_pow(exp, m.clone());
        assert!(b.is_valid());
        assert_eq!(b.to_string(), out);
        assert_eq!(a.clone().mod_pow(exp, &m), b);
        assert_eq!((&a).mod_pow(exp, m.clone()), b);
        assert_eq!((&a).mod_pow(exp, &m), b);
        let mut c = a.clone();
        c.mod_pow_assign(exp, m.clone());
        assert!(c.is_valid());
        assert_eq!(c, b);
        let mut c = a.clone();
        c.mod_pow_assign(exp, &m);
        assert_eq!(c, b);
        assert_eq!(natural_matrix_mod_pow_naive(&a, exp, &m), b);
    };
    test("[]_{0,0}", 0, "3", "[]_{0,0}");
    test("[0, 0; 0, 0]", 0, "1", "[0, 0; 0, 0]");
    test("[5]", 0, "7", "[1]");
    test("[5]", 6, "7", "[1]");
    test("[1, 1; 1, 0]", 10, "100", "[89, 55; 55, 34]");
    test(
        "[2, 0, 1; 1, 3, 0; 0, 1, 1]",
        4,
        "7",
        "[3, 4, 2; 3, 6, 4; 4, 6, 1]",
    );
    test("[0, 1; 0, 0]", 100, "5", "[0, 0; 0, 0]");
    test(
        "[1, 1; 1, 0]",
        1000,
        "18446744073709551615",
        "[1965850429854679336, 6604850919967101945; 6604850919967101945, 13807743583597129006]",
    );
    test(
        "[1, 1; 1, 0]",
        1000,
        "18446744073709551616",
        "[9079565065540428013, 817770325994397771; 817770325994397771, 8261794739546030242]",
    );
    test("[1, 2; 3, 4]", 0, "100000000000000000000", "[1, 0; 0, 1]");
    test(
        "[1, 1; 1, 0]",
        100,
        "100000000000000000000",
        "[73147844013817084101, 54224848179261915075; 54224848179261915075, 18922995834555169026]",
    );
    test(
        "[18446744073709551621, 3, 10000000000000000000; 1, 36893488147419103232, 0; 7, 8, 3689348\
        8147419103244]",
        9,
        "36893488147419103245",
        "[29156977653101492947, 16372012617656044250, 16798908331414990910; 22898615315029647335, \
        9908754484142014072, 25741322963062603120; 19074344230430892373, 19697972777148307085, 143\
        46543474910213439]",
    );
    test(
        "[0, 1267650600228229401496703205376; 0, 0]",
        2,
        "340282366920938463463374607431768211457",
        "[0, 0; 0, 0]",
    );
    test(
        "[170141183460469231731687303715884105728, 1; 3, 170141183460469231731687303715884105739]",
        33,
        "340282366920938463463374607431768211457",
        "[174053754435147848839344272012472702356, 47953911460480253542406107187776236942; 1438617\
        34381440760627218321563328710826, 20982046658553710879062236214474885804]",
    );
}

#[test]
#[should_panic]
fn mod_pow_fail_1() {
    // The modulus is zero.
    let _ = NaturalMatrix::from_str("[]_{0,0}")
        .unwrap()
        .mod_pow(2, Natural::ZERO);
}

#[test]
#[should_panic]
fn mod_pow_fail_2() {
    // The matrix is not square.
    let _ = NaturalMatrix::from_str("[1, 2]")
        .unwrap()
        .mod_pow(2, Natural::from(3u32));
}

#[test]
#[should_panic]
fn mod_pow_fail_3() {
    // The matrix is not reduced, and the exponent is 1.
    let _ = NaturalMatrix::from_str("[7]")
        .unwrap()
        .mod_pow(1, Natural::from(7u32));
}

#[test]
#[should_panic]
fn mod_pow_ref_fail() {
    // The matrix is not reduced, and the modulus does not fit in a limb.
    let _ = (&NaturalMatrix::from_str("[1000000000000000000000000000000]").unwrap()).mod_pow(
        0,
        &Natural::from_str("1000000000000000000000000000000").unwrap(),
    );
}

#[test]
#[should_panic]
fn mod_pow_assign_fail() {
    // The matrix is not square, and the exponent is 1.
    let mut a = NaturalMatrix::from_str("[1; 2]").unwrap();
    a.mod_pow_assign(1, Natural::from(3u32));
}

#[test]
fn mod_pow_properties() {
    natural_matrix_unsigned_natural_triple_gen_var_2().test_properties(|(a, exp, m)| {
        let b = (&a).mod_pow(exp, &m);
        assert!(b.is_valid());
        assert_eq!((b.height(), b.width()), (a.height(), a.width()));
        assert!(b.mod_is_reduced(&m));
        assert_eq!(a.clone().mod_pow(exp, m.clone()), b);
        assert_eq!(a.clone().mod_pow(exp, &m), b);
        assert_eq!((&a).mod_pow(exp, m.clone()), b);
        let mut c = a.clone();
        c.mod_pow_assign(exp, m.clone());
        assert_eq!(c, b);
        let mut c = a.clone();
        c.mod_pow_assign(exp, &m);
        assert_eq!(c, b);
        assert_eq!(natural_matrix_mod_pow_naive(&a, exp, &m), b);
        // The power of the transpose is the transpose of the power.
        assert_eq!(a.transpose().mod_pow(exp, &m), b.transpose());
        if let Some(exp_plus_1) = exp.checked_add(1) {
            // One more factor multiplies the power by the matrix, on either side.
            let c = (&a).mod_pow(exp_plus_1, &m);
            assert_eq!((&b).mod_mul(&a, &m), c);
            assert_eq!((&a).mod_mul(&b, &m), c);
        }
        // When m is a power of 2, the power agrees with the power modulo a power of 2.
        if m.is_power_of_2() {
            assert_eq!(a.mod_power_of_2_pow(exp, m.significant_bits() - 1), b);
        }
    });

    natural_matrix_natural_pair_gen_var_3().test_properties(|(a, m)| {
        assert_eq!(
            (&a).mod_pow(0, &m),
            NaturalMatrix::identity(a.height()) % &m
        );
        assert_eq!((&a).mod_pow(1, &m), a);
        assert_eq!((&a).mod_pow(2, &m), (&a).mod_square(&m));
    });

    unsigned_matrix_unsigned_unsigned_triple_gen_var_9().test_properties(|(a, exp, m)| {
        // The `u64` elements are raised to powers as their `Natural` counterparts are.
        assert_eq!(
            NaturalMatrix::from(a.clone()).mod_pow(exp, Natural::from(m)),
            NaturalMatrix::from(a.mod_pow(exp, m))
        );
    });
}
