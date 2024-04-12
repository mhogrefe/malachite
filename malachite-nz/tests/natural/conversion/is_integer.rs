// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::conversion::traits::IsInteger;
use malachite_nz::natural::Natural;
use malachite_nz::test_util::generators::natural_gen;
use std::str::FromStr;

#[test]
fn test_is_integer() {
    let test = |n, out| {
        assert_eq!(Natural::from_str(n).unwrap().is_integer(), out);
    };
    test("0", true);
    test("1", true);
    test("100", true);
}

#[test]
fn is_integer_properties() {
    natural_gen().test_properties(|n| {
        assert!(n.is_integer());
    });
}
