// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::logic::traits::NotAssign;
use malachite_base::test_util::generators::signed_gen;
use malachite_nz::integer::Integer;
use malachite_nz::platform::SignedLimb;
use malachite_nz::test_util::generators::{integer_gen, natural_gen};
use rug;
use std::str::FromStr;

#[test]
fn test_not() {
    let test = |s, out| {
        let n = Integer::from_str(s).unwrap();

        let not = !n.clone();
        assert!(not.is_valid());
        assert_eq!(not.to_string(), out);

        let not = !&n;
        assert!(not.is_valid());
        assert_eq!(not.to_string(), out);

        assert_eq!((!rug::Integer::from_str(s).unwrap()).to_string(), out);

        let mut x = n;
        x.not_assign();
        assert!(x.is_valid());
        assert_eq!(x.to_string(), out);
    };
    test("0", "-1");
    test("123", "-124");
    test("-123", "122");
    test("1000000000000", "-1000000000001");
    test("-1000000000000", "999999999999");
    test("-2147483648", "2147483647");
    test("2147483647", "-2147483648");
}

#[test]
fn not_properties() {
    integer_gen().test_properties(|x| {
        let not = !x.clone();
        assert!(not.is_valid());

        let rug_not = !rug::Integer::from(&x);
        assert_eq!(Integer::from(&rug_not), not);

        let not_alt = !&x;
        assert!(not_alt.is_valid());
        assert_eq!(not_alt, not);

        let mut not_alt = x.clone();
        not_alt.not_assign();
        assert_eq!(not_alt, not);

        assert_ne!(not, x);
        assert_eq!(!&not, x);
        assert_eq!(x >= 0, not < 0);
    });

    signed_gen::<SignedLimb>().test_properties(|i| {
        assert_eq!(!Integer::from(i), !i);
    });

    natural_gen().test_properties(|x| {
        assert_eq!(!Integer::from(&x), !x);
    });
}
