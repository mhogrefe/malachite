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
    CanonicalPrimitivePart, CanonicalPrimitivePartAssign, Content,
    ContentAndCanonicalPrimitivePart, PrimitivePart,
};
use malachite_nz::integer_matrix::IntegerMatrix;
use malachite_nz::integer_vector::IntegerVector;
use malachite_nz::natural::Natural;
use malachite_nz::test_util::generators::integer_matrix_gen;

#[test]
fn test_canonical_primitive_part() {
    let test = |s, content: &str, canonical_primitive_part| {
        let a = IntegerMatrix::from_str(s).unwrap();
        let content = Natural::from_str(content).unwrap();
        let b = a.clone().canonical_primitive_part();
        assert!(b.is_valid());
        assert_eq!(b.to_string(), canonical_primitive_part);
        assert_eq!((&a).canonical_primitive_part(), b);
        let mut c = a.clone();
        c.canonical_primitive_part_assign();
        assert_eq!(c, b);
        assert_eq!(
            a.clone().content_and_canonical_primitive_part(),
            (content.clone(), b.clone())
        );
        assert_eq!((&a).content_and_canonical_primitive_part(), (content, b));
    };
    test("[]_{0,0}", "0", "[]_{0,0}");
    test("[]_{2,0}", "0", "[]_{2,0}");
    test("[0]", "0", "[0]");
    test("[0, 0; 0, 0]", "0", "[0, 0; 0, 0]");
    test("[1]", "1", "[1]");
    test("[-1]", "1", "[1]");
    test("[-7]", "7", "[1]");
    test("[6, 4; 10, 8]", "2", "[3, 2; 5, 4]");
    test("[-6, 4; -10, 8]", "2", "[3, -2; 5, -4]");
    test("[0, -6; 0, 9]", "3", "[0, 2; 0, -3]");
    test("[0, 0; -2, 4]", "2", "[0, 0; 1, -2]");
    test("[12, -18, 30]", "6", "[2, -3, 5]");
    test("[-4; 6; -10]", "2", "[2; -3; 5]");
    test(
        "[-1000000000000000000000000, 3000000000000000000000000; 0, 5000000000000000000000000]",
        "1000000000000000000000000",
        "[1, -3; 0, -5]",
    );
}

#[test]
fn canonical_primitive_part_properties() {
    integer_matrix_gen().test_properties(|a| {
        let canonical = (&a).canonical_primitive_part();
        assert!(canonical.is_valid());
        assert_eq!(a.clone().canonical_primitive_part(), canonical);
        let mut b = a.clone();
        b.canonical_primitive_part_assign();
        assert_eq!(b, canonical);
        let content = (&a).content();
        assert_eq!(
            (&a).content_and_canonical_primitive_part(),
            (content.clone(), canonical.clone())
        );
        assert_eq!(
            a.clone().content_and_canonical_primitive_part(),
            (content, canonical.clone())
        );

        // It is the canonical primitive part of the elements as a vector, since a vector's first
        // nonzero element is the matrix's first in row-major order.
        let v = IntegerVector {
            elements: a.to_elements(),
        };
        assert_eq!(
            canonical.elements_ref(),
            v.canonical_primitive_part().elements
        );
        assert_eq!(
            (canonical.height(), canonical.width()),
            (a.height(), a.width())
        );
        // It is the primitive part with its first nonzero element, in row-major order, made
        // positive, so a matrix and its negation share it.
        let primitive_part = (&a).primitive_part();
        let first = primitive_part.elements_ref().iter().find(|&x| *x != 0u32);
        assert_eq!(
            canonical,
            if first.is_some_and(|x| *x < 0u32) {
                -primitive_part
            } else {
                primitive_part
            }
        );
        assert!(
            canonical
                .elements_ref()
                .iter()
                .find(|&x| *x != 0u32)
                .is_none_or(|x| *x > 0u32)
        );
        assert_eq!((-&a).canonical_primitive_part(), canonical);
    });
}
