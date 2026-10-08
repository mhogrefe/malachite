// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{
    Content, ContentAndPrimitivePart, DivisibleBy, Gcd, PrimitivePart, PrimitivePartAssign,
};
use malachite_base::test_util::generators::unsigned_vector_gen;
use malachite_base::unsigned_vector::UnsignedVector;
use malachite_base::vector::Vector;

#[test]
fn test_content_and_primitive_part() {
    let test = |s, content: u64, primitive_part| {
        let v = UnsignedVector::<u64>::from_str(s).unwrap();
        assert_eq!(v.clone().content(), content);
        assert_eq!((&v).content(), content);
        let w = v.clone().primitive_part();
        assert_eq!(w.to_string(), primitive_part);
        assert_eq!((&v).primitive_part(), w);
        let mut x = v.clone();
        x.primitive_part_assign();
        assert_eq!(x, w);
        assert_eq!(v.clone().content_and_primitive_part(), (content, w.clone()));
        assert_eq!((&v).content_and_primitive_part(), (content, w));
    };
    test("()", 0, "()");
    test("(0)", 0, "(0)");
    test("(0, 0)", 0, "(0, 0)");
    test("(1)", 1, "(1)");
    test("(7)", 7, "(1)");
    test("(6, 4, 10)", 2, "(3, 2, 5)");
    test("(0, 6, 0, 9)", 3, "(0, 2, 0, 3)");
    test("(12, 18, 30)", 6, "(2, 3, 5)");
    test("(5, 7)", 1, "(5, 7)");
    test(
        "(18446744073709551615, 18446744073709551615)",
        18446744073709551615,
        "(1, 1)",
    );
}

#[test]
fn content_and_primitive_part_properties() {
    unsigned_vector_gen().test_properties(|v| {
        let content = (&v).content();
        assert_eq!(v.clone().content(), content);
        let primitive_part = (&v).primitive_part();
        assert_eq!(v.clone().primitive_part(), primitive_part);
        let mut w = v.clone();
        w.primitive_part_assign();
        assert_eq!(w, primitive_part);
        assert_eq!(
            (&v).content_and_primitive_part(),
            (content, primitive_part.clone())
        );
        assert_eq!(
            v.clone().content_and_primitive_part(),
            (content, primitive_part.clone())
        );

        // The content is the GCD of the elements, and is zero only for a vector of zeros.
        assert_eq!(content, v.elements.iter().fold(0, |g, &x| g.gcd(x)));
        assert_eq!(content == 0, v.elements.iter().all(|&x| x == 0));
        assert!(v.elements.iter().all(|&x| x.divisible_by(content)));
        // v = cont(v) pp(v), element by element, and the dimension is unchanged.
        assert_eq!(primitive_part.dimension(), v.dimension());
        for (&x, &y) in v.elements.iter().zip(&primitive_part.elements) {
            assert_eq!(x, content * y);
        }
        if content != 0 {
            assert_eq!((&primitive_part).content(), 1);
        }
        assert_eq!((&primitive_part).primitive_part(), primitive_part);
    });
}
