// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::Sign;
use malachite_base::num::basic::traits::Zero;
use malachite_base::polynomial::{Content, ContentAndPrimitivePart, Polynomial, PrimitivePart};
use malachite_nz::integer_polynomial::IntegerPolynomial;
use malachite_nz::test_util::generators::integer_polynomial_gen;
use malachite_q::Rational;
use malachite_q::rational_polynomial::RationalPolynomial;
use malachite_q::test_util::generators::rational_polynomial_gen;
use std::cmp::Ordering::*;

#[test]
fn test_content_and_primitive_part() {
    let test = |s, content, primitive_part| {
        let p = RationalPolynomial::from_str(s).unwrap();
        let c = p.clone().content();
        assert!(c.is_valid());
        assert_eq!(c.to_string(), content);
        assert_eq!((&p).content(), c);

        let q = p.clone().primitive_part();
        assert!(q.is_valid());
        assert_eq!(q.to_string(), primitive_part);
        assert_eq!((&p).primitive_part(), q);

        assert_eq!(
            p.clone().content_and_primitive_part(),
            (c.clone(), q.clone())
        );
        assert_eq!((&p).content_and_primitive_part(), (c, q));
    };
    test("0", "0", "0");
    test("1", "1", "1");
    test("-1", "1", "1");
    test("3/7", "3/7", "1");
    test("-x", "1", "x");
    test("6*x+4", "2", "3*x+2");
    test("-2/3*x-4/3", "2/3", "x+2");
    test("1/2*x^2+1/3", "1/6", "3*x^2+2");
    test("-5/2*x^3+10/3", "5/6", "3*x^3-4");
    test("x-1/2", "1/2", "2*x-1");
    test("-1/2*x^2+1/3", "1/6", "3*x^2-2");
    test("4/9*x^2-2/3*x+8/15", "2/45", "10*x^2-15*x+12");
}

#[test]
fn content_and_primitive_part_properties() {
    rational_polynomial_gen().test_properties(|p| {
        let content = (&p).content();
        assert!(content.is_valid());
        assert!(content >= 0u32);
        assert_eq!(p.clone().content(), content);
        let primitive_part = (&p).primitive_part();
        assert!(primitive_part.is_valid());
        assert_eq!(p.clone().primitive_part(), primitive_part);
        assert_eq!(
            (&p).content_and_primitive_part(),
            (content.clone(), primitive_part.clone())
        );
        assert_eq!(
            p.clone().content_and_primitive_part(),
            (content.clone(), primitive_part.clone())
        );

        assert_eq!(content == 0u32, p == RationalPolynomial::ZERO);
        if p != RationalPolynomial::ZERO {
            assert_eq!((&primitive_part).content(), 1u32);
            assert_eq!(primitive_part.leading_coefficient().sign(), Greater);
        }
        // p = sgn(lc(p)) cont(p) pp(p), coefficient by coefficient.
        let negative = p.leading_coefficient() < 0u32;
        assert_eq!(primitive_part.len(), p.len());
        for (i, d) in primitive_part.coefficients_asc().iter().enumerate() {
            let product = &content * Rational::from(d);
            assert_eq!(
                p.coefficient(u64::try_from(i).unwrap()),
                if negative { -product } else { product }
            );
        }
        // Dividing by the content leaves a primitive integer polynomial.
        assert_eq!(
            (&RationalPolynomial::from(primitive_part.clone())).primitive_part(),
            primitive_part
        );
    });

    integer_polynomial_gen().test_properties(|p| {
        // An integer polynomial has the same content and primitive part either way.
        let q = RationalPolynomial::from(p.clone());
        assert_eq!((&q).content(), Rational::from((&p).content()));
        assert_eq!((&q).primitive_part(), (&p).primitive_part());
    });

    // The primitive part is an `IntegerPolynomial`.
    let _: IntegerPolynomial = RationalPolynomial::ZERO.primitive_part();
}
