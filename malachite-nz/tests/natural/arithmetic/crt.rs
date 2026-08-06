// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{CoprimeWith, Crt};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::strings::ToDebugString;
use malachite_base::test_util::generators::unsigned_quadruple_gen_var_13;
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use malachite_nz::test_util::generators::{natural_pair_gen_var_8, natural_quadruple_gen_var_5};
use malachite_nz::test_util::natural::arithmetic::crt::crt_simple;
use std::panic::catch_unwind;
use std::str::FromStr;

#[test]
fn test_crt() {
    let test = |s, t, u, v, out| {
        let r1 = Natural::from_str(s).unwrap();
        let m1 = Natural::from_str(t).unwrap();
        let r2 = Natural::from_str(u).unwrap();
        let m2 = Natural::from_str(v).unwrap();

        let result = r1.clone().crt(m1.clone(), r2.clone(), m2.clone());
        assert_eq!(result.to_debug_string(), out);
        assert_eq!(
            r1.clone()
                .crt(m1.clone(), r2.clone(), &m2)
                .to_debug_string(),
            out
        );
        assert_eq!(
            r1.clone()
                .crt(m1.clone(), &r2, m2.clone())
                .to_debug_string(),
            out
        );
        assert_eq!(r1.clone().crt(m1.clone(), &r2, &m2).to_debug_string(), out);
        assert_eq!(
            r1.clone()
                .crt(&m1, r2.clone(), m2.clone())
                .to_debug_string(),
            out
        );
        assert_eq!(r1.clone().crt(&m1, r2.clone(), &m2).to_debug_string(), out);
        assert_eq!(r1.clone().crt(&m1, &r2, m2.clone()).to_debug_string(), out);
        assert_eq!(r1.clone().crt(&m1, &r2, &m2).to_debug_string(), out);
        assert_eq!(
            (&r1)
                .crt(m1.clone(), r2.clone(), m2.clone())
                .to_debug_string(),
            out
        );
        assert_eq!(
            (&r1).crt(m1.clone(), r2.clone(), &m2).to_debug_string(),
            out
        );
        assert_eq!(
            (&r1).crt(m1.clone(), &r2, m2.clone()).to_debug_string(),
            out
        );
        assert_eq!((&r1).crt(m1.clone(), &r2, &m2).to_debug_string(), out);
        assert_eq!(
            (&r1).crt(&m1, r2.clone(), m2.clone()).to_debug_string(),
            out
        );
        assert_eq!((&r1).crt(&m1, r2.clone(), &m2).to_debug_string(), out);
        assert_eq!((&r1).crt(&m1, &r2, m2.clone()).to_debug_string(), out);
        assert_eq!((&r1).crt(&m1, &r2, &m2).to_debug_string(), out);

        assert_eq!(crt_simple(r1, m1, r2, m2).to_debug_string(), out);
    };
    test("0", "1", "0", "1", "Some(0)");
    // - crt_helper: c == 0 and m2 == 1: the second congruence is vacuous
    test("5", "10", "0", "1", "Some(5)");
    test("2", "3", "3", "5", "Some(8)");
    test("2", "6", "2", "7", "Some(2)");
    // - crt_helper: c == 0 and m2 > 1: not coprime, even though the congruences are compatible
    test("2", "4", "0", "2", "None");
    // - c != 0 and the moduli are not coprime
    test("1", "4", "3", "6", "None");
    // Note that moduli below 2^64 are single-limb on 64-bit builds, so the multi-limb rows must
    // exceed that, not merely look long.
    // - both moduli multi-limb and coprime
    test(
        "123",
        "98765432123456789012345678990",
        "456",
        "12345678987654321012345678901",
        "Some(852813132001985672006700208575057191239692965020660540523)",
    );
    // - multi-limb m1, single-limb m2
    test(
        "9876543212345678901234567897",
        "98765432123456789012345678990",
        "5",
        "7",
        "Some(207407407459259256925925925877)",
    );
    // - single-limb m1, multi-limb m2
    test(
        "2",
        "7",
        "9876543212345678901234567897",
        "12345678987654321012345678901",
        "Some(59259259162962962950617283501)",
    );
    // - vacuous second congruence with a multi-limb m1
    test(
        "98765432123456789012345678989",
        "98765432123456789012345678990",
        "0",
        "1",
        "Some(98765432123456789012345678989)",
    );
    // - multi-limb moduli that are not coprime (both even)
    test(
        "2",
        "98765432123456789012345678990",
        "4",
        "36925814703692581470369258146",
        "None",
    );
    // - two single-limb moduli whose product needs two limbs, so the word dispatch cannot apply
    test(
        "3",
        "18446744073709551557",
        "4",
        "18446744073709551533",
        "Some(269390207145742948168885365600372308432)",
    );
}

