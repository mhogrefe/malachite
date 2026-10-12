// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::Dot;
use malachite_nz::test_util::generators::integer_vector_pair_gen_var_1;
use malachite_q::Rational;
use malachite_q::rational_vector::RationalVector;
use malachite_q::test_util::generators::{
    rational_vector_pair_gen_var_1, rational_vector_triple_gen_var_1,
};
use malachite_q::test_util::rational_vector::arithmetic::dot::rational_vector_dot_naive;

#[test]
fn test_dot() {
    let test = |a, b, out| {
        let v = RationalVector::from_str(a).unwrap();
        let w = RationalVector::from_str(b).unwrap();
        let d = (&v).dot(&w);
        assert!(d.is_valid());
        assert_eq!(d.to_string(), out);
        assert_eq!((&v).dot(w.clone()), d);
        assert_eq!(v.clone().dot(&w), d);
        assert_eq!(v.clone().dot(w.clone()), d);
        assert_eq!(rational_vector_dot_naive(&v, &w), d);
    };
    test("()", "()", "0");
    test("(1/2)", "(2/3)", "1/3");
    test("(1/2, -2/3)", "(4/5, 3/2)", "-3/5");
    test("(1/3, 1/6)", "(3, -6)", "0");
    test("(1/2, 1/3, 1/5)", "(1/7, 1/11, 1/13)", "3517/30030");
    test(
        "(1000000000000000000000/3, 1)",
        "(3/1000000000000000000000, -1)",
        "0",
    );
}

#[test]
#[should_panic]
fn dot_fail() {
    RationalVector::from_str("(1/2, 2)")
        .unwrap()
        .dot(RationalVector::from_str("(1/2)").unwrap());
}

#[test]
fn dot_properties() {
    rational_vector_pair_gen_var_1().test_properties(|(v, w)| {
        let d = (&v).dot(&w);
        assert!(d.is_valid());
        assert_eq!((&v).dot(w.clone()), d);
        assert_eq!(v.clone().dot(&w), d);
        assert_eq!(v.clone().dot(w.clone()), d);
        assert_eq!(rational_vector_dot_naive(&v, &w), d);
        // The dot product is symmetric.
        assert_eq!((&w).dot(&v), d);
        // Negating either vector negates the dot product.
        assert_eq!((-&v).dot(&w), -&d);
        assert_eq!((&v).dot(-&w), -d);
    });

    rational_vector_triple_gen_var_1().test_properties(|(u, v, w)| {
        // The dot product is additive in each argument.
        assert_eq!((&u + &v).dot(&w), (&u).dot(&w) + (&v).dot(&w));
    });

    integer_vector_pair_gen_var_1().test_properties(|(v, w)| {
        // It agrees with the IntegerVector implementation.
        assert_eq!(
            RationalVector::from(v.clone()).dot(RationalVector::from(w.clone())),
            Rational::from(v.dot(w))
        );
    });
}
