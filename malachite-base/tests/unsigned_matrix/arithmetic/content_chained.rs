// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::{Content, ContentChained, Gcd};
use malachite_base::test_util::generators::unsigned_matrix_unsigned_pair_gen_var_6;
use malachite_base::unsigned_matrix::UnsignedMatrix;
use malachite_base::unsigned_vector::UnsignedVector;

#[test]
fn test_content_chained() {
    let test = |s, x: u64, out: u64| {
        let a = UnsignedMatrix::<u64>::from_str(s).unwrap();
        assert_eq!(a.clone().content_chained(x), out);
        assert_eq!((&a).content_chained(x), out);
    };
    test("[]_{0,0}", 0, 0);
    test("[]_{0,0}", 1, 1);
    test("[]_{0,0}", 4, 4);
    test("[]_{2,0}", 0, 0);
    test("[]_{2,0}", 1, 1);
    test("[]_{2,0}", 4, 4);
    test("[0]", 0, 0);
    test("[0]", 1, 1);
    test("[0]", 4, 4);
    test("[0, 0; 0, 0]", 0, 0);
    test("[0, 0; 0, 0]", 1, 1);
    test("[0, 0; 0, 0]", 4, 4);
    test("[1]", 0, 1);
    test("[1]", 1, 1);
    test("[1]", 4, 1);
    test("[7]", 0, 7);
    test("[7]", 1, 1);
    test("[7]", 4, 1);
    test("[6, 4; 10, 8]", 0, 2);
    test("[6, 4; 10, 8]", 1, 1);
    test("[6, 4; 10, 8]", 4, 2);
    test("[0, 6; 0, 9]", 0, 3);
    test("[0, 6; 0, 9]", 1, 1);
    test("[0, 6; 0, 9]", 4, 1);
}

#[test]
fn content_chained_properties() {
    unsigned_matrix_unsigned_pair_gen_var_6().test_properties(|(a, x)| {
        let out = (&a).content_chained(x);
        assert_eq!(a.clone().content_chained(x), out);
        // It is the GCD of `x` and the content, as for the elements as a vector.
        assert_eq!(out, (&a).content().gcd(x));
        let v = UnsignedVector::<u64> {
            elements: a.to_elements(),
        };
        assert_eq!(v.content_chained(x), out);
    });
}