#[test]
fn crt_fail() {
    assert_panic!(Natural::from(3u8).crt(Natural::from(3u8), Natural::ZERO, Natural::ONE));
    assert_panic!(Natural::ZERO.crt(Natural::ONE, Natural::from(3u8), Natural::from(3u8)));
    assert_panic!(Natural::ZERO.crt(Natural::ZERO, Natural::ZERO, Natural::ONE));
    assert_panic!(Natural::ZERO.crt(Natural::ONE, Natural::ZERO, Natural::ZERO));
    assert_panic!((&Natural::from(3u8)).crt(&Natural::from(3u8), &Natural::ZERO, &Natural::ONE));
    assert_panic!((&Natural::ZERO).crt(&Natural::ONE, &Natural::from(3u8), &Natural::from(3u8)));
}

#[test]
fn crt_properties() {
    natural_quadruple_gen_var_5().test_properties(|(r1, m1, r2, m2)| {
        assert!(r1 < m1);
        assert!(r2 < m2);
        let result = (&r1).crt(&m1, &r2, &m2);

        assert_eq!(r1.clone().crt(m1.clone(), r2.clone(), m2.clone()), result);
        assert_eq!(r1.clone().crt(m1.clone(), r2.clone(), &m2), result);
        assert_eq!(r1.clone().crt(m1.clone(), &r2, m2.clone()), result);
        assert_eq!(r1.clone().crt(m1.clone(), &r2, &m2), result);
        assert_eq!(r1.clone().crt(&m1, r2.clone(), m2.clone()), result);
        assert_eq!(r1.clone().crt(&m1, r2.clone(), &m2), result);
        assert_eq!(r1.clone().crt(&m1, &r2, m2.clone()), result);
        assert_eq!(r1.clone().crt(&m1, &r2, &m2), result);
        assert_eq!((&r1).crt(m1.clone(), r2.clone(), m2.clone()), result);
        assert_eq!((&r1).crt(m1.clone(), r2.clone(), &m2), result);
        assert_eq!((&r1).crt(m1.clone(), &r2, m2.clone()), result);
        assert_eq!((&r1).crt(m1.clone(), &r2, &m2), result);
        assert_eq!((&r1).crt(&m1, r2.clone(), m2.clone()), result);
        assert_eq!((&r1).crt(&m1, r2.clone(), &m2), result);
        assert_eq!((&r1).crt(&m1, &r2, m2.clone()), result);
        assert_eq!(
            crt_simple(r1.clone(), m1.clone(), r2.clone(), m2.clone()),
            result
        );

        // The solution is symmetric in the two congruences.
        assert_eq!((&r2).crt(&m2, &r1, &m1), result);
        assert_eq!(result.is_some(), (&m1).coprime_with(&m2));
        if let Some(x) = result {
            assert!(x < &m1 * &m2);
            assert_eq!(&x % &m1, r1);
            assert_eq!(&x % &m2, r2);
        }
    });

    natural_pair_gen_var_8().test_properties(|(x, m)| {
        assert_eq!((&x).crt(&m, &Natural::ZERO, &Natural::ONE), Some(x.clone()));
        assert_eq!(
            Natural::ZERO.crt(Natural::ONE, x.clone(), m.clone()),
            Some(x)
        );
    });

    unsigned_quadruple_gen_var_13::<Limb>().test_properties(|(r1, m1, r2, m2)| {
        assert_eq!(
            Natural::from(r1).crt(Natural::from(m1), Natural::from(r2), Natural::from(m2)),
            r1.crt(m1, r2, m2).map(Natural::from)
        );
    });
}
