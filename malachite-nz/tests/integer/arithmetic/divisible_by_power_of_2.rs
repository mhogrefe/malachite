// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::DivisibleByPowerOf2;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::test_util::generators::{signed_unsigned_pair_gen_var_1, unsigned_gen};
use malachite_nz::integer::Integer;
use malachite_nz::platform::SignedLimb;
use malachite_nz::test_util::generators::{
    integer_gen, integer_unsigned_pair_gen_var_2, integer_unsigned_pair_gen_var_4,
    integer_unsigned_pair_gen_var_5, natural_unsigned_pair_gen_var_4,
};
use std::str::FromStr;

#[test]
fn test_divisible_by_power_of_2() {
    let test = |n, pow, out| {
        assert_eq!(
            Integer::from_str(n).unwrap().divisible_by_power_of_2(pow),
            out
        );
        assert_eq!(
            rug::Integer::from_str(n)
                .unwrap()
                .is_divisible_2pow(u32::exact_from(pow)),
            out
        );
    };
    test("0", 0, true);
    test("0", 10, true);
    test("0", 100, true);
    test("123", 0, true);
    test("123", 1, false);
    test("-123", 0, true);
    test("-123", 1, false);
    test("1000000000000", 0, true);
    test("1000000000000", 12, true);
    test("1000000000000", 13, false);
    test("-1000000000000", 0, true);
    test("-1000000000000", 12, true);
    test("-1000000000000", 13, false);
    test("4294967295", 0, true);
    test("4294967295", 1, false);
    test("-4294967295", 0, true);
    test("-4294967295", 1, false);
    test("4294967296", 0, true);
    test("4294967296", 32, true);
    test("4294967296", 33, false);
    test("-4294967296", 0, true);
    test("-4294967296", 32, true);
    test("-4294967296", 33, false);
    test("18446744073709551615", 0, true);
    test("18446744073709551615", 1, false);
    test("-18446744073709551615", 0, true);
    test("-18446744073709551615", 1, false);
    test("18446744073709551616", 0, true);
    test("18446744073709551616", 64, true);
    test("18446744073709551616", 65, false);
    test("-18446744073709551616", 0, true);
    test("-18446744073709551616", 64, true);
    test("-18446744073709551616", 65, false);
}

#[test]
fn divisible_by_power_of_2_properties() {
    integer_unsigned_pair_gen_var_2().test_properties(|(x, pow)| {
        let divisible = x.divisible_by_power_of_2(pow);
        assert_eq!(
            rug::Integer::from(&x).is_divisible_2pow(u32::exact_from(pow)),
            divisible
        );
        if x != 0 {
            assert_eq!(x.trailing_zeros().unwrap() >= pow, divisible);
        }
        assert_eq!((-&x).divisible_by_power_of_2(pow), divisible);
        assert!((&x << pow).divisible_by_power_of_2(pow));
        assert_eq!(&x >> pow << pow == x, divisible);
    });

    integer_unsigned_pair_gen_var_4().test_properties(|(x, pow)| {
        assert!(x.divisible_by_power_of_2(pow));
        assert!(rug::Integer::from(&x).is_divisible_2pow(u32::exact_from(pow)));
        if x != 0 {
            assert!(x.trailing_zeros().unwrap() >= pow);
        }
        assert!((-&x).divisible_by_power_of_2(pow));
        assert_eq!(&x >> pow << pow, x);
    });

    integer_unsigned_pair_gen_var_5().test_properties(|(x, pow)| {
        assert!(!x.divisible_by_power_of_2(pow));
        assert!(!rug::Integer::from(&x).is_divisible_2pow(u32::exact_from(pow)));
        if x != 0 {
            assert!(x.trailing_zeros().unwrap() < pow);
        }
        assert!(!(-&x).divisible_by_power_of_2(pow));
        assert_ne!(&x >> pow << pow, x);
    });

    integer_gen().test_properties(|x| {
        assert!(x.divisible_by_power_of_2(0));
    });

    unsigned_gen().test_properties(|pow| {
        assert!(Integer::ZERO.divisible_by_power_of_2(pow));
    });

    natural_unsigned_pair_gen_var_4().test_properties(|(x, pow)| {
        assert_eq!(
            x.divisible_by_power_of_2(pow),
            Integer::from(x).divisible_by_power_of_2(pow)
        );
    });

    signed_unsigned_pair_gen_var_1::<SignedLimb, u64>().test_properties(|(x, pow)| {
        assert_eq!(
            x.divisible_by_power_of_2(pow),
            Integer::from(x).divisible_by_power_of_2(pow)
        );
    });
}
