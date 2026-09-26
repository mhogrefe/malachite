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
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::polynomial::{
    Content, ContentAndPrimitivePart, Polynomial, PrimitivePart, PrimitivePartAssign,
};
use malachite_base::test_util::generators::unsigned_polynomial_gen;
use malachite_base::unsigned_polynomial::UnsignedPolynomial;

#[test]
fn test_content_and_primitive_part() {
    fn test<T: PrimitiveUnsigned>(s: &str, content: T, primitive_part: &str) {
        let p = UnsignedPolynomial::<T>::from_str(s).unwrap();
        assert_eq!(p.clone().content(), content);
        assert_eq!((&p).content(), content);

        let q = p.clone().primitive_part();
        assert!(q.is_valid());
        assert_eq!(q.to_string(), primitive_part);
        assert_eq!((&p).primitive_part(), q);
        let mut r = p.clone();
        r.primitive_part_assign();
        assert_eq!(r, q);

        assert_eq!(p.clone().content_and_primitive_part(), (content, q.clone()));
        assert_eq!((&p).content_and_primitive_part(), (content, q));
    }
    test::<u8>("0", 0, "0");
    test::<u8>("1", 1, "1");
    test::<u8>("7", 7, "1");
    test::<u8>("x", 1, "x");
    test::<u8>("6*x^2+4*x+10", 2, "3*x^2+2*x+5");
    test::<u8>("6*x^2+9", 3, "2*x^2+3");
    test::<u8>("255*x+85", 85, "3*x+1");
    test::<u8>("3*x^2+5*x+7", 1, "3*x^2+5*x+7");
    test::<u64>("1000000000000*x^3+2000000000000", 1000000000000, "x^3+2");
    test::<u64>("12*x^5+18*x^3+30", 6, "2*x^5+3*x^3+5");
    test::<u128>(
        "340282366920938463463374607431768211455*x+5",
        5,
        "68056473384187692692674921486353642291*x+1",
    );
}

#[test]
fn content_and_primitive_part_properties() {
    unsigned_polynomial_gen().test_properties(|p| {
        let content = (&p).content();
        assert_eq!(p.clone().content(), content);
        let primitive_part = (&p).primitive_part();
        assert!(primitive_part.is_valid());
        assert_eq!(p.clone().primitive_part(), primitive_part);
        let mut q = p.clone();
        q.primitive_part_assign();
        assert_eq!(q, primitive_part);
        assert_eq!(
            (&p).content_and_primitive_part(),
            (content, primitive_part.clone())
        );
        assert_eq!(
            p.clone().content_and_primitive_part(),
            (content, primitive_part.clone())
        );

        // The content divides every coefficient, and is zero only for the zero polynomial.
        assert_eq!(content == 0, p == UnsignedPolynomial::ZERO);
        for &c in p.coefficients_asc() {
            assert!(c.divisible_by(content));
        }
        // p = cont(p) pp(p), coefficient by coefficient, and the primitive part is primitive.
        assert_eq!(primitive_part.len(), p.len());
        for (&c, &d) in p
            .coefficients_asc()
            .iter()
            .zip(primitive_part.coefficients_asc())
        {
            assert_eq!(c, content * d);
        }
        if p != UnsignedPolynomial::ZERO {
            assert_eq!((&primitive_part).content(), 1);
        }
        // Taking the primitive part again changes nothing.
        assert_eq!((&primitive_part).primitive_part(), primitive_part);
    });
}
