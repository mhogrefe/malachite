// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::named::Named;
use malachite_base::num::basic::traits::Zero;
use malachite_base::test_util::generators::u64_polynomial_gen;
use malachite_base::u64_polynomial::U64Polynomial;

#[test]
fn test_named() {
    assert_eq!(U64Polynomial::NAME, "U64Polynomial");
}

#[test]
fn test_constants() {
    assert!(U64Polynomial::ZERO.is_valid());
    assert!(U64Polynomial::one().is_valid());
    assert!(U64Polynomial::two().is_valid());

    assert_eq!(U64Polynomial::ZERO.to_string(), "0");
    assert_eq!(U64Polynomial::one().to_string(), "1");
    assert_eq!(U64Polynomial::two().to_string(), "2");

    // The zero polynomial has no degree at all; the other two are constants.
    assert_eq!(U64Polynomial::ZERO.degree(), None);
    assert_eq!(U64Polynomial::one().degree(), Some(0));
    assert_eq!(U64Polynomial::two().degree(), Some(0));

    assert_eq!(U64Polynomial::from(0u32), U64Polynomial::ZERO);
    assert_eq!(U64Polynomial::from(1u32), U64Polynomial::one());
    assert_eq!(U64Polynomial::from(2u32), U64Polynomial::two());
}

#[test]
fn test_default() {
    let p = U64Polynomial::default();
    assert!(p.is_valid());
    assert_eq!(p, U64Polynomial::ZERO);
    assert_eq!(p.to_string(), "0");
}

#[test]
fn constants_properties() {
    u64_polynomial_gen().test_properties(|p| {
        // Every polynomial a generator produces is valid, and is its own clone.
        assert!(p.is_valid());
        assert_eq!(p.clone(), p);
        // Only the zero polynomial has no degree.
        assert_eq!(p.degree().is_none(), p == U64Polynomial::ZERO);
    });
}
