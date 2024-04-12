// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::Parity;
use malachite_base::num::logic::traits::TrailingZeros;
use malachite_base::test_util::generators::signed_gen_var_6;
use malachite_nz::integer::Integer;
use malachite_nz::platform::{Limb, SignedLimb};
use malachite_nz::test_util::generators::integer_gen;
use malachite_nz::test_util::integer::logic::trailing_zeros::integer_trailing_zeros_alt;
use std::str::FromStr;

#[test]
fn test_trailing_zeros() {
    let test = |s, out| {
        let n = Integer::from_str(s).unwrap();
        assert_eq!(n.trailing_zeros(), out);
        assert_eq!(integer_trailing_zeros_alt(&n), out);
    };
    test("0", None);
    test("123", Some(0));
    test("-123", Some(0));
    test("1000000000000", Some(12));
    test("-1000000000000", Some(12));
    test("4294967295", Some(0));
    test("-4294967295", Some(0));
    test("4294967296", Some(32));
    test("-4294967296", Some(32));
    test("18446744073709551615", Some(0));
    test("-18446744073709551615", Some(0));
    test("18446744073709551616", Some(64));
    test("-18446744073709551616", Some(64));
}

#[allow(clippy::cmp_owned, clippy::useless_conversion)]
#[test]
fn significant_bits_properties() {
    integer_gen().test_properties(|x| {
        let trailing_zeros = x.trailing_zeros();
        assert_eq!(integer_trailing_zeros_alt(&x), trailing_zeros);
        assert_eq!(trailing_zeros.is_none(), x == 0);
        assert_eq!((-&x).trailing_zeros(), trailing_zeros);
        if x != 0 {
            let trailing_zeros = trailing_zeros.unwrap();
            assert_ne!((!&x).trailing_zeros() == Some(0), trailing_zeros == 0);
            if trailing_zeros <= u64::from(Limb::MAX) {
                assert!((&x >> trailing_zeros).odd());
                assert_eq!(&x >> trailing_zeros << trailing_zeros, x);
            }
        }
    });

    signed_gen_var_6::<SignedLimb>().test_properties(|i| {
        assert_eq!(
            Integer::from(i).trailing_zeros(),
            Some(TrailingZeros::trailing_zeros(i))
        );
    });
}
