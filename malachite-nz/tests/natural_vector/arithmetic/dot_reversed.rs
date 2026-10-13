// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{Dot, DotReversed};
use malachite_base::vector::Vector;
use malachite_nz::natural_vector::NaturalVector;
use malachite_nz::test_util::generators::natural_vector_natural_vector_natural_triple_gen_var_2;

fn reversed(w: &NaturalVector) -> NaturalVector {
    NaturalVector::from_owned_elements(w.elements.iter().rev().cloned().collect())
}

#[test]
fn test_dot_reversed() {
    let test = |v, w, out| {
        let v = NaturalVector::from_str(v).unwrap();
        let w = NaturalVector::from_str(w).unwrap();
        let d = (&v).dot_reversed(&w);
        assert!(d.is_valid());
        assert_eq!(d.to_string(), out);
        assert_eq!((&v).dot_reversed(w.clone()), d);
        assert_eq!(v.clone().dot_reversed(&w), d);
        assert_eq!(v.clone().dot_reversed(w.clone()), d);
    };
    test("()", "()", "0");
    test("(1, 2, 3)", "(4, 5, 6)", "28");
    test("(3)", "(4)", "12");
    test(
        "(1000000000000000000000, 1)",
        "(1000000000000000000000, 1000000000000000000000)",
        "1000000000000000000001000000000000000000000",
    );
    test(
        "(18446744073709551615, 1)",
        "(18446744073709551615, 18446744073709551615)",
        "340282366920938463444927863358058659840",
    );
}

#[test]
#[should_panic]
fn dot_reversed_fail() {
    NaturalVector::from_str("(1, 2)")
        .unwrap()
        .dot_reversed(NaturalVector::from_str("(1)").unwrap());
}

#[test]
fn dot_reversed_properties() {
    natural_vector_natural_vector_natural_triple_gen_var_2().test_properties(|(v, w, _)| {
        let d = (&v).dot_reversed(&w);
        assert!(d.is_valid());
        assert_eq!((&v).dot_reversed(w.clone()), d);
        assert_eq!(v.clone().dot_reversed(&w), d);
        assert_eq!(v.clone().dot_reversed(w.clone()), d);
        // It is the dot product with the second vector reversed, and it is symmetric.
        assert_eq!((&v).dot(&reversed(&w)), d);
        assert_eq!((&w).dot_reversed(&v), d);
        // Reversing both vectors leaves it unchanged.
        assert_eq!(reversed(&v).dot_reversed(&reversed(&w)), d);
    });
}
