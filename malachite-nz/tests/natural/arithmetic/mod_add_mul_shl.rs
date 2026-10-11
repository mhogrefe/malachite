// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{
    ModAdd, ModAddMul, ModAddMulShl, ModAddMulShlAssign, ModIsReduced, ModMul, ModNeg, ModShl,
    ModSubMulShl,
};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::test_util::generators::unsigned_quintuple_gen_var_2;
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use malachite_nz::test_util::generators::{
    natural_natural_natural_unsigned_natural_quintuple_gen_var_1, natural_quadruple_gen_var_1,
};
use malachite_nz::test_util::natural::arithmetic::mod_add_mul_shl::mod_add_mul_shl_naive;

#[test]
fn test_mod_add_mul_shl() {
    let test = |s, t, u, bits, v, out| {
        let x = Natural::from_str(s).unwrap();
        let y = Natural::from_str(t).unwrap();
        let z = Natural::from_str(u).unwrap();
        let m = Natural::from_str(v).unwrap();
        let w = (&x).mod_add_mul_shl(&y, &z, bits, &m);
        assert!(w.is_valid());
        assert_eq!(w.to_string(), out);
        assert_eq!(
            x.clone()
                .mod_add_mul_shl(y.clone(), z.clone(), bits, m.clone()),
            w
        );
        assert_eq!(x.clone().mod_add_mul_shl(y.clone(), z.clone(), bits, &m), w);
        assert_eq!(x.clone().mod_add_mul_shl(y.clone(), &z, bits, m.clone()), w);
        assert_eq!(x.clone().mod_add_mul_shl(y.clone(), &z, bits, &m), w);
        assert_eq!(x.clone().mod_add_mul_shl(&y, z.clone(), bits, m.clone()), w);
        assert_eq!(x.clone().mod_add_mul_shl(&y, z.clone(), bits, &m), w);
        assert_eq!(x.clone().mod_add_mul_shl(&y, &z, bits, m.clone()), w);
        assert_eq!(x.clone().mod_add_mul_shl(&y, &z, bits, &m), w);
        let mut v = x.clone();
        v.mod_add_mul_shl_assign(y.clone(), z.clone(), bits, m.clone());
        assert!(v.is_valid());
        assert_eq!(v, w);
        let mut v = x.clone();
        v.mod_add_mul_shl_assign(y.clone(), z.clone(), bits, &m);
        assert!(v.is_valid());
        assert_eq!(v, w);
        let mut v = x.clone();
        v.mod_add_mul_shl_assign(y.clone(), &z, bits, m.clone());
        assert!(v.is_valid());
        assert_eq!(v, w);
        let mut v = x.clone();
        v.mod_add_mul_shl_assign(y.clone(), &z, bits, &m);
        assert!(v.is_valid());
        assert_eq!(v, w);
        let mut v = x.clone();
        v.mod_add_mul_shl_assign(&y, z.clone(), bits, m.clone());
        assert!(v.is_valid());
        assert_eq!(v, w);
        let mut v = x.clone();
        v.mod_add_mul_shl_assign(&y, z.clone(), bits, &m);
        assert!(v.is_valid());
        assert_eq!(v, w);
        let mut v = x.clone();
        v.mod_add_mul_shl_assign(&y, &z, bits, m.clone());
        assert!(v.is_valid());
        assert_eq!(v, w);
        let mut v = x.clone();
        v.mod_add_mul_shl_assign(&y, &z, bits, &m);
        assert!(v.is_valid());
        assert_eq!(v, w);
        assert_eq!(mod_add_mul_shl_naive(&x, &y, &z, bits, &m), w);
    };
    test("0", "0", "0", 0, "1", "0");
    test("3", "2", "5", 1, "7", "2");
    test("10", "14", "3", 2, "15", "13");
    test("200", "100", "3", 40, "255", "245");
    test(
        "100000000000000000000",
        "100000000000000000000",
        "1000000000000",
        5,
        "1000000000000000000000000000057",
        "99999999999999817600",
    );
    test(
        "18446744073709551614",
        "18446744073709551614",
        "18446744073709551614",
        64,
        "18446744073709551615",
        "0",
    );
    test(
        "1267650600228229401496703205374",
        "1267650600228229401496703205373",
        "1267650600228229401496703205372",
        200,
        "1267650600228229401496703205375",
        "5",
    );
    test(
        "0",
        "1",
        "1",
        1000,
        "10000000000000000000000000000000000000000",
        "6542167660429831652624386837205668069376",
    );
}

#[test]
#[should_panic]
fn mod_add_mul_shl_fail_1() {
    Natural::ZERO.mod_add_mul_shl(Natural::ZERO, Natural::ZERO, 0, Natural::ZERO);
}

#[test]
#[should_panic]
fn mod_add_mul_shl_fail_2() {
    Natural::from(7u32).mod_add_mul_shl(Natural::ONE, Natural::ONE, 1, Natural::from(7u32));
}

