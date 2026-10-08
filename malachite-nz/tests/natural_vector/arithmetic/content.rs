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
use malachite_base::num::basic::traits::Zero;
use malachite_base::test_util::generators::unsigned_vector_gen;
use malachite_base::vector::Vector;
use malachite_nz::natural::Natural;
use malachite_nz::natural_vector::NaturalVector;
use malachite_nz::test_util::generators::natural_vector_gen;

#[test]
fn test_content_and_primitive_part() {
    let test = |s, content, primitive_part| {
        let v = NaturalVector::from_str(s).unwrap();
        let content = Natural::from_str(content).unwrap();
        assert_eq!(v.clone().content(), content);
        assert_eq!((&v).content(), content);
        let w = v.clone().primitive_part();
        assert_eq!(w.to_string(), primitive_part);
        assert_eq!((&v).primitive_part(), w);
        let mut x = v.clone();
        x.primitive_part_assign();
        assert_eq!(x, w);
        assert_eq!(
            v.clone().content_and_primitive_part(),
            (content.clone(), w.clone())
        );
        assert_eq!((&v).content_and_primitive_part(), (content, w));
    };
    test("()", "0", "()");
    test("(0)", "0", "(0)");
    test("(0, 0)", "0", "(0, 0)");
    test("(1)", "1", "(1)");
    test("(7)", "7", "(1)");
    test("(6, 4, 10)", "2", "(3, 2, 5)");
    test("(0, 6, 0, 9)", "3", "(0, 2, 0, 3)");
    test(
        "(1000000000000000000000000, 3000000000000000000000000)",
        "1000000000000000000000000",
        "(1, 3)",
    );
}

#[test]
fn content_and_primitive_part_properties() {
    natural_vector_gen().test_properties(|v| {
        let content = (&v).content();
        assert!(content.is_valid());
        assert_eq!(v.clone().content(), content);
        let primitive_part = (&v).primitive_part();
        assert_eq!(v.clone().primitive_part(), primitive_part);
        let mut w = v.clone();
        w.primitive_part_assign();
        assert_eq!(w, primitive_part);
        assert_eq!(
            (&v).content_and_primitive_part(),
            (content.clone(), primitive_part.clone())
        );
        assert_eq!(
            v.clone().content_and_primitive_part(),
            (content.clone(), primitive_part.clone())
        );

        // The content is the GCD of the elements, and is zero only for a vector of zeros.
        assert_eq!(content, v.elements.iter().fold(Natural::ZERO, Gcd::gcd));
        assert_eq!(content == 0u32, v.elements.iter().all(|x| *x == 0u32));
        assert!(v.elements.iter().all(|x| x.divisible_by(&content)));
        // v = cont(v) pp(v), element by element, and the dimension is unchanged.
        assert_eq!(primitive_part.dimension(), v.dimension());
        for (x, y) in v.elements.iter().zip(&primitive_part.elements) {
            assert_eq!(*x, &content * y);
        }
        if content != 0u32 {
            assert_eq!((&primitive_part).content(), 1u32);
        }
        assert_eq!((&primitive_part).primitive_part(), primitive_part);
    });

    unsigned_vector_gen().test_properties(|v| {
        // The `u64` and `Natural` vectors agree.
        let w = NaturalVector::from(v.clone());
        assert_eq!((&w).content(), (&v).content());
        assert_eq!((&w).primitive_part(), (&v).primitive_part());
    });
}
