// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{AddDot, Dot, SubDot, SubDotAssign};
use malachite_nz::natural::Natural;
use malachite_nz::natural_vector::NaturalVector;
use malachite_nz::test_util::generators::natural_vector_natural_vector_natural_triple_gen_var_2;

#[test]
fn test_sub_dot() {
    let test = |x, v, w, out| {
        let x = Natural::from_str(x).unwrap();
        let v = NaturalVector::from_str(v).unwrap();
        let w = NaturalVector::from_str(w).unwrap();
        let r = (&x).sub_dot(&v, &w);
        assert!(r.is_valid());
        assert_eq!(r.to_string(), out);
        assert_eq!(x.clone().sub_dot(v.clone(), w.clone()), r);
        assert_eq!(x.clone().sub_dot(v.clone(), &w), r);
        assert_eq!(x.clone().sub_dot(&v, w.clone()), r);
        assert_eq!(x.clone().sub_dot(&v, &w), r);
        let mut y = x.clone();
        y.sub_dot_assign(v.clone(), w.clone());
        assert_eq!(y, r);
        let mut y = x.clone();
        y.sub_dot_assign(v.clone(), &w);
        assert_eq!(y, r);
        let mut y = x.clone();
        y.sub_dot_assign(&v, w.clone());
        assert_eq!(y, r);
        let mut y = x.clone();
        y.sub_dot_assign(&v, &w);
        assert_eq!(y, r);
    };
    test("0", "()", "()", "0");
    test("164", "(1, 2, 3)", "(4, 5, 6)", "132");
    test("29", "(3)", "(4)", "17");
    test(
        "2000000000000000000003000000000000000000000",
        "(1000000000000000000000, 1)",
        "(1000000000000000000000, 1000000000000000000000)",
        "1000000000000000000002000000000000000000000",
    );
    test(
        "680564733841876926908302470789826871296",
        "(18446744073709551615, 1)",
        "(18446744073709551615, 18446744073709551615)",
        "340282366920938463463374607431768211456",
    );
}

#[test]
#[should_panic]
fn sub_dot_fail_1() {
    Natural::from_str("1").unwrap().sub_dot(
        NaturalVector::from_str("(1, 2)").unwrap(),
        NaturalVector::from_str("(1)").unwrap(),
    );
}

#[test]
#[should_panic]
fn sub_dot_fail_2() {
    // The dot product, 5, is greater than 4.
    Natural::from(4u32).sub_dot(
        NaturalVector::from_str("(1, 2)").unwrap(),
        NaturalVector::from_str("(1, 2)").unwrap(),
    );
}

#[test]
fn sub_dot_properties() {
    natural_vector_natural_vector_natural_triple_gen_var_2().test_properties(|(v, w, x)| {
        // Start from a value at least the dot product, so that the result is a `Natural`.
        let d = (&v).dot(&w);
        let x = x + &d;
        let r = (&x).sub_dot(&v, &w);
        assert!(r.is_valid());
        assert_eq!(x.clone().sub_dot(v.clone(), w.clone()), r);
        assert_eq!(x.clone().sub_dot(v.clone(), &w), r);
        assert_eq!(x.clone().sub_dot(&v, w.clone()), r);
        assert_eq!(x.clone().sub_dot(&v, &w), r);
        let mut y = x.clone();
        y.sub_dot_assign(v.clone(), w.clone());
        assert_eq!(y, r);
        let mut y = x.clone();
        y.sub_dot_assign(v.clone(), &w);
        assert_eq!(y, r);
        let mut y = x.clone();
        y.sub_dot_assign(&v, w.clone());
        assert_eq!(y, r);
        let mut y = x.clone();
        y.sub_dot_assign(&v, &w);
        assert_eq!(y, r);
        // It is the dot product subtracted from `x`.
        assert_eq!(r, &x - (&v).dot(&w));
        // Adding it back recovers `x`.
        assert_eq!((&r).add_dot(&v, &w), x);
    });
}
