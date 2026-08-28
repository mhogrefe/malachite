// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::conversion::traits::IsGaussianInteger;
use malachite_nz::integer::Integer;
use malachite_nz::test_util::generators::integer_gen;
use std::str::FromStr;

#[test]
fn test_is_gaussian_integer() {
    let test = |n, out| {
        assert_eq!(Integer::from_str(n).unwrap().is_gaussian_integer(), out);
    };
    test("0", true);
    test("1", true);
    test("100", true);
    test("-100", true);
}

#[test]
fn is_gaussian_integer_properties() {
    integer_gen().test_properties(|n| {
        assert!(n.is_gaussian_integer());
    });
}
