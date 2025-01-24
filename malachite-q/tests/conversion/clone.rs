// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_nz::test_util::generators::integer_pair_gen;
use malachite_q::test_util::generators::{rational_gen, rational_pair_gen};
use malachite_q::Rational;
use num::BigRational;
use rug;
use std::str::FromStr;

#[test]
#[allow(clippy::redundant_clone)]
fn test_clone() {
    let test = |u| {
        let x = Rational::from_str(u).unwrap().clone();
        assert_eq!(x.to_string(), u);
        assert!(x.is_valid());

        let x = BigRational::from_str(u).unwrap().clone();
        assert_eq!(x.to_string(), u);

        let x = rug::Rational::from_str(u).unwrap().clone();
        assert_eq!(x.to_string(), u);
    };
    test("123");
    test("1000000000000");
    test("-123");
    test("-1000000000000");
    test("22/7");
    test("-22/7");
    test("100/101");
    test("-100/101");
}

#[test]
fn test_clone_from() {
    let test = |u, v| {
        let mut x = Rational::from_str(u).unwrap();
        x.clone_from(&Rational::from_str(v).unwrap());
        assert_eq!(x.to_string(), v);
        assert!(x.is_valid());

        let mut x = BigRational::from_str(u).unwrap();
        x.clone_from(&BigRational::from_str(v).unwrap());
        assert_eq!(x.to_string(), v);

        let mut x = rug::Rational::from_str(u).unwrap();
        x.clone_from(&rug::Rational::from_str(v).unwrap());
        assert_eq!(x.to_string(), v);
    };
    test("-123", "456");
    test("-123", "1000000000000");
    test("1000000000000", "-123");
    test("1000000000000", "2000000000000");
    test("123", "22/7");
    test("123", "-22/7");
    test("-123", "22/7");
    test("-123", "-22/7");
}

#[allow(clippy::redundant_clone)]
#[test]
fn clone_and_clone_from_properties() {
    rational_gen().test_properties(|x| {
        let mut_x = x.clone();
        assert!(mut_x.is_valid());
        assert_eq!(mut_x, x);

        assert_eq!(Rational::from(&BigRational::from(&x).clone()), x);
        assert_eq!(Rational::from(&rug::Rational::from(&x).clone()), x);
    });

    rational_pair_gen().test_properties(|(x, y)| {
        let mut mut_x = x.clone();
        mut_x.clone_from(&y);
        assert!(mut_x.is_valid());
        assert_eq!(mut_x, y);

        let mut num_x = BigRational::from(&x);
        num_x.clone_from(&BigRational::from(&y));
        assert_eq!(Rational::from(&num_x), y);

        let mut rug_x = rug::Rational::from(&x);
        rug_x.clone_from(&rug::Rational::from(&y));
        assert_eq!(Rational::from(&rug_x), y);
    });

    integer_pair_gen().test_properties(|(i, j)| {
        let x = Rational::from(&i);
        let y = Rational::from(&j);

        let mut mut_i = i.clone();
        let mut mut_x = x.clone();
        mut_i.clone_from(&j);
        mut_x.clone_from(&y);
        assert_eq!(mut_x, mut_i);
    });
}
