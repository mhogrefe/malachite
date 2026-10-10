// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{
    ModAddMul, ModIsReduced, ModMul, ModNeg, ModSquare, ModSub, ModSubMul, ModSubMulAssign,
};
use malachite_base::num::basic::traits::{One, Two, Zero};
use malachite_base::test_util::generators::unsigned_quadruple_gen_var_4;
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use malachite_nz::test_util::generators::{
    natural_pair_gen_var_8, natural_quadruple_gen_var_1, natural_triple_gen_var_3,
};
use malachite_nz::test_util::natural::arithmetic::mod_sub_mul::mod_sub_mul_naive;

#[test]
fn test_mod_sub_mul() {
    let test = |s, t, u, v, out| {
        let x = Natural::from_str(s).unwrap();
        let y = Natural::from_str(t).unwrap();
        let z = Natural::from_str(u).unwrap();
        let m = Natural::from_str(v).unwrap();
        let w = (&x).mod_sub_mul(&y, &z, &m);
        assert!(w.is_valid());
        assert_eq!(w.to_string(), out);
        assert_eq!(x.clone().mod_sub_mul(y.clone(), z.clone(), m.clone()), w);
        assert_eq!(x.clone().mod_sub_mul(y.clone(), z.clone(), &m), w);
        assert_eq!(x.clone().mod_sub_mul(y.clone(), &z, m.clone()), w);
        assert_eq!(x.clone().mod_sub_mul(y.clone(), &z, &m), w);
        assert_eq!(x.clone().mod_sub_mul(&y, z.clone(), m.clone()), w);
        assert_eq!(x.clone().mod_sub_mul(&y, z.clone(), &m), w);
        assert_eq!(x.clone().mod_sub_mul(&y, &z, m.clone()), w);
        assert_eq!(x.clone().mod_sub_mul(&y, &z, &m), w);
        let mut v = x.clone();
        v.mod_sub_mul_assign(y.clone(), z.clone(), m.clone());
        assert!(v.is_valid());
        assert_eq!(v, w);
        let mut v = x.clone();
        v.mod_sub_mul_assign(y.clone(), z.clone(), &m);
        assert!(v.is_valid());
        assert_eq!(v, w);
        let mut v = x.clone();
        v.mod_sub_mul_assign(y.clone(), &z, m.clone());
        assert!(v.is_valid());
        assert_eq!(v, w);
        let mut v = x.clone();
        v.mod_sub_mul_assign(y.clone(), &z, &m);
        assert!(v.is_valid());
        assert_eq!(v, w);
        let mut v = x.clone();
        v.mod_sub_mul_assign(&y, z.clone(), m.clone());
        assert!(v.is_valid());
        assert_eq!(v, w);
        let mut v = x.clone();
        v.mod_sub_mul_assign(&y, z.clone(), &m);
        assert!(v.is_valid());
        assert_eq!(v, w);
        let mut v = x.clone();
        v.mod_sub_mul_assign(&y, &z, m.clone());
        assert!(v.is_valid());
        assert_eq!(v, w);
        let mut v = x.clone();
        v.mod_sub_mul_assign(&y, &z, &m);
        assert!(v.is_valid());
        assert_eq!(v, w);
        assert_eq!(mod_sub_mul_naive(&x, &y, &z, &m), w);
    };
    test("0", "0", "0", "1", "0");
    test("3", "2", "5", "7", "0");
    test("6", "2", "5", "7", "3");
    test("10", "14", "3", "15", "13");
    test("200", "100", "3", "255", "155");
    test(
        "100000000000000000000",
        "100000000000000000000",
        "1000000000000",
        "1000000000000000000000000000057",
        "100000000000000005700",
    );
    test(
        "18446744073709551614",
        "18446744073709551614",
        "18446744073709551614",
        "18446744073709551615",
        "18446744073709551613",
    );
    test(
        "18446744073709551616",
        "18446744073709551615",
        "18446744073709551621",
        "36893488147419103232",
        "18446744073709551621",
    );
    test(
        "1267650600228229401496703205374",
        "1267650600228229401496703205373",
        "1267650600228229401496703205372",
        "1267650600228229401496703205375",
        "1267650600228229401496703205368",
    );
    test(
        "0",
        "1",
        "1",
        "10000000000000000000000000000000000000000",
        "9999999999999999999999999999999999999999",
    );
}

#[test]
#[should_panic]
fn mod_sub_mul_fail_1() {
    Natural::ZERO.mod_sub_mul(Natural::ZERO, Natural::ZERO, Natural::ZERO);
}

#[test]
#[should_panic]
fn mod_sub_mul_fail_2() {
    Natural::from(7u32).mod_sub_mul(Natural::ONE, Natural::ONE, Natural::from(7u32));
}

#[test]
#[should_panic]
fn mod_sub_mul_fail_3() {
    Natural::ONE.mod_sub_mul(Natural::from(7u32), Natural::ONE, Natural::from(7u32));
}

#[test]
#[should_panic]
fn mod_sub_mul_fail_4() {
    Natural::ONE.mod_sub_mul(
        Natural::ONE,
        Natural::from_str("1000000000000000000000000000000").unwrap(),
        Natural::from_str("1000000000000000000000000000000").unwrap(),
    );
}

