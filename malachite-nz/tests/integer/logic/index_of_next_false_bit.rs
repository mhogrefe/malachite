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
use malachite_nz::integer::logic::bit_scan::limbs_index_of_next_false_bit_neg;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_nz::platform::{Limb, SignedLimb};
use malachite_nz::test_util::generators::{
    integer_gen, integer_unsigned_pair_gen_var_2, natural_unsigned_pair_gen_var_4,
};
use malachite_nz::test_util::integer::logic::index_of_next_false_bit::*;
use rug;
use std::str::FromStr;

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_index_of_next_false_bit_neg() {
    let test = |xs, u, out| {
        assert_eq!(limbs_index_of_next_false_bit_neg(xs, u), out);
    };
    test(&[1], 0, None);
    test(&[1], 100, None);
    test(&[0b100], 0, Some(0));
    test(&[0b100], 1, Some(1));
    test(&[0b100], 2, None);
    test(&[0b100], 3, None);
    test(&[0, 0b101], 0, Some(0));
    test(&[0, 0b101], 20, Some(20));
    test(&[0, 0b101], 31, Some(31));
    test(&[0, 0b101], 32, Some(34));
    test(&[0, 0b101], 33, Some(34));
    test(&[0, 0b101], 34, Some(34));
    test(&[0, 0b101], 35, None);
    test(&[0, 0b101], 100, None);
    test(&[0, 0, 0b101], 36, Some(36));
    test(&[0, 0, 0b101], 64, Some(66));
    test(&[0, 0, 0b101, 0b101], 96, Some(96));
    test(&[0, 0, 0b101, 0b101], 97, Some(98));
}

#[test]
fn test_index_of_next_false_bit() {
    let test = |s, u, out| {
        let n = Integer::from_str(s).unwrap();
        assert_eq!(n.index_of_next_false_bit(u), out);
        assert_eq!(integer_index_of_next_false_bit_alt(&n, u), out);
        assert_eq!(
            rug::Integer::from_str(s)
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

    test("-21474836480", 0, Some(0));
    test("-21474836480", 20, Some(20));
    test("-21474836480", 31, Some(31));
    test("-21474836480", 32, Some(34));
    test("-21474836480", 33, Some(34));
    test("-21474836480", 34, Some(34));
    test("-21474836480", 35, None);
    test("-21474836480", 36, None);
    test("-21474836480", 100, None);
    test("-92233720368547758080", 36, Some(36));
    test("-92233720368547758080", 64, Some(66));
    test("-396140812663555408336267509760", 96, Some(96));
    test("-396140812663555408336267509760", 97, Some(98));
}

#[test]
fn limbs_index_of_next_false_bit_neg_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_unsigned_pair_gen_var_20().test_properties_with_config(&config, |(xs, u)| {
        assert_eq!(
            limbs_index_of_next_false_bit_neg(&xs, u),
            (-Natural::from_owned_limbs_asc(xs)).index_of_next_false_bit(u)
        );
    });
}

#[test]
fn index_of_next_false_bit_properties() {
    integer_unsigned_pair_gen_var_2().test_properties(|(n, u)| {
        let result = n.index_of_next_false_bit(u);
        assert_eq!(result, integer_index_of_next_false_bit_alt(&n, u));
        assert_eq!(
            rug::Integer::from(&n)
                .find_zero(u32::exact_from(u))
                .map(u64::from),
            result
        );
        assert_eq!(result.is_some(), &n >> u != -1);
        if let Some(result) = result {
            assert!(result >= u);
            assert!(!n.get_bit(result));
            assert_eq!(result == u, !n.get_bit(u));
        }
        assert_eq!((!n).index_of_next_true_bit(u), result);
    });

    integer_gen().test_properties(|n| {
        assert_eq!(n.index_of_next_false_bit(0), (!n).trailing_zeros());
    });

    unsigned_gen().test_properties(|u| {
        assert_eq!(Integer::ZERO.index_of_next_false_bit(u), Some(u));
        assert_eq!(Integer::NEGATIVE_ONE.index_of_next_false_bit(u), None);
    });

    natural_unsigned_pair_gen_var_4().test_properties(|(n, index)| {
        assert_eq!(
            Integer::from(&n).index_of_next_false_bit(index),
            n.index_of_next_false_bit(index)
        );
    });

    signed_unsigned_pair_gen_var_1::<SignedLimb, u64>().test_properties(|(i, index)| {
        assert_eq!(
            Integer::from(i).index_of_next_false_bit(index),
            i.index_of_next_false_bit(index)
        );
    });
}
