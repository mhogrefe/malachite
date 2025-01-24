// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::logic::traits::{BitBlockAccess, LowMask, SignificantBits};
use malachite_base::test_util::generators::common::GenConfig;
use malachite_base::test_util::generators::{
    unsigned_pair_gen_var_16, unsigned_triple_gen_var_5,
    unsigned_vec_unsigned_unsigned_triple_gen_var_3,
};
use malachite_base::test_util::num::logic::bit_block_access::get_bits_naive;
use malachite_nz::natural::logic::bit_block_access::{limbs_slice_get_bits, limbs_vec_get_bits};
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use malachite_nz::test_util::generators::{
    natural_unsigned_pair_gen, natural_unsigned_unsigned_triple_gen_var_4,
};
use std::str::FromStr;

fn verify_limbs_get_bits(xs: &[Limb], start: u64, end: u64, out: &[Limb]) {
    let n = Natural::from_limbs_asc(xs);
    let result = n.get_bits(start, end);
    assert_eq!(get_bits_naive::<Natural, Natural>(&n, start, end), result);
    assert_eq!(Natural::from_limbs_asc(out), result);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_slice_get_bits() {
    let test = |xs: &[Limb], start: u64, end: u64, out: &[Limb]| {
        assert_eq!(limbs_slice_get_bits(xs, start, end), out);
        verify_limbs_get_bits(xs, start, end, out);
    };
    // - limb_start >= len
    test(&[], 10, 20, &[]);
    // - limb_start < len
    // - limb_end >= len
    // - offset != 0
    test(&[0x12345678, 0xabcdef01], 16, 48, &[0xef011234]);
    // - limb_end < len
    test(&[0x12345678, 0xabcdef01], 4, 16, &[0x567]);
    // - offset == 0
    test(&[0x12345678, 0xabcdef01], 0, 100, &[0x12345678, 0xabcdef01]);
    test(&[0x12345678, 0xabcdef01], 10, 10, &[]);
}

#[test]
#[should_panic]
fn limbs_slice_get_bits_fail() {
    limbs_slice_get_bits(&[123], 10, 5);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_vec_get_bits() {
    let test = |xs: Vec<Limb>, start: u64, end: u64, out: &[Limb]| {
        assert_eq!(limbs_vec_get_bits(xs, start, end), out);
    };
    test(vec![], 10, 20, &[]);
    test(vec![0x12345678, 0xabcdef01], 16, 48, &[0xef011234, 0]);
    test(vec![0x12345678, 0xabcdef01], 4, 16, &[0x567]);
    test(
        vec![0x12345678, 0xabcdef01],
        0,
        100,
        &[0x12345678, 0xabcdef01],
    );
    test(vec![0x12345678, 0xabcdef01], 10, 10, &[0]);
}

#[test]
#[should_panic]
fn limbs_vec_get_bits_fail() {
    limbs_vec_get_bits(vec![123], 10, 5);
}

#[test]
fn test_get_bits() {
    let test = |n, start, end, out| {
        assert_eq!(
            Natural::from_str(n).unwrap().get_bits(start, end),
            Natural::from_str(out).unwrap()
        );
        assert_eq!(
            Natural::from_str(n).unwrap().get_bits_owned(start, end),
            Natural::from_str(out).unwrap()
        );
        assert_eq!(
            get_bits_naive::<Natural, Natural>(&Natural::from_str(n).unwrap(), start, end),
            Natural::from_str(out).unwrap()
        );
    };
    test("12379813738590787192", 16, 48, "4009824820");
    test("12379813738590787192", 4, 16, "1383");
    test("12379813738590787192", 0, 100, "12379813738590787192");
    test("12379813738590787192", 10, 10, "0");
}

#[test]
#[should_panic]
fn get_bits_fail() {
    Natural::from(123u32).get_bits(10, 5);
}

#[test]
#[should_panic]
fn get_bits_owned_fail() {
    Natural::from(123u32).get_bits_owned(10, 5);
}

#[test]
fn limbs_get_bits_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_unsigned_unsigned_triple_gen_var_3().test_properties_with_config(
        &config,
        |(xs, start, end)| {
            let out = limbs_slice_get_bits(&xs, start, end);
            verify_limbs_get_bits(&xs, start, end, &out);
            let out = limbs_vec_get_bits(xs.to_vec(), start, end);
            verify_limbs_get_bits(&xs, start, end, &out);
        },
    );
}

#[test]
fn get_bits_properties() {
    natural_unsigned_unsigned_triple_gen_var_4().test_properties(|(n, start, end)| {
        let bits = n.get_bits(start, end);
        assert_eq!(n.clone().get_bits_owned(start, end), bits);
        assert_eq!(get_bits_naive::<Natural, Natural>(&n, start, end), bits);
        assert!(bits <= n);
        let significant_bits = n.significant_bits();
        assert_eq!(
            n.get_bits(start + significant_bits, end + significant_bits),
            0
        );
        assert_eq!(
            (!&n).get_bits(start, end),
            Natural::low_mask(end - start) - &bits
        );
        let mut mut_n = n.clone();
        mut_n.assign_bits(start, end, &bits);
        assert_eq!(n, mut_n);
    });

    natural_unsigned_pair_gen().test_properties(|(n, start)| {
        assert_eq!(n.get_bits(start, start), 0);
    });

    unsigned_pair_gen_var_16().test_properties(|(start, end)| {
        assert_eq!(Natural::ZERO.get_bits(start, end), 0);
    });

    unsigned_triple_gen_var_5::<Limb, u64>().test_properties(|(n, start, end)| {
        assert_eq!(
            Natural::from(n).get_bits(start, end),
            n.get_bits(start, end)
        );
    });
}
