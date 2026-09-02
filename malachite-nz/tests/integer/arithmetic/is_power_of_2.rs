// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::IsPowerOf2;
use malachite_base::test_util::generators::{signed_gen, unsigned_gen};
use malachite_nz::integer::Integer;
use malachite_nz::platform::{Limb, SignedLimb};
use malachite_nz::test_util::generators::{integer_gen, natural_gen};
use rug;
use std::str::FromStr;

#[test]
fn test_is_power_of_2() {
    let test = |n, out| {
        assert_eq!(Integer::from_str(n).unwrap().is_power_of_2(), out);
        assert_eq!(rug::Integer::from_str(n).unwrap().is_power_of_two(), out);
    };
    test("0", false);
    test("1", true);
    test("2", true);
    test("3", false);
    test("4", true);
    test("5", false);
    test("6", false);
    test("7", false);
    test("8", true);
    test("1024", true);
    test("1025", false);
    test("1000000000000", false);
    test("1099511627776", true);
    test("-1", false);
    test("-2", false);
    test("-4", false);
    test("-1099511627776", false);
}

#[test]
fn is_power_of_2_properties() {
    integer_gen().test_properties(|x| {
        let is_power = x.is_power_of_2();
        assert_eq!(rug::Integer::from(&x).is_power_of_two(), is_power);
        assert_eq!(x > 0u32 && x.unsigned_abs_ref().is_power_of_2(), is_power);
        assert!(!(-&x).is_power_of_2() || !is_power);
        if is_power {
            assert!(x > 0u32);
        }
    });

    natural_gen().test_properties(|x| {
        assert_eq!(Integer::from(&x).is_power_of_2(), x.is_power_of_2());
    });

    unsigned_gen::<Limb>().test_properties(|u| {
        assert_eq!(Integer::from(u).is_power_of_2(), u.is_power_of_2());
    });

    signed_gen::<SignedLimb>().test_properties(|i| {
        assert_eq!(Integer::from(i).is_power_of_2(), i.is_power_of_2());
    });
}
