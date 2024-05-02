// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{DivisibleBy, DivisibleByPowerOf2, Parity};
use malachite_base::num::basic::traits::{One, Two};
use malachite_base::test_util::generators::signed_gen;
use malachite_nz::integer::Integer;
use malachite_nz::platform::SignedLimb;
use malachite_nz::test_util::generators::integer_gen;
use std::str::FromStr;

#[test]
fn test_even() {
    let test = |n, out| {
        assert_eq!(Integer::from_str(n).unwrap().even(), out);
    };
    test("0", true);
    test("1", false);
    test("2", true);
    test("3", false);
    test("123", false);
    test("1000000000000", true);
    test("1000000000001", false);
    test("-1", false);
    test("-2", true);
    test("-3", false);
    test("-123", false);
    test("-1000000000000", true);
    test("-1000000000001", false);
}

#[test]
fn test_odd() {
    let test = |n, out| {
        assert_eq!(Integer::from_str(n).unwrap().odd(), out);
    };
    test("0", false);
    test("1", true);
    test("2", false);
    test("3", true);
    test("123", true);
    test("1000000000000", false);
    test("1000000000001", true);
    test("-1", true);
    test("-2", false);
    test("-3", true);
    test("-123", true);
    test("-1000000000000", false);
    test("-1000000000001", true);
}

#[test]
fn even_properties() {
    integer_gen().test_properties(|ref x| {
        let even = x.even();
        assert_eq!(!x.odd(), even);
        assert_eq!(x.divisible_by(Integer::TWO), even);
        assert_eq!(x.divisible_by_power_of_2(1), even);
        assert_eq!((x + Integer::ONE).odd(), even);
        assert_eq!((x - Integer::ONE).odd(), even);
        assert_eq!((-x).even(), even);
    });

    signed_gen::<SignedLimb>().test_properties(|i| {
        assert_eq!(i.even(), Integer::from(i).even());
    });
}

#[test]
fn odd_properties() {
    integer_gen().test_properties(|ref x| {
        let odd = x.odd();
        assert_eq!(!x.even(), odd);
        assert_eq!(!x.divisible_by(Integer::TWO), odd);
        assert_eq!(!(x).divisible_by_power_of_2(1), odd);
        assert_eq!((x + Integer::ONE).even(), odd);
        assert_eq!((x - Integer::ONE).even(), odd);
        assert_eq!((-x).odd(), odd);
    });

    signed_gen::<SignedLimb>().test_properties(|i| {
        assert_eq!(i.odd(), Integer::from(i).odd());
    });
}