#[test]
#[should_panic]
fn mod_sub_mul_ref_fail() {
    (&Natural::ZERO).mod_sub_mul(&Natural::ONE, &Natural::TWO, &Natural::TWO);
}

#[test]
#[should_panic]
fn mod_sub_mul_assign_fail() {
    let mut x = Natural::TWO;
    x.mod_sub_mul_assign(Natural::ONE, Natural::ONE, Natural::TWO);
}

#[test]
#[should_panic]
fn mod_sub_mul_assign_ref_fail() {
    let mut x = Natural::ZERO;
    let m = Natural::from_str("1000000000000000000000000000000").unwrap();
    x.mod_sub_mul_assign(&Natural::ONE, &m, &m);
}

#[test]
fn mod_sub_mul_properties() {
    natural_quadruple_gen_var_1().test_properties(|(x, y, z, m)| {
        assert!(x.mod_is_reduced(&m));
        assert!(y.mod_is_reduced(&m));
        assert!(z.mod_is_reduced(&m));
        let result = (&x).mod_sub_mul(&y, &z, &m);
        assert!(result.is_valid());
        assert!(result.mod_is_reduced(&m));

        // The forms agree.
        assert_eq!(
            x.clone().mod_sub_mul(y.clone(), z.clone(), m.clone()),
            result
        );
        assert_eq!(x.clone().mod_sub_mul(y.clone(), z.clone(), &m), result);
        assert_eq!(x.clone().mod_sub_mul(y.clone(), &z, m.clone()), result);
        assert_eq!(x.clone().mod_sub_mul(y.clone(), &z, &m), result);
        assert_eq!(x.clone().mod_sub_mul(&y, z.clone(), m.clone()), result);
        assert_eq!(x.clone().mod_sub_mul(&y, z.clone(), &m), result);
        assert_eq!(x.clone().mod_sub_mul(&y, &z, m.clone()), result);
        assert_eq!(x.clone().mod_sub_mul(&y, &z, &m), result);
        let mut v = x.clone();
        v.mod_sub_mul_assign(y.clone(), z.clone(), m.clone());
        assert!(v.is_valid());
        assert_eq!(v, result);
        let mut v = x.clone();
        v.mod_sub_mul_assign(y.clone(), z.clone(), &m);
        assert!(v.is_valid());
        assert_eq!(v, result);
        let mut v = x.clone();
        v.mod_sub_mul_assign(y.clone(), &z, m.clone());
        assert!(v.is_valid());
        assert_eq!(v, result);
        let mut v = x.clone();
        v.mod_sub_mul_assign(y.clone(), &z, &m);
        assert!(v.is_valid());
        assert_eq!(v, result);
        let mut v = x.clone();
        v.mod_sub_mul_assign(&y, z.clone(), m.clone());
        assert!(v.is_valid());
        assert_eq!(v, result);
        let mut v = x.clone();
        v.mod_sub_mul_assign(&y, z.clone(), &m);
        assert!(v.is_valid());
        assert_eq!(v, result);
        let mut v = x.clone();
        v.mod_sub_mul_assign(&y, &z, m.clone());
        assert!(v.is_valid());
        assert_eq!(v, result);
        let mut v = x.clone();
        v.mod_sub_mul_assign(&y, &z, &m);
        assert!(v.is_valid());
        assert_eq!(v, result);

        // It agrees with exact arithmetic reduced once, and with the unfused combination, and the
        // factors commute.
        assert_eq!(mod_sub_mul_naive(&x, &y, &z, &m), result);
        assert_eq!((&x).mod_sub((&y).mod_mul(&z, &m), &m), result);
        assert_eq!((&x).mod_sub_mul(&z, &y, &m), result);
        // Negating a factor gives the opposite operation, which undoes this one.
        assert_eq!((&x).mod_add_mul(&(&y).mod_neg(&m), &z, &m), result);
        assert_eq!((&result).mod_add_mul(&y, &z, &m), x);
        // Negating everything negates the result.
        assert_eq!(
            (&x).mod_neg(&m).mod_sub_mul((&y).mod_neg(&m), z, &m),
            result.mod_neg(&m)
        );
    });

    natural_triple_gen_var_3().test_properties(|(x, y, m)| {
        // A zero factor changes nothing, and a factor of 1 leaves the difference.
        assert_eq!((&x).mod_sub_mul(&y, &Natural::ZERO, &m), x);
        assert_eq!((&x).mod_sub_mul(&Natural::ZERO, &y, &m), x);
        if m != 1u32 {
            assert_eq!(
                (&x).mod_sub_mul(&y, &Natural::ONE, &m),
                (&x).mod_sub(&y, &m)
            );
        }
    });

    natural_pair_gen_var_8().test_properties(|(x, m)| {
        // Starting from zero gives the square, negated.
        assert_eq!(
            Natural::ZERO.mod_sub_mul(&x, &x, &m),
            (&x).mod_square(&m).mod_neg(&m)
        );
    });

    unsigned_quadruple_gen_var_4::<Limb>().test_properties(|(x, y, z, m)| {
        // It agrees with the primitive implementation.
        assert_eq!(
            Natural::from(x).mod_sub_mul(Natural::from(y), Natural::from(z), Natural::from(m)),
            x.mod_sub_mul(y, z, m)
        );
    });
}
