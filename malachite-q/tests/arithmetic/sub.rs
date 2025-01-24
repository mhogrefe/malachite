// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::traits::Zero;
use malachite_nz::test_util::generators::integer_pair_gen;
use malachite_q::test_util::arithmetic::sub::sub_naive;
use malachite_q::test_util::generators::{rational_gen, rational_pair_gen};
use malachite_q::Rational;
use num::BigRational;
use rug;
use std::str::FromStr;

#[test]
fn test_sub() {
    let test = |s, t, out| {
        let u = Rational::from_str(s).unwrap();
        let v = Rational::from_str(t).unwrap();

        let mut n = u.clone();
        n -= v.clone();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let mut n = u.clone();
        n -= &v;
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = u.clone() - v.clone();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = &u - v.clone();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = u.clone() - &v;
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = &u - &v;
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = BigRational::from_str(s).unwrap() - BigRational::from_str(t).unwrap();
        assert_eq!(n.to_string(), out);

        let n = rug::Rational::from_str(s).unwrap() - rug::Rational::from_str(t).unwrap();
        assert_eq!(n.to_string(), out);
    };
    test("0", "0", "0");
    test("0", "-123", "123");
    test("123", "0", "123");
    test("123", "-456", "579");
    test("0", "123", "-123");
    test("123", "123", "0");
    test("123", "456", "-333");
    test("0", "-123", "123");
    test("-123", "0", "-123");
    test("-123", "-456", "333");
    test("0", "123", "-123");
    test("-123", "-123", "0");
    test("1/2", "-1/3", "5/6");
    test("1/2", "1/3", "1/6");
    test("-1/2", "-1/3", "-1/6");
    test("-1/2", "1/3", "-5/6");
    test("1/2", "-1/2", "1");
    test("1/2", "1/2", "0");
    test("-1/2", "-1/2", "0");
    test("-1/2", "1/2", "-1");
}

#[allow(clippy::eq_op)]
#[test]
fn sub_properties() {
    rational_pair_gen().test_properties(|(x, y)| {
        let diff_val_val = x.clone() - y.clone();
        let diff_val_ref = x.clone() - &y;
        let diff_ref_val = &x - y.clone();
        let diff = &x - &y;
        assert!(diff_val_val.is_valid());
        assert!(diff_val_ref.is_valid());
        assert!(diff_ref_val.is_valid());
        assert!(diff.is_valid());
        assert_eq!(diff_val_val, diff);
        assert_eq!(diff_val_ref, diff);
        assert_eq!(diff_ref_val, diff);

        let mut mut_x = x.clone();
        mut_x -= y.clone();
        assert!(mut_x.is_valid());
        assert_eq!(mut_x, diff);
        let mut mut_x = x.clone();
        mut_x -= &y;
        assert!(mut_x.is_valid());
        assert_eq!(mut_x, diff);

        let mut mut_x = rug::Rational::from(&x);
        mut_x -= rug::Rational::from(&y);
        assert_eq!(Rational::from(&mut_x), diff);

        assert_eq!(
            Rational::from(&(BigRational::from(&x) - BigRational::from(&y))),
            diff
        );
        assert_eq!(
            Rational::from(&(rug::Rational::from(&x) - rug::Rational::from(&y))),
            diff
        );
        assert_eq!(sub_naive(x.clone(), y.clone()), diff);
        assert_eq!(&y - &x, -&diff);
        assert_eq!(&diff + &y, x);
        assert_eq!(x - diff, y);
    });

    rational_gen().test_properties(|ref x| {
        assert_eq!(x - Rational::ZERO, *x);
        assert_eq!(Rational::ZERO - x, -x);
        assert_eq!(x - -x, x << 1);
        assert_eq!(x - x, 0);
    });

    integer_pair_gen().test_properties(|(x, y)| {
        if x >= y {
            assert_eq!(&x - &y, Rational::from(x) - Rational::from(y));
        } else {
            assert_eq!(-(&y - &x), Rational::from(x) - Rational::from(y));
        }
    });
}
