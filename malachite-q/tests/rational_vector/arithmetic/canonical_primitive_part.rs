// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{
    CanonicalPrimitivePart, Content, ContentAndCanonicalPrimitivePart, PrimitivePart,
};
use malachite_nz::test_util::generators::integer_vector_gen;
use malachite_q::Rational;
use malachite_q::rational_vector::RationalVector;
use malachite_q::test_util::generators::rational_vector_gen;

#[test]
fn test_canonical_primitive_part() {
    let test = |s, content, primitive_part| {
        let v = RationalVector::from_str(s).unwrap();
        let c = Rational::from_str(content).unwrap();
        let w = v.clone().canonical_primitive_part();
        assert_eq!(w.to_string(), primitive_part);
        assert_eq!((&v).canonical_primitive_part(), w);
        assert_eq!(
            v.clone().content_and_canonical_primitive_part(),
            (c.clone(), w.clone())
        );
        assert_eq!((&v).content_and_canonical_primitive_part(), (c, w));
    };
    test("()", "0", "()");
    test("(0, 0)", "0", "(0, 0)");
    test("(-3/7)", "3/7", "(1)");
    test("(2/3, 4/3)", "2/3", "(1, 2)");
    // A negative first nonzero element negates every element.
    test("(-1/2, 2/3, -5)", "1/6", "(3, -4, 30)");
    test("(0, -4/9, 2/15)", "2/45", "(0, 10, -3)");
}

#[test]
fn canonical_primitive_part_properties() {
    rational_vector_gen().test_properties(|v| {
        let cpp = (&v).canonical_primitive_part();
        assert_eq!(v.clone().canonical_primitive_part(), cpp);
        let content = (&v).content();
        assert_eq!(
            (&v).content_and_canonical_primitive_part(),
            (content.clone(), cpp.clone())
        );
        assert_eq!(
            v.clone().content_and_canonical_primitive_part(),
            (content, cpp.clone())
        );

        // It is the primitive part, or its negation, chosen so that the first nonzero element is
        // positive: the canonical primitive part of the cleared numerators.
        let primitive_part = (&v).primitive_part();
        assert!(cpp == primitive_part || cpp == -&primitive_part);
        if let Some(first) = cpp.elements.iter().find(|x| **x != 0u32) {
            assert!(*first > 0u32);
        }
        let numerators = v.to_numerators_and_denominator().0;
        assert_eq!(numerators.canonical_primitive_part(), cpp);
        // A vector and its negation have the same canonical primitive part.
        let negated = -&v;
        assert_eq!((&negated).canonical_primitive_part(), cpp);
    });

    integer_vector_gen().test_properties(|v| {
        // An integer vector has the same canonical primitive part either way.
        assert_eq!(
            RationalVector::from(v.clone()).canonical_primitive_part(),
            v.canonical_primitive_part()
        );
    });
}
