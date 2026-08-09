// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::traits::One;
use malachite_nz::natural::Natural;
use malachite_q::Rational;
use malachite_q::rational::arithmetic::traits::Approximate;
use malachite_q::test_util::generators::rational_natural_pair_gen_var_5;
use std::str::FromStr;

#[test]
fn test_farey_neighbors() {
    let test = |s, n, left, right| {
        let x = Rational::from_str(s).unwrap();
        let n = Natural::from_str(n).unwrap();
        let (l, r) = x.farey_neighbors(&n);
        assert!(l.is_valid());
        assert!(r.is_valid());
        assert_eq!(l.to_string(), left);
        assert_eq!(r.to_string(), right);
    };
    // - inside the classical [0, 1] range
    test("1/2", "5", "2/5", "3/5");
    test("3/4", "4", "2/3", "1");
    // - the order equals the denominator, so the neighbors must use other denominators
    test("2/5", "5", "1/3", "1/2");
    // - zero
    test("0", "1", "-1", "1");
    test("0", "5", "-1/5", "1/5");
    // - integers, whose neighbors are one unit of the largest denominator away
    test("1", "1", "0", "2");
    test("2", "1", "1", "3");
    test("2", "3", "5/3", "7/3");
    // - negative
    test("-3/4", "7", "-4/5", "-5/7");
    test("-2", "3", "-7/3", "-5/3");
    // - outside [0, 1], with a large order
    test("22/7", "100", "311/99", "305/97");
    // - multi-limb, with an order large enough to admit the input itself
    test(
        "12345678987654321012345678901/98765432123456789012345678901",
        "1000000000000000000000000000000",
        "116401027992572581504050862378/931208226035799147750641393479",
        "118166872772859517730517036741/945334984309879843483926505640",
    );
}

#[test]
#[should_panic]
fn farey_neighbors_fail() {
    Rational::from_str("1/5")
        .unwrap()
        .farey_neighbors(&Natural::from(4u32));
}

#[test]
fn farey_neighbors_properties() {
    rational_natural_pair_gen_var_5().test_properties(|(x, n)| {
        let (l, r) = x.farey_neighbors(&n);
        assert!(l.is_valid());
        assert!(r.is_valid());
        // The neighbors bracket x, and their denominators respect the order.
        assert!(l < x);
        assert!(x < r);
        assert!(l.to_denominator() <= n);
        assert!(r.to_denominator() <= n);
        // Each neighbor is adjacent to x in the Farey sense: consecutive fractions a/b and c/d
        // satisfy |ad - bc| = 1, so the gap is exactly the reciprocal of the product of the
        // denominators. (The two neighbors are not adjacent to each other; x lies between them.)
        let d = x.to_denominator();
        assert_eq!(
            &x - &l,
            Rational::from_naturals(Natural::ONE, l.to_denominator() * &d)
        );
        assert_eq!(
            &r - &x,
            Rational::from_naturals(Natural::ONE, r.to_denominator() * d)
        );
        // Nothing with a small enough denominator fits between a neighbor and x, so the closer
        // neighbor is the best approximation.
        let approx = (&x).approximate(&n);
        assert!(approx == x || approx == l || approx == r);
        // The negation reflects the pair.
        let (nl, nr) = (-&x).farey_neighbors(&n);
        assert_eq!(nl, -&r);
        assert_eq!(nr, -l);
    });
}
