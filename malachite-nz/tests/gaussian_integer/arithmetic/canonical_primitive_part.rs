// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{
    CanonicalPrimitivePart, CanonicalPrimitivePartAssign, CanonicalizeUnit, Conjugate, Content,
    ContentAndCanonicalPrimitivePart, MulI, PrimitivePart,
};
use malachite_base::num::basic::traits::Zero;
use malachite_nz::gaussian_integer::GaussianInteger;
use malachite_nz::test_util::generators::gaussian_integer_gen;
use std::str::FromStr;

#[test]
fn test_canonical_primitive_part() {
    let test = |s, content_out, primitive_out| {
        let x = GaussianInteger::from_str(s).unwrap();

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
        let mut y = x;
        y.canonical_primitive_part_assign();
        assert_eq!(y.to_string(), primitive_out);
    };
    test("0", "0", "0");
    test("1", "1", "1");
    // The units all have canonical primitive part 1.
    test("i", "1", "1");
    test("-1", "1", "1");
    test("-i", "1", "1");
    test("2", "2", "1");
    test("-6i", "6", "1");
    // The canonical associate has argument in (-pi/4, pi/4]: positive real part, and imaginary part
    // b with -a < b <= a.
    test("2+4i", "2", "2-i");
    test("-6+9i", "3", "3+2i");
    test("6-9i", "3", "3+2i");
    test("-6-9i", "3", "3-2i");
    test("12+18i", "6", "3-2i");
    test("7+11i", "1", "11-7i");
    test("3+4i", "1", "4-3i");
    // The boundary: argument pi/4 is canonical, -pi/4 is not.
    test("1+i", "1", "1+i");
    test("1-i", "1", "1+i");
    test("1000000000000+2500000000000i", "500000000000", "5-2i");
}

#[test]
fn canonical_primitive_part_properties() {
    gaussian_integer_gen().test_properties(|x| {
        let cpp = (&x).canonical_primitive_part();
        assert!(cpp.real.is_valid());
        assert!(cpp.imaginary.is_valid());
        assert_eq!(x.clone().canonical_primitive_part(), cpp);
        let mut y = x.clone();
        y.canonical_primitive_part_assign();
        assert_eq!(y, cpp);
        let content = (&x).content();
        assert_eq!(
            (&x).content_and_canonical_primitive_part(),
            (content.clone(), cpp.clone())
        );
        assert_eq!(
            x.clone().content_and_canonical_primitive_part(),
            (content.clone(), cpp.clone())
        );

        // It is the primitive part in canonical unit form.
        assert_eq!((&x).primitive_part().canonicalize_unit(), cpp);
        assert_eq!((&cpp).canonical_primitive_part(), cpp);
        if x == 0u32 {
            assert_eq!(cpp, GaussianInteger::ZERO);
        } else {
            assert_eq!((&cpp).content(), 1);
            // The canonical associate: positive real part, imaginary part in (-a, a].
            assert!(cpp.real > 0u32);
            assert!(-&cpp.real < cpp.imaginary && cpp.imaginary <= cpp.real);
        }
        // All four associates share it.
        assert_eq!((-&x).canonical_primitive_part(), cpp);
        assert_eq!((&x).mul_i().canonical_primitive_part(), cpp);
        assert_eq!((-(&x).mul_i()).canonical_primitive_part(), cpp);
        // Conjugation can move it to a different associate, but the canonical form of the
        // conjugate's primitive part is the canonical form of the conjugate.
        assert_eq!(
            (&x).conjugate().canonical_primitive_part(),
            (&cpp).conjugate().canonicalize_unit()
        );
    });
}
