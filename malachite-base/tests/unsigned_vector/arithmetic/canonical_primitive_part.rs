// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{
    CanonicalPrimitivePart, CanonicalPrimitivePartAssign, ContentAndCanonicalPrimitivePart,
    ContentAndPrimitivePart, PrimitivePart,
};
use malachite_base::test_util::generators::unsigned_vector_gen;
use malachite_base::unsigned_vector::UnsignedVector;

#[test]
fn test_canonical_primitive_part() {
    let test = |s, content: u64, primitive_part| {
        let v = UnsignedVector::<u64>::from_str(s).unwrap();
        let w = v.clone().canonical_primitive_part();
        assert_eq!(w.to_string(), primitive_part);
        assert_eq!((&v).canonical_primitive_part(), w);
        let mut x = v.clone();
        x.canonical_primitive_part_assign();
        assert_eq!(x, w);
        assert_eq!(
            v.clone().content_and_canonical_primitive_part(),
            (content, w.clone())
        );
        assert_eq!((&v).content_and_canonical_primitive_part(), (content, w));
    };
    test("()", 0, "()");
    test("(0, 0)", 0, "(0, 0)");
    test("(7)", 7, "(1)");
    test("(6, 4, 10)", 2, "(3, 2, 5)");
    test("(0, 6, 0, 9)", 3, "(0, 2, 0, 3)");
}

#[test]
fn canonical_primitive_part_properties() {
    unsigned_vector_gen().test_properties(|v| {
        // With no negative elements there is only one associate, so this is the primitive part.
        let cpp = (&v).canonical_primitive_part();
        assert_eq!(cpp, (&v).primitive_part());
        assert_eq!(v.clone().canonical_primitive_part(), cpp);
        let mut w = v.clone();
        w.canonical_primitive_part_assign();
        assert_eq!(w, cpp);
        assert_eq!(
            (&v).content_and_canonical_primitive_part(),
            (&v).content_and_primitive_part()
        );
        assert_eq!(
            v.clone().content_and_canonical_primitive_part(),
            v.content_and_primitive_part()
        );
    });
}
