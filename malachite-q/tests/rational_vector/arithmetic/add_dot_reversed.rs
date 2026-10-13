// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{
    AddDot, AddDotReversed, AddDotReversedAssign, DotReversed, SubDotReversed,
};
use malachite_base::vector::Vector;
use malachite_q::Rational;
use malachite_q::rational_vector::RationalVector;
use malachite_q::test_util::generators::rational_vector_rational_vector_rational_triple_gen_var_1;

fn reversed(w: &RationalVector) -> RationalVector {
    RationalVector::from_owned_elements(w.elements.iter().rev().cloned().collect())
}

#[test]
fn test_add_dot_reversed() {
    let test = |x, v, w, out| {
        let x = Rational::from_str(x).unwrap();
        let v = RationalVector::from_str(v).unwrap();
        let w = RationalVector::from_str(w).unwrap();
        let r = (&x).add_dot_reversed(&v, &w);
        assert!(r.is_valid());
        assert_eq!(r.to_string(), out);
        assert_eq!(x.clone().add_dot_reversed(v.clone(), w.clone()), r);
        assert_eq!(x.clone().add_dot_reversed(v.clone(), &w), r);
        assert_eq!(x.clone().add_dot_reversed(&v, w.clone()), r);
        assert_eq!(x.clone().add_dot_reversed(&v, &w), r);
        let mut y = x.clone();
        y.add_dot_reversed_assign(v.clone(), w.clone());
        assert_eq!(y, r);
        let mut y = x.clone();
        y.add_dot_reversed_assign(v.clone(), &w);
        assert_eq!(y, r);
        let mut y = x.clone();
        y.add_dot_reversed_assign(&v, w.clone());
        assert_eq!(y, r);
        let mut y = x.clone();
        y.add_dot_reversed_assign(&v, &w);
        assert_eq!(y, r);
    };
    test("0", "()", "()", "0");
    test("1/3", "(1/2, -2/3)", "(4/5, 3/2)", "11/20");
    test("-2", "(3/7)", "(-14/9)", "-8/3");
    test(
        "1/1000000000000",
        "(1000000000000/7, 1)",
        "(7/1000000000000, 1/3)",
        "125000000000000000000021/2625000000000",
    );
    test(
        "18446744073709551616/3",
        "(18446744073709551615, -1/2)",
        "(-18446744073709551615, 5/18446744073709551616)",
        "850705917302346158907467563574499475441/55340232221128654848",
    );
}

#[test]
#[should_panic]
fn add_dot_reversed_fail_1() {
    Rational::from_str("1").unwrap().add_dot_reversed(
        RationalVector::from_str("(1, 2)").unwrap(),
        RationalVector::from_str("(1)").unwrap(),
    );
}

#[test]
fn add_dot_reversed_properties() {
    rational_vector_rational_vector_rational_triple_gen_var_1().test_properties(|(v, w, x)| {
        let r = (&x).add_dot_reversed(&v, &w);
        assert!(r.is_valid());
        assert_eq!(x.clone().add_dot_reversed(v.clone(), w.clone()), r);
        assert_eq!(x.clone().add_dot_reversed(v.clone(), &w), r);
        assert_eq!(x.clone().add_dot_reversed(&v, w.clone()), r);
        assert_eq!(x.clone().add_dot_reversed(&v, &w), r);
        let mut y = x.clone();
        y.add_dot_reversed_assign(v.clone(), w.clone());
        assert_eq!(y, r);
        let mut y = x.clone();
        y.add_dot_reversed_assign(v.clone(), &w);
        assert_eq!(y, r);
        let mut y = x.clone();
        y.add_dot_reversed_assign(&v, w.clone());
        assert_eq!(y, r);
        let mut y = x.clone();
        y.add_dot_reversed_assign(&v, &w);
        assert_eq!(y, r);
        // It is the reversed dot product added to `x`.
        assert_eq!(r, &x + (&v).dot_reversed(&w));
        // Reversing the second vector gives the addition of the ordinary dot product.
        assert_eq!((&x).add_dot(&v, &reversed(&w)), r);
        // Subtracting it back recovers `x`.
        assert_eq!((&r).sub_dot_reversed(&v, &w), x);
    });
}
