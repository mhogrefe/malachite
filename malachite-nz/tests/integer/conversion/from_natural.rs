// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_nz::test_util::generators::natural_gen;
use std::str::FromStr;

#[test]
fn test_from_natural() {
    let test = |s| {
        let u = Natural::from_str(s).unwrap();

        let x = Integer::from(u.clone());
        assert!(x.is_valid());
        assert_eq!(x.to_string(), s);

        let x = Integer::from(&u);
        assert!(x.is_valid());
        assert_eq!(x.to_string(), s);
    };
    test("0");
    test("123");
    test("1000000000000");
    test("4294967295");
    test("4294967296");
}

#[test]
fn from_natural_properties() {
    natural_gen().test_properties(|x| {
        let integer_x = Integer::from(x.clone());
        assert!(integer_x.is_valid());
        assert_eq!(integer_x.to_string(), x.to_string());

        let integer_x_alt = Integer::from(&x);
        assert!(integer_x_alt.is_valid());
        assert_eq!(integer_x_alt, integer_x);

        assert_eq!(Natural::try_from(&integer_x).as_ref(), Ok(&x));
        assert_eq!(Natural::try_from(integer_x), Ok(x));
    });
}
