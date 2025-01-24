// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{
    CeilingLogBase2, CheckedLogBase2, FloorLogBase2, IsPowerOf2, PowerOf2,
};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::test_util::generators::common::GenConfig;
use malachite_base::test_util::generators::{unsigned_gen_var_1, unsigned_vec_gen_var_1};
use malachite_nz::natural::arithmetic::log_base_2::{
    limbs_ceiling_log_base_2, limbs_checked_log_base_2, limbs_floor_log_base_2,
};
use malachite_nz::natural::logic::significant_bits::limbs_significant_bits;
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use malachite_nz::test_util::generators::natural_gen_var_2;
use std::str::FromStr;

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_floor_log_base_2() {
    let test = |xs, out| {
        assert_eq!(limbs_floor_log_base_2(xs), out);
    };
    test(&[0b1], 0);
    test(&[0b10], 1);
    test(&[0b11], 1);
    test(&[0b100], 2);
    test(&[0, 0b1], 32);
    test(&[0, 0b1101], 35);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_floor_log_base_2_fail() {
    limbs_floor_log_base_2(&[]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_ceiling_log_base_2() {
    let test = |xs, out| {
        assert_eq!(limbs_ceiling_log_base_2(xs), out);
    };
    test(&[0b1], 0);
    test(&[0b10], 1);
    test(&[0b11], 2);
    test(&[0b100], 2);
    test(&[0, 0b1], 32);
    test(&[0, 0b1101], 36);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_ceiling_log_base_2_fail() {
    limbs_ceiling_log_base_2(&[]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_checked_log_base_2() {
    let test = |xs, out| {
        assert_eq!(limbs_checked_log_base_2(xs), out);
    };
    test(&[0b1], Some(0));
    test(&[0b10], Some(1));
    test(&[0b11], None);
    test(&[0b100], Some(2));
    test(&[0, 0b1], Some(32));
    test(&[0, 0b1101], None);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_checked_log_base_2_fail() {
    limbs_checked_log_base_2(&[]);
}

#[test]
fn limbs_floor_log_base_2_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_gen_var_1().test_properties_with_config(&config, |xs| {
        let floor_log_base_2 = limbs_floor_log_base_2(&xs);
        assert_eq!(xs.len() == 1, floor_log_base_2 < Limb::WIDTH);
        assert_eq!(floor_log_base_2, limbs_significant_bits(&xs) - 1);
        assert_eq!(
            floor_log_base_2,
            Natural::from_limbs_asc(&xs).floor_log_base_2()
        );
    });
}

#[test]
fn limbs_ceiling_log_base_2_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_gen_var_1().test_properties_with_config(&config, |xs| {
        let ceiling_log_base_2 = limbs_ceiling_log_base_2(&xs);
        assert_eq!(
            xs.len() == 1 || xs == [0, 1],
            ceiling_log_base_2 <= Limb::WIDTH
        );
        assert_eq!(
            ceiling_log_base_2,
            Natural::from_limbs_asc(&xs).ceiling_log_base_2()
        );
    });
}

#[test]
fn limbs_checked_log_base_2_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_gen_var_1().test_properties_with_config(&config, |xs| {
        let checked_log_base_2 = limbs_checked_log_base_2(&xs);
        assert_eq!(
            checked_log_base_2,
            Natural::from_limbs_asc(&xs).checked_log_base_2()
        );
    });
}

#[test]
fn test_floor_log_base_2() {
    let test = |n, out| {
        assert_eq!(Natural::from_str(n).unwrap().floor_log_base_2(), out);
    };
    test("1", 0);
    test("100", 6);
    test("1000000000000", 39);
    test("4294967295", 31);
    test("4294967296", 32);
    test("18446744073709551615", 63);
    test("18446744073709551616", 64);
}

#[test]
#[should_panic]
fn floor_log_base_2_fail() {
    Natural::ZERO.floor_log_base_2();
}

#[test]
fn test_ceiling_log_base_2() {
    let test = |n, out| {
        assert_eq!(Natural::from_str(n).unwrap().ceiling_log_base_2(), out);
    };
    test("1", 0);
    test("100", 7);
    test("1000000000000", 40);
    test("4294967295", 32);
    test("4294967296", 32);
    test("4294967297", 33);
    test("18446744073709551615", 64);
    test("18446744073709551616", 64);
    test("18446744073709551617", 65);
}

#[test]
#[should_panic]
fn ceiling_log_base_2_fail() {
    Natural::ZERO.ceiling_log_base_2();
}

#[test]
fn test_checked_log_base_2() {
    let test = |n, out| {
        assert_eq!(Natural::from_str(n).unwrap().checked_log_base_2(), out);
    };
    test("1", Some(0));
    test("100", None);
    test("1000000000000", None);
    test("4294967295", None);
    test("4294967296", Some(32));
    test("4294967297", None);
    test("18446744073709551615", None);
    test("18446744073709551616", Some(64));
    test("18446744073709551617", None);
}

#[test]
#[should_panic]
fn checked_log_base_2_fail() {
    Natural::ZERO.checked_log_base_2();
}

#[test]
fn floor_log_base_2_properties() {
    natural_gen_var_2().test_properties(|x| {
        let floor_log_base_2 = x.floor_log_base_2();
        assert_eq!(x <= Limb::MAX, floor_log_base_2 < Limb::WIDTH);
        assert_eq!(floor_log_base_2, x.significant_bits() - 1);
        assert_eq!(floor_log_base_2, limbs_floor_log_base_2(&x.to_limbs_asc()));
        assert!(Natural::power_of_2(floor_log_base_2) <= x);
        assert!(x < Natural::power_of_2(floor_log_base_2 + 1));
    });

    unsigned_gen_var_1::<Limb>().test_properties(|u| {
        assert_eq!(u.floor_log_base_2(), Natural::from(u).floor_log_base_2());
    });
}

#[test]
fn ceiling_log_base_2_properties() {
    natural_gen_var_2().test_properties(|x| {
        let ceiling_log_base_2 = x.ceiling_log_base_2();
        assert_eq!(
            x <= Natural::power_of_2(Limb::WIDTH),
            ceiling_log_base_2 <= Limb::WIDTH
        );
        assert_eq!(
            ceiling_log_base_2,
            limbs_ceiling_log_base_2(&x.to_limbs_asc())
        );
        if ceiling_log_base_2 != 0 {
            assert!(Natural::power_of_2(ceiling_log_base_2 - 1) < x);
        }
        assert!(x <= Natural::power_of_2(ceiling_log_base_2));
    });

    unsigned_gen_var_1::<Limb>().test_properties(|u| {
        assert_eq!(
            u.ceiling_log_base_2(),
            Natural::from(u).ceiling_log_base_2()
        );
    });
}

#[test]
fn checked_log_base_2_properties() {
    natural_gen_var_2().test_properties(|x| {
        let checked_log_base_2 = x.checked_log_base_2();
        assert_eq!(
            checked_log_base_2,
            limbs_checked_log_base_2(&x.to_limbs_asc())
        );
        assert_eq!(checked_log_base_2.is_some(), x.is_power_of_2());
        if let Some(log_base_2) = checked_log_base_2 {
            assert_eq!(x.floor_log_base_2(), log_base_2);
            assert_eq!(x.ceiling_log_base_2(), log_base_2);
            assert_eq!(x <= Limb::MAX, log_base_2 < Limb::WIDTH);
            assert_eq!(log_base_2, x.significant_bits() - 1);
            assert_eq!(Natural::power_of_2(log_base_2), x);
        }
    });

    unsigned_gen_var_1::<Limb>().test_properties(|u| {
        assert_eq!(
            u.checked_log_base_2(),
            Natural::from(u).checked_log_base_2()
        );
    });
}
