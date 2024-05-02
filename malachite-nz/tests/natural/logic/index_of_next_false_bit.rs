// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::{BitAccess, BitScan};
use malachite_base::test_util::generators::common::GenConfig;
use malachite_base::test_util::generators::{
    unsigned_gen, unsigned_pair_gen_var_2, unsigned_vec_unsigned_pair_gen_var_16,
};
use malachite_nz::natural::logic::bit_scan::limbs_index_of_next_false_bit;
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use malachite_nz::test_util::generators::{natural_gen, natural_unsigned_pair_gen_var_4};
use malachite_nz::test_util::natural::logic::index_of_next_false_bit::*;
use rug;
use std::str::FromStr;

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_index_of_next_false_bit() {
    let test = |xs, u, out| {
        assert_eq!(limbs_index_of_next_false_bit(xs, u), out);
    };
    test(&[], 0, 0);
    test(&[], 100, 100);
    test(&[0], 0, 0);
    test(&[0], 100, 100);
    test(&[0b100], 0, 0);
    test(&[0b100], 1, 1);
    test(&[0b100], 2, 3);
    test(&[0b100], 3, 3);
    test(&[0, 0b1011], 0, 0);
    test(&[0, 0b1011], 20, 20);
    test(&[0, 0b1011], 31, 31);
    test(&[0, 0b1011], 32, 34);
    test(&[0, 0b1011], 33, 34);
    test(&[0, 0b1011], 34, 34);
    test(&[0, 0b1011], 35, 36);
    test(&[0, 0b1011], 100, 100);
    test(&[0, 0b1011, 0xfffffff0, u32::MAX, 1], 64, 64);
    test(&[0, 0b1011, 0xfffffff0, u32::MAX, 1], 68, 129);
}

#[test]
fn test_index_of_next_false_bit() {
    let test = |n, u, out| {
        assert_eq!(
            Natural::from_str(n).unwrap().index_of_next_false_bit(u),
            out
        );
        assert_eq!(
            natural_index_of_next_false_bit_alt(&Natural::from_str(n).unwrap(), u),
            out
        );
        assert_eq!(
            rug::Integer::from_str(n)
                .unwrap()
                .find_zero(u32::exact_from(u))
                .map(u64::from),
            out
        );
    };
    test("0", 0, Some(0));
    test("0", 100, Some(100));
    test("47244640256", 0, Some(0));
    test("47244640256", 20, Some(20));
    test("47244640256", 31, Some(31));
    test("47244640256", 32, Some(34));
    test("47244640256", 33, Some(34));
    test("47244640256", 34, Some(34));
    test("47244640256", 35, Some(36));
    test("47244640256", 100, Some(100));
    test("680564733841876926631601309731428237312", 64, Some(64));
    test("680564733841876926631601309731428237312", 68, Some(129));
}

#[test]
fn limbs_index_of_next_false_bit_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_unsigned_pair_gen_var_16().test_properties_with_config(&config, |(xs, u)| {
        assert_eq!(
            Some(limbs_index_of_next_false_bit(&xs, u)),
            Natural::from_owned_limbs_asc(xs).index_of_next_false_bit(u)
        );
    });
}

#[test]
fn index_of_next_false_bit_properties() {
    natural_unsigned_pair_gen_var_4().test_properties(|(n, u)| {
        let result = n.index_of_next_false_bit(u);
        assert_eq!(result, natural_index_of_next_false_bit_alt(&n, u));
        assert_eq!(
            rug::Integer::from(&n)
                .find_zero(u32::exact_from(u))
                .map(u64::from),
            result
        );
        let result = result.unwrap();
        assert!(result >= u);
        assert!(!n.get_bit(result));
        assert_eq!(result == u, !n.get_bit(u));
        assert_eq!((!n).index_of_next_true_bit(u), Some(result));
    });

    natural_gen().test_properties(|n| {
        assert_eq!(n.index_of_next_false_bit(0), (!n).trailing_zeros());
    });

    unsigned_gen().test_properties(|u| {
        assert_eq!(Natural::ZERO.index_of_next_false_bit(u), Some(u));
    });

    unsigned_pair_gen_var_2::<Limb, u64>().test_properties(|(u, index)| {
        assert_eq!(
            Natural::from(u).index_of_next_false_bit(index),
            u.index_of_next_false_bit(index)
        );
    });
}
