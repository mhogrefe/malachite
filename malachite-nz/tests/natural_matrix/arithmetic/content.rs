// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::{
    Content, ContentAndPrimitivePart, PrimitivePart, PrimitivePartAssign,
};
use malachite_nz::natural::Natural;
use malachite_nz::natural_matrix::NaturalMatrix;
use malachite_nz::natural_vector::NaturalVector;
use malachite_nz::test_util::generators::natural_matrix_gen;

#[test]
fn test_content_and_primitive_part() {
    let test = |s, content: &str, primitive_part| {
        let a = NaturalMatrix::from_str(s).unwrap();
        let content = Natural::from_str(content).unwrap();
        assert_eq!(a.clone().content(), content);
        assert_eq!((&a).content(), content);
        let b = a.clone().primitive_part();
        assert!(b.is_valid());
        assert_eq!(b.to_string(), primitive_part);
        assert_eq!((&a).primitive_part(), b);
        let mut c = a.clone();
        c.primitive_part_assign();
        assert_eq!(c, b);
        assert_eq!(
            a.clone().content_and_primitive_part(),
            (content.clone(), b.clone())
        );
        assert_eq!((&a).content_and_primitive_part(), (content, b));
    };
    test("[]_{0,0}", "0", "[]_{0,0}");
    test("[]_{2,0}", "0", "[]_{2,0}");
    test("[0]", "0", "[0]");
    test("[0, 0; 0, 0]", "0", "[0, 0; 0, 0]");
    test("[1]", "1", "[1]");
    test("[7]", "7", "[1]");
    test("[6, 4; 10, 8]", "2", "[3, 2; 5, 4]");
    test("[0, 6; 0, 9]", "3", "[0, 2; 0, 3]");
    test("[12, 18, 30]", "6", "[2, 3, 5]");
    test("[4; 6; 10]", "2", "[2; 3; 5]");
    test(
        "[1000000000000000000000000, 3000000000000000000000000; 0, 5000000000000000000000000]",
        "1000000000000000000000000",
        "[1, 3; 0, 5]",
    );
}

#[test]
fn content_and_primitive_part_properties() {
    natural_matrix_gen().test_properties(|a| {
        let content = (&a).content();
        assert_eq!(a.clone().content(), content);
        let primitive_part = (&a).primitive_part();
        assert!(primitive_part.is_valid());
        assert_eq!(a.clone().primitive_part(), primitive_part);
        let mut b = a.clone();
        b.primitive_part_assign();
        assert_eq!(b, primitive_part);
        assert_eq!(
            (&a).content_and_primitive_part(),
            (content.clone(), primitive_part.clone())
        );
        assert_eq!(
            a.clone().content_and_primitive_part(),
            (content.clone(), primitive_part.clone())
        );

        // The content and primitive part are those of the elements as a vector, and the shape is
        // unchanged.
        let v = NaturalVector {
            elements: a.elements.clone(),
        };
        assert_eq!((&v).content(), content);
        assert_eq!(
            (primitive_part.height(), primitive_part.width()),
            (a.height(), a.width())
        );
        assert_eq!(primitive_part.elements, v.primitive_part().elements);
        // They do not depend on the order of the elements.
        assert_eq!((&a.transpose()).content(), content);
        assert_eq!(a.transpose().primitive_part(), primitive_part.transpose());
    });
}
