// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::basic::traits::Zero;
use malachite_base::test_util::generators::unsigned_polynomial_gen;
use malachite_base::unsigned_polynomial::UnsignedPolynomial;

#[test]
fn test_constants() {
    assert!(UnsignedPolynomial::<u64>::ZERO.is_valid());
    assert!(UnsignedPolynomial::<u64>::one().is_valid());
    assert!(UnsignedPolynomial::<u64>::two().is_valid());
    assert!(UnsignedPolynomial::<u64>::x().is_valid());

    assert_eq!(UnsignedPolynomial::<u64>::ZERO.to_string(), "0");
    assert_eq!(UnsignedPolynomial::<u64>::one().to_string(), "1");
    assert_eq!(UnsignedPolynomial::<u64>::two().to_string(), "2");
    assert_eq!(UnsignedPolynomial::<u64>::x().to_string(), "x");

    // The zero polynomial has no degree at all; the others are constants, except x.
    assert_eq!(UnsignedPolynomial::<u64>::ZERO.degree(), None);
    assert_eq!(UnsignedPolynomial::<u64>::one().degree(), Some(0));
    assert_eq!(UnsignedPolynomial::<u64>::two().degree(), Some(0));
    assert_eq!(UnsignedPolynomial::<u64>::x().degree(), Some(1));

    assert_eq!(
        UnsignedPolynomial::<u64>::from(0u64),
        UnsignedPolynomial::<u64>::ZERO
    );
    assert_eq!(
        UnsignedPolynomial::<u64>::from(1u64),
        UnsignedPolynomial::<u64>::one()
    );
    assert_eq!(
        UnsignedPolynomial::<u64>::from(2u64),
        UnsignedPolynomial::<u64>::two()
    );
    assert_eq!(
        UnsignedPolynomial::<u64>::from_str("x").unwrap(),
        UnsignedPolynomial::<u64>::x()
    );
}

#[test]
fn test_default() {
    let p = UnsignedPolynomial::<u64>::default();
    assert!(p.is_valid());
    assert_eq!(p, UnsignedPolynomial::<u64>::ZERO);
    assert_eq!(p.to_string(), "0");
}

#[test]
fn constants_properties() {
    unsigned_polynomial_gen().test_properties(|p| {
        // Every polynomial a generator produces is valid, and is its own clone.
        assert!(p.is_valid());
        assert_eq!(p.clone(), p);
        // Only the zero polynomial has no degree.
        assert_eq!(p.degree().is_none(), p == UnsignedPolynomial::<u64>::ZERO);
    });
}
