// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_nz::natural::Natural;
use malachite_nz::test_util::generators::natural_gen;
use malachite_q::Rational;
use std::str::FromStr;

#[test]
fn test_from_natural() {
    let test = |s| {
        let u = Natural::from_str(s).unwrap();

        let x = Rational::from(u.clone());
        assert!(x.is_valid());
        assert_eq!(x.to_string(), s);

        let x = Rational::from(&u);
        assert!(x.is_valid());
        assert_eq!(x.to_string(), s);
    };
    test("0");
    test("123");
    test("1000000000000");
}

#[test]
fn from_natural_properties() {
    natural_gen().test_properties(|x| {
        let rational_x = Rational::from(x.clone());
        assert!(rational_x.is_valid());
        assert_eq!(rational_x.to_string(), x.to_string());

        let rational_x_alt = Rational::from(&x);
        assert!(rational_x_alt.is_valid());
        assert_eq!(rational_x_alt, rational_x);

        assert_eq!(Natural::try_from(&rational_x).as_ref(), Ok(&x));
        assert_eq!(Natural::try_from(rational_x), Ok(x));
    });
}
