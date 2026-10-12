// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::cmp::Ordering::{self, *};
use core::str::FromStr;
use malachite_base::num::comparison::traits::{OrdProducts, OrdProductsNeg};
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_nz::test_util::generators::{integer_quadruple_gen, natural_quadruple_gen};

#[test]
fn test_cmp_products() {
    let test = |a, b, c, d, out: Ordering, out_neg: Ordering| {
        let a = Integer::from_str(a).unwrap();
        let b = Integer::from_str(b).unwrap();
        let c = Integer::from_str(c).unwrap();
        let d = Integer::from_str(d).unwrap();
        assert_eq!(Integer::cmp_products(&a, &b, &c, &d), out);
        assert_eq!((&a * &b).cmp(&(&c * &d)), out);
        assert_eq!(Integer::cmp_products_neg(&a, &b, &c, &d), out_neg);
        assert_eq!((&a * &b).cmp(&-(&c * &d)), out_neg);
    };
    test("3", "7", "7", "3", Equal, Greater);
    test("-3", "-7", "7", "3", Equal, Greater);
    test("-3", "7", "-7", "3", Equal, Less);
    test("3", "7", "-4", "5", Greater, Greater);
    test("-3", "7", "4", "5", Less, Less);
    test("-3", "7", "-4", "5", Less, Less);
    test("-3", "7", "-4", "6", Greater, Less);
    test("0", "7", "0", "-5", Equal, Equal);
    test("0", "7", "4", "-5", Greater, Less);
    test("0", "7", "4", "5", Less, Greater);
    test("3", "7", "0", "5", Greater, Greater);
    test("3", "-7", "0", "5", Less, Less);
}

#[test]
fn cmp_products_grid() {
    // Every quadruple from a grid of values around limb boundaries agrees with the multiplied
    // comparison.
    let grid: Vec<Integer> = [
        "0",
        "1",
        "-1",
        "2",
        "-2",
        "7",
        "-255",
        "256",
        "-65536",
        "18446744073709551616",
        "-18446744073709551615",
        "340282366920938463463374607431768211456",
        "-1000000000000000000000000000000000000001",
    ]
    .iter()
    .map(|s| Integer::from_str(s).unwrap())
    .collect();
    for a in &grid {
        for b in &grid {
            let left = a * b;
            for c in &grid {
                for d in &grid {
                    let right = c * d;
                    assert_eq!(Integer::cmp_products(a, b, c, d), left.cmp(&right));
                    assert_eq!(Integer::cmp_products_neg(a, b, c, d), left.cmp(&-right));
                }
            }
        }
    }
}

#[test]
fn cmp_products_properties() {
    integer_quadruple_gen().test_properties(|(a, b, c, d)| {
        let ord = Integer::cmp_products(&a, &b, &c, &d);
        assert_eq!((&a * &b).cmp(&(&c * &d)), ord);
        let ord_neg = Integer::cmp_products_neg(&a, &b, &c, &d);
        assert_eq!((&a * &b).cmp(&-(&c * &d)), ord_neg);
        // The negated form is the plain form with one right factor negated.
        assert_eq!(Integer::cmp_products(&a, &b, &-&c, &d), ord_neg);
        // The factors on each side commute, and swapping the sides reverses the result.
        assert_eq!(Integer::cmp_products(&b, &a, &d, &c), ord);
        assert_eq!(Integer::cmp_products(&c, &d, &a, &b), ord.reverse());
        // The sign of ab + cd does not depend on which product is on which side.
        assert_eq!(Integer::cmp_products_neg(&c, &d, &a, &b), ord_neg);
    });

    natural_quadruple_gen().test_properties(|(a, b, c, d)| {
        // It agrees with the Natural implementation.
        assert_eq!(
            Integer::cmp_products(
                &Integer::from(&a),
                &Integer::from(&b),
                &Integer::from(&c),
                &Integer::from(&d)
            ),
            Natural::cmp_products(&a, &b, &c, &d)
        );
    });
}
