// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::PowerOf2;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::{BitAccess, SignificantBits};
use malachite_base::test_util::generators::common::GenConfig;
use malachite_base::test_util::generators::{
    signed_unsigned_pair_gen_var_1, unsigned_vec_unsigned_pair_gen_var_18,
};
use malachite_nz::integer::logic::bit_access::limbs_get_bit_neg;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_nz::platform::{Limb, SignedLimb};
use malachite_nz::test_util::generators::{integer_gen_var_4, integer_unsigned_pair_gen_var_2};
use rug;
use std::str::FromStr;

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_get_bit_neg() {
    let test = |xs: &[Limb], index: u64, out: bool| {
        assert_eq!(limbs_get_bit_neg(xs, index), out);
    };
    test(&[1], 0, true);
    test(&[1], 100, true);
    test(&[123], 2, true);
    test(&[123], 3, false);
    test(&[123], 100, true);
    test(&[0, 0b1011], 0, false);
    test(&[0, 0b1011], 32, true);
    test(&[0, 0b1011], 33, false);
    test(&[0, 0b1011], 34, true);
    test(&[0, 0b1011], 35, false);
    test(&[0, 0b1011], 100, true);
    test(&[1, 0b1011], 0, true);
    test(&[1, 0b1011], 32, false);
    test(&[1, 0b1011], 33, false);
    test(&[1, 0b1011], 34, true);
    test(&[1, 0b1011], 35, false);
    test(&[1, 0b1011], 100, true);
}

#[test]
fn test_get_bit() {
    let test = |n, index, out| {
        assert_eq!(Integer::from_str(n).unwrap().get_bit(index), out);
        assert_eq!(
            rug::Integer::from_str(n)
                .unwrap()
                .get_bit(u32::exact_from(index)),
            out
        );
    };

    test("0", 0, false);
    test("0", 100, false);
    test("123", 2, false);
    test("123", 3, true);
    test("123", 100, false);
    test("-123", 0, true);
    test("-123", 1, false);
    test("-123", 100, true);
    test("1000000000000", 12, true);
    test("1000000000000", 100, false);
    test("-1000000000000", 12, true);
    test("-1000000000000", 100, true);
    test("4294967295", 31, true);
    test("4294967295", 32, false);
    test("4294967296", 31, false);
    test("4294967296", 32, true);
    test("4294967296", 33, false);
    test("-4294967295", 0, true);
    test("-4294967295", 1, false);
    test("-4294967295", 31, false);
    test("-4294967295", 32, true);
    test("-4294967295", 33, true);
    test("-4294967296", 0, false);
    test("-4294967296", 31, false);
    test("-4294967296", 32, true);
    test("-4294967296", 33, true);
}

#[test]
fn limbs_get_bit_neg_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_unsigned_pair_gen_var_18().test_properties_with_config(&config, |(xs, index)| {
        assert_eq!(
            (-Natural::from_limbs_asc(&xs)).get_bit(index),
            limbs_get_bit_neg(&xs, index)
        );
    });
}

#[test]
fn get_bit_properties() {
    integer_unsigned_pair_gen_var_2().test_properties(|(n, index)| {
        let bit = n.get_bit(index);
        assert_eq!(rug::Integer::from(&n).get_bit(u32::exact_from(index)), bit);
        assert_eq!(&n & Integer::power_of_2(index) != 0, bit);
        assert_eq!(!(!n).get_bit(index), bit);
    });

    integer_gen_var_4().test_properties(|n| {
        let significant_bits = n.significant_bits();
        assert!(!n.get_bit(significant_bits));
        if n != 0 {
            assert!(n.get_bit(significant_bits - 1));
        }
    });

    signed_unsigned_pair_gen_var_1::<SignedLimb, u64>().test_properties(|(i, index)| {
        assert_eq!(Integer::from(i).get_bit(index), i.get_bit(index));
    });
}
