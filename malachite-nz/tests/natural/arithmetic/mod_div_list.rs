// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{
    DivExact, Gcd, Mod, ModDiv, ModDivList, ModIsReduced, ModMul,
};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::strings::ToDebugString;
use malachite_base::test_util::generators::unsigned_triple_gen_var_12;
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use malachite_nz::test_util::generators::{
    natural_gen_var_1, natural_pair_gen_var_8, natural_triple_gen_var_3,
};
use malachite_nz::test_util::natural::arithmetic::mod_div_list::mod_div_list_simple;
use std::panic::catch_unwind;
use std::str::FromStr;

#[test]
fn test_mod_div_list() {
    let test = |s, t, u, out| {
        let b = Natural::from_str(s).unwrap();
        let c = Natural::from_str(t).unwrap();
        let m = Natural::from_str(u).unwrap();

        let result = b.clone().mod_div_list(c.clone(), m.clone());
        assert_eq!(result.to_debug_string(), out);

        let result = b.clone().mod_div_list(c.clone(), &m);
        assert_eq!(result.to_debug_string(), out);

        let result = b.clone().mod_div_list(&c, m.clone());
        assert_eq!(result.to_debug_string(), out);

        let result = b.clone().mod_div_list(&c, &m);
        assert_eq!(result.to_debug_string(), out);

        let result = (&b).mod_div_list(c.clone(), m.clone());
        assert_eq!(result.to_debug_string(), out);

        let result = (&b).mod_div_list(c.clone(), &m);
        assert_eq!(result.to_debug_string(), out);

        let result = (&b).mod_div_list(&c, m.clone());
        assert_eq!(result.to_debug_string(), out);

        let result = (&b).mod_div_list(&c, &m);
        assert_eq!(result.to_debug_string(), out);

        assert_eq!(mod_div_list_simple(b, c, m).to_debug_string(), out);
    };
    test("0", "0", "1", "Some((0, 1, 1))");
    test("0", "0", "10", "Some((0, 1, 10))");
    test("1", "0", "10", "None");
    test("0", "7", "10", "Some((0, 10, 1))");
    test("1", "3", "10", "Some((7, 10, 1))");
    test("6", "4", "10", "Some((4, 5, 2))");
    test("5", "5", "10", "Some((1, 2, 5))");
    test("2", "5", "10", "None");
    // - zero divisor with a multi-limb modulus: the x == 0 branch of gcdinv_helper (moduli below
    //   2^64 are single-limb on 64-bit builds, so the exemplar must exceed that)
    test(
        "0",
        "0",
        "98765432123456789012345678990",
        "Some((0, 1, 98765432123456789012345678990))",
    );
    // - zero divisor and nonzero dividend with a multi-limb modulus
    test("1", "0", "98765432123456789012345678990", "None");
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
    // - negative cofactor from limbs_extended_gcd in gcdinv_helper (the limbs_sub lift branch)
    test(
        "123",
        "12345678987654321012345678901",
        "98765432123456789012345678990",
        "Some((34989702408577318988141271143, 98765432123456789012345678990, 1))",
    );
    test(
        "1",
        "12345678987654321",
        "98765432123456789",
        "Some((1777777788, 98765432123456789, 1))",
    );
    test(
        "12345678987654318",
        "12345678987654324",
        "98765432123456790",
        "Some((9070294768707482, 16460905353909465, 6))",
    );
    test(
        "1000000000000000000000000000000000000001",
        "700000000000000000000000000000000000007",
        "1300000000000000000000000000000000000039",
        "Some((578571428571428571428571428571428571448, 1300000000000000000000000000000000000039, \
        1))",
    );
    test(
        "24681357024681357024681357024",
        "36925814703692581470369258146",
        "98765432123456789012345678990",
        "Some((36963634533273950157590132899, 49382716061728394506172839495, 2))",
    );
}

#[test]
fn mod_div_list_fail() {
    assert_panic!(Natural::from(30u8).mod_div_list(Natural::ONE, Natural::from(3u32)));
    assert_panic!(Natural::ONE.mod_div_list(Natural::from(30u8), Natural::from(3u32)));
    assert_panic!(Natural::from(30u8).mod_div_list(Natural::ONE, &Natural::from(3u32)));
    assert_panic!(Natural::ONE.mod_div_list(&Natural::from(30u8), Natural::from(3u32)));
    assert_panic!((&Natural::from(30u8)).mod_div_list(Natural::ONE, Natural::from(3u32)));
    assert_panic!((&Natural::ONE).mod_div_list(&Natural::from(30u8), &Natural::from(3u32)));
}

#[test]
fn mod_div_list_properties() {
    natural_triple_gen_var_3().test_properties(|(b, c, m)| {
        assert!(b.mod_is_reduced(&m));
        assert!(c.mod_is_reduced(&m));
        let result = (&b).mod_div_list(&c, &m);

        assert_eq!(b.clone().mod_div_list(c.clone(), m.clone()), result);
        assert_eq!(b.clone().mod_div_list(c.clone(), &m), result);
        assert_eq!(b.clone().mod_div_list(&c, m.clone()), result);
        assert_eq!(b.clone().mod_div_list(&c, &m), result);
        assert_eq!((&b).mod_div_list(c.clone(), m.clone()), result);
        assert_eq!((&b).mod_div_list(c.clone(), &m), result);
        assert_eq!((&b).mod_div_list(&c, m.clone()), result);
        assert_eq!(mod_div_list_simple(b.clone(), c.clone(), m.clone()), result);

        let q = (&b).mod_div(&c, &m);
        assert_eq!(result.is_some(), q.is_some());
        if let Some((start, stride, length)) = result {
            assert_eq!(length, (&c).gcd(&m));
            assert_eq!(stride, (&m).div_exact(&length));
            assert!(start < stride);
            // any single quotient is start plus some multiple of stride
            assert_eq!(q.unwrap().mod_op(&stride), start);
            // Spot-check that the first few elements of the progression are quotients.
            let mut i = Natural::ZERO;
            while i < length && i < 4u32 {
                assert_eq!((&start + &stride * &i).mod_mul(&c, &m), b);
                i += Natural::ONE;
            }
        }
    });

    unsigned_triple_gen_var_12::<Limb>().test_properties(|(b, c, m)| {
        assert_eq!(
            Natural::from(b).mod_div_list(Natural::from(c), Natural::from(m)),
            b.mod_div_list(c, m).map(|(s, t, l)| (
                Natural::from(s),
                Natural::from(t),
                Natural::from(l)
            ))
        );
    });

    natural_pair_gen_var_8().test_properties(|(x, m)| {
        if m > 1u32 {
            let g = (&x).gcd(&m);
            assert_eq!(
                Natural::ZERO.mod_div_list(&x, &m),
                Some((Natural::ZERO, (&m).div_exact(&g), g))
            );
            assert_eq!(
                (&x).mod_div_list(&Natural::ONE, &m),
                Some((x.clone(), m.clone(), Natural::ONE))
            );
        }
    });

    natural_gen_var_1().test_properties(|m| {
        assert_eq!(
            Natural::ONE.mod_div_list(&Natural::ONE, &m),
            Some((Natural::ONE, m.clone(), Natural::ONE))
        );
        assert_eq!(
            Natural::ZERO.mod_div_list(&Natural::ZERO, &m),
            Some((Natural::ZERO, Natural::ONE, m.clone()))
        );
    });
}
