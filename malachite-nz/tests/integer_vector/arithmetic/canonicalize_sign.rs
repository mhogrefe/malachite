// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{
    CanonicalPrimitivePart, CanonicalizeSign, CanonicalizeSignAssign, Content,
};
use malachite_base::vector::Vector;
use malachite_nz::integer_vector::IntegerVector;
use malachite_nz::test_util::generators::integer_vector_gen;

#[test]
fn test_canonicalize_sign() {
    let test = |s, out| {
        let v = IntegerVector::from_str(s).unwrap();
        assert_eq!(v.clone().canonicalize_sign().to_string(), out);
        assert_eq!((&v).canonicalize_sign().to_string(), out);
        let mut w = v;
        w.canonicalize_sign_assign();
        assert_eq!(w.to_string(), out);
    };
    test("()", "()");
    test("(0)", "(0)");
    test("(0, 0)", "(0, 0)");
    test("(5)", "(5)");
    test("(-5)", "(5)");
    test("(2, -3)", "(2, -3)");
    test("(-2, 3)", "(2, -3)");
    // Leading zeros are skipped: it is the first nonzero element that is made positive.
    test("(0, -2, 3)", "(0, 2, -3)");
    test(
        "(0, 0, -1000000000000000000000)",
        "(0, 0, 1000000000000000000000)",
    );
}

#[test]
fn canonicalize_sign_properties() {
    integer_vector_gen().test_properties(|v| {
        let w = (&v).canonicalize_sign();
        assert_eq!(v.clone().canonicalize_sign(), w);
        let mut x = v.clone();
        x.canonicalize_sign_assign();
        assert_eq!(x, w);

        // The result is the vector or its negation, whichever has a non-negative pivot, and the
        // dimension is unchanged.
        assert!(w == v || w == -&v);
        assert!(w.pivot().is_none_or(|x| *x > 0u32));
        assert_eq!(w.dimension(), v.dimension());
        // It is idempotent, and a vector and its negation share it.
        assert_eq!((&w).canonicalize_sign(), w);
        assert_eq!((-&v).canonicalize_sign(), w);
        // The content is unchanged, and the canonical primitive part is the primitive part of the
        // result.
        assert_eq!((&w).content(), (&v).content());
        assert_eq!(
            (&v).canonical_primitive_part(),
            (&w).canonical_primitive_part()
        );
    });
}
