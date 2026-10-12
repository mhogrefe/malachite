// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::cmp::Ordering::{self, *};
use core::str::FromStr;
use malachite_base::num::basic::traits::One;
use malachite_base::num::comparison::traits::OrdProduct;
use malachite_nz::natural::Natural;
use malachite_nz::test_util::generators::{natural_pair_gen, natural_triple_gen};

#[test]
fn test_cmp_product() {
    let test = |a, b, c, out: Ordering| {
        let a = Natural::from_str(a).unwrap();
        let b = Natural::from_str(b).unwrap();
        let c = Natural::from_str(c).unwrap();
        assert_eq!(Natural::cmp_product(&a, &b, &c), out);
        assert_eq!((&a * &b).cmp(&c), out);
    };
    // - a zero factor
    test("0", "5", "0", Equal);
    test("0", "5", "3", Less);
    // - c is zero
    test("3", "7", "0", Greater);
    // - a factor of 1
    test("1", "7", "7", Equal);
    test("7", "1", "8", Less);
    // - the multiplication
    test("3", "7", "21", Equal);
    test("3", "7", "20", Greater);
    test("3", "7", "22", Less);
    // - the bit lengths, with a factor at least c
    test("3", "7", "5", Greater);
    // - the bit lengths
    test("3", "7", "100", Less);
    // - the multiplication, with factors of equal bit length
    test("255", "255", "65025", Equal);
    test("255", "255", "65024", Greater);
    test(
        "123456789012345678901234567890",
        "987654321098765432109876543210",
        "121932631137021795226185032733622923332237463801111263526900",
        Equal,
    );
}

#[test]
fn cmp_product_grid() {
    // Every triple from a grid of values around limb boundaries agrees with the multiplied
    // comparison.
    let grid: Vec<Natural> = [
        "0",
        "1",
        "2",
        "3",
        "7",
        "255",
        "256",
        "65535",
        "65536",
        "4294967296",
        "18446744073709551615",
        "18446744073709551616",
        "18446744073709551617",
        "340282366920938463463374607431768211456",
        "340282366920938463463374607431768211457",
        "1000000000000000000000000000000000000001",
    ]
    .iter()
    .map(|s| Natural::from_str(s).unwrap())
    .collect();
    for a in &grid {
        for b in &grid {
            let product = a * b;
            assert_eq!(Natural::cmp_product(a, b, &product), Equal);
            assert_eq!(Natural::cmp_product(a, b, &(&product + Natural::ONE)), Less);
            for c in &grid {
                assert_eq!(Natural::cmp_product(a, b, c), product.cmp(c));
            }
        }
    }
}

#[test]
fn cmp_product_properties() {
    natural_triple_gen().test_properties(|(a, b, c)| {
        let ord = Natural::cmp_product(&a, &b, &c);
        assert_eq!((&a * &b).cmp(&c), ord);
        // The factors commute.
        assert_eq!(Natural::cmp_product(&b, &a, &c), ord);
    });

    natural_pair_gen().test_properties(|(a, b)| {
        // Near the product, only the multiplication can decide.
        let product = &a * &b;
        assert_eq!(Natural::cmp_product(&a, &b, &product), Equal);
        assert_eq!(
            Natural::cmp_product(&a, &b, &(&product + Natural::ONE)),
            Less
        );
        if product != 0u32 {
            assert_eq!(
                Natural::cmp_product(&a, &b, &(&product - Natural::ONE)),
                Greater
            );
        }
    });
}
