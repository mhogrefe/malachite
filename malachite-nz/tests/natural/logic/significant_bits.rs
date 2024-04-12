// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::conversion::traits::WrappingFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::test_util::generators::common::GenConfig;
use malachite_base::test_util::generators::unsigned_vec_gen_var_1;
use malachite_nz::natural::arithmetic::log_base_2::limbs_floor_log_base_2;
use malachite_nz::natural::logic::significant_bits::limbs_significant_bits;
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use num::BigUint;
use rug;
use std::str::FromStr;

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_significant_bits() {
    let test = |xs, out| {
        assert_eq!(limbs_significant_bits::<Limb>(xs), out);
    };
    test(&[0b1], 1);
    test(&[0b10], 2);
    test(&[0b11], 2);
    test(&[0b100], 3);
    test(&[0, 0b1], 33);
    test(&[0, 0b1101], 36);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_significant_bits_fail() {
    limbs_significant_bits::<Limb>(&[]);
}

#[test]
fn test_significant_bits() {
    let test = |n, out| {
        assert_eq!(Natural::from_str(n).unwrap().significant_bits(), out);
        assert_eq!(
            u64::wrapping_from(BigUint::from_str(n).unwrap().bits()),
            out
        );
        assert_eq!(
            u64::from(rug::Integer::from_str(n).unwrap().significant_bits()),
            out
        );
    };
    test("0", 0);
    test("100", 7);
    test("1000000000000", 40);
    test("4294967295", 32);
    test("4294967296", 33);
    test("18446744073709551615", 64);
    test("18446744073709551616", 65);
}

#[test]
fn limbs_significant_bits_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_gen_var_1().test_properties_with_config(&config, |xs| {
        let significant_bits = limbs_significant_bits(&xs);
        assert_eq!(xs.len() == 1, significant_bits <= Limb::WIDTH);
        assert_eq!(significant_bits, limbs_floor_log_base_2(&xs) + 1);
        assert_eq!(
            significant_bits,
            Natural::from_owned_limbs_asc(xs).significant_bits()
        );
    });
}
