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
    signed_unsigned_unsigned_triple_gen_var_2, unsigned_pair_gen_var_7, unsigned_triple_gen_var_20,
    unsigned_vec_unsigned_unsigned_triple_gen_var_4,
};
use malachite_base::test_util::num::logic::bit_block_access::get_bits_naive;
use malachite_nz::integer::logic::bit_block_access::{
    limbs_neg_limb_get_bits, limbs_slice_neg_get_bits, limbs_vec_neg_get_bits,
};
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_nz::platform::{Limb, SignedLimb};
use malachite_nz::test_util::generators::{
    integer_unsigned_pair_gen_var_2, integer_unsigned_unsigned_triple_gen_var_2,
    natural_unsigned_unsigned_triple_gen_var_4,
};
use std::str::FromStr;

#[cfg(feature = "32_bit_limbs")]
fn verify_limbs_neg_limb_get_bits(x: Limb, start: u64, end: u64, out: &[Limb]) {
    let n = -Natural::from(x);
    let result = n.get_bits(start, end);
    assert_eq!(get_bits_naive::<Integer, Natural>(&n, start, end), result);
    assert_eq!(Natural::from_limbs_asc(out), result);
}

#[cfg(feature = "32_bit_limbs")]
fn verify_limbs_neg_get_bits(xs: &[Limb], start: u64, end: u64, out: &[Limb]) {
    let n = -Natural::from_limbs_asc(xs);
    let result = n.get_bits(start, end);
    assert_eq!(get_bits_naive::<Integer, Natural>(&n, start, end), result);
    assert_eq!(Natural::from_limbs_asc(out), result);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_neg_limb_get_bits() {
    let test = |x: Limb, start: u64, end: u64, out: &[Limb]| {
        assert_eq!(limbs_neg_limb_get_bits(x, start, end), out);
        verify_limbs_neg_limb_get_bits(x, start, end, out);
    };
    // - trailing_zeros < end
    // - start >= Limb::WIDTH
    test(1, 40, 50, &[0x3ff]);
    // - start < Limb::WIDTH
    // - trailing_zeros < start
    test(0x12345678, 16, 48, &[0xffffedcb]);
    test(0x12345678, 4, 16, &[0xa98]);
    // - trailing_zeros >= start
    test(0x12345678, 0, 100, &[0xedcba988, u32::MAX, u32::MAX, 0xf]);
    test(0x12345678, 10, 10, &[]);
    // - trailing_zeros >= end
    test(0x80000000, 5, 10, &[]);
}

#[test]
#[should_panic]
fn limbs_neg_limb_get_bits_fail() {
    limbs_neg_limb_get_bits(123, 10, 5);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_slice_neg_get_bits() {
    let test = |xs: &[Limb], start: u64, end: u64, out: &[Limb]| {
        assert_eq!(limbs_slice_neg_get_bits(xs, start, end), out);
        verify_limbs_neg_get_bits(xs, start, end, out);
    };
    // - trailing_zeros < end
    // - limb_start >= len
    test(&[1], 40, 50, &[0x3ff]);
    // - limb_start < len
    // - limb_end >= len
    // - offset != 0
    // - trailing_zeros < start
    test(&[0x12345678, 0xabcdef01], 16, 48, &[0x10feedcb]);
    // - limb_end < len
    test(&[0x12345678, 0xabcdef01], 4, 16, &[0xa98]);
    // - offset == 0
    // - trailing_zeros >= start
    test(
        &[0x12345678, 0xabcdef01],
        0,
        100,
        &[0xedcba988, 0x543210fe, u32::MAX, 0xf],
    );
    test(&[0x12345678, 0xabcdef01], 10, 10, &[]);
    // - trailing_zeros >= end
    test(&[0, 0x80000000], 5, 10, &[]);
}

#[test]
#[should_panic]
fn limbs_slice_neg_get_bits_fail() {
    limbs_slice_neg_get_bits(&[123], 10, 5);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_vec_neg_get_bits() {
    let test = |xs: &[Limb], start: u64, end: u64, out: &[Limb]| {
        assert_eq!(limbs_vec_neg_get_bits(xs.to_vec(), start, end), out);
        verify_limbs_neg_get_bits(xs, start, end, out);
    };
    test(&[1], 40, 50, &[0x3ff]);
    test(&[0x12345678, 0xabcdef01], 16, 48, &[0x10feedcb]);
    test(&[0x12345678, 0xabcdef01], 4, 16, &[0xa98]);
    test(
        &[0x12345678, 0xabcdef01],
        0,
        100,
        &[0xedcba988, 0x543210fe, u32::MAX, 0xf],
    );
    test(&[0x12345678, 0xabcdef01], 10, 10, &[]);
    test(&[0, 0x80000000], 5, 10, &[]);
}

#[test]
#[should_panic]
fn limbs_vec_neg_get_bits_fail() {
    limbs_vec_neg_get_bits(vec![123], 10, 5);
}

#[test]
fn test_get_bits() {
    let test = |s, start, end, out| {
        let u = Integer::from_str(s).unwrap();
        let out = Natural::from_str(out).unwrap();

        assert_eq!(u.get_bits(start, end), out);
        assert_eq!(u.clone().get_bits_owned(start, end), out);
        assert_eq!(get_bits_naive::<Integer, Natural>(&u, start, end), out);
    };
    test("12379813738590787192", 16, 48, "4009824820");
    test("12379813738590787192", 4, 16, "1383");
    test("12379813738590787192", 0, 100, "12379813738590787192");
    test("12379813738590787192", 10, 10, "0");
    test("-12379813738590787192", 16, 48, "285142475");
    test("-12379813738590787192", 4, 16, "2712");
    test(
        "-12379813738590787192",
        0,
        100,
        "1267650600215849587758112418184",
    );
    test("-12379813738590787192", 10, 10, "0");
}

#[test]
#[should_panic]
fn get_bits_fail() {
    Integer::from(123).get_bits(10, 5);
}

#[test]
#[should_panic]
fn get_bits_owned_fail() {
    Integer::from(123).get_bits_owned(10, 5);
}

#[test]
fn limbs_neg_limb_get_bits_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_triple_gen_var_20().test_properties_with_config(&config, |(x, start, end)| {
        let out = limbs_neg_limb_get_bits(x, start, end);
        let n = -Natural::from(x);
        let result = n.get_bits(start, end);
        assert_eq!(get_bits_naive::<Integer, Natural>(&n, start, end), result);
        assert_eq!(Natural::from_owned_limbs_asc(out), result);
    });
}

#[test]
fn limbs_neg_get_bits_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_unsigned_unsigned_triple_gen_var_4().test_properties_with_config(
        &config,
        |(xs, start, end)| {
            let out = limbs_slice_neg_get_bits(&xs, start, end);
            let n = -Natural::from_limbs_asc(&xs);
            let result = n.get_bits(start, end);
            assert_eq!(get_bits_naive::<Integer, Natural>(&n, start, end), result);
            assert_eq!(Natural::from_owned_limbs_asc(out), result);

            let out = limbs_vec_neg_get_bits(xs.clone(), start, end);
            let n = -Natural::from_owned_limbs_asc(xs);
            let result = n.get_bits(start, end);
            assert_eq!(get_bits_naive::<Integer, Natural>(&n, start, end), result);
            assert_eq!(Natural::from_owned_limbs_asc(out), result);
        },
    );
}

#[test]
fn get_bit_properties() {
    integer_unsigned_unsigned_triple_gen_var_2().test_properties(|(n, start, end)| {
        let bits = n.get_bits(start, end);
        assert_eq!(n.clone().get_bits_owned(start, end), bits);
        assert_eq!(get_bits_naive::<Integer, Natural>(&n, start, end), bits);
        let significant_bits = n.significant_bits();
        assert_eq!(
            n.get_bits(start + significant_bits, end + significant_bits),
            if n >= 0 {
                Natural::ZERO
            } else {
                Natural::low_mask(end - start)
            }
        );
        assert_eq!(
            (!&n).get_bits(start, end),
            Natural::low_mask(end - start) - &bits
        );
        let mut mut_n = n.clone();
        mut_n.assign_bits(start, end, &bits);
        assert_eq!(n, mut_n);
    });

    integer_unsigned_pair_gen_var_2().test_properties(|(n, start)| {
        assert_eq!(n.get_bits(start, start), 0);
    });

    unsigned_pair_gen_var_7().test_properties(|(start, end)| {
        assert_eq!(Integer::ZERO.get_bits(start, end), 0);
    });

    natural_unsigned_unsigned_triple_gen_var_4().test_properties(|(n, start, end)| {
        assert_eq!(
            Integer::from(&n).get_bits(start, end),
            n.get_bits(start, end)
        );
    });

    signed_unsigned_unsigned_triple_gen_var_2::<Limb, SignedLimb, u64>().test_properties(
        |(n, start, end)| {
            assert_eq!(
                Integer::from(n).get_bits(start, end),
                n.get_bits(start, end)
            );
        },
    );
}
