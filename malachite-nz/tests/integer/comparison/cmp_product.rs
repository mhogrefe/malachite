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
use malachite_nz::natural::Natural;
use malachite_nz::test_util::generators::{
    integer_pair_gen, integer_triple_gen, natural_triple_gen,
};

#[test]
fn test_cmp_product() {
    let test = |a, b, c, out: Ordering, out_neg: Ordering| {
        let a = Integer::from_str(a).unwrap();
        let b = Integer::from_str(b).unwrap();
        let c = Integer::from_str(c).unwrap();
        assert_eq!(Integer::cmp_product(&a, &b, &c), out);
        assert_eq!((&a * &b).cmp(&c), out);
        assert_eq!(Integer::cmp_product_neg(&a, &b, &c), out_neg);
        assert_eq!((&a * &b).cmp(&-&c), out_neg);
    };
    test("3", "7", "21", Equal, Greater);
    test("-3", "-7", "21", Equal, Greater);
    test("-3", "7", "-21", Equal, Less);
    test("3", "7", "-21", Greater, Equal);
    test("-3", "7", "21", Less, Equal);
    test("-3", "7", "-20", Less, Less);
    test("-3", "7", "-22", Greater, Less);
    test("0", "7", "0", Equal, Equal);
    test("0", "7", "5", Less, Greater);
    test("0", "-7", "-5", Greater, Less);
    test("3", "7", "0", Greater, Greater);
    test("-3", "7", "0", Less, Less);
}

#[test]
fn cmp_product_grid() {
    // Every triple from a grid of values around limb boundaries agrees with the multiplied
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
            let product = a * b;
            for c in &grid {
                assert_eq!(Integer::cmp_product(a, b, c), product.cmp(c));
                assert_eq!(Integer::cmp_product_neg(a, b, c), product.cmp(&-c));
            }
        }
    }
}

#[test]
fn cmp_product_properties() {
    integer_triple_gen().test_properties(|(a, b, c)| {
        let ord = Integer::cmp_product(&a, &b, &c);
        assert_eq!((&a * &b).cmp(&c), ord);
        let ord_neg = Integer::cmp_product_neg(&a, &b, &c);
        assert_eq!((&a * &b).cmp(&-&c), ord_neg);
        // The negated form is the plain form on `-c`.
        assert_eq!(Integer::cmp_product(&a, &b, &-&c), ord_neg);
        // The factors commute, and negating one factor and `c` reverses the result.
        assert_eq!(Integer::cmp_product(&b, &a, &c), ord);
        assert_eq!(Integer::cmp_product(&-&a, &b, &-&c), ord.reverse());
    });

    integer_pair_gen().test_properties(|(a, b)| {
        let product = &a * &b;
        assert_eq!(Integer::cmp_product(&a, &b, &product), Equal);
        assert_eq!(Integer::cmp_product_neg(&a, &b, &-product), Equal);
    });

    natural_triple_gen().test_properties(|(a, b, c)| {
        // It agrees with the Natural implementation.
        assert_eq!(
            Integer::cmp_product(&Integer::from(&a), &Integer::from(&b), &Integer::from(&c)),
            Natural::cmp_product(&a, &b, &c)
        );
    });
}
