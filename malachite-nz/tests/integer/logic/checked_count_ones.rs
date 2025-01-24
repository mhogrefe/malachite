// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::logic::traits::CountOnes;
use malachite_base::test_util::generators::signed_gen_var_2;
use malachite_nz::integer::Integer;
use malachite_nz::platform::SignedLimb;
use malachite_nz::test_util::generators::integer_gen;
use malachite_nz::test_util::integer::logic::checked_count_ones::{
    integer_checked_count_ones_alt_1, integer_checked_count_ones_alt_2,
};
use std::str::FromStr;

#[test]
fn test_checked_count_ones() {
    let test = |s, out| {
        let u = Integer::from_str(s).unwrap();

        assert_eq!(u.checked_count_ones(), out);
        assert_eq!(integer_checked_count_ones_alt_1(&u), out);
        assert_eq!(integer_checked_count_ones_alt_2(&u), out);
    };
    test("0", Some(0));
    test("105", Some(4));
    test("-105", None);
    test("1000000000000", Some(13));
    test("-1000000000000", None);
    test("4294967295", Some(32));
    test("-4294967295", None);
    test("4294967296", Some(1));
    test("-4294967296", None);
    test("18446744073709551615", Some(64));
    test("-18446744073709551615", None);
    test("18446744073709551616", Some(1));
    test("-18446744073709551616", None);
}

#[test]
fn checked_count_ones_properties() {
    integer_gen().test_properties(|x| {
        let ones = x.checked_count_ones();
        assert_eq!(integer_checked_count_ones_alt_1(&x), ones);
        assert_eq!(integer_checked_count_ones_alt_2(&x), ones);
        assert_eq!(ones == Some(0), x == 0);
        assert_eq!((!x).checked_count_zeros(), ones);
    });

    signed_gen_var_2::<SignedLimb>().test_properties(|i| {
        assert_eq!(
            Integer::from(i).checked_count_ones(),
            Some(CountOnes::count_ones(i))
        );
    });
}
