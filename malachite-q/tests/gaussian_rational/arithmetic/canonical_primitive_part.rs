// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{
    CanonicalPrimitivePart, CanonicalizeUnit, Conjugate, Content, ContentAndCanonicalPrimitivePart,
    MulI, PrimitivePart,
};
use malachite_base::num::basic::traits::Zero;
use malachite_nz::gaussian_integer::GaussianInteger;
use malachite_nz::test_util::generators::gaussian_integer_gen;
use malachite_q::gaussian_rational::GaussianRational;
use malachite_q::test_util::generators::gaussian_rational_gen;
use std::str::FromStr;

#[test]
fn test_canonical_primitive_part() {
    let test = |s, content_out, primitive_out| {
        let x = GaussianRational::from_str(s).unwrap();

        let (content, primitive) = x.clone().content_and_canonical_primitive_part();
        assert!(primitive.real.is_valid());
        assert!(primitive.imaginary.is_valid());
        assert_eq!(content.to_string(), content_out);
        assert_eq!(primitive.to_string(), primitive_out);

        let (content, primitive) = (&x).content_and_canonical_primitive_part();
        assert_eq!(content.to_string(), content_out);
        assert_eq!(primitive.to_string(), primitive_out);

        assert_eq!(
            x.clone().canonical_primitive_part().to_string(),
            primitive_out
        );
        assert_eq!((&x).canonical_primitive_part().to_string(), primitive_out);
    };
    test("0", "0", "0");
    test("1", "1", "1");
    // The units all have canonical primitive part 1.
    test("i", "1", "1");
    test("-1", "1", "1");
    test("-i/3", "1/3", "1");
    test("1/2+i/3", "1/6", "3+2i");
    test("-1/2+3i/4", "1/4", "3+2i");
    test("-6+9i", "3", "3+2i");
    test("-6-9i", "3", "3-2i");
    test("2/3+4i/3", "2/3", "2-i");
    test("1/5-i/5", "1/5", "1+i");
}

#[test]
fn canonical_primitive_part_properties() {
    gaussian_rational_gen().test_properties(|x| {
        let cpp = (&x).canonical_primitive_part();
        assert!(cpp.real.is_valid());
        assert!(cpp.imaginary.is_valid());
        assert_eq!(x.clone().canonical_primitive_part(), cpp);
        let content = (&x).content();
        assert_eq!(
            (&x).content_and_canonical_primitive_part(),
            (content.clone(), cpp.clone())
        );
        assert_eq!(
            x.clone().content_and_canonical_primitive_part(),
            (content, cpp.clone())
        );

        // It is the primitive part in canonical unit form.
        assert_eq!((&x).primitive_part().canonicalize_unit(), cpp);
        if x == 0u32 {
            assert_eq!(cpp, GaussianInteger::ZERO);
        } else {
            assert_eq!((&cpp).content(), 1);
            assert!(cpp.real > 0u32);
            assert!(-&cpp.real < cpp.imaginary && cpp.imaginary <= cpp.real);
        }
        // All four associates share it.
        assert_eq!((-&x).canonical_primitive_part(), cpp);
        assert_eq!((&x).mul_i().canonical_primitive_part(), cpp);
        assert_eq!(
            (&x).conjugate().canonical_primitive_part(),
            (&cpp).conjugate().canonicalize_unit()
        );
    });

    gaussian_integer_gen().test_properties(|x| {
        // A Gaussian integer has the same canonical primitive part either way.
        assert_eq!(
            GaussianRational::from(x.clone()).canonical_primitive_part(),
            x.canonical_primitive_part()
        );
    });
}
