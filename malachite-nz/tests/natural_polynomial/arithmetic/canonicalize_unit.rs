// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{CanonicalizeUnit, CanonicalizeUnitAssign};
use malachite_nz::natural_polynomial::NaturalPolynomial;
use malachite_nz::test_util::generators::natural_polynomial_gen;

#[test]
fn test_canonicalize_unit() {
    let test = |s, out| {
        let p = NaturalPolynomial::from_str(s).unwrap();
        let q = p.clone().canonicalize_unit();
        assert!(q.is_valid());
        assert_eq!(q.to_string(), out);
        assert_eq!((&p).canonicalize_unit(), q);
        let mut r = p;
        r.canonicalize_unit_assign();
        assert_eq!(r, q);
    };
    test("0", "0");
    test("3*x^2+2", "3*x^2+2");
}

#[test]
fn canonicalize_unit_properties() {
    natural_polynomial_gen().test_properties(|p| {
        let q = (&p).canonicalize_unit();
        assert!(q.is_valid());
        assert_eq!(p.clone().canonicalize_unit(), q);
        let mut r = p.clone();
        r.canonicalize_unit_assign();
        assert_eq!(r, q);
        // Canonicalizing again changes nothing.
        assert_eq!((&q).canonicalize_unit(), q);
        // Every coefficient is non-negative, so this is the identity.
        assert_eq!(q, p);
    });
}
