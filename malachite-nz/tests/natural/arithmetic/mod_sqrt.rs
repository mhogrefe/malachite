// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::assert_panic;
use malachite_base::num::arithmetic::traits::{JacobiSymbol, ModMul, ModSqrt, Parity, PowerOf2};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_nz::natural::Natural;
use malachite_nz::test_util::generators::{natural_gen, natural_pair_gen_var_8};
use std::panic::catch_unwind;
use std::str::FromStr;

#[test]
fn test_mod_sqrt() {
    let test = |x, m, out: Option<&str>| {
        let x = Natural::from_str(x).unwrap();
        let m = Natural::from_str(m).unwrap();
        let out = out.map(|s| Natural::from_str(s).unwrap());
        assert_eq!((&x).mod_sqrt(&m), out);
        assert_eq!((&x).mod_sqrt(m.clone()), out);
        assert_eq!(x.clone().mod_sqrt(&m), out);
        assert_eq!(x.mod_sqrt(m), out);
    };
    // - x <= 1: always a root of itself, for any modulus
    test("0", "1", Some("0"));
    test("0", "2", Some("0"));
    test("1", "2", Some("1"));
    test("1", "1000000", Some("1"));
    // - small moduli use the exhaustive search
    test("2", "3", None);
    test("4", "5", Some("2"));
    test("2", "7", Some("3"));
    // - the search works for small even moduli too...
    test("4", "6", Some("2"));
    // - ...but only scans t <= (m - 1) / 2, so a root in the upper half of an even modulus is
    //   missed (3 ^ 2 = 3 mod 6); this matches FLINT
    test("3", "6", None);
    // - m >= 600 and even: None, even though 2 ^ 2 = 4
    test("4", "600", None);
    // - m >= 600 and a perfect square (841 = 29 ^ 2): None, even though 2 ^ 2 = 4; this test also
    //   keeps the quadratic-nonresidue search terminating
    test("4", "841", None);
    // - composite m = 611 = 13 * 47 = 3 mod 4 with Jacobi symbol 1: the fast path returns a value
    //   that is not a root (183 ^ 2 = 495 mod 611) without noticing; this matches FLINT
    test("3", "611", Some("183"));
    // - composite m = 609 = 3 * 7 * 29 = 1 mod 8 with Jacobi symbol 1: the Tonelli-Shanks iteration
    //   cap trips and reports failure; this matches FLINT
    test("2", "609", None);
    // - prime m = 65537 = 1 mod 8: Tonelli-Shanks
    test("12909", "65537", Some("50618"));
    // - prime m = 2 ^ 224 - 2 ^ 96 + 1 = 1 mod 8: Tonelli-Shanks
    test(
        "15241578750190521",
        "26959946667150639794667015087019630673557916260026308143510066298881",
        Some("26959946667150639794667015087019630673557916260026308143509942842092"),
    );
    // - prime m = 2 ^ 255 - 19 = 5 mod 8
    test(
        "15241578750190521",
        "57896044618658097711785492504343953926634992332820282019728792003956564819949",
        Some("57896044618658097711785492504343953926634992332820282019728792003956441363160"),
    );
    // - odd m in (50, 600) with Jacobi symbol -1: the shortcut skips the search
    test("2", "53", None);
    // - large odd m with Jacobi symbol -1
    test("3", "65537", None);
    // - m = 5 mod 8 where the first candidate is not a root and the adjustment by a power of 2
    //   applies; note that the result is the root 2 ^ 255 - 21, not its negation 2
    test(
        "4",
        "57896044618658097711785492504343953926634992332820282019728792003956564819949",
        Some("57896044618658097711785492504343953926634992332820282019728792003956564819947"),
    );
    // - prime m = 2 ^ 521 - 1 = 3 mod 4
    test(
        "15241578750190521",
        "68647976601306097149819007990813932172694353001433054093944634591855431833976560\
        52122559640661454554977296311391480858037121987999716643812574028291115057151",
        Some("123456789"),
    );
}

#[test]
fn mod_sqrt_fail() {
    assert_panic!(Natural::from(3u32).mod_sqrt(Natural::from(3u32)));
    assert_panic!(Natural::from(30u32).mod_sqrt(Natural::from(3u32)));
    assert_panic!(Natural::ZERO.mod_sqrt(Natural::ZERO));
}

#[test]
fn mod_sqrt_properties() {
    natural_pair_gen_var_8().test_properties(|(x, m)| {
        let result = (&x).mod_sqrt(&m);
        assert_eq!((&x).mod_sqrt(m.clone()), result);
        assert_eq!(x.clone().mod_sqrt(&m), result);
        assert_eq!(x.clone().mod_sqrt(m.clone()), result);
        if let Some(r) = &result {
            assert!(*r < m);
        }
        if x <= 1 {
            assert_eq!(result, Some(x.clone()));
        }
        // For odd moduli the sub-600 search is exhaustive: roots come in pairs t and m - t, so
        // scanning half the range decides existence.
        if m.odd() && m < 600 {
            match &result {
                Some(r) => {
                    assert_eq!(r.mod_mul(r, &m), x);
                }
                None => {
                    let mut t = Natural::ZERO;
                    while t < m {
                        assert_ne!((&t).mod_mul(&t, &m), x);
                        t += Natural::ONE;
                    }
                }
            }
        }
    });

    // For prime moduli, every square has a root and every value with Jacobi symbol -1 has none. The
    // moduli cover all three algorithmic paths: 65537 and p224 are 1 mod 8 (Tonelli-Shanks), p25519
    // is 5 mod 8, and m521 is 3 mod 4.
    let primes = [
        Natural::from(65537u32),
        Natural::power_of_2(224) - Natural::power_of_2(96) + Natural::ONE,
        Natural::power_of_2(255) - Natural::from(19u32),
        Natural::power_of_2(521) - Natural::ONE,
    ];
    for p in primes {
        natural_gen().test_properties(|x| {
            let x = x % &p;
            let square = (&x).mod_mul(&x, &p);
            let root = (&square).mod_sqrt(&p).unwrap();
            assert_eq!((&root).mod_mul(&root, &p), square);
            if x > 1u32 && (&x).jacobi_symbol(&p) == -1 {
                assert_eq!((&x).mod_sqrt(&p), None);
            }
        });
    }
}
