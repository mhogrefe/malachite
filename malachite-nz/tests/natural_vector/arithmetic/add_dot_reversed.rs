// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{
    AddDot, AddDotReversed, AddDotReversedAssign, DotReversed,
};
use malachite_base::vector::Vector;
use malachite_nz::natural::Natural;
use malachite_nz::natural_vector::NaturalVector;
use malachite_nz::test_util::generators::natural_vector_natural_vector_natural_triple_gen_var_2;

fn reversed(w: &NaturalVector) -> NaturalVector {
    NaturalVector::from_owned_elements(w.elements.iter().rev().cloned().collect())
}

#[test]
fn test_add_dot_reversed() {
    let test = |x, v, w, out| {
        let x = Natural::from_str(x).unwrap();
        let v = NaturalVector::from_str(v).unwrap();
        let w = NaturalVector::from_str(w).unwrap();
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
    test("100", "(1, 2, 3)", "(4, 5, 6)", "128");
    test("5", "(3)", "(4)", "17");
    test(
        "1000000000000000000000",
        "(1000000000000000000000, 1)",
        "(1000000000000000000000, 1000000000000000000000)",
        "1000000000000000000002000000000000000000000",
    );
    test(
        "18446744073709551616",
        "(18446744073709551615, 1)",
        "(18446744073709551615, 18446744073709551615)",
        "340282366920938463463374607431768211456",
    );
}

#[test]
#[should_panic]
fn add_dot_reversed_fail_1() {
    Natural::from_str("1").unwrap().add_dot_reversed(
        NaturalVector::from_str("(1, 2)").unwrap(),
        NaturalVector::from_str("(1)").unwrap(),
    );
}

#[test]
fn add_dot_reversed_properties() {
    natural_vector_natural_vector_natural_triple_gen_var_2().test_properties(|(v, w, x)| {
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
    });
}
