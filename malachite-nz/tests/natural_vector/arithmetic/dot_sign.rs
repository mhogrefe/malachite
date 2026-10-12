// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::cmp::Ordering::{self, *};
use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{Dot, DotSign, Sign};
use malachite_nz::natural_vector::NaturalVector;
use malachite_nz::test_util::generators::natural_vector_pair_gen_var_1;

#[test]
fn test_dot_sign() {
    let test = |a, b, out: Ordering| {
        let v = NaturalVector::from_str(a).unwrap();
        let w = NaturalVector::from_str(b).unwrap();
        assert_eq!((&v).dot_sign(&w), out);
        assert_eq!((&v).dot_sign(w.clone()), out);
        assert_eq!(v.clone().dot_sign(&w), out);
        assert_eq!(v.clone().dot_sign(w.clone()), out);
        assert_eq!(v.dot(w).sign(), out);
    };
    // - the 0-dimensional vectors
    test("()", "()", Equal);
    test("(0, 0, 0)", "(1, 2, 3)", Equal);
    // - supports that do not meet
    test("(1, 0, 3)", "(0, 5, 0)", Equal);
    // - supports that meet
    test("(1, 0, 3)", "(0, 5, 7)", Greater);
    test("(0, 0, 0, 2)", "(0, 0, 0, 2)", Greater);
    test(
        "(18446744073709551616, 0)",
        "(0, 18446744073709551616)",
        Equal,
    );
    test(
        "(18446744073709551616, 0)",
        "(1, 18446744073709551616)",
        Greater,
    );
}

#[test]
#[should_panic]
fn dot_sign_fail() {
    NaturalVector::from_str("(1, 2)")
        .unwrap()
        .dot_sign(NaturalVector::from_str("(1)").unwrap());
}

#[test]
fn dot_sign_properties() {
    natural_vector_pair_gen_var_1().test_properties(|(v, w)| {
        let sign = (&v).dot_sign(&w);
        assert_eq!((&v).dot_sign(w.clone()), sign);
        assert_eq!(v.clone().dot_sign(&w), sign);
        assert_eq!(v.clone().dot_sign(w.clone()), sign);
        assert_ne!(sign, Less);
        // It is the sign of the dot product, and it is symmetric.
        assert_eq!((&v).dot(&w).sign(), sign);
        assert_eq!((&w).dot_sign(&v), sign);
    });
}
