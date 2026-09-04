// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::cmp::Ordering::*;
use malachite_base::num::arithmetic::traits::{
    Abs, Conjugate, Content, ContentAndPrimitivePart, MulI, PrimitivePart, Sign,
};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_nz::gaussian_integer::GaussianInteger;
use malachite_nz::test_util::generators::gaussian_integer_gen;
use malachite_q::Rational;
use malachite_q::gaussian_rational::GaussianRational;
use malachite_q::test_util::generators::{
    gaussian_rational_gen, gaussian_rational_rational_pair_gen, rational_gen,
};
use std::str::FromStr;

#[test]
fn test_content_and_primitive_part() {
    let test = |s, content_out, primitive_out| {
        let x = GaussianRational::from_str(s).unwrap();

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
    test("-1/2", "1/2", "-1");
    test("3i/4", "3/4", "i");
    test("2+4i", "2", "1+2i");
    test("-6+9i", "3", "-2+3i");
    test("1/2+i/3", "1/6", "3+2i");
    test("2/3+2i/3", "2/3", "1+i");
    test("2/3+4i/3", "2/3", "1+2i");
    test("1/2+i", "1/2", "1+2i");
    test("-3/5+4i/5", "1/5", "-3+4i");
    test("22/7-i/3", "1/21", "66-7i");
    test("1/6-5i/4", "1/12", "2-15i");
}

fn scale(x: &GaussianInteger, c: &Rational) -> GaussianRational {
    GaussianRational {
        real: Rational::from(x.real.clone()) * c,
        imaginary: Rational::from(x.imaginary.clone()) * c,
    }
}

#[test]
fn content_and_primitive_part_properties() {
    gaussian_rational_gen().test_properties(|x| {
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
        assert!(content >= 0);
        if x == GaussianRational::ZERO {
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

    gaussian_rational_rational_pair_gen().test_properties(|(x, c)| {
        let (content, primitive) = (&x).content_and_primitive_part();
        let scaled = GaussianRational {
            real: &x.real * &c,
            imaginary: &x.imaginary * &c,
        };
        let (scaled_content, scaled_primitive) = scaled.content_and_primitive_part();
        assert_eq!(scaled_content, content * (&c).abs());
        if c != 0 && x != GaussianRational::ZERO {
            assert_eq!(scaled_primitive, if c > 0 { primitive } else { -primitive });
        }
    });

    gaussian_integer_gen().test_properties(|x| {
        let (content, primitive) = (&x).content_and_primitive_part();
        assert_eq!(
            GaussianRational::from(x).content_and_primitive_part(),
            (Rational::from(content), primitive)
        );
    });

    rational_gen().test_properties(|q| {
        let (content, primitive) = GaussianRational::from(q.clone()).content_and_primitive_part();
        assert_eq!(content, (&q).abs());
        assert_eq!(
            primitive,
            match q.sign() {
                Equal => GaussianInteger::ZERO,
                Greater => GaussianInteger::ONE,
                Less => -GaussianInteger::ONE,
            }
        );
    });
}
