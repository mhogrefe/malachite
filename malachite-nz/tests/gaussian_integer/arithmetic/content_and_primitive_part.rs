// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::cmp::Ordering::*;
use malachite_base::num::arithmetic::traits::{
    Conjugate, Content, ContentAndPrimitivePart, Gcd, MulI, PrimitivePart, Sign, UnsignedAbs,
};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_nz::gaussian_integer::GaussianInteger;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_nz::test_util::generators::{
    gaussian_integer_gen, gaussian_integer_natural_pair_gen, integer_gen,
};
use std::str::FromStr;

#[test]
fn test_content_and_primitive_part() {
    let test = |s, content_out, primitive_out| {
        let x = GaussianInteger::from_str(s).unwrap();

        let (content, primitive) = x.clone().content_and_primitive_part();
        assert!(content.is_valid());
        assert!(primitive.real.is_valid());
        assert!(primitive.imaginary.is_valid());
        assert_eq!(content.to_string(), content_out);
        assert_eq!(primitive.to_string(), primitive_out);

        let (content, primitive) = (&x).content_and_primitive_part();
        assert_eq!(content.to_string(), content_out);
        assert_eq!(primitive.to_string(), primitive_out);

        assert_eq!(x.clone().content().to_string(), content_out);
        assert_eq!((&x).content().to_string(), content_out);
        assert_eq!(x.clone().primitive_part().to_string(), primitive_out);
        assert_eq!((&x).primitive_part().to_string(), primitive_out);
    };
    test("0", "0", "0");
    test("1", "1", "1");
    test("i", "1", "i");
    test("-1", "1", "-1");
    test("2", "2", "1");
    test("-6i", "6", "-i");
    test("2+4i", "2", "1+2i");
    test("-6+9i", "3", "-2+3i");
    test("6-9i", "3", "2-3i");
    test("-6-9i", "3", "-2-3i");
    test("12+18i", "6", "2+3i");
    test("7+11i", "1", "7+11i");
    test("3+4i", "1", "3+4i");
    test("1000000000000+2500000000000i", "500000000000", "2+5i");
}

fn scale(x: &GaussianInteger, n: &Natural) -> GaussianInteger {
    GaussianInteger {
        real: &x.real * Integer::from(n),
        imaginary: &x.imaginary * Integer::from(n),
    }
}

#[test]
fn content_and_primitive_part_properties() {
    gaussian_integer_gen().test_properties(|x| {
        let (content, primitive) = (&x).content_and_primitive_part();
        assert!(content.is_valid());
        assert!(primitive.real.is_valid());
        assert!(primitive.imaginary.is_valid());

        assert_eq!(
            x.clone().content_and_primitive_part(),
            (content.clone(), primitive.clone())
        );
        assert_eq!((&x).content(), content);
        assert_eq!(x.clone().content(), content);
        assert_eq!((&x).primitive_part(), primitive);
        assert_eq!(x.clone().primitive_part(), primitive);

        // the product is the original number
        assert_eq!(scale(&primitive, &content), x);
        // the content is the GCD of the parts
        assert_eq!(
            content,
            (&x.real).unsigned_abs().gcd((&x.imaginary).unsigned_abs())
        );
        if x == 0u32 {
            assert_eq!(content, 0);
            assert_eq!(primitive, GaussianInteger::ZERO);
        } else {
            assert_ne!(content, 0);
            // the primitive part has coprime parts, and is its own primitive part
            assert_eq!((&primitive).content(), 1);
            assert_eq!((&primitive).primitive_part(), primitive);
        }
        // units and conjugation move through the primitive part
        assert_eq!(
            (-&x).content_and_primitive_part(),
            (content.clone(), -&primitive)
        );
        assert_eq!(
            (&x).mul_i().content_and_primitive_part(),
            (content.clone(), (&primitive).mul_i())
        );
        assert_eq!(
            (&x).conjugate().content_and_primitive_part(),
            (content, (&primitive).conjugate())
        );
    });

    gaussian_integer_natural_pair_gen().test_properties(|(x, n)| {
        let (content, primitive) = (&x).content_and_primitive_part();
        let (scaled_content, scaled_primitive) = scale(&x, &n).content_and_primitive_part();
        assert_eq!(scaled_content, content * &n);
        if n != 0u32 && x != 0u32 {
            assert_eq!(scaled_primitive, primitive);
        }
    });

    integer_gen().test_properties(|n| {
        let (content, primitive) = GaussianInteger::from(n.clone()).content_and_primitive_part();
        assert_eq!(content, (&n).unsigned_abs());
        assert_eq!(
            primitive,
            match n.sign() {
                Equal => GaussianInteger::ZERO,
                Greater => GaussianInteger::ONE,
                Less => -GaussianInteger::ONE,
            }
        );
    });
}
