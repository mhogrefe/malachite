// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::traits::Zero;
use malachite_nz::natural_polynomial::NaturalPolynomial;
use malachite_nz::test_util::generators::natural_polynomial_gen;

#[test]
fn test_constants() {
    assert!(NaturalPolynomial::ZERO.is_valid());
    assert!(NaturalPolynomial::one().is_valid());
    assert!(NaturalPolynomial::two().is_valid());

    assert_eq!(NaturalPolynomial::ZERO.to_string(), "0");
    assert_eq!(NaturalPolynomial::one().to_string(), "1");
    assert_eq!(NaturalPolynomial::two().to_string(), "2");

    // The zero polynomial has no degree at all; the other two are constants.
    assert_eq!(NaturalPolynomial::ZERO.degree(), None);
    assert_eq!(NaturalPolynomial::one().degree(), Some(0));
    assert_eq!(NaturalPolynomial::two().degree(), Some(0));

    assert_eq!(NaturalPolynomial::from(0u32), NaturalPolynomial::ZERO);
    assert_eq!(NaturalPolynomial::from(1u32), NaturalPolynomial::one());
    assert_eq!(NaturalPolynomial::from(2u32), NaturalPolynomial::two());
}

#[test]
fn test_default() {
    let p = NaturalPolynomial::default();
    assert!(p.is_valid());
    assert_eq!(p, NaturalPolynomial::ZERO);
    assert_eq!(p.to_string(), "0");
}

#[test]
fn constants_properties() {
    natural_polynomial_gen().test_properties(|p| {
        // Every polynomial a generator produces is valid, and is its own clone.
        assert!(p.is_valid());
        assert_eq!(p.clone(), p);
        // Only the zero polynomial has no degree.
        assert_eq!(p.degree().is_none(), p == NaturalPolynomial::ZERO);
    });
}
