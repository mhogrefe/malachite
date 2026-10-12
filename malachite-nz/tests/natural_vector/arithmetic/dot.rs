// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::Dot;
use malachite_nz::integer_vector::IntegerVector;
use malachite_nz::natural_vector::NaturalVector;
use malachite_nz::test_util::generators::{
    natural_vector_pair_gen_var_1, natural_vector_triple_gen_var_1,
};
use malachite_nz::test_util::natural_vector::arithmetic::dot::natural_vector_dot_naive;

#[test]
fn test_dot() {
    let test = |a, b, out| {
        let v = NaturalVector::from_str(a).unwrap();
        let w = NaturalVector::from_str(b).unwrap();
        let d = (&v).dot(&w);
        assert!(d.is_valid());
        assert_eq!(d.to_string(), out);
        assert_eq!((&v).dot(w.clone()), d);
        assert_eq!(v.clone().dot(&w), d);
        assert_eq!(v.clone().dot(w.clone()), d);
        assert_eq!(natural_vector_dot_naive(&v, &w), d);
    };
    test("()", "()", "0");
    test("(5)", "(6)", "30");
    test("(1, 2, 3)", "(4, 5, 6)", "32");
    test("(0, 0)", "(7, 8)", "0");
    test(
        "(1000000000000000000000, 1)",
        "(1000000000000000000000, 1000000000000000000000)",
        "1000000000000000000001000000000000000000000",
    );
    test(
        "(18446744073709551615, 18446744073709551615)",
        "(18446744073709551615, 18446744073709551615)",
        "680564733841876926852962238568698216450",
    );
}

#[test]
#[should_panic]
fn dot_fail() {
    NaturalVector::from_str("(1, 2)")
        .unwrap()
        .dot(NaturalVector::from_str("(1, 2, 3)").unwrap());
}

#[test]
fn dot_properties() {
    natural_vector_pair_gen_var_1().test_properties(|(v, w)| {
        let d = (&v).dot(&w);
        assert!(d.is_valid());
        assert_eq!((&v).dot(w.clone()), d);
        assert_eq!(v.clone().dot(&w), d);
        assert_eq!(v.clone().dot(w.clone()), d);
        assert_eq!(natural_vector_dot_naive(&v, &w), d);
        // The dot product is symmetric.
        assert_eq!((&w).dot(&v), d);
        // It agrees with the IntegerVector implementation.
        assert_eq!(IntegerVector::from(v).dot(IntegerVector::from(w)), d);
    });

    natural_vector_triple_gen_var_1().test_properties(|(u, v, w)| {
        // The dot product is additive in each argument.
        assert_eq!((&u + &v).dot(&w), (&u).dot(&w) + (&v).dot(&w));
    });
}
