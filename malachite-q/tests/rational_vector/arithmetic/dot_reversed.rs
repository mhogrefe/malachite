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
use malachite_q::rational_vector::RationalVector;
use malachite_q::test_util::generators::rational_vector_rational_vector_rational_triple_gen_var_1;

fn reversed(w: &RationalVector) -> RationalVector {
    RationalVector::from_owned_elements(w.elements.iter().rev().cloned().collect())
}

#[test]
fn test_dot_reversed() {
    let test = |v, w, out| {
        let v = RationalVector::from_str(v).unwrap();
        let w = RationalVector::from_str(w).unwrap();
        let d = (&v).dot_reversed(&w);
        assert!(d.is_valid());
        assert_eq!(d.to_string(), out);
        assert_eq!((&v).dot_reversed(w.clone()), d);
        assert_eq!(v.clone().dot_reversed(&w), d);
        assert_eq!(v.clone().dot_reversed(w.clone()), d);
    };
    test("()", "()", "0");
    test("(1/2, -2/3)", "(4/5, 3/2)", "13/60");
    test("(3/7)", "(-14/9)", "-2/3");
    test(
        "(1000000000000/7, 1)",
        "(7/1000000000000, 1/3)",
        "1000000000000000000000147/21000000000000",
    );
    test(
        "(18446744073709551615, -1/2)",
        "(-18446744073709551615, 5/18446744073709551616)",
        "170141183460469231814697652047577087995/18446744073709551616",
    );
}

#[test]
#[should_panic]
fn dot_reversed_fail() {
    RationalVector::from_str("(1, 2)")
        .unwrap()
        .dot_reversed(RationalVector::from_str("(1)").unwrap());
}

#[test]
fn dot_reversed_properties() {
    rational_vector_rational_vector_rational_triple_gen_var_1().test_properties(|(v, w, _)| {
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
