// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{
    ModPowerOf2Add, ModPowerOf2AddMul, ModPowerOf2AddMulAssign, ModPowerOf2IsReduced,
    ModPowerOf2Mul, ModPowerOf2Neg, ModPowerOf2Square, ModPowerOf2SubMul,
};
use malachite_base::num::basic::traits::{One, Two, Zero};
use malachite_base::test_util::generators::unsigned_quadruple_gen_var_3;
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use malachite_nz::test_util::generators::{
    natural_natural_natural_unsigned_quadruple_gen_var_2,
    natural_natural_unsigned_triple_gen_var_4, natural_unsigned_pair_gen_var_11,
};
use malachite_nz::test_util::natural::arithmetic::mod_power_of_2_add_mul::*;

#[test]
fn test_mod_power_of_2_add_mul() {
    let test = |s, t, u, pow, out| {
        let x = Natural::from_str(s).unwrap();
        let y = Natural::from_str(t).unwrap();
        let z = Natural::from_str(u).unwrap();
        let w = (&x).mod_power_of_2_add_mul(&y, &z, pow);
        assert!(w.is_valid());
        assert_eq!(w.to_string(), out);
        assert_eq!(
            x.clone().mod_power_of_2_add_mul(y.clone(), z.clone(), pow),
            w
        );
        assert_eq!(x.clone().mod_power_of_2_add_mul(y.clone(), &z, pow), w);
        assert_eq!(x.clone().mod_power_of_2_add_mul(&y, z.clone(), pow), w);
        assert_eq!(x.clone().mod_power_of_2_add_mul(&y, &z, pow), w);
        let mut v = x.clone();
        v.mod_power_of_2_add_mul_assign(y.clone(), z.clone(), pow);
        assert_eq!(v, w);
        let mut v = x.clone();
        v.mod_power_of_2_add_mul_assign(y.clone(), &z, pow);
        assert_eq!(v, w);
        let mut v = x.clone();
        v.mod_power_of_2_add_mul_assign(&y, z.clone(), pow);
        assert_eq!(v, w);
        let mut v = x.clone();
        v.mod_power_of_2_add_mul_assign(&y, &z, pow);
        assert!(v.is_valid());
        assert_eq!(v, w);
        assert_eq!(mod_power_of_2_add_mul_naive(&x, &y, &z, pow), w);
    };
    test("0", "0", "0", 0, "0");
    test("1", "0", "1", 1, "1");
    test("1", "1", "1", 1, "0");
    test("3", "2", "5", 5, "13");
    test("13", "2", "5", 5, "23");
    test("10", "14", "3", 4, "4");
    test("200", "100", "3", 8, "244");
    test(
        "100000000000000000000",
        "100000000000000000000",
        "1000000000000",
        100,
        "1123253182298106683257149980672",
    );
    test(
        "1267650600228229401496703205375",
        "1267650600228229401496703205375",
        "1267650600228229401496703205375",
        100,
        "0",
    );
    test(
        "18446744073709551615",
        "18446744073709551615",
        "18446744073709551615",
        64,
        "0",
    );
    test(
        "18446744073709551616",
        "3",
        "9223372036854775808",
        65,
        "9223372036854775808",
    );
}

#[test]
#[should_panic]
fn mod_power_of_2_add_mul_fail_1() {
    Natural::ONE.mod_power_of_2_add_mul(Natural::ZERO, Natural::ZERO, 0);
}

#[test]
#[should_panic]
fn mod_power_of_2_add_mul_fail_2() {
    Natural::ZERO.mod_power_of_2_add_mul(Natural::ONE, Natural::ZERO, 0);
}

#[test]
#[should_panic]
fn mod_power_of_2_add_mul_fail_3() {
    Natural::ZERO.mod_power_of_2_add_mul(Natural::ZERO, Natural::ONE, 0);
}

#[test]
#[should_panic]
fn mod_power_of_2_add_mul_fail_4() {
    Natural::ONE.mod_power_of_2_add_mul(
        Natural::ONE,
        Natural::from_str("1267650600228229401496703205376").unwrap(),
        100,
    );
}

#[test]
#[should_panic]
fn mod_power_of_2_add_mul_ref_fail() {
    (&Natural::ZERO).mod_power_of_2_add_mul(&Natural::ONE, &Natural::TWO, 1);
}

