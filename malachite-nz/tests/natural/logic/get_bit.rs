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
    unsigned_pair_gen_var_2, unsigned_vec_unsigned_pair_gen_var_16,
};
use malachite_nz::natural::logic::bit_access::limbs_get_bit;
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use malachite_nz::test_util::generators::{natural_gen, natural_unsigned_pair_gen_var_4};
use malachite_nz::test_util::natural::logic::get_bit::num_get_bit;
use num::BigUint;
use rug;
use std::str::FromStr;

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_get_bit() {
    let test = |xs: &[Limb], index: u64, out: bool| {
        assert_eq!(limbs_get_bit(xs, index), out);
    };
    test(&[1], 0, true);
    test(&[1], 100, false);
    test(&[123], 2, false);
    test(&[123], 3, true);
    test(&[123], 100, false);
    test(&[0, 0b1011], 0, false);
    test(&[0, 0b1011], 32, true);
    test(&[0, 0b1011], 33, true);
    test(&[0, 0b1011], 34, false);
    test(&[0, 0b1011], 35, true);
    test(&[0, 0b1011], 100, false);
}

#[test]
fn test_get_bit() {
    let test = |n, index, out| {
        assert_eq!(Natural::from_str(n).unwrap().get_bit(index), out);
        assert_eq!(num_get_bit(&BigUint::from_str(n).unwrap(), index), out);
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
    test("1000000000000", 12, true);
    test("1000000000000", 100, false);
}

#[test]
fn limbs_get_bit_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_unsigned_pair_gen_var_16().test_properties_with_config(&config, |(xs, index)| {
        assert_eq!(
            Natural::from_limbs_asc(&xs).get_bit(index),
            limbs_get_bit(&xs, index)
        );
    });
}

#[test]
fn get_bit_properties() {
    natural_unsigned_pair_gen_var_4().test_properties(|(n, index)| {
        let bit = n.get_bit(index);
        assert_eq!(num_get_bit(&BigUint::from(&n), index), bit);
        assert_eq!(rug::Integer::from(&n).get_bit(u32::exact_from(index)), bit);
        assert_eq!(&n & Natural::power_of_2(index) != 0, bit);
        assert_ne!((!n).get_bit(index), bit);
    });

    natural_gen().test_properties(|n| {
        let significant_bits = n.significant_bits();
        assert!(!n.get_bit(significant_bits));
        if n != 0 {
            assert!(n.get_bit(significant_bits - 1));
        }
    });

    unsigned_pair_gen_var_2::<Limb, u64>().test_properties(|(u, index)| {
        assert_eq!(Natural::from(u).get_bit(index), u.get_bit(index));
    });
}
