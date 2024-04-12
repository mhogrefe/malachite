// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::PowerOf2;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::BitAccess;
use malachite_base::test_util::generators::common::GenConfig;
use malachite_base::test_util::generators::{
    unsigned_vec_unsigned_pair_gen_var_16, unsigned_vec_unsigned_pair_gen_var_17,
};
use malachite_nz::natural::logic::bit_access::{limbs_slice_set_bit, limbs_vec_set_bit};
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use malachite_nz::test_util::generators::natural_unsigned_pair_gen_var_4;
use malachite_nz::test_util::natural::logic::set_bit::num_set_bit;
use num::BigUint;
use rug;
use std::str::FromStr;

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_slice_set_bit() {
    let test = |xs: &[Limb], index: u64, out: &[Limb]| {
        let mut mut_xs = xs.to_vec();
        limbs_slice_set_bit(&mut mut_xs, index);
        assert_eq!(mut_xs, out);
    };
    test(&[0, 1], 0, &[1, 1]);
    test(&[1, 1], 0, &[1, 1]);
    test(&[1, 1], 1, &[3, 1]);
    test(&[3, 1], 33, &[3, 3]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_slice_set_bit_fail() {
    let mut mut_xs = vec![1u32, 2, 3];
    limbs_slice_set_bit(&mut mut_xs, 100);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_vec_set_bit() {
    let test = |xs: &[Limb], index: u64, out: &[Limb]| {
        let mut mut_xs = xs.to_vec();
        limbs_vec_set_bit(&mut mut_xs, index);
        assert_eq!(mut_xs, out);
    };
    test(&[0, 1], 0, &[1, 1]);
    test(&[1, 1], 0, &[1, 1]);
    test(&[1, 1], 1, &[3, 1]);
    test(&[3, 1], 33, &[3, 3]);
    test(&[3, 3], 100, &[3, 3, 0, 16]);
    test(&[3, 3], 128, &[3, 3, 0, 0, 1]);
    test(&[], 32, &[0, 1]);
}

#[test]
fn test_set_bit() {
    let test = |u, index, out| {
        let mut n = Natural::from_str(u).unwrap();
        n.set_bit(index);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let mut n = BigUint::from_str(u).unwrap();
        num_set_bit(&mut n, index);
        assert_eq!(n.to_string(), out);

        let mut n = rug::Integer::from_str(u).unwrap();
        n.set_bit(u32::exact_from(index), true);
        assert_eq!(n.to_string(), out);
    };
    test("0", 10, "1024");
    test("100", 0, "101");
    test("1000000000000", 10, "1000000001024");
    test("1000000000000", 100, "1267650600228229402496703205376");
    test("5", 100, "1267650600228229401496703205381");
}

#[test]
fn limbs_slice_set_bit_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_unsigned_pair_gen_var_17().test_properties_with_config(
        &config,
        |(mut xs, index)| {
            let mut n = Natural::from_limbs_asc(&xs);
            limbs_slice_set_bit(&mut xs, index);
            n.set_bit(index);
            assert_eq!(Natural::from_owned_limbs_asc(xs), n);
        },
    );
}

#[test]
fn limbs_vec_set_bit_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_unsigned_pair_gen_var_16().test_properties_with_config(
        &config,
        |(mut xs, index)| {
            let mut n = Natural::from_limbs_asc(&xs);
            limbs_vec_set_bit(&mut xs, index);
            n.set_bit(index);
            assert_eq!(Natural::from_owned_limbs_asc(xs), n);
        },
    );
}

#[test]
fn natural_set_bit_properties() {
    natural_unsigned_pair_gen_var_4().test_properties(|(n, index)| {
        let mut mut_n = n.clone();
        mut_n.set_bit(index);
        assert!(mut_n.is_valid());
        let result = mut_n;

        let mut mut_n = n.clone();
        mut_n.assign_bit(index, true);
        assert_eq!(mut_n, result);

        let mut num_n = BigUint::from(&n);
        num_set_bit(&mut num_n, index);
        assert_eq!(Natural::from(&num_n), result);

        let mut rug_n = rug::Integer::from(&n);
        rug_n.set_bit(u32::exact_from(index), true);
        assert_eq!(Natural::exact_from(&rug_n), result);

        assert_eq!(&n | Natural::power_of_2(index), result);

        assert_ne!(result, 0);
        assert!(result >= n);
        if n.get_bit(index) {
            assert_eq!(result, n);
        } else {
            assert_ne!(result, n);
            let mut mut_result = result;
            mut_result.clear_bit(index);
            assert_eq!(mut_result, n);
        }
    });
}
