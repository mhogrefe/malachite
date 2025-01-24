// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_nz::integer::Integer;
use malachite_nz::test_util::generators::integer_pair_gen;
use malachite_q::test_util::generators::{
    rational_integer_integer_triple_gen, rational_integer_pair_gen,
    rational_rational_integer_triple_gen,
};
use malachite_q::Rational;
use std::cmp::Ordering::{self, *};
use std::str::FromStr;

#[test]
fn test_partial_cmp_integer() {
    let test = |s, t, out| {
        let u = Rational::from_str(s).unwrap();
        let v = Integer::from_str(t).unwrap();

        assert_eq!(u.partial_cmp(&v), out);
        assert_eq!(v.partial_cmp(&u).map(Ordering::reverse), out);
    };
    test("0", "0", Some(Equal));
    test("0", "5", Some(Less));
    test("123", "123", Some(Equal));
    test("123", "124", Some(Less));
    test("123", "122", Some(Greater));
    test("1000000000000", "123", Some(Greater));
    test("123", "1000000000000", Some(Less));
    test("1000000000000", "1000000000000", Some(Equal));
    test("-1000000000000", "1000000000000", Some(Less));
    test("-1000000000000", "0", Some(Less));

    test("0", "-5", Some(Greater));
    test("-123", "-123", Some(Equal));
    test("-123", "-124", Some(Greater));
    test("-123", "-122", Some(Less));
    test("-1000000000000", "-123", Some(Less));
    test("-123", "-1000000000000", Some(Greater));
    test("-1000000000000", "-1000000000000", Some(Equal));
    test("1000000000000", "-1000000000000", Some(Greater));
    test("1000000000000", "0", Some(Greater));

    test("99/100", "1", Some(Less));
    test("101/100", "1", Some(Greater));
    test("22/7", "3", Some(Greater));
    test("22/7", "4", Some(Less));
    test("-99/100", "-1", Some(Greater));
    test("-101/100", "-1", Some(Less));
    test("-22/7", "-3", Some(Less));
    test("-22/7", "-4", Some(Greater));
}

#[test]
fn partial_cmp_integer_properties() {
    rational_integer_pair_gen().test_properties(|(x, y)| {
        let cmp = x.partial_cmp(&y);
        assert_eq!(x.cmp(&Rational::from(&y)), cmp.unwrap());
        assert_eq!(
            rug::Rational::from(&x).partial_cmp(&rug::Integer::from(&y)),
            cmp
        );
        assert_eq!(y.partial_cmp(&x), cmp.map(Ordering::reverse));
    });

    rational_rational_integer_triple_gen().test_properties(|(x, z, y)| {
        if x < y && y < z {
            assert!(x < z);
        } else if x > y && y > z {
            assert!(x > z);
        }
    });

    rational_integer_integer_triple_gen().test_properties(|(y, x, z)| {
        if x < y && y < z {
            assert!(x < z);
        } else if x > y && y > z {
            assert!(x > z);
        }
    });

    integer_pair_gen().test_properties(|(x, y)| {
        assert_eq!(Rational::from(&x).partial_cmp(&y), Some(x.cmp(&y)));
        assert_eq!(x.partial_cmp(&Rational::from(&y)), Some(x.cmp(&y)));
    });
}
