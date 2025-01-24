// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::DivisibleByPowerOf2;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::test_util::generators::common::GenConfig;
use malachite_base::test_util::generators::{
    unsigned_gen, unsigned_pair_gen_var_2, unsigned_vec_unsigned_pair_gen_var_20,
};
use malachite_nz::natural::arithmetic::divisible_by_power_of_2::limbs_divisible_by_power_of_2;
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use malachite_nz::test_util::generators::{
    natural_gen, natural_unsigned_pair_gen_var_10, natural_unsigned_pair_gen_var_4,
    natural_unsigned_pair_gen_var_9,
};
use rug;
use std::str::FromStr;

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_divisible_by_power_of_2() {
    let test = |xs: &[Limb], pow: u64, out: bool| {
        assert_eq!(limbs_divisible_by_power_of_2(xs, pow), out);
    };
    test(&[1], 0, true);
    test(&[1], 1, false);
    test(&[2], 0, true);
    test(&[2], 1, true);
    test(&[2], 2, false);
    test(&[3], 1, false);
    test(&[122, 456], 1, true);
    test(&[0, 0, 1], 64, true);
    test(&[0, 0, 1], 65, false);
    test(&[0, 0, 1], 100, false);
    test(&[3567587328, 232], 11, true);
    test(&[3567587328, 232], 12, true);
    test(&[3567587328, 232], 13, false);
}

#[test]
fn test_divisible_by_power_of_2() {
    let test = |n, pow, out| {
        assert_eq!(
            Natural::from_str(n).unwrap().divisible_by_power_of_2(pow),
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
    test("1000000000000", 0, true);
    test("1000000000000", 12, true);
    test("1000000000000", 13, false);
    test("4294967295", 0, true);
    test("4294967295", 1, false);
    test("4294967296", 0, true);
    test("4294967296", 32, true);
    test("4294967296", 33, false);
    test("18446744073709551615", 0, true);
    test("18446744073709551615", 1, false);
    test("18446744073709551616", 0, true);
    test("18446744073709551616", 64, true);
    test("18446744073709551616", 65, false);
}

#[test]
fn limbs_divisible_by_power_of_2_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_unsigned_pair_gen_var_20().test_properties_with_config(&config, |(xs, pow)| {
        assert_eq!(
            limbs_divisible_by_power_of_2(&xs, pow),
            Natural::from_owned_limbs_asc(xs).divisible_by_power_of_2(pow),
        );
    });
}

#[test]
fn divisible_by_power_of_2_properties() {
    natural_unsigned_pair_gen_var_4().test_properties(|(x, pow)| {
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

    natural_unsigned_pair_gen_var_9().test_properties(|(x, pow)| {
        assert!(x.divisible_by_power_of_2(pow));
        assert!(rug::Integer::from(&x).is_divisible_2pow(u32::exact_from(pow)));
        if x != 0 {
            assert!(x.trailing_zeros().unwrap() >= pow);
        }
        assert!((-&x).divisible_by_power_of_2(pow));
        assert_eq!(&x >> pow << pow, x);
    });

    natural_unsigned_pair_gen_var_10().test_properties(|(x, pow)| {
        assert!(!x.divisible_by_power_of_2(pow));
        assert!(!rug::Integer::from(&x).is_divisible_2pow(u32::exact_from(pow)));
        if x != 0 {
            assert!(x.trailing_zeros().unwrap() < pow);
        }
        assert!(!(-&x).divisible_by_power_of_2(pow));
        assert_ne!(&x >> pow << pow, x);
    });

    natural_gen().test_properties(|x| {
        assert!(x.divisible_by_power_of_2(0));
    });

    unsigned_gen().test_properties(|pow| {
        assert!(Natural::ZERO.divisible_by_power_of_2(pow));
    });

    unsigned_pair_gen_var_2::<Limb, u64>().test_properties(|(x, pow)| {
        assert_eq!(
            x.divisible_by_power_of_2(pow),
            Natural::from(x).divisible_by_power_of_2(pow)
        );
    });
}
