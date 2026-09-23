// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::basic::traits::Zero;
use malachite_nz::integer_polynomial::IntegerPolynomial;
use malachite_nz::test_util::generators::integer_polynomial_gen;

#[test]
fn test_constants() {
    assert!(IntegerPolynomial::ZERO.is_valid());
    assert!(IntegerPolynomial::one().is_valid());
    assert!(IntegerPolynomial::two().is_valid());
    assert!(IntegerPolynomial::negative_one().is_valid());
    assert!(IntegerPolynomial::x().is_valid());

    assert_eq!(IntegerPolynomial::ZERO.to_string(), "0");
    assert_eq!(IntegerPolynomial::one().to_string(), "1");
    assert_eq!(IntegerPolynomial::two().to_string(), "2");
    assert_eq!(IntegerPolynomial::negative_one().to_string(), "-1");
    assert_eq!(IntegerPolynomial::x().to_string(), "x");

    // The zero polynomial has no degree at all; the others are constants, except x.
    assert_eq!(IntegerPolynomial::ZERO.degree(), None);
    assert_eq!(IntegerPolynomial::one().degree(), Some(0));
    assert_eq!(IntegerPolynomial::two().degree(), Some(0));
    assert_eq!(IntegerPolynomial::negative_one().degree(), Some(0));
    assert_eq!(IntegerPolynomial::x().degree(), Some(1));

    assert_eq!(IntegerPolynomial::from(0u32), IntegerPolynomial::ZERO);
    assert_eq!(IntegerPolynomial::from(1u32), IntegerPolynomial::one());
    assert_eq!(IntegerPolynomial::from(2u32), IntegerPolynomial::two());
    assert_eq!(
        IntegerPolynomial::from(-1i32),
        IntegerPolynomial::negative_one()
    );
    assert_eq!(
        IntegerPolynomial::from_str("x").unwrap(),
        IntegerPolynomial::x()
    );
}

#[test]
fn test_default() {
    let p = IntegerPolynomial::default();
    assert!(p.is_valid());
    assert_eq!(p, IntegerPolynomial::ZERO);
    assert_eq!(p.to_string(), "0");
}

#[test]
fn constants_properties() {
    integer_polynomial_gen().test_properties(|p| {
        // Every polynomial a generator produces is valid, and is its own clone.
        assert!(p.is_valid());
        assert_eq!(p.clone(), p);
        // Only the zero polynomial has no degree.
        assert_eq!(p.degree().is_none(), p == IntegerPolynomial::ZERO);
    });
}
