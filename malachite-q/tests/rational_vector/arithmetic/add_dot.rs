// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{AddDot, AddDotAssign, Dot, SubDot};
use malachite_q::Rational;
use malachite_q::rational_vector::RationalVector;
use malachite_q::test_util::generators::rational_vector_rational_vector_rational_triple_gen_var_1;

#[test]
fn test_add_dot() {
    let test = |x, v, w, out| {
        let x = Rational::from_str(x).unwrap();
        let v = RationalVector::from_str(v).unwrap();
        let w = RationalVector::from_str(w).unwrap();
        let r = (&x).add_dot(&v, &w);
        assert!(r.is_valid());
        assert_eq!(r.to_string(), out);
        assert_eq!(x.clone().add_dot(v.clone(), w.clone()), r);
        assert_eq!(x.clone().add_dot(v.clone(), &w), r);
        assert_eq!(x.clone().add_dot(&v, w.clone()), r);
        assert_eq!(x.clone().add_dot(&v, &w), r);
        let mut y = x.clone();
        y.add_dot_assign(v.clone(), w.clone());
        assert_eq!(y, r);
        let mut y = x.clone();
        y.add_dot_assign(v.clone(), &w);
        assert_eq!(y, r);
        let mut y = x.clone();
        y.add_dot_assign(&v, w.clone());
        assert_eq!(y, r);
        let mut y = x.clone();
        y.add_dot_assign(&v, &w);
        assert_eq!(y, r);
    };
    test("0", "()", "()", "0");
    test("1/3", "(1/2, -2/3)", "(4/5, 3/2)", "-4/15");
    test("-2", "(3/7)", "(-14/9)", "-8/3");
    test(
        "1/1000000000000",
        "(1000000000000/7, 1)",
        "(7/1000000000000, 1/3)",
        "4000000000003/3000000000000",
    );
    test(
        "18446744073709551616/3",
        "(18446744073709551615, -1/2)",
        "(-18446744073709551615, 5/18446744073709551616)",
        "-37662610412320084578250783402352860008237568627181709426703/110680464442257309696",
    );
}

#[test]
#[should_panic]
fn add_dot_fail_1() {
    Rational::from_str("1").unwrap().add_dot(
        RationalVector::from_str("(1, 2)").unwrap(),
        RationalVector::from_str("(1)").unwrap(),
    );
}

#[test]
fn add_dot_properties() {
    rational_vector_rational_vector_rational_triple_gen_var_1().test_properties(|(v, w, x)| {
        let r = (&x).add_dot(&v, &w);
        assert!(r.is_valid());
        assert_eq!(x.clone().add_dot(v.clone(), w.clone()), r);
        assert_eq!(x.clone().add_dot(v.clone(), &w), r);
        assert_eq!(x.clone().add_dot(&v, w.clone()), r);
        assert_eq!(x.clone().add_dot(&v, &w), r);
        let mut y = x.clone();
        y.add_dot_assign(v.clone(), w.clone());
        assert_eq!(y, r);
        let mut y = x.clone();
        y.add_dot_assign(v.clone(), &w);
        assert_eq!(y, r);
        let mut y = x.clone();
        y.add_dot_assign(&v, w.clone());
        assert_eq!(y, r);
        let mut y = x.clone();
        y.add_dot_assign(&v, &w);
        assert_eq!(y, r);
        // It is the dot product added to `x`.
        assert_eq!(r, &x + (&v).dot(&w));
        // Subtracting it back recovers `x`.
        assert_eq!((&r).sub_dot(&v, &w), x);
    });
}
