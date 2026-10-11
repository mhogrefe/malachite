// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{
    ModPowerOf2Add, ModPowerOf2AddMul, ModPowerOf2AddMulShl, ModPowerOf2AddMulShlAssign,
    ModPowerOf2IsReduced, ModPowerOf2Mul, ModPowerOf2Neg, ModPowerOf2Shl, ModPowerOf2SubMulShl,
};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::test_util::generators::unsigned_quintuple_gen_var_1;
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use malachite_nz::test_util::generators::{
    natural_natural_natural_unsigned_quadruple_gen_var_2,
    natural_natural_natural_unsigned_unsigned_quintuple_gen_var_1,
};
use malachite_nz::test_util::natural::arithmetic::mod_power_of_2_add_mul_shl::*;

#[test]
fn test_mod_power_of_2_add_mul_shl() {
    let test = |s, t, u, bits, pow, out| {
        let x = Natural::from_str(s).unwrap();
        let y = Natural::from_str(t).unwrap();
        let z = Natural::from_str(u).unwrap();
        let w = (&x).mod_power_of_2_add_mul_shl(&y, &z, bits, pow);
        assert!(w.is_valid());
        assert_eq!(w.to_string(), out);
        assert_eq!(
            x.clone()
                .mod_power_of_2_add_mul_shl(y.clone(), z.clone(), bits, pow),
            w
        );
        assert_eq!(
            x.clone()
                .mod_power_of_2_add_mul_shl(y.clone(), &z, bits, pow),
            w
        );
        assert_eq!(
            x.clone()
                .mod_power_of_2_add_mul_shl(&y, z.clone(), bits, pow),
            w
        );
        assert_eq!(x.clone().mod_power_of_2_add_mul_shl(&y, &z, bits, pow), w);
        let mut v = x.clone();
        v.mod_power_of_2_add_mul_shl_assign(y.clone(), z.clone(), bits, pow);
        assert!(v.is_valid());
        assert_eq!(v, w);
        let mut v = x.clone();
        v.mod_power_of_2_add_mul_shl_assign(y.clone(), &z, bits, pow);
        assert!(v.is_valid());
        assert_eq!(v, w);
        let mut v = x.clone();
        v.mod_power_of_2_add_mul_shl_assign(&y, z.clone(), bits, pow);
        assert!(v.is_valid());
        assert_eq!(v, w);
        let mut v = x.clone();
        v.mod_power_of_2_add_mul_shl_assign(&y, &z, bits, pow);
        assert!(v.is_valid());
        assert_eq!(v, w);
        assert_eq!(mod_power_of_2_add_mul_shl_naive(&x, &y, &z, bits, pow), w);
    };
    test("0", "0", "0", 0, 0, "0");
    test("3", "2", "5", 1, 5, "23");
    test("10", "14", "3", 2, 6, "50");
    test("7", "1", "1", 8, 8, "7");
    test("7", "5", "3", 20, 10, "7");
    test(
        "100000000000000000000",
        "100000000000000000000",
        "1000000000000",
        5,
        100,
        "449885024048990622321109630976",
    );
    test(
        "1267650600228229401496703205375",
        "1267650600228229401496703205375",
        "1267650600228229401496703205375",
        99,
        100,
        "633825300114114700748351602687",
    );
    test(
        "18446744073709551615",
        "18446744073709551615",
        "18446744073709551615",
        0,
        64,
        "0",
    );
    test(
        "18446744073709551616",
        "3",
        "9223372036854775808",
        1,
        65,
        "0",
    );
}

#[test]
#[should_panic]
fn mod_power_of_2_add_mul_shl_fail_1() {
    Natural::ONE.mod_power_of_2_add_mul_shl(Natural::ZERO, Natural::ZERO, 0, 0);
}

#[test]
#[should_panic]
fn mod_power_of_2_add_mul_shl_fail_2() {
    Natural::ZERO.mod_power_of_2_add_mul_shl(Natural::ONE, Natural::ZERO, 0, 0);
}

#[test]
#[should_panic]
fn mod_power_of_2_add_mul_shl_fail_3() {
    Natural::ZERO.mod_power_of_2_add_mul_shl(Natural::ZERO, Natural::ONE, 0, 0);
}

#[test]
#[should_panic]
fn mod_power_of_2_add_mul_shl_ref_fail() {
    (&Natural::ZERO).mod_power_of_2_add_mul_shl(&Natural::ONE, &Natural::from(8u32), 1, 3);
}

#[test]
#[should_panic]
fn mod_power_of_2_add_mul_shl_assign_fail() {
    let mut x = Natural::from(8u32);
    x.mod_power_of_2_add_mul_shl_assign(Natural::ONE, Natural::ONE, 1, 3);
}

