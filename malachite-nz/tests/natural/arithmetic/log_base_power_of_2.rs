// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{
    CeilingLogBase, CeilingLogBase2, CeilingLogBasePowerOf2, CheckedLogBase, CheckedLogBase2,
    CheckedLogBasePowerOf2, DivisibleBy, FloorLogBase, FloorLogBase2, FloorLogBasePowerOf2,
    IsPowerOf2, PowerOf2,
};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::test_util::generators::common::GenConfig;
use malachite_base::test_util::generators::{
    unsigned_gen_var_11, unsigned_pair_gen_var_21, unsigned_vec_unsigned_pair_gen_var_13,
};
use malachite_nz::natural::arithmetic::log_base_power_of_2::{
    limbs_ceiling_log_base_power_of_2, limbs_checked_log_base_power_of_2,
    limbs_floor_log_base_power_of_2,
};
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use malachite_nz::test_util::generators::{natural_gen_var_2, natural_unsigned_pair_gen_var_8};
use malachite_nz::test_util::natural::arithmetic::log_base_power_of_2::*;
use std::str::FromStr;

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_floor_log_base_power_of_2() {
    let test = |xs: &[Limb], pow, out| {
        assert_eq!(limbs_floor_log_base_power_of_2(xs, pow), out);
    };
    test(&[0b1], 1, 0);
    test(&[0b1], 5, 0);
    test(&[0b10], 1, 1);
    test(&[0b10], 2, 0);
    test(&[0b10], 5, 0);
    test(&[0b11], 1, 1);
    test(&[0b11], 2, 0);
    test(&[0b11], 5, 0);
    test(&[0b100], 1, 2);
    test(&[0b100], 2, 1);
    test(&[0b100], 5, 0);
    test(&[0, 0b1], 1, 32);
    test(&[0, 0b1], 2, 16);
    test(&[0, 0b1], 3, 10);
    test(&[0, 0b1], 4, 8);
    test(&[0, 0b1], 32, 1);
    test(&[0, 0b1], 33, 0);
    test(&[0, 0b1101], 1, 35);
    test(&[0, 0b1101], 2, 17);
    test(&[0, 0b1101], 5, 7);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_floor_log_base_power_of_2_fail() {
    limbs_floor_log_base_power_of_2(&[1], 0);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_ceiling_log_base_power_of_2() {
    let test = |xs: &[Limb], pow, out| {
        assert_eq!(limbs_ceiling_log_base_power_of_2(xs, pow), out);
    };
    test(&[0b1], 1, 0);
    test(&[0b1], 5, 0);
    test(&[0b10], 1, 1);
    test(&[0b10], 2, 1);
    test(&[0b10], 5, 1);
    test(&[0b11], 1, 2);
    test(&[0b11], 2, 1);
    test(&[0b11], 5, 1);
    test(&[0b100], 1, 2);
    test(&[0b100], 2, 1);
    test(&[0b100], 5, 1);
    test(&[0, 0b1], 1, 32);
    test(&[0, 0b1], 2, 16);
    test(&[0, 0b1], 3, 11);
    test(&[0, 0b1], 4, 8);
    test(&[0, 0b1], 32, 1);
    test(&[0, 0b1], 33, 1);
    test(&[0, 0b1101], 1, 36);
    test(&[0, 0b1101], 2, 18);
    test(&[0, 0b1101], 5, 8);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_ceiling_log_base_power_of_2_fail() {
    limbs_ceiling_log_base_power_of_2(&[1], 0);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_checked_log_base_power_of_2() {
    let test = |xs: &[Limb], pow, out| {
        assert_eq!(limbs_checked_log_base_power_of_2(xs, pow), out);
    };
    test(&[0b1], 1, Some(0));
    test(&[0b1], 5, Some(0));
    test(&[0b10], 1, Some(1));
    test(&[0b10], 2, None);
    test(&[0b10], 5, None);
    test(&[0b11], 1, None);
    test(&[0b11], 2, None);
    test(&[0b11], 5, None);
    test(&[0b100], 1, Some(2));
    test(&[0b100], 2, Some(1));
    test(&[0b100], 5, None);
    test(&[0, 0b1], 1, Some(32));
    test(&[0, 0b1], 2, Some(16));
    test(&[0, 0b1], 3, None);
    test(&[0, 0b1], 4, Some(8));
    test(&[0, 0b1], 32, Some(1));
    test(&[0, 0b1], 33, None);
    test(&[0, 0b1101], 1, None);
    test(&[0, 0b1101], 2, None);
    test(&[0, 0b1101], 5, None);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_checked_log_base_power_of_2_fail() {
    limbs_checked_log_base_power_of_2(&[1], 0);
}

#[test]
fn test_floor_log_base_power_of_2() {
    let test = |n, pow, out| {
        assert_eq!(
            Natural::from_str(n).unwrap().floor_log_base_power_of_2(pow),
            out
        );
    };
    test("1", 1, 0);
    test("1", 2, 0);
    test("1", 5, 0);
    test("100", 1, 6);
    test("100", 2, 3);
    test("100", 5, 1);
    test("1000000000000", 1, 39);
    test("1000000000000", 2, 19);
    test("1000000000000", 5, 7);
    test("4294967295", 1, 31);
    test("4294967295", 2, 15);
    test("4294967295", 5, 6);
    test("4294967296", 1, 32);
    test("4294967296", 2, 16);
    test("4294967296", 8, 4);
    test("4294967296", 5, 6);
    test("18446744073709551615", 1, 63);
    test("18446744073709551615", 2, 31);
    test("18446744073709551615", 5, 12);
    test("18446744073709551616", 1, 64);
    test("18446744073709551616", 2, 32);
    test("18446744073709551616", 8, 8);
    test("18446744073709551616", 20, 3);
}

#[test]
#[should_panic]
fn floor_log_base_power_of_2_fail_1() {
    Natural::ZERO.floor_log_base_power_of_2(1);
}

#[test]
#[should_panic]
fn floor_log_base_power_of_2_fail_2() {
    Natural::ONE.floor_log_base_power_of_2(0);
}

#[test]
fn test_ceiling_log_base_power_of_2() {
    let test = |n, pow, out| {
        assert_eq!(
            Natural::from_str(n)
                .unwrap()
                .ceiling_log_base_power_of_2(pow),
            out
        );
    };
    test("1", 1, 0);
    test("1", 2, 0);
    test("1", 5, 0);
    test("100", 1, 7);
    test("100", 2, 4);
    test("100", 5, 2);
    test("1000000000000", 1, 40);
    test("1000000000000", 2, 20);
    test("1000000000000", 5, 8);
    test("4294967295", 1, 32);
    test("4294967295", 2, 16);
    test("4294967295", 5, 7);
    test("4294967296", 1, 32);
    test("4294967296", 2, 16);
    test("4294967296", 8, 4);
    test("4294967296", 5, 7);
    test("18446744073709551615", 1, 64);
    test("18446744073709551615", 2, 32);
    test("18446744073709551615", 5, 13);
    test("18446744073709551616", 1, 64);
    test("18446744073709551616", 2, 32);
    test("18446744073709551616", 8, 8);
    test("18446744073709551616", 20, 4);
}

#[test]
#[should_panic]
fn ceiling_log_base_power_of_2_fail_1() {
    Natural::ZERO.ceiling_log_base_power_of_2(1);
}

#[test]
#[should_panic]
fn ceiling_log_base_power_of_2_fail_2() {
    Natural::ONE.ceiling_log_base_power_of_2(0);
}

#[test]
fn test_checked_log_base_power_of_2() {
    let test = |n, pow, out| {
        assert_eq!(
            Natural::from_str(n)
                .unwrap()
                .checked_log_base_power_of_2(pow),
            out
        );
    };
    test("1", 1, Some(0));
    test("1", 2, Some(0));
    test("1", 5, Some(0));
    test("100", 1, None);
    test("100", 2, None);
    test("100", 5, None);
    test("1000000000000", 1, None);
    test("1000000000000", 2, None);
    test("1000000000000", 5, None);
    test("4294967295", 1, None);
    test("4294967295", 2, None);
    test("4294967295", 5, None);
    test("4294967296", 1, Some(32));
    test("4294967296", 2, Some(16));
    test("4294967296", 8, Some(4));
    test("4294967296", 5, None);
    test("18446744073709551615", 1, None);
    test("18446744073709551615", 2, None);
    test("18446744073709551615", 5, None);
    test("18446744073709551616", 1, Some(64));
    test("18446744073709551616", 2, Some(32));
    test("18446744073709551616", 8, Some(8));
    test("18446744073709551616", 20, None);
}

#[test]
#[should_panic]
fn checked_log_base_power_of_2_fail_1() {
    Natural::ZERO.checked_log_base_power_of_2(1);
}

#[test]
#[should_panic]
fn checked_log_base_power_of_2_fail_2() {
    Natural::ONE.checked_log_base_power_of_2(0);
}

#[test]
fn limbs_floor_log_base_power_of_2_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_unsigned_pair_gen_var_13().test_properties_with_config(&config, |(xs, pow)| {
        assert_eq!(
            limbs_floor_log_base_power_of_2(&xs, pow),
            Natural::from_limbs_asc(&xs).floor_log_base_power_of_2(pow),
        );
    });
}

#[test]
fn limbs_ceiling_log_base_power_of_2_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_unsigned_pair_gen_var_13().test_properties_with_config(&config, |(xs, pow)| {
        assert_eq!(
            limbs_ceiling_log_base_power_of_2(&xs, pow),
            Natural::from_limbs_asc(&xs).ceiling_log_base_power_of_2(pow)
        );
    });
}

#[test]
fn limbs_checked_log_base_power_of_2_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_unsigned_pair_gen_var_13().test_properties_with_config(&config, |(xs, pow)| {
        assert_eq!(
            limbs_checked_log_base_power_of_2(&xs, pow),
            Natural::from_limbs_asc(&xs).checked_log_base_power_of_2(pow)
        );
    });
}

#[test]
fn floor_log_base_power_of_2_properties() {
    natural_unsigned_pair_gen_var_8().test_properties(|(n, pow)| {
        let floor_log = n.floor_log_base_power_of_2(pow);
        assert_eq!(floor_log == 0, n.significant_bits() - 1 < pow);
        assert_eq!(n.floor_log_base(&Natural::power_of_2(pow)), floor_log);

        let product = floor_log * pow;
        assert!(Natural::power_of_2(product) <= n);
        assert!(Natural::power_of_2(product + pow) > n);

        let ceiling_log = n.ceiling_log_base_power_of_2(pow);
        if n.is_power_of_2() && (n.significant_bits() - 1).divisible_by(pow) {
            assert_eq!(ceiling_log, floor_log);
        } else {
            assert_eq!(ceiling_log, floor_log + 1);
        }
    });

    natural_gen_var_2().test_properties(|n| {
        assert_eq!(n.floor_log_base_power_of_2(1), n.floor_log_base_2());
    });

    unsigned_gen_var_11().test_properties(|pow| {
        assert_eq!(Natural::ONE.floor_log_base_power_of_2(pow), 0);
    });

    unsigned_pair_gen_var_21::<Limb, u64>().test_properties(|(n, pow)| {
        assert_eq!(
            n.floor_log_base_power_of_2(pow),
            Natural::from(n).floor_log_base_power_of_2(pow)
        );
    });
}

#[test]
fn ceiling_log_base_power_of_2_properties() {
    natural_unsigned_pair_gen_var_8().test_properties(|(n, pow)| {
        let ceiling_log = n.ceiling_log_base_power_of_2(pow);
        assert_eq!(ceiling_log, ceiling_log_base_power_of_2_naive_nz(&n, pow));
        assert_eq!(ceiling_log == 0, n == Natural::ONE);
        assert_eq!(n.ceiling_log_base(&Natural::power_of_2(pow)), ceiling_log);

        let product = ceiling_log * pow;
        assert!(Natural::power_of_2(product) >= n);
        if product != 0 {
            assert!(Natural::power_of_2(product - pow) < n);
        }

        let floor_log = n.floor_log_base_power_of_2(pow);
        if n.is_power_of_2() && (n.significant_bits() - 1).divisible_by(pow) {
            assert_eq!(floor_log, ceiling_log);
        } else {
            assert_eq!(floor_log, ceiling_log - 1);
        }
    });

    natural_gen_var_2().test_properties(|n| {
        assert_eq!(n.ceiling_log_base_power_of_2(1), n.ceiling_log_base_2());
    });

    unsigned_gen_var_11().test_properties(|pow| {
        assert_eq!(Natural::ONE.ceiling_log_base_power_of_2(pow), 0);
    });

    unsigned_pair_gen_var_21::<Limb, u64>().test_properties(|(n, pow)| {
        assert_eq!(
            n.ceiling_log_base_power_of_2(pow),
            Natural::from(n).ceiling_log_base_power_of_2(pow)
        );
    });
}

#[test]
fn checked_log_base_power_of_2_properties() {
    natural_unsigned_pair_gen_var_8().test_properties(|(n, pow)| {
        let checked_log = n.checked_log_base_power_of_2(pow);
        assert_eq!(n.checked_log_base(&Natural::power_of_2(pow)), checked_log);
        assert_eq!(
            checked_log.is_some(),
            n.is_power_of_2() && (n.significant_bits() - 1).divisible_by(pow)
        );
        if let Some(log) = checked_log {
            assert_eq!(Natural::power_of_2(log * pow), n);
            assert_eq!(log == 0, n == Natural::ONE);
            assert_eq!(n.floor_log_base_power_of_2(pow), log);
            assert_eq!(n.ceiling_log_base_power_of_2(pow), log);
        }
    });

    natural_gen_var_2().test_properties(|n| {
        assert_eq!(n.checked_log_base_power_of_2(1), n.checked_log_base_2());
    });

    unsigned_gen_var_11().test_properties(|pow| {
        assert_eq!(Natural::ONE.checked_log_base_power_of_2(pow), Some(0));
    });

    unsigned_pair_gen_var_21::<Limb, u64>().test_properties(|(n, pow)| {
        assert_eq!(
            n.checked_log_base_power_of_2(pow),
            Natural::from(n).checked_log_base_power_of_2(pow)
        );
    });
}
