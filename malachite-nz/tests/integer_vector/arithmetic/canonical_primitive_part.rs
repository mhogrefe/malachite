// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{
    CanonicalPrimitivePart, CanonicalPrimitivePartAssign, Content,
    ContentAndCanonicalPrimitivePart, PrimitivePart,
};
use malachite_nz::integer_vector::IntegerVector;
use malachite_nz::natural::Natural;
use malachite_nz::test_util::generators::integer_vector_gen;

#[test]
fn test_canonical_primitive_part() {
    let test = |s, content, primitive_part| {
        let v = IntegerVector::from_str(s).unwrap();
        let content = Natural::from_str(content).unwrap();
        let w = v.clone().canonical_primitive_part();
        assert_eq!(w.to_string(), primitive_part);
        assert_eq!((&v).canonical_primitive_part(), w);
        let mut x = v.clone();
        x.canonical_primitive_part_assign();
        assert_eq!(x, w);
        assert_eq!(
            v.clone().content_and_canonical_primitive_part(),
            (content.clone(), w.clone())
        );
        assert_eq!((&v).content_and_canonical_primitive_part(), (content, w));
    };
    test("()", "0", "()");
    test("(0, 0)", "0", "(0, 0)");
    test("(1)", "1", "(1)");
    test("(-1)", "1", "(1)");
    test("(-7)", "7", "(1)");
    test("(6, 4, 10)", "2", "(3, 2, 5)");
    // A negative first element negates every element.
    test("(-6, 4, -10)", "2", "(3, -2, 5)");
    test("(6, -4, 10)", "2", "(3, -2, 5)");
    // Leading zeros are skipped: it is the first nonzero element that is made positive.
    test("(0, -6, 0, 9)", "3", "(0, 2, 0, -3)");
    test("(0, 0, -5)", "5", "(0, 0, 1)");
    test(
        "(-1000000000000000000000000, 3000000000000000000000000)",
        "1000000000000000000000000",
        "(1, -3)",
    );
}

#[test]
fn canonical_primitive_part_properties() {
    integer_vector_gen().test_properties(|v| {
        let cpp = (&v).canonical_primitive_part();
        assert_eq!(v.clone().canonical_primitive_part(), cpp);
        let mut w = v.clone();
        w.canonical_primitive_part_assign();
        assert_eq!(w, cpp);
        let content = (&v).content();
        assert_eq!(
            (&v).content_and_canonical_primitive_part(),
            (content.clone(), cpp.clone())
        );
        assert_eq!(
            v.clone().content_and_canonical_primitive_part(),
            (content.clone(), cpp.clone())
        );

        // It is the primitive part, or its negation, chosen so that the first nonzero element is
        // positive.
        let primitive_part = (&v).primitive_part();
        assert!(cpp == primitive_part || cpp == -&primitive_part);
        if let Some(first) = cpp.elements.iter().find(|x| **x != 0u32) {
            assert!(*first > 0u32);
        }
        if content != 0u32 {
            assert_eq!((&cpp).content(), 1u32);
        }
        assert_eq!((&cpp).canonical_primitive_part(), cpp);
        assert_eq!(cpp.dimension(), v.dimension());
        // A vector and its negation have the same canonical primitive part.
        let negated = -&v;
        assert_eq!((&negated).canonical_primitive_part(), cpp);
    });
}
