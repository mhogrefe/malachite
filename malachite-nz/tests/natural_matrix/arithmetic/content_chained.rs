// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{Content, ContentChained, Gcd};
use malachite_nz::natural::Natural;
use malachite_nz::natural_matrix::NaturalMatrix;
use malachite_nz::natural_vector::NaturalVector;
use malachite_nz::test_util::generators::natural_matrix_natural_pair_gen;

#[test]
fn test_content_chained() {
    let test = |s, x, out| {
        let a = NaturalMatrix::from_str(s).unwrap();
        let x = Natural::from_str(x).unwrap();
        let out = Natural::from_str(out).unwrap();
        assert_eq!(a.clone().content_chained(x.clone()), out);
        assert_eq!(a.clone().content_chained(&x), out);
        assert_eq!((&a).content_chained(x.clone()), out);
        assert_eq!((&a).content_chained(&x), out);
    };
    test("[]_{0,0}", "0", "0");
    test("[]_{0,0}", "1", "1");
    test("[]_{0,0}", "4", "4");
    test("[]_{2,0}", "0", "0");
    test("[]_{2,0}", "1", "1");
    test("[]_{2,0}", "4", "4");
    test("[0]", "0", "0");
    test("[0]", "1", "1");
    test("[0]", "4", "4");
    test("[0, 0; 0, 0]", "0", "0");
    test("[0, 0; 0, 0]", "1", "1");
    test("[0, 0; 0, 0]", "4", "4");
    test("[1]", "0", "1");
    test("[1]", "1", "1");
    test("[1]", "4", "1");
    test("[7]", "0", "7");
    test("[7]", "1", "1");
    test("[7]", "4", "1");
    test("[6, 4; 10, 8]", "0", "2");
    test("[6, 4; 10, 8]", "1", "1");
    test("[6, 4; 10, 8]", "4", "2");
    test("[0, 6; 0, 9]", "0", "3");
    test("[0, 6; 0, 9]", "1", "1");
    test("[0, 6; 0, 9]", "4", "1");
}

#[test]
fn content_chained_properties() {
    natural_matrix_natural_pair_gen().test_properties(|(a, x)| {
        let out = (&a).content_chained(&x);
        assert!(out.is_valid());
        assert_eq!(a.clone().content_chained(x.clone()), out);
        assert_eq!(a.clone().content_chained(&x), out);
        assert_eq!((&a).content_chained(x.clone()), out);
        // It is the GCD of `x` and the content, as for the elements as a vector.
        assert_eq!(out, (&a).content().gcd(&x));
        let v = NaturalVector {
            elements: a.elements.clone(),
        };
        assert_eq!(v.content_chained(x), out);
    });
}
