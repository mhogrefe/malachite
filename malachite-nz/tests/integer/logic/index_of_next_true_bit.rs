// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::{NegativeOne, Zero};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::{BitAccess, BitScan};
use malachite_base::test_util::generators::common::GenConfig;
use malachite_base::test_util::generators::{
    signed_unsigned_pair_gen_var_1, unsigned_gen, unsigned_vec_unsigned_pair_gen_var_20,
};
use malachite_nz::integer::logic::bit_scan::limbs_index_of_next_true_bit_neg;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_nz::platform::{Limb, SignedLimb};
use malachite_nz::test_util::generators::{
    integer_gen, integer_unsigned_pair_gen_var_2, natural_unsigned_pair_gen_var_4,
};
use malachite_nz::test_util::integer::logic::index_of_next_true_bit::*;
use rug;
use std::str::FromStr;

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_index_of_next_true_bit_neg() {
    let test = |xs, u, out| {
        assert_eq!(limbs_index_of_next_true_bit_neg(xs, u), out);
    };
    test(&[1], 0, 0);
    test(&[1], 100, 100);
    test(&[0b100], 0, 2);
    test(&[0b100], 1, 2);
    test(&[0b100], 2, 2);
    test(&[0b100], 3, 3);
    test(&[0, 0b101], 0, 32);
    test(&[0, 0b101], 20, 32);
    test(&[0, 0b101], 31, 32);
    test(&[0, 0b101], 32, 32);
    test(&[0, 0b101], 33, 33);
    test(&[0, 0b101], 34, 35);
    test(&[0, 0b101], 35, 35);
    test(&[0, 0b101], 36, 36);
    test(&[0, 0b101], 100, 100);
    test(&[0, 0, 0b101], 64, 64);
    test(&[0, 0, 0b101], 66, 67);
    test(&[0, 0, 0b101, 0b101], 96, 97);
    test(&[0, 0, 0b101, 0b101], 98, 99);
}

#[test]
fn test_index_of_next_true_bit() {
    let test = |s, u, out| {
        let n = Integer::from_str(s).unwrap();
        assert_eq!(n.index_of_next_true_bit(u), out);
        assert_eq!(integer_index_of_next_true_bit_alt(&n, u), out);
        assert_eq!(
            rug::Integer::from_str(s)
                .unwrap()
                .find_one(u32::exact_from(u))
                .map(u64::from),
            out
        );
    };
    test("0", 0, None);
    test("0", 100, None);
    test("47244640256", 0, Some(32));
    test("47244640256", 20, Some(32));
    test("47244640256", 31, Some(32));
    test("47244640256", 32, Some(32));
    test("47244640256", 33, Some(33));
    test("47244640256", 34, Some(35));
    test("47244640256", 35, Some(35));
    test("47244640256", 36, None);
    test("47244640256", 100, None);
    test("340282366925890223602069384504899796992", 91, Some(91));
    test("340282366925890223602069384504899796992", 92, Some(128));

    test("-21474836480", 0, Some(32));
    test("-21474836480", 20, Some(32));
    test("-21474836480", 31, Some(32));
    test("-21474836480", 32, Some(32));
    test("-21474836480", 33, Some(33));
    test("-21474836480", 34, Some(35));
    test("-21474836480", 35, Some(35));
    test("-21474836480", 36, Some(36));
    test("-21474836480", 100, Some(100));
    test("-92233720368547758080", 64, Some(64));
    test("-92233720368547758080", 66, Some(67));
    test("-396140812663555408336267509760", 96, Some(97));
    test("-396140812663555408336267509760", 98, Some(99));
}

#[test]
fn limbs_index_of_next_true_bit_neg_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_unsigned_pair_gen_var_20().test_properties_with_config(&config, |(xs, u)| {
        assert_eq!(
            Some(limbs_index_of_next_true_bit_neg(&xs, u)),
            (-Natural::from_owned_limbs_asc(xs)).index_of_next_true_bit(u)
        );
    });
}

#[test]
fn index_of_next_true_bit_properties() {
    integer_unsigned_pair_gen_var_2().test_properties(|(n, u)| {
        let result = n.index_of_next_true_bit(u);
        assert_eq!(result, integer_index_of_next_true_bit_alt(&n, u));
        assert_eq!(
            rug::Integer::from(&n)
                .find_one(u32::exact_from(u))
                .map(u64::from),
            result
        );
        assert_eq!(result.is_some(), &n >> u != 0);
        if let Some(result) = result {
            assert!(result >= u);
            assert!(n.get_bit(result));
            assert_eq!(result == u, n.get_bit(u));
        }
        assert_eq!((!n).index_of_next_false_bit(u), result);
    });

    integer_gen().test_properties(|n| {
        assert_eq!(n.index_of_next_true_bit(0), n.trailing_zeros());
    });

    unsigned_gen().test_properties(|u| {
        assert_eq!(Integer::ZERO.index_of_next_true_bit(u), None);
        assert_eq!(Integer::NEGATIVE_ONE.index_of_next_true_bit(u), Some(u));
    });

    natural_unsigned_pair_gen_var_4().test_properties(|(n, index)| {
        assert_eq!(
            Integer::from(&n).index_of_next_true_bit(index),
            n.index_of_next_true_bit(index)
        );
    });

    signed_unsigned_pair_gen_var_1::<SignedLimb, u64>().test_properties(|(i, index)| {
        assert_eq!(
            Integer::from(i).index_of_next_true_bit(index),
            i.index_of_next_true_bit(index)
        );
    });
}
