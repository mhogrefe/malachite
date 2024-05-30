// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::traits::Zero;
use malachite_base::vecs::vec_from_str;
use malachite_nz::integer::Integer;
use malachite_nz::test_util::generators::integer_pair_gen;
use malachite_nz::test_util::generators::integer_vec_gen;
use malachite_q::test_util::arithmetic::add::add_naive;
use malachite_q::test_util::arithmetic::add::rational_sum_naive;
use malachite_q::test_util::generators::{
    rational_gen, rational_pair_gen, rational_triple_gen, rational_vec_gen,
};
use malachite_q::Rational;
use num::BigRational;
use std::iter::{once, Sum};
use std::str::FromStr;

#[test]
fn test_add() {
    let test = |s, t, out| {
        let u = Rational::from_str(s).unwrap();
        let v = Rational::from_str(t).unwrap();

        let mut n = u.clone();
        n += v.clone();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let mut n = u.clone();
        n += &v;
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = u.clone() + v.clone();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = &u + v.clone();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = u.clone() + &v;
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = &u + &v;
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = BigRational::from_str(s).unwrap() + BigRational::from_str(t).unwrap();
        assert_eq!(n.to_string(), out);

        let n = rug::Rational::from_str(s).unwrap() + rug::Rational::from_str(t).unwrap();
        assert_eq!(n.to_string(), out);
    };
    test("0", "0", "0");
    test("0", "123", "123");
    test("123", "0", "123");
    test("123", "456", "579");
    test("0", "-123", "-123");
    test("123", "-123", "0");
    test("123", "-456", "-333");
    test("0", "123", "123");
    test("-123", "0", "-123");
    test("-123", "456", "333");
    test("0", "-123", "-123");
    test("-123", "123", "0");
    test("1/2", "1/3", "5/6");
    test("1/2", "-1/3", "1/6");
    test("-1/2", "1/3", "-1/6");
    test("-1/2", "-1/3", "-5/6");
    test("1/2", "1/2", "1");
    test("1/2", "-1/2", "0");
    test("-1/2", "1/2", "0");
    test("-1/2", "-1/2", "-1");
}

#[test]
fn test_sum() {
    let test = |xs, out| {
        let xs = vec_from_str(xs).unwrap();
        let sum = Rational::sum(xs.iter().cloned());
        assert!(sum.is_valid());
        assert_eq!(sum.to_string(), out);

        let sum_alt = Rational::sum(xs.iter());
        assert!(sum_alt.is_valid());
        assert_eq!(sum_alt, sum);

        let sum_alt = rational_sum_naive(xs.into_iter());
        assert!(sum_alt.is_valid());
        assert_eq!(sum_alt, sum);
    };
    test("[]", "0");
    test("[22/7]", "22/7");
    test("[22/7, 1/3]", "73/21");
    test(
        "[0, 1, 2/3, 3/4, 4/5, 5/6, 6/7, 7/8, 8/9, 9/10]",
        "19079/2520",
    );
    test(
        "[123456/78901, 34567/890123, 45678/90123]",
        "342501191973781/162294410775211",
    );
}

#[test]
fn add_properties() {
    rational_pair_gen().test_properties(|(x, y)| {
        let sum_val_val = x.clone() + y.clone();
        let sum_val_ref = x.clone() + &y;
        let sum_ref_val = &x + y.clone();
        let sum = &x + &y;
        assert!(sum_val_val.is_valid());
        assert!(sum_val_ref.is_valid());
        assert!(sum_ref_val.is_valid());
        assert!(sum.is_valid());
        assert_eq!(sum_val_val, sum);
        assert_eq!(sum_val_ref, sum);
        assert_eq!(sum_ref_val, sum);

        let mut mut_x = x.clone();
        mut_x += y.clone();
        assert!(mut_x.is_valid());
        assert_eq!(mut_x, sum);
        let mut mut_x = x.clone();
        mut_x += &y;
        assert_eq!(mut_x, sum);
        assert!(mut_x.is_valid());

        let mut mut_x = rug::Rational::from(&x);
        mut_x += rug::Rational::from(&y);
        assert_eq!(Rational::from(&mut_x), sum);

        assert_eq!(
            Rational::from(&(BigRational::from(&x) + BigRational::from(&y))),
            sum
        );
        assert_eq!(
            Rational::from(&(rug::Rational::from(&x) + rug::Rational::from(&y))),
            sum
        );
        assert_eq!(add_naive(x.clone(), y.clone()), sum);
        assert_eq!(&y + &x, sum);
        assert_eq!(&sum - &x, y);
        assert_eq!(sum - y, x);
    });

    rational_gen().test_properties(|ref x| {
        assert_eq!(x + Rational::ZERO, *x);
        assert_eq!(Rational::ZERO + x, *x);
        assert_eq!(x + x, x << 1);
        assert_eq!(x + (-x), 0);
    });

    rational_triple_gen().test_properties(|(x, y, z)| {
        assert_eq!((&x + &y) + &z, x + (y + z));
    });

    integer_pair_gen().test_properties(|(x, y)| {
        assert_eq!(&x + &y, Rational::from(x) + Rational::from(y));
    });
}

#[test]
fn sum_properties() {
    rational_vec_gen().test_properties(|xs| {
        let sum = Rational::sum(xs.iter().cloned());
        assert!(sum.is_valid());

        let sum_alt = Rational::sum(xs.iter());
        assert!(sum_alt.is_valid());
        assert_eq!(sum_alt, sum);

        let sum_alt = rational_sum_naive(xs.into_iter());
        assert!(sum_alt.is_valid());
        assert_eq!(sum_alt, sum);
    });

    rational_gen().test_properties(|x| {
        assert_eq!(Rational::sum(once(&x)), x);
        assert_eq!(Rational::sum(once(x.clone())), x);
    });

    rational_pair_gen().test_properties(|(x, y)| {
        let sum = &x + &y;
        assert_eq!(Rational::sum([&x, &y].into_iter()), sum);
        assert_eq!(Rational::sum([x, y].into_iter()), sum);
    });

    integer_vec_gen().test_properties(|xs| {
        assert_eq!(
            Rational::sum(xs.iter().map(Rational::from)),
            Rational::from(Integer::sum(xs.into_iter()))
        );
    });
}
