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
use malachite_nz::test_util::generators::integer_quadruple_gen;
use malachite_q::Rational;
use malachite_q::test_util::generators::{rational_pair_gen, rational_quadruple_gen};

#[test]
fn test_cmp_products() {
    let test = |w, x, y, z, out: Ordering, out_neg: Ordering| {
        let w = Rational::from_str(w).unwrap();
        let x = Rational::from_str(x).unwrap();
        let y = Rational::from_str(y).unwrap();
        let z = Rational::from_str(z).unwrap();
        assert_eq!(Rational::cmp_products(&w, &x, &y, &z), out);
        assert_eq!((&w * &x).cmp(&(&y * &z)), out);
        assert_eq!(Rational::cmp_products_neg(&w, &x, &y, &z), out_neg);
        assert_eq!((&w * &x).cmp(&-(&y * &z)), out_neg);
    };
    // - the signs and zeros
    test("3/7", "7/3", "1/2", "2", Equal, Greater);
    test("3/7", "7/3", "1/2", "3", Less, Greater);
    test("-3/7", "-7/3", "-1/2", "-3", Less, Greater);
    test("-3/7", "7/3", "1/2", "-2", Equal, Less);
    test("-3/7", "7/3", "1/2", "-3", Greater, Less);
    test("3/7", "7/3", "-1/2", "3", Greater, Less);
    test("-3/7", "7/3", "1/2", "3", Less, Greater);
    test("0", "7/3", "0", "-5", Equal, Equal);
    test("0", "7/3", "1/2", "-5", Greater, Less);
    test("3/7", "7/3", "0", "-5", Greater, Greater);
    test("3/7", "-7/3", "0", "5", Less, Less);
    // - the bit lengths and the first screen
    test("1/3", "1/5", "7", "9/2", Less, Greater);
    // - the pairing screens
    test("2/3", "1/5", "3/5", "1/4", Less, Greater);
    test("2/3", "1/4", "3/5", "1/5", Greater, Greater);
    test("11/10", "9/10", "9/10", "11/10", Equal, Greater);
    // - the last resort
    test("11/10", "9/10", "99/100", "1", Equal, Greater);
    test("11/10", "9/10", "49/50", "1", Greater, Greater);
    test("11/10", "9/10", "1", "1", Less, Greater);
}

#[test]
fn cmp_products_grid() {
    // Every quadruple from a grid of values around limb boundaries agrees with the multiplied
    // comparison.
    let grid: Vec<Rational> = [
        "0",
        "1",
        "-1",
        "-1/2",
        "3/7",
        "-7/3",
        "255/256",
        "65535/65536",
        "-18446744073709551616/3",
        "340282366920938463463374607431768211456/7",
        "1000000000000000000000000000000000000001/1000000000000000000000000000000000000000",
    ]
    .iter()
    .map(|s| Rational::from_str(s).unwrap())
    .collect();
    for w in &grid {
        for x in &grid {
            let left = w * x;
            for y in &grid {
                for z in &grid {
                    let right = y * z;
                    assert_eq!(Rational::cmp_products(w, x, y, z), left.cmp(&right));
                    assert_eq!(Rational::cmp_products_neg(w, x, y, z), left.cmp(&-right));
                }
            }
        }
    }
}

#[test]
fn cmp_products_properties() {
    rational_quadruple_gen().test_properties(|(w, x, y, z)| {
        let ord = Rational::cmp_products(&w, &x, &y, &z);
        assert_eq!((&w * &x).cmp(&(&y * &z)), ord);
        let ord_neg = Rational::cmp_products_neg(&w, &x, &y, &z);
        assert_eq!((&w * &x).cmp(&-(&y * &z)), ord_neg);
        // The negated form is the plain form with one right factor negated.
        assert_eq!(Rational::cmp_products(&w, &x, &-&y, &z), ord_neg);
        // The factors on each side commute, and swapping the sides reverses the result.
        assert_eq!(Rational::cmp_products(&x, &w, &z, &y), ord);
        assert_eq!(Rational::cmp_products(&y, &z, &w, &x), ord.reverse());
        // The sign of wx + yz does not depend on which product is on which side.
        assert_eq!(Rational::cmp_products_neg(&y, &z, &w, &x), ord_neg);
    });

    rational_pair_gen().test_properties(|(x, y)| {
        assert_eq!(Rational::cmp_products(&x, &y, &y, &x), Equal);
        assert_eq!(Rational::cmp_products_neg(&x, &y, &-&y, &x), Equal);
    });

    integer_quadruple_gen().test_properties(|(a, b, c, d)| {
        // It agrees with the Integer implementation.
        assert_eq!(
            Rational::cmp_products(
                &Rational::from(&a),
                &Rational::from(&b),
                &Rational::from(&c),
                &Rational::from(&d)
            ),
            Integer::cmp_products(&a, &b, &c, &d)
        );
    });
}
