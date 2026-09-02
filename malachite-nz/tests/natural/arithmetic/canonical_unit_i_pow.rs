// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{CanonicalUnitIPow, CanonicalizeUnit};
use malachite_nz::natural::Natural;
use malachite_nz::test_util::generators::natural_gen;
use std::str::FromStr;

#[test]
fn test_canonical_unit_i_pow() {
    let test = |s, out| {
        let x = Natural::from_str(s).unwrap();
        assert_eq!(x.canonical_unit_i_pow(), out);
    };
    test("0", 0);
    test("1", 0);
    test("123", 0);
    test("1000000000000", 0);
}

#[test]
fn canonical_unit_i_pow_properties() {
    natural_gen().test_properties(|x| {
        let k = x.canonical_unit_i_pow();
        assert!(k == 0 || k == 2);
        assert_eq!(k, 0);
        let y = (&x).canonicalize_unit();
        assert_eq!(y.canonical_unit_i_pow(), 0);
        assert_eq!(y, x);
    });
}