#[test]
#[should_panic]
fn mod_power_of_2_add_mul_shl_assign_ref_fail() {
    let mut x = Natural::ZERO;
    x.mod_power_of_2_add_mul_shl_assign(&Natural::from(8u32), &Natural::ONE, 1, 3);
}

#[test]
fn mod_power_of_2_add_mul_shl_properties() {
    natural_natural_natural_unsigned_unsigned_quintuple_gen_var_1().test_properties(
        |(x, y, z, bits, pow)| {
            let w = (&x).mod_power_of_2_add_mul_shl(&y, &z, bits, pow);
            assert!(w.is_valid());
            assert!(w.mod_power_of_2_is_reduced(pow));
            // The forms agree.
            assert_eq!(
                x.clone()
                    .mod_power_of_2_add_mul_shl(y.clone(), z.clone(), bits, pow),
                w
            );
            assert_eq!(
                x.clone()
                    .mod_power_of_2_add_mul_shl(y.clone(), &z, bits, pow),
                w
            );
            assert_eq!(
                x.clone()
                    .mod_power_of_2_add_mul_shl(&y, z.clone(), bits, pow),
                w
            );
            assert_eq!(x.clone().mod_power_of_2_add_mul_shl(&y, &z, bits, pow), w);
            let mut v = x.clone();
            v.mod_power_of_2_add_mul_shl_assign(y.clone(), z.clone(), bits, pow);
            assert_eq!(v, w);
            let mut v = x.clone();
            v.mod_power_of_2_add_mul_shl_assign(y.clone(), &z, bits, pow);
            assert_eq!(v, w);
            let mut v = x.clone();
            v.mod_power_of_2_add_mul_shl_assign(&y, z.clone(), bits, pow);
            assert_eq!(v, w);
            let mut v = x.clone();
            v.mod_power_of_2_add_mul_shl_assign(&y, &z, bits, pow);
            assert_eq!(v, w);

            // It agrees with exact arithmetic reduced once, and with the unfused combination, and
            // the factors commute.
            assert_eq!(mod_power_of_2_add_mul_shl_naive(&x, &y, &z, bits, pow), w);
            assert_eq!(
                (&x).mod_power_of_2_add(
                    (&y).mod_power_of_2_mul(&z, pow)
                        .mod_power_of_2_shl(bits, pow),
                    pow
                ),
                w
            );
            assert_eq!((&x).mod_power_of_2_add_mul_shl(&z, &y, bits, pow), w);
            // Negating a factor gives the opposite operation, which undoes this one.
            assert_eq!(
                (&x).mod_power_of_2_sub_mul_shl(&(&y).mod_power_of_2_neg(pow), &z, bits, pow),
                w
            );
            assert_eq!((&w).mod_power_of_2_sub_mul_shl(&y, &z, bits, pow), x);
            // Negating everything negates the result.
            assert_eq!(
                (&x).mod_power_of_2_neg(pow).mod_power_of_2_add_mul_shl(
                    (&y).mod_power_of_2_neg(pow),
                    z.clone(),
                    bits,
                    pow
                ),
                (&w).mod_power_of_2_neg(pow)
            );
            // Shifts compose.
            assert_eq!(
                (&x).mod_power_of_2_add_mul_shl(&(&y).mod_power_of_2_shl(1u64, pow), &z, bits, pow),
                (&x).mod_power_of_2_add_mul_shl(&y, &z, bits + 1, pow)
            );
            // Shifting by at least `pow` changes nothing.
            if bits >= pow {
                assert_eq!(w, x);
            }
        },
    );

    natural_natural_natural_unsigned_quadruple_gen_var_2().test_properties(|(x, y, z, pow)| {
        // With no shift, this is `mod_power_of_2_add_mul`.
        assert_eq!(
            (&x).mod_power_of_2_add_mul_shl(&y, &z, 0, pow),
            (&x).mod_power_of_2_add_mul(&y, &z, pow)
        );
        // Shifting by `pow` changes nothing.
        assert_eq!((&x).mod_power_of_2_add_mul_shl(&y, &z, pow, pow), x);
    });

    unsigned_quintuple_gen_var_1::<Limb>().test_properties(|(x, y, z, bits, pow)| {
        // It agrees with the primitive implementation.
        assert_eq!(
            Natural::from(x).mod_power_of_2_add_mul_shl(
                Natural::from(y),
                Natural::from(z),
                bits,
                pow
            ),
            x.mod_power_of_2_add_mul_shl(y, z, bits, pow)
        );
    });
}
