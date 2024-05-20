// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::test_util::common::test_custom_cmp_helper;
use malachite_q::arithmetic::traits::SimplestRationalInInterval;
use malachite_q::test_util::arithmetic::simplest_rational_in_interval::*;
use malachite_q::test_util::generators::{
    rational_gen, rational_pair_gen, rational_pair_gen_var_3, rational_pair_gen_var_4,
    rational_pair_gen_var_5, rational_pair_gen_var_6, rational_triple_gen,
    rational_triple_gen_var_2, rational_triple_gen_var_3,
};
use malachite_q::Rational;
use std::cmp::Ordering::*;
use std::str::FromStr;

#[test]
fn test_cmp_complexity() {
    let strings = &["0", "1", "-1", "1/2", "-1/2", "5/2", "1/100", "99/100", "-99/100"];
    test_custom_cmp_helper::<Rational, _>(strings, Rational::cmp_complexity);
}

#[test]
fn test_simplest_rational_in_open_interval() {
    let test = |x, y, out| {
        let x = Rational::from_str(x).unwrap();
        let y = Rational::from_str(y).unwrap();
        assert_eq!(
            Rational::simplest_rational_in_open_interval(&x, &y).to_string(),
            out
        );
        assert_eq!(
            simplest_rational_in_open_interval_explicit(&x, &y).to_string(),
            out
        );
        assert_eq!(
            simplest_rational_in_open_interval_naive(&x, &y).to_string(),
            out
        );
    };
    test("0", "2", "1");
    test("0", "1", "1/2");
    test("-1", "1", "0");
    test("-9", "10", "0");
    test("0", "1/2", "1/3");
    test("1/2", "1", "2/3");
    test("0", "1/3", "1/4");
    test("1/3", "1", "1/2");
    test("1/3", "1/2", "2/5");
    test("157/50", "63/20", "22/7");
    test("2/3", "1", "3/4");
    test("1/2", "2/3", "3/5");
    test("1/2", "3/5", "4/7");
    test("3/5", "2/3", "5/8");
    test("2/3", "3/4", "5/7");
}

#[test]
#[should_panic]
fn simplest_rational_in_open_interval_fail_1() {
    Rational::simplest_rational_in_open_interval(&Rational::ONE, &Rational::ONE);
}

#[test]
#[should_panic]
fn simplest_rational_in_open_interval_fail_2() {
    Rational::simplest_rational_in_open_interval(&Rational::ONE, &Rational::ZERO);
}

#[test]
fn test_simplest_rational_in_closed_interval() {
    let test = |x, y, out| {
        let x = Rational::from_str(x).unwrap();
        let y = Rational::from_str(y).unwrap();
        assert_eq!(
            Rational::simplest_rational_in_closed_interval(&x, &y).to_string(),
            out
        );
    };
    test("0", "2", "0");
    test("0", "1", "0");
    test("-1", "1", "0");
    test("-9", "10", "0");
    test("0", "1/2", "0");
    test("1/2", "1", "1");
    test("0", "1/3", "0");
    test("1/3", "1", "1");
    test("1/3", "1/2", "1/2");
    test("157/50", "63/20", "22/7");
}

#[test]
#[should_panic]
fn simplest_rational_in_closed_interval_fail() {
    Rational::simplest_rational_in_closed_interval(&Rational::ONE, &Rational::ZERO);
}

#[test]
fn cmp_complexity_properties() {
    rational_pair_gen().test_properties(|(x, y)| {
        let ord = x.cmp_complexity(&y);
        assert_eq!(y.cmp_complexity(&x).reverse(), ord);
        assert_eq!(x == y, x.cmp_complexity(&y) == Equal);
    });

    rational_gen().test_properties(|x| {
        assert_eq!(x.cmp_complexity(&x), Equal);
    });

    rational_triple_gen().test_properties(|(x, y, z)| {
        if x.cmp_complexity(&y) == Less && y.cmp_complexity(&z) == Less {
            assert!(x.cmp_complexity(&z) == Less);
        } else if x.cmp_complexity(&y) == Greater && y.cmp_complexity(&z) == Greater {
            assert!(x.cmp_complexity(&z) == Greater);
        }
    });
}

#[test]
fn simplest_rational_in_open_interval_properties() {
    rational_pair_gen_var_3().test_properties(|(x, y)| {
        let s = Rational::simplest_rational_in_open_interval(&x, &y);
        assert!(s.is_valid());
        assert_eq!(simplest_rational_in_open_interval_explicit(&x, &y), s);
        assert!(s > x);
        assert!(s < y);
        assert_eq!(Rational::simplest_rational_in_open_interval(&-y, &-x), -s);
    });

    rational_pair_gen_var_5().test_properties(|(x, y)| {
        assert_eq!(
            simplest_rational_in_open_interval_naive(&x, &y),
            Rational::simplest_rational_in_open_interval(&x, &y)
        );
    });

    rational_triple_gen_var_2().test_properties(|(x, y, z)| {
        let q = Rational::simplest_rational_in_open_interval(&x, &z);
        assert!(q.cmp_complexity(&y) <= Equal);
    });
}

#[test]
fn simplest_rational_in_closed_interval_properties() {
    rational_pair_gen_var_4().test_properties(|(x, y)| {
        let s = Rational::simplest_rational_in_closed_interval(&x, &y);
        assert!(s.is_valid());
        assert!(s >= x);
        assert!(s <= y);
        assert_eq!(Rational::simplest_rational_in_closed_interval(&-y, &-x), -s);
    });

    rational_pair_gen_var_6().test_properties(|(x, y)| {
        assert_eq!(
            Rational::simplest_rational_in_closed_interval(&x, &y),
            simplest_rational_in_closed_interval_naive(&x, &y)
        );
    });

    rational_triple_gen_var_3().test_properties(|(x, y, z)| {
        let q = Rational::simplest_rational_in_closed_interval(&x, &z);
        assert!(q.cmp_complexity(&y) <= Equal);
    });

    rational_gen().test_properties(|x| {
        assert_eq!(Rational::simplest_rational_in_closed_interval(&x, &x), x);
    });
}
