// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::NegAssign;
use malachite_base::test_util::generators::signed_gen;
use malachite_nz::integer::Integer;
use malachite_nz::platform::{SignedDoubleLimb, SignedLimb};
use malachite_nz::test_util::generators::{integer_gen, natural_gen};
use num::BigInt;
use std::str::FromStr;

#[test]
fn test_neg() {
    let test = |s, out| {
        let u = Integer::from_str(s).unwrap();

        let neg = -u.clone();
        assert!(neg.is_valid());
        assert_eq!(neg.to_string(), out);

        let neg = -&u;
        assert!(neg.is_valid());
        assert_eq!(neg.to_string(), out);

        assert_eq!((-BigInt::from_str(s).unwrap()).to_string(), out);
        assert_eq!((-rug::Integer::from_str(s).unwrap()).to_string(), out);

        let mut x = u;
        x.neg_assign();
        assert!(x.is_valid());
        assert_eq!(x.to_string(), out);
    };
    test("0", "0");
    test("123", "-123");
    test("-123", "123");
    test("1000000000000", "-1000000000000");
    test("-1000000000000", "1000000000000");
    test("-2147483648", "2147483648");
    test("2147483648", "-2147483648");
}

#[test]
fn neg_properties() {
    integer_gen().test_properties(|x| {
        let negative = -x.clone();
        assert!(negative.is_valid());
        assert!(negative.is_valid());

        let negative_alt = -&x;
        assert!(negative_alt.is_valid());
        assert_eq!(negative_alt, negative);

        assert_eq!(Integer::from(&-BigInt::from(&x)), negative);
        assert_eq!(Integer::from(&-rug::Integer::from(&x)), negative);

        assert_eq!(negative == x, x == 0);
        assert_eq!(-&negative, x);
        assert_eq!(x + negative, 0);
    });

    signed_gen::<SignedLimb>().test_properties(|x| {
        assert_eq!(Integer::from(-SignedDoubleLimb::from(x)), -Integer::from(x));
    });

    natural_gen().test_properties(|x| {
        assert_eq!(-&x, -Integer::from(x));
    });
}
