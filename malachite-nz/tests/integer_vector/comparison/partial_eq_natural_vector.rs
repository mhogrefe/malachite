// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_nz::integer_vector::IntegerVector;
use malachite_nz::natural_vector::NaturalVector;
use malachite_nz::test_util::generators::{
    integer_vector_natural_vector_pair_gen, natural_vector_gen,
};

#[test]
fn test_partial_eq_natural_vector() {
    let test = |s, t, out| {
        let v = IntegerVector::from_str(s).unwrap();
        let w = NaturalVector::from_str(t).unwrap();
        assert_eq!(v == w, out);
        assert_eq!(w == v, out);
    };
    test("()", "()", true);
    test("()", "(0)", false);
    test("(0)", "()", false);
    test("(1)", "(1)", true);
    test("(1, 2)", "(1, 2)", true);
    test("(1, 2)", "(2, 1)", false);
    test("(1, 2, 3)", "(1, 2)", false);
    test(
        "(1000000000000000000000000, 3)",
        "(1000000000000000000000000, 3)",
        true,
    );
    // A negative element matches nothing, including its absolute value.
    test("(-1)", "(1)", false);
    test(
        "(3, -1000000000000000000000000)",
        "(3, 1000000000000000000000000)",
        false,
    );
    // A trailing zero matters: the dimensions differ.
    test("(1, 0)", "(1)", false);
}

// Comparing with a converted vector is the reference the direct comparison is checked against.
#[allow(clippy::cmp_owned, clippy::op_ref)]
#[test]
fn partial_eq_natural_vector_properties() {
    integer_vector_natural_vector_pair_gen().test_properties(|(v, w)| {
        let eq = v == w;
        assert_eq!(w == v, eq);
        // Extra refs for type inference: with `IntegerVector: PartialEq<NaturalVector>` in scope,
        // `v == w` would look for that impl.
        assert_eq!(&v == &IntegerVector::from(w.clone()), eq);
        if eq {
            assert_eq!(v.dimension(), w.dimension());
        }
    });

    natural_vector_gen().test_properties(|w| {
        let w_i = IntegerVector::from(w.clone());
        assert!(w_i == w);
        assert!(w == w_i);
    });
}
