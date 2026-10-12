// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::cmp::Ordering::{self, *};
use core::str::FromStr;
use malachite_base::num::comparison::traits::OrdProducts;
use malachite_nz::natural::Natural;
use malachite_nz::test_util::generators::{natural_pair_gen, natural_quadruple_gen};

#[test]
fn test_cmp_products() {
    let test = |a, b, c, d, out: Ordering| {
        let a = Natural::from_str(a).unwrap();
        let b = Natural::from_str(b).unwrap();
        let c = Natural::from_str(c).unwrap();
        let d = Natural::from_str(d).unwrap();
        assert_eq!(Natural::cmp_products(&a, &b, &c, &d), out);
        assert_eq!((&a * &b).cmp(&(&c * &d)), out);
    };
    // - a zero factor on the left
    test("0", "5", "0", "0", Equal);
    test("0", "5", "3", "7", Less);
    // - a zero factor on the right
    test("3", "7", "0", "9", Greater);
    // - the first pairing decides
    test("3", "7", "3", "7", Equal);
    test("3", "7", "7", "3", Equal);
    test("3", "7", "4", "8", Less);
    test("3", "7", "3", "8", Less);
    test("3", "7", "2", "7", Greater);
    // - neither pairing decides, and the bit lengths cannot either
    test("3", "7", "4", "5", Greater);
    test("3", "7", "5", "4", Greater);
    test("3", "7", "2", "11", Less);
    // - the multiplication, with factors of equal bit length
    test("1000000", "1000000", "999999", "1000001", Greater);
    test(
        "123456789012345678901234567890",
        "987654321098765432109876543210",
        "123456789012345678901234567891",
        "987654321098765432109876543209",
        Less,
    );
}

#[test]
fn cmp_products_grid() {
    // Every quadruple from a grid of values around limb boundaries agrees with the multiplied
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
        "4294967296",
        "18446744073709551615",
        "18446744073709551616",
        "18446744073709551617",
        "340282366920938463463374607431768211456",
        "1000000000000000000000000000000000000001",
    ]
    .iter()
    .map(|s| Natural::from_str(s).unwrap())
    .collect();
    for a in &grid {
        for b in &grid {
            let left = a * b;
            for c in &grid {
                for d in &grid {
                    assert_eq!(Natural::cmp_products(a, b, c, d), left.cmp(&(c * d)));
                }
            }
        }
    }
}

#[test]
fn cmp_products_properties() {
    natural_quadruple_gen().test_properties(|(a, b, c, d)| {
        let ord = Natural::cmp_products(&a, &b, &c, &d);
        assert_eq!((&a * &b).cmp(&(&c * &d)), ord);
        // The factors on each side commute, and swapping the sides reverses the result.
        assert_eq!(Natural::cmp_products(&b, &a, &d, &c), ord);
        assert_eq!(Natural::cmp_products(&c, &d, &a, &b), ord.reverse());
    });

    natural_pair_gen().test_properties(|(a, b)| {
        // A product compared with itself, in either order, is equal.
        assert_eq!(Natural::cmp_products(&a, &b, &a, &b), Equal);
        assert_eq!(Natural::cmp_products(&a, &b, &b, &a), Equal);
    });
}