#[test]
#[should_panic]
fn mod_add_mul_shl_fail_3() {
    (&Natural::ONE).mod_add_mul_shl(&Natural::from(7u32), &Natural::ONE, 1, &Natural::from(7u32));
}

#[test]
#[should_panic]
fn mod_add_mul_shl_fail_4() {
    let mut x = Natural::ONE;
    let m = Natural::from_str("1000000000000000000000000000000").unwrap();
    x.mod_add_mul_shl_assign(&Natural::ONE, &m, 1, &m);
}

#[test]
fn mod_add_mul_shl_properties() {
    natural_natural_natural_unsigned_natural_quintuple_gen_var_1().test_properties(
        |(x, y, z, bits, m)| {
            let w = (&x).mod_add_mul_shl(&y, &z, bits, &m);
            assert!(w.is_valid());
            assert!(w.mod_is_reduced(&m));
            // The forms agree.
            assert_eq!(
                x.clone()
                    .mod_add_mul_shl(y.clone(), z.clone(), bits, m.clone()),
                w
            );
            assert_eq!(x.clone().mod_add_mul_shl(y.clone(), z.clone(), bits, &m), w);
            assert_eq!(x.clone().mod_add_mul_shl(y.clone(), &z, bits, m.clone()), w);
            assert_eq!(x.clone().mod_add_mul_shl(y.clone(), &z, bits, &m), w);
            assert_eq!(x.clone().mod_add_mul_shl(&y, z.clone(), bits, m.clone()), w);
            assert_eq!(x.clone().mod_add_mul_shl(&y, z.clone(), bits, &m), w);
            assert_eq!(x.clone().mod_add_mul_shl(&y, &z, bits, m.clone()), w);
            assert_eq!(x.clone().mod_add_mul_shl(&y, &z, bits, &m), w);
            let mut v = x.clone();
            v.mod_add_mul_shl_assign(y.clone(), z.clone(), bits, m.clone());
            assert!(v.is_valid());
            assert_eq!(v, w);
            let mut v = x.clone();
            v.mod_add_mul_shl_assign(y.clone(), z.clone(), bits, &m);
            assert!(v.is_valid());
            assert_eq!(v, w);
            let mut v = x.clone();
            v.mod_add_mul_shl_assign(y.clone(), &z, bits, m.clone());
            assert!(v.is_valid());
            assert_eq!(v, w);
            let mut v = x.clone();
            v.mod_add_mul_shl_assign(y.clone(), &z, bits, &m);
            assert!(v.is_valid());
            assert_eq!(v, w);
            let mut v = x.clone();
            v.mod_add_mul_shl_assign(&y, z.clone(), bits, m.clone());
            assert!(v.is_valid());
            assert_eq!(v, w);
            let mut v = x.clone();
            v.mod_add_mul_shl_assign(&y, z.clone(), bits, &m);
            assert!(v.is_valid());
            assert_eq!(v, w);
            let mut v = x.clone();
            v.mod_add_mul_shl_assign(&y, &z, bits, m.clone());
            assert!(v.is_valid());
            assert_eq!(v, w);
            let mut v = x.clone();
            v.mod_add_mul_shl_assign(&y, &z, bits, &m);
            assert!(v.is_valid());
            assert_eq!(v, w);

            // It agrees with exact arithmetic reduced once, and with the unfused combination, and
            // the factors commute.
            assert_eq!(mod_add_mul_shl_naive(&x, &y, &z, bits, &m), w);
            assert_eq!((&x).mod_add((&y).mod_mul(&z, &m).mod_shl(bits, &m), &m), w);
            assert_eq!((&x).mod_add_mul_shl(&z, &y, bits, &m), w);
            // Negating a factor gives the opposite operation, which undoes this one.
            assert_eq!((&x).mod_sub_mul_shl(&(&y).mod_neg(&m), &z, bits, &m), w);
            assert_eq!((&w).mod_sub_mul_shl(&y, &z, bits, &m), x);
            // Negating everything negates the result.
            assert_eq!(
                (&x).mod_neg(&m)
                    .mod_add_mul_shl((&y).mod_neg(&m), z.clone(), bits, &m),
                (&w).mod_neg(&m)
            );
            // Shifts compose.
            assert_eq!(
                (&x).mod_add_mul_shl(&(&y).mod_shl(1u64, &m), &z, bits, &m),
                (&x).mod_add_mul_shl(&y, &z, bits + 1, &m)
            );
        },
    );

    natural_quadruple_gen_var_1().test_properties(|(x, y, z, m)| {
        // With no shift, this is `mod_add_mul`.
        assert_eq!(
            (&x).mod_add_mul_shl(&y, &z, 0, &m),
            (&x).mod_add_mul(&y, &z, &m)
        );
    });

    unsigned_quintuple_gen_var_2::<Limb>().test_properties(|(x, y, z, bits, m)| {
        // It agrees with the primitive implementation.
        assert_eq!(
            Natural::from(x).mod_add_mul_shl(
                Natural::from(y),
                Natural::from(z),
                bits,
                Natural::from(m)
            ),
            x.mod_add_mul_shl(y, z, bits, m)
        );
    });
}
