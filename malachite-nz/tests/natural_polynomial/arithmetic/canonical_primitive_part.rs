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
use malachite_nz::natural::Natural;
use malachite_nz::natural_polynomial::NaturalPolynomial;
use malachite_nz::test_util::generators::natural_polynomial_gen;

#[test]
fn test_canonical_primitive_part() {
    let test = |s, content, primitive_part| {
        let p = NaturalPolynomial::from_str(s).unwrap();
        let content = Natural::from_str(content).unwrap();
        let q = p.clone().canonical_primitive_part();
        assert!(q.is_valid());
        assert_eq!(q.to_string(), primitive_part);
        assert_eq!((&p).canonical_primitive_part(), q);
        let mut r = p.clone();
        r.canonical_primitive_part_assign();
        assert_eq!(r, q);
        assert_eq!(
            p.clone().content_and_canonical_primitive_part(),
            (content.clone(), q.clone())
        );
        assert_eq!((&p).content_and_canonical_primitive_part(), (content, q));
    };
    test("0", "0", "0");
    test("1", "1", "1");
    test("7", "7", "1");
    test("6*x^2+4*x+10", "2", "3*x^2+2*x+5");
    test("12*x^5+18*x^3+30", "6", "2*x^5+3*x^3+5");
}

#[test]
fn canonical_primitive_part_properties() {
    natural_polynomial_gen().test_properties(|p| {
        // With no negative coefficients there is only one associate, so this is the primitive part.
        let cpp = (&p).canonical_primitive_part();
        assert!(cpp.is_valid());
        assert_eq!(cpp, (&p).primitive_part());
        assert_eq!(p.clone().canonical_primitive_part(), cpp);
        let mut q = p.clone();
        q.canonical_primitive_part_assign();
        assert_eq!(q, cpp);
        assert_eq!(
            (&p).content_and_canonical_primitive_part(),
            (&p).content_and_primitive_part()
        );
        assert_eq!(
            p.clone().content_and_canonical_primitive_part(),
            p.content_and_primitive_part()
        );
    });
}
