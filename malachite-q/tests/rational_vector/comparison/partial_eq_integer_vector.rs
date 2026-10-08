// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::vector::Vector;
use malachite_nz::integer_vector::IntegerVector;
use malachite_nz::test_util::generators::integer_vector_gen;
use malachite_q::rational_vector::RationalVector;
use malachite_q::test_util::generators::rational_vector_integer_vector_pair_gen;

#[test]
fn test_partial_eq_integer_vector() {
    let test = |s, t, out| {
        let v = RationalVector::from_str(s).unwrap();
        let w = IntegerVector::from_str(t).unwrap();
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
    // An element written as a fraction matches if it is an integer.
    test("(-4/2, 3)", "(-2, 3)", true);
    test("(-1, 2)", "(-1, 2)", true);
    // A non-integer element matches nothing, including its floor.
    test("(1/2)", "(0)", false);
    test("(-3/2)", "(-2)", false);
    // A trailing zero matters: the dimensions differ.
    test("(1, 0)", "(1)", false);
}

// Comparing with a converted vector is the reference the direct comparison is checked against.
#[allow(clippy::cmp_owned, clippy::op_ref)]
#[test]
fn partial_eq_integer_vector_properties() {
    rational_vector_integer_vector_pair_gen().test_properties(|(v, w)| {
        let eq = v == w;
        assert_eq!(w == v, eq);
        // Extra refs for type inference: with `RationalVector: PartialEq<IntegerVector>` in scope,
        // `v == w` would look for that impl.
        assert_eq!(&v == &RationalVector::from(w.clone()), eq);
        if eq {
            assert_eq!(v.dimension(), w.dimension());
        }
    });

    integer_vector_gen().test_properties(|w| {
        let w_q = RationalVector::from(w.clone());
        assert!(w_q == w);
        assert!(w == w_q);
    });
}
