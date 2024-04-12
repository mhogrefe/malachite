// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_nz::integer::Integer;
use malachite_nz::test_util::generators::integer_pair_gen;
use malachite_q::test_util::generators::rational_integer_pair_gen;
use malachite_q::Rational;
use rug;
use std::str::FromStr;

#[test]
fn test_rational_partial_eq_integer() {
    let test = |s, t, out| {
        let u = Rational::from_str(s).unwrap();
        let v = Integer::from_str(t).unwrap();

        assert_eq!(u == v, out);
        assert_eq!(v == u, out);
        assert_eq!(
            rug::Rational::from_str(s).unwrap() == rug::Rational::from_str(t).unwrap(),
            out
        );
    };
    test("0", "0", true);
    test("0", "5", false);
    test("0", "-5", false);
    test("123", "123", true);
    test("123", "-123", false);
    test("-123", "123", false);
    test("-123", "-123", true);
    test("123", "5", false);
    test("123", "-5", false);
    test("1000000000000", "123", false);
    test("123", "1000000000000", false);
    test("1000000000000", "1000000000000", true);
    test("-1000000000000", "1000000000000", false);
    test("-1000000000000", "-1000000000000", true);
    test("22/7", "3", false);
    test("1/2", "2", false);
    test("-1/2", "2", false);
    test("-1/2", "-2", false);
}

#[allow(clippy::cmp_owned)]
#[test]
fn partial_eq_integer_properties() {
    rational_integer_pair_gen().test_properties(|(x, y)| {
        let eq = x == y;
        assert_eq!(y == x, eq);
        assert_eq!(x == Rational::from(&y), eq);
        assert_eq!(rug::Rational::from(&x) == rug::Integer::from(&y), eq);
    });

    integer_pair_gen().test_properties(|(x, y)| {
        assert_eq!(Rational::from(&x) == y, x == y);
        assert_eq!(x == Rational::from(&y), x == y);
    });
}
