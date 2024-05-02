// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_nz::test_util::generators::natural_gen;
use malachite_nz::test_util::natural::arithmetic::neg::neg_num;
use num::BigUint;
use rug;
use std::str::FromStr;

#[test]
fn test_neg() {
    let test = |s, out| {
        let u = Natural::from_str(s).unwrap();

        let neg = -u.clone();
        assert!(neg.is_valid());
        assert_eq!(neg.to_string(), out);

        let neg = -&u;
        assert!(neg.is_valid());
        assert_eq!(neg.to_string(), out);

        assert_eq!((-rug::Integer::from_str(s).unwrap()).to_string(), out);
        assert_eq!(neg_num(BigUint::from_str(s).unwrap()).to_string(), out);
    };
    test("0", "0");
    test("123", "-123");
    test("1000000000000", "-1000000000000");
    test("2147483648", "-2147483648");
}

#[test]
fn neg_properties() {
    natural_gen().test_properties(|x| {
        let neg = -x.clone();
        assert!(neg.is_valid());

        let neg_alt = -&x;
        assert!(neg_alt.is_valid());
        assert_eq!(neg_alt, neg);

        assert_eq!(Integer::from(&(-rug::Integer::from(&x))), neg);
        assert_eq!(Integer::from(&neg_num(BigUint::from(&x))), neg);

        assert_eq!(-Integer::from(&x), neg);
        assert_eq!(neg == x, x == 0);
        assert_eq!(-neg, x);
    });
}
