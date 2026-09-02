// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::IsUnit;
use malachite_nz::natural::Natural;
use malachite_nz::test_util::generators::natural_gen;
use std::str::FromStr;

#[test]
fn test_is_unit() {
    let test = |s, out| {
        let x = Natural::from_str(s).unwrap();
        assert_eq!(x.is_unit(), out);
    };
    test("0", false);
    test("1", true);
    test("2", false);
    test("123", false);
    test("1000000000000", false);
}

#[test]
fn is_unit_properties() {
    natural_gen().test_properties(|x| {
        let is_unit = x.is_unit();
        assert_eq!(is_unit, x == 1u32);
    });
}
