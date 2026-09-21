// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::named::Named;
use malachite_base::num::basic::traits::Zero;
use malachite_q::rational_polynomial::RationalPolynomial;
use malachite_q::test_util::generators::rational_polynomial_gen;

#[test]
fn test_named() {
    assert_eq!(RationalPolynomial::NAME, "RationalPolynomial");
}

#[test]
fn test_constants() {
    assert!(RationalPolynomial::ZERO.is_valid());
    assert!(RationalPolynomial::one().is_valid());
    assert!(RationalPolynomial::two().is_valid());

    assert_eq!(RationalPolynomial::ZERO.to_string(), "0");
    assert_eq!(RationalPolynomial::one().to_string(), "1");
    assert_eq!(RationalPolynomial::two().to_string(), "2");

    // The zero polynomial has no degree at all; the other two are constants.
    assert_eq!(RationalPolynomial::ZERO.degree(), None);
    assert_eq!(RationalPolynomial::one().degree(), Some(0));
    assert_eq!(RationalPolynomial::two().degree(), Some(0));

    assert_eq!(RationalPolynomial::from(0u32), RationalPolynomial::ZERO);
    assert_eq!(RationalPolynomial::from(1u32), RationalPolynomial::one());
    assert_eq!(RationalPolynomial::from(2u32), RationalPolynomial::two());
}

#[test]
fn test_default() {
    let p = RationalPolynomial::default();
    assert!(p.is_valid());
    assert_eq!(p, RationalPolynomial::ZERO);
    assert_eq!(p.to_string(), "0");
}

#[test]
fn constants_properties() {
    rational_polynomial_gen().test_properties(|p| {
        // Every polynomial a generator produces is valid, and is its own clone.
        assert!(p.is_valid());
        assert_eq!(p.clone(), p);
        // Only the zero polynomial has no degree.
        assert_eq!(p.degree().is_none(), p == RationalPolynomial::ZERO);
    });
}
