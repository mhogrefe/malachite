// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{
    CanonicalUnitIPow, CanonicalizeUnit, CanonicalizeUnitAssign,
};
use malachite_nz::natural::Natural;
use malachite_nz::test_util::generators::natural_gen;
use std::str::FromStr;

#[test]
fn test_canonicalize_unit() {
    let test = |s, out| {
        let x = Natural::from_str(s).unwrap();

        let y = x.clone().canonicalize_unit();
        assert!(y.is_valid());
        assert_eq!(y.to_string(), out);

        let y = (&x).canonicalize_unit();
        assert!(y.is_valid());
        assert_eq!(y.to_string(), out);

        let mut y = x;
        y.canonicalize_unit_assign();
        assert!(y.is_valid());
        assert_eq!(y.to_string(), out);
    };
    test("0", "0");
    test("123", "123");
    test("1000000000000", "1000000000000");
}

#[test]
fn canonicalize_unit_properties() {
    natural_gen().test_properties(|x| {
        let y = x.clone().canonicalize_unit();
        assert!(y.is_valid());
        assert_eq!((&x).canonicalize_unit(), y);
        let mut x_alt = x.clone();
        x_alt.canonicalize_unit_assign();
        assert_eq!(x_alt, y.clone());

        assert_eq!(y, x);
        assert_eq!((&y).canonicalize_unit(), y.clone());
        assert_eq!(y.canonical_unit_i_pow(), 0);
    });
}
