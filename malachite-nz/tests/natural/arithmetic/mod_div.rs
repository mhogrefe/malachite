// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{
    CoprimeWith, Gcd, ModDiv, ModInverse, ModIsReduced, ModMul,
};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::strings::ToDebugString;
use malachite_base::test_util::generators::unsigned_triple_gen_var_12;
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use malachite_nz::test_util::generators::{
    natural_gen_var_1, natural_pair_gen_var_8, natural_triple_gen_var_3,
};
use malachite_nz::test_util::natural::arithmetic::mod_div::mod_div_simple;
use std::panic::catch_unwind;
use std::str::FromStr;

#[test]
fn test_mod_div() {
    let test = |s, t, u, out| {
        let b = Natural::from_str(s).unwrap();
        let c = Natural::from_str(t).unwrap();
        let m = Natural::from_str(u).unwrap();

        let result = b.clone().mod_div(c.clone(), m.clone());
        assert_eq!(result.to_debug_string(), out);
        assert!(result.is_none_or(|n| n.is_valid()));

        let result = b.clone().mod_div(c.clone(), &m);
        assert_eq!(result.to_debug_string(), out);
        assert!(result.is_none_or(|n| n.is_valid()));

        let result = b.clone().mod_div(&c, m.clone());
        assert_eq!(result.to_debug_string(), out);
        assert!(result.is_none_or(|n| n.is_valid()));

        let result = b.clone().mod_div(&c, &m);
        assert_eq!(result.to_debug_string(), out);
        assert!(result.is_none_or(|n| n.is_valid()));

        let result = (&b).mod_div(c.clone(), m.clone());
        assert_eq!(result.to_debug_string(), out);
        assert!(result.is_none_or(|n| n.is_valid()));

        let result = (&b).mod_div(c.clone(), &m);
        assert_eq!(result.to_debug_string(), out);
        assert!(result.is_none_or(|n| n.is_valid()));

        let result = (&b).mod_div(&c, m.clone());
        assert_eq!(result.to_debug_string(), out);
        assert!(result.is_none_or(|n| n.is_valid()));

        let result = (&b).mod_div(&c, &m);
        assert_eq!(result.to_debug_string(), out);
        assert!(result.is_none_or(|n| n.is_valid()));

        assert_eq!(mod_div_simple(b, c, m).to_debug_string(), out);
    };
    test("0", "0", "1", "Some(0)");
    test("0", "0", "10", "Some(0)");
    test("1", "0", "10", "None");
    test("0", "7", "10", "Some(0)");
    test("1", "3", "10", "Some(7)");
    test("6", "4", "10", "Some(4)");
    test("5", "5", "10", "Some(1)");
    test("2", "5", "10", "None");
    // Note that moduli below 2^64 are single-limb on 64-bit builds, so the multi-limb exemplars
    // must exceed that, not merely look long.
    // - zero divisor and zero dividend with a multi-limb modulus
    test("0", "0", "98765432123456789012345678990", "Some(0)");
    // - zero divisor and nonzero dividend with a multi-limb modulus
    test("1", "0", "98765432123456789012345678990", "None");
    // - zero dividend with a multi-limb modulus
    test(
        "0",
        "12345678987654321012345678901",
        "98765432123456789012345678990",
        "Some(0)",
    );
    // - no quotient with a single-limb modulus
    test(
        "12345678987654322",
        "12345678987654324",
        "98765432123456790",
        "None",
    );
    // - no quotient with a multi-limb modulus
    test(
        "24681357024681357024681357023",
        "36925814703692581470369258146",
        "98765432123456789012345678990",
        "None",
    );
    // - negative cofactor from limbs_extended_gcd in gcdinv_helper (the limbs_sub lift branch); gcd
    //   = 1, so the witness is unique and independently checkable
    test(
        "123",
        "12345678987654321012345678901",
        "98765432123456789012345678990",
        "Some(34989702408577318988141271143)",
    );
    test(
        "1",
        "12345678987654321",
        "98765432123456789",
        "Some(1777777788)",
    );
    test(
        "123",
        "12345678987654321",
        "98765432123456789",
        "Some(218666667924)",
    );
    test(
        "98765432123456788",
        "98765432123456788",
        "98765432123456789",
        "Some(1)",
    );
    test(
        "12345678987654318",
        "12345678987654324",
        "98765432123456790",
        "Some(74913916184345342)",
    );
    test(
        "1000000000000000000000000000000000000001",
        "700000000000000000000000000000000000007",
        "1300000000000000000000000000000000000039",
        "Some(578571428571428571428571428571428571448)",
    );
    test(
        "24681357024681357024681357024",
        "36925814703692581470369258146",
        "98765432123456789012345678990",
        "Some(86346350595002344663762972394)",
    );
}

