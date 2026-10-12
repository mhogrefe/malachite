// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::cmp::Ordering::{self, *};
use core::str::FromStr;
use malachite_base::num::comparison::traits::{OrdProduct, OrdProductNeg};
use malachite_nz::integer::Integer;
use malachite_nz::test_util::generators::integer_triple_gen;
use malachite_q::Rational;
use malachite_q::test_util::generators::{rational_pair_gen, rational_triple_gen};

#[test]
fn test_cmp_product() {
    let test = |x, y, z, out: Ordering, out_neg: Ordering| {
        let x = Rational::from_str(x).unwrap();
        let y = Rational::from_str(y).unwrap();
        let z = Rational::from_str(z).unwrap();
        assert_eq!(Rational::cmp_product(&x, &y, &z), out);
        assert_eq!((&x * &y).cmp(&z), out);
        assert_eq!(Rational::cmp_product_neg(&x, &y, &z), out_neg);
        assert_eq!((&x * &y).cmp(&-&z), out_neg);
    };
    // - the signs and zeros
    test("3/7", "7/3", "1", Equal, Greater);
    test("3/7", "7/3", "2", Less, Greater);
    test("-3/7", "-7/3", "1/2", Greater, Greater);
    test("-3/7", "7/3", "-1", Equal, Less);
    test("-3/7", "7/3", "-2", Greater, Less);
    test("-3/7", "7/3", "-1/2", Less, Less);
    test("3/7", "7/3", "-5", Greater, Less);
    test("-3/7", "7/3", "5", Less, Greater);
    test("0", "7/3", "0", Equal, Equal);
    test("0", "7/3", "-1/9", Greater, Less);
    test("3/7", "0", "1/9", Less, Greater);
    test("3/7", "7/3", "0", Greater, Greater);
    test("3/7", "-7/3", "0", Less, Less);
    // - the bit lengths
    test("1/3", "1/5", "7", Less, Greater);
    test("5", "3", "1/7", Greater, Greater);
    // - the factorwise screens
    test("2/3", "1/5", "3/5", Less, Greater);
    test("7/5", "6/5", "7/5", Greater, Greater);
    // - the last resort
    test("11/10", "9/10", "99/100", Equal, Greater);
    test("11/10", "9/10", "49/50", Greater, Greater);
    test("11/10", "9/10", "1", Less, Greater);
}

#[test]
fn cmp_product_grid() {
    // Every triple from a grid of values around limb boundaries agrees with the multiplied
    // comparison.
    let grid: Vec<Rational> = [
        "0",
        "1",
        "-1",
        "2",
        "-1/2",
        "3/7",
        "-7/3",
        "255/256",
        "-256/255",
        "65535/65536",
        "18446744073709551616/3",
        "-3/18446744073709551615",
        "340282366920938463463374607431768211456/7",
        "-7/340282366920938463463374607431768211457",
        "1000000000000000000000000000000000000001/1000000000000000000000000000000000000000",
    ]
    .iter()
    .map(|s| Rational::from_str(s).unwrap())
    .collect();
    for x in &grid {
        for y in &grid {
            let product = x * y;
            for z in &grid {
                assert_eq!(Rational::cmp_product(x, y, z), product.cmp(z));
                assert_eq!(Rational::cmp_product_neg(x, y, z), product.cmp(&-z));
            }
        }
    }
}

#[test]
fn cmp_product_properties() {
    rational_triple_gen().test_properties(|(x, y, z)| {
        let ord = Rational::cmp_product(&x, &y, &z);
        assert_eq!((&x * &y).cmp(&z), ord);
        let ord_neg = Rational::cmp_product_neg(&x, &y, &z);
        assert_eq!((&x * &y).cmp(&-&z), ord_neg);
        // The negated form is the plain form on `-z`.
        assert_eq!(Rational::cmp_product(&x, &y, &-&z), ord_neg);
        // The factors commute, and negating one factor and `z` reverses the result.
        assert_eq!(Rational::cmp_product(&y, &x, &z), ord);
        assert_eq!(Rational::cmp_product(&-&x, &y, &-&z), ord.reverse());
    });

    rational_pair_gen().test_properties(|(x, y)| {
        let product = &x * &y;
        assert_eq!(Rational::cmp_product(&x, &y, &product), Equal);
        assert_eq!(Rational::cmp_product_neg(&x, &y, &-product), Equal);
    });

    integer_triple_gen().test_properties(|(a, b, c)| {
        // It agrees with the Integer implementation.
        assert_eq!(
            Rational::cmp_product(
                &Rational::from(&a),
                &Rational::from(&b),
                &Rational::from(&c)
            ),
            Integer::cmp_product(&a, &b, &c)
        );
    });
}
