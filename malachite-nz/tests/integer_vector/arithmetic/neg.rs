// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{
    CanonicalPrimitivePart, Content, NegAssign, PrimitivePart,
};
use malachite_nz::integer_vector::IntegerVector;
use malachite_nz::test_util::generators::{integer_vector_gen, natural_vector_gen};

#[test]
fn test_neg() {
    let test = |s, out| {
        let v = IntegerVector::from_str(s).unwrap();
        assert_eq!((-v.clone()).to_string(), out);
        assert_eq!((-&v).to_string(), out);
        let mut w = v;
        w.neg_assign();
        assert_eq!(w.to_string(), out);
    };
    test("()", "()");
    test("(0)", "(0)");
    test("(0, 0)", "(0, 0)");
    test("(1)", "(-1)");
    test("(-1)", "(1)");
    test("(1, -3, 2)", "(-1, 3, -2)");
    test(
        "(-5, 0, 1000000000000000000000)",
        "(5, 0, -1000000000000000000000)",
    );
}

#[test]
fn neg_properties() {
    integer_vector_gen().test_properties(|v| {
        let w = -&v;
        assert_eq!(-v.clone(), w);
        let mut x = v.clone();
        x.neg_assign();
        assert_eq!(x, w);

        // Negating twice gives the vector back, and only a vector of zeros is its own negation.
        assert_eq!(-&w, v);
        assert_eq!(w == v, v.elements.iter().all(|x| *x == 0u32));
        assert_eq!(w.dimension(), v.dimension());
        for (x, y) in v.elements.iter().zip(&w.elements) {
            assert_eq!(*y, -x);
        }
        // The content and the canonical primitive part are unchanged, and the primitive part is
        // negated.
        assert_eq!((&w).content(), (&v).content());
        assert_eq!((&w).primitive_part(), -(&v).primitive_part());
        assert_eq!(
            (&w).canonical_primitive_part(),
            (&v).canonical_primitive_part()
        );
    });

    natural_vector_gen().test_properties(|v| {
        // Negating a `NaturalVector`'s conversion negates every element.
        let w = -IntegerVector::from(v.clone());
        assert!(w.elements.iter().zip(&v.elements).all(|(x, y)| -x == *y));
    });
}