#[test]
fn mod_div_fail() {
    assert_panic!(Natural::from(30u8).mod_div(Natural::ONE, Natural::from(3u32)));
    assert_panic!(Natural::ONE.mod_div(Natural::from(30u8), Natural::from(3u32)));
    assert_panic!(Natural::from(30u8).mod_div(Natural::ONE, &Natural::from(3u32)));
    assert_panic!(Natural::ONE.mod_div(&Natural::from(30u8), Natural::from(3u32)));
    assert_panic!((&Natural::from(30u8)).mod_div(Natural::ONE, Natural::from(3u32)));
    assert_panic!((&Natural::ONE).mod_div(&Natural::from(30u8), &Natural::from(3u32)));
}

#[test]
fn mod_div_properties() {
    natural_triple_gen_var_3().test_properties(|(b, c, m)| {
        assert!(b.mod_is_reduced(&m));
        assert!(c.mod_is_reduced(&m));
        let q = (&b).mod_div(&c, &m);
        assert!(q.clone().is_none_or(|n| n.is_valid()));

        assert_eq!(b.clone().mod_div(c.clone(), m.clone()), q);
        assert_eq!(b.clone().mod_div(c.clone(), &m), q);
        assert_eq!(b.clone().mod_div(&c, m.clone()), q);
        assert_eq!(b.clone().mod_div(&c, &m), q);
        assert_eq!((&b).mod_div(c.clone(), m.clone()), q);
        assert_eq!((&b).mod_div(c.clone(), &m), q);
        assert_eq!((&b).mod_div(&c, m.clone()), q);
        assert_eq!(mod_div_simple(b.clone(), c.clone(), m.clone()), q);

        assert_eq!(q.is_some(), &b % (&c).gcd(&m) == 0u32);
        if let Some(q) = q {
            assert!(q.mod_is_reduced(&m));
            assert_eq!((&q).mod_mul(&c, &m), b);
            if c != 0u32 && (&c).coprime_with(&m) {
                assert_eq!(q, (&b).mod_mul((&c).mod_inverse(&m).unwrap(), &m));
            }
        }
        let product = (&b).mod_mul(&c, &m);
        assert!(product.mod_div(&c, &m).is_some());
    });

    unsigned_triple_gen_var_12::<Limb>().test_properties(|(b, c, m)| {
        assert_eq!(
            Natural::from(b).mod_div(Natural::from(c), Natural::from(m)),
            b.mod_div(c, m).map(Natural::from)
        );
    });

    natural_pair_gen_var_8().test_properties(|(x, m)| {
        assert!((&x).mod_div(&x, &m).is_some());
        if m > 1u32 {
            assert_eq!((&x).mod_div(&Natural::ONE, &m), Some(x.clone()));
            assert_eq!(Natural::ZERO.mod_div(&x, &m), Some(Natural::ZERO));
        }
    });

    natural_gen_var_1().test_properties(|m| {
        assert_eq!(Natural::ONE.mod_div(&Natural::ONE, &m), Some(Natural::ONE));
        let m_minus_1 = &m - Natural::ONE;
        assert_eq!((&m_minus_1).mod_div(&m_minus_1, &m), Some(Natural::ONE));
    });
}
