// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::Sign;
use malachite_base::test_util::generators::unsigned_gen;
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use malachite_nz::test_util::generators::natural_gen;
use rug;
use std::cmp::Ordering::*;
use std::str::FromStr;

#[test]
fn test_sign() {
    let test = |s, out| {
        assert_eq!(Natural::from_str(s).unwrap().sign(), out);
        assert_eq!(rug::Integer::from_str(s).unwrap().cmp0(), out);
    };
    test("0", Equal);
    test("123", Greater);
    test("1000000000000", Greater);
}

#[test]
fn sign_properties() {
    natural_gen().test_properties(|n| {
        let sign = n.sign();
        assert_eq!(rug::Integer::from(&n).cmp0(), sign);
        assert_ne!(sign, Less);
        assert_eq!(n.partial_cmp(&0), Some(sign));
        assert_eq!((-n).sign(), sign.reverse());
    });

    unsigned_gen::<Limb>().test_properties(|u| {
        assert_eq!(Natural::from(u).sign(), u.sign());
    });
}
