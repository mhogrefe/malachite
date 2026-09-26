// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::DivisibleBy;
use malachite_base::num::basic::traits::Zero;
use malachite_base::polynomial::{
    Content, ContentAndPrimitivePart, Polynomial, PrimitivePart, PrimitivePartAssign,
};
use malachite_nz::integer::Integer;
use malachite_nz::integer_polynomial::IntegerPolynomial;
use malachite_nz::natural::Natural;
use malachite_nz::test_util::generators::integer_polynomial_gen;

#[test]
fn test_content_and_primitive_part() {
    let test = |s, content, primitive_part| {
        let p = IntegerPolynomial::from_str(s).unwrap();
        let content = Natural::from_str(content).unwrap();
        assert_eq!(p.clone().content(), content);
        assert_eq!((&p).content(), content);

        let q = p.clone().primitive_part();
        assert!(q.is_valid());
        assert_eq!(q.to_string(), primitive_part);
        assert_eq!((&p).primitive_part(), q);
        let mut r = p.clone();
        r.primitive_part_assign();
        assert_eq!(r, q);

        assert_eq!(
            p.clone().content_and_primitive_part(),
            (content.clone(), q.clone())
        );
        assert_eq!((&p).content_and_primitive_part(), (content, q));
    };
    test("0", "0", "0");
    test("1", "1", "1");
    test("-1", "1", "1");
    test("-7", "7", "1");
    test("x", "1", "x");
    test("-x", "1", "x");
    test("6*x^2+4*x+10", "2", "3*x^2+2*x+5");
    test("-6*x^2+4*x-10", "2", "3*x^2-2*x+5");
    test("-2*x-4", "2", "x+2");
    test("6*x^2-9", "3", "2*x^2-3");
    test("-3*x^2+5*x-7", "1", "3*x^2-5*x+7");
    test(
        "-1000000000000000000000*x+3000000000000000000000",
        "1000000000000000000000",
        "x-3",
    );
    test("12*x^5-18*x^3+30", "6", "2*x^5-3*x^3+5");
}

#[test]
fn content_and_primitive_part_properties() {
    integer_polynomial_gen().test_properties(|p| {
        let content = (&p).content();
        assert!(content.is_valid());
        assert_eq!(p.clone().content(), content);
        let primitive_part = (&p).primitive_part();
        assert!(primitive_part.is_valid());
        assert_eq!(p.clone().primitive_part(), primitive_part);
        let mut q = p.clone();
        q.primitive_part_assign();
        assert_eq!(q, primitive_part);
        assert_eq!(
            (&p).content_and_primitive_part(),
            (content.clone(), primitive_part.clone())
        );
        assert_eq!(
            p.clone().content_and_primitive_part(),
            (content.clone(), primitive_part.clone())
        );

        // The content divides every coefficient, and is zero only for the zero polynomial.
        assert_eq!(content == 0u32, p == IntegerPolynomial::ZERO);
        for c in p.coefficients_asc() {
            assert!(c.divisible_by(Integer::from(&content)));
        }
        if p != IntegerPolynomial::ZERO {
            assert_eq!((&primitive_part).content(), 1u32);
        }
        assert_eq!((&primitive_part).primitive_part(), primitive_part);
        assert_eq!(primitive_part.len(), p.len());
        // p = sgn(lc(p)) cont(p) pp(p), coefficient by coefficient.
        let negative = *p.leading_coefficient() < 0u32;
        let content = Integer::from(&content);
        for (c, d) in p
            .coefficients_asc()
            .iter()
            .zip(primitive_part.coefficients_asc())
        {
            let product = &content * d;
            assert_eq!(*c, if negative { -product } else { product });
        }
        if p != IntegerPolynomial::ZERO {
            assert!(*primitive_part.leading_coefficient() > 0u32);
        }
    });
}
