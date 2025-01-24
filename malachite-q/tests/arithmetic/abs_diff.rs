// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{Abs, AbsDiff, AbsDiffAssign};
use malachite_base::num::basic::traits::Zero;
use malachite_nz::test_util::generators::integer_pair_gen;
use malachite_q::test_util::generators::{rational_gen, rational_pair_gen, rational_triple_gen};
use malachite_q::Rational;
use std::str::FromStr;

#[test]
fn test_abs_diff_rational() {
    let test = |s, t, out| {
        let u = Rational::from_str(s).unwrap();
        let v = Rational::from_str(t).unwrap();

        let mut n = u.clone();
        n.abs_diff_assign(v.clone());
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let mut n = u.clone();
        n.abs_diff_assign(&v);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = u.clone().abs_diff(v.clone());
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = u.clone().abs_diff(&v);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = (&u).abs_diff(v.clone());
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = (&u).abs_diff(&v);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());
    };
    test("0", "0", "0");
    test("0", "-123", "123");
    test("123", "0", "123");
    test("123", "-456", "579");
    test("0", "123", "123");
    test("123", "123", "0");
    test("123", "456", "333");
    test("0", "-123", "123");
    test("-123", "0", "123");
    test("-123", "-456", "333");
    test("0", "123", "123");
    test("-123", "-123", "0");
    test("1/2", "-1/3", "5/6");
    test("1/2", "1/3", "1/6");
    test("-1/2", "-1/3", "1/6");
    test("-1/2", "1/3", "5/6");
    test("1/2", "-1/2", "1");
    test("1/2", "1/2", "0");
    test("-1/2", "-1/2", "0");
    test("-1/2", "1/2", "1");
}

#[test]
fn abs_diff_properties() {
    rational_pair_gen().test_properties(|(x, y)| {
        let mut mut_x = x.clone();
        mut_x.abs_diff_assign(&y);
        assert!(mut_x.is_valid());
        let diff = mut_x;
        assert!(diff >= 0);

        let mut mut_x = x.clone();
        mut_x.abs_diff_assign(y.clone());
        assert!(mut_x.is_valid());
        assert_eq!(mut_x, diff);

        let diff_alt = x.clone().abs_diff(y.clone());
        assert_eq!(diff_alt, diff);
        assert!(diff_alt.is_valid());

        let diff_alt = x.clone().abs_diff(&y);
        assert_eq!(diff_alt, diff);
        assert!(diff_alt.is_valid());

        let diff_alt = (&x).abs_diff(y.clone());
        assert_eq!(diff_alt, diff);
        assert!(diff_alt.is_valid());

        let diff_alt = (&x).abs_diff(&y);
        assert_eq!(diff_alt, diff);
        assert!(diff_alt.is_valid());

        assert_eq!((&x - &y).abs(), diff);
        assert_eq!((&y).abs_diff(&x), diff);
        assert_eq!((-&x).abs_diff(-&y), diff);
        assert_eq!(diff == 0, x == y);
    });

    rational_gen().test_properties(|x| {
        assert_eq!((&x).abs_diff(Rational::ZERO), (&x).abs());
        assert_eq!((&x).abs_diff(&x), 0);
        assert_eq!(Rational::ZERO.abs_diff(&x), x.abs());
    });

    rational_triple_gen().test_properties(|(x, y, z)| {
        assert!((&x).abs_diff(&z) <= x.abs_diff(&y) + y.abs_diff(z));
    });

    integer_pair_gen().test_properties(|(x, y)| {
        assert_eq!(
            (&x).abs_diff(&y),
            Rational::from(x).abs_diff(Rational::from(y))
        );
    });
}
