// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::IsUnit;
use malachite_q::Rational;
use malachite_q::test_util::generators::rational_gen;
use std::str::FromStr;

#[test]
fn test_is_unit() {
    let test = |s, out| {
        let x = Rational::from_str(s).unwrap();
        assert_eq!(x.is_unit(), out);
    };
    test("0", false);
    test("1", true);
    test("-1", true);
    test("22/7", true);
    test("-22/7", true);
    test("1/1000000000000", true);
}

#[test]
fn is_unit_properties() {
    rational_gen().test_properties(|x| {
        let is_unit = x.is_unit();
        assert_eq!(is_unit, x != 0u32);
        assert_eq!((-&x).is_unit(), is_unit);
    });
}
