// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{Content, ContentAndPrimitivePart, PrimitivePart};
use malachite_nz::integer_vector::IntegerVector;
use malachite_nz::test_util::generators::integer_vector_gen;
use malachite_q::Rational;
use malachite_q::rational_vector::RationalVector;
use malachite_q::test_util::generators::rational_vector_gen;

#[test]
fn test_content_and_primitive_part() {
    let test = |s, content, primitive_part| {
        let v = RationalVector::from_str(s).unwrap();
        let c = Rational::from_str(content).unwrap();
        assert_eq!(v.clone().content(), c);
        assert_eq!((&v).content(), c);
        let w = v.clone().primitive_part();
        assert_eq!(w.to_string(), primitive_part);
        assert_eq!((&v).primitive_part(), w);
        assert_eq!(
            v.clone().content_and_primitive_part(),
            (c.clone(), w.clone())
        );
        assert_eq!((&v).content_and_primitive_part(), (c, w));
    };
    test("()", "0", "()");
    test("(0)", "0", "(0)");
    test("(0, 0)", "0", "(0, 0)");
    test("(1)", "1", "(1)");
    test("(-3/7)", "3/7", "(-1)");
    test("(6, 4, 10)", "2", "(3, 2, 5)");
    // The content is the GCD of the numerators over the common denominator.
    test("(2/3, 4/3)", "2/3", "(1, 2)");
    test("(1/2, 1/3)", "1/6", "(3, 2)");
    // Every element keeps its sign.
    test("(-1/2, 2/3, -5)", "1/6", "(-3, 4, -30)");
    test("(0, -4/9, 2/15)", "2/45", "(0, -10, 3)");
}

#[test]
fn content_and_primitive_part_properties() {
    rational_vector_gen().test_properties(|v| {
        let content = (&v).content();
        assert!(content.is_valid());
        assert!(content >= 0u32);
        assert_eq!(v.clone().content(), content);
        let primitive_part = (&v).primitive_part();
        assert_eq!(v.clone().primitive_part(), primitive_part);
        assert_eq!(
            (&v).content_and_primitive_part(),
            (content.clone(), primitive_part.clone())
        );
        assert_eq!(
            v.clone().content_and_primitive_part(),
            (content.clone(), primitive_part.clone())
        );

        // The content is zero only for a vector of zeros.
        assert_eq!(content == 0u32, v.elements.iter().all(|x| *x == 0u32));
        // v = cont(v) pp(v), element by element, and the dimension is unchanged.
        assert_eq!(primitive_part.dimension(), v.dimension());
        for (x, y) in v.elements.iter().zip(&primitive_part.elements) {
            assert_eq!(*x, &content * Rational::from(y));
        }
        // The primitive part has coprime integer elements.
        if content != 0u32 {
            assert_eq!((&primitive_part).content(), 1u32);
        }
        // It is the primitive part of the cleared numerators.
        let numerators = v.to_numerators_and_denominator().0;
        assert_eq!((&numerators).primitive_part(), primitive_part);
        // Negating the vector negates its primitive part.
        let negated = RationalVector {
            elements: v.elements.iter().map(|x| -x).collect(),
        };
        assert_eq!(
            (&negated).primitive_part().elements,
            primitive_part
                .elements
                .iter()
                .map(|x| -x)
                .collect::<Vec<_>>()
        );
    });

    integer_vector_gen().test_properties(|v| {
        // An integer vector has the same content and primitive part either way.
        let w = RationalVector::from(v.clone());
        assert_eq!((&w).content(), Rational::from((&v).content()));
        assert_eq!((&w).primitive_part(), (&v).primitive_part());
    });

    // The primitive part is an `IntegerVector`.
    let _: IntegerVector = RationalVector::from_str("()").unwrap().primitive_part();
}
