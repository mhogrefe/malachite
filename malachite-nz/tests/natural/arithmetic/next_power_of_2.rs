// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{
    CeilingLogBase2, IsPowerOf2, NextPowerOf2, NextPowerOf2Assign, PowerOf2,
};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::test_util::generators::common::GenConfig;
use malachite_base::test_util::generators::{unsigned_gen, unsigned_vec_gen_var_1};
use malachite_nz::natural::arithmetic::next_power_of_2::{
    limbs_next_power_of_2, limbs_slice_next_power_of_2_in_place, limbs_vec_next_power_of_2_in_place,
};
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use malachite_nz::test_util::generators::natural_gen;
use rug;
use std::str::FromStr;

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_next_power_of_2_and_limbs_vec_next_power_of_2_in_place() {
    let test = |xs: &[Limb], out: &[Limb]| {
        assert_eq!(limbs_next_power_of_2(xs), out);

        let mut xs = xs.to_vec();
        limbs_vec_next_power_of_2_in_place(&mut xs);
        assert_eq!(xs, out);
    };
    test(&[3], &[4]);
    test(&[6, 7], &[0, 8]);
    test(&[100, 101, 102], &[0, 0, 128]);
    test(&[123, 456], &[0, 512]);
    test(&[123, 456, u32::MAX], &[0, 0, 0, 1]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_next_power_of_2_fail() {
    limbs_next_power_of_2(&[]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_slice_next_power_of_2_in_place_fail() {
    limbs_slice_next_power_of_2_in_place(&mut []);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_vec_next_power_of_2_in_place_fail() {
    limbs_vec_next_power_of_2_in_place(&mut Vec::new());
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_slice_next_power_of_2_in_place() {
    let test = |xs: &[Limb], carry: bool, out: &[Limb]| {
        let mut xs = xs.to_vec();
        assert_eq!(limbs_slice_next_power_of_2_in_place(&mut xs), carry);
        assert_eq!(xs, out);
    };
    test(&[3], false, &[4]);
    test(&[6, 7], false, &[0, 8]);
    test(&[100, 101, 102], false, &[0, 0, 128]);
    test(&[123, 456], false, &[0, 512]);
    test(&[123, 456, u32::MAX], true, &[0, 0, 0]);
}

#[test]
fn test_next_power_of_2() {
    let test = |s, out| {
        let u = Natural::from_str(s).unwrap();

        let mut n = u.clone();
        n.next_power_of_2_assign();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = u.clone().next_power_of_2();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = (&u).next_power_of_2();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = rug::Integer::from_str(s).unwrap().next_power_of_two();
        assert_eq!(n.to_string(), out);
    };
    test("0", "1");
    test("1", "1");
    test("2", "2");
    test("3", "4");
    test("4", "4");
    test("5", "8");
    test("6", "8");
    test("7", "8");
    test("8", "8");
    test("9", "16");
    test("10", "16");
    test("123", "128");
    test("1000", "1024");
    test("1000000", "1048576");
    test("1000000000", "1073741824");
    test("1000000000000", "1099511627776");
    test("1073741823", "1073741824");
    test("1073741824", "1073741824");
    test("1073741825", "2147483648");
    test("2147483647", "2147483648");
    test("2147483648", "2147483648");
    test("2147483649", "4294967296");
    test("21344980687", "34359738368");
}

#[test]
fn limbs_next_power_of_2_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_gen_var_1().test_properties_with_config(&config, |xs| {
        assert_eq!(
            Natural::from_owned_limbs_asc(limbs_next_power_of_2(&xs)),
            Natural::from_owned_limbs_asc(xs).next_power_of_2(),
        );
    });
}

#[test]
fn limbs_slice_next_power_of_2_in_place_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_gen_var_1().test_properties_with_config(&config, |mut xs| {
        let old_xs = xs.clone();
        let carry = limbs_slice_next_power_of_2_in_place(&mut xs);
        let n = Natural::from_owned_limbs_asc(old_xs).next_power_of_2();
        let mut expected_xs = n.into_limbs_asc();
        assert_eq!(carry, expected_xs.len() == xs.len() + 1);
        expected_xs.resize(xs.len(), 0);
        assert_eq!(xs, expected_xs);
    });
}

#[test]
fn limbs_vec_next_power_of_2_in_place_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_gen_var_1().test_properties_with_config(&config, |mut xs| {
        let old_xs = xs.clone();
        limbs_vec_next_power_of_2_in_place(&mut xs);
        let n = Natural::from_owned_limbs_asc(old_xs).next_power_of_2();
        assert_eq!(Natural::from_owned_limbs_asc(xs), n);
    });
}

#[test]
fn mod_power_of_2_add_properties() {
    natural_gen().test_properties(|n| {
        let mut mut_n = n.clone();
        mut_n.next_power_of_2_assign();
        assert!(mut_n.is_valid());
        let result = mut_n;

        let result_alt = (&n).next_power_of_2();
        assert!(result_alt.is_valid());
        assert_eq!(result_alt, result);

        let result_alt = n.clone().next_power_of_2();
        assert!(result_alt.is_valid());
        assert_eq!(result_alt, result);

        let result_alt = Natural::exact_from(&rug::Integer::from(&n).next_power_of_two());
        assert_eq!(result_alt, result);

        assert!(result.is_power_of_2());
        assert!(result >= n);
        if n != 0 {
            assert!(&result >> 1 < n);
            assert_eq!(Natural::power_of_2(n.ceiling_log_base_2()), result);
        }
    });

    unsigned_gen::<Limb>().test_properties(|u| {
        if let Some(power) = u.checked_next_power_of_two() {
            assert_eq!(Natural::from(u).next_power_of_2(), u.next_power_of_2());
            assert_eq!(power, Natural::from(u).next_power_of_2());
        }
    });
}