#[test]
#[should_panic]
fn mod_power_of_2_add_mul_assign_fail() {
    let mut x = Natural::TWO;
    x.mod_power_of_2_add_mul_assign(Natural::ONE, Natural::ONE, 1);
}

#[test]
#[should_panic]
fn mod_power_of_2_add_mul_assign_ref_fail() {
    let mut x = Natural::ZERO;
    x.mod_power_of_2_add_mul_assign(
        &Natural::ONE,
        &Natural::from_str("1267650600228229401496703205376").unwrap(),
        100,
    );
}

#[test]
fn mod_power_of_2_add_mul_properties() {
    natural_natural_natural_unsigned_quadruple_gen_var_2().test_properties(|(x, y, z, pow)| {
        assert!(x.mod_power_of_2_is_reduced(pow));
        assert!(y.mod_power_of_2_is_reduced(pow));
        assert!(z.mod_power_of_2_is_reduced(pow));
        let result = (&x).mod_power_of_2_add_mul(&y, &z, pow);
        assert!(result.is_valid());
        assert!(result.mod_power_of_2_is_reduced(pow));

        // The forms agree.
        assert_eq!(
            x.clone().mod_power_of_2_add_mul(y.clone(), z.clone(), pow),
            result
        );
        assert_eq!(x.clone().mod_power_of_2_add_mul(y.clone(), &z, pow), result);
        assert_eq!(x.clone().mod_power_of_2_add_mul(&y, z.clone(), pow), result);
        assert_eq!(x.clone().mod_power_of_2_add_mul(&y, &z, pow), result);
        let mut v = x.clone();
        v.mod_power_of_2_add_mul_assign(y.clone(), z.clone(), pow);
        assert_eq!(v, result);
        let mut v = x.clone();
        v.mod_power_of_2_add_mul_assign(y.clone(), &z, pow);
        assert_eq!(v, result);
        let mut v = x.clone();
        v.mod_power_of_2_add_mul_assign(&y, z.clone(), pow);
        assert_eq!(v, result);
        let mut v = x.clone();
        v.mod_power_of_2_add_mul_assign(&y, &z, pow);
        assert!(v.is_valid());
        assert_eq!(v, result);

        // It agrees with exact arithmetic reduced once, and with the unfused combination, and the
        // factors commute.
        assert_eq!(mod_power_of_2_add_mul_naive(&x, &y, &z, pow), result);
        assert_eq!(
            (&x).mod_power_of_2_add((&y).mod_power_of_2_mul(&z, pow), pow),
            result
        );
        assert_eq!((&x).mod_power_of_2_add_mul(&z, &y, pow), result);
        // Negating a factor gives the opposite operation, which undoes this one.
        assert_eq!(
            (&x).mod_power_of_2_sub_mul(&(&y).mod_power_of_2_neg(pow), &z, pow),
            result
        );
        assert_eq!((&result).mod_power_of_2_sub_mul(&y, &z, pow), x);
        // Negating everything negates the result.
        assert_eq!(
            (&x).mod_power_of_2_neg(pow).mod_power_of_2_add_mul(
                (&y).mod_power_of_2_neg(pow),
                z,
                pow
            ),
            result.mod_power_of_2_neg(pow)
        );
    });

    natural_natural_unsigned_triple_gen_var_4().test_properties(|(x, y, pow)| {
        // A zero factor changes nothing, and a factor of 1 leaves the sum.
        assert_eq!((&x).mod_power_of_2_add_mul(&y, &Natural::ZERO, pow), x);
        assert_eq!((&x).mod_power_of_2_add_mul(&Natural::ZERO, &y, pow), x);
        if pow != 0 {
            assert_eq!(
                (&x).mod_power_of_2_add_mul(&y, &Natural::ONE, pow),
                (&x).mod_power_of_2_add(&y, pow)
            );
        }
    });

    natural_unsigned_pair_gen_var_11().test_properties(|(x, pow)| {
        // Starting from zero gives the square.
        assert_eq!(
            Natural::ZERO.mod_power_of_2_add_mul(&x, &x, pow),
            (&x).mod_power_of_2_square(pow)
        );
    });

    unsigned_quadruple_gen_var_3::<Limb>().test_properties(|(x, y, z, pow)| {
        // It agrees with the primitive implementation.
        assert_eq!(
            Natural::from(x).mod_power_of_2_add_mul(Natural::from(y), Natural::from(z), pow),
            x.mod_power_of_2_add_mul(y, z, pow)
        );
    });
}
