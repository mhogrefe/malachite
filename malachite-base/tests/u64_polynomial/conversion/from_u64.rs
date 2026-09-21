// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::basic::traits::Zero;
use malachite_base::test_util::generators::{bool_gen, unsigned_gen};
use malachite_base::u64_polynomial::U64Polynomial;

#[test]
fn test_from_u64() {
    let test = |s, out| {
        let p = U64Polynomial::from(u64::from_str(s).unwrap());
        assert!(p.is_valid());
        assert_eq!(p.to_string(), out);
    };
    test("0", "0");
    test("1", "1");
    test("123", "123");
    test("18446744073709551615", "18446744073709551615");
}

#[test]
fn test_from_primitive() {
    assert_eq!(U64Polynomial::from(0u32).to_string(), "0");
    assert_eq!(U64Polynomial::from(123u8).to_string(), "123");
    assert_eq!(U64Polynomial::from(123u16).to_string(), "123");
    assert_eq!(U64Polynomial::from(123u32).to_string(), "123");
    assert_eq!(U64Polynomial::from(123u64).to_string(), "123");
    assert_eq!(U64Polynomial::from(false).to_string(), "0");
    assert_eq!(U64Polynomial::from(true).to_string(), "1");
}

#[test]
fn from_u64_properties() {
    unsigned_gen::<u64>().test_properties(|x| {
        let p = U64Polynomial::from(x);
        assert!(p.is_valid());
        // A number becomes the constant polynomial with that value: degree 0, or no degree when the
        // number is zero.
        if x == 0 {
            assert_eq!(p, U64Polynomial::ZERO);
            assert_eq!(p.degree(), None);
        } else {
            assert_eq!(p.degree(), Some(0));
        }
        assert_eq!(p.coefficient(0), x);
        assert_eq!(p.to_string(), x.to_string());
    });

    // A narrower type goes the same way its widening does.
    unsigned_gen::<u32>().test_properties(|x| {
        assert_eq!(U64Polynomial::from(x), U64Polynomial::from(u64::from(x)));
    });

    bool_gen().test_properties(|b| {
        assert_eq!(U64Polynomial::from(b), U64Polynomial::from(u64::from(b)));
    });
}
