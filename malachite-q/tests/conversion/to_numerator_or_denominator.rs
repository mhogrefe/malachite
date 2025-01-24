// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{Abs, UnsignedAbs};
use malachite_base::num::basic::traits::One;
use malachite_base::strings::ToDebugString;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_nz::test_util::generators::integer_gen;
use malachite_q::test_util::generators::rational_gen;
use malachite_q::Rational;
use num::{BigRational, Signed};
use std::str::FromStr;

#[test]
fn test_to_numerator() {
    let test = |s, n| {
        let q = Rational::from_str(s).unwrap();
        assert_eq!(q.to_numerator().to_string(), n);
        assert_eq!(q.clone().into_numerator().to_string(), n);
        assert_eq!(q.numerator_ref().to_string(), n);

        assert_eq!(
            BigRational::from_str(s).unwrap().numer().abs().to_string(),
            n
        );
        assert_eq!(
            rug::Rational::from_str(s)
                .unwrap()
                .numer()
                .clone()
                .abs()
                .to_string(),
            n
        );
    };
    test("0", "0");
    test("1/2", "1");
    test("-1/2", "1");
    test("100/101", "100");
    test("-100/101", "100");
}

#[test]
fn to_numerator_properties() {
    rational_gen().test_properties(|q| {
        let n = q.to_numerator();
        assert_eq!(q.clone().into_numerator(), n);
        assert_eq!(*q.numerator_ref(), n);
        assert_eq!(q.to_numerator_and_denominator().0, n);

        assert_eq!(Integer::from(BigRational::from(&q).numer()).abs(), n);
        assert_eq!(Integer::from(rug::Rational::from(&q).numer()).abs(), n);
    });

    integer_gen().test_properties(|n| {
        assert_eq!(Rational::from(&n).into_numerator(), n.abs());
    });
}

#[test]
fn test_to_denominator() {
    let test = |s, d| {
        let q = Rational::from_str(s).unwrap();
        assert_eq!(q.to_denominator().to_string(), d);
        assert_eq!(q.clone().into_denominator().to_string(), d);
        assert_eq!(q.denominator_ref().to_string(), d);

        assert_eq!(
            BigRational::from_str(s).unwrap().denom().abs().to_string(),
            d
        );
        assert_eq!(
            rug::Rational::from_str(s)
                .unwrap()
                .denom()
                .clone()
                .abs()
                .to_string(),
            d
        );
    };
    test("0", "1");
    test("1/2", "2");
    test("-1/2", "2");
    test("100/101", "101");
    test("-100/101", "101");
}

#[test]
fn to_denominator_properties() {
    rational_gen().test_properties(|q| {
        let d = q.to_denominator();
        assert_eq!(q.clone().into_denominator(), d);
        assert_eq!(*q.denominator_ref(), d);
        assert_eq!(q.to_numerator_and_denominator().1, d);

        assert_eq!(Integer::from(BigRational::from(&q).denom()).abs(), d);
        assert_eq!(Integer::from(rug::Rational::from(&q).denom()).abs(), d);
    });

    integer_gen().test_properties(|n| {
        assert_eq!(Rational::from(&n).into_denominator(), 1);
    });
}

#[test]
fn test_to_numerator_and_denominator() {
    let test = |s, nd| {
        let q = Rational::from_str(s).unwrap();
        assert_eq!(q.to_numerator_and_denominator().to_debug_string(), nd);
        assert_eq!(
            q.clone().into_numerator_and_denominator().to_debug_string(),
            nd
        );
        assert_eq!(q.numerator_and_denominator_ref().to_debug_string(), nd);
    };
    test("0", "(0, 1)");
    test("1/2", "(1, 2)");
    test("-1/2", "(1, 2)");
    test("100/101", "(100, 101)");
    test("-100/101", "(100, 101)");
}

#[test]
fn to_numerator_and_denominator_properties() {
    rational_gen().test_properties(|q| {
        let (n, d) = q.to_numerator_and_denominator();
        assert_eq!(
            q.clone().into_numerator_and_denominator(),
            (n.clone(), d.clone())
        );
        assert_eq!(q.numerator_and_denominator_ref(), (&n, &d));

        assert_eq!(Rational::from_naturals(n, d), q.abs());
    });

    integer_gen().test_properties(|n| {
        assert_eq!(
            Rational::from(&n).into_numerator_and_denominator(),
            (n.unsigned_abs(), Natural::ONE)
        );
    });
}
